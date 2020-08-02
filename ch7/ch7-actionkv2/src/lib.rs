#[macro_use]
extern crate serde_derive;

extern crate byteorder;
extern crate crc;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::{SeekFrom, BufReader, BufWriter};
use std::fs::{File, OpenOptions};
use std::path::{Path};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::{crc32};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)] // #[derive(Debug)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)] // #[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new() // An example of the "Builder" pattern. Each method returns a new instance of the OpenOptions struct with the relevant option set.
                .read(true)   // Enable reading
                .write(true)  // Enable writing (not strictly necessary, as it's implied by append)
                .create(true) // Create a file at `path` if ir doesn't already exist
                .append(true) // Don't delete any content that's already been written to disk.
                .open(path)?;
        Ok(ActionKV { f: f, index: HashMap::new() })
    }

    /// Assumes that f is already at the right place in the file
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        { // f.by_ref() is required because .take(n) creates a new Read instance. Using a reference within this block allows us to sidestep ownership issues.
            f.by_ref()
             .take(data_len as u64)
             .read_to_end(&mut data)?;
        }
        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!("data corruption encountered ({:08x} != {:08x})", checksum, saved_checksum);
        }

        let val = data.split_off(key_len as usize);
        let key = data;

        Ok( KeyValuePair { key: key, value: val }  )
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        //let mut f = BufReader::new(&mut self.f);
        self.f.seek(SeekFrom::End(0))
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let current_position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => { // "Unexpected" is relative. The application may not have expected it, but we expect files to be finite.
                            break;
                        },
                        _ => return Err(err),
                    }
                },
            };

            self.index.insert(kv.key, current_position);
        }

        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> { // we need to wrap Option within Result to allow for the possibilities of I/O errors as well as missing values occuring
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };

        let kv = self.get_at(position)?;

        Ok(Some(ByteString::from(kv.value)))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;

        // Even though `f.seek` does take `&mut self`, my understanding is calling it
        // with `SeekFrom::Current(0)` simply says "where am I at in the stream" without
        // altering anything. In fact there's even a convenience function that claims
        // exactly this: https://doc.rust-lang.org/std/io/trait.Seek.html#method.stream_position
        //
        // However, commenting these two lines makes `test_index_maintenance` pass
        let final_pos = f.seek(SeekFrom::Current(0))?;
        println!("after reading at {}, final_pos is {}", position, final_pos);
        Ok(kv)
    }

    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);

        let mut found : Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => { // "Unexpected" is relative. The application may not have expected it, but we expect files to be finite.
                            break;
                        },
                        _ => return Err(err),
                    }
                },
            };

            if kv.key == target {
                found = Some((position, kv.value));
            }

            // important to keep looping until the end of the file,
            // in case the key has been overwritten
        }

        Ok(found)
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;

        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&mut tmp)?;

        println!("current_position: {}, {:?}, {:?}", current_position, key, value);
        Ok(current_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

}


#[cfg(test)]
mod tests {
    use super::ActionKV;
    use tempdir::TempDir;

    #[test]
    fn test_index_maintenance() {
        let key1 = "a".as_bytes();
        let val1 = "1".as_bytes();

        let key2 = "b".as_bytes();
        let val2 = "2".as_bytes();

        let key3 = "c".as_bytes();
        let val3 = "3".as_bytes();

        let key4 = "d".as_bytes();
        let val4 = "4".as_bytes();

        let dir = TempDir::new("testing").expect("failed to create temp dir");
        let path = dir.path().join("foo.txt");

        let mut store = ActionKV::open(path.as_path()).expect("failed to create store");

        store.insert(key1, val1).expect("failed to insert key1");
        store.insert(key2, val2).expect("failed to insert key2");
        store.insert(key3, val3).expect("failed to insert key3");

        // My understanding of how everything works is that retreiving `key1` *should* leave the file
        // pointing at the start of `key2`.
        //
        // Thus when we insert `4`, even though it gets *written* to the end of the file, the
        // index will be pointing to the incorrect location.
        //
        // You can verify this by inspecting the `index` values that get printed or retrieving the incorrect value for `key4`
        //
        // However, that only happens **WITH** the .seek(Current(0))` in `get_at`

        let retrieved_key_1 = store.get(key1).expect("failed to retrieve key 1");
        let retrieved_val_1 = retrieved_key_1.expect("None returned for key1");
        assert_eq!(val1, retrieved_val_1.as_slice());

        store.insert(key4, val4).expect("failed to insert key4");

        let retrieved_key_4 = store.get(key4).expect("failed to retrieve key 3");
        let retrieved_val_4 = retrieved_key_4.expect("None returned for key1");
        println!("index: {:?}", store.index);
        assert_eq!(val4, retrieved_val_4.as_slice());
    }
}

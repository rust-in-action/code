extern crate bincode;
extern crate libactionkv;

use std::collections::HashMap;
use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open file");

    a.load().expect("unable to load data");
    store_index_on_disk(&mut a, INDEX_KEY);

    match action {
        "get" => {
            let index_as_bytes = a.get(&INDEX_KEY).unwrap().unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(value) => println!("{:?}", value), // needs to use Debug as [u8] is arbitrary bytes
            }
        },
        "delete" => a.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.insert(key, value).unwrap()
        },
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.update(key, value).unwrap()
        },
        _ => eprintln!("{}", &USAGE),
    }
}

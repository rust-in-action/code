pub fn main() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        if let Some(_) = i.checked_add(1000) {
            if i % 10000 == 0 {
                print! {"{i}...\n"}
            }
            i += 1000;
        } else {
            break;
        }
    }
}

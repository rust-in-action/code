fn is_strong(pw: String) -> bool {
    pw.len() > 5
}


fn is_strong_asref<T: AsRef<str>>(pw: T) -> bool {
    pw.as_ref().len() > 5
}

fn is_strong_intostring<T: Into<String>>(pw: T) -> bool {
    pw.into().len() > 5
}


fn main() {
    println!("try cargo test");
}

#[test]
fn test_original() {
    let pw = String::from("justok");
    assert!(is_strong(pw));
}

#[test]
fn test_asref() {
    let pw1 = String::from("justok");
    assert!(is_strong_asref(pw1));

    let pw2 = "justok";
    assert!(is_strong_asref(pw2));
}

#[test]
fn test_intostring() {
    let pw1 = String::from("justok");
    assert!(is_strong_intostring(pw1));

    let pw2 = "justok";
    assert!(is_strong_intostring(pw2));
}

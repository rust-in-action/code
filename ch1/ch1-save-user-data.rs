use std::str;

#[derive(Debug)]
struct User {
    id: u8,
    secret: String,
}

fn store_secrets(user: &User, buffer: &mut[u8]) {
    let _secret = user.secret.clone();

    
    // assume we're writing to a database
    println!("{:?}: {}", user, str::from_utf8(&buffer).unwrap());
}

fn main() {
    let buffer = &mut[0u8; 1024];
    let u1 = User {
        id: 1,
        secret: String::from("Pa55w0rd!"),
    };
    let u2 = User {
        id: 2,
        secret: String::from("correct horse battery staple"),
    };

    store_secrets(&u1, buffer);
    store_secrets(&u2, buffer);
    

}
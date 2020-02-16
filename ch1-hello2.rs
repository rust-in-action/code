fn greet_world() {
    println!("Hello, world!");  // our old friend.

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド"; 

    let regions = [southern_germany, japan];

    for region in regions.iter() {
            println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
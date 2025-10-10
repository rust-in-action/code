mod ch2_3arrays;
mod ch2_add_floats;
mod ch2_add_with_lifetimes;
mod ch2_bufreader_lines;
mod ok;

fn main() {
    println!("-- ok --");
    ok::main();

    println!("-- ch2_3arrays --");
    ch2_3arrays::main();

    println!("-- ch2_add_floats --");
    ch2_add_floats::main();

    println!("-- ch2_add_with_lifetimes --");
    ch2_add_with_lifetimes::main();

    println!("-- ch2_bufreader_lines --");
    ch2_bufreader_lines::main();
}

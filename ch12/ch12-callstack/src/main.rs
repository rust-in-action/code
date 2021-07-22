fn print_depth(depth:usize) {
    for _ in 0..depth {
        print!("#");
    }
    println!("");
}

fn dive(depth: usize, max_depth: usize) {
    print_depth(depth);
    if depth >= max_depth {
        return;

    } else {
        dive(depth+1, max_depth);
    }
    print_depth(depth);
}

fn main() {
    dive(0, 5);
}

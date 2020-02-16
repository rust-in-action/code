use std::collections::HashMap;

fn main() {
    let text = "once upon a time ...";
    let mut word_counts = HashMap::new();
    
    let pairs = text.split(" ")
                    .map(|x| { (x, 1) });
    
    for (word, count) in pairs {
        let tmp = word_counts.entry(word)
                             .or_insert(0);
        *tmp += count;
    }
    println!("{:?}", word_counts);
}

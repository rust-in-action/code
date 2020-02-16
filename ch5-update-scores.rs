#[derive(Debug)]
struct Score(isize);

impl std::ops::AddAssign for Score {
    fn add_assign(&mut self, right_hand: Score) {
        self.0 += right_hand.0;
    }
}

fn main() {
    let mut scores = vec![
        Score(1),
        Score(3),
        Score(5)
    ];
    println!("initial: {:?}", scores);

    for mut score in &mut scores {
        *score += Score(1);
    }
    println!("imperative: {:?}", scores);

    scores.iter_mut().map(|score| *score += Score(1)).collect();
    println!("higher-order: {:?}", scores);
}

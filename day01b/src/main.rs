use std::collections::HashMap;

pub fn main() {
    let mut a: Vec<usize> = Vec::new();
    let mut b: HashMap<usize, usize> = HashMap::new();

    include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(left, right)| {
            a.push(left.parse().unwrap());
            *b.entry(right.parse().unwrap()).or_default() += 1;
        });

    let sum = a
        .into_iter()
        .map(|n| b.get(&n).map_or(0, |a| a * n))
        .sum::<usize>();

    println!("{sum}");
}

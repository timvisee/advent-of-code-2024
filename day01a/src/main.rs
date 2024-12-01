pub fn main() {
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();

    include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(left, right)| {
            a.push(left.parse().unwrap());
            b.push(right.parse().unwrap());
        });

    a.sort_unstable();
    b.sort_unstable();

    let sum = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>();

    println!("{sum}");
}

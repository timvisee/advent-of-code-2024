pub fn main() {
    let (mut a, mut b) = (Vec::with_capacity(1000), Vec::with_capacity(1000));
    let num_len = include_bytes!("../input.txt")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    for line in include_bytes!("../input.txt").split(|&b| b == b'\n') {
        a.push(atoi::atoi::<usize>(&line[0..num_len]).unwrap());
        b.push(atoi::atoi::<usize>(&line[num_len + 3..]).unwrap());
    }

    a.sort_unstable();
    b.sort_unstable();

    println!(
        "{}",
        a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum::<usize>(),
    );
}

pub fn main() {
    let (mut a, mut b) = (Vec::with_capacity(1000), [0; 100_000]);
    let num_lenl = include_bytes!("../input.txt")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    for line in include_bytes!("../input.txt").split(|&b| b == b'\n') {
        a.push(atoi::atoi::<usize>(&line[0..num_lenl]).unwrap());
        b[atoi::atoi::<usize>(&line[num_lenl + 3..]).unwrap()] += 1;
    }

    println!("{}", a.into_iter().map(|n| b[n] * n).sum::<usize>());
}

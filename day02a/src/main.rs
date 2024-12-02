pub fn main() {
    let count = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| {
            let nums = line
                .split(|&b| b == b' ')
                .map(|n| atoi::atoi::<isize>(n).unwrap())
                .collect::<Vec<_>>();

            nums.windows(2).all(|n| {
                let n = n[1].wrapping_sub(n[0]);
                (1..=3).contains(&n)
            }) || nums.windows(2).all(|n| {
                let n = n[0].wrapping_sub(n[1]);
                (1..=3).contains(&n)
            })
        })
        .count();

    println!("{count}",);
}

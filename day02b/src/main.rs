use itertools::Itertools;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|line| {
                let nums = line
                    .split(|&b| b == b' ')
                    .map(|n| atoi::atoi::<isize>(n).unwrap())
                    .collect::<Vec<_>>();
                (0..=nums.len()).any(|i| {
                    nums[0..i]
                        .iter()
                        .chain(&nums[nums.len().min(i + 1)..])
                        .tuple_windows()
                        .try_fold(0, |ord, (a, b)| {
                            if ord >= 0 && (1..=3).contains(&(b - a)) {
                                Ok(1)
                            } else if ord <= 0 && (1..=3).contains(&(a - b)) {
                                Ok(-1)
                            } else {
                                Err(())
                            }
                        })
                        .is_ok()
                })
            })
            .count(),
    );
}

use itertools::Itertools;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|line| {
                line.split(|&b| b == b' ')
                    .map(|n| atoi::atoi::<isize>(n).unwrap())
                    .tuple_windows()
                    .try_fold(0, |acc, (a, b)| {
                        if acc >= 0 && (1..=3).contains(&(b - a)) {
                            Ok(1)
                        } else if acc <= 0 && (1..=3).contains(&(a - b)) {
                            Ok(-1)
                        } else {
                            Err(())
                        }
                    })
                    .is_ok()
            })
            .count(),
    );
}

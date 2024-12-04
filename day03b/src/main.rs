use regex::bytes::Regex;

pub fn main() {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;

    println!(
        "{}",
        re.captures_iter(include_bytes!("../input.txt"))
            .filter(|capture| {
                if capture.get(0).unwrap().as_bytes() == b"do()" {
                    enabled = true;
                    return false;
                } else if capture.get(0).unwrap().as_bytes() == b"don't()" {
                    enabled = false;
                }
                enabled
            })
            .map(|capture| {
                atoi::atoi::<usize>(capture.get(2).unwrap().as_bytes()).unwrap()
                    * atoi::atoi::<usize>(capture.get(3).unwrap().as_bytes()).unwrap()
            })
            .sum::<usize>(),
    );
}

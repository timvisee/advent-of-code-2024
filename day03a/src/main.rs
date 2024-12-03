use regex::bytes::Regex;

pub fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    println!(
        "{}",
        re.captures_iter(include_bytes!("../input.txt"))
            .map(|capture| {
                atoi::atoi::<usize>(capture.get(1).unwrap().as_bytes()).unwrap()
                    * atoi::atoi::<usize>(capture.get(2).unwrap().as_bytes()).unwrap()
            })
            .sum::<usize>(),
    );
}

pub fn main() {
    let mut word = [0; 4];
    let map = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .flat_map(|(x, y)| {
                [
                    [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                    [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                    [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                    [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
                ]
            })
            .filter(|coords| {
                let mut iter = coords.iter().map(|(x, y)| {
                    map.get(*y as usize)
                        .and_then(|row| row.get(*x as usize).copied())
                        .unwrap_or_default()
                });
                word.fill_with(|| iter.next().unwrap_or_default());
                &word == b"XMAS" || &word == b"SAMX"
            })
            .count(),
    );
}

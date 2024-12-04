pub fn main() {
    let mut cross = [0; 4];
    let map = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .map(|(x, y)| {
                [
                    (x + 1, y + 1), // Center
                    (x, y),         // NE
                    (x, y + 2),     // SE
                    (x + 2, y),     // NW
                    (x + 2, y + 2), // SW
                ]
            })
            .filter(|coords| {
                let mut iter = coords.iter().map(|(x, y)| {
                    map.get(*y as usize)
                        .and_then(|row| row.get(*x as usize).copied())
                        .unwrap_or_default()
                });

                if iter.next().is_none_or(|n| n != b'A') {
                    return false;
                }

                cross[0] = iter.next().unwrap_or_default();
                cross[1] = iter.next().unwrap_or_default();
                cross[2] = iter.next().unwrap_or_default();
                cross[3] = iter.next().unwrap_or_default();

                &cross == b"MMSS" || &cross == b"MSMS" || &cross == b"SSMM" || &cross == b"SMSM"
            })
            .count(),
    );
}

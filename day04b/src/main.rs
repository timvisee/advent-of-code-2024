pub fn main() {
    let mut cross = [0; 5];
    let map = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    let count = (0..map[0].len() as isize)
        .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                (x, y),
                (x, y + 2),
                (x + 1, y + 1),
                (x + 2, y),
                (x + 2, y + 2),
            ]
        })
        .filter(|coords| {
            let mut iter = coords.into_iter().scan(0, |_, (x, y)| {
                map.get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
            });
            cross.fill_with(|| iter.next().unwrap_or_default());
            &cross == b"MMASS" || &cross == b"MSAMS" || &cross == b"SSAMM" || &cross == b"SMASM"
        })
        .count();

    println!("{count}");
}

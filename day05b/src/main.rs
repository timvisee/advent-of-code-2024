use ahash::AHashMap;

pub fn main() {
    let data = include_bytes!("../input.txt");
    let mid = data.windows(2).position(|b| b == b"\n\n").unwrap();

    let mut orders: AHashMap<_, Vec<_>> =
        data[0..mid]
            .split(|&b| b == b'\n')
            .fold(AHashMap::new(), |mut orders, range| {
                let mut split = range.split(|&b| b == b'|');
                orders
                    .entry(atoi::atoi::<usize>(split.next().unwrap()).unwrap())
                    .or_default()
                    .push(atoi::atoi::<usize>(split.next().unwrap()).unwrap());
                orders
            });
    orders.values_mut().for_each(|pages| pages.sort_unstable());

    println!(
        "{}",
        data[mid + 2..]
            .split(|&b| b == b'\n')
            .map(|pages| {
                pages
                    .split(|&b| b == b',')
                    .map(|page| atoi::atoi::<usize>(page).unwrap())
                    .collect::<Vec<_>>()
            })
            .filter_map(|mut pages| {
                'outer: for attempt in 0.. {
                    for (i, page) in pages.iter().enumerate() {
                        if let Some(orders) = orders.get(page) {
                            if let Some(j) = pages[0..i]
                                .iter()
                                .position(|page| orders.binary_search(page).is_ok())
                            {
                                pages.swap(i, j);
                                continue 'outer;
                            }
                        }
                    }
                    return (attempt > 0).then_some(pages[pages.len() / 2]);
                }
                None
            })
            .sum::<usize>(),
    );
}

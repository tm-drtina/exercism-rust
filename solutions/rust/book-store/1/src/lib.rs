const PRICES: [u32; 6] = [0, 800, 1520, 2160, 2560, 3000];

fn basket_price(counts: [u32; 5]) -> u32 {
    if counts[0] <= 1 {
        PRICES[counts.iter().sum::<u32>() as usize]
    } else {
        (0..5).filter_map(|n| {
            if counts[n] > 0 {
                let mut c = counts;
                c.iter_mut().take(n + 1).for_each(|i| *i -= 1);
                c.sort_unstable();
                c.reverse(); // #lazy
                Some(PRICES[n + 1] + basket_price(c))
            } else {
                None
            }
        }).min().unwrap()
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut amounts = [0u32; 5];
    for book in books {
        amounts[*book as usize - 1] += 1;
    }
    amounts.sort_unstable();
    amounts.reverse(); // #lazy
    basket_price(amounts)
}

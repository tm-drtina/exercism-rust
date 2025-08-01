pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.bytes()
                .enumerate()
                .map(|(x, ch)| match ch {
                    b'*' => '*',
                    b' ' => {
                        let x_range = x.saturating_sub(1)..=(x + 1).min(row.len() - 1);
                        let y_range = y.saturating_sub(1)..=(y + 1).min(garden.len() - 1);
                        match garden[y_range].iter().map(|arr| arr[x_range.clone()].bytes().filter(|ch| *ch == b'*').count()).sum::<usize>() as u8 {
                            0 => ' ',
                            x@1..=8 => (x + b'0') as char,
                            x => panic!("Count more than 8! Count: {x}"),
                        }
                        
                    },
                    ch => panic!("Unexpected char in input '{}'", ch as char),
                })
                .collect()
        })
        .collect()
}

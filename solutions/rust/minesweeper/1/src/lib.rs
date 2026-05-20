fn is_mine(minefield: &[&str], x: usize, y: usize) -> bool {
    let Some(row) = minefield.get(y) else {
        return false;
    };

    let Some(ch) = row.as_bytes().get(x) else {
        return false;
    };

    *ch == b'*'
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(y, row)| {
            String::from_utf8(
                row.bytes()
                    .enumerate()
                    .map(|(x, b)| {
                        if b == b'*' {
                            b'*'
                        } else {
                            let count = (y.saturating_add_signed(-1)..=y + 1)
                                .flat_map(|y| (x.saturating_add_signed(-1)..=x + 1).map(move |x| (x, y)))
                                .filter(|(x_, y_)| x != *x_ || y != *y_)
                                .filter(|(x, y)| is_mine(minefield, *x, *y))
                                .count() as u8;

                            if count > 0 {
                                b'0' + count
                            } else {
                                b' '
                            }
                        }
                    })
                    .collect(),
            )
            .unwrap()
        })
        .collect()
}

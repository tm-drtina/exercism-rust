/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let Ok((length, sum)) = code.chars().rev().filter_map(|ch| {
        match ch {
            '0'..='9' => Some(Ok(ch as u8 - b'0')),
            ' ' => None,
            _ => Some(Err(())) ,
        }
    }).try_fold((0, 0), |(len, sum), num| {
        let num = num? as u32;
        let value = if len % 2 > 0 {
            // even positions
            let mut num = num * 2;
            if num > 9 {
                num -= 9;
            }
            num
        } else {
            // odds
            num
        };
        Ok::<(i32, u32), ()>((len + 1, sum + value))
    }) else {
        return false;
    };

    if length < 2 {
        false
    } else {
        sum % 10 == 0
    }
}

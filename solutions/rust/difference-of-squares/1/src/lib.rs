pub fn square_of_sum(n: u32) -> u32 {
    let a = (n * n + n) / 2;
    a * a
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|a| a*a).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

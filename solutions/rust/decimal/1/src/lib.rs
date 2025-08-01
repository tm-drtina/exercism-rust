use std::cmp::Ordering;

use num_bigint::{BigInt, Sign};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Decimal {
    int: BigInt,
    dec_points: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if let Some((mut int, dec)) = input.split_once('.') {
            // avoid trailing zeros
            let dec = dec.trim_end_matches('0');

            // This might be `-0.1`, where int part is `-0` and we would end up losing the sign
            let mut positive = true;
            if int.starts_with('-') {
                positive = false;
                int = &int[1..];
            }

            let mut int: BigInt = int
                .parse()
                .inspect_err(|e| eprintln!("Invalid int part: '{int}'. Err {e}"))
                .ok()?;
            let dec_points = dec.len() as u32;
            if dec_points > 0 {
                let dec: BigInt = dec
                    .parse()
                    .inspect_err(|e| eprintln!("Invalid dec part: '{dec}'. Err {e}"))
                    .ok()?;
                assert!(matches!(dec.sign(), Sign::NoSign | Sign::Plus));

                int = int * BigInt::from(10u64).pow(dec_points) + dec;
            }

            if !positive {
                int = -int;
            }

            Some(Self { int, dec_points }.normalized())
        } else {
            Some(
                Self {
                    int: input
                        .parse()
                        .inspect_err(|e| eprintln!("Invalid int part: '{input}'. Err {e}"))
                        .ok()?,
                    dec_points: 0,
                }
                .normalized(),
            )
        }
    }

    fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    fn normalize(&mut self) {
        if self.dec_points == 0 {
            return;
        }

        // Reduce exponent
        while &self.int % 10u32 == BigInt::ZERO {
            self.int /= 10u32;
            self.dec_points -= 1;
        }
    }

    fn unify_exp(&mut self, rhs: &mut Self) {
        match self.dec_points.cmp(&rhs.dec_points) {
            Ordering::Less => {
                self.int *= BigInt::from(10u32).pow(rhs.dec_points - self.dec_points);
                self.dec_points += rhs.dec_points - self.dec_points;
            },
            Ordering::Greater => {
                rhs.int *= BigInt::from(10u32).pow(self.dec_points - rhs.dec_points);
                rhs.dec_points += self.dec_points - rhs.dec_points;
            },
            Ordering::Equal => {},
        }
    }
}

impl std::ops::Add for Decimal {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.unify_exp(&mut rhs);
        assert_eq!(self.dec_points, rhs.dec_points);

        Self {
            int: self.int + rhs.int,
            dec_points: self.dec_points,
        }
        .normalized()
    }
}

impl std::ops::Sub for Decimal {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self.unify_exp(&mut rhs);
        assert_eq!(self.dec_points, rhs.dec_points);

        Self {
            int: self.int - rhs.int,
            dec_points: self.dec_points,
        }
        .normalized()
    }
}

impl std::ops::Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            int: self.int * rhs.int,
            dec_points: self.dec_points + rhs.dec_points,
        }
        .normalized()
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.int.sign().cmp(&other.int.sign()) {
            Ordering::Equal => {}
            x => return Some(x),
        }
        let mut n1 = self.clone();
        let mut n2 = other.clone();
        n1.unify_exp(&mut n2);
        Some(n1.int.cmp(&n2.int))
    }
}

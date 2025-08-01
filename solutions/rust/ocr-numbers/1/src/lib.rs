// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

struct LinesIter<'a>(std::str::Lines<'a>, usize);
impl<'a> LinesIter<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.lines(), 0)
    }
}
impl<'a> Iterator for LinesIter<'a> {
    type Item = Result<NumberIter<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let l1 = self.0.next()?;
        self.1 += 1;
        let Some(l2) = self.0.next() else {
            return Some(Err(Error::InvalidRowCount(self.1)));
        };
        self.1 += 1;
        let Some(l3) = self.0.next() else {
            return Some(Err(Error::InvalidRowCount(self.1)));
        };
        self.1 += 1;
        let Some(_l4) = self.0.next() else {
            return Some(Err(Error::InvalidRowCount(self.1)));
        };
        self.1 += 1;

        Some(NumberIter::new(l1, l2, l3))
    }
}

struct NumberIter<'a>(&'a str, &'a str, &'a str);
impl<'a> NumberIter<'a> {
    pub fn new(l1: &'a str, l2: &'a str, l3: &'a str) -> Result<Self, Error> {
        if l1.len() % 3 != 0 {
            Err(Error::InvalidColumnCount(l1.len()))
        } else {
            assert_eq!(l1.len(), l2.len());
            assert_eq!(l2.len(), l3.len());
            Ok(Self(l1, l2, l3))
        }
    }
}
impl<'a> Iterator for NumberIter<'a> {
    type Item = char;

    #[rustfmt::skip]
    fn next(&mut self) -> Option<Self::Item> {
        let (s1, rest) = self.0.split_at_checked(3)?;
        self.0 = rest;
        let (s2, rest) = self.1.split_at_checked(3)?;
        self.1 = rest;
        let (s3, rest) = self.2.split_at_checked(3)?;
        self.2 = rest;

        Some(match (s1, s2, s3) {
            (" _ ",
             "| |",
             "|_|") => '0',
            ("   ",
             "  |",
             "  |") => '1',
            (" _ ",
             " _|",
             "|_ ") => '2',
            (" _ ",
             " _|",
             " _|") => '3',
            ("   ",
             "|_|",
             "  |") => '4',
            (" _ ",
             "|_ ",
             " _|") => '5',
            (" _ ",
             "|_ ",
             "|_|") => '6',
            (" _ ",
             "  |",
             "  |",) => '7',
            (" _ ",
             "|_|",
             "|_|") => '8',
            (" _ ",
             "|_|",
             " _|") => '9',
            _ => '?'
        })
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut res = String::new();
    for (i, row) in LinesIter::new(input).enumerate() {
        let row = row?;
        if i > 0 {
            res.push(',');
        }
        for num in row {
            res.push(num);
        }
    }

    Ok(res)
}

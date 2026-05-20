pub struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(str::parse::<u32>)
                        .collect()
                })
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.data.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.data
            .iter()
            .map(|row| row.get(col_no - 1).copied())
            .collect()
    }
}

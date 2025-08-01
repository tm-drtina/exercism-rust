type Piece = (u8, u8);

struct State {
    chain: Vec<Piece>,
    available: Vec<Piece>,
}

impl State {
    fn new(input: &[(u8, u8)]) -> Self {
        Self {
            chain: input.iter().take(1).copied().collect(),
            available: input.iter().skip(1).copied().collect(),
        }
    }

    fn is_valid(&self) -> bool {
        self.chain.first().map(|x| x.0) == self.chain.last().map(|x| x.1)
    }

    fn is_final(&self) -> bool {
        self.available.is_empty()
    }

    fn solve(mut self) -> Result<Vec<Piece>, Self> {
        if self.is_final() {
            return if self.is_valid() { Ok(self.chain) } else { Err(self) }
        }
        for i in (0..self.available.len()).rev() {
            let matches = self.chain.last().unwrap().1 == self.available[i].0;
            let matches_rev = self.chain.last().unwrap().1 == self.available[i].1;
            if matches || matches_rev {
                let mut piece = self.available.swap_remove(i);
                if !matches {
                    std::mem::swap(&mut piece.0, &mut piece.1);
                }
                self.chain.push(piece);
                match self.solve() {
                    Ok(res) => return Ok(res),
                    Err(new_self) => self = new_self,
                }
                self.available.push(self.chain.pop().unwrap());
            }
        }
        Err(self)
    }
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    State::new(input).solve().ok()
}

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    HighCard([u8; 5]),
    OnePair([u8; 4]),
    TwoPair([u8; 3]),
    ThreeOfAKind([u8; 3]),
    Straight([u8; 1]),
    Flush([u8; 5]),
    FullHouse([u8; 2]),
    FourOfAKind([u8; 2]),
    StraightFlush([u8; 1]),
}

impl Score {
    fn compute(hand: &ParsedHand) -> Self {
        let flush = hand.iter().all(|c| c.color == hand[0].color);
        let hand_values = hand.map(|h| h.value);
        let straight = if hand_values.windows(2).all(|w| w[0] - w[1] == 1) {
            Some(hand_values[0])
        } else if hand_values == [14, 5, 4, 3, 2] {
            // Special case where we consider ace as 1. Note that the high card is `5`, not `14`!
            Some(5)
        } else {
            None
        };
        let mut single = Vec::with_capacity(5);
        let mut pairs = Vec::with_capacity(2);
        let mut threes = None;
        let mut fours = None;
        let last_state = hand.iter().skip(1).fold((hand[0], 1), |(prev, count), c| {
            if prev != *c {
                match count {
                    1 => single.push(prev.value),
                    2 => pairs.push(prev.value),
                    3 if threes.is_none() => threes = Some(prev.value),
                    4 if fours.is_none() => fours = Some(prev.value),
                    _ => unreachable!(),
                }
                (*c, 1)
            } else {
                (prev, count + 1)
            }
        });
        match last_state {
            (c, 1) => single.push(c.value),
            (c, 2) => pairs.push(c.value),
            (c, 3) if threes.is_none() => threes = Some(c.value),
            (c, 4) if fours.is_none() => fours = Some(c.value),
            _ => unreachable!(),
        }

        match (
            straight.is_some(),
            flush,
            pairs.len(),
            threes.is_some(),
            fours.is_some(),
        ) {
            (true, true, _, _, _) => Self::StraightFlush([straight.unwrap()]),
            (_, _, 0, false, true) => Self::FourOfAKind([fours.unwrap(), single[0]]),
            (_, _, 1, true, false) => Self::FullHouse([threes.unwrap(), pairs[0]]),
            (_, true, _, _, _) => Self::Flush([
                hand[0].value,
                hand[1].value,
                hand[2].value,
                hand[3].value,
                hand[4].value,
            ]),
            (true, _, _, _, _) => Self::Straight([straight.unwrap()]),
            (_, _, 0, true, false) => Self::ThreeOfAKind([threes.unwrap(), single[0], single[1]]),
            (_, _, 2, false, false) => Self::TwoPair([pairs[0], pairs[1], single[0]]),
            (_, _, 1, false, false) => Self::OnePair([pairs[0], single[0], single[1], single[2]]),
            (_, _, 0, false, false) => {
                Self::HighCard([single[0], single[1], single[2], single[3], single[4]])
            }
            _ => unreachable!(
                "Unknown hand: {hand:?}\nStraight: {straight:?}\nFlush: {flush}\nPairs: {pairs:?}\nThrees: {threes:?}\nFours: {fours:?}"
            ),
        }
    }
}

type ParsedHand = [Card; 5];

#[derive(Debug, Clone, Copy)]
struct Card {
    value: u8,
    color: u8,
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Card {}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl FromStr for Card {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.bytes();
        let value = match bytes.next().unwrap() {
            b @ b'2'..=b'9' => b - b'0',
            b'1' => {
                if bytes.next() != Some(b'0') {
                    panic!("Invalid char! Expected `0` after `1`");
                }
                10
            }
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            ch => panic!("Invalid char: {ch} == {:?}", char::from_u32(ch as u32)),
        };
        let color = bytes.next().unwrap();
        Ok(Self { value, color })
    }
}

#[derive(Debug)]
struct Hand<'a> {
    raw: &'a str,
    score: Score,
}

impl<'a> Hand<'a> {
    fn new(raw: &'a str) -> Self {
        let mut card_iter = raw.split_ascii_whitespace();
        let mut parsed: [Card; 5] =
            std::array::from_fn(|_| card_iter.next().unwrap().parse().unwrap());
        parsed.sort_unstable_by(|a, b| b.cmp(a));
        let score = Score::compute(&parsed);
        Self { raw, score }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands = hands.iter().copied().map(Hand::new).collect::<Vec<_>>();
    let max_hand = hands.iter().max_by_key(|h| &h.score).unwrap();
    hands
        .iter()
        .filter(|h| h.score == max_hand.score)
        .map(|h| h.raw)
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hand {
    pub cards: Vec<u8>,
    pub bid: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

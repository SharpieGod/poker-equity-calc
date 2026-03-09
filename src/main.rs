use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Suit {
    Diamonds,
    Spades,
    Hearts,
    Clubs,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { suit, rank }
    }

    fn from_str(str: &str) -> Self {
        let suit = str.chars().nth(1).unwrap();
        let rank = str.chars().nth(0).unwrap();

        let SUIT_MAP: HashMap<char, Suit> = {
            let mut m = HashMap::new();
            m.insert('c', Suit::Clubs);
            m.insert('d', Suit::Diamonds);
            m.insert('h', Suit::Hearts);
            m.insert('s', Suit::Spades);
            m
        };
        let RANK_MAP: HashMap<char, Rank> = {
            let mut m = HashMap::new();
            m.insert('2', Rank::Two);
            m.insert('3', Rank::Three);
            m.insert('4', Rank::Four);
            m.insert('5', Rank::Five);
            m.insert('6', Rank::Six);
            m.insert('7', Rank::Seven);
            m.insert('8', Rank::Eight);
            m.insert('9', Rank::Nine);
            m.insert('t', Rank::Ten);
            m.insert('j', Rank::Jack);
            m.insert('q', Rank::Queen);
            m.insert('k', Rank::King);
            m.insert('a', Rank::Ace);
            m
        };

        Card {
            suit: SUIT_MAP[&suit],
            rank: RANK_MAP[&rank],
        }
    }
}
fn main() {
    let hand = Vec::from([Card::from_str("ah"), Card::from_str("ad")]);
    let board = Vec::from([Card::from_str("ac"), Card::from_str("5d")]);

    println!("{}", eval(&hand, &board));
}

fn eval(hand: &Vec<Card>, board: &Vec<Card>) -> u32 {
    let mut comb = hand.iter().collect::<Vec<&Card>>();
    comb.extend(board);

    comb.sort_by(|a, b| {
        let a_rank = a.rank as u32;
        let b_rank = b.rank as u32;
        b_rank.cmp(&a_rank)
    });

    // Order of hands: best to worst

    let ranks = comb.iter().map(|x| x.rank).collect::<Vec<Rank>>();
    let mut ranks_map: HashMap<&Rank, i32> = HashMap::new();

    for rank in &ranks {
        *ranks_map.entry(rank).or_insert(0) += 1;
    }

    let ranks_count = ranks_map.into_iter().collect::<Vec<(&Rank, i32)>>();

    println!("{:?}", ranks_count);

    1
}

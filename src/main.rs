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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, PartialEq)]
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
    let board = Vec::from([
        Card::from_str("ac"),
        Card::from_str("5d"),
        Card::from_str("as"),
    ]);

    println!("{}", eval(&hand, &board));
}

fn eval(hand: &Vec<Card>, board: &Vec<Card>) -> u32 {
    let mut comb = hand.iter().collect::<Vec<&Card>>();
    let mut hand_type: HandType = HandType::HighCard;
    let mut best_cards: Vec<&Card>;

    comb.extend(board);

    comb.sort_by(|a, b| {
        let a_rank = a.rank as u32;
        let b_rank = b.rank as u32;
        b_rank.cmp(&a_rank)
    });

    // Order of hands: best to worst

    let mut ranks_count_map: HashMap<Rank, (u32, Vec<&Card>)> = HashMap::new();

    for card in &comb {
        let entry = ranks_count_map.entry(card.rank).or_insert((0, Vec::new()));
        entry.0 += 1;
        entry.1.push(*card);
    }

    let mut counts_rank_map: HashMap<u32, Vec<Rank>> = HashMap::new();

    for (rank, (count, _)) in &ranks_count_map {
        counts_rank_map.entry(*count).or_default().push(*rank);
    }

    if counts_rank_map.contains_key(&4) {
        // Four of a kind

        let target_rank = &counts_rank_map.get(&4).unwrap()[0];
        let mut four_of_a_kind: Vec<&Card> = ranks_count_map.get(target_rank).unwrap().1.to_vec();

        let other: Vec<&Card> = comb
            .iter()
            .filter(|c| !four_of_a_kind.contains(c))
            .copied()
            .collect();

        four_of_a_kind.push(other[0]);
        best_cards = four_of_a_kind.iter().copied().collect();
        hand_type = HandType::FourOfAKind;

        println!("{:?}", four_of_a_kind)
    } else if counts_rank_map.contains_key(&3) {
        // Full house or three of a kind

        if counts_rank_map.contains_key(&2) {
            // Full house

            let three_rank = &counts_rank_map.get(&3).unwrap()[0];
            let pair_rank = &counts_rank_map.get(&2).unwrap()[0];

            let mut three_of_a_kind: Vec<&Card> =
                ranks_count_map.get(three_rank).unwrap().1.to_vec();

            let pair: Vec<&Card> = ranks_count_map.get(pair_rank).unwrap().1.to_vec();
        } else {
            // Three of a kind
        }
    }

    // println!("{:?}", ranks_count_map);

    1
}

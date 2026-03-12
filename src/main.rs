use core::fmt;
use std::collections::HashMap;

static CARD_VALUES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

static CARD_SUITS: [&str; 4] = ["♠", "♣", "♥", "♦"];
static CARD_COLORS: [&str; 4] = ["\x1b[90m", "\x1b[32m", "\x1b[31m", "\x1b[34m"];
const RESET: &str = "\x1b[0m";

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            CARD_COLORS[self.suit as usize],
            CARD_SUITS[self.suit as usize],
            CARD_VALUES[self.rank as usize],
            RESET
        )
    }
}
impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { suit, rank }
    }

    fn from_str(str: &str) -> Self {
        let suit = str.chars().nth(1).unwrap();
        let rank = str.chars().nth(0).unwrap();

        let suit_map: HashMap<char, Suit> = {
            let mut m = HashMap::new();
            m.insert('c', Suit::Clubs);
            m.insert('d', Suit::Diamonds);
            m.insert('h', Suit::Hearts);
            m.insert('s', Suit::Spades);
            m
        };
        let rank_map: HashMap<char, Rank> = {
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
            suit: suit_map[&suit],
            rank: rank_map[&rank],
        }
    }
}

struct CardTracker {
    internal_cards: Vec<Card>,
    internal_rank_map: HashMap<Rank, Vec<Card>>,
    internal_counter: HashMap<u32, Vec<Rank>>,
}

impl CardTracker {
    fn from(cards: &[Card]) -> CardTracker {
        let mut ranks_count_map: HashMap<Rank, Vec<Card>> = HashMap::new();

        for card in cards {
            let entry = ranks_count_map.entry(card.rank).or_default();
            entry.push(*card);
        }

        let mut counts_rank_map: HashMap<u32, Vec<Rank>> = HashMap::new();

        for (rank, cards) in &ranks_count_map {
            counts_rank_map
                .entry(cards.len() as u32)
                .or_default()
                .push(*rank);
        }

        CardTracker {
            internal_cards: cards.to_vec(),
            internal_rank_map: ranks_count_map,
            internal_counter: counts_rank_map,
        }
    }

    fn cards_with_group_of(&self, count: u32, nth: usize) -> Vec<Card> {
        let target_rank = &self.internal_counter.get(&count).unwrap()[nth];
        self.internal_rank_map.get(target_rank).unwrap().to_vec()
    }

    fn contains(&self, count: u32) -> bool {
        self.internal_counter.contains_key(&count)
    }

    fn contains_multiple(&self, counts: &[u32]) -> bool {
        let mut tracking_counter = self.internal_counter.clone();

        for count in counts {
            if !tracking_counter.contains_key(count) {
                return false;
            }

            match tracking_counter.get_mut(count) {
                Some(vec) if !vec.is_empty() => {
                    vec.pop();
                }
                _ => return false,
            };
        }

        true
    }

    fn cards_without(&self, filter: &Vec<Card>, take: usize) -> Vec<Card> {
        self.internal_cards
            .iter()
            .filter(|x| !filter.contains(x))
            .copied()
            .take(take)
            .collect::<Vec<Card>>()
    }

    fn filter_suit(&self, suit: &Suit) -> Vec<Card> {
        self.internal_cards
            .iter()
            .filter(|x| x.suit == *suit)
            .copied()
            .collect()
    }
}

fn main() {
    let mut all_cards: Vec<Card> = Vec::new();

    for rank in 0..13 {
        for suit in 0..4 {
            all_cards.push(Card {
                rank: unsafe { std::mem::transmute(rank as u8) },
                suit: unsafe { std::mem::transmute(suit as u8) },
            });
        }
    }

    let mut out: Vec<Vec<usize>> = Vec::new();

    comb(&mut Vec::new(), 0, 51, 5, &mut out);

    for c in out {
        let cards: Vec<Card> = c.iter().map(|x| all_cards[*x]).collect();

        eval(&cards, &Vec::new());
    }
}

fn eval(hand: &Vec<Card>, board: &Vec<Card>) -> u64 {
    let mut comb = hand.to_vec();
    let mut hand_type: HandType = HandType::HighCard;
    let mut best_cards: Vec<Card> = Vec::new();

    comb.extend(board);

    comb.sort_by(|a, b| {
        let a_rank = a.rank as u32;
        let b_rank = b.rank as u32;
        b_rank.cmp(&a_rank)
    });

    // Order of hands: best to worst

    let card_tracker = CardTracker::from(comb.as_slice());

    if card_tracker.contains(4) {
        // Four of a kind

        let four_of_a_kind: Vec<Card> = card_tracker.cards_with_group_of(4, 0);
        let other: Vec<Card> = card_tracker.cards_without(&four_of_a_kind, 1);

        hand_type = HandType::FourOfAKind;
        best_cards.clear();
        best_cards.extend(four_of_a_kind);
        best_cards.extend(other);
    } else if card_tracker.contains(3) {
        // Full house or three of a kind

        if card_tracker.contains(2) {
            // Full house

            let three: Vec<Card> = card_tracker.cards_with_group_of(3, 0);
            let two: Vec<Card> = card_tracker.cards_with_group_of(2, 0);

            hand_type = HandType::FullHouse;
            best_cards.clear();
            best_cards.extend(three);
            best_cards.extend(two);
        } else {
            // Three of a kind
            let three: Vec<Card> = card_tracker.cards_with_group_of(3, 0);
            let other: Vec<Card> = card_tracker.cards_without(&three, 2);

            hand_type = HandType::ThreeOfAKind;
            best_cards.clear();
            best_cards.extend(three);
            best_cards.extend(other);
        }
    } else if card_tracker.contains(2) {
        // Pair or two pair

        if card_tracker.contains_multiple(&[2, 2]) {
            // two pair
            let first_two = card_tracker.cards_with_group_of(2, 0);
            let second_two = card_tracker.cards_with_group_of(2, 1);

            best_cards.clear();
            best_cards.extend(first_two);
            best_cards.extend(second_two);
            let other = card_tracker.cards_without(&best_cards, 1);
            best_cards.extend(other);

            hand_type = HandType::TwoPair;
        } else {
            // Pair
            let two = card_tracker.cards_with_group_of(2, 0);
            let other = card_tracker.cards_without(&two, 3);
            best_cards.clear();
            best_cards.extend(two);
            best_cards.extend(other);

            hand_type = HandType::Pair;
        }
    }

    if hand_type < HandType::Flush {
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            let filtered = card_tracker.filter_suit(&suit);

            if filtered.len() < 5 {
                continue;
            }

            best_cards.clear();
            best_cards.extend(filtered.iter().take(5).copied().collect::<Vec<Card>>());
            hand_type = HandType::Flush;

            break;
        }
    }

    if hand_type < HandType::Straight {
        let mut l = 0;
        let mut r;
        let mut is_wheel = false;
        let mut is_straight = false;
        let mut straight_start;
        let mut ranks = comb.iter().map(|x| x.rank).collect::<Vec<Rank>>();
        ranks.dedup();

        while l < ranks.len() {
            r = l + 1;

            while r < ranks.len()
                && (ranks[r] as i32 - ranks[r - 1] as i32 == -1
                    || ranks[r] == Rank::Two && ranks[0] == Rank::Ace)
            {
                r += 1;
            }

            if r - l + 1 >= 5 {
                // Straight

                is_straight = true;
                is_wheel = r == ranks.len();
                straight_start = ranks[l];
                break;
            }

            l = r;
        }

        // if is_straight {
        //     best_cards.clear();
        //     hand_type = HandType::Straight;
        // }
    }

    if hand_type == HandType::HighCard {
        best_cards.extend(comb.iter().take(5));
    }
    // println!(
    //     "{:?} {}",
    //     hand_type,
    //     best_cards
    //         .iter()
    //         .map(|c| c.to_string())
    //         .collect::<Vec<_>>()
    //         .join(" "),
    // );

    if best_cards.len() == 4 {
        println!("{:?} {:?}", best_cards, hand_type);
    }

    let calc: u64 = 15 * best_cards[4].rank as u64
        + 15_u64.pow(2) * best_cards[3].rank as u64
        + 15_u64.pow(3) * best_cards[2].rank as u64
        + 15_u64.pow(4) * best_cards[1].rank as u64
        + 15_u64.pow(5) * best_cards[0].rank as u64
        + 15_u64.pow(6) * hand_type as u64;

    calc
}

fn comb(curr: &mut Vec<usize>, start: usize, end: usize, n: usize, out: &mut Vec<Vec<usize>>) {
    if curr.len() == n {
        out.push(curr.clone());
        return;
    }

    for i in start..end {
        curr.push(i);
        comb(curr, i + 1, end, n, out);
        curr.pop();
    }
}

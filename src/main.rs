use core::fmt;
use rand::seq::SliceRandom;
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Instant,
};

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

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
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

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            HandType::HighCard => "High Card",
            HandType::Pair => "Pair",
            HandType::TwoPair => "Two Pair",
            HandType::ThreeOfAKind => "Three of a Kind",
            HandType::Straight => "Straight",
            HandType::Flush => "Flush",
            HandType::FullHouse => "Full House",
            HandType::FourOfAKind => "Four of a Kind",
            HandType::StraightFlush => "Straight Flush",
            HandType::RoyalFlush => "Royal Flush",
        };
        write!(f, "{}", s)
    }
}
impl Card {
    fn from_str(str: &str) -> Option<Self> {
        let suit = str.chars().nth(1).unwrap_or_default();
        let rank = str.chars().nth(0).unwrap_or_default();

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

        if !suit_map.contains_key(&suit) || !rank_map.contains_key(&rank) {
            return None;
        }

        Some(Card {
            suit: suit_map[&suit],
            rank: rank_map[&rank],
        })
    }

    fn from_list(l: &[&str]) -> Option<Vec<Card>> {
        let mut out = Vec::new();

        for s in l {
            if let Some(c) = Card::from_str(s) {
                out.push(c);
            } else {
                return None;
            }
        }

        Some(out)
    }
}

struct CardTracker {
    internal_cards: Vec<Card>,
    count_ranks: [Vec<Rank>; 5],
}

impl CardTracker {
    fn from(cards: &[Card]) -> CardTracker {
        let mut rank_counts = [0u8; 13];
        for card in cards {
            rank_counts[card.rank as usize] += 1;
        }

        let mut count_ranks: [Vec<Rank>; 5] = Default::default();
        for (rank_idx, &count) in rank_counts.iter().enumerate() {
            if count > 0 {
                count_ranks[count as usize]
                    .push(unsafe { std::mem::transmute::<u8, Rank>(rank_idx as u8) });
            }
        }

        // sort descending — replaces the HashMap sort
        for v in count_ranks.iter_mut() {
            v.sort_by_key(|b| std::cmp::Reverse(*b as u8));
        }

        CardTracker {
            internal_cards: cards.to_vec(),
            count_ranks,
        }
    }

    fn contains(&self, count: u32) -> bool {
        !self.count_ranks[count as usize].is_empty()
    }

    fn contains_multiple(&self, counts: &[u32]) -> bool {
        // no clone needed anymore — just check lengths
        let mut needed = [0u8; 5];
        for &c in counts {
            needed[c as usize] += 1;
        }
        needed
            .iter()
            .enumerate()
            .all(|(i, &n)| self.count_ranks[i].len() >= n as usize)
    }

    fn cards_with_group_of(&self, count: u32, nth: usize) -> Vec<Card> {
        let rank = self.count_ranks[count as usize][nth];
        self.internal_cards
            .iter()
            .filter(|c| c.rank == rank)
            .copied()
            .collect()
    }

    fn cards_without(&self, filter: &[Card], take: usize) -> Vec<Card> {
        self.internal_cards
            .iter()
            .filter(|x| !filter.contains(x))
            .copied()
            .take(take)
            .collect()
    }

    fn filter_suit(&self, suit: &Suit) -> Vec<Card> {
        self.internal_cards
            .iter()
            .filter(|x| x.suit == *suit)
            .copied()
            .collect()
    }

    fn get_with_rank(&self, rank: Rank) -> Card {
        self.internal_cards
            .iter()
            .find(|c| c.rank == rank)
            .copied()
            .unwrap()
    }
}
#[derive(Debug, Clone, Copy)]
struct EvalResult {
    score: u64,
    hand_type: HandType,
}

struct ResultsManager {
    results: HashMap<u64, HashMap<u8, EvalResult>>,
}

#[derive(Debug, Clone)]
struct AggResult {
    hand_counter: HashMap<u8, HashMap<HandType, u64>>,
    eq_counter: HashMap<u8, f64>,
    count: u64,
    ties: HashMap<Vec<u8>, u64>,
}

impl ResultsManager {
    fn new() -> Self {
        ResultsManager {
            results: HashMap::new(),
        }
    }

    fn insert(&mut self, board: u64, player_key: u8, result: EvalResult) {
        let player_map = self.results.entry(board).or_default();
        player_map.insert(player_key, result);
    }

    fn agg(&self, board: &[Card]) -> AggResult {
        let mut cards = board.to_vec();
        cards.sort_by(|a, b| {
            let a_rank = a.rank as u32;
            let b_rank = b.rank as u32;
            b_rank.cmp(&a_rank)
        });

        let mut hand_counter: HashMap<u8, HashMap<HandType, u64>> = HashMap::new();
        let mut eq_counter = HashMap::new();
        let mut count = 0_u64;
        let mut ties = HashMap::new();

        let filter_cards: Vec<u64> = board.iter().map(encode_card).collect();

        for (encoded_board, result) in &self.results {
            // check every filter card exists somewhere in the 5 slots
            let matches = filter_cards
                .iter()
                .all(|&card_enc| (0..5).any(|i| (encoded_board >> (i * 6)) & 0x3F == card_enc));

            if !matches {
                continue;
            }

            count += 1;
            let best_score = result.values().map(|v| v.score).max().unwrap();

            for (player_key, player_result) in result {
                *hand_counter
                    .entry(*player_key)
                    .or_default()
                    .entry(player_result.hand_type)
                    .or_default() += 1;
            }

            let mut winners = result
                .iter()
                .filter(|p| p.1.score == best_score)
                .map(|p| *p.0)
                .collect::<Vec<u8>>();

            if winners.len() == 1 {
                *eq_counter.entry(winners[0]).or_insert(0_f64) += 1_f64;
            } else {
                winners.sort();
                *ties.entry(winners).or_default() += 1;
            }
        }

        AggResult {
            hand_counter,
            eq_counter,
            count,
            ties,
        }
    }
}

struct Player {
    hand: [Card; 2],
    player_key: u8,
}

impl Player {
    fn initiate(hand: [&str; 2], player_key: u8) -> Option<Player> {
        if let Some(cards) = Card::from_list(&hand) {
            return Some(Player {
                hand: cards.try_into().unwrap(),
                player_key,
            });
        }

        None
    }
}

fn take_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or_default();

    input.trim().to_lowercase().to_string()
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn encode_card(card: &Card) -> u64 {
    (card.rank as u64) * 4 + (card.suit as u64)
}

fn encode_board(cards: &[Card]) -> u64 {
    cards
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, card)| acc | (encode_card(card) << (i * 6)))
}

fn print_rainbow(text: &str) {
    for (i, ch) in text.chars().enumerate() {
        let hue = (i * 5) % 360;
        let (r, g, b) = hsl_to_rgb(hue as f32, 0.5, 0.5);
        print!("\x1b[38;2;{r};{g};{b}m{ch}");
    }
    print!("\x1b[0m");
    println!();
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn main() {
    clear_screen();

    let mut all_cards: Vec<Card> = Vec::new();

    for rank in 0..13 {
        for suit in 0..4 {
            all_cards.push(Card {
                rank: unsafe { std::mem::transmute::<u8, Rank>(rank as u8) },
                suit: unsafe { std::mem::transmute::<u8, Suit>(suit as u8) },
            });
        }
    }

    let player_count: u8;
    let mut player_cards: Vec<Card> = Vec::new();

    clear_screen();

    loop {
        print_rainbow(
            r"            ____       _               _____            _ _         
           |  _ \ ___ | | _____ _ __  | ____|__ _ _   _(_) |_ _   _ 
           | |_) / _ \| |/ / _ \ '__| |  _| / _` | | | | | __| | | |
           |  __/ (_) |   <  __/ |    | |__| (_| | |_| | | |_| |_| |
           |_|   \___/|_|\_\___|_|    |_____\__, |\__,_|_|\__|\__, |
                                               |_|            |___/ 
                  ____      _            _       _             
                 / ___|__ _| | ___ _   _| | __ _| |_ ___  _ __ 
                | |   / _` | |/ __| | | | |/ _` | __/ _ \| '__|
                | |__| (_| | | (__| |_| | | (_| | || (_) | |   
                 \____\__,_|_|\___|\__,_|_|\__,_|\__\___/|_|   
                 
                 ",
        );
        print!("Enter number of players: ");

        io::stdout().flush().unwrap();
        let input = take_input();

        if let Ok(n) = input.parse::<u8>()
            && n > 0
            // 23 is max number of players (52 - 5 community cards) // 2 = 23
            && n <= 23
        {
            player_count = n;
            break;
        }

        clear_screen();
    }
    clear_screen();

    let mut players: Vec<Player> = vec![];
    let mut rng = rand::thread_rng();

    for player_key in 0..player_count {
        loop {
            clear_screen();
            println!(
                "{}\n",
                (0..player_count)
                    .map(|n| (n, players.get(n as usize).map(|p| p.hand)))
                    .map(|p| format!(
                        "player {}: {}",
                        p.0 + 1,
                        match p.1 {
                            Some(hand) => hand.map(|c| c.to_string()).join(" "),
                            _ => "".to_string(),
                        }
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            println!(
                "Enter hand for player {} (h for help, r for random): ",
                player_key + 1
            );

            let input = take_input();

            if input.is_empty() {
                continue;
            }

            if input == "h" {
                clear_screen();
                println!("Ranks are: 1 2 3 4 5 6 7 8 9 t j q k a");
                println!("Suits are: s h d c");
                println!("\nPlayer hand consits of: Rank Suit space Rank Suit\n");
                println!("Pocket Aces: ah ad");
                println!("Jack Ten Suited: jc tc");
                println!("Seven Deuce: 7c 2s");
                println!("\nPress enter to continue");
                take_input();
            } else if input == "r" {
                let hand: [Card; 2] = all_cards
                    .iter()
                    .filter(|c| !player_cards.contains(*c))
                    .copied()
                    .collect::<Vec<Card>>()
                    .choose_multiple(&mut rng, 2)
                    .copied()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                player_cards.extend_from_slice(&hand);
                players.push(Player { hand, player_key });

                break;
            } else if let Some(player) = Player::initiate(
                input
                    .split_whitespace()
                    .take(2)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_default(),
                player_key,
            ) && player.hand.iter().all(|c| !player_cards.contains(c))
            {
                player_cards.extend_from_slice(&player.hand);
                players.push(player);

                break;
            }

            clear_screen();
        }
        clear_screen();
    }

    let mut combinations = Vec::new();

    all_cards = all_cards
        .iter()
        .filter(|x| !player_cards.contains(x))
        .copied()
        .collect();

    let mut results_manager = ResultsManager::new();

    comb(&mut Vec::new(), 0, all_cards.len(), 5, &mut combinations);

    let start = Instant::now();
    println!("Loading...");

    for c in &combinations {
        for player in &players {
            let mut cards: Vec<Card> = c.iter().map(|x| all_cards[*x]).collect();
            cards.sort_by(|a, b| {
                let a_rank = a.rank as u32;
                let b_rank = b.rank as u32;
                b_rank.cmp(&a_rank)
            });

            let result = eval(&player.hand, &cards);
            results_manager.insert(encode_board(&cards), player.player_key, result);
        }
    }

    println!(
        "Finished loading in {}s",
        (start.elapsed().as_millis() as f64 / 10_f64).round() / 100_f64
    );
    println!("\nPress enter to continue");

    take_input();
    let mut board: Vec<Card> = Vec::new();
    let mut needs_refresh = true;
    let mut agg_result: AggResult = AggResult {
        hand_counter: HashMap::new(),
        eq_counter: HashMap::new(),
        count: 1,
        ties: HashMap::new(),
    };

    // Ways to arrange group of 5..=0 to count total amount of permutations without computing them
    let fact = [120, 24, 6, 2, 1, 1];
    let mut base_agg = None;
    loop {
        clear_screen();
        if needs_refresh
            && board.is_empty()
            && let Some(agg) = base_agg.clone()
        {
            agg_result = agg;
            needs_refresh = false;
        } else if needs_refresh {
            let mut sorted_board = board.clone();
            sorted_board.sort_by(|a, b| {
                let a_rank = a.rank as u32;
                let b_rank = b.rank as u32;
                b_rank.cmp(&a_rank)
            });
            agg_result = results_manager.agg(&sorted_board);

            if board.is_empty() {
                base_agg = Some(agg_result.clone());
            }

            needs_refresh = false;
        }
        println!(
            "{}",
            match board
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
            {
                s if s.is_empty() => "Board is empty.".to_string(),
                s => s,
            }
        );

        println!();

        for player in &players {
            let equity = agg_result
                .eq_counter
                .get(&player.player_key)
                .unwrap_or(&0_f64);
            println!(
                "({}) player {}: {} {}%",
                player.player_key + 1,
                player.player_key + 1,
                player.hand.map(|x| x.to_string()).join(" "),
                (*equity / agg_result.count as f64 * 100_f64 * 100_f64).round() / 100_f64
            );
        }

        let mut ties = agg_result.ties.iter().collect::<Vec<_>>();
        ties.sort_by(|a, b| a.0.len().cmp(&b.0.len()).reverse());

        for tie in ties {
            println!(
                "{}-way tie{}: {}%",
                tie.0.len(),
                if tie.0.len() == players.len() {
                    String::new()
                } else {
                    format!(
                        " ({})",
                        tie.0
                            .iter()
                            .map(|p| (p + 1).to_string())
                            .collect::<Vec<_>>()
                            .join("-"),
                    )
                },
                (*tie.1 as f64 / agg_result.count as f64 * 100_f64 * 100_f64).round() / 100_f64
            );
        }

        println!("\n(p to pop, r for random, h for help):");
        let input = take_input();

        if input == "h" {
            clear_screen();
            println!("Ranks are: 1 2 3 4 5 6 7 8 9 t j q k a");
            println!("Suits are: s h d c");
            println!("");
            println!("Enter a card with format: Rank Suit (no spaces) to add it to the board.");
            println!("\nAce of Hearts: ah\n3 of Clubs: 3c\nTen of Spades: ts");
            println!("Enter player number to view made hand breakdown.");
            println!("\nPress enter to continue");
            take_input();
            continue;
        }

        if input == "r" && board.len() < 5 {
            let new_card = all_cards
                .iter()
                .filter(|c| !board.contains(c))
                .copied()
                .collect::<Vec<_>>()
                .choose(&mut rng)
                .unwrap()
                .clone();

            board.push(new_card);
            needs_refresh = true;
        }

        if input == "p" && !board.is_empty() {
            board.pop();
            needs_refresh = true;
        }

        if players
            .iter()
            .map(|p| (p.player_key + 1).to_string())
            .collect::<Vec<String>>()
            .contains(&input)
        {
            // player hands made breakdown
            let player = &players[input.parse::<usize>().unwrap() - 1];
            clear_screen();

            let mut hand_type_results: Vec<(HandType, u64)> = agg_result
                .hand_counter
                .get(&player.player_key)
                .unwrap()
                .iter()
                .map(|(&hand_type, &count)| (hand_type, count))
                .collect();

            hand_type_results.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            println!(
                "Hand breakdown for player {}: {}\n\n{}",
                player.player_key + 1,
                player.hand.map(|x| x.to_string()).join(" "),
                hand_type_results
                    .iter()
                    .map(|(hand_type, counter)| format!(
                        "{:<17} {:>5.2}% ({})",
                        format!("{}:", hand_type),
                        (*counter as f64 / agg_result.count as f64 * 100_f64 * 100_f64).round()
                            / 100_f64,
                        counter * fact[board.len()]
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            println!("\nPress enter to continue");

            take_input();
        }

        if input.len() == 2 && board.len() < 5 {
            let new_card = Card::from_str(&input);

            if let Some(c) = new_card {
                if board.contains(&c) || player_cards.contains(&c) {
                    continue;
                }

                board.push(c);
                needs_refresh = true;
            }
        }
    }
}

fn eval(hand: &[Card], board: &Vec<Card>) -> EvalResult {
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
        } else if card_tracker.contains_multiple(&[3, 3]) {
            let three = card_tracker.cards_with_group_of(3, 0);
            let two_other: Vec<Card> = card_tracker
                .cards_with_group_of(3, 1)
                .iter()
                .take(2)
                .copied()
                .collect();

            hand_type = HandType::FullHouse;
            best_cards.clear();
            best_cards.extend(three);
            best_cards.extend(two_other);
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

            // Search for straight = Straight Flush, if starts with Ace, Royal flush!!

            let mut l = 0;
            let mut r;
            let mut is_straight = false;
            let mut straight_start = Rank::Ace;
            let mut filtered_ranks = filtered.iter().map(|x| x.rank).collect::<Vec<Rank>>();
            filtered_ranks.dedup();

            while l < filtered_ranks.len() {
                r = l;

                while r < filtered_ranks.len() - 1
                    && filtered_ranks[r] as u8 - filtered_ranks[r + 1] as u8 == 1
                {
                    r += 1;
                }

                if filtered_ranks[r] == Rank::Two && filtered_ranks[0] == Rank::Ace {
                    r += 1;
                }

                if r - l + 1 >= 5 {
                    // Straight
                    is_straight = true;
                    straight_start = filtered_ranks[l];
                    break;
                }

                l = r + 1;
            }
            best_cards.clear();

            if is_straight {
                if straight_start != Rank::Five {
                    best_cards.extend(
                        (0..5)
                            .map(|n| straight_start as u8 - n)
                            .map(|rank| unsafe { std::mem::transmute::<u8, Rank>(rank) })
                            .map(|rank| Card { rank, suit })
                            .collect::<Vec<Card>>(),
                    );
                } else {
                    best_cards.extend(
                        (0..4)
                            .map(|n| straight_start as u8 - n)
                            .map(|rank| unsafe { std::mem::transmute::<u8, Rank>(rank) })
                            .map(|rank| Card { rank, suit })
                            .collect::<Vec<Card>>(),
                    );

                    best_cards.push(Card {
                        rank: Rank::Ace,
                        suit,
                    });
                }

                if straight_start == Rank::Ace {
                    hand_type = HandType::RoyalFlush;
                } else {
                    hand_type = HandType::StraightFlush;
                }
            } else {
                best_cards.extend(filtered.iter().take(5).copied().collect::<Vec<Card>>());
                hand_type = HandType::Flush;
            }

            break;
        }
    }

    if hand_type < HandType::Straight {
        let mut l = 0;
        let mut r;
        let mut is_straight = false;
        let mut straight_start = Rank::Ace;
        let mut ranks = comb.iter().map(|x| x.rank).collect::<Vec<Rank>>();
        ranks.dedup();

        while l < ranks.len() {
            r = l;

            while r < ranks.len() - 1 && ranks[r] as u8 - ranks[r + 1] as u8 == 1 {
                r += 1;
            }

            if ranks[r] == Rank::Two && ranks[0] == Rank::Ace {
                r += 1;
            }

            if r - l + 1 >= 5 {
                // Straight
                is_straight = true;
                straight_start = ranks[l];
                break;
            }

            l = r + 1;
        }

        if is_straight {
            best_cards.clear();
            hand_type = HandType::Straight;

            if straight_start != Rank::Five {
                best_cards.extend(
                    (0..5)
                        .map(|n| straight_start as u8 - n)
                        .map(|rank| unsafe { std::mem::transmute::<u8, Rank>(rank) })
                        .map(|rank| card_tracker.get_with_rank(rank))
                        .collect::<Vec<Card>>(),
                );
            } else {
                best_cards.extend(
                    (0..4)
                        .map(|n| straight_start as u8 - n)
                        .map(|rank| unsafe { std::mem::transmute::<u8, Rank>(rank) })
                        .map(|rank| card_tracker.get_with_rank(rank))
                        .collect::<Vec<Card>>(),
                );

                best_cards.push(card_tracker.get_with_rank(Rank::Ace));
            }
        }
    }

    if hand_type == HandType::HighCard {
        best_cards.extend(comb.iter().take(5));
    }

    let calc: u64 = 15 * best_cards[4].rank as u64
        + 15_u64.pow(2) * best_cards[3].rank as u64
        + 15_u64.pow(3) * best_cards[2].rank as u64
        + 15_u64.pow(4) * best_cards[1].rank as u64
        + 15_u64.pow(5) * best_cards[0].rank as u64
        + 15_u64.pow(6) * hand_type as u64;

    EvalResult {
        score: calc,
        hand_type,
    }
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

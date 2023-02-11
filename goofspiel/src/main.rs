use rand::seq::SliceRandom;

// Should be ordered.
const N_CARDS: usize = 9;
// TODO: Create a Card type
const CARDS: [i32; N_CARDS] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[derive(Debug)]
enum PlayerType {
    Human,
    Bot,
}

#[derive(Debug)]
struct Player {
    preferences: Vec<i32>,
    player_type: PlayerType,
    score: f32,
}

#[derive(PartialEq)]
enum Order {
    UNK,
    LT,
    GT,
}

struct DeconstructedPlayer<'a> {
    partial_orders: std::collections::HashMap<(i32, i32), Order>,
    cards: &'a [i32; N_CARDS],
}

impl DeconstructedPlayer {
    // TODO: Handle the i==j case better
    fn add_po(&self, i: i32, j: i32) -> () {
        self.partial_orders.insert((i, j), Order::LT);
        self.partial_orders.insert((j, i), Order::GT);
        for card in self.cards {
            if let Some(&card_i_order) = self.partial_orders.get(&(*card, i)) {
                if card_i_order == Order::LT {
                    match self.partial_orders.get(&(*card, j)) {
                        Some(&order) => assert!(order == Order::LT),
                        None => self.add_po(*card, j),
                    }
                }
            }
        }
    }
}

/// Return a decision with a random preference.
fn get_random_bot(cards: &[i32; N_CARDS]) -> Player {
    let mut preferences = cards.to_vec();
    preferences.shuffle(&mut rand::thread_rng());

    Player {
        preferences: preferences,
        player_type: PlayerType::Bot,
        score: 0f32,
    }
}

#[derive(Debug)]
struct ParseInputError;

impl std::fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unexpected format on inupt")
    }
}

impl std::error::Error for ParseInputError {}

fn get_human_pref_for_card(card: &i32) -> Result<i32, failure::Error> {
    // TODO: Make sure this is valid.
    println!("Choose value for card {}", card);
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input)?;
    Ok(input
        .strip_suffix("\n")
        .ok_or(ParseInputError)?
        .parse::<i32>()?)
}

fn get_human(cards: &[i32; N_CARDS]) -> Result<Player, failure::Error> {
    let preferences: Result<Vec<i32>, failure::Error> =
        cards.iter().map(get_human_pref_for_card).collect();

    Ok(Player {
        preferences: preferences?,
        player_type: PlayerType::Human,
        score: 0f32,
    })
}

#[derive(Debug)]
struct NoMaxExistsError;

impl std::fmt::Display for NoMaxExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "this should never happen")
    }
}

impl std::error::Error for NoMaxExistsError {}

fn allot_to_highest_bidder(card: i32, player_by_bid: Vec<(i32, &mut Player)>) -> () {
    // TODO: Should I include the types
    let best_bid = player_by_bid.iter().map(|(bid, _)| *bid).max().unwrap();

    let num_matches: i32 = player_by_bid
        .iter()
        .filter(|(bid, _)| *bid == best_bid)
        .count()
        .try_into()
        .unwrap();

    player_by_bid
        .into_iter()
        .filter(|(bid, _)| *bid == best_bid)
        .for_each(|(_, player)| (*player).score += card as f32 / num_matches as f32);
}

fn print_scores(players: &Vec<Player>) -> () {
    let output = players
        .iter()
        .enumerate()
        // TODO: Round
        .map(|(pi, player)| format!("{:?} {}: {}", player.player_type, pi, player.score))
        .collect::<Vec<String>>()
        .join(" / ");

    println!("{}", output)
}

// TODO: Do something better with cards.
fn score_players(mut players: Vec<Player>, cards: &[i32; N_CARDS]) -> Result<(), failure::Error> {
    for (ci, card) in cards.iter().enumerate() {
        println!("Bids on card {}", card);
        let mut player_by_bid: Vec<(i32, &mut Player)> = Vec::new();
        for (pi, player) in players.iter_mut().enumerate() {
            let bid = player.preferences[ci];
            println!("{:?} player {} bid {}", player.player_type, pi, bid);
            player_by_bid.push((bid, player));
        }
        allot_to_highest_bidder(*card, player_by_bid);
        print_scores(&players)
        // TODO: Pause here for input.
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    let mut all_players: Vec<Player> = (0..3).map(|_| get_random_bot(&CARDS)).collect();
    all_players.push(get_human(&CARDS).unwrap());
    score_players(all_players, &CARDS).unwrap();
}

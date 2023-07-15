# I actually don't remember how this code fits in.

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


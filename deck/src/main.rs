use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new()-> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];

        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
    
}

fn main() {
    let mut deck = Deck::new();
    println!("Starting amount of cards before dealing: {}", deck.cards.len());
    deck.shuffle();
    let hand = deck.deal(2);

    println!("Here's your hand: {:#?}", hand);
    println!("Here's your deck of cards: {:#?}. Remaining cards after dealing: {}", deck, deck.cards.len());
}

mod playing_cards {
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    pub enum Color {
        Spades,
        Hearts,
        Diamonds,
        Clubs,
    }

    #[derive(Debug, Clone)]
    pub enum Value {
        Ace,
        King,
        Queen,
        Jack,
        Number(u8),
    }

    #[derive(Debug, Clone)]
    pub struct Card {
        value: Value,
        color: Color,
    }

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            let value_ch = match self.value {
                Value::Ace => 'A',
                Value::King => 'K',
                Value::Queen => 'Q',
                Value::Jack => 'J',
                Value::Number(n) => (n + ('0' as u8)) as char,
            };
            let color_ch: char = match self.color {
                Color::Spades => 'S',
                Color::Hearts => 'H',
                Color::Diamonds => 'D',
                Color::Clubs => 'C',
            };
            write!(f, "{}{}", value_ch, color_ch)
        }
    }

    #[derive(Debug)]
    pub struct Deck {
        cards: Vec<Card>,
    }

    impl Deck {
        pub fn new() -> Deck {
            let mut cards = Vec::new();
            for color in [Color::Spades, Color::Hearts, Color::Diamonds, Color::Clubs] {
                cards.push(Card {
                    value: Value::Ace,
                    color,
                });
                cards.push(Card {
                    value: Value::King,
                    color,
                });
                cards.push(Card {
                    value: Value::Queen,
                    color,
                });
                cards.push(Card {
                    value: Value::Jack,
                    color,
                });
                for n in 2..=10 {
                    cards.push(Card {
                        value: Value::Number(n),
                        color,
                    });
                }
            }
            Deck { cards }
        }

        fn draw(&mut self) -> Option<Card> {
            let card = self.cards.last().cloned();
            self.cards.truncate(self.cards.len() - 1);
            card
        }

        pub fn len(&self) -> usize {
            self.cards.len()
        }
    }
}

use playing_cards::{Color, Value, Card, Deck};
fn main() {
    //let some_color = playing_cards::Color::Spades;
    let some_color = Color::Spades;
    println!("{:?}", some_color);
    let some_value = Value::Number(9);
    let some_card = Card {
        value: some_value,
        color: some_color,
    };
    println!("{:?}", some_card);
    println!("{}", some_card);
    let deck = Deck::new();
    println! {"{:?}", deck};
    println! {"{}", deck.len()};
}

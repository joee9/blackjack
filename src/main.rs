use std::fmt;

use enum_iterator::{all,Sequence};


#[derive(Debug,Sequence)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

#[derive(Debug,Sequence)]
enum Value {
    Ace,
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
    King
}

struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    fn to_str(&self) -> String {

        let v: &str = match self.value {
            Value::Ace =>   "1",
            Value::Two =>   "2",
            Value::Three => "3",
            Value::Four =>  "4",
            Value::Five =>  "5",
            Value::Six =>   "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine =>  "9",
            Value::Ten =>   "T",
            Value::Jack =>  "J",
            Value::Queen => "Q",
            Value::King =>  "K"
        };

        let s: &str = match self.suit {
            Suit::Club    => "C",
            Suit::Diamond => "D",
            Suit::Heart   => "H",
            Suit::Spade   => "S"
        };

        return s.to_owned() + v
    }

}

struct StandardDeck {
    cards: [Card; 52]
}

impl Default for StandardDeck {
    fn default() -> Self {
        let cards: [Card; 52] = 0;
        
    }
}

impl StandardDeck {

}


fn main() {
    println!("Hello, world!");

    let c: Card = Card {
        suit: Suit::Club,
        value: Value::Three,
    };

    println!("{}", c.to_str())
}

use core::num;
use std::fmt;
use arrayvec::ArrayVec;


#[derive(Clone, Copy)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

impl Suit {
    const VARIANTS: [Self; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
}

#[derive(Clone, Copy)]
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

impl Value {
    const VARIANTS: [Self; 13] = 
        [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King
        ];
}

struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    fn to_string(&self) -> String {

        let v: &str = match self.value {
            Value::Ace =>   "A",
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

        return v.to_owned() + s
    }

}

struct StandardDeck {
    cards: ArrayVec<Card, 52>
}

impl Default for StandardDeck {
    fn default() -> Self {
        let mut cards = ArrayVec::<Card, 52>::new();

        for s in Suit::VARIANTS.iter().copied() {
            for v in Value::VARIANTS.iter().copied() {
                cards.push(Card {
                    suit: s,
                    value: v
                })
            }
        };

        Self {cards: cards}
    }

}

struct Shoe {
    cards: Vec<Card>
}

impl Shoe {
    fn shuffled(&self, num_decks: i32) -> Self {

        let mut cards = Vec::<Card>::new();

    }
}



fn main() {

    let deck = StandardDeck::default();

    for c in deck.cards {
        println!("{}", c.to_string());
    }
}

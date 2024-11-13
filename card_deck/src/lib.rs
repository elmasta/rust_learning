use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
	pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0, 4);
        Self::translate(random_number)
	}

	pub fn translate(value: u8) -> Suit {
        match value+1 {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0, 13);
        Self::translate(random_number)
	}

	pub fn translate(value: u8) -> Rank {
        match value+1 {
            1 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            _ => Rank::Number(value),
        }
	}
}

#[derive(Debug, PartialEq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}

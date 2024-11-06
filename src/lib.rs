//! This crate has for purpose to model the cards from the French Tarot game.
//! All the card types implement the trait Card, which is used to know their
//! score points, how their rank compared to others in a trick (to know who won
//! it), and a method which can compare two cards under a certain trick theme.
//!
//! You'll find `Trump` and `ColorCard` as implementations of Card.
//! This library also supplies `Theme` which represents a trick's "theme",
//! which is used to know how cards compare and which one wins the trick.
//! `Face` and `Color` are used to represent face cards and suit colors
//! respectively.

mod errors;

use crate::errors::{PipValueError, TrumpValueError};

#[derive(PartialEq, Eq, Clone, Copy)]
/// Enum representing the four different suits colors.
pub enum Color {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Clone, Copy)]
/// Enum representing the fous different faces ranks.
pub enum Face {
    King,
    Queen,
    Knight,
    Jack,
}

pub trait Card {
    /// Method that gives the score of the current card, which can
    /// be used to calculate players scores, and know who won a Tarot game.
    ///
    /// # Example
    /// ```
    /// # use tarot::*;
    /// //is a king, scores 3.5 points
    /// let king = ColorCard::new_face(Face::Queen, Color::Hearts);
    /// assert_eq!(king.points(), 3.5);
    ///
    /// //is a pip card, scores 0.5 points
    /// let pip = ColorCard::new_pip(1, Color::Hearts).unwrap();
    /// assert_eq!(pip.points(), 0.5);
    ///
    /// //is an oudler, scores 4.5 points
    /// let fool = TrumpCard::Fool;
    /// assert_eq!(fool.points(), 4.5);
    /// ```
    fn points(&self) -> f32;

    /// Method that gives the rank of the current card, according to given
    /// theme.
    ///
    /// # Example
    /// ```
    /// # use tarot::*;
    /// let hearts = Color::Hearts;
    /// let hearts_theme = Theme::Color(hearts);
    /// let hearts_king = ColorCard::new_face(Face::King, hearts);
    /// let hearts_five = ColorCard::new_pip(5, hearts).unwrap();
    ///
    /// let little_one = TrumpCard::little_one();
    ///
    /// assert!(hearts_king.rank(hearts_theme) > hearts_five.rank(hearts_theme));
    /// assert!(little_one.rank(hearts_theme) > hearts_king.rank(hearts_theme));
    /// ```
    fn rank(&self, theme: Theme) -> u8;
}

#[derive(Clone, Copy)]
/// Enum representing the Theme of a trick.
pub enum Theme {
    Trump,
    Color(Color),
}

impl Theme {
    pub fn is_color(&self) -> bool {
        matches!(self, Theme::Color(_))
    }

    pub fn is_trump(&self) -> bool {
        !self.is_color()
    }

    /// Returns the color of the theme, wrapped in an option.
    pub fn color(&self) -> Option<Color> {
        match self {
            Theme::Trump => Option::None,
            Theme::Color(c) => Option::Some(*c),
        }
    }

    /// Returns the color of the theme.
    /// # Panic
    /// In the case where the theme is not color.
    pub fn color_checked(&self) -> Color {
        match self {
            Theme::Trump => panic!("color asked for a trump trick"),
            Theme::Color(c) => *c,
        }
    }
}

/// Represents a color card, i.e. not a trump card.
/// # Example
/// ```
/// # use tarot::*;
/// let king_of_spades = ColorCard::new_face(Face::King, Color::Spades);
///
/// let seven_of_diamonds = ColorCard::new_pip(7, Color::Diamonds);
/// ```
pub struct ColorCard {
    color: Color,
    face: Option<Face>,
    number: u8,
}

impl ColorCard {
    /// Creates a new face card.
    pub fn new_face(face: Face, color: Color) -> Self {
        ColorCard {
            color,
            face: Option::Some(face),
            number: match face {
                Face::King => 14,
                Face::Queen => 13,
                Face::Knight => 12,
                Face::Jack => 11,
            },
        }
    }

    /// Creates a new pip card.
    /// Returns an error in case of invalid value given for number argument (ie. equals 0 or is greater than max pip value : 10)
    pub fn new_pip(number: u8, color: Color) -> Result<Self, PipValueError> {
        if number > 0 && number < 11 {
            Ok(ColorCard {
                color,
                number,
                face: None,
            })
        } else {
            Err(PipValueError::new(number))
        }
    }

    /// Returns the color of the card.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Returns the current card's number.
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Returns whether this card is a face card.
    pub fn is_face(&self) -> bool {
        self.face.is_some()
    }

    /// Returns whether this card is a pip card.
    pub fn is_pip(&self) -> bool {
        !self.is_face()
    }

    /// Returns the current card's optional face
    pub fn face(&self) -> Option<Face> {
        self.face
    }

    /// Returns the unwrapped current card's optional face.
    pub fn face_checked(&self) -> Face {
        self.face.unwrap()
    }
}

/// Enum representing a trump card.
/// # Example
/// ```
/// # use tarot::*;
/// let fool = TrumpCard::Fool;
///
/// let fifteen_of_trumps = TrumpCard::new_trump_card(15).unwrap();
/// ```
pub enum TrumpCard {
    Number(u8),
    Fool,
}

impl TrumpCard {
    /// Creates a new pip card.
    /// Returns an error in case of invalid value given for number argument (ie. equals 0 or is greater than max trump value : 21)
    pub fn new_trump_card(value: u8) -> Result<Self, TrumpValueError> {
        if value > 0 && value < 22 {
            Ok(TrumpCard::Number(value))
        } else {
            Err(TrumpValueError::new(value))
        }
    }

    /// Returns the oudler 1 of trumps, also called the "little one".
    pub fn little_one() -> Self {
        TrumpCard::Number(1)
    }

    /// Returns the oudler 21 of trumps, also called "the world".
    pub fn the_world() -> Self {
        TrumpCard::Number(21)
    }

    /// Returns whether the current trump card is the fool.
    pub fn is_fool(&self) -> bool {
        matches!(self, Self::Fool)
    }

    /// Returns whether the current trump card is an oudler.
    pub fn is_oudler(&self) -> bool {
        match self {
            TrumpCard::Fool => true,
            TrumpCard::Number(n) => *n == 1 || *n == 21,
        }
    }

    /// Returns the optional number of the trump card.
    pub fn number(&self) -> Option<u8> {
        match self {
            TrumpCard::Number(n) => Option::Some(*n),
            _ => Option::None,
        }
    }
}

impl Card for ColorCard {
    fn points(&self) -> f32 {
        if self.is_face() {
            match self.face().unwrap() {
                Face::King => 4.5,
                Face::Queen => 3.5,
                Face::Knight => 2.5,
                Face::Jack => 1.5,
            }
        } else {
            0.5
        }
    }

    fn rank(&self, theme: Theme) -> u8 {
        if theme.is_color() && self.color == theme.color_checked() {
            self.number
        } else {
            0
        }
    }
}

impl Card for TrumpCard {
    fn points(&self) -> f32 {
        if self.is_oudler() {
            4.5
        } else {
            0.5
        }
    }

    fn rank(&self, theme: Theme) -> u8 {
        match self {
            TrumpCard::Fool => 0,
            TrumpCard::Number(n) => {
                if theme.is_color() {
                    14 + *n
                } else {
                    *n
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fool() {
        let mut trump = TrumpCard::Fool;
        assert!(trump.is_fool());
        trump = TrumpCard::Number(1);
        assert!(!trump.is_fool());
    }

    #[test]
    fn test_is_oudler() {
        let mut trump = TrumpCard::Fool;
        assert!(trump.is_oudler());

        trump = TrumpCard::Number(1);
        assert!(trump.is_oudler());

        trump = TrumpCard::Number(21);
        assert!(trump.is_oudler());

        for i in 2..20 {
            trump = TrumpCard::Number(i);
            assert!(!trump.is_oudler());
        }
    }

    #[test]
    fn test_pip_value_error() {
        let wrong_pip_result = ColorCard::new_pip(32, Color::Hearts);
        assert!(wrong_pip_result.is_err());

        let wrong_pip_result = ColorCard::new_pip(0, Color::Hearts);
        assert!(wrong_pip_result.is_err());
    }

    #[test]
    fn test_trump_value_error() {
        let wrong_trump_value = TrumpCard::new_trump_card(23);
        assert!(wrong_trump_value.is_err());

        let wrong_trump_value = TrumpCard::new_trump_card(0);
        assert!(wrong_trump_value.is_err());
    }

    #[test]
    fn test_ranks() {
        let higher_face = ColorCard::new_face(Face::King, Color::Hearts);
        let lower_face = ColorCard::new_face(Face::Knight, Color::Hearts);
        let higher_pip = ColorCard::new_pip(8, Color::Hearts).unwrap();
        let lower_pip = ColorCard::new_pip(2, Color::Hearts).unwrap();
        let out_of_theme = ColorCard::new_face(Face::King, Color::Spades);
        let little_one = TrumpCard::little_one();
        let the_world = TrumpCard::the_world();
        let fool = TrumpCard::Fool;

        let trump_theme = Theme::Trump;
        let color_theme = Theme::Color(Color::Hearts);

        assert_eq!(higher_face.rank(trump_theme), 0);
        assert_eq!(higher_pip.rank(trump_theme), 0);
        assert_eq!(fool.rank(trump_theme), 0);
        assert_eq!(little_one.rank(trump_theme), 1);
        assert_eq!(the_world.rank(trump_theme), 21);

        assert_eq!(higher_face.rank(color_theme), 14);
        assert_eq!(lower_face.rank(color_theme), 12);
        assert_eq!(little_one.rank(color_theme), 15);
        assert_eq!(the_world.rank(color_theme), 35);
        assert_eq!(lower_pip.rank(color_theme), 2);
        assert_eq!(out_of_theme.rank(color_theme), 0);
        assert_eq!(fool.rank(color_theme), 0);
    }

    #[test]
    fn test_points() {
        let king = ColorCard::new_face(Face::King, Color::Hearts);
        assert_eq!(king.points(), 4.5);
        let queen = ColorCard::new_face(Face::Queen, Color::Hearts);
        assert_eq!(queen.points(), 3.5);
        let knight = ColorCard::new_face(Face::Knight, Color::Hearts);
        assert_eq!(knight.points(), 2.5);
        let jack = ColorCard::new_face(Face::Jack, Color::Hearts);
        assert_eq!(jack.points(), 1.5);

        for i in 1..10 {
            let pip = ColorCard::new_pip(i, Color::Hearts).unwrap();
            assert_eq!(pip.points(), 0.5);
        }

        let oudlers = vec![
            TrumpCard::Fool,
            TrumpCard::little_one(),
            TrumpCard::the_world(),
        ];

        for oudler in oudlers {
            assert_eq!(oudler.points(), 4.5);
        }

        for i in 2..20 {
            let trump = TrumpCard::new_trump_card(i).unwrap();
            assert_eq!(trump.points(), 0.5);
        }
    }
}

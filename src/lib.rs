#![allow(dead_code)]

#[cfg(any(not(windows), all(windows, not(feature = "fallbacks"))))]
mod universal {
    pub const TICK: &str = "✔";
    pub const CROSS: &str = "✖";
    pub const STAR: &str = "★";
    pub const SQUARE: &str = "▇";
    pub const SQUARE_SMALL: &str = "◻";
    pub const SQUARE_SMALL_FILLED: &str = "◼";
    pub const PLAY: &str = "▶";
    pub const CIRCLE: &str = "◯";
    pub const CIRCLE_FILLED: &str = "◉";
    pub const CIRCLE_DOTTED: &str = "◌";
    pub const CIRCLE_DOUBLE: &str = "◎";
    pub const CIRCLE_CIRCLE: &str = "ⓞ";
    pub const CIRCLE_CROSS: &str = "ⓧ";
    pub const CIRCLE_PIPE: &str = "Ⓘ";
    pub const CIRCLE_QUESTION_MARK: &str = "?";
    pub const BULLET: &str = "●";
    pub const DOT: &str = "․";
    pub const LINE: &str = "─";
    pub const ELLIPSIS: &str = "…";
    pub const POINTER: &str = "❯";
    pub const POINTER_SMALL: &str = "›";
    pub const INFO: &str = "ℹ";
    pub const WARNING: &str = "⚠";
    pub const HAMBURGER: &str = "☰";
    pub const SMILEY: &str = "㋡";
    pub const MUSTACHE: &str = "෴";
    pub const HEART: &str = "♥";
    pub const NODEJS: &str = "⬢";
    pub const ARROW_UP: &str = "↑";
    pub const ARROW_DOWN: &str = "↓";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_RIGHT: &str = "→";
    pub const RADIO_ON: &str = "◉";
    pub const RADIO_OFF: &str = "◯";
    pub const CHECKBOX_ON: &str = "☒";
    pub const CHECKBOX_OFF: &str = "☐";
    pub const CHECKBOX_CIRCLE_ON: &str = "ⓧ";
    pub const CHECKBOX_CIRCLE_OFF: &str = "Ⓘ";
    pub const QUESTION_MARK_PREFIX: &str = "?";
    pub const ONE_HALF: &str = "½";
    pub const ONE_THIRD: &str = "⅓";
    pub const ONE_QUARTER: &str = "¼";
    pub const ONE_FIFTH: &str = "⅕";
    pub const ONE_SIXTH: &str = "⅙";
    pub const ONE_SEVENTH: &str = "⅐";
    pub const ONE_EIGHTH: &str = "⅛";
    pub const ONE_NINTH: &str = "⅑";
    pub const ONE_TENTH: &str = "⅒";
    pub const TWO_THIRDS: &str = "⅔";
    pub const TWO_FIFTHS: &str = "⅖";
    pub const THREE_QUARTERS: &str = "¾";
    pub const THREE_FIFTHS: &str = "⅗";
    pub const THREE_EIGHTHS: &str = "⅜";
    pub const FOUR_FIFTHS: &str = "⅘";
    pub const FIVE_SIXTHS: &str = "⅚";
    pub const FIVE_EIGHTHS: &str = "⅝";
    pub const SEVEN_EIGHTHS: &str = "⅞";
}

#[cfg(any(not(windows), all(windows, not(feature = "fallbacks"))))]
pub use universal::*;

#[cfg(all(windows, feature = "fallbacks"))]
mod win {
    pub const TICK: &str = "√";
    pub const CROSS: &str = "×";
    pub const STAR: &str = "*";
    pub const SQUARE: &str = "█";
    pub const SQUARE_SMALL: &str = "[ ]";
    pub const SQUARE_SMALL_FILLED: &str = "[█]";
    pub const PLAY: &str = "►";
    pub const CIRCLE: &str = "( )";
    pub const CIRCLE_FILLED: &str = "(*)";
    pub const CIRCLE_DOTTED: &str = "( )";
    pub const CIRCLE_DOUBLE: &str = "( )";
    pub const CIRCLE_CIRCLE: &str = "(○)";
    pub const CIRCLE_CROSS: &str = "(×)";
    pub const CIRCLE_PIPE: &str = "(│)";
    pub const CIRCLE_QUESTION_MARK: &str = "(?)";
    pub const BULLET: &str = "*";
    pub const DOT: &str = ".";
    pub const LINE: &str = "─";
    pub const ELLIPSIS: &str = "...";
    pub const POINTER: &str = ">";
    pub const POINTER_SMALL: &str = "»";
    pub const INFO: &str = "i";
    pub const WARNING: &str = "‼";
    pub const HAMBURGER: &str = "≡";
    pub const SMILEY: &str = "☺";
    pub const MUSTACHE: &str = "┌─┐";
    pub const HEART: &str = "♥";
    pub const NODEJS: &str = "♦";
    pub const ARROW_UP: &str = "↑";
    pub const ARROW_DOWN: &str = "↓";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_RIGHT: &str = "→";
    pub const RADIO_ON: &str = "(*)";
    pub const RADIO_OFF: &str = "( )";
    pub const CHECKBOX_ON: &str = "[×]";
    pub const CHECKBOX_OFF: &str = "[ ]";
    pub const CHECKBOX_CIRCLE_ON: &str = "(×)";
    pub const CHECKBOX_CIRCLE_OFF: &str = "( )";
    pub const QUESTION_MARK_PREFIX: &str = "？";
    pub const ONE_HALF: &str = "1/2";
    pub const ONE_THIRD: &str = "1/3";
    pub const ONE_QUARTER: &str = "1/4";
    pub const ONE_FIFTH: &str = "1/5";
    pub const ONE_SIXTH: &str = "1/6";
    pub const ONE_SEVENTH: &str = "1/7";
    pub const ONE_EIGHTH: &str = "1/8";
    pub const ONE_NINTH: &str = "1/9";
    pub const ONE_TENTH: &str = "1/10";
    pub const TWO_THIRDS: &str = "2/3";
    pub const TWO_FIFTHS: &str = "2/5";
    pub const THREE_QUARTERS: &str = "3/4";
    pub const THREE_FIFTHS: &str = "3/5";
    pub const THREE_EIGHTHS: &str = "3/8";
    pub const FOUR_FIFTHS: &str = "4/5";
    pub const FIVE_SIXTHS: &str = "5/6";
    pub const FIVE_EIGHTHS: &str = "5/8";
    pub const SEVEN_EIGHTHS: &str = "7/8";
}

#[cfg(all(windows, feature = "fallbacks"))]
pub use win::*;

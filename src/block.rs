use std::fmt;
use std::borrow::Cow;
use std::slice::Iter;

use self::BlockPosition::*;
use color::Color;

/// The `BlockPosition` type. Tells lemonbar where to position the Block.
#[derive(PartialEq)]
pub enum BlockPosition {
    Left,
    Center,
    Right,
}

/// The block which is output to lemonbar. Typically limited to a single function.
pub struct Block<'a> {
    /// Where to align the block
    pub align:      BlockPosition,
    /// Background color
    pub bg_color:   Option<Color>,
    /// Foreground color
    pub fg_color:   Option<Color>,
    /// The block's "icon" - printed on the left of the `text`
    pub icon:       Cow<'a, str>,
    /// Main content of the block
    pub text:       Cow<'a, str>,
}

impl BlockPosition {
    /// Returns a slice::Iter with all the BlockPositions. Used for iterating through the
    /// alignments when printing the Bar.
    pub fn iter() -> Iter<'static, BlockPosition> {
        static POSITIONS: [BlockPosition; 3] = [Left, Center, Right];
        POSITIONS.iter()
    }
}

impl<'a> fmt::Display for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = if self.icon == "" {
            self.text.to_string()
        } else {
            format!("{} {}", self.icon, self.text)
        };

        let background = match self.bg_color {
            Some(ref color) => Cow::Owned(color.to_string()),
            None => Cow::Borrowed("-")
        };

        let foreground = match self.fg_color {
            Some(ref color) => Cow::Owned(color.to_string()),
            None => Cow::Borrowed("-")
        };

        write!(f, "%{{B{}}}%{{F{}}}{}%{{F-}}%{{B-}}", background, foreground, content)
    }
}

impl fmt::Display for BlockPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let position = match *self {
            BlockPosition::Left => "%{l}",
            BlockPosition::Center => "%{c}",
            BlockPosition::Right => "%{r}",
        };

        write!(f, "{}", position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use color::Color;
    use std::borrow::Cow;

    #[test]
    fn block_all_colors() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: Some(Color::rgb(0x18, 0x18, 0x18)),
            fg_color: Some(Color::rgb(0xe8, 0xe8, 0xe8)),
            icon: Cow::Borrowed("i"),
            text: Cow::Borrowed("test"),
        };

        assert_eq!(block.to_string(), "%{B#ff181818}%{F#ffe8e8e8}i test%{F-}%{B-}");
    }

    #[test]
    fn block_bg_only() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: Some(Color::rgb(0x18, 0x18, 0x18)),
            fg_color: None,
            icon: Cow::Borrowed("i"),
            text: Cow::Borrowed("test"),
        };

        assert_eq!(block.to_string(), "%{B#ff181818}%{F-}i test%{F-}%{B-}");
    }

    #[test]
    fn block_fg_only() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: None,
            fg_color: Some(Color::rgb(0xe8, 0xe8, 0xe8)),
            icon: Cow::Borrowed("i"),
            text: Cow::Borrowed("test"),
        };

        assert_eq!(block.to_string(), "%{B-}%{F#ffe8e8e8}i test%{F-}%{B-}");
    }

    #[test]
    fn block_no_colors() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: None,
            fg_color: None,
            icon: Cow::Borrowed("i"),
            text: Cow::Borrowed("test"),
        };

        assert_eq!(block.to_string(), "%{B-}%{F-}i test%{F-}%{B-}");
    }

    #[test]
    fn block_position_to_string() {
        assert_eq!(BlockPosition::Left.to_string(), "%{l}");
        assert_eq!(BlockPosition::Center.to_string(), "%{c}");
        assert_eq!(BlockPosition::Right.to_string(), "%{r}");
    }
}

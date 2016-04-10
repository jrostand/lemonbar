use std::fmt;
use std::slice::Iter;

use self::BlockPosition::*;

#[derive(PartialEq)]
pub enum BlockPosition {
    Left,
    Center,
    Right,
}

pub struct Block {
    pub align:      BlockPosition,
    pub bg_color:   Option<Color>,
    pub fg_color:   Option<Color>,
    pub icon:       String,
    pub text:       String,
}

#[derive(PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl BlockPosition {
    pub fn iter() -> Iter<'static, BlockPosition> {
        static POSITIONS: [BlockPosition; 3] = [Left, Center, Right];
        POSITIONS.iter()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = if self.icon == "" {
            self.text.to_string()
        } else {
            format!("{} {}", self.icon, self.text)
        };

        let background = match self.bg_color {
            Some(ref color) => color.to_string(),
            None => "-".to_string()
        };

        let foreground = match self.fg_color {
            Some(ref color) => color.to_string(),
            None => "-".to_string()
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Color(alpha, red, green, blue) = *self;

        write!(f, "#{:x}{:x}{:x}{:x}", alpha, red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Block, BlockPosition};

    #[test]
    fn block_all_colors() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: Some(Color(255, 0x18, 0x18, 0x18)),
            fg_color: Some(Color(255, 0xe8, 0xe8, 0xe8)),
            icon: "i".to_string(),
            text: "test".to_string(),
        };

        assert!(block.to_string() == "%{B#ff181818}%{F#ffe8e8e8}i test%{F-}%{B-}");
    }

    #[test]
    fn block_bg_only() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: Some(Color(255, 0x18, 0x18, 0x18)),
            fg_color: None,
            icon: "i".to_string(),
            text: "test".to_string(),
        };

        assert!(block.to_string() == "%{B#ff181818}%{F-}i test%{F-}%{B-}");
    }

    #[test]
    fn block_fg_only() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: None,
            fg_color: Some(Color(255, 0xe8, 0xe8, 0xe8)),
            icon: "i".to_string(),
            text: "test".to_string(),
        };

        assert!(block.to_string() == "%{B-}%{F#ffe8e8e8}i test%{F-}%{B-}");
    }

    #[test]
    fn block_no_colors() {
        let block = Block {
            align: BlockPosition::Left,
            bg_color: None,
            fg_color: None,
            icon: "i".to_string(),
            text: "test".to_string(),
        };

        assert!(block.to_string() == "%{B-}%{F-}i test%{F-}%{B-}");
    }

    #[test]
    fn block_position_to_string() {
        assert!(BlockPosition::Left.to_string() == "%{l}");
        assert!(BlockPosition::Center.to_string() == "%{c}");
        assert!(BlockPosition::Right.to_string() == "%{r}");
    }

    #[test]
    fn color_to_string() {
        let color = Color(255, 24, 24, 24);

        assert!(color.to_string() == "#ff181818");
    }
}

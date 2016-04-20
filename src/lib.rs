use std::fmt;
use std::fmt::Write;

pub mod block;
pub mod button;
pub mod color;

/// The `Bar` struct holds Blocks together and makes them all into a lemonbar string.
pub struct Bar<'a> {
    /// The Blocks, in display order, to output to lemonbar
    pub blocks: Vec<block::Block<'a>>,
}

impl<'a> fmt::Display for Bar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for position in block::BlockPosition::iter() {
            // Get only those blocks in our current position
            let blocks = self.blocks.iter().filter(|b| b.align == *position);

            // Write the position string to the buffer
            try!(write!(f, "{}", position));

            // Write the blocks to the buffer
            for block in blocks {
                try!(write!(f, "{}", block));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::borrow::Cow;

    #[test]
    fn bar_to_string() {
        let block1 = block::Block {
            align: block::BlockPosition::Left,
            bg_color: Some(color::Color::rgb(0x18, 0x18, 0x18)),
            fg_color: Some(color::Color::rgb(0xe8, 0xe8, 0xe8)),
            icon: Cow::Borrowed("icon"),
            text: Cow::Borrowed("text"),
        };

        let bar = Bar {
            blocks: vec![block1],
        };

        assert_eq!(bar.to_string(), "%{l}%{B#ff181818}%{F#ffe8e8e8}icon text%{F-}%{B-}%{c}%{r}");
    }
}

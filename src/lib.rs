use std::fmt;
use std::fmt::Write;

pub mod block;
pub mod button;

/// The `Bar` struct holds Blocks together and makes them all into a lemonbar string.
pub struct Bar {
    /// The Blocks, in display order, to output to lemonbar
    pub blocks: Vec<block::Block>,
}

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bar_string = String::new();

        for position in block::BlockPosition::iter() {
            let blocks = self.blocks.iter().filter(|b| b.align == *position);

            bar_string = format!("{}{}", bar_string, position);

            for block in blocks {
                bar_string = format!("{}{}", bar_string, block);
            }
        }

        write!(f, "{}", bar_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_to_string() {
        let block1 = block::Block {
            align: block::BlockPosition::Left,
            bg_color: Some(block::Color(0xff, 0x18, 0x18, 0x18)),
            fg_color: Some(block::Color(0xff, 0xe8, 0xe8, 0xe8)),
            icon: "icon".to_string(),
            text: "text".to_string(),
        };

        let bar = Bar {
            blocks: vec![block1],
        };

        assert!(bar.to_string() == "%{l}%{B#ff181818}%{F#ffe8e8e8}icon text%{F-}%{B-}%{c}%{r}");
    }
}

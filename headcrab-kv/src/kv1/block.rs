use std::fmt::Display;

use super::Key;

type Depth = usize;

/// A block containing keys and optionally sub-blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub name: String,
    blocks: Vec<Block>,
    pub pairs: Vec<Key>,
    depth: Depth,
}

impl Block {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blocks: Vec::new(),
            pairs: Vec::new(),
            depth: 0,
        }
    }

    pub fn add_pair(&mut self, p: Key) {
        self.pairs.push(p)
    }

    /// Update a block's and its children's depth
    fn update_depth(&mut self, depth: Depth) {
        self.depth = depth;
        for b in &mut self.blocks {
            b.update_depth(depth + 1);
        }
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.update_depth(self.depth + 1);
        self.blocks.push(block);
    }

    #[cfg(test)]
    pub(super) fn with_blocks(mut self, mut blocks: Vec<Block>) -> Self {
        for b in &mut blocks {
            b.update_depth(self.depth + 1);
        }
        self.blocks = blocks;
        self
    }

    #[cfg(test)]
    pub fn with_pairs(mut self, keys: Vec<Key>) -> Self {
        self.pairs = keys;
        self
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TAB: &'static str = "    ";
        let indent: String = std::iter::repeat(TAB).take(self.depth).collect();

        write!(f, "{indent}{}\n", self.name)?;
        write!(f, "{indent}{{\n")?;

        for p in &self.pairs {
            write!(f, "{indent}{TAB}{p}\n")?;
        }

        for b in &self.blocks {
            write!(f, "{b}")?;
        }

        write!(f, "{indent}}}\n")?;

        Ok(())
    }
}

/// A block containing keys and optionally sub-blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub name: String,
    pub blocks: Vec<Block>,
    pub pairs: Vec<super::Key>,
}

impl Block {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blocks: Vec::new(),
            pairs: Vec::new(),
        }
    }

    pub fn add_pair(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.pairs.push(super::Key::new(key, value))
    }

    pub fn with_blocks(mut self, blocks: Vec<Block>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn with_pairs(mut self, keys: Vec<super::Key>) -> Self {
        self.pairs = keys;
        self
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut block_string = vec![];

        block_string.push(format!("{}\n", self.name));
        block_string.push("{\n".to_string());

        for key in self.pairs.clone() {
            block_string.push(format!("\t{}\n", key));
        }

        for block in self.blocks.clone() {
            for string in block.to_strings() {
                block_string.push(format!("\t{}", string));
            }
        }

        block_string.push("}\n".to_string());

        block_string
    }
}

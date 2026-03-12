use crate::kv1::{Block, KV1Tree, Key};

const STRING: &'static str = r#"block1
{
    "key1" "value1"
}
block2
{
    "key2" "value2"
    block3
    {
        "key3" "value3"
    }
}
"#;

fn get_tree() -> KV1Tree {
    KV1Tree {
        blocks: vec![
            Block::new("block1").with_pairs(vec![Key::new("key1", "value1")]),
            Block::new("block2")
                .with_pairs(vec![Key::new("key2", "value2")])
                .with_blocks(vec![
                    Block::new("block3").with_pairs(vec![Key::new("key3", "value3")]),
                ]),
        ],
    }
}

#[test]
pub fn kv1_from_str() {
    assert_eq!(STRING.parse::<KV1Tree>().unwrap(), get_tree())
}

#[test]
pub fn kv1_to_str() {
    assert_eq!(get_tree().to_string(), STRING)
}

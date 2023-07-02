use crate::structs::BinaryNode;

pub fn create_tree() -> BinaryNode<u8> {
    BinaryNode {
        value: 20,
        right: Some(Box::new(BinaryNode {
            value: 50,
            right: Some(Box::new(BinaryNode {
                value: 100,
                right: None,
                left: None,
            })),
            left: Some(Box::new(BinaryNode {
                value: 30,
                right: Some(Box::new(BinaryNode {
                    value: 45,
                    right: None,
                    left: None,
                })),
                left: Some(Box::new(BinaryNode {
                    value: 29,
                    right: None,
                    left: None,
                })),
            })),
        })),
        left: Some(Box::new(BinaryNode {
            value: 10,
            right: Some(Box::new(BinaryNode {
                value: 15,
                right: None,
                left: None,
            })),
            left: Some(Box::new(BinaryNode {
                value: 5,
                right: Some(Box::new(BinaryNode {
                    value: 7,
                    right: None,
                    left: None,
                })),
                left: None,
            })),
        })),
    }
}

use crate::structs::BinaryNode;

pub fn create_tree() -> BinaryNode<u32> {
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

pub fn create_tree_2() -> BinaryNode<u32> {
    BinaryNode {
        value: 20,
        right: Some(Box::new(BinaryNode {
            value: 50,
            right: None,
            left: Some(Box::new(BinaryNode {
                value: 30,
                right: Some(Box::new(BinaryNode {
                    value: 45,
                    right: Some(Box::new(BinaryNode {
                        value: 49,
                        right: None,
                        left: None,
                    })),
                    left: None,
                })),
                left: Some(Box::new(BinaryNode {
                    value: 29,
                    right: None,
                    left: Some(Box::new(BinaryNode {
                        value: 21,
                        right: None,
                        left: None,
                    })),
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

pub struct GraphEdge {
    pub to: usize,
    pub weight: usize,
}
pub type WeightedAdjacencyList = Vec<Vec<GraphEdge>>;
pub type MatrixValue = usize;
pub type Matrix = Vec<MatrixValue>;
pub type WeightedAdjacencyMatrix = Vec<Matrix>;

pub struct BinaryNode {
    value: usize,
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

type Curr = Option<Box<BinaryNode>>;
type Path = Vec<usize>;

fn walk(curr: &Curr, path: &mut Path) {
    if curr.is_none() {
        return;
    }
    let curr = curr.as_ref().unwrap();

    path.push(curr.value);
    walk(&curr.left, path);
    walk(&curr.right, path);
}

pub fn pre_order_search(head: BinaryNode) -> Path {
    let mut path: Path = Vec::new();
    let head = Some(Box::new(head));
    walk(&head, &mut path);
    return path;
}

#[cfg(test)]
mod bt_pre_order {
    use super::*;
    #[test]
    fn it_works() {
        let tree = BinaryNode {
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
        };
        let result: Path = Vec::from([20, 10, 5, 7, 15, 50, 30, 29, 45, 100]);
        assert_eq!(pre_order_search(tree), result);
    }
}

use crate::structs::BinaryNode;

type Node = Option<Box<BinaryNode<u32>>>;

pub fn compare_bt(a: &Node, b: &Node) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    if a.is_none() || b.is_none() {
        return false;
    }

    if let Some(a) = &a {
        if let Some(b) = &b {
            if &a.value != &b.value {
                return false;
            }
        }
    }

    let a = a.as_ref().unwrap();
    let b = b.as_ref().unwrap();

    return compare_bt(&a.left, &b.left) && compare_bt(&a.right, &b.right);
}

#[cfg(test)]
mod bt_compare {
    use crate::stuff::{create_tree, create_tree_2};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            compare_bt(
                &Some(Box::new(create_tree())),
                &Some(Box::new(create_tree()))
            ),
            true
        );
        assert_eq!(
            compare_bt(
                &Some(Box::new(create_tree())),
                &Some(Box::new(create_tree_2()))
            ),
            false
        );
    }
}

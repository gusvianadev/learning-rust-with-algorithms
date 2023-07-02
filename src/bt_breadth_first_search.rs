use crate::structs::BinaryNode;

pub fn bt_breadth_first_search(head: BinaryNode<u8>, needle: u8) -> bool {
    let mut queue = vec![head];

    while queue.len() > 0 {
        let node = queue.remove(0);

        if node.value == needle {
            return true;
        };

        if let Some(left) = node.left {
            queue.push(*left);
        }
        if let Some(right) = node.right {
            queue.push(*right);
        }
    }

    false
}

#[cfg(test)]
mod bt_breadth_first_search {
    use super::*;
    use crate::stuff::create_tree;

    #[test]
    fn it_works() {
        assert_eq!(bt_breadth_first_search(create_tree(), 45), true);
        assert_eq!(bt_breadth_first_search(create_tree(), 7), true);
        assert_eq!(bt_breadth_first_search(create_tree(), 69), false);
    }
}

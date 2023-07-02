use crate::structs::BinaryNode;

fn search(curr: Option<Box<BinaryNode<u32>>>, needle: u32) -> bool {
    if let Some(curr) = curr {
        if curr.value == needle {
            return true;
        }

        if curr.value < needle {
            return search(curr.right, needle);
        }

        return search(curr.left, needle);
    }

    false
}

pub fn dfs(head: BinaryNode<u32>, needle: u32) -> bool {
    return search(Some(Box::new(head)), needle);
}

#[cfg(test)]
mod dfs_on_bst {
    use super::*;
    use crate::stuff::create_tree;

    #[test]
    fn it_works() {
        assert_eq!(dfs(create_tree(), 45), true);
        assert_eq!(dfs(create_tree(), 7), true);
        assert_eq!(dfs(create_tree(), 69), false);
    }
}

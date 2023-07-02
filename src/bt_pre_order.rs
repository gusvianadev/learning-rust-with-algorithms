use crate::structs::BinaryNode;

type Curr = Option<Box<BinaryNode<u8>>>;
type Path = Vec<u8>;

fn walk(curr: &Curr, path: &mut Path) {
    if curr.is_none() {
        return;
    }
    let curr = curr.as_ref().unwrap();

    path.push(curr.value);
    walk(&curr.left, path);
    walk(&curr.right, path);
}

pub fn pre_order_search(head: BinaryNode<u8>) -> Path {
    let mut path: Path = Vec::new();
    let head = Some(Box::new(head));
    walk(&head, &mut path);
    return path;
}

#[cfg(test)]
mod bt_pre_order {
    use crate::stuff::create_tree;

    use super::*;
    #[test]
    fn it_works() {
        let result: Path = Vec::from([20, 10, 5, 7, 15, 50, 30, 29, 45, 100]);
        assert_eq!(pre_order_search(create_tree()), result);
    }
}

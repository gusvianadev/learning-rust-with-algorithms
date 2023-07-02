use crate::structs::BinaryNode;

type Curr = Option<Box<BinaryNode<u8>>>;
type Path = Vec<u8>;

fn walk(curr: &Curr, path: &mut Path) {
    if curr.is_none() {
        return;
    }
    let curr = curr.as_ref().unwrap();

    walk(&curr.left, path);
    walk(&curr.right, path);
    path.push(curr.value);
}

pub fn post_order_search(head: BinaryNode<u8>) -> Path {
    let mut path: Path = Vec::new();
    let head = Some(Box::new(head));
    walk(&head, &mut path);
    return path;
}

#[cfg(test)]
mod bt_post_order {
    use crate::stuff::create_tree;

    use super::*;
    #[test]
    fn it_works() {
        let result: Path = Vec::from([7, 5, 15, 10, 29, 45, 30, 100, 50, 20]);
        assert_eq!(post_order_search(create_tree()), result);
    }
}

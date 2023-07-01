#![allow(unused)]

struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            value,
            next: old_head,
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.value)
    }
}

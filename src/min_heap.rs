#[derive(Debug)]
struct MinHeap {
    len: usize,
    data: Vec<u32>,
}

impl MinHeap {
    fn new() -> Self {
        Self {
            len: 0,
            data: vec![],
        }
    }

    fn parent(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child(idx: usize) -> usize {
        idx * 2 + 2
    }

    fn insert(&mut self, value: u32) {
        self.data.push(value);
        self.heapify_up(self.len);
        self.len += 1;
    }

    fn delete(&mut self) -> Option<u32> {
        if self.len == 0 {
            return None;
        }

        let out = self.data[0];
        self.len -= 1;

        if self.len == 0 {
            self.data = vec![];
            return Some(out);
        }

        self.data[0] = self.data[self.len];
        self.heapify_down(0);

        return Some(out);
    }

    fn heapify_down(&mut self, idx: usize) {
        let left_index = Self::left_child(idx);
        let right_index = Self::right_child(idx);

        if idx >= self.len || left_index >= self.len {
            return;
        }

        let left_value = self.data[left_index];
        let right_value = self.data[right_index];
        let value = self.data[idx];

        if left_value > right_value && value > right_value {
            self.data[idx] = right_value;
            self.data[right_index] = value;
            self.heapify_down(right_index);
        } else if right_value > left_value && value > left_value {
            self.data[idx] = left_value;
            self.data[left_index] = value;
            self.heapify_down(left_index);
        }
    }

    fn heapify_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let parent = Self::parent(idx);
        let parent_value = self.data[parent];
        let value = self.data[idx];

        if parent_value > value {
            self.data[idx] = parent_value;
            self.data[parent] = value;
            self.heapify_up(parent);
        }
    }
}

#[cfg(test)]
mod test_min_heap {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = MinHeap::new();

        assert_eq!(heap.len, 0);
        heap.insert(5);
        heap.insert(3);
        heap.insert(69);
        heap.insert(420);
        heap.insert(4);
        heap.insert(1);
        heap.insert(8);
        heap.insert(7);

        assert_eq!(heap.len, 8);
        assert_eq!(heap.delete(), Some(1));
        assert_eq!(heap.delete(), Some(3));
        assert_eq!(heap.delete(), Some(4));
        assert_eq!(heap.delete(), Some(5));
        assert_eq!(heap.len, 4);
        assert_eq!(heap.delete(), Some(7));
        assert_eq!(heap.delete(), Some(8));
        assert_eq!(heap.delete(), Some(69));
        assert_eq!(heap.delete(), Some(420));
        assert_eq!(heap.len, 0);
    }
}

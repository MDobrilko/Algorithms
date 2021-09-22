pub struct Heap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap { data: Vec::new() }
    }

    fn sift_up(&mut self, mut elem_idx: usize) {
        let parent = |elem_idx: usize| (elem_idx - 1) / 2;

        while elem_idx > 0 && self.data[elem_idx] < self.data[parent(elem_idx)] {
            self.data.swap(elem_idx, parent(elem_idx));
            elem_idx = parent(elem_idx);
        }
    }

    fn sift_down(&mut self, mut elem_idx: usize) {
        let left_child = |elem_idx: usize| 2 * elem_idx + 1;
        let right_child = |elem_idx: usize| 2 * elem_idx + 2;

        while left_child(elem_idx) < self.data.len() {
            let child_idx = if right_child(elem_idx) < self.data.len()
                && self.data[right_child(elem_idx)] < self.data[left_child(elem_idx)]
            {
                right_child(elem_idx)
            } else {
                left_child(elem_idx)
            };

            if self.data[elem_idx] <= self.data[child_idx] {
                break;
            } else {
                self.data.swap(elem_idx, child_idx);
                elem_idx = child_idx;
            }
        }
    }

    pub fn push(&mut self, elem: T) {
        self.data.push(elem);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let data_len = self.data.len();

        if data_len > 0 {
            self.data.swap(0, data_len - 1);
            let min_elem = self.data.pop().unwrap();
            self.sift_down(0);

            Some(min_elem)
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<&T> {
        self.data.get(0)
    }
}

/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::cmp::PartialOrd,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut child_idx = self.count;
        let mut parent_idx = self.parent_idx(child_idx);
        while (self.comparator)(&self.items[child_idx], &self.items[parent_idx]) && parent_idx > 0{
            self.items.swap(child_idx, parent_idx);
            child_idx = parent_idx;
            parent_idx = self.parent_idx(child_idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        // if mini heap: oper = <, need smaller one, else need larger one
        // so, this func will return the idx which will be changed
        if self.children_present(idx) {
            if self.right_child_idx(idx) > self.count {
                return self.left_child_idx(idx);
            }
            if (&self.comparator)(&self.items[self.left_child_idx(idx)], &self.items[self.right_child_idx(idx)]) {
                return self.left_child_idx(idx);
            } else {
                return self.right_child_idx(idx);
            }
        }
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count > 0 {
            // 将堆顶元素与最后一个元素交换，并移除最后一个元素
            self.items.swap(1, self.count);
            let res = self.items.pop().unwrap();
            self.count -= 1;
            let mut idx = 1;
            let mut may_change_idx = self.smallest_child_idx(idx);
            // 如果当前节点没有子节点，直接返回堆顶元素
            if may_change_idx == 0 {
                return Some(res);
            }
            // 循环直到找到合适的位置或没有子节点
            while (self.comparator)(&self.items[may_change_idx], &self.items[idx]) {
                // 需要交换当前节点和其最小子节点
                self.items.swap(may_change_idx, idx);
                idx = may_change_idx;
                may_change_idx = self.smallest_child_idx(idx);
                // 如果没有子节点，直接返回堆顶元素
                if may_change_idx == 0 {
                    return Some(res);
                }
            }
            // 返回堆顶元素
            Some(res)
        } else {
            // 如果堆为空，返回 None
            None
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
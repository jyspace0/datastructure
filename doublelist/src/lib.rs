use std::{marker::PhantomData, ptr};

/// 양방향 리스트 (using raw pointer)
/// - DList : 양방향 리스트를 전체 구조체 (head, tail, len)
/// - DNode : 한 칸을 나타내는 구조체 (elem, next, prev)
/// - DLink : 연결을 나타내는 타입 (*mut DNode<T>)
type DLink<T> = *mut DNode<T>;

struct DNode<T> {
    elem: T,
    next: DLink<T>,
    prev: DLink<T>,
}

pub struct DList<T> {
    head: DLink<T>,
    tail: DLink<T>,
    len: usize,
}

impl<T> DNode<T> {
    pub fn new(value: T) -> DLink<T> {
        Box::into_raw(Box::new(DNode {
            elem: value,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }))
    }
}

impl<T> DList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_head = DNode::new(value);

        if self.head.is_null() {
            self.tail = new_head;
        } else {
            unsafe {
                (*new_head).next = self.head;
                (*self.head).prev = new_head;
            }
        }

        self.head = new_head;
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_tail = DNode::new(value);

        if self.tail.is_null() {
            self.head = new_tail;
        } else {
            unsafe {
                (*new_tail).prev = self.tail;
                (*self.tail).next = new_tail;
            }
        }

        self.tail = new_tail;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        } else {
            unsafe {
                let out = Box::from_raw(self.head);
                self.head = out.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                } else {
                    (*self.head).prev = ptr::null_mut();
                }

                self.len -= 1;
                Some(out.elem)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        } else {
            unsafe {
                let out = Box::from_raw(self.tail);
                self.tail = out.prev;

                if self.tail.is_null() {
                    self.head = ptr::null_mut();
                } else {
                    (*self.tail).next = ptr::null_mut();
                }

                self.len -= 1;
                Some(out.elem)
            }
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            return None;
        } else {
            unsafe { Some(&(*self.head).elem) }
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        if self.tail.is_null() {
            return None;
        } else {
            unsafe { Some(&(*self.tail).elem) }
        }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        if self.head.is_null() {
            return None;
        } else {
            unsafe { Some(&mut (*self.head).elem) }
        }
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        if self.tail.is_null() {
            return None;
        } else {
            unsafe { Some(&mut (*self.tail).elem) }
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for DList<T> {
    fn drop(&mut self) {
        // while let Some(_) = self.pop_front() {}
        while self.pop_front().is_some() {}
    }
}

// 3가지 형태의 Iterator 구현
// 1) IntoIter
pub struct IntoIter<T>(DList<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}
impl<T> ExactSizeIterator for IntoIter<T> {}

// 2) Iter
pub struct Iter<'a, T> {
    head: DLink<T>,
    tail: DLink<T>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a DList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 || self.head.is_null() {
            return None;
        }

        unsafe {
            let node = &*self.head;
            self.head = node.next;
            self.len -= 1;
            Some(&node.elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 || self.tail.is_null() {
            return None;
        }

        unsafe {
            let node = &*self.tail;
            self.tail = node.prev;
            self.len -= 1;
            Some(&node.elem)
        }
    }
}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

pub struct IterMut<'a, T> {
    head: *mut DNode<T>,
    tail: *mut DNode<T>,
    len: usize,
    _marker: PhantomData<&'a mut T>,
}
impl<'a, T> IntoIterator for &'a mut DList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 || self.head.is_null() {
            return None;
        }

        unsafe {
            let node = &mut *self.head;
            self.head = node.next;
            self.len -= 1;
            Some(&mut node.elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 || self.tail.is_null() {
            return None;
        }

        unsafe {
            let node = &mut *self.tail;
            self.tail = node.prev;
            self.len -= 1;
            Some(&mut node.elem)
        }
    }
}
impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_check_new() {
        let mut list: DList<i32> = DList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn basic_check_push_pop() {
        let mut list: DList<i32> = DList::new();

        list.push_front(10);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());

        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());

        // push_back (빈 리스트 -> 원소 추가)
        list.push_back(30);
        list.push_back(40);
        assert_eq!(list.len(), 2);

        // pop_back (다수 원소 -> 단일 원소 -> 빈 리스트)
        assert_eq!(list.pop_back(), Some(40));
        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());

        list.push_front(10);
        list.push_back(20);
        list.push_front(30);
        list.push_back(40); // 30 -> 10 -> 20-> 40
        list.push_front(50);
        list.push_back(60); // 50 -> 30 -> 10 -> 20 -> 40 -> 60
        assert_eq!(list.len(), 6);
        assert_eq!(list.pop_front(), Some(50));
        assert_eq!(list.pop_back(), Some(60));
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_back(), Some(40));
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_peek_and_peek_mut() {
        let mut list = DList::new();

        // 빈 리스트에서 peek / peek_mut
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.peek_front_mut(), None);
        assert_eq!(list.peek_back_mut(), None);

        list.push_back(1);
        list.push_back(2);

        // 참조 읽기
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&2));

        // 가변 읽기 및 값 수정
        if let Some(front) = list.peek_front_mut() {
            *front = 100;
        }
        if let Some(back) = list.peek_back_mut() {
            *back = 200;
        }

        assert_eq!(list.peek_front(), Some(&100));
        assert_eq!(list.peek_back(), Some(&200));
    }

    #[test]
    fn test_iter_immutable() {
        let mut list = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // for 루프 (&list -> IntoIterator)
        let mut collected = Vec::new();
        for &val in &list {
            collected.push(val);
        }
        assert_eq!(collected, vec![1, 2, 3]);

        // 양방향 순회 (next & next_back 교차 호출)
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = DList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        // for 루프 (&mut list -> IntoIterator)
        for val in &mut list {
            *val += 1;
        }

        // 양방향 가변 순회 (next & next_back 교차 호출)
        let mut iter_mut = list.iter_mut();
        if let Some(first) = iter_mut.next() {
            *first *= 10; // 11 -> 110
        }
        if let Some(last) = iter_mut.next_back() {
            *last *= 10; // 31 -> 310
        }
        assert_eq!(iter_mut.next(), Some(&mut 21));
        assert_eq!(iter_mut.next(), None);
        assert_eq!(iter_mut.next_back(), None);

        assert_eq!(list.pop_front(), Some(110));
        assert_eq!(list.pop_back(), Some(310));
        assert_eq!(list.pop_front(), Some(21));
    }

    #[test]
    fn test_into_iter_and_drop() {
        let mut list = DList::new();
        list.push_back(String::from("a"));
        list.push_back(String::from("b"));
        list.push_back(String::from("c"));

        // 소유권 소비 이터레이터 및 양방향 호출
        let mut into_iter = list.into_iter();
        assert_eq!(into_iter.next(), Some(String::from("a")));
        assert_eq!(into_iter.next_back(), Some(String::from("c")));
        assert_eq!(into_iter.next(), Some(String::from("b")));
        assert_eq!(into_iter.next(), None);
        assert_eq!(into_iter.next_back(), None);

        // 원소가 남아있는 상태로 스코프를 벗어나 Drop 호출 검증
        let mut drop_list = DList::new();
        drop_list.push_back(String::from("leak_check_1"));
        drop_list.push_back(String::from("leak_check_2"));
        drop(drop_list);
    }
}

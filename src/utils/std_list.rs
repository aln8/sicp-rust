use std::{
    alloc::{self, Layout},
    fmt::Debug,
    marker::PhantomData,
    mem,
    ptr::{self, NonNull},
};

type Link<T> = Option<NonNull<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            next: None,
            prev: None,
            elem,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.elem
    }
}

// private
impl<T> List<T> {
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;
        node.prev = None;
        // let test: Option<NonNull<Node<T>>> = Some((&mut node).into());
        // let node: Option<NonNull<Node<T>>> = NonNull::new(Box::into_raw(node));
        // let node: Option<NonNull<Node<T>>> = NonNull::new(Box::leak(node));
        let node: Option<NonNull<Node<T>>> = Some(Box::leak(node).into());

        match self.head {
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
            None => self.tail = node,
        }
        self.head = node;
        self.len += 1;
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        node.prev = self.tail;
        node.next = None;
        let node: Option<NonNull<Node<T>>> = Some(Box::leak(node).into());

        match self.tail {
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
            None => self.head = node,
        }
        self.tail = node;
        self.len += 1;
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let mut node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(next) => (*next.as_ptr()).prev = None,
            }
            self.len -= 1;
            node
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let mut node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(prev) => (*prev.as_ptr()).next = None,
            }
            self.len -= 1;
            node
        })
    }
}

unsafe impl<#[may_dangle] T> Drop for List<T> {
    fn drop(&mut self) {
        // drop guard for drop again if panic happens
        struct DropGuard<'a, T>(&'a mut List<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                // println!("drop guard works");
                while self.0.pop_front_node().is_some() {}
            }
        }

        while let Some(node) = self.pop_front_node() {
            // println!("drop works");
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push_back(elem);
        }
    }
}

impl<'a, T: Copy> Extend<&'a T> for List<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for elem in iter {
            self.push_back(elem.clone());
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        // .extend() requires impl Extend for List
        list.extend(iter);
        list
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        // .collect() requires impl FromIterator for List<T>
        self.iter().cloned().collect()
    }

    // fn clone_from(&mut self, source: &Self) {

    // }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T: PartialOrd> PartialOrd for List<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for List<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// here Node<T> is on stack, not in Box, although node's head/tail
// pointer might be pointing to heap address. It's only borrow. So
// default drop it's enough, no need to deal with box drop recursion issue.
pub struct Iter<'a, T: 'a> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T: Debug> Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Iter")
            // this ManuallyDrop is important, since it's creating a List
            // with current Iter's ptr. however, after this function, List
            // is dropped, and *ptr will be dropped as well, which will cause
            // rest *ptr dereferenced got freed error.
            // Now this List will never be dropped, but that's fine since it's on
            // debug msg display. It shouldn't affect production use.
            .field(&*mem::ManuallyDrop::new(List {
                head: self.head,
                tail: self.tail,
                len: self.len,
                marker: PhantomData,
            }))
            .field(&self.len())
            .finish()
    }
}

// here since it's all pointer, and Iter<'_, won't change where it's pointed to
// also will no deallocate it. So it's safe to just alias pointer
impl<T: Clone> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.head.map(|node| {
            let node = unsafe { &*node.as_ptr() };
            self.len -= 1;
            self.head = node.next;
            &node.elem
        })
    }

    // for ExactSizeIterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    // since we implement DoubleEndedIterator, need to
    // rewrite last method to last element
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.tail.map(|node| unsafe {
            let node = &*node.as_ptr();
            self.len -= 1;
            self.tail = node.prev;
            &node.elem
        })
    }
}

// this require size_hint() return (exact_size, Some(exact_size))
impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub struct IterMut<'a, T: 'a> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<T: Debug> Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IterMut")
            // See Iter impl Debug fmt() notes
            .field(&*mem::ManuallyDrop::new(List {
                head: self.head,
                tail: self.tail,
                len: self.len,
                marker: PhantomData,
            }))
            .field(&self.len())
            .finish()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.head.map(|node| {
            let node = unsafe { &mut *node.as_ptr() };
            self.len -= 1;
            self.head = node.next;
            &mut node.elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.tail.map(|node| {
            let node = unsafe { &mut *node.as_ptr() };
            self.len -= 1;
            self.tail = node.prev;
            &mut node.elem
        })
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

// since impl Clone for List<T>, IntoIter can utilize #[derive(Clone)]
#[derive(Clone)]
pub struct IntoIter<T> {
    list: List<T>,
}

impl<T: Debug> Debug for IntoIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // field &self.list is debug info by impl Debug for List<T>
        f.debug_tuple("IntoIter").field(&self.list).finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len()
    }
}

// public
impl<T> List<T> {
    #[inline]
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        self.push_front_node(Box::new(Node::new(elem)))
    }

    pub fn push_back(&mut self, elem: T) {
        self.push_back_node(Box::new(Node::new(elem)))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len(),
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len(),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        // self.len() == 0  // this also works
        self.head.is_none()
    }

    pub fn clear(&mut self) {
        // *self = Self::new(); this is fine
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use std::panic;

    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len, 0);
        // Populate list
        list.push_front(1);
        list.push_front(2);
        println!("{:?}", list);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        assert_eq!(list.len(), 1);
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_borrow() {
        #[derive(Debug, PartialEq)]
        struct Bad<T>(T);

        // uncomment Drop should cause compiler drop order lifetime
        // error, since here are using PhantomData for illustrate compiler
        // that T should be forced to outlive List<T>

        // impl<T> Drop for Bad<T> {
        //     fn drop(&mut self) {
        //         println!("drop Bad<T>")
        //     }
        // }

        let mut list = List::new();
        let a = 3;
        let b = Bad(&a);
        list.push_front(b);
        assert_eq!(list.pop_front(), Some(Bad(&3)));

        // #![feature(dropck_eyepatch)]
        // unsafe impl<#[may_dangle] T> Drop for List<T> {
        // may_dangle tells compiler dropck(drop check) the
        // the pointer may dangle, but we will never touch that memory
        // so we can bypass strick borrow check like below.

        let mut list = List::new();
        let a = 3;
        list.push_front(&a);
        assert_eq!(list.pop_front(), Some(&3));
    }

    #[test]
    fn panic_test() {
        // here we create a struct PanicDrop, it will panic if been dropped
        // in this case if there is no DropGuard for List<T> Drop implementation
        // the default compiler will compile since panic is been unwind(), so list
        // will cause memory leak, since it won't try drop again.
        // however, with DropGuard, it will try again since we manually called DropGuard
        // to drop. And it will cause abort(), since panic in panic cause abort()
        struct PanicDrop(i32);

        impl Drop for PanicDrop {
            fn drop(&mut self) {
                panic!("intent panic");
            }
        }

        let mut list = List::new();
        list.push_front(PanicDrop(1));
        // list.push_front(PanicDrop(2));
        // list.push_front(PanicDrop(3));
        panic::catch_unwind(|| {
            drop(list);
        });
    }

    #[test]
    fn iteration() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        println!("{:?}", iter);
        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut list2 = List::new();
        list2.push_front(4);
        list2.push_front(5);
        list2.push_front(6);

        let mut into_iter = list.into_iter();
        list2.extend(into_iter);
        let mut into_iter = list2.into_iter();
        assert_eq!(into_iter.next_back(), Some(1));
        assert_eq!(into_iter.next(), Some(6));
        assert_eq!(into_iter.next_back(), Some(2));
        assert_eq!(into_iter.next(), Some(5));
        assert_eq!(into_iter.next_back(), Some(3));
        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), None);

        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut list2 = List::new();
        list2.push_front(4);
        list2.push_front(5);
        list2.push_front(6);

        let mut iter = list.iter();
        list2.extend(iter);
        let mut into_iter = list2.into_iter();
        println!("{:?}", into_iter);
        assert_eq!(into_iter.next_back(), Some(1));
        assert_eq!(into_iter.next(), Some(6));
        assert_eq!(into_iter.next_back(), Some(2));
        assert_eq!(into_iter.next(), Some(5));
        assert_eq!(into_iter.next_back(), Some(3));
        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), None);
    }
}

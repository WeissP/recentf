use serde::Serialize;

use super::node::Node;
use std::{collections::VecDeque, iter::Peekable};

type IntoIter<'a, T> = Peekable<std::vec::IntoIter<Node<'a, T>>>;
type IterStack<'a, T> = VecDeque<IntoIter<'a, T>>;

#[derive(Default)]
pub struct Iter<'a, T: Serialize> {
    stack: IterStack<'a, T>,
}

pub struct IterItem<'a, T> {
    pub segments: &'a [&'a str],
    pub val: Option<T>,
    pub children_num: usize,
}

impl<'a, T> Iter<'a, T>
where
    T: Serialize,
{
    pub fn new(root: Node<'a, T>) -> Self {
        Iter {
            stack: vec![vec![root].into_iter().peekable()].into(),
        }
    }
}

impl<'a, T: Serialize> Iterator for Iter<'a, T> {
    // (path segments, children number)
    type Item = IterItem<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut elem: &mut IntoIter<'a, T>;
        let mut cur: Node<'a, T>;
        loop {
            if self.stack.is_empty() {
                return None;
            }
            elem = self.stack.back_mut().unwrap();
            if let Some(x) = elem.next() {
                cur = x;
                break;
            };
            self.stack.pop_back();
        }
        // children will be taken, so we need to store the length in advance.
        let child_num = cur.children.len();
        if cur.has_children() {
            let children = std::mem::take(&mut cur.children);
            self.stack.push_back(children.into_iter().peekable());
        };

        Some(IterItem {
            segments: cur.segments,
            val: cur.val,
            children_num: child_num,
        })
    }
}

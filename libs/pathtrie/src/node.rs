use super::iter::{Iter, IterItem};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Node<'a, T>
where
    T: Serialize,
{
    pub segments: &'a [&'a str],

    #[serde(flatten)]
    pub val: Option<T>,
    pub children: Vec<Node<'a, T>>,
}

// impl<'a> FromIterator<Vec<&'a str>> for Node<'a, ()> {
//     fn from_iter<It: IntoIterator<Item = Vec<&'a str>>>(iter: It) -> Self {
//         let mut root = Node::default();
//         for v in iter {
//             root.insert(v, None);
//         }
//         root
//     }
// }

impl<'a, T> Default for Node<'a, T>
where
    T: Serialize,
{
    fn default() -> Self {
        Node {
            segments: &[],
            val: None,
            children: Vec::new(),
        }
    }
}

impl<'a> From<&'a Vec<Vec<&'a str>>> for Node<'a, ()> {
    fn from(vv: &'a Vec<Vec<&'a str>>) -> Self {
        let mut root = Node::default();
        for v in vv {
            root.insert(v, None);
        }
        root
    }
}

impl<'a, T> IntoIterator for Node<'a, T>
where
    T: Serialize,
{
    type Item = IterItem<'a, T>;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a, T> Node<'a, T>
where
    T: Serialize,
{
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn insert_helper(&mut self, segs: &'a [&'a str], val: Option<T>, to: usize) {
        if self.segments.is_empty() {
            self.segments = &segs[..=to];
        }

        // if there are still segs wait for inserting
        match segs.get(to + 1) {
            Some(new_seg) => {
                let child_op = self
                    .children
                    .iter_mut()
                    .find(|x| match x.segments.get(to + 1) {
                        Some(seg) => seg == new_seg,
                        _ => false,
                    });

                match child_op {
                    Some(child) => child.insert_helper(segs, val, to + 1),
                    _ => {
                        self.children.push(Node::default());
                        self.children
                            .last_mut()
                            .unwrap()
                            .insert_helper(segs, val, to + 1)
                    }
                }
            }
            None => self.val = val,
        };
    }

    pub fn insert(&mut self, full_path: &'a [&'a str], val: Option<T>) {
        assert!(!full_path.is_empty());
        self.insert_helper(full_path, val, 0);
    }
}

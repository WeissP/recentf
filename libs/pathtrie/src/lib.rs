use self::{format::Format, node::Node};
use serde::Serialize;
use std::mem::take;

pub mod format;
mod iter;
pub mod node;

pub type Segs<'a> = &'a [&'a str];

#[derive(Default, Debug, Clone, Serialize)]
pub struct Tree<'a, T, F: Format>
where
    T: Serialize,
{
    pub root: Node<'a, T>,
    pub fmt: F,
}

impl<'a, T, F: Format> Tree<'a, T, F>
where
    T: Serialize,
{
    pub fn print(mut self, compressed: bool, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // the length of the level_stack is the depth of the current node
        let mut level_stack = Vec::with_capacity(30);
        level_stack.push(0);
        for item in take(&mut self.root).into_iter() {
            // it would happen if the last iterated node is a blatt
            // we need to find the level that matched the current node
            while *level_stack.last().unwrap() >= item.segments.len() {
                level_stack.pop();
            }
            // skip the node that only has one child.
            if !compressed || item.children_num != 1 {
                self.fmt.print_seg(item.segments, &level_stack, f)?;
                level_stack.push(item.segments.len());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{fmt::Display, path::Path};

    use crate::{
        format::{Format, Segs},
        node::Node,
        Tree,
    };

    #[derive(Clone)]
    struct Fmt<'a> {
        pub indent: &'a str,
        pub prefix: &'a str,
        pub width: usize,
    }

    impl<'a> Format for Fmt<'a> {
        fn prefix(&self) -> &'a str {
            self.prefix
        }

        fn single_indent(&self) -> &'a str {
            self.indent
        }

        fn max_width(&self) -> usize {
            self.width
        }

        fn suffix(&self, _segs: Segs, _level_stack: &Vec<usize>) -> String {
            String::new()
        }
    }

    struct PrintTree<'a>(Tree<'a, (), Fmt<'a>>);

    impl<'a> Display for PrintTree<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.clone().print(true, f)
        }
    }

    #[test]
    fn print() {
        let paths: Vec<_> = ["/a/a1/file", "/a/a1/b/file"]
            .iter()
            .map(|p| {
                Path::new(&p[1..]) // remove the root slash
                    .into_iter()
                    .map(|x| x.to_str().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let mut root = Node::default();
        for p in &paths {
            root.insert(p, None)
        }
        // let paths: Vec<_> = vec!["/a/a1/file", "/a/a1/b/file"]
        //     .iter()
        //     .map(|x| {
        //         Path::new(&x[1..]) // remove the root slash
        //             .iter()
        //             .map(|x| x.to_str().unwrap())
        //             .collect::<Vec<_>>()
        //     })
        //     .collect();
        let tree = PrintTree(Tree {
            root,
            fmt: Fmt {
                indent: "    ",
                prefix: "",
                width: 80,
            },
        });

        println!("{}", &tree);
        assert_eq!(1, 2)
    }
}

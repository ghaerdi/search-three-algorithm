use std::{collections::VecDeque};

mod globals;

#[derive(Debug, Clone)]
struct Node {
    value: char,
    nodes: Vec<Node>,
}

impl Node {
    fn genesis() -> Self {
        Node {
            value: '\0',
            nodes: vec![],
        }
    }
    // fn example() -> Self {
    //     Node {
    //         value: '\0',
    //         nodes: vec![Node {
    //             value: 'p',
    //             nodes: vec![Node {
    //                 value: 'e',
    //                 nodes: vec![Node {
    //                     value: 'p',
    //                     nodes: vec![Node {
    //                         value: 'e',
    //                         nodes: vec![],
    //                     }],
    //                 }],
    //             }],
    //         }],
    //     }
    // }
}

#[derive(Debug, Clone)]
struct Three {
    root: Node,
}

impl Three {
    pub fn new() -> Self {
        Three {
            root: Node::genesis(),
        }
    }
    pub fn insert(&mut self, text: &str) {
        let chars = Self::str_to_chars(text);
        let (node, mut chars) = Self::get_node_by_chars(&mut self.root, chars);

        if let Some(c) = chars.pop_back() {
            let mut temp = Node { value: c, nodes: vec![] };
            while let Some(c) = chars.pop_back() {
                temp = Node { value: c, nodes: vec![temp] }
            }

            node.nodes.push(temp);
        }
    }

    // TODO: Move this fn to a utils file
    fn str_to_chars(s: &str) -> VecDeque<char> {
        let mut chars = VecDeque::with_capacity(s.len());

        s.chars().into_iter().for_each(|v| {
            chars.push_back(v);
        });

        chars
    }

    fn get_node_by_chars<'a>(node: &'a mut Node, mut chars: VecDeque<char>) -> (&'a mut Node, VecDeque<char>) {
        if let Some(c) = chars.pop_front() {
            let node_position = node.nodes.clone().into_iter().position(|el| el.value == c);

            match node_position {
                Some(position) => return Self::get_node_by_chars(&mut node.nodes[position], chars),
                None => {
                    chars.push_front(c);
                    return (node, chars);
                }
            }
        }
        (node, chars)
    }
}

fn main() {
    let mut t = Three::new();

    t.insert("pepe");
    t.insert("pepito");
    t.insert("maico");
    t.insert("miguel");

    // globals::FRUITS.into_iter().for_each(|v| {
    //     t.insert(v);
    // });

    println!("{:#?}", t);
}

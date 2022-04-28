use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
struct Node {
    value: char,
    nodes: HashMap<char, Node>,
    is_end: bool,
}

struct NodeBuilder {
    value: char,
    nodes: Vec<Node>,
}

impl NodeBuilder {
    fn push_node(&mut self, node: Node) -> &mut Self {
        self.nodes.push(node);
        self
    }

    fn build(&self) -> Node {
        let mut map = HashMap::new();
        self.nodes.clone().into_iter().for_each(|el| {
            map.insert(el.value, el);
        });

        Node {
            value: self.value,
            nodes: map,
            is_end: false,
        }
    }
}

impl Node {
    fn new(c: char) -> NodeBuilder {
        NodeBuilder {
            value: c,
            nodes: vec![],
        }
    }

    fn genesis() -> Self {
        Self::new('\0').build()
    }
}

#[derive(Debug)]
pub struct Vocabulary {
    root: Node,
    len: usize,
}

impl Vocabulary {
    pub fn new() -> Self {
        Self {
            root: Node::genesis(),
            len: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, text: &str) {
        let chars = Self::str_to_chars(text);
        let (node, mut chars) = Self::get_node_by_chars(&mut self.root, chars);

        match chars.pop_back() {
            Some(c) => {
                let mut temp = Node::new(c).build();

                temp.is_end = true;

                while let Some(c) = chars.pop_back() {
                    temp = Node::new(c).push_node(temp).build();
                }

                node.nodes.insert(temp.value, temp);
                self.len += 1;
            }
            None => match node.is_end {
                true => println!("{} already exist", text),
                false => {
                    node.is_end = true;
                    self.len += 1;
                }
            },
        }
    }

    pub fn search(&mut self, text: &str) -> Vec<String> {
        if text.is_empty() {
            return vec![];
        }

        let text = text.to_lowercase();

        let chars = Self::str_to_chars(&text);
        let (node, chars) = Self::get_node_by_chars(&mut self.root, chars);

        if !chars.is_empty() {
            return vec![];
        }

        Self::nodes_to_text(&text, node.clone())
    }

    pub fn get_all(&mut self) -> Vec<String> {
        let chars = Self::str_to_chars("");
        let (node, _) = Self::get_node_by_chars(&mut self.root, chars);
        Self::nodes_to_text("", node.clone())
    }

    fn str_to_chars(s: &str) -> VecDeque<char> {
        let mut chars = VecDeque::with_capacity(s.len());

        s.chars().into_iter().for_each(|v| {
            chars.push_back(v);
        });

        chars
    }

    fn get_node_by_chars(
        node: &mut Node,
        mut chars: VecDeque<char>,
    ) -> (&mut Node, VecDeque<char>) {
        if let Some(c) = chars.pop_front() {
            match node.nodes.contains_key(&c) {
                true => return Self::get_node_by_chars(node.nodes.get_mut(&c).unwrap(), chars),
                false => {
                    chars.push_front(c);
                    return (node, chars);
                }
            }
        }
        (node, chars)
    }

    fn nodes_to_text(text: &str, node: Node) -> Vec<String> {
        struct TempNode {
            node: Node,
            text: String,
        }

        let mut stack = vec![];
        let mut words = vec![];

        node.nodes.into_iter().for_each(|(_, el)| {
            stack.push(TempNode {
                node: el,
                text: text.to_string(),
            })
        });

        fn recursive(stack: &mut Vec<TempNode>, words: &mut Vec<String>) {
            if stack.is_empty() {
                return;
            }

            let current = stack.pop().unwrap();
            let text = format!("{}{}", current.text, current.node.value);

            match !current.node.nodes.is_empty() {
                true => current.node.nodes.into_iter().for_each(|(_, el)| {
                    if current.node.is_end {
                        words.push(text.clone());
                    }

                    stack.push(TempNode {
                        node: el,
                        text: text.clone(),
                    });
                }),
                false => words.push(text),
            }

            recursive(stack, words)
        }

        recursive(&mut stack, &mut words);

        words
    }
}

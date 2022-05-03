use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
struct Node {
    value: char,
    nodes: HashMap<char, Node>,
    words: Vec<String>,
}

struct NodeBuilder {
    value: char,
    nodes: Vec<Node>,
    words: Vec<String>,
}

impl NodeBuilder {
    fn push_node(&mut self, node: Node) -> &mut Self {
        self.nodes.push(node);
        self
    }

    fn push_word(&mut self, text: &str) -> &mut Self {
        self.words.push(text.to_owned());
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
            words: self.words.clone(),
        }
    }
}

impl Node {
    fn new(c: char) -> NodeBuilder {
        NodeBuilder {
            value: c,
            nodes: vec![],
            words: vec![],
        }
    }

    fn genesis() -> Self {
        Self::new('\0').build()
    }
}

#[derive(Debug)]
pub struct Vocabulary {
    root: Node,
}

impl Vocabulary {
    pub fn new() -> Self {
        Self {
            root: Node::genesis(),
        }
    }
    pub fn len(&self) -> usize {
        self.root.words.len()
    }

    pub fn insert(&mut self, text: &str) {
        let chars = Self::str_to_chars(text);
        let (node, mut chars) = Self::get_node_by_chars_and_insert(&mut self.root, chars, text);
        match chars.pop_back() {
            Some(c) => {
                let mut temp = Node::new(c).push_word(text).build();

                while let Some(c) = chars.pop_back() {
                    temp = Node::new(c).push_node(temp).push_word(text).build();
                }

                node.nodes.insert(temp.value, temp);
            }
            None => match node.words.contains(&text.to_owned()) {
                true => println!("{} already exist", text),
                false => {
                    node.words.push(text.to_owned());
                }
            },
        }
    }

    pub fn search(&self, text: &str) -> &Vec<String> {
        if text.is_empty() {
            return self.get_all();
        }

        let text = text.to_lowercase();

        let chars = Self::str_to_chars(&text);
        let (node, _) = Self::get_node_by_chars(&self.root, chars);

        &node.words
    }

    pub fn get_all(&self) -> &Vec<String> {
        &self.root.words
    }

    fn str_to_chars(s: &str) -> VecDeque<char> {
        let mut chars = VecDeque::with_capacity(s.len());

        s.chars().into_iter().for_each(|v| {
            chars.push_back(v);
        });

        chars
    }

    fn get_node_by_chars_and_insert<'a>(
        node: &'a mut Node,
        mut chars: VecDeque<char>,
        text: &str,
    ) -> (&'a mut Node, VecDeque<char>) {
        if let Some(c) = chars.pop_front() {
            node.words.push(text.to_owned());
            if node.nodes.contains_key(&c) {
                return Self::get_node_by_chars_and_insert(
                    node.nodes.get_mut(&c).unwrap(),
                    chars,
                    text,
                );
            }
            chars.push_front(c);
            return (node, chars);
        }

        (node, chars)
    }

    fn get_node_by_chars(node: &Node, mut chars: VecDeque<char>) -> (&Node, VecDeque<char>) {
        if let Some(c) = chars.pop_front() {
            if node.nodes.contains_key(&c) {
                return Self::get_node_by_chars(node.nodes.get(&c).unwrap(), chars);
            }
            chars.push_front(c);
            return (node, chars);
        }
        (node, chars)
    }
}

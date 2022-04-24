use std::collections::VecDeque;

#[derive(Clone)]
struct Node {
    value: char,
    nodes: Vec<Node>,
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
        Node {
            value: self.value.clone(),
            nodes: self.nodes.clone(),
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

pub struct Vocabulary {
    root: Node,
}

impl Vocabulary {
    pub fn new() -> Self {
        Self {
            root: Node::genesis(),
        }
    }
    pub fn insert(&mut self, text: &str) {
        let text = text.to_lowercase();

        let chars = Self::str_to_chars(&text);
        let (node, mut chars) = Self::get_node_by_chars(&mut self.root, chars);

        if let Some(c) = chars.pop_back() {
            let mut temp = Node::new(c).build();

            while let Some(c) = chars.pop_back() {
                temp = Node::new(c).push_node(temp).build();
            }

            node.nodes.push(temp);
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

    // TODO: Move this fn to a utils file
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

    fn nodes_to_text(text: &str, node: Node) -> Vec<String> {
        struct TempNode {
            node: Node,
            text: String,
        }

        let mut stack: Vec<TempNode> = vec![];
        let mut words = vec![];

        node.nodes.into_iter().for_each(|el| {
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

            if !current.node.nodes.is_empty() {
                current.node.nodes.into_iter().for_each(|el| {
                    let text = format!("{}{}", current.text, current.node.value);

                    stack.push(TempNode { node: el, text })
                });
            } else {
                words.push(format!("{}{}", current.text, current.node.value));
            }

            recursive(stack, words);
        }

        recursive(&mut stack, &mut words);

        words
    }
}

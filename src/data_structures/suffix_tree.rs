use core::fmt;

const INF: usize = usize::MAX;
const ROOT: usize = 0;

struct Node {
    start: usize,
    end: usize,
    suffix_link: usize,
    children: Vec<usize>,
}

impl Node {
    fn new(start: usize, end: usize, child_count: usize) -> Node {
        Node {
            start,
            end,
            suffix_link: 0,
            children: vec![0; child_count],
        }
    }

    fn edge_length(&self, position: usize) -> usize {
        std::cmp::min(self.end, position + 1) - self.start
    }
}

pub struct SuffixTree {
    nodes: Vec<Node>,
    empty_node_indicies: Vec<usize>,
    text: Vec<u8>,
    alphabet: Vec<u8>,
    alphabet_indexer: Vec<Option<usize>>,
    position: usize,
    need_suffix_link: usize,
    tree_remainder: usize,
    node_active: usize,
    edge_active: usize,
    length_active: usize,
    first_loop_flag: bool,
}
impl SuffixTree {
    pub fn new() -> SuffixTree {
        let mut st = SuffixTree {
            nodes: Vec::new(),
            empty_node_indicies: Vec::new(),
            text: Vec::new(),
            alphabet: Vec::new(),
            alphabet_indexer: vec![None; u8::MAX as usize],
            need_suffix_link: 0,
            tree_remainder: 0,
            edge_active: 0,
            length_active: 0,
            position: 0,
            node_active: ROOT,
            first_loop_flag: true,
        };
        st.nodes.push(Node::new(0, 0, 0)); // Root node
        st
    }

    fn new_node(&mut self, start: usize, end: usize) -> usize {
        let nd = Node::new(start, end, self.alphabet.len());
        if !self.empty_node_indicies.is_empty() {
            let index = self.empty_node_indicies.pop().unwrap();
            self.nodes[index] = nd;
            return index;
        } else {
            self.nodes.push(nd);
            return self.nodes.len() - 1;
        }
    }

    fn char_index(&self, c: u8) -> usize {
        self.alphabet_indexer[c as usize].unwrap()
    }

    fn extend_alphabet(&mut self, c: u8) {
        self.alphabet.push(c);
        self.alphabet_indexer[c as usize] = Some(self.alphabet.len() - 1);
        for node in self.nodes.iter_mut() {
            node.children.resize(self.alphabet.len(), 0);
        }
    }

    fn add_suffix_link(&mut self, node: usize) {
        if self.need_suffix_link != 0 {
            self.nodes[self.need_suffix_link].suffix_link = node;
            self.delete_children(self.need_suffix_link);
            self.nodes[self.need_suffix_link].children = self.nodes[node].children.clone();
        }
        self.need_suffix_link = node;
    }

    fn delete_children(&mut self, node: usize) {
        let mut safety: usize = 0;
        let mut stack: Vec<usize> = Vec::new();
        self.nodes[node].children.iter().filter(|x| **x != 0).map(|x| *x).collect::<Vec<usize>>().clone_into(&mut stack); //disgusting functional style hacks
        while !stack.is_empty() {
            let current: usize = stack.pop().unwrap();
            stack.extend(self.nodes[current].children.iter().filter(|x| **x != 0));
            self.empty_node_indicies.push(current); // Marks node for replacement later
            dbg!(&stack);
            println!("{}", self);

            safety += 1;
            if safety > 5 {
                panic!("infinite loop");
            }
        }
    }

    fn walk_down(&mut self, node: usize) -> bool {
        if self.length_active >= self.nodes[node].edge_length(self.position) {
            self.edge_active += self.nodes[node].edge_length(self.position);
            self.length_active -= self.nodes[node].edge_length(self.position);
            self.node_active = node;
            return true;
        }
        return false;
    }

    pub fn extend(&mut self, char_to_add: u8) {
        let s = self;

        match s.first_loop_flag {
            true => s.first_loop_flag = false,
            false => s.position += 1,
        }

        s.text.push(char_to_add);
        if !s.alphabet.contains(&char_to_add) {
            s.extend_alphabet(char_to_add);
        }

        s.need_suffix_link = 0;
        s.tree_remainder += 1;

        while s.tree_remainder > 0 {
            if s.length_active == 0 {
                s.edge_active = s.position;
            }
            if s.nodes[s.node_active].children[s.char_index(s.text[s.edge_active])] == 0 {
                let leaf = s.new_node(s.position, INF);

                let i = s.char_index(s.text[s.edge_active]);
                s.nodes[s.node_active].children[i] = leaf;
                s.add_suffix_link(s.node_active);
            } else {
                let next = s.nodes[s.node_active].children[s.char_index(s.text[s.edge_active])];
                if s.walk_down(next) {
                    continue;
                }
                if s.text[s.nodes[next].start + s.length_active] == char_to_add {
                    s.length_active += 1;
                    s.add_suffix_link(s.node_active);
                    break;
                }
                let split = s.new_node(s.nodes[next].start, s.nodes[next].start + s.length_active);

                let i = s.char_index(s.text[s.edge_active]);
                s.nodes[s.node_active].children[i] = split;

                let leaf = s.new_node(s.position, INF);

                let i = s.char_index(char_to_add);
                s.nodes[split].children[i] = leaf;

                s.nodes[next].start += s.length_active;
                let next_start = s.nodes[next].start; // Deviation due to borrow checker

                let i = s.char_index(s.text[next_start]);
                s.nodes[split].children[i] = next;

                s.add_suffix_link(split);
            }
            s.tree_remainder -= 1;
            if s.node_active == ROOT && s.length_active > 0 {
                s.length_active -= 1;
                s.edge_active = s.position - s.tree_remainder + 1;
            } else {
                s.node_active = match s.nodes[s.node_active].suffix_link {
                    x if x > 0 => s.nodes[s.node_active].suffix_link,
                    x if x == 0 => ROOT,
                    _ => panic!("oopsie!"),
                }
            }
        }
    }
}

impl fmt::Display for SuffixTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Suffix tree for: {}",
            self.text.iter().map(|x| *x as char).collect::<String>()
        )?;
        for (index, node) in self.nodes.iter().enumerate() {
            let mut substring = String::new();
            match node.end {
                0 => (),
                INF => substring.push_str(
                    &self.text.iter().map(|x| *x as char).collect::<String>()[node.start..],
                ),
                x => substring.push_str(
                    &self.text.iter().map(|x| *x as char).collect::<String>()[node.start..x],
                ),
            }
            writeln!(f, "{:<3} | {:<10} | {}", index, substring, node)?;
        }
        write!(f, "")
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let end: String = match self.end {
            INF => "End".to_string(),
            x => x.to_string(),
        };
        let sl: String = match self.suffix_link {
            0 => "No SL".to_string(),
            x => x.to_string(),
        };
        let children: Vec<&usize> = self.children.iter().filter(|x| **x != 0).collect();
        write!(
            f,
            "{:<6} | {:<6} | {:<6} | {:?}",
            self.start, end, sl, children
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

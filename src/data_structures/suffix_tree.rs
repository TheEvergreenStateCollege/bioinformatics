use core::fmt;
use crate::data_structures::byte_vec::ByteVec;

const INF: u32 = u32::MAX;
const ROOT: u32 = 0;

struct Node {
    start: u32,
    end: u32,
    suffix_link: u32,
    children: ByteVec<u32>,
}

impl Node {
    fn new(start: u32, end: u32, child_count: u8) -> Node {
        let mut children: ByteVec<u32> = ByteVec::new();
        for _ in 0..child_count {
            children.push(0_u32);
        }
        Node {
            start,
            end,
            suffix_link: 0,
            children,
        }
    }

    fn edge_length(&self, position: u32) -> u32 {
        std::cmp::min(self.end, position + 1) - self.start
    }
}

pub struct SuffixTree {
    nodes: Vec<Node>,
    empty_node_indicies: Vec<u32>,
    text: Vec<u8>,
    alphabet: Vec<u8>,
    alphabet_indexer: Vec<Option<usize>>,
    position: i32,
    need_suffix_link: u32,
    tree_remainder: u32,
    node_active: u32,
    edge_active: u32,
    length_active: u32,
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
            position: -1,
            node_active: ROOT,
        };
        st.nodes.push(Node::new(0, 0, 0)); // Root node
        st
    }

    fn new_node(&mut self, start: u32, end: u32) -> u32 {
        let nd = Node::new(start, end, self.alphabet.len() as u8);
        if !self.empty_node_indicies.is_empty() {
            let index = self.empty_node_indicies.pop().unwrap();
            self.nodes[index as usize] = nd;
            return index;
        } else {
            self.nodes.push(nd);
            return (self.nodes.len() - 1) as u32;
        }
    }

    fn char_index(&self, c: u8) -> usize {
        self.alphabet_indexer[c as usize].unwrap()
    }

    fn extend_alphabet(&mut self, c: u8) {
        self.alphabet.push(c);
        self.alphabet_indexer[c as usize] = Some(self.alphabet.len() - 1);
        for node in self.nodes.iter_mut() {
            let alph_len = self.alphabet.len() as u8;
            while node.children.len() < alph_len as usize {
                node.children.push(0)
            }
        }
    }

    fn add_suffix_link(&mut self, node: u32) {
        if self.need_suffix_link != 0 {
            self.nodes[self.need_suffix_link as usize].suffix_link = node;
        }
        self.need_suffix_link = node;
    }

    fn delete_children(&mut self, node: usize) {
        let mut safety: usize = 0;
        let mut stack: Vec<u32> = Vec::new();
        self.nodes[node].children.iter().filter(|x| **x != 0).map(|x| *x).collect::<Vec<u32>>().clone_into(&mut stack); //disgusting functional style hacks
        while !stack.is_empty() {
            let current: u32 = stack.pop().unwrap();
            stack.extend(self.nodes[current as usize].children.iter().filter(|x| **x != 0));
            self.empty_node_indicies.push(current); // Marks node for replacement later
            dbg!(&stack);
            println!("{}", self);

            safety += 1;
            if safety > 5 {
                panic!("infinite loop");
            }
        }
    }

    fn walk_down(&mut self, node: u32) -> bool {
        if self.length_active >= self.nodes[node as usize].edge_length(self.position as u32) {
            self.edge_active += self.nodes[node as usize].edge_length(self.position as u32);
            self.length_active -= self.nodes[node as usize].edge_length(self.position as u32);
            self.node_active = node;
            return true;
        }
        return false;
    }

    pub fn extend(&mut self, char_to_add: u8) {
        let s = self;
        
        s.text.push(char_to_add);
        if !s.alphabet.contains(&char_to_add) {
            s.extend_alphabet(char_to_add);
        }

        s.position += 1;
        s.need_suffix_link = 0;
        s.tree_remainder += 1;

        while s.tree_remainder > 0 {
            if s.length_active == 0 {
                s.edge_active = s.position as u32;
            }
            if s.nodes[s.node_active as usize].children[s.char_index(s.text[s.edge_active as usize])] == 0 {
                let leaf = s.new_node(s.position as u32, INF);

                let i = s.char_index(s.text[s.edge_active as usize]);
                s.nodes[s.node_active as usize].children[i] = leaf;
                s.add_suffix_link(s.node_active);
            } else {
                let next = s.nodes[s.node_active as usize].children[s.char_index(s.text[s.edge_active as usize])] as usize;
                if s.walk_down(next as u32) {
                    continue;
                }
                if s.text[(s.nodes[next].start + s.length_active) as usize] == char_to_add {
                    s.length_active += 1;
                    s.add_suffix_link(s.node_active);
                    break;
                }
                let split = s.new_node(s.nodes[next].start, s.nodes[next].start + s.length_active);

                let i = s.char_index(s.text[s.edge_active as usize]);
                s.nodes[s.node_active as usize].children[i] = split;

                let leaf = s.new_node(s.position as u32, INF);

                let i = s.char_index(char_to_add);
                s.nodes[split as usize].children[i] = leaf;

                s.nodes[next].start += s.length_active;
                let next_start = s.nodes[next].start as usize; // Deviation due to borrow checker

                let i = s.char_index(s.text[next_start]);
                s.nodes[split as usize].children[i] = next as u32;

                s.add_suffix_link(split);
            }
            s.tree_remainder -= 1;
            if s.node_active == ROOT && s.length_active > 0 {
                s.length_active -= 1;
                s.edge_active = s.position as u32 - s.tree_remainder + 1;
            } else {
                s.node_active = match s.nodes[s.node_active as usize].suffix_link {
                    x if x > 0 => s.nodes[s.node_active as usize].suffix_link,
                    x if x == 0 => ROOT,
                    _ => panic!("oopsie!"),
                }
            }
        }
    }
    
    pub fn find_substring(&self, substring: &str) -> (u32, u32) { //match start, match end (exclusive)
        let mut current_node = 0_u32; //start at root
        let mut index_in_node = 0_u32; //Node has no substring it refers to
        let mut chars_in_node = 0_u32;
        let mut match_size = 0_u32;

        let s = self;

        for c in substring.as_bytes() {
            if index_in_node == chars_in_node {
                let child = s.nodes[current_node as usize].children[s.char_index(*c)];
                if child == 0 {
                    break;
                }
                match_size += 1;
                current_node = child;
                chars_in_node = match s.nodes[current_node as usize].end {
                    INF => s.nodes[current_node as usize].edge_length(s.text.len() as u32),
                    _ => s.nodes[current_node as usize].edge_length(0), // Placeholder 0 - get_length will not use position when working with internal nodes
                };
                index_in_node = 1;
                continue;
            }
            if s.text[(s.nodes[current_node as usize].start + index_in_node) as usize] == *c {
                index_in_node += 1;
                match_size += 1;
                continue;
            }
            break;
        }
        let end_of_match = s.nodes[current_node as usize].start + index_in_node;
        return (end_of_match - match_size, end_of_match); //Will return (0,0) if no match is found
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
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
                    &self.text.iter().map(|x| *x as char).collect::<String>()[node.start as usize..],
                ),
                x => substring.push_str(
                    &self.text.iter().map(|x| *x as char).collect::<String>()[node.start as usize..x as usize],
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
        let children: Vec<&u32> = self.children.iter().filter(|x| **x != 0).collect();
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

    #[test]
    fn find_substring_for_all_substrings() {
        let mut st = SuffixTree::new();
        let input_str = "xacaadaaacd";
        let input: Vec<char> = input_str.chars().collect();
        for c in input {
            st.extend(c as u8);
        }
        for i in 0..input_str.len() + 1 {
            for j in i+1..input_str.len() + 1 {
                let test_str = &input_str[i..j];
                println!("{} ", test_str);
                assert_eq!((i as u32, j as u32), st.find_substring(test_str));
            }
        }
    }
}

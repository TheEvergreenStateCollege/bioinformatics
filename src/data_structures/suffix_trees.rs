#![allow(dead_code, unused_variables, unused_imports)]
use core::fmt;
use std::fmt::format;

const ROOT: usize = 1;

#[derive(Debug)]
enum End {
    Root,
    End,
    Index(usize),
}

// We should be able to find all occurences of a substring in a string,
// not just the first one. Not sure how to implement that.
#[derive(Debug)]
struct Node {
    /// The index in the string at the start of the substring the node represents
    /// start is None only for the root. The value of start in Root should never be accessed,
    /// so I'm setting it up to panic if it ever happens
    start: Option<usize>,

    /// The index in the string at the end of the substring the node represents
    /// The enum has infinity for unbounded substrings, and Root for the root, even though it should never be accessed and will panic
    end: End,

    /// This refers to another node which this node links to, if it exists
    suffix_link: Option<usize>,

    /// All nodes are stored in a vector and refered to by index. This stores the indicies of child nodes
    /// the size of this vec is always the alphabet size, for simplicity (it does waste memory though)
    children: Vec<usize>,
}

impl Node {
    fn new(size: usize, start: Option<usize>, end: End) -> Self {
        Self {
            start,
            end,
            suffix_link: None,
            children: vec![0; size],
        }
    }

    fn add_children<T>(mut self, children: T)
    where
        T: IntoIterator<Item = usize>, // allows for singular usize or container of. need to read more on this.
    {
        let iter = children.into_iter();
        for value in iter {
            if !self.children.contains(&value) {
                self.children.push(value);
            }
        }
    }

    // Never called on Root
    fn get_length(&self, position: usize) -> usize {
        let upper_bound = match self.end {
            // I'm not sure if the min is requred after accounting for infinity
            // From testing it seems like it isn't. I'll leave it in to be safe
            End::Index(i) => {
                if i < position + 1 {
                    return i;
                } else {
                    println!("Position + 1 was smaller!");
                    return position + 1;
                }
            }
            End::End => position + 1,
            End::Root => panic!("Tried to get end of root"),
        };
        let lower_bound = self.start.expect("Tried to get start of root");
        upper_bound - lower_bound
    }
}

//make node trait and make root special for hash table. because that could be scary good.
#[derive(Debug)]
pub struct SuffixTree {
    /// The string we're indexing into
    string: String,
    /// Nodes in the tree
    nodes: Vec<Node>,

    /// Alphabet of chars we have seen.
    alphabet: String,
    /// Lookup table from char to the index in children which that char corresponds to
    alphabet_indexer: Vec<usize>,
    /// Node that needs to be suffix linked,
    need_sl: Option<usize>,

    // String tracking offset variables.
    /// How far we are into the string for construction.
    position: isize,
    /// How many characters from input need resolving yet.
    remainder: usize,

    // Node Tracking offset variables.
    /// What node were evaluating from
    active_node: usize,
    /// What edge were working in.
    active_edge: usize,
    /// How many into that edge/node (unsure)
    active_length: usize,
}

impl SuffixTree {
    pub fn new(string: &str) -> Self {
        let mut tree = Self {
            string: String::new(),
            nodes: vec![
                Node::new(0, Some(0), End::Index(0)), //Placeholder node
                Node::new(0, None, End::Root),
            ], //Root node
            alphabet: String::new(),
            alphabet_indexer: Vec::new(),
            need_sl: None,
            position: -1,
            remainder: 0,
            active_node: 1,
            active_edge: 0,
            active_length: 0,
        };
        tree.append_string(string);
        tree
    }

    fn extend_alphabet(&mut self, c: char) {
        // Using a lookup table this way wastes memory (256 or less if using ascii chars),
        // But up to 2^32 if using 4 byte chars. However, it makes char_index very fast
        // Which matters because it will be called often
        self.alphabet.push(c);
        //The char will always be u32 or smaller (intrinsic feature of char type)
        let table_index = c as usize;
        // 0 in the lookup table is just a placeholder
        // Also, resize will truncate, so this check is required
        if self.alphabet_indexer.len() < table_index + 1 {
            self.alphabet_indexer.resize(table_index + 1, 0);
        }
        self.alphabet_indexer[table_index] = self.alphabet.len() - 1;

        for node in self.nodes.iter_mut() {
            // Resize can truncate, but the alphabet size only increases so it isn't a problem
            node.children.resize(self.alphabet.len(), 0);
        }
    }

    pub fn append_string(&mut self, s: &str) {
        for c in s.chars() {
            self.extend(c);
            println!("{}", self);
        }
    }

    fn add_suffix_link(&mut self, node: usize) {
        if let Some(sl) = self.need_sl {
            if node == 0 {
                //Prevents making suffix links to root
                self.nodes[sl].suffix_link = None;
            } else {
                self.nodes[sl].suffix_link = Some(node);
            }
        }
        self.need_sl = Some(node);
    }

    fn walk_down(&mut self, node: usize) -> bool {
        let length = self.nodes[node].get_length(self.position as usize);
        if self.active_length >= length {
            self.active_edge += length;
            self.active_length -= length;
            self.active_node = node;
            return true;
        }
        false
    }

    fn text(&self, index: usize) -> char {
        // This is O(1) but will cause very wrong behavior if you use chars that are more than 1 byte
        self.string.as_bytes()[index] as char
    }
    fn text_index(&self, index: usize) -> usize {
        self.char_index(self.text(index))
    }
    fn char_index(&self, c: char) -> usize {
        self.alphabet_indexer[c as usize]
    }

    fn extend(&mut self, c: char) {
        let s = self; // Alias self to s for brevity
        s.position += 1;
        s.string.push(c);
        if !s.alphabet.contains(c) {
            s.extend_alphabet(c);
        }
        s.need_sl = None;
        s.remainder += 1;

        while s.remainder > 0 {
            if s.active_length == 0 {
                s.active_edge = s.position as usize;
            }
            if s.nodes[s.active_node].children[s.text_index(s.active_edge)] == 0 {
                let leaf = Node::new(s.alphabet.len(), Some(s.position as usize), End::End);
                s.nodes.push(leaf);
                let leaf_index = s.nodes.len() - 1;
                let active_edge_index = s.text_index(s.active_edge);
                s.nodes[s.active_node].children[active_edge_index] = leaf_index;
                s.add_suffix_link(s.active_node); // Rule 2
            } else {
                let next =
                    s.nodes[s.active_node].children[s.text_index(s.active_edge)];
                if s.walk_down(next) {
                    continue; // Observation 2
                }
                if s.text(s.nodes[next].start.unwrap() + s.active_length) == c {
                    // Observation 1
                    s.active_length += 1;
                    s.add_suffix_link(s.active_node); // Observation 3
                    break;
                }
                // Internal nodes are the only nodes with an end other than the end of the string
                let split = Node::new(
                    s.alphabet.len(),
                    Some(s.nodes[next].start.unwrap()),
                    // For the range represented by [start-end], end is exclusive
                    End::Index(s.nodes[next].start.unwrap() + s.active_length),
                );
                s.nodes.push(split);
                let split_index = s.nodes.len() - 1;
                let active_edge_index = s.text_index(s.active_edge);
                s.nodes[s.active_node].children[active_edge_index] = split_index;

                let leaf = Node::new(s.alphabet.len(), Some(s.position as usize), End::End);
                s.nodes.push(leaf);
                let leaf_index = s.nodes.len() - 1;
                let char_index = s.char_index(c);

                s.nodes[split_index].children[char_index] = leaf_index;
                s.nodes[next].start = Some(s.nodes[next].start.unwrap() + s.active_length);
                let next_char_index = s.text_index(s.nodes[next].start.unwrap());
                s.nodes[split_index].children[next_char_index] = next;
                s.add_suffix_link(split_index); // Rule 2
            }
            s.remainder -= 1;
            if s.active_node == ROOT && s.active_length > 0 {
                // Rule 1
                s.active_length -= 1;
                s.active_edge = s.position as usize - s.remainder + 1;
            } else {
                s.active_node = s.nodes[s.active_node].suffix_link.unwrap_or(ROOT);
                //Rule 3
            }
        }
    }

    pub fn find_substring(&self, substring: &str) -> (usize, usize) {
        let mut current_node: usize = 0; //start at root
        let mut index_in_node: usize = 0; //Node has no substring it refers to
        let mut chars_in_node: usize = 0;
        let mut match_size: usize = 0;

        for c in substring.chars() {
            if index_in_node + 1 >= chars_in_node {
                let child = self.nodes[current_node].children[self.char_index(c)];
                if child == 0 {
                    return (self.nodes[current_node].start.unwrap(), match_size);
                } else {
                    current_node = child;
                    chars_in_node = self.nodes[current_node].get_length(self.string.len());
                    index_in_node = 0;
                }
            } else if self
                .string
                .chars()
                .nth(self.nodes[current_node].start.unwrap() + index_in_node)
                .unwrap()
                == c
            {
                index_in_node += 1;
                match_size += 1;
                continue;
            } else {
                return (self.nodes[current_node].start.unwrap(), match_size);
            }
        }
        (0, 0)
    }
}

impl fmt::Display for SuffixTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Suffix tree for: {}", self.string)?;
        for (index, node) in self.nodes.iter().enumerate().skip(1) {
            let mut substring = String::new();
            if node.start.is_some() {
                let end: usize = match node.end {
                    End::Index(x) => x,
                    End::End => self.string.len(),
                    _ => 0,
                };
                substring.push_str(&self.string[node.start.unwrap()..end]);
            }
            writeln!(f, "{:<3} | {:<10} | {}", index, substring, node)?;
        }
        write!(f, "")
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start: String = match self.start {
            Some(x) => x.to_string(),
            None => "Root".to_string(),
        };
        let end: String = match self.end {
            End::Root => "Root".to_string(),
            End::Index(x) => (x - 1).to_string(),
            End::End => "End".to_string(),
        };
        let sl: String = match self.suffix_link {
            Some(x) => x.to_string(),
            None => "No SL".to_string(),
        };
        let children: Vec<&usize> = self.children.iter().filter(|x| **x != 0).collect();
        write!(f, "{:<6} | {:<6} | {:<6} | {:?}", start, end, sl, children)
    }
}

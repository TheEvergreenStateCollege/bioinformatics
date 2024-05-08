#![allow(dead_code, unused_variables)]

use std::borrow::BorrowMut;
enum End {
    Root,
    Infinity,
    Index(usize),
}

enum Start {
    Root,
    Index(usize),
}

struct Node {
    start: Start, // start of where this node exists in the string.
    end: End,     // end point of the node in the string
    suffix_link: Option<usize>,
    length: usize, // how many over. lets see if we can manip this in a smart way.
    children: Vec<usize>, // lets try by indexing instead of ownership, for now.

                   // should start be a vector of all instances or just the first one?
                   // like, would that be useful to the biologists?
}

impl Node {
    fn new(size: usize, start: Start, end: End) -> Self {
        Self {
            start,
            end,
            suffix_link: None,
            length: 0,
            children: vec![0; size],
            //is there someway the size of children can be determiend
            //by sharing the alphabet from the suffix tree?
        }
    }

    // data manip functions
    fn set_start(&mut self, start: usize) {
        self.start = Start::Index(start);
    }
    fn set_length(&mut self, length: usize) {
        self.length = length;
    }
    fn add_child<T>(mut self, child: T)
    // more proof curlys should always be on there own line!
    where
        T: IntoIterator<Item = usize>, // allows for singular usize or container of. need to read more on this.
    {
        let iter = child.into_iter();
        for value in iter {
            if !self.children.contains(&value) {
                self.children.push(value);
            }
        }
    }

    // I changed it to be in-line with the original function -
    // it will return 0 if you call it on the root
    fn get_length(&mut self, position: &usize) -> usize {
        // I assume the original C code expects not to have this function called on the root
        // Because it would return a negative number and that doesn't make sense
        let upper_bound = if let End::Index(i) = self.end {
            i
        } else {
            position + 1
        };
        if let Start::Index(lower_bound) = self.start {
            return upper_bound - lower_bound;
        };
        return 0;
    }
}
//make node trait and make root special for hash table. because that could be scary good.

struct SuffixTree {
    /// Our string were "pointing" into
    string: String,
    /// Nodes in the tree
    nodes: Vec<Node>,
    // the first element should always be the root. I think.

    //book keeping:
    /// Alphabet of chars we have seen.
    alphabet: String,
    /// Size of the alphabet
    size: usize,

    // uncertain about these being needed.
    /// Shortcut to index and maybe secret size value?
    last_added: Option<usize>,
    /// Node that needs to be suffix linked,
    need_sl: Option<usize>,

    // String tracking offset variables.
    /// How far we are into the string for construction.
    position: usize,
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
    fn new(size: usize) -> Self {
        Self {
            string: String::new(),
            nodes: vec![Node::new(size, Start::Root, End::Root)],

            alphabet: String::new(),
            size,

            last_added: None,
            need_sl: None,

            position: 0, //This is -1 in the original code - how do we handle the same case?
            remainder: 0,

            active_node: 0,
            active_edge: 0,
            active_length: 0,
        }
    }

    fn char_index(&self, c: char) -> usize {
        for (index, val) in self.alphabet.chars().enumerate() {
            if val == c {
                return index;
            }
        }
        return 0;
    }

    fn append_string(&mut self, s: &str) {
        self.string.push_str(s);
        self.remainder += s.len();
    }

    fn add_suffix_link(&mut self, node: usize) {
        if let Some(sl) = self.need_sl {
            self.nodes[sl].suffix_link = Some(node);
        }
        self.need_sl = Some(node);
    }

    fn walk_down(&mut self, node: usize) -> bool {
        let length = self.nodes[node].get_length(&self.position);
        if self.active_length >= length {
            if self.active_edge == 0 {
                return false; // if not were probably at root. so were not walking down.
            }
            self.active_edge += length;
            self.active_length -= length;
            self.active_node = node;
            return true;
        }
        return false;
    }

    fn extend(&mut self) {
        self.need_sl = None;
        // Increment the remainder to account for the char waiting to be inserted
        self.remainder += 1;
        self.position += 1;
        let c = self.string.chars().nth(self.position).unwrap();

        while self.remainder > 0 {
            if self.active_length == 0 {
                self.active_edge = self.position;
            }
            // This is true if the edge that would contain the index of the child node corresponding to the
            // char we are adding is 0, meaning no such node exists
            if self.nodes[self.active_node].children[self.active_edge] == 0 {
                let leaf = Node::new(self.size, Start::Index(self.position), End::Infinity);
                self.nodes.push(leaf);
                let leaf_index = self.nodes.len() - 1;
                // Change the value of the edge to the index of the new node,
                // Which is len() because the new node is at the end
                self.nodes[self.active_node].children[self.active_edge] = leaf_index;
                self.add_suffix_link(self.active_node);
            } else {
                let next = self.nodes[self.active_node].children[self.active_edge];
                if self.walk_down(next) {
                    continue;
                }
                // This should always be true, because next is a child of another node, and
                // start would only be Root if it belongs to the root
                if let Start::Index(start) = self.nodes[next].start {
                    if self.string.chars().nth(start + self.active_length).unwrap() == c {
                        self.active_length += 1;
                        self.add_suffix_link(self.active_node);
                        break;
                    }
                    let split = Node::new(
                        self.size,
                        Start::Index(start),
                        End::Index(start + self.active_length),
                    );
                    self.nodes.push(split);
                    let i = self.char_index(self.string.chars().nth(self.active_edge).unwrap());
                    let split_index = self.nodes.len() - 1;
                    self.nodes[self.active_node].children[i] = split_index;

                    let leaf = Node::new(self.size, Start::Index(self.position), End::Infinity);
                    self.nodes.push(leaf);
                    let leaf_index = self.nodes.len() - 1;

                    let char_index = self.char_index(c);

                    self.nodes[split_index].children[char_index] = leaf_index;
                    if let Start::Index(sti) = self.nodes[next].start {
                        self.nodes[next].start = Start::Index(sti + self.active_length);
                        let next_char_index = self.char_index(self.string.chars().nth(sti).unwrap());
                        self.nodes[split_index].children[next_char_index] = next;
                        self.add_suffix_link(split_index);
                    };
                }
            }
            self.remainder -= 1;
            if self.active_node == 0 && self.active_length > 0 { //0 is the root node
                self.active_length -= 1;
                self.active_edge = self.position - self.remainder + 1;
            } else {
                self.active_node = if self.nodes[self.active_node].suffix_link > Some(0) {
                    self.nodes[self.active_node].suffix_link.unwrap()
                } else {
                    0 //Root node
                }
            }
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn make_tree() {
        let tree = SuffixTree::new(4);
    }
}

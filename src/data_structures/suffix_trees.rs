enum End {
    Root,
    Infinity,
    Index(usize),
}

// We should be able to find all occurences of a substring in a string,
// not just the first one. Not sure how to implement that.
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
    fn get_length(&mut self, position: &usize) -> usize {
        let upper_bound = match self.end {
            //I'm not sure if the min is requred after accounting for infinity
            End::Index(i) => std::cmp::min(i, position + 1),
            End::Infinity => position + 1,
            End::Root => panic!("Tried to get end of root"),
        };
        let lower_bound = self.start.expect("Tried to get start of root");
        return upper_bound - lower_bound;
    }
}
//make node trait and make root special for hash table. because that could be scary good.

struct SuffixTree {
    /// The string we're indexing into
    string: String,
    /// Nodes in the tree
    nodes: Vec<Node>,

    /// Alphabet of chars we have seen.
    alphabet: String,
    /// Size of the alphabet
    size: usize,

    // uncertain about these being needed.
    // Shortcut to index and maybe secret size value?
    // last_added: Option<usize>,
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
            nodes: vec![Node::new(size, None, End::Root)],
            alphabet: String::new(),
            size,
            // last_added: None,
            need_sl: None,
            position: 0,
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
                return false; // if not we're probably at root. so we're not walking down.
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
        let c = self.string.chars().nth(self.position).unwrap();

        while self.remainder > 0 {
            if self.active_length == 0 {
                self.active_edge = self.position;
            }
            // Children contains indicies into a vec containing all nodes. If the index is 0, it means that there is no such node
            if self.nodes[self.active_node].children[self.active_edge] == 0 {
                let leaf = Node::new(self.size, Some(self.position), End::Infinity);
                self.nodes.push(leaf);
                let leaf_index = self.nodes.len() - 1;
                self.nodes[self.active_node].children[self.active_edge] = leaf_index;
                self.add_suffix_link(self.active_node);
            } else {
                let next = self.nodes[self.active_node].children[self.active_edge];
                if self.walk_down(next) {
                    continue;
                }
                let start = self.nodes[next]
                    .start
                    .expect("Tried to access start in root");
                if self
                    .string
                    .chars()
                    .nth(start + self.active_length)
                    .expect("start + active_length out of bounds in string")
                    == c
                {
                    self.active_length += 1;
                    self.add_suffix_link(self.active_node);
                    break;
                }
                let split = Node::new(
                    self.size,
                    Some(start),
                    End::Index(start + self.active_length),
                );
                self.nodes.push(split);
                let active_edge_index = self.char_index(
                    self.string
                        .chars()
                        .nth(self.active_edge)
                        .expect("active_edge out of bounds in string"),
                );
                let split_index = self.nodes.len() - 1;
                self.nodes[self.active_node].children[active_edge_index] = split_index;
                let leaf = Node::new(self.size, Some(self.position), End::Infinity);
                self.nodes.push(leaf);
                let leaf_index = self.nodes.len() - 1;
                let char_index = self.char_index(c);
                self.nodes[split_index].children[char_index] = leaf_index;
                self.nodes[next].start = Some(start + self.active_length);
                let next_char_index = self.char_index(self.string.chars().nth(start).unwrap());
                self.nodes[split_index].children[next_char_index] = next;
                self.add_suffix_link(split_index);
            }
            self.remainder -= 1;
            if self.active_node == 0 && self.active_length > 0 {
                //0 is the index of the root node
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
        // The original code has position start at -1 and increments at the start of extend,
        // but we use a usize so we start at 0 and increment at the end
        self.position += 1;
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn make_tree() {
        let tree = SuffixTree::new(4);
    }
}

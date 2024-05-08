#[derive(Debug)]
enum End {
    Root,
    Infinity,
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
    fn get_length(&mut self, position: &usize) -> usize {
        let upper_bound = match self.end {
            //I'm not sure if the min is requred after accounting for infinity
            End::Index(i) => {if i < position + 1 {return i;} else {println!("Position + 1 was smaller!"); return position + 1;}},
            End::Infinity => position + 1,
            End::Root => panic!("Tried to get end of root"),
        };
        let lower_bound = self.start.expect("Tried to get start of root");
        return upper_bound - lower_bound;
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
    /// Size of the alphabet
    alphabet_size: usize,
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
    pub fn new(string: &str, alphabet_size: usize) -> Self {
        let mut tree = Self {
            string: String::new(),
            nodes: vec![Node::new(alphabet_size, None, End::Root)],
            alphabet: String::new(),
            alphabet_size,
            need_sl: None,
            position: 0,
            remainder: 0,
            active_node: 0,
            active_edge: 0,
            active_length: 0,
        };
        tree.append_string(string);
        tree
    }

    fn char_index(&self, c: char) -> usize {
        for (index, val) in self.alphabet.chars().enumerate() {
            if val == c {
                return index;
            }
        }
        return 0;
    }

    pub fn append_string(&mut self, s: &str) {
        for c in s.chars() {
            self.extend(c);
        }
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

    //Original extend adds the char to the string, but ours assumes it has already been added
    fn extend(&mut self, c: char) {
        self.string.push(c);
        self.need_sl = None;
        // Increment the remainder to account for the char waiting to be inserted
        self.remainder += 1;

        while self.remainder > 0 {
            if self.active_length == 0 {
                self.active_edge = self.position;
            }
            // Children contains indicies into a vec containing all nodes. If the index is 0, it means that there is no such node
            if self.nodes[self.active_node].children[self.active_edge] == 0 {
                let leaf = Node::new(self.alphabet_size, Some(self.position), End::Infinity);
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
                    self.alphabet_size,
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
                let leaf = Node::new(self.alphabet_size, Some(self.position), End::Infinity);
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
            //0 is the index of the root node
            if self.active_node == 0 && self.active_length > 0 {
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

    pub fn find_substring(&mut self, substring: &str) -> (usize, usize) {
        let mut current_node: usize = 0; //start at root
        let mut index_in_node: usize = 0; //Node has no substring it refers to
        let mut chars_in_node: usize = 0;
        let mut match_size: usize = 0;

        for c in substring.chars() {
            if index_in_node == chars_in_node - 1 {
                let child = self.nodes[current_node].children[self.char_index(c)];
                if child == 0 {
                    return (self.nodes[current_node].start.unwrap(), match_size);
                } else {
                    current_node = child;
                    chars_in_node = self.nodes[current_node].get_length(&self.string.len());
                    index_in_node = 0;
                }
            } else {
                if self.string.chars().nth(self.nodes[current_node].start.unwrap() + index_in_node).unwrap() == c {
                    index_in_node += 1;
                    match_size += 1;
                    continue;
                } else {
                    return (self.nodes[current_node].start.unwrap(), match_size);
                }
            }
        }
        (0, 0)
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn make_tree() {
        let tree = SuffixTree::new(4);
    }
}

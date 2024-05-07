#![allow(dead_code, unused_variables)]
//thing to extancieate copy / clone?

const ALPHABET_SIZE: u32 = 4;

struct Node {
    start: Option<usize>, // start of where this node exists in the string.
    end: Option<usize>, // end point, none if not set.
    suffix_link: Option<usize>,
    length: usize,        // how many over. lets see if we can manip this in a smart way.
    children: Vec<usize>, // lets try by indexing instead of ownership, for now.

                          // should start be a vector of all instances or just the first one?
                          // like, would that be useful to the biologists?
}

impl Node {
    fn new(size: usize) -> Self {
        Self {
            start: None, //because root isnt in the string. Technically.
            end: None,
            suffix_link: None,
            length: 0,
            children: vec![0; size],
            //is there someway the size of children can be determiend
            //by sharing the alphabet from the suffix tree?
        }
    }
    // data manip functions
    fn set_start(&mut self, start: usize) {
        self.start = Some(start);
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
    //data retrive functions.
    fn get_length(&mut self, position: &usize) -> Option<usize> {
        match (self.end, self.start) {
            (Some(start), None) => {
                Some((position + 1) - start) //plus one needed?
            }
            (Some(start), Some(end)) => {
                Some(end - start) //does plus one need exist?
            }
            _ => None,
        }
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
    active_edge: Option<usize>,
    /// How many into that edge/node (unsure)
    active_length: usize,
}

impl SuffixTree {
    fn new(size: usize) -> Self {
        Self {
            string: String::new(),
            nodes: vec![Node::new(size)],

            alphabet: String::new(),
            size,

            last_added: None,
            need_sl: None,

            position: 0,
            remainder: 0,

            active_node: 0,
            active_edge: None,
            active_length: 0,
        }
    }

    fn append_string(mut self, s: &str) {
        //I have no idea if this is right the linter just corrected my code 5 times into this
        self.string.push_str(s);
        self.remainder += s.len() as usize;
    }

    fn add_suffix_link(&mut self, node: usize) {
        if let Some(sl) = self.need_sl  { self.nodes[sl].suffix_link = Some(node);}
        self.need_sl = Some(node);
    }

    fn walk_down(&mut self, node: usize) -> bool {
        if let Some(length) = self.nodes[node].get_length(&self.position) {
            if self.active_length >= length {
                match self.active_edge.as_mut() {
                    // as active edge might not be set
                    Some(ae) => *ae += length,
                    _ => return false, // if not were probably at root. so were not walking down.
                }
                self.active_length -= length;
                self.active_node = node;
                return true;
            }
        }
        return false;
    }

    fn extend(mut self) {
        self.need_sl = None;
        // Increment the remainder to account for the char waiting to be inserted
        self.remainder += 1;
        self.position += 1;

        while self.remainder > 0 {
            if self.active_length == 0 {
                self.active_edge = Some(self.position);
            }
            // If there isn't an edge we add a
            if let Some(edge) = self.active_edge {
                if self.nodes[self.active_node].children[edge] == 0 {
                    let mut leaf = Node::new(self.size);
                    leaf.set_start(self.position);
                    self.nodes.push(leaf);
                    // Change the value of the edge to the index of the new node,
                    // Which is len() because the new node is at the end
                    self.nodes[self.active_node].children[edge] = self.nodes.len();
                    self.add_suffix_link(self.active_node);
                }
            }  else {
                
            }
        }
    }
}

//
//new_node DONE
//edge length pointer skerting.
//active edge? NO, thats lame... why????
//add_SL (add suffix Link.)
//
//walk down
//
//suffix tree init, DONE?
//
//extend / add char,

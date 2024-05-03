#![allow(dead_code, unused_variables)]
//thing to extancieate copy / clone?

struct Node {
    start: Option<usize>, // start of where this node exists in the string.
    end: Option<usize>,   // end point, none if not set.
    length: usize,        // how many over. lets see if we can manip this in a smart way.
    children: Vec<usize>, // lets try by indexing instead of ownership, for now.

                          // should start be a vector of all instances or just the first one?
                          // like, would that be useful to the biologists?
}

impl Node {
    fn new() -> Self {
        Self {
            start: None, //because root isnt in the string. Technically.
            end: None,
            length: 0,
            children: Vec::<usize>::new(),
            //is there someway the size of children can be determiend
            //by sharing the alphabet from the suffix tree?
        }
    }
    // data manip functions
    fn set_start(mut self, start: usize) -> Self {
        self.start = Some(start);
        self
    }
    fn set_length(mut self, length: usize) -> Self {
        self.length = length;
        self
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
    //root: Node, //changed in favor for indexing, for now...

    // data members:
    string: String, //Our string were "pointing" into.
    // Would be cool if we didnt need to own this.
    nodes: Vec<Node>,
    // the first element should always be the root. I think.

    //book keeping:
    alphabet: String, // To know how many characters we have seen before.

    // uncertain about these being needed.
    last_added: Option<usize>, // Shortcut to index and maybe secret size value?
    need_sl: Option<usize>,    // Node that needs to be suffix linked,

    // String tracking offset variables.
    position: usize,  // How far we are into the string for construction.
    remainder: usize, // How many characters from input need resolving yet.

    // Node Tracking offset variables.
    active_node: usize,         // What node were evaluating from
    active_edge: Option<usize>, // What edge were working in.
    active_length: usize,       // How many into that edge/node (unsure)
}

impl SuffixTree {
    fn new() -> Self {
        let mut new_self = Self {
            //make root I guess?
            string: String::new(),
            nodes: Vec::<Node>::new(),

            alphabet: String::new(),

            last_added: None,
            need_sl: None,

            position: 0,
            remainder: 0,

            active_node: 0,
            active_edge: None,
            active_length: 0,
        };
        new_self.nodes.push(Node::new());
        return new_self;
    }

    fn append_string(mut self, s: &str) {
        //I have no idea if this is right the linter just corrected my code 5 times into this
        self.string.push_str(s);
        self.remainder += s.len() as usize;
    }

    fn walk_down(&mut self, node: usize) -> bool {
        if let Some(length) = self.nodes[node].get_length(&self.position) {
            if self.active_length >= length {
                match self.active_edge.as_mut() { // as active edge might not be set
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
        while self.remainder > 0 {
            if self.active_length == 0 {
                self.active_edge = Some(self.position);
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

//thing to extancieate copy / clone?


use std::cmp::min;

struct Node {
    start:    Option<u32>, // start of where this node exists in the string. 
    end:      Option<u32>, // end point, none if not set. 
    length:   u32,         // how many over. lets see if we can manip this in a smart way.
    children: Vec<u32>,    // lets try by indexing instead of ownership, for now.
    
    // should start be a vector of all instances or just the first one? 
    // like, would that be useful to the biologists? 
}

impl Node{
    fn new() -> Self {
        Self {
            start: None, //because root isnt in the string. Technically.
            end: None,
            length: 0,
            children: Vec::<u32>::new(), 
            //is there someway the size of children can be determiend 
            //by sharing the alphabet from the suffix tree? 
        }
    }
    // data manip functions
    fn set_start(mut self, start: u32) -> Self {
        self.start =  Some(start);
        self

    }
    fn set_length(mut self, length: u32 ) -> Self {
        self.length = length;
        self

    }
    fn add_child<T>(mut self, child: T) // more proof curlys should always be on there own line!
    where
        T: IntoIterator<Item = u32>, // allows for singular u32 or container of. need to read more on this.
    {
        let iter = child.into_iter();
        for value in iter {
            if !self.children.contains(&value)
            {
                self.children.push(value);
            }
        }
    }
    //data retrive functions. 
    fn get_length(mut self, position: &u32) -> Option<u32> {
        match (self.end, self.start) {
            (Some(start), None)  => {
                Some((position + 1) - start)
            },
            (Some(start), Some(end)) => {
                Some(min(end, position + 1) - start)
            },
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
    last_added: Option<u32>,   // Shortcut to index and maybe secret size value? 
    need_sl: Option<u32>,      // Node that needs to be suffix linked, 

    // String tracking offset variables.
    position: Option<u32>, // How far we are into the string for construction. 
    remainder: u32,        // How many characters from input need resolving yet.

    // Node Tracking offset variables.
    active_node: u32,   // What node were evaluating from
    active_child: u32,  // What edge were working in.
    active_length: u32, // How many into that edge/node (unsure)

}

impl SuffixTree{
    fn new() -> Self {
        Self{
            //make root I guess?
            string: String::new(),
            nodes: Vec::<Node>::new(),

            alphabet: String::new(),

            last_added: None,
            need_sl: None,

            position : None,
            remainder: 0,

            active_node:   0,
            active_child:  0,
            active_length: 0,

        }
        
    }
    fn append_string(mut self, s: &str) { 
    //I have no idea if this is right the linter just corrected my code 5 times into this
        self.string.push_str(s);
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
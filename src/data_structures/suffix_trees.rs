struct Node {
    start: u32,         // start of where this node exists in the string. 
    length: u32,        // how many over. lets see if we can manip this in a smart way.
    children: Vec<u32>, // lets try by indexing instead of ownership, for now.
    
    // should start be a vector of all instances or just the first one? 
    // like, would that be useful to the biologists? 
}

//    needs:
// new_node()

struct SuffixTree {
    //root: Node, //changed in favor for indexing, again, for now...
    
    // data members:
    string: String, //Our string were "pointing" into. 
    // Would be cool if we didnt need to own this. 
    nodes: Vec<Node>,
    // the first element should always be the root. I think. 

    //book keeping:
    alphabet: String, // To know how many characters we have seen before. 

    // uncertain about these being needed.
    last_added: u32,   // Shortcut to index and maybe secret size value? 
    need_sl: u32,      // Node that needs to be suffix linked, 

    // String tracking offset variables.
    position: u32,      // How far we are into the string for construction.
    remainder: u32,     // How many characters from input need resolving yet.

    // Node Tracking offset variables.
    active_node: u32,   // What node were evaluating from
    active_child: u32,  // What edge were working in.
    active_length: u32, // How many into that edge/node (unsure)

}

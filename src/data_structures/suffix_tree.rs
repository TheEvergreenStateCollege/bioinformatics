use core::fmt;

const INF: usize = usize::MAX;
const ALPHABET_SIZE: usize = 256; // Deviation - signed int in C++
const ROOT: usize = 1;

struct Node
{
    start: usize,
    end: usize,
    suffix_link: usize,
    children: [usize; ALPHABET_SIZE],
}

impl Node
{
    fn new(start: usize, end: usize) -> Node { Node {start, end, suffix_link: 0, children: [0; ALPHABET_SIZE]}}

    fn edge_length(&self, position: usize) -> usize
    {
        std::cmp::min(self.end, position + 1) - self.start
    }
}

pub struct SuffixTree
{
    nodes: Vec<Node>, // Deviation - used to be array
    text: Vec<char>, // Deviation - used to be array
    last_added: usize,
    position: usize,
    need_suffix_link: usize,
    tree_remainder: usize,
    node_active: usize,
    edge_active: usize,
    length_active: usize,
    first_loop_flag: bool,
}
impl SuffixTree
{
    pub fn new() -> SuffixTree
    {
        let mut st = SuffixTree
        {
            nodes: Vec::new(),
            text: Vec::new(),
            last_added: 1,
            need_suffix_link: 0,
            tree_remainder: 0,
            edge_active: 0,
            length_active: 0,
            position: 0,
            node_active: 1, // Index of root node
            first_loop_flag: true,
        };
        st.nodes.push(Node::new(0,0));
        st.nodes.push(Node::new(0,0));
        st
    }

    fn new_node(&mut self, start: usize, end: usize) -> usize
    {
        let nd = Node::new(start, end);
        self.last_added += 1;
        self.nodes.push(nd);
        return self.last_added;
    }

    fn add_suffix_link(&mut self, node: usize)
    {
        if self.need_suffix_link > 0 
        {
            self.nodes[self.need_suffix_link].suffix_link = node;
        }
        self.need_suffix_link = node;
    }
    
    fn walk_down(&mut self, node: usize) -> bool
    {
        if self.length_active >= self.nodes[node].edge_length(self.position)
        {
            self.edge_active += self.nodes[node].edge_length(self.position);
            dbg!(self.length_active);
            dbg!(self.nodes[node].edge_length(self.position));
            self.length_active -= self.nodes[node].edge_length(self.position);
            self.node_active = node;
            return true;
        }
        return false;
    }

    pub fn extend(&mut self, c: char)
    {
        let s = self;

        match s.first_loop_flag { // DEVIATION
            true => s.first_loop_flag = false,
            false => s.position += 1,
        }

        s.text.push(c);
        s.need_suffix_link = 0;
        s.tree_remainder += 1;
        
        while s.tree_remainder > 0
        {
            if s.length_active == 0 {
                s.edge_active = s.position;}
            if s.nodes[s.node_active].children[s.text[s.edge_active] as usize] == 0
            {
                let leaf = s.new_node(s.position, INF);
                s.nodes[s.node_active].children[s.text[s.edge_active] as usize] = leaf;
                s.add_suffix_link(s.node_active);
            }
            else
            {
                let next = s.nodes[s.node_active].children[s.text[s.edge_active] as usize];
                if s.walk_down(next) {
                    continue;}
                if s.text[s.nodes[next].start + s.length_active] == c
                {
                    s.length_active += 1;
                    s.add_suffix_link(s.node_active);
                    break;
                }
                let split = s.new_node(s.nodes[next].start, s.nodes[next].start + s.length_active);
                s.nodes[s.node_active].children[s.text[s.edge_active] as usize] = split;

                let leaf = s.new_node(s.position, INF);
                s.nodes[split].children[c as usize] = leaf;
                s.nodes[next].start += s.length_active;
                let next_start = s.nodes[next].start; // Deviation due to borrow checker
                s.nodes[split].children[s.text[next_start] as usize] = next;
                s.add_suffix_link(split);
            }
            s.tree_remainder -= 1;
            if s.node_active == ROOT && s.length_active > 0
            {
                s.length_active -= 1;
                s.edge_active = s.position - s.tree_remainder + 1;
            }
            else {
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
        writeln!(f, "Suffix tree for: {}", self.text.iter().collect::<String>())?;
        for (index, node) in self.nodes.iter().enumerate().skip(1) {
            let mut substring = String::new();
            match node.end {
                0 => (),
                INF => substring.push_str(&self.text.iter().collect::<String>()[node.start..]),
                x => substring.push_str(&self.text.iter().collect::<String>()[node.start..x]),
            }
            writeln!(f, "{:<3} | {:<10} | {}", index, substring, node)?;
        }
        write!(f, "")
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start: String = match self.start {
            0 => "Root".to_string(),
            x => x.to_string(),
        };
        let end: String = match self.end {
            0 => "Root".to_string(),
            INF => "End".to_string(),
            x => x.to_string(),
        };
        let sl: String = match self.suffix_link {
            0 => "No SL".to_string(),
            x => x.to_string(),
        };
        let children: Vec<&usize> = self.children.iter().filter(|x| **x != 0).collect();
        write!(f, "{:<6} | {:<6} | {:<6} | {:?}", start, end, sl, children)
    }
}
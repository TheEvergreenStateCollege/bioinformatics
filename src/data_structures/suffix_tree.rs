use core::fmt;

const INF: usize = usize::MAX;
const ALPHABET_SIZE: usize = 256;
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
    nodes: Vec<Node>,
    text: Vec<u8>,
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
            node_active: ROOT,
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
            self.length_active -= self.nodes[node].edge_length(self.position);
            self.node_active = node;
            return true;
        }
        return false;
    }

    pub fn extend(&mut self, char_to_add: u8)
    {
        let s = self;

        match s.first_loop_flag {
            true => s.first_loop_flag = false,
            false => s.position += 1,
        }

        s.text.push(char_to_add); // I think this cast will fail for multi-byte chars
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
                if s.text[s.nodes[next].start + s.length_active] == char_to_add
                {
                    s.length_active += 1;
                    s.add_suffix_link(s.node_active);
                    break;
                }
                let split = s.new_node(s.nodes[next].start, s.nodes[next].start + s.length_active);
                s.nodes[s.node_active].children[s.text[s.edge_active] as usize] = split;

                let leaf = s.new_node(s.position, INF);
                s.nodes[split].children[char_to_add as usize] = leaf;
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
        writeln!(f, "Suffix tree for: {}", self.text.iter().map(|x| *x as char).collect::<String>())?;
        for (index, node) in self.nodes.iter().enumerate().skip(1) {
            let mut substring = String::new();
            match node.end {
                0 => (),
                INF => substring.push_str(&self.text.iter().map(|x| *x as char).collect::<String>()[node.start..]),
                x => substring.push_str(&self.text.iter().map(|x| *x as char).collect::<String>()[node.start..x]),
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test] // This test looks disgusting but should ensure that the result is correct
    fn test_parity() {
        let mut st = SuffixTree::new();
        for c in "xefisbfgouerfiuwehiuwerfiweuhfwioufxefxfis".chars().into_iter() {
            st.extend(c as u8);
        }
        assert_eq!(st.to_string(), "Suffix tree for: xefisbfgouerfiuwehiuwerfiweuhfwioufxefxfis
1   |            | Root   | Root   | No SL  | [7, 13, 8, 10, 40, 18, 46, 33, 6, 20, 43, 56]
2   | isbfgouerfiuwehiuwerfiweuhfwioufxefxfis | 3      | End    | No SL  | []
3   | isbfgouerfiuwehiuwerfiweuhfwioufxefxfis | 3      | End    | No SL  | []
4   | sbfgouerfiuwehiuwerfiweuhfwioufxefxfis | 4      | End    | No SL  | []
5   | sbfgouerfiuwehiuwerfiweuhfwioufxefxfis | 4      | End    | No SL  | []
6   | sbfgouerfiuwehiuwerfiweuhfwioufxefxfis | 4      | End    | No SL  | []
7   | bfgouerfiuwehiuwerfiweuhfwioufxefxfis | 5      | End    | No SL  | []
8   | f          | 2      | 3      | 1      | [9, 16, 42, 54]
9   | gouerfiuwehiuwerfiweuhfwioufxefxfis | 7      | End    | No SL  | []
10  | gouerfiuwehiuwerfiweuhfwioufxefxfis | 7      | End    | No SL  | []
11  | erfiuwehiuwerfiweuhfwioufxefxfis | 10     | End    | No SL  | []
12  | erfiuwehiuwerfiweuhfwioufxefxfis | 10     | End    | No SL  | []
13  | e          | 1      | 2      | 1      | [52, 23, 31, 38]
14  | uwehiuwerfiweuhfwioufxefxfis | 14     | End    | No SL  | []
15  | uwehiuwerfiweuhfwioufxefxfis | 14     | End    | No SL  | []
16  | i          | 3      | 4      | 18     | [4, 17, 35]
17  | uwehiuwerfiweuhfwioufxefxfis | 14     | End    | No SL  | []
18  | i          | 3      | 4      | 1      | [45, 5, 25, 36]
19  | hiuwerfiweuhfwioufxefxfis | 17     | End    | No SL  | []
20  | u          | 9      | 10     | 1      | [12, 48, 39, 27]
21  | hiuwerfiweuhfwioufxefxfis | 17     | End    | No SL  | []
22  | hiuwerfiweuhfwioufxefxfis | 17     | End    | No SL  | []
23  | hiuwerfiweuhfwioufxefxfis | 17     | End    | No SL  | []
24  | iuwerfiweuhfwioufxefxfis | 18     | End    | No SL  | []
25  | uwe        | 14     | 17     | 27     | [19, 26]
26  | rfiweuhfwioufxefxfis | 22     | End    | No SL  | []
27  | we         | 15     | 17     | 29     | [21, 28]
28  | rfiweuhfwioufxefxfis | 22     | End    | No SL  | []
29  | e          | 16     | 17     | 13     | [22, 30, 37]
30  | rfiweuhfwioufxefxfis | 22     | End    | No SL  | []
31  | rfi        | 11     | 14     | 33     | [14, 32]
32  | weuhfwioufxefxfis | 25     | End    | No SL  | []
33  | rfi        | 11     | 14     | 16     | [15, 34]
34  | weuhfwioufxefxfis | 25     | End    | No SL  | []
35  | weuhfwioufxefxfis | 25     | End    | No SL  | []
36  | weuhfwioufxefxfis | 25     | End    | No SL  | []
37  | uhfwioufxefxfis | 27     | End    | No SL  | []
38  | uhfwioufxefxfis | 27     | End    | No SL  | []
39  | hfwioufxefxfis | 28     | End    | No SL  | []
40  | h          | 17     | 18     | 1      | [41, 24]
41  | fwioufxefxfis | 29     | End    | No SL  | []
42  | wioufxefxfis | 30     | End    | No SL  | []
43  | w          | 15     | 16     | 1      | [29, 44]
44  | ioufxefxfis | 31     | End    | No SL  | []
45  | oufxefxfis | 32     | End    | No SL  | []
46  | ou         | 8      | 10     | 20     | [11, 47]
47  | fxefxfis   | 34     | End    | No SL  | []
48  | fxefxfis   | 34     | End    | No SL  | []
49  | efxfis     | 36     | End    | No SL  | []
50  | ef         | 1      | 3      | 52     | [2, 51]
51  | xfis       | 38     | End    | No SL  | []
52  | f          | 2      | 3      | 8      | [3, 53]
53  | xfis       | 38     | End    | No SL  | []
54  | x          | 35     | 36     | 56     | [49, 55]
55  | fis        | 39     | End    | No SL  | []
56  | x          | Root   | 1      | 1      | [50, 57]
57  | fis        | 39     | End    | No SL  | []
")
    }
    #[test]
    fn test_parity_2() {
        let mut st = SuffixTree::new();
        for c in "xabxac".chars().into_iter() {
            st.extend(c as u8);
        }
        assert_eq!(st.to_string(), "Suffix tree for: xabxac
1   |            | Root   | Root   | No SL  | [7, 4, 9, 5]
2   | bxac       | 2      | End    | No SL  | []
3   | bxac       | 2      | End    | No SL  | []
4   | bxac       | 2      | End    | No SL  | []
5   | xa         | Root   | 2      | 7      | [2, 6]
6   | c          | 5      | End    | No SL  | []
7   | a          | 1      | 2      | 1      | [3, 8]
8   | c          | 5      | End    | No SL  | []
9   | c          | 5      | End    | No SL  | []
");
}
}
#include <iostream>
#include <stdio.h>
#include <string>
#include <limits.h>

using namespace std;

const int INF = INT_MAX;
const int ALPHABET_SIZE = 256;
const int MAX_NODES = 100;
const int ROOT = 1;

struct Node
{
    int start;
    int end;
    int suffix_link;
    int next[ALPHABET_SIZE];

    Node(int start, int end = INF) : start(start), end(end), suffix_link(0) {}
    Node() : start(-1), end(-1), suffix_link(0) {}

    int edge_length(int position)
    {
        return min(end, position + 1) - start;
    }
};

class SuffixTree
{
    Node nodes[2 * MAX_NODES];
    char text[MAX_NODES];
    int last_added;
    int position;
    int need_suffix_link;
    int tree_remainder;
    int active_node;
    int active_edge;
    int active_length;

public:
    SuffixTree()
    {
        last_added = 1,
        need_suffix_link = 0,
        tree_remainder = 0,
        active_edge = 0,
        active_length = 0,
        position = -1,
        active_node = 1,
        nodes[0] = Node(0, 0);
        nodes[1] = Node(-1, -1);
    }

    int new_node(int start, int end = INF)
    {
        Node nd = Node(start, end);
        for (int i = 0; i < ALPHABET_SIZE; i++)
            nd.next[i] = 0;
        nodes[++last_added] = nd;
        return last_added;
    }

    void add_SL(int node)
    {
        if (need_suffix_link > 0)
            nodes[need_suffix_link].suffix_link = node;
        need_suffix_link = node;
    }

    bool walk_down(int node)
    {
        if (active_length >= nodes[node].edge_length(position))
        {
            active_edge += nodes[node].edge_length(position);
            active_length -= nodes[node].edge_length(position);
            active_node = node;
            return true;
        }
        return false;
    }

    void extend(char c)
    {
        text[++position] = c;
        need_suffix_link = 0;
        tree_remainder++;
        while (tree_remainder > 0)
        {
            if (active_length == 0)
                active_edge = position;
            if (nodes[active_node].next[(int)text[active_edge]] == 0)
            {
                int leaf = new_node(position);
                nodes[active_node].next[(int)text[active_edge]] = leaf;
                add_SL(active_node); // rule 2
            }
            else
            {
                int nxt = nodes[active_node].next[(int)text[active_edge]];
                if (walk_down(nxt))
                    continue; // observation 2
                if (text[nodes[nxt].start + active_length] == c)
                { // observation 1
                    active_length++;
                    add_SL(active_node); // observation 3
                    break;
                }
                int split = new_node(nodes[nxt].start, nodes[nxt].start + active_length);
                nodes[active_node].next[(int)text[active_edge]] = split;

                int leaf = new_node(position);
                nodes[split].next[(int)c] = leaf;
                nodes[nxt].start += active_length;
                nodes[split].next[(int)text[nodes[nxt].start]] = nxt;
                add_SL(split); // rule 2
            }
            tree_remainder--;
            if (active_node == ROOT && active_length > 0)
            { // rule 1
                active_length--;
                active_edge = position - tree_remainder + 1;
            }
            else
                active_node = nodes[active_node].suffix_link > 0 ? nodes[active_node].suffix_link : ROOT; // rule 3
        }
    }
    void print()
    {
        printf("Suffix tree for: %s\n", text);
        int i = 1; // Skips placeholder node
        while (i < last_added + 1)
        {
            Node n = nodes[i];
            printf("%-3d |", i);
            // Pointer arithmetic plus format specifier for substrings to print string slice
            printf(" %-10.*s |", n.end - n.start, text + n.start);

            if (n.start == -1)
            {
                printf(" Root   |");
            }
            else
            {
                printf(" %-6d |", n.start);
            }

            if (n.end == INF)
            {
                printf(" End    |");
            }
            else if (n.end == -1)
            {
                printf(" Root   |");
            }
            else
            {
                printf(" %-6d |", n.end);
            }

            if (n.suffix_link == 0)
            {
                printf(" No SL  |");
            }
            else
            {
                printf(" %-6d |", n.suffix_link);
            }

            printf(" [");
            bool comma_flag = false;
            for (int j = 0; j < ALPHABET_SIZE; j++)
            {
                if (n.next[j] != 0)
                {
                    if (comma_flag)
                    {
                        printf(", ");
                    }
                    else
                    {
                        comma_flag = true;
                    }
                    printf("%d", n.next[j]);
                }
            }
            printf("]");
            printf("\n");
            i++;
        }
    }
};

int main()
{
    struct SuffixTree st = SuffixTree();
    string input = "xaccxaca$";
    const char *c_string = input.c_str();
    for (int i = 0; i < (int)input.length(); i++)
    {
        st.extend(c_string[i]);
        st.print();
    }
    return 0;
}
// The end value of nodes in actually exclusive, so internal nodes don't include
// the last character of their range ([start-end] is actually [start-(end-1)]).
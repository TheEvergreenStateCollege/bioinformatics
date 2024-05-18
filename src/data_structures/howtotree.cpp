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
    int children[ALPHABET_SIZE];

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
    int node_active;
    int edge_active;
    int length_active;

public:
    SuffixTree()
    {
        last_added = 1,
        need_suffix_link = 0,
        tree_remainder = 0,
        edge_active = 0,
        length_active = 0,
        position = -1,
        node_active = 1,
        nodes[0] = Node(0, 0);
        nodes[1] = Node(-1, -1);
    }

    int new_node(int start, int end = INF)
    {
        Node nd = Node(start, end);
        for (int i = 0; i < ALPHABET_SIZE; i++)
            nd.children[i] = 0;
        nodes[++last_added] = nd;
        return last_added;
    }

    void add_suffix_link(int node)
    {
        if (need_suffix_link > 0)
            nodes[need_suffix_link].suffix_link = node;
        need_suffix_link = node;
    }

    bool walk_down(int node)
    {
        if (length_active >= nodes[node].edge_length(position))
        {
            edge_active += nodes[node].edge_length(position);
            length_active -= nodes[node].edge_length(position);
            node_active = node;
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
            if (length_active == 0)
                edge_active = position;
            if (nodes[node_active].children[(int)text[edge_active]] == 0)
            {
                int leaf = new_node(position);
                nodes[node_active].children[(int)text[edge_active]] = leaf;
                add_suffix_link(node_active); // rule 2
            }
            else
            {
                int next = nodes[node_active].children[(int)text[edge_active]];
                if (walk_down(next))
                    continue; // observation 2
                if (text[nodes[next].start + length_active] == c)
                { // observation 1
                    length_active++;
                    add_suffix_link(node_active); // observation 3
                    break;
                }
                int split = new_node(nodes[next].start, nodes[next].start + length_active);
                nodes[node_active].children[(int)text[edge_active]] = split;

                int leaf = new_node(position);
                nodes[split].children[(int)c] = leaf;
                nodes[next].start += length_active;
                nodes[split].children[(int)text[nodes[next].start]] = next;
                add_suffix_link(split); // rule 2
            }
            tree_remainder--;
            if (node_active == ROOT && length_active > 0)
            { // rule 1
                length_active--;
                edge_active = position - tree_remainder + 1;
            }
            else
                node_active = nodes[node_active].suffix_link > 0 ? nodes[node_active].suffix_link : ROOT; // rule 3
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
                if (n.children[j] != 0)
                {
                    if (comma_flag)
                    {
                        printf(", ");
                    }
                    else
                    {
                        comma_flag = true;
                    }
                    printf("%d", n.children[j]);
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
    int i = 0;
    while (c_string[i] != NULL)
    {
        st.extend(c_string[i]);
        i++;
    }
    st.print();
    return 0;
}
// The end value of nodes in actually exclusive, so internal nodes don't include
// the last character of their range ([start-end] is actually [start-(end-1)]).
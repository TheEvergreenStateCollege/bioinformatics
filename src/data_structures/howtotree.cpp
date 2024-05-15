#include <iostream>
#include <stdio.h>
#include <string>

const int oo = 1 << 25;        // hard coded stupid large numbers
const int ALPHABET_SIZE = 256; // hard coded max alphabet size.
const int MAXN = 5000;

using namespace std;

int root;
int last_added;
int pos;
int needSL;
int tree_remainder;
int active_node;
int active_e;
int active_len;

struct node
{
    int start, end, slink;
    int next[ALPHABET_SIZE];

    int edge_length()
    {
        return min(end, pos + 1) - start;
    }
};

node tree[2 * MAXN];
char text[MAXN];

int new_node(int start, int end = oo)
{
    node nd;
    nd.start = start;
    nd.end = end;
    nd.slink = 0;
    for (int i = 0; i < ALPHABET_SIZE; i++)
        nd.next[i] = 0;
    tree[++last_added] = nd;
    return last_added;
}

void add_SL(int node)
{
    if (needSL > 0)
        tree[needSL].slink = node;
    needSL = node;
}

bool walk_down(int node)
{
    if (active_len >= tree[node].edge_length())
    {
        active_e += tree[node].edge_length();
        active_len -= tree[node].edge_length();
        active_node = node;
        return true;
    }
    return false;
}

void st_init()
{
    needSL = 0, last_added = 0, pos = -1,
    tree_remainder = 0, active_node = 0, active_e = 0, active_len = 0;
    root = active_node = new_node(-1, -1);
}

void st_extend(char c)
{
    text[++pos] = c;
    needSL = 0;
    tree_remainder++;
    while (tree_remainder > 0)
    {
        if (active_len == 0)
            active_e = pos;
        if (tree[active_node].next[text[active_e]] == 0)
        {
            int leaf = new_node(pos);
            tree[active_node].next[text[active_e]] = leaf;
            add_SL(active_node); // rule 2
        }
        else
        {
            int nxt = tree[active_node].next[text[active_e]];
            if (walk_down(nxt))
                continue; // observation 2
            if (text[tree[nxt].start + active_len] == c)
            { // observation 1
                active_len++;
                add_SL(active_node); // observation 3
                break;
            }
            int split = new_node(tree[nxt].start, tree[nxt].start + active_len);
            tree[active_node].next[text[active_e]] = split;
            int leaf = new_node(pos);
            tree[split].next[c] = leaf;
            tree[nxt].start += active_len;
            tree[split].next[text[tree[nxt].start]] = nxt;
            add_SL(split); // rule 2
        }
        tree_remainder--;
        if (active_node == root && active_len > 0)
        { // rule 1
            active_len--;
            active_e = pos - tree_remainder + 1;
        }
        else
            active_node = tree[active_node].slink > 0 ? tree[active_node].slink : root; // rule 3
    }
}

int main()
{
    st_init();
    string input = "xabxac";
    for (int i = 0; i < input.length(); i++) {
        st_extend(input[i]);
    }
    
    int i = 0;
    while (i < last_added + 1) {
        node n = tree[i];
        printf("%-3d |", i);
        printf(" %-6d |", n.start);
        if (n.end == oo) 
        {
            printf(" End    |");
        } else 
        {
            printf(" %-6d |", n.end);
        }
        if (n.slink == 0) 
        {
            printf(" No SL  |");
        } else 
        {
            printf(" %-6d |", n.slink);
        }
        printf(" [");
        bool comma_flag = false;
        for (int j = 0; j < ALPHABET_SIZE; j ++ ) {
            if (n.next[j] != 0) {
                if (comma_flag) {
                    printf(", ");
                } else {
                    comma_flag = true;
                }
                printf("%d", n.next[j]);
            }
        }
        printf("]");
        printf("\n");
        i++;
    }
    return 0;
}
// The end value of nodes in actually exclusive, so internal nodes don't include
// the last character of their range ([start-end] is actually [start-(end-1)]).
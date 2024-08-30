export class SuffixTree {
    root: Record<string, any>;
    steps: Array<{ char: string; path: string[] }>;
    nodeToParent: WeakMap<Record<string, any>, Record<string, any>>;

    constructor() {
        this.root = {};
        this.steps = [];
        this.nodeToParent = new WeakMap();
        this.nodeToParent.set(this.root, null);
    }

    addSuffix(suffix: string) {
        let node = this.root;
        for (const char of suffix) {
            if (!node[char]) {
                node[char] = {};
                this.steps.push({ char, path: this.getPath(node) });
                this.nodeToParent.set(node[char], node);
            }
            node = node[char];
        }
        node.isEnd = true;
    }

    addString(str: string) {
        console.log(`Adding string: ${str}`);
        for (let i = 0; i < str.length; i++) {
            this.addSuffix(str.slice(i));
            console.log(`Added suffix: ${str.slice(i)}`);
        }
    }

    getPath(node: Record<string, any>): string[] {
        const path: string[] = [];
        while (node && node !== this.root) {
            const parent = this.nodeToParent.get(node);
            if (parent) {
                const entry = Object.entries(parent).find(([key, value]) => value === node);
                if (entry) {
                    path.unshift(entry[0]);
                }
                node = parent;
            } else {
                break;
            }
        }
        return path;
    }

    visualize(): Array<{ char: string; path: string[] }> {
        console.log(`Visualization steps: ${JSON.stringify(this.steps, null, 2)}`);
        return this.steps;
    }
}

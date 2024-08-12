import React, { useState, useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { SuffixTree } from '../lib/SuffixTree';

interface TreeNode {
    name?: string;
    children?: TreeNode[];
}

const SuffixTreeVisualizer: React.FC = () => {
    const [input, setInput] = useState<string>('');
    const [visualizationData, setVisualizationData] = useState<Array<{ char: string; path: string[] }>>([]);
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const svgRef = useRef<SVGSVGElement | null>(null);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setInput(e.target.value);
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const suffixTree = new SuffixTree();
        suffixTree.addString(input);
        const visualData = suffixTree.visualize();
        console.log('Visualization Data:', visualData);
        setVisualizationData(visualData);
        setCurrentIndex(0);
    };

    const handleNextStep = () => {
        if (currentIndex < visualizationData.length) {
            setCurrentIndex(currentIndex + 1);
        }
    };
    const handleNext10Steps = () => {
        if (currentIndex < visualizationData.length) {
            setCurrentIndex(Math.min(currentIndex + 10, visualizationData.length));
        }
    };
    const handleNext100Steps = () => {
        if (currentIndex < visualizationData.length) {
            setCurrentIndex(Math.min(currentIndex + 100, visualizationData.length));
        }
    };

    useEffect(() => {
        if (!svgRef.current) return;

        const svg = d3.select(svgRef.current);
        svg.selectAll('*').remove();

        const width = 500;
        const height = 500;
        const margin = { top: 20, right: 20, bottom: 30, left: 40 };

        svg.attr('width', width)
            .attr('height', height)
            .attr('viewBox', `0 0 ${width} ${height}`)
            .attr('preserveAspectRatio', 'xMidYMid meet');

        const root = d3.hierarchy<TreeNode>(buildTree(visualizationData.slice(0, currentIndex)));
        const treeLayout = d3.tree<TreeNode>()
            .size([height - margin.top - margin.bottom, width - margin.left - margin.right])
            // .nodeSize([12,12])
            // .separation((a, b) => {
            //     return a.parent == b.parent ? 1 : 2;
            // });

        treeLayout(root);

        const linkPathGenerator = d3.linkHorizontal<{ source: { x: number; y: number }; target: { x: number; y: number } }>()
            .x(d => d.y)
            .y(d => d.x);

        //branches
        svg.append('g')
            .attr('transform', `translate(${margin.left},${margin.top})`)
            .selectAll('path')
            .data(root.links())
            .enter()
            .append('path')
            .attr('d', d => linkPathGenerator(d)!)
            .attr('fill', 'none')
            .attr('class', 'stroke-current text-green-800');

        //dots to represent nodes with more than 1 child (otherwise they take up too much space)
        svg.append('g')
            .attr('transform', `translate(${margin.left},${margin.top})`)
            .selectAll('circle')
            .data(root.descendants().filter(d => d.children && d.children.length > 1))
            .enter()
            .append('circle')
            .attr('cx', d => d.y)
            .attr('cy', d => d.x)
            .attr('r', d => 4)
            .attr('class', 'fill-current text-green-800');

        //characters
        svg.append('g')
            .attr('transform', `translate(${margin.left},${margin.top})`)
            .selectAll('text')
            .data(root.descendants())
            .enter()
            .append('text')
            .attr('x', d => d.y + 10)
            .attr('y', d => d.x)
            .attr('class', 'fill-current text-green-900')
            .text(d => d.data.name || '');
    }, [visualizationData, currentIndex]);

    const buildTree = (steps: Array<{ char: string; path: string[] }>): TreeNode => {
        const root: TreeNode = { children: [] };

        steps.forEach(({ char, path }) => {
            let node = root;
            path.forEach(p => {
                let child = node.children?.find(child => child.name === p);
                if (!child) {
                    child = { name: p, children: [] };
                    if (!node.children) {
                        node.children = [];
                    }
                    node.children.push(child);
                }
                node = child;
            });
            if (!node.children) {
                node.children = [];
            }
            node.children.push({ name: char, children: [] });
        });

        return root;
    };

    return (
        <div>
            <form className={"text-center"} onSubmit={handleSubmit}>
                <label>
                    Enter text to create a suffix tree:
                    <input className={"border m-2"} type="text" value={input} onChange={handleChange} />
                </label>
                <button className={"bg-green-900 text-green-100 p-2"} type="submit">Generate Suffix Tree</button>
            </form>
            <div className={"text-center"}>
                <button className={"m-2 bg-green-900 text-green-100 p-2"} onClick={handleNextStep}
                        disabled={currentIndex >= visualizationData.length}>Next Step
                </button>
                <button className={"m-2 bg-green-900 text-green-100 p-2"} onClick={handleNext10Steps}
                        disabled={currentIndex >= visualizationData.length}>Next 10 Steps
                </button>
                <button className={"m-2 bg-green-900 text-green-100 p-2"} onClick={handleNext100Steps}
                        disabled={currentIndex >= visualizationData.length}>Next 100 Steps
                </button>
            </div>
            <svg ref={svgRef}></svg>
        </div>
    );
};

export default SuffixTreeVisualizer;

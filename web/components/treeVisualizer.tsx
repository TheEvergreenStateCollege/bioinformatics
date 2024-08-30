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
        setCurrentIndex(currentIndex + 10 < visualizationData.length ? currentIndex + 10 : visualizationData.length);
    };
    const handleNext100Steps = () => {
        setCurrentIndex(currentIndex + 10 < visualizationData.length ? currentIndex + 100 : visualizationData.length);
    };
     const handleNext1000Steps = () => {
        setCurrentIndex(currentIndex + 10 < visualizationData.length ? currentIndex + 1000 : visualizationData.length);
    };
    const [zoomTransform, setZoomTransform] = useState<d3.ZoomTransform | null>(null);

    useEffect(() => {
        if (!svgRef.current) return;

        const svg = d3.select(svgRef.current);
        svg.selectAll('*').remove();

        const g = svg.append('g');
        const zoom = d3.zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.5, 2])
            .on('zoom', (event) => {
                setZoomTransform(event.transform);
            });

        svg.call(zoom);
        const width = svgRef.current.clientWidth;
        const height = svgRef.current.clientHeight;

        svg
            .attr('width', width)
            .attr('height', height)
            .attr('viewBox', `0 0 ${width} ${height}`)
            .attr('preserveAspectRatio', 'xMidYMid meet');

        const root = d3.hierarchy<TreeNode>(buildTree(visualizationData.slice(0, currentIndex)));
        const treeLayout = d3.tree<TreeNode>()
            .size([height, width])
            .nodeSize([25, 20]);

        treeLayout(root);

        const linkGenerator = d3.linkHorizontal()
            .x(d => d.y)
            .y(d => d.x);

        const [xMin, yMin, viewBoxWidth, viewBoxHeight] = svg.attr('viewBox').split(' ').map(Number);
        const xMax = xMin + viewBoxWidth;
        const yMax = yMin + viewBoxHeight;

        const visibleLinks = root.links().filter(link => {
            const transform = zoomTransform || d3.zoomIdentity;
            const sourceX = transform.applyX(link.source.y);
            const sourceY = transform.applyY(link.source.x);
            const targetX = transform.applyX(link.target.y);
            const targetY = transform.applyY(link.target.x);

            const sourceVisible = (
                sourceY >= yMin && sourceY <= yMax &&
                sourceX >= xMin && sourceX <= xMax
            );
            const targetVisible = (
                targetY >= yMin && targetY <= yMax &&
                targetX >= xMin && targetX <= xMax
            );

            const crossesX = (sourceX < xMin && targetX > xMax) || (sourceX > xMax && targetX < xMin);
            const crossesY = (sourceY < yMin && targetY > yMax) || (sourceY > yMax && targetY < yMin);

            return sourceVisible || targetVisible || crossesX || crossesY;
        });

        const visibleNodes = root.descendants().filter(d => {
            const transform = zoomTransform || d3.zoomIdentity;
            const x = transform.applyX(d.y);
            const y = transform.applyY(d.x);
            return x >= xMin && x <= xMax && y >= yMin && y <= yMax;
        });


        console.log('Total Links:', root.links().length);
        console.log('Visible Links:', visibleLinks.length);

        g.selectAll('.link')
            .data(visibleLinks)
            .enter()
            .append('path')
            .attr('class', 'link')
            .attr('d', linkGenerator)
            .attr('fill', 'none')
            .attr('stroke', '#22543d');

        g.selectAll('.node')
            .data(visibleNodes.filter(d => d.children && d.children.length > 1))
            .enter()
            .append('circle')
            .classed('node', true)
            .attr('cx', d => d.y)
            .attr('cy', d => d.x)
            .attr('r', 5)
            .attr('fill', '#22543d');

        g.selectAll('.label')
            .data(visibleNodes)
            .enter()
            .append('text')
            .classed('label', true)
            .attr('x', d => d.y)
            .attr('y', d => d.x - 10)
            .attr('fill', '#1c4532')
            .text(d => d.data.name);


        if (zoomTransform) {
            g.attr('transform', zoomTransform.toString());
        }
    }, [visualizationData, currentIndex, zoomTransform]);


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
        <div className="flex flex-col items-center justify-center w-full min-h-screen">
            <form onSubmit={handleSubmit} className="mb-4">
                <label>
                    Enter text to create a suffix tree:
                    <input type="text" value={input} onChange={handleChange} className="ml-2 border-2 p-1"/>
                </label>
                <button type="submit" className="ml-2 bg-green-500 text-white p-1 rounded">Generate</button>
            </form>
            <div className="text-center mb-2">
                <button onClick={handleNextStep} className="bg-green-500 text-white p-1 rounded">Next Step</button>
                <button onClick={handleNext10Steps} className="bg-green-500 text-white p-1 rounded mx-2">Next 10 Steps
                </button>
                <button onClick={handleNext100Steps} className="bg-green-500 text-white p-1 rounded mx-2">Next 100 Steps
                </button>
                <button onClick={handleNext1000Steps} className="bg-green-500 text-white p-1 rounded mx-2">Next 1000 Steps
                </button>
            </div>
            <div className="text-lg mb-4">
                <p>Current Step: {currentIndex} / {visualizationData.length}</p>
            </div>
            <div className="flex flex-col items-center  w-full h-screen">
                <div className="w-full lg:w-3/4 xl:w-1/2 h-3/4">
                    <svg ref={svgRef} className="w-full h-full border-4 border-green-800 bg-lime-50"></svg>
                </div>
            </div>
        </div>
    );
};

export default SuffixTreeVisualizer;

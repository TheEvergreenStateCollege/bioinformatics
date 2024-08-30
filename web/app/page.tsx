'use client'
import SuffixTreeVisualizer from '@/components/treeVisualizer';

const HomePage: React.FC = () => {
    return (
        <div>
            <h1 className={"text-2xl text-center pt-4"}>Suffix Tree Visualizer</h1>
            <SuffixTreeVisualizer />
        </div>
    );
};

export default HomePage;

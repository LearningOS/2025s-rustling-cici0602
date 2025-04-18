/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    // 添加节点：若节点不存在则添加并返回true，否则返回false
    fn add_node(&mut self, node: &str) -> bool {
        let node_str = node.to_string();
        if self.adjacency_table().contains_key(&node_str) {
            return false;
        }
        self.adjacency_table_mutable().insert(node_str, Vec::new());
        true
    }

    // 添加边：自动处理节点存在性，并添加双向边
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        let from_str = from.to_string();
        let to_str = to.to_string();

        // 确保节点存在
        self.add_node(from);
        self.add_node(to);

        // 添加双向边
        self.adjacency_table_mutable()
            .entry(from_str.clone())
            .or_default()
            .push((to_str.clone(), weight));
        self.adjacency_table_mutable()
            .entry(to_str.clone())
            .or_default()
            .push((from_str, weight));
    }

    // 检查节点是否存在
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }

    // 获取所有节点
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    // 获取所有边（返回引用类型以匹配测试用例）
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

impl Graph for UndirectedGraph {
    fn new() -> Self {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Neighbors {
    parents: HashSet<char>,
    children: HashSet<char>,
}

impl Neighbors {
    pub fn new() -> Neighbors {
        Neighbors {
            parents: HashSet::new(),
            children: HashSet::new(),
        }
    }

    pub fn add_parent(&mut self, parent: char) {
        self.parents.insert(parent);
    }

    pub fn add_child(&mut self, child: char) {
        self.children.insert(child);
    }

    pub fn remove_parent(&mut self, parent: char) {
        self.parents.remove(&parent);
    }

    pub fn is_root(&self) -> bool {
        self.parents.is_empty()
    }

    pub fn get_children(&self) -> &HashSet<char> {
        &self.children
    }
}

impl Default for Neighbors {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct DAG {
    nodes: HashMap<char, Neighbors>,
}

impl DAG {
    pub fn get_children(&self, node: char) -> Option<&HashSet<char>> {
        self.nodes.get(&node).map(Neighbors::get_children)
    }

    pub fn delete_root_node(&mut self, node: char) -> Result<(), &str> {
        if !self.nodes[&node].is_root() {
            return Err("Node is not a root node");
        }
        for child in self.nodes[&node].get_children().clone() {
            self.nodes.get_mut(&child).unwrap().remove_parent(node);
        }
        self.nodes.remove(&node);
        Ok(())
    }

    fn get_root_nodes(&self) -> HashSet<char> {
        self.nodes
            .iter()
            .filter(|(_, neigh)| neigh.is_root())
            .map(|(node, _)| *node)
            .collect()
    }

    pub fn root_nodes_exist(&self) -> bool {
        !self.get_root_nodes().is_empty()
    }

    pub fn get_min_root_node(&mut self, except: &HashSet<char>) -> Option<char> {
        self.get_root_nodes()
            .iter()
            .filter(|node| !except.contains(node))
            .min()
            .copied()
    }

    pub fn new() -> DAG {
        DAG {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: char, to: char) {
        self.nodes.entry(from).or_default().add_child(to);
        self.nodes.entry(to).or_default().add_parent(from);
    }
}

impl Default for DAG {
    fn default() -> Self {
        Self::new()
    }
}

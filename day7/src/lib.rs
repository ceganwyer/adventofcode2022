use std::collections::HashMap;
use eyre::{WrapErr, Result, eyre};

pub type TreeIndex = usize;

#[derive(Debug)]
pub struct Tree {
    arena: Vec<Option<Node>>,
    root: Option<TreeIndex>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: None
        }
    }

    pub fn iter(&mut self) -> PreorderIter {
        PreorderIter::new(self.root)
    }

    pub fn set_root(&mut self, root: Option<TreeIndex>) {
        self.root = root;
    }

    pub fn add_node(&mut self, node: Node) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        return index;
    }

    pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<Node> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&Node> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        }
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut Node> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        }
    }

    pub fn find_node(&self, name: &str) -> Option<TreeIndex> {
        let mut index: TreeIndex = 0;
        for node in self.arena.iter() {
            if let Some(node) = node {
                if node.name == name {
                    return Some(index)
                }
                index += 1;
            }
        }
        None
    }

    pub fn create_new_child(&mut self, parent_index: TreeIndex, name: &str, size: Option<i32>)
    -> Result<TreeIndex> {
        let new_child = Node::new(name.to_string(), size, Vec::new(), Some(parent_index));
        let index = self.add_node(new_child);
        let parent = self.node_at_mut(parent_index)
            .ok_or_else(|| eyre!("Couldn't find parent node at {}", parent_index))?;
        parent.children.push(index);

        Ok(index)
    }
}

pub struct PreorderIter {
    stack: Vec<TreeIndex>
}

impl PreorderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreorderIter {
                stack: vec![index]
            }
        } else {
            PreorderIter {
                stack: vec![]
            }
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.stack.pop() {
            if let Some(node) = tree.node_at(node_index) {
                self.stack.append(node.children.clone().as_mut())
            }

            return Some(node_index)
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct Node {
    pub name: String,
    pub children: Vec<TreeIndex>,
    pub size: Option<i32>,
    pub parent: Option<TreeIndex>,
}

impl Node {
    pub fn new(name: String, size: Option<i32>, children: Vec<TreeIndex>, parent: Option<TreeIndex>) -> Self {
        Node {
            name,
            size,
            children,
            parent
        }
    }
}

pub fn parse_tree(input: String) -> Result<Tree> {
    let mut tree = Tree::new();
    let root = tree.add_node(Node {
        name: "/".to_string(),
        children: vec![],
        size: None,
        parent: None,
    });
    tree.set_root(Some(root));
    Ok(parse_commands(input, tree)?)
}

fn parse_commands(input: String, mut tree: Tree) -> Result<Tree> {
    let mut current_node_index: TreeIndex = 0;
    for line in input.lines() {
        let mut parts = line.trim_start_matches("$").split_whitespace();
        let prefix = parts.next().ok_or_else(|| eyre!("No prefix found!"))?;
        let suffix = parts.next();
        match prefix {
            "cd" => {
                let name = suffix.ok_or_else(|| eyre!("No suffix found!"))?;
                current_node_index = process_cd(name, current_node_index, &tree)?;
            },
            "ls" => continue,
            "dir" => {
                let name = suffix.ok_or_else(|| eyre!("No suffix found!"))?;
                tree.create_new_child(current_node_index, name, None)?;
            },
            _ => {
                let size: i32 = prefix.parse().wrap_err("Unable to parse file size!")?;
                let name = suffix.ok_or_else(|| eyre!("No suffix found!"))?;
                tree.create_new_child(current_node_index, name, Some(size))?;
            },
        }
    }
    Ok(tree)
}

fn process_cd(name: &str, mut current_node_index: TreeIndex, tree: &Tree) -> Result<TreeIndex> {
    if name == ".." {
        current_node_index = tree.node_at(current_node_index)
            .ok_or_else(|| eyre!("No node found at {}!", current_node_index))?
            .parent
            .ok_or_else(|| eyre!("No parent found!"))?;
    } else {
        current_node_index = tree.find_node(name)
            .ok_or_else(|| eyre!("Node {} not found!", name))?
    }
    Ok(current_node_index)
}

pub fn sum_directories(mut tree: Tree) -> HashMap<String, i32> {
    let mut results = HashMap::new();
    let mut iter = tree.iter();
    while let Some(node_index) = iter.next(&tree) {
        if let Some(node) = tree.node_at(node_index) {
            if node.size == None {
                println!("Calculating: {}", node.name);
                let entry = results.entry(node.name.clone()).or_insert(0);
                let size = calc_dir_size(node_index, 0, &tree);
                *entry = size;
            }
        }
    }
    results
}

fn calc_dir_size(node_index: TreeIndex, mut accum: i32, tree: &Tree) -> i32 {
    if let Some(node) = tree.node_at(node_index) {
        println!("{} - {:?}: {}", node.name, node.size, accum);
        if node.children.is_empty() {
            accum += node.size.unwrap_or(0);
        } else {
            for child in &node.children {
                accum += calc_dir_size(child.clone(), accum, &tree);
            }
        }
    }

    accum
}
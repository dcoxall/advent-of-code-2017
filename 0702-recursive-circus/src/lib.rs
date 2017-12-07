#[cfg(test)]
mod tests {
    use super::balance;

    #[test]
    fn it_works() {
        let progs = vec![
            (String::from("pbga"), 66, vec![]),
            (String::from("xhth"), 57, vec![]),
            (String::from("ebii"), 61, vec![]),
            (String::from("havc"), 66, vec![]),
            (String::from("ktlj"), 57, vec![]),
            (String::from("fwft"), 72, vec![String::from("ktlj"), String::from("cntj"), String::from("xhth")]),
            (String::from("qoyq"), 66, vec![]),
            (String::from("padx"), 45, vec![String::from("pbga"), String::from("havc"), String::from("qoyq")]),
            (String::from("tknk"), 41, vec![String::from("ugml"), String::from("padx"), String::from("fwft")]),
            (String::from("jptl"), 61, vec![]),
            (String::from("ugml"), 68, vec![String::from("gyxo"), String::from("ebii"), String::from("jptl")]),
            (String::from("gyxo"), 61, vec![]),
            (String::from("cntj"), 57, vec![]),
        ];
        assert_eq!(60, balance(&progs));
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Rc<RefCell<Node>> {
        Rc::new(
            RefCell::new(
                Node {
                    name: String::new(),
                    weight: 0,
                    children: Vec::new(),
                }
            )
        )
    }

    fn total_weight(&self) -> u32 {
        self.children.iter()
            .map(|n| n.borrow().total_weight())
            .sum::<u32>() + self.weight
    }

    fn imbalanced(&self) -> Option<(Rc<RefCell<Node>>, i32)> {
        let mut count: HashMap<u32, usize> = HashMap::new();
        for child in self.children.iter() {
            let child = child.borrow();
            let entry = count.entry(child.total_weight()).or_insert(0);
            *entry += 1;
        }
        if let Some((expected, _)) = count.iter().max_by_key(|&(_, n)| n) {
            for child in self.children.iter() {
                let child_b = child.borrow();
                let total = child_b.total_weight();
                if total != *expected {
                    let diff = (*expected as i32) - (total as i32);
                    return Some((child.clone(), diff));
                }
            }
        }
        None
    }
}

pub fn balance(progs: &Vec<(String, u32, Vec<String>)>) -> u32 {
    let mut node_map: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    let mut remove_list: Vec<&str> = Vec::new();
    for &(ref name, ref weight, ref children) in progs {
        {
            let node = node_map.entry(name).or_insert_with(|| Node::new());
            let mut node = node.borrow_mut();
            node.weight = *weight;
            node.name = name.to_string();
        }

        {
            let children: Vec<Rc<RefCell<Node>>> = children.into_iter()
                .map(|n| node_map.entry(n).or_insert_with(|| Node::new()).clone())
                .collect();

            let node = node_map.entry(name).or_insert_with(|| Node::new());
            let mut node = node.borrow_mut();
            node.children = children;
        }

        for child in children {
            remove_list.push(child);
        }
    }

    for name in &remove_list {
        node_map.remove(name);
    }

    // iterator to go over all nodes
    let mut nodes: Vec<Rc<RefCell<Node>>> = node_map.values().map(|n| n.clone()).collect();
    let mut imbalanced: Vec<(Rc<RefCell<Node>>, i32)> = Vec::new();
    while let Some(node) = nodes.pop() {
        let node_borrow = node.borrow();
        if let Some(result) = node_borrow.imbalanced() {
            imbalanced.push(result);
        }
        for child in &node_borrow.children {
            nodes.push(child.clone());
        }
    }

    // So we have some imbalanced nodes. The last is the one we want
    if let Some(&(ref node, ref diff)) = imbalanced.iter().last() {
        let node = node.borrow();
        let corrected_weight = (node.weight as i32) + *diff;
        corrected_weight as u32
    } else {
        0
    }
}

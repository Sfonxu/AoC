use std::collections::HashMap;
use std::path::Path;
use std::fs;

enum OperationKind {
    And = 0,
    Or,
    Not,
    LShift,
    RShift,
    Store,
}

struct Node<'a> {
    lhs: &'a str,
    rhs: &'a str,
    operation: OperationKind,
    has_value: bool,
    value: u16,
}

trait FindValue {
    fn find_value(&mut self, node_name: &str) -> u16;
    fn value_or_find(&mut self, node_name: &str) -> u16;
}

impl FindValue for HashMap::<&str, Node<'_>> {
    fn find_value(&mut self, node_name: &str) -> u16 {
	if !self[node_name].has_value {
	match self[node_name].operation {
	    OperationKind::And => {
		self.get_mut(node_name).unwrap().value = self.value_or_find(self[node_name].lhs) & self.value_or_find(self[node_name].rhs);
	    }
	    OperationKind::Or => {
		self.get_mut(node_name).unwrap().value = self.value_or_find(self[node_name].lhs) | self.value_or_find(self[node_name].rhs);
	    }
	    OperationKind::Not => {
		self.get_mut(node_name).unwrap().value = !self.value_or_find(self[node_name].lhs);
	    }
	    OperationKind::LShift => {
		self.get_mut(node_name).unwrap().value = self.value_or_find(self[node_name].lhs) << self.value_or_find(self[node_name].rhs);
	    }
	    OperationKind::RShift => {
		self.get_mut(node_name).unwrap().value = self.value_or_find(self[node_name].lhs) >> self.value_or_find(self[node_name].rhs);
	    }
	    OperationKind::Store => {
		self.get_mut(node_name).unwrap().value = self.value_or_find(self[node_name].lhs);
	    }
	}
	    self.get_mut(node_name).unwrap().has_value = true;
	}
	return self[node_name].value;
    }

    fn value_or_find(&mut self, node_name: &str) -> u16 {
	if let Ok(value) = node_name.parse::<u16>() {
	    return value;
	}
	return self.find_value(node_name);
    } 
}

fn get_op_kind(input: &str) -> OperationKind {
    for word in input.split(' ') {
	match word {
	    "AND" => return OperationKind::And,
	    "OR" => return OperationKind::Or,
	    "NOT" => return OperationKind::Not,
	    "LSHIFT" => return OperationKind::LShift,
	    "RSHIFT" => return OperationKind::RShift,
	    _ => {}
	}
    }
    return OperationKind::Store
}

fn get_node(input: &str) -> (Node<'_>, &str) {
    let op_kind = get_op_kind(input);
    let mut has_value = false;
    let split: Vec::<&str> = input.split(' ').collect();
    match op_kind {
	OperationKind::And | OperationKind::Or | OperationKind::LShift | OperationKind::RShift => {
	    return (Node{
		lhs: split[0],
		rhs: split[2],
		operation: op_kind,
		has_value: has_value,
		value: 0
	    }, split[split.len()-1]);
	}
	OperationKind::Store => {
	    let mut value = 0;
	    if let Ok(lhs) = split[0].parse::<u16>() {
		has_value = true;
		value = lhs;
	    }
	    return (Node{
		lhs: split[0],
		rhs: "",
		operation: op_kind,
		has_value: has_value,
		value: value
	    }, split[split.len()-1])
	}
	OperationKind::Not => {
	    let mut value = 0;
	    if let Ok(lhs) = split[1].parse::<u16>() {
		has_value = true;
		value = lhs;
	    }
	    return (Node{
		lhs: split[1],
		rhs: "",
		operation: op_kind,
		has_value: has_value,
		value: value
	    }, split[split.len()-1])
	}
    }
}

fn main() {
    // Part One
    let path = Path::new("../input");
    let input = fs::read_to_string(path).expect("Failed to read input!");
    let mut nodes_part_one: HashMap::<&str, Node> = HashMap::new();
    for line in input.lines() {
	let node = get_node(line);
	nodes_part_one.insert(node.1, node.0);
    }
    let result_part_one = nodes_part_one.find_value("a");
    println!("The value of a at the end of part one is {}", result_part_one);

    // Part Two
    let mut nodes_part_two: HashMap::<&str, Node> = HashMap::new();
    for line in input.lines() {
	let node = get_node(line);
	nodes_part_two.insert(node.1, node.0);
    }
    nodes_part_two.get_mut("b").unwrap().value = result_part_one;
    let result_part_two = nodes_part_two.find_value("a");
    println!("The value of a at the end of part two is {}", result_part_two);
}

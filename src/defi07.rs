#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Directory,
    File,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    node_type: NodeType,
    size: u32
}

impl Node {
    fn new(name: String, node_type: NodeType, size: u32, parent : Option<usize>) -> Node { 
        Node {
            name: name,
            parent: parent,
            children: Vec::new(),
            node_type: node_type,
            size: size
        }
    }

    fn get_children_by_name(&self, nodes : Vec<Node>, search_item : String) -> usize {
        for node_id in self.children.clone()  {
            if nodes[node_id].name == search_item {
                return node_id;
            }
        }

        todo!()
    }
    fn get_size(&self, nodes : Vec<Node>) -> u32 {
        if self.size > 0 {
            return self.size;
        }

        let mut total = 0;
        for node_id in self.children.clone() {
            total += nodes[node_id].get_size(nodes.clone());
        }

        total
    }
}

fn parse_tree(input : String) -> Vec<Node> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut nodes : Vec<Node> = Vec::new();
    nodes.push(Node::new("/".to_string(), NodeType::Directory, 0, None));

    let mut current_node : usize = 0;
    for line in lines {
        let line_elements = line.split(' ').collect::<Vec<&str>>();
        match line_elements[0] {
             "$" => {
                match line_elements[1] {
                     "cd" => {
                        match line_elements[2] {
                            ".." => {
                                let val = nodes[current_node].parent.unwrap();
                                current_node = val;
                            },
                            "/" => {},
                            _ => {
                                current_node = nodes[current_node].get_children_by_name(nodes.clone(), line_elements[2].to_string());
                            }
                        }
                     },
                     "ls" => {

                     }
                     &_ => todo!(),
                }
             },
             "dir" => {
                let len = nodes.len();
                nodes[current_node].children.push(len);
                nodes.push(
                    Node::new(line_elements[1].to_string(), NodeType::Directory, 0, Some(current_node))
                );
            },
             _ => {
                let len = nodes.len();
                nodes[current_node].children.push(len);
                nodes.push(
                    Node::new(line_elements[1].to_string(), NodeType::File, line_elements[0].parse::<u32>().unwrap(), Some(current_node))
                );
            },
        }
    }

    nodes
}

pub fn do_work(input : String) -> u32 {
    let tree = parse_tree(input);

    let mut total = 0;
    for node in tree.clone() {
        let size = node.get_size(tree.clone());

        if node.node_type == NodeType::Directory && size <= 100000 {
            total += size;
        }
    }

    total
}

    tree.get_size()
}

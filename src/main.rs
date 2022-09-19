mod boolean_tree;

pub struct Node{pub tree: boolean_tree::BooleanTree<bool>}

impl Node{
    pub fn new(in_tree: boolean_tree::BooleanTree<bool>) -> Node {Node{tree: in_tree}}
}

fn main() {
    // F(A) = !B & C | D
    let node_a = boolean_tree::BooleanTree::new(
        boolean_tree::and_node(
            boolean_tree::not_val_node(false, 1),
            boolean_tree::or_node(
                boolean_tree::val_node(false, 2),
                boolean_tree::val_node(false, 3)
            )
        )
    );

    // F(B) = A & D
    let node_b = boolean_tree::BooleanTree::new(
        boolean_tree::and_node(
            boolean_tree::val_node(false, 0),
            boolean_tree::val_node(false, 3)
        )
    );

    // F(C) = B
    let node_c = boolean_tree::BooleanTree::new(
        boolean_tree::val_node(false, 1)
    );

    // F(D) = !A | D
    let node_d = boolean_tree::BooleanTree::new(
        boolean_tree::or_node(
            boolean_tree::not_val_node(false, 0),
            boolean_tree::val_node(false, 3)
        )
    );

    let temp_1 = Node::new(node_a);

    let mut temp_2 : Vec<Node> = Vec::new();

    temp_2.push(temp_1);
    
    let temp_3 = &temp_2[0];

    println!("{}", temp_3.tree.resolve());
}
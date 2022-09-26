mod tree;
mod slow_tree;

fn main() {

    // // F(A) = !B & C | D
    // let node_a_slow = slow_tree::BooleanTree::new(
    //     slow_tree::and_node(
    //         slow_tree::not_val_node(false, 1),
    //         slow_tree::or_node(
    //             slow_tree::val_node(false, 2),
    //             slow_tree::val_node(false, 3)
    //         )
    //     )
    // );
    
    let x = &tree::Node::not_val_node(false, 1);
    let y = &tree::Node::val_node(false, 2);
    let z = &tree::Node::val_node(false, 3);
    let x1 = &tree::Node::or_node(
        y,
        z
    );
    // F(A) = !B & C | D
    let mut node_a = tree::BooleanTree::new(
        tree::Node::and_node(
            x,
            x1
        )
    );

    // // F(B) = A & D
    // let node_b = tree::BooleanTree::new(
    //     tree::and_node(
    //         tree::val_node(false, 0),
    //         tree::val_node(false, 3)
    //     )
    // );
    
    let x = &tree::Node::val_node(false, 0);
    let y = &tree::Node::val_node(false, 3);
    // F(A) = !B & C | D
    let mut node_b = tree::BooleanTree::new(
        tree::Node::and_node(
            x,
            y
        )
    );

    // // F(C) = B
    // let node_c = tree::BooleanTree::new(
    //     tree::val_node(false, 1)
    // );

    // F(A) = !B & C | D
    let mut node_c = tree::BooleanTree::new(
        tree::Node::val_node(false, 1)
    );

    // // F(D) = !A | D
    // let node_d = tree::BooleanTree::new(
    //     tree::or_node(
    //         tree::not_val_node(false, 0),
    //         tree::val_node(false, 3)
    //     )
    // );
    
    let x = &tree::Node::not_val_node(false, 0);
    let y = &tree::Node::val_node(false, 3);
    // F(A) = !B & C | D
    let mut node_d = tree::BooleanTree::new(
        tree::Node::or_node(
            x,
            y
        )
    );

    let mut network = Vec::from([node_a, node_b, node_c, node_d]);

    for node in network.iter(){
        println!("{}", node.to_str());
        println!("{}", node.to_json());
        println!("{}", node.to_exp());
        println!("{}", node.resolve());
    }
}
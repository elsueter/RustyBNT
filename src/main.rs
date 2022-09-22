mod tree;

fn main() {

    // // F(A) = !B & C | D
    // let node_a = tree::BooleanTree::new(
    //     tree::and_node(
    //         tree::not_val_node(false, 1),
    //         tree::or_node(
    //             tree::val_node(false, 2),
    //             tree::val_node(false, 3)
    //         )
    //     )
    // );

    // // F(B) = A & D
    // let node_b = tree::BooleanTree::new(
    //     tree::and_node(
    //         tree::val_node(false, 0),
    //         tree::val_node(false, 3)
    //     )
    // );

    // // F(C) = B
    // let node_c = tree::BooleanTree::new(
    //     tree::val_node(false, 1)
    // );

    // // F(D) = !A | D
    // let node_d = tree::BooleanTree::new(
    //     tree::or_node(
    //         tree::not_val_node(false, 0),
    //         tree::val_node(false, 3)
    //     )
    // );
    
    let x = &tree::not_val_node(false, 1);
    let y = &tree::val_node(false, 2);
    let z = &tree::val_node(false, 3);
    let x1 = &tree::or_node(
        y,
        z
    );
    // F(A) = !B & C | D
    let node_a = tree::BooleanTree::new(
        tree::and_node(
            x,
            x1
        )
    );

    let mut temp: Vec<tree::BooleanTree<bool>> = Vec::from([node_a]);

    for node in temp.iter(){
        println!("{}", node.resolve());
    };

}
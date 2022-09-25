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
    
    let x = &tree::not_val_node(true, 1);
    let y = &tree::val_node(true, 2);
    let z = &tree::val_node(false, 3);
    let x1 = &tree::or_node(
        y,
        z
    );
    let x2 = &tree::and_node(
        x1,
        x1
    );
    // F(A) = !B & C | D
    let mut node_a = Box::new(
        tree::BooleanTree::new(
            tree::and_node(
                x2,
                x1
            )
        )
    );

    node_a.print_tree();

}
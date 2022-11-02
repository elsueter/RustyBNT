mod tree;

pub fn test() -> Vec<tree::Tree> {
    // F(A) = !B & C | D
    let node_a = tree::Tree::new(
        Vec::from([
            tree::and_node(),
            tree::or_node(),
            tree::not_node(1),
            tree::val_node(2),
            tree::val_node(3),
        ])
    );

    // // F(B) = A & D
    let node_b = tree::Tree::new(
        Vec::from([
            tree::and_node(),
            tree::val_node(0),
            tree::val_node(3),
        ])
    );

    // // F(C) = B
    let node_c = tree::Tree::new(
        Vec::from([
            tree::val_node(1),
        ])
    );

    // // F(D) = !A | D
    let node_d = tree::Tree::new(
        Vec::from([
            tree::or_node(),
            tree::not_node(0),
            tree::val_node(3),
        ])
    );

    Vec::from([node_a, node_b, node_c, node_d])
}
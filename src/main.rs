mod tree;

fn main() {
    // F(A) = !B & C | D
    let node_a = tree::BooleanTree::new(
        tree::and_node(
            tree::not_val_node(false, 1),
            tree::or_node(
                tree::val_node(true, 2),
                tree::val_node(false, 3)
            )
        )
    );

    // F(B) = A & D
    let node_b = tree::BooleanTree::new(
        tree::and_node(
            tree::val_node(false, 0),
            tree::val_node(false, 3)
        )
    );

    // F(C) = B
    let node_c = tree::BooleanTree::new(
        tree::val_node(false, 1)
    );

    // F(D) = !A | D
    let node_d = tree::BooleanTree::new(
        tree::or_node(
            tree::not_val_node(false, 0),
            tree::val_node(false, 3)
        )
    );

    let mut temp: Vec<tree::BooleanTree<bool>> = Vec::new();

    temp.push(node_a);

    println!("{}", temp[0].resolve());

}
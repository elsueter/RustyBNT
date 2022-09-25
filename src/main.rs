mod tree;
mod slow_tree;

fn main() {

    // F(A) = !B & C | D
    let node_a_slow = slow_tree::BooleanTree::new(
        slow_tree::and_node(
            slow_tree::not_val_node(false, 1),
            slow_tree::or_node(
                slow_tree::val_node(false, 2),
                slow_tree::val_node(false, 3)
            )
        )
    );

    let temp_1: Vec<slow_tree::BooleanTree<bool>> = Vec::from([node_a_slow]);

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

    let temp_2: Vec<tree::BooleanTree<bool>> = Vec::from([node_a]);

    use std::time::Instant;
    let now = Instant::now();

    for i in 1..1000000{
        for node in temp_1.iter(){
            node.resolve();
        };
    }

    let elapsed = now.elapsed();
    println!("Slow elapsed: {:.2?}", elapsed);
    let now = Instant::now();

    for i in 1..1000000{
        for node in temp_2.iter(){
            node.resolve();
        };
    }

    let elapsed = now.elapsed();
    println!("Normal elapsed: {:.2?}", elapsed);

}
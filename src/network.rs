mod tree;
mod exp_parser;

pub fn create_network(in_string: String) -> Vec<tree::Tree>{
    let shunted_string = exp_parser::shunt_expression(in_string);

    let mut temp: Vec<tree::Node> = vec![];
    let mut flag = false;
    let mut skip = false;
    
    for (i, c) in shunted_string[0].iter().enumerate(){
        match c{
            38 =>  temp.push(tree::and_node()),
            124 =>  temp.push(tree::or_node()),
            _ => (),
        }
    }

    let offset = temp.len();

    for (i, c) in shunted_string[1].iter().enumerate(){

        if skip{
            skip = false;
            continue;
        }
        if c == &61{
            flag = true;
            continue;
        }
        if flag{
            if c == &33{
                temp.push(tree::not_node(temp.len()-offset+1));
                skip = true;
                continue;
            }
            temp.push(tree::val_node(temp.len()-offset+1));
        }
    }

    vec![tree::Tree::new(temp)]
}

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

    // // // F(B) = A & D
    // let node_b = tree::Tree::new(
    //     Vec::from([
    //         tree::and_node(),
    //         tree::val_node(0),
    //         tree::val_node(3),
    //     ])
    // );

    // // // F(C) = B
    // let node_c = tree::Tree::new(
    //     Vec::from([
    //         tree::val_node(1),
    //     ])
    // );

    // // // F(D) = !A | D
    // let node_d = tree::Tree::new(
    //     Vec::from([
    //         tree::or_node(),
    //         tree::not_node(0),
    //         tree::val_node(3),
    //     ])
    // );

    Vec::from([node_a])
}
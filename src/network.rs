use crate::network::tree::val_node;

mod tree;
mod exp_parser;

pub fn create_network(in_string: String) -> Vec<tree::Tree>{
    let tokens = exp_parser::extract_tokens(&in_string);

    let mut temp: Vec<tree::Node> = vec![];
    
    for token in tokens.iter().skip(1){
        match token.val[0]{
            33 => temp.push(tree::not_node(token.id)),
            38 => temp.push(tree::and_node()),
            124 => temp.push(tree::or_node()),
            _ => temp.push(val_node(token.id)),
        }
    }

    vec![tree::Tree::new(temp)]
}
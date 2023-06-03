mod tree;
mod exp_parser;

#[derive(Debug)]
//a collection of trees to form a full Boolean network
pub struct Network{
    pub trees: Vec<tree::Tree>,
    nodes: Vec<exp_parser::Token>,
    index: usize,
}

type State = Vec<bool>;

impl Network{

    pub fn resolve_node(&mut self, cur_state: &State, node: usize) -> Vec<bool>{
        //this could maybe be improved?
        let mut result: Vec<bool> = cur_state.to_vec();
        result[node] = self.trees[node].resolve(&cur_state);
        result
    }

    pub fn resolve_all(&mut self, cur_state: &State) -> Vec<bool>{
        //this could maybe be improved?
        let mut result: Vec<bool> = vec![false; cur_state.len()];
        for i in (0..self.trees.len()) {
            result[i] = self.trees[i].resolve(&cur_state);
        }
        result
    }

    //TODO this is horrible and needs cleaning up (non linear seach and comparison)
    fn get_id(&mut self, in_token: Vec<u8>) -> usize{
        for (i, node) in self.nodes.iter().enumerate() {
            if node.val == in_token{
                return i;
            }
        }
        self.nodes.push(exp_parser::Token{id: self.index, val: in_token});
        self.index += 1;
        return self.index -1;
    }

}


//TODO lots of fixes but not critical to operation
pub fn create_network(in_strings: Vec<String>) -> Network{

    //TODO lots of empty vector instantiations
    let mut out_network: Network = Network{trees: vec![], nodes: vec![], index: 0};

    for string in in_strings.iter() {
        let tokens = exp_parser::to_postfix(&string);

        //TODO horrid on multiple fronts
        //TODO only works if the first element is the LHS of expression and is bad design
        //TODO having to utilise clone to keep borrow checker happy with Vec<x> - alternative should be implemented
        out_network.get_id(tokens[0].val.clone());

        let mut tree: Vec<tree::Node> = vec![];
        
        for token in tokens.iter().skip(1){
            match token.val[0]{
                //TODO dont like this
                33 => tree.push(tree::not_node(out_network.get_id(token.val[1..].to_vec()))),
                38 => tree.push(tree::and_node()),
                124 => tree.push(tree::or_node()),
                //TODO clone needs removing
                _ => tree.push(tree::val_node(out_network.get_id(token.val.clone()))),
            }
        }

        out_network.trees.push(tree::Tree::new(tree));
    }

    out_network

}
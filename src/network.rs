mod tree;
mod exp_parser;

#[derive(Debug)]
pub struct Network{
    pub trees: Vec<tree::Tree>,
    nodes: Vec<exp_parser::Token>,
    index: usize,
}

impl Network{
    pub fn resolve(&mut self, cur_state: Vec<bool>) -> Vec<bool>{
        let mut result: Vec<bool> = vec![false; 4];
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

    let mut out_network: Network = Network{trees: vec![], nodes: vec![], index: 0};

    for string in in_strings.iter() {
        let tokens = exp_parser::to_postfix(&string);

        //horrid on multiple fronts
        //only works if the first element is the LHS of expression and is bad design
        //having to utilise clone to keep borrow checker happy with Vec<x> - alternative should be implemented
        out_network.get_id(tokens[0].val.clone());

        let mut tree: Vec<tree::Node> = vec![];
        
        for token in tokens.iter().skip(1){
            match token.val[0]{
                //dont like this
                33 => tree.push(tree::not_node(out_network.get_id(token.val[1..].to_vec()))),
                38 => tree.push(tree::and_node()),
                124 => tree.push(tree::or_node()),
                //clone needs removing
                _ => tree.push(tree::val_node(out_network.get_id(token.val.clone()))),
            }
        }

        out_network.trees.push(tree::Tree::new(tree));
    }

    out_network

}
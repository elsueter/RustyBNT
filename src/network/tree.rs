use serde::{Serialize};
use serde_json::{json, Value};

#[derive(Serialize)]
pub enum Op<T>{
    And,
    Or,
    Val(T)
}

pub struct Node<T>{
    op: Op<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    name: Option<u8>
}

impl Node<bool>{
    pub fn new(in_op: Op<bool>, in_left: Node<bool>, in_right: Node<bool>) -> Node<bool> {
        Node{op: in_op, left: Some(Box::new(in_left)), right: Some(Box::new(in_right)), name: None}
    }

    //string export stuff
    fn to_str(&self, mut buffer: Vec<String>, layer: usize, lr: bool) -> Vec<String>{

        let mut curr = "".to_string();

        if(buffer.len() < layer+1){
            buffer.push("".to_string());
        }
        
        if(lr){
            curr.push_str(&"|-".to_string())
        }else{
            curr.push_str(&"\\-".to_string())
        }

        match self.op{
            Op::And => {
                curr.push_str(&"And ".to_string());
                if(layer > 0){
                    buffer[layer-1].push_str(&"  ".to_string());
                }
            },
            Op::Or => {
                curr.push_str(&"Or  ".to_string());
                if(layer > 0){
                    buffer[layer-1].push_str(&"   ".to_string());
                }
            },
            Op::Val(x) => {
                if let Some(value) = self.name{ 
                    curr.push_str(&format!("{}", value));
                }else{
                    curr.push_str(&"".to_string());
                }
                if(lr){
                    if(buffer.len() < layer+2){
                        buffer.push("   ".to_string());
                    }else{
                        buffer[layer+1].push_str(&"   ".to_string());
                    }
                }
            }
        }
        buffer[layer].push_str(&curr);

        //TODO - see about new implementation without needing `let`
        if let Some(left) = &self.left {
            buffer = left.to_str(buffer, layer+1, true);
        }
        //TODO - see about new implementation without needing `let`
        if let Some(right) = &self.right {
            buffer = right.to_str(buffer, layer+1, false);
        }

        return buffer;
    }
    fn to_json(&self) -> Value{

        let l;

        if let Some(x) = &self.left{
            l = x.to_json();
        }else{
            l = Value::from("None");
        }

        let r;

        if let Some(x) = &self.right{
            r = x.to_json();
        }else{
            r = Value::from("None");
        }

        json!({
            "op": self.op,
            "left": l,
            "right": r,
            "name": self.name
        })
    }
    fn to_exp(&self) -> String{

        let l;

        if let Some(x) = &self.left{
            l = x.to_exp();
        }else{
            l = "".to_string();
        }

        let r;

        if let Some(x) = &self.right{
            r = x.to_exp();
        }else{
            r = "".to_string();
        }

        match self.op{
            Op::And => format!("{}&{}", l, r),
            Op::Or => format!("{}|{}", l, r),
            Op::Val(x) => self.name.unwrap().to_string()
        }
    }

}

pub fn and_node(l: Node<bool>, r: Node<bool>) -> Node<bool> {Node::new(Op::And, l, r)}
pub fn or_node(l: Node<bool>, r: Node<bool>) -> Node<bool> {Node::new(Op::Or, l, r)}
pub fn val_node(in_val: bool, in_name: u8) -> Node<bool> {Node::<bool>{op: Op::Val(in_val), left: None, right: None, name: Some(in_name)}}
pub fn not_val_node(in_val: bool, in_name: u8) -> Node<bool> {Node::<bool>{op: Op::Val(!in_val), left: None, right: None, name: Some(in_name)}}

pub struct BooleanTree<T>{nodes: Option<Vec<Node<T>>>, root: Node<T>}

impl BooleanTree<bool>{
    pub fn new(in_node: Node<bool>) -> BooleanTree<bool> {BooleanTree{nodes: None, root: in_node}}

    pub fn resolve(&self) -> bool {
        Self::resolve_node(&self.root)
    }

    fn resolve_node(cur_node: &Node<bool>) -> bool {
        let l = match &cur_node.left{
            Some(x) => BooleanTree::resolve_node(x),
            None => false
        };

        let r = match &cur_node.right{
            Some(x) => BooleanTree::resolve_node(x),
            None => false
        };

        match cur_node.op{
            Op::And => {l & r},
            Op::Or => {l | r},
            Op::Val(x) => x
        }
    }

    //string export stuff
    pub fn to_str(&self) -> String{
        let buffer = Vec::new();
        self.root.to_str(buffer, 0, false).join("\n")
    }
    pub fn to_json(&self) -> String{
        serde_json::to_string_pretty(&self.root.to_json()).unwrap()
    }
    pub fn to_exp(&self) -> String{
        self.root.to_exp()
    }
}
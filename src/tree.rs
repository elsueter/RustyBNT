use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub enum Op<T>{
    And,
    Or,
    Val(T)
}

pub struct Node<'a, T>{
    op: Op<T>,
    left: Option<&'a Node<'a, T>>,
    right: Option<&'a Node<'a, T>>,
    name: Option<u8>
}

impl Node<'_, bool>{

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

        if let Some(x) = self.left{
            l = x.to_json();
        }else{
            return json!({
                "op": self.op,
                "name": self.name
            })
        }

        let r;

        if let Some(x) = self.right{
            r = x.to_json();
        }else{
            return json!({
                "op": self.op,
                "name": self.name
            })
        }

        json!({
            "op": self.op,
            "left": l,
            "right": r
        })
    }

    fn to_exp(&self) -> String{

        let l;

        if let Some(x) = self.left{
            l = x.to_exp();
        }else{
            l = "".to_string();
        }

        let r;

        if let Some(x) = self.right{
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

    pub fn and_node<'a>(l: &'a Node<'a, bool>, r: &'a Node<'a, bool>) -> Node<'a, bool> {Node::<bool>{op: Op::And, left: Some(l), right: Some(r), name: None}}
    pub fn or_node<'a>(l: &'a Node<'a, bool>, r: &'a Node<'a, bool>) -> Node<'a, bool> {Node::<bool>{op: Op::Or, left: Some(l), right: Some(r), name: None}}
    pub fn val_node(in_val: bool, in_name: u8) -> Node<'static, bool> {Node::<bool>{op: Op::Val(in_val), left: None, right: None, name: Some(in_name)}}
    pub fn not_val_node(in_val: bool, in_name: u8) -> Node<'static, bool> {Node::<bool>{op: Op::Val(!in_val), left: None, right: None, name: Some(in_name)}}
}

pub struct BooleanTree<'a, T>{root: Node<'a, T>}

impl BooleanTree<'_, bool>{
    pub fn new(in_node: Node<bool>) -> BooleanTree<bool> {BooleanTree{root: in_node}}

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

    pub fn resolve(&self) -> bool {
        Self::resolve_node(&self.root)
    }

    fn resolve_node(cur_node: &Node<bool>) -> bool {
        let mut l: Option<bool> = None;
        let mut r: Option<bool> = None;

        if let Some(left) = &cur_node.left {
            l = Some(BooleanTree::resolve_node(left));
        }
        if let Some(right) = &cur_node.right {
            r = Some(BooleanTree::resolve_node(right));
        }

        let l: bool = if let Some(x) = l{x} else{false};
        let r: bool = if let Some(x) = r{x} else{false};

        match cur_node.op{
            Op::And => {l & r},
            Op::Or => {l | r},
            Op::Val(x) => x
        }
    }
}
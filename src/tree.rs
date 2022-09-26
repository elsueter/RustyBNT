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

    pub fn to_json(&self) -> Value{
        self.root.to_json()
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
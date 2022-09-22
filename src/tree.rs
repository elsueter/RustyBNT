pub enum Op<T>{
    And,
    Or,
    Val(T)
}

pub struct Node<'a, T>{
    op: Op<T>,
    left: Option<Box<&'a Node<'a, T>>>,
    right: Option<Box<&'a Node<'a, T>>>,
    name: Option<u8>
}

impl Node<'_, bool>{}

pub fn and_node<'a>(l: &'a Node<'a, bool>, r: &'a Node<'a, bool>) -> Node<'a, bool> {Node::<bool>{op: Op::And, left: Some(Box::new(l)), right: Some(Box::new(r)), name: None}}
pub fn or_node<'a>(l: &'a Node<'a, bool>, r: &'a Node<'a, bool>) -> Node<'a, bool> {Node::<bool>{op: Op::Or, left: Some(Box::new(l)), right: Some(Box::new(r)), name: None}}
pub fn val_node(in_val: bool, in_name: u8) -> Node<'static, bool> {Node::<bool>{op: Op::Val(in_val), left: None, right: None, name: Some(in_name)}}
pub fn not_val_node(in_val: bool, in_name: u8) -> Node<'static, bool> {Node::<bool>{op: Op::Val(!in_val), left: None, right: None, name: Some(in_name)}}

pub struct BooleanTree<'a, T>{root: Node<'a, T>}

impl BooleanTree<'_, bool>{
    pub fn new(in_node: Node<bool>) -> BooleanTree<bool> {BooleanTree{root: in_node}}

    pub fn resolve(&self) -> bool {
        Self::resolve_node(&Box::new(&self.root))
    }

    fn resolve_node(cur_node: &Box<&Node<bool>>) -> bool {
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
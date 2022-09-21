#[derive(Clone)]
enum Op<T>{
    And,
    Or,
    Val(T)
}

#[derive(Clone)]
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
}

pub fn and_node(l: Node<bool>, r: Node<bool>) -> Node<bool> {Node::new(Op::And, l, r)}
pub fn or_node(l: Node<bool>, r: Node<bool>) -> Node<bool> {Node::new(Op::Or, l, r)}
pub fn val_node(in_val: bool, in_name: u8) -> Node<bool> {Node::<bool>{op: Op::Val(in_val), left: None, right: None, name: Some(in_name)}}
pub fn not_val_node(in_val: bool, in_name: u8) -> Node<bool> {Node::<bool>{op: Op::Val(!in_val), left: None, right: None, name: Some(in_name)}}

pub struct BooleanTree<T>{root: Node<T>}

impl BooleanTree<bool>{
    pub fn new(in_node: Node<bool>) -> BooleanTree<bool> {BooleanTree{root: in_node}}

    pub fn resolve(&self) -> bool {
        Self::resolve_node(Box::new(self.root.clone()))
    }

    fn resolve_node(cur_node: Box<Node<bool>>) -> bool {
        let mut l: Option<bool> = None;
        let mut r: Option<bool> = None;

        if let Some(left) = cur_node.left {
            l = Some(BooleanTree::resolve_node(left));
        }
        if let Some(right) = cur_node.right {
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
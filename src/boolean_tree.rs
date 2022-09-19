type ChildNode<T> = Option<Box<BTNode<T>>>;

enum Op<T>{
    And,
    Or,
    Val(T)
}

pub struct BTNode<T>{
    op: Op<T>,
    left: ChildNode<T>,
    right: ChildNode<T>,
    name: Option<u8>
}

impl BTNode<bool>{
    fn new(in_op: Op<bool>, l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {BTNode::<bool>{op: in_op, left: Some(Box::new(l)), right: Some(Box::new(r)), name: None}}
}

pub fn and_node(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {BTNode::new(Op::And, l, r)}
pub fn or_node(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {BTNode::new(Op::Or, l, r)}
pub fn val_node(in_val: bool, in_name: u8) -> BTNode<bool> {BTNode::<bool>{op: Op::Val(in_val), left: None, right: None, name: Some(in_name)}}
pub fn not_val_node(in_val: bool, in_name: u8) -> BTNode<bool> {BTNode::<bool>{op: Op::Val(!in_val), left: None, right: None, name: Some(in_name)}}

pub struct BooleanTree<T>{pub root: Option<BTNode<T>>}

impl BooleanTree<bool>{
    pub fn new(in_root: BTNode<bool>) -> Self {BooleanTree::<bool>{root: Some(in_root)}}

    pub fn resolve(self) -> bool{BooleanTree::resolve_node(&Box::new(self.root.unwrap()))}

    fn resolve_node(cur_node: &Box<BTNode<bool>>) -> bool {
        let mut l: Option<bool> = None;
        let mut r: Option<bool> = None;

        if let Some(left) = &cur_node.left{
            l = Some(BooleanTree::resolve_node(left))
        }
        if let Some(right) = &cur_node.right{
            r = Some(BooleanTree::resolve_node(right))
        }

        let l = if let Some(x) = l{x} else{false};
        let r = if let Some(x) = r{x} else{false};

        match cur_node.op{
            Op::And => {r & l},
            Op::Or => {r | l},
            Op::Val(x) => x
        }
    }
}
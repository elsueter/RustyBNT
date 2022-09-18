type ChildNode<T> = Option<Box<BTNode<T>>>;

enum Op<T> {
    And,
    Or,
    Id(T)
}

struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>
}

impl BTNode<bool> {
    pub fn new(op: Op<bool>, l: BTNode<bool>, r: BTNode<bool>) -> Self {
        BTNode::<bool>{
            op: op, left: Some(Box::new(l)), right: Some(Box::new(r))
        }
    }
}

fn AndNode(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {BTNode::new(Op::And, l, r)}
fn OrNode(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {BTNode::new(Op::Or, l, r)}
fn IdNode(value: bool) -> BTNode<bool> {BTNode{op: Op::Id(value), left: None, right: None}}
fn NotNode(value: bool) -> BTNode<bool> {IdNode(!value)}

struct BooleanTree<T> {
    head: Option<BTNode<T>>
}

impl BooleanTree<bool> {
    pub fn new(head: BTNode<bool>) -> Self {BooleanTree::<bool>{head: Some(head)}}

    pub fn collapse(node: &Box<BTNode<bool>>) -> bool {
        let mut l: Option<bool> = None;
        let mut r: Option<bool> = None;

        if let Some(left) = &node.left {
            l = Some(BooleanTree::collapse(left));
        }

        if let Some(right) = &node.right {
            r = Some(BooleanTree::collapse(right));
        }

        let l = if let Some(x) = l{x} else {false};
        let r = if let Some(x) = r{x} else {false};

        match node.op {
            Op::And => {l & r}
            Op::Or => {l | r}
            Op::Id(x) => x, 
        }
    }
}

fn main() {
    
    let bt = BooleanTree::new(
        AndNode(
            IdNode(true),
            IdNode(true)
        )
    );
    

    println!("{}", BooleanTree::collapse(
        &Box::new(bt.head.expect("No head initialized.")))
    )
}
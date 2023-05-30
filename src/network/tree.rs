#[derive(Debug)]
pub enum Op{
    And,
    Or,
    Not,
    Val
}

#[derive(Debug)]
pub struct Node{
    op: Op,
    val: bool,
    id: usize
}

impl Node{
    fn new(in_op: Op, in_id: usize) -> Node {Node{val: false, op: in_op, id: in_id}}
}

pub fn and_node() -> Node {Node::new(Op::And, usize::MAX)}
pub fn or_node() -> Node {Node::new(Op::Or, usize::MAX)}
pub fn not_node(in_id: usize) -> Node {Node::new(Op::Not, in_id)}
pub fn val_node(in_id: usize) -> Node {Node::new(Op::Val, in_id)}

#[derive(Debug)]
pub struct Tree{nodes: Vec<Node>}

impl Tree{
    pub fn new(in_nodes: Vec<Node>) -> Tree {Tree{nodes: in_nodes}}

    pub fn resolve(&mut self, cur_state: Vec<bool>) -> bool{
        let mut buffer: Vec<bool> = vec![];
        for node in self.nodes.iter(){
            match node.op{
                Op::And =>{
                    let a = buffer.pop().unwrap();
                    let b = buffer.pop().unwrap();
                    buffer.push(a & b);
                },
                Op::Or => {
                    let a = buffer.pop().unwrap();
                    let b = buffer.pop().unwrap();
                    buffer.push(a | b);
                },
                Op::Not => buffer.push(!cur_state[node.id]),
                Op::Val => buffer.push(cur_state[node.id]),
            }
        }
        buffer[0]
    }
}
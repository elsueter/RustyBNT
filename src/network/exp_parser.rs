use bytes::BufMut;

// F(A) = !B & C | D

pub fn shunt_expression(in_string: String) -> Vec<Vec<u8>>{
    let mut nodes: Vec<u8> = vec![];
    let mut ops: Vec<u8> = vec![];
    for c in in_string.as_bytes(){
        match c{
            32 => (),
            38 => ops.put_u8(*c),
            // 61 => (),
            124 => ops.put_u8(*c),
            _ => nodes.put_u8(*c)
        }
    }

    vec![ops, nodes]
}
#[derive(Debug)]
pub struct Token{
    id: u8,
    val: Vec<u8>
}

// F(A) = !B & C | D

pub fn extract_tokens(in_string: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut index = 1;

    let mut cur_token: Vec<u8> = vec![];
    for c in in_string.as_bytes(){
        match c{
            // ' '
            32 => {
                if cur_token.len() > 0 {
                    tokens.push(Token{id: index, val: cur_token});
                    cur_token = vec![];
                    index += 1;
                }
            },
            // '&','|'
            38 | 124 => (),
            // '(',')'
            40 | 41 => (),
            // '='
            61 => (),
            _ => cur_token.push(*c)
        }
    }
    tokens.push(Token{id: index, val: cur_token});

    tokens
}
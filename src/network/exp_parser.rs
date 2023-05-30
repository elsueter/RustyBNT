#[derive(Debug)]
pub struct Token{
    pub id: usize,
    pub val: Vec<u8>
}

pub fn extract_tokens(in_string: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut operators: Vec<Token> = vec![];
    let mut index = 0;

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
            38 | 124 => {
                operators.push(Token{id: 0, val: vec![*c]});
            },
            // '('
            40 => {
                operators.push(Token{id: 0, val: vec![*c]});
            },
            // ')'
            41 => {
                if cur_token.len() > 0 {
                    tokens.push(Token{id: index, val: cur_token});
                    cur_token = vec![];
                    index += 1;
                }

                while let Some(op) = operators.pop(){
                    if op.val[0] == 40 {
                        //operators.pop();
                        break;
                    }else{
                        tokens.push(op);
                    }
                }
            },
            // '='
            61 => (),
            _ => cur_token.push(*c)
        }
    }
    if cur_token.len() > 0 {
        tokens.push(Token{id: index, val: cur_token});
    }

    while let Some(op) = operators.pop(){
        tokens.push(op);
    }

    tokens
}
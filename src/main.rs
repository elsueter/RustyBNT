mod network;

fn main() {
    let mut network = network::create_network(vec![
        "A = !B & C | D".to_string(),
        "B = C & D".to_string(),
        "C = !C".to_string(),
        "D = (!A & C) | B".to_string(),
        "E = (D & A) | (!A & B)".to_string()
        ]);

    let mut temp: Vec<bool> = vec![false, false, false, false, false];

    for i in 0..10{
        temp = network.resolve_all(&temp);
        println!("{:?}", temp);
    }
}
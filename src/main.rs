mod network;

fn main() {
    let mut network = network::create_network(vec![
        "A = !B & C | D".to_string(),
        "B = C & D".to_string(),
        "C = !C".to_string(),
        "D = (!A & C) | B".to_string()
        ]);
    let temp = network.resolve(Vec::from([true, false, true, false]));

    println!("{} {} {} {}", temp[0], temp[1], temp[2], temp[3]);
}
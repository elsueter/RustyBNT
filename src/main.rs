mod network;

fn main() {
    let mut network = network::create_network("A = !B & C | D".to_string());
    network[0].resolve(Vec::from([false, true, true, true]));
}
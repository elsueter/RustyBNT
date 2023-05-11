mod network;

fn main() {
    let inputstr = "F(A) = !B & C | D".to_string();
    println!("input string: {:?}", inputstr);
    let mut network = network::create_network(inputstr);
    let mut network2 = network::test();
    println!("Expression parsed network{:?}", network);
    println!("Known test network{:?}", network2);

    use std::time::Instant;
    let now = Instant::now();

    println!("network 1 results: {:?}", network[0].resolve(Vec::from([false, false, false, false])));
    println!("network 2 results: {:?}", network2[0].resolve(Vec::from([false, false, false, false])));

    let elapsed = now.elapsed();
    println!("For Elapsed: {:.2?}", elapsed);

    // network::create_network("F(A) = !B & C | D".to_string());

}
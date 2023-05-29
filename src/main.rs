mod network;

fn main() {
    let inputstr = "A = !B & C | D".to_string();
    println!("input string: {:?}", inputstr);
    
    use std::time::Instant;
    let now = Instant::now();

    let mut network = network::create_network(inputstr);
    println!("{:?}", network[0].resolve(Vec::from([false, false, false, false])));
    
    let elapsed = now.elapsed();
    println!("For Elapsed: {:.2?}", elapsed);
}
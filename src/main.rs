mod network;

fn main() {

    let network = network::test();

    use std::time::Instant;
    let now = Instant::now();

    for node in network.iter(){
        println!("{}", node.resolve());
    }

    let elapsed = now.elapsed();
    println!("For Elapsed: {:.2?}", elapsed);
}
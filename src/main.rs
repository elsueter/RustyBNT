mod network;

fn main() {

    let mut network = network::test();

    use std::time::Instant;
    let now = Instant::now();

    for i in 0..network.len(){
        network[i].resolve(Vec::from([false, false, false, false]));
    }

    let elapsed = now.elapsed();
    println!("For Elapsed: {:.2?}", elapsed);

}
use std::time::Instant;

fn main() {
    let start = Instant::now();

    for i in 1..=100_000_000 {
        if i % 100_000_000 == 0 {
            println!("{}", i);
        }
    }

    let duration = start.elapsed();
    println!("Elapsed duration: {:.2?}", duration);
}

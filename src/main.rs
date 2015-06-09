extern crate rand;

use std::io;
use std::num;
use rand::Rng;

fn main() {
    const MAX : usize = 256;
    let mut freq = [0; MAX];
    let mut rng = rand::thread_rng();

    for _ in 0..MAX*16384 {
        let n = rng.gen_range(0, MAX);
        freq[n] += 1;
    }

    let mut tdiff = 0;
    for i in 0..MAX {
        let diff = freq[i] - 16384;
        tdiff += diff;
        println!("{:3}: {:4}", i, diff);
    }

    println!("{}", tdiff);
}

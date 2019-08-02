extern crate mpi;

use mpi::request::WaitGuard;
use mpi::traits::*;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();


    if rank==0{
        let input_file = std::env::args().nth(1).expect("Program requires input file path");
        println!("{:?}", input_file);
        let f = File::open(input_file).unwrap();
        let file = BufReader::new(&f);
        for (num, line) in file.lines().enumerate() {
            if num == 0{
                let numbers: Vec<i32> = line.unwrap().split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                println!("{:?}",numbers);
                let s=numbers[0]/size;  //how many slices
                let m = numbers[0]%size; //modulus
                if m != 0 {
                    panic!("Input size needs to be divisible by number of processors");
                }
                let mut theBoard: [[i32; numbers[0]]; numbers[0]];
            }
            else{
                for (i, c) in line.chars().enumerate() {
                    print!("{}",c);
                    // do something with character `c` and index `i`
                }
                println!("");
            }
        }
    }
    // let processor = mpi::environment::processor_name();
    // println!("Hello from task {} on {:?}!",rank,processor);
    // if rank==0{
    //     println!("MASTER: Number of MPI tasks is: {}",size);
    // }
}

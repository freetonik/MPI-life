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
        let mut args: Vec<String> = env::args().collect();
        println!("{:?}", args);
        if args.len() < 2{
            panic!("Requires at least 1 argument to run");
        }
        let f = File::open(args[1]).unwrap();
        let file = BufReader::new(&f);
        for (num, line) in file.lines().enumerate() {
            if num == 0{
                let numbers: Vec<i32> = line.unwrap().split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                println!("{:?}",numbers);
            }
        }
    }
    // let processor = mpi::environment::processor_name();
    // println!("Hello from task {} on {:?}!",rank,processor);
    // if rank==0{
    //     println!("MASTER: Number of MPI tasks is: {}",size);
    // }
}

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
                let N:usize = numbers[0] as usize;
                let mut theBoard = vec![vec![0; N]; N];
            }
            else{
                // let str_line: String
                for (i, c) in line.ok().unwrap().chars().enumerate() {
                    theBoard[num-1][i] = c-'0';
                    // do something with character `c` and index `i`
                }
                // println!("");
            }
        }
        println!("{:?}", theBoard);
    }
    // let processor = mpi::environment::processor_name();
    // println!("Hello from task {} on {:?}!",rank,processor);
    // if rank==0{
    //     println!("MASTER: Number of MPI tasks is: {}",size);
    // }
}

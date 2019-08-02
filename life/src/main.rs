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
        let mut the_board: Vec<Vec<i32>> = vec![vec![0; 1]; 1];
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
                let n:usize = numbers[0] as usize;
                the_board = vec![vec![0; n]; n];
            }
            else{
                //iterate over the chars for each line and convert string 0/1 into ints
                //put the ints into theBoard with the initial state
                for (i, c) in line.ok().unwrap().chars().enumerate() {
                    the_board[num-1][i] = (c as i32)-('0' as i32);
                }
            }
        }
        println!("{:?}", the_board);
        let mut info[4]:i32;
        info[0]=n; info[1]=s; info[2]=numbers[1]; info[3]=numbers[2];
        for (let mut dest:i32 =0; dest<size; dest++) {
            MPI_Send(&info, 4, MPI_INT, dest, 1, MPI_COMM_WORLD); //send info
        }

    }
    // let processor = mpi::environment::processor_name();
    // println!("Hello from task {} on {:?}!",rank,processor);
    // if rank==0{
    //     println!("MASTER: Number of MPI tasks is: {}",size);
    // }
}

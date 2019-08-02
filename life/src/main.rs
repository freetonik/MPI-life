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
        let mut s:i32 = 0;
        let mut n:i32 = 0;
        let mut generations:i32 = 0;
        let mut out_points:i32 = 0;
        for (num, line) in file.lines().enumerate() {
            if num == 0{
                let numbers: Vec<i32> = line.unwrap().split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                println!("{:?}",numbers);
                s=numbers[0]/size;  //how many slices
                let m = numbers[0]%size; //modulus
                if m != 0 {
                    panic!("Input size needs to be divisible by number of processors");
                }
                n = numbers[0];
                the_board = vec![vec![0; n as usize]; n as usize];
                generations = numbers[1];
                out_points = numbers[2];
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
        let mut info: Vec<i32> = Vec::new();
        info.push(n);
        info.push(s);
        info.push(generations);
        info.push(out_points);
        println!("{:?}",info);
        for dest in 0..size {
            world.process_at_rank(dest).send(&info[..]); //send info
        }
        for dest in 0..size{
          let mut slice: Vec<Vec<i32>> = Vec::new();
          for k in 0..s{
            for l in 0..n{
              slice[k][l]=the_board[k+(z*s)][l];  //cut a slice from the the board
            }
          }
          world.process_at_rank(dest).send(&slice[..]);
          // MPI_Send(&slice, n*s, MPI_INT, dest, 1, MPI_COMM_WORLD);  //and send it
        }
    }
    // let processor = mpi::environment::processor_name();
    // println!("Hello from task {} on {:?}!",rank,processor);
    // if rank==0{
    //     println!("MASTER: Number of MPI tasks is: {}",size);
    // }
}

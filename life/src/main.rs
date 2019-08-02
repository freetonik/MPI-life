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
        // println!("{:?}", the_board);
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
          for k in 0..s{
            let mut slice: Vec<i32> = Vec::new();
            for l in 0..n{
              slice.push(the_board[(k+(dest*s)) as usize][l as usize]);  //cut a slice from the the board
            }
            world.process_at_rank(dest).send(&slice[..]);
          }
          // MPI_Send(&slice, n*s, MPI_INT, dest, 1, MPI_COMM_WORLD);  //and send it
        }
    }
    let (msg, status) = world.any_process().receive_vec::<i32>();
    // println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status);
    let n:i32 = msg[0];
    let s:i32 = msg[1];
    let generations:i32 = msg[2];
    let out_points:i32 = msg[3];
    let mut slice: Vec<Vec<i32>> = Vec::new();
    for _k in 0..s{
        let (msg, status) = world.any_process().receive_vec::<i32>();
        // println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status);
        slice.push(msg.clone());
    }
    // println!("Process {} got slice {:?}",rank,slice);
    let mut fromdown:Vec<i32>= Vec::new();
    let mut fromup:Vec<i32>= Vec::new();  //Vectors to send and to receive
    // for g in 1..generations{ //generations for loop
    for g in 1..2{ //generations for loop
        if rank!=size-1 {// all except for last send down
          // println!("Process {} slice {:?}",rank, slice[(s-1) as usize]);
          world.process_at_rank(rank+1).send_with_tag(&slice[(s-1) as usize][..],1);//sending data up => tag=1
          println!("Process {} sent data to {}",rank, rank+1);
        } else {
          fromdown = vec![0; n as usize]; // last one generates empty stripe "from down"
        }
        if rank!=0{ // all except for first receive from up
            println!("Process {} wait data from {}",rank, rank-1);
            let (msg, _status) = world.any_process().receive_vec_with_tag::<i32>(1);
            fromup=msg;
        } else {
            fromup = vec![0; n as usize]; // first one generats empty line "from up"
        }
        if rank!=0{ // all except for first send up
            world.process_at_rank(rank-1).send_with_tag(&slice[(s-1) as usize][..],0);//sending data down => tag=0
            println!("Process {} sent data to {}",rank, rank-1);
        }
        if rank!=size-1 { // all except for last receive from down
            println!("Process {} wait data from {}",rank, rank+1);
            let (msg, _status) = world.any_process().receive_vec_with_tag::<i32>(0);
            fromdown=msg;
        }
        println!("Process {} fromup {:?} fromdown {:?}",rank,fromup,fromdown);
    }
    // let processor = mpi::environment::processor_name();
    // println!("Hello from task {} on {:?}!",rank,processor);
    // if rank==0{
    //     println!("MASTER: Number of MPI tasks is: {}",size);
    // }
}

extern crate mpi;

use mpi::request::WaitGuard;
use mpi::traits::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let processor = mpi::environment::processor_name();
    println!("Hello from task {} on {:?}!",rank,processor);
    if rank==0{
        println!("MASTER: Number of MPI tasks is: {}",size);
    }
}

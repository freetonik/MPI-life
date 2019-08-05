from mpi4py import MPI
# import numpy
#
# comm = MPI.COMM_WORLD
# rank = comm.Get_rank()

# from mpi4py import MPI
import sys

comm = MPI.COMM_WORLD
size = comm.Get_size()
rank = comm.Get_rank()
name = MPI.Get_processor_name()

if len(sys.argv) < 2:
    if rank == 0:
        print("Needs 1 argument w/ input file")
    quit()

if rank == 0:
    f = open(sys.argv[1],"r")
    n = 0
    s = 0
    generations = 0
    out_points = 0
    the_board = []
    for num, line in enumerate(f, 1):
        if num == 1:
            line = line.split()
            n=int(line[0])
            s=int(n/size)
            m=n%size;
            if m != 0:
                print("Input size needs to be divisible by number of processors")
                quit()
            generations=int(line[1])
            out_points=int(line[2])
        else:
            l_array=[]
            for c in line:
                if c == '0' or c == '1':
                    l_array.append(int(c))
            the_board.append(l_array)

    # print(the_board)
    # print("Num rows: " + str(len(the_board)))
    # for row in the_board:
    #     print(len(row))
    info = []
    info.append(n);
    info.append(s);
    info.append(generations);
    info.append(out_points);
    for dest in range(size):
        comm.send(info, dest=dest, tag=1)
        # world.process_at_rank(dest).send(&info[..]); //send info
    for dest in range(size):
        # let mut slice: Vec<i32> = Vec::new();
        # for l in 0..n{
        #   slice.push(the_board[(k+(dest*s)) as usize][l as usize]);  //cut a slice from the the board
        # }
        # world.process_at_rank(dest).send(&slice[..]);
        start=dest*s
        end=dest*s+s
        comm.send(the_board[start:end], dest=dest, tag=1)
      # // MPI_Send(&slice, n*s, MPI_INT, dest, 1, MPI_COMM_WORLD);  //and send it


info = comm.recv(source=0, tag=1)
print("Hello, World! I am process "+ str(rank) + " with info: " + str(info))

slice = comm.recv(source=0, tag=1)
print("Hello, World! I am process "+ str(rank) + " with slice: " + str(slice) + " \n" + str(len(slice)))


# passing MPI datatypes explicitly
# if rank == 0:
#     data = numpy.arange(1000, dtype='i')
#     comm.Send([data, MPI.INT], dest=1, tag=77)
# elif rank == 1:
#     data = numpy.empty(1000, dtype='i')
#     comm.Recv([data, MPI.INT], source=0, tag=77)

# automatic MPI datatype discovery
# if rank == 0:
#     data = numpy.arange(100, dtype=numpy.float64)
#     print("Sending rank "+str(rank))
#     comm.Send(data, dest=1, tag=13)
#     print("Sent rank "+str(rank))
#     data = numpy.empty(100, dtype=numpy.float64)
#     print("Receiving rank "+str(rank))
#     comm.Recv(data, source=1, tag=13)
#     print("Recieved rank "+str(rank))
#     print(data)
# elif rank == 1:
#     data = numpy.empty(100, dtype=numpy.float64)
#     print("Receiving rank "+str(rank))
#     comm.Recv(data, source=0, tag=13)
#     print("Recieved rank "+str(rank))
#     print(data)
#     data = numpy.arange(100, dtype=numpy.float64)
#     print("Sending rank "+str(rank))
#     comm.Send(data, dest=0, tag=13)
#     print("Sent rank "+str(rank))

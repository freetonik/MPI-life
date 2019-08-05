from mpi4py import MPI
# import numpy
#
# comm = MPI.COMM_WORLD
# rank = comm.Get_rank()

# from mpi4py import MPI
import sys

size = MPI.COMM_WORLD.Get_size()
rank = MPI.COMM_WORLD.Get_rank()
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
            s=n/size
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
    print(the_board)

sys.stdout.write(
    "Hello, World! I am process %d of %d on %s.\n"
    % (rank, size, name))


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

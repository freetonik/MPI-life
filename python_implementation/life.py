from mpi4py import MPI
import numpy

comm = MPI.COMM_WORLD
rank = comm.Get_rank()

# passing MPI datatypes explicitly
# if rank == 0:
#     data = numpy.arange(1000, dtype='i')
#     comm.Send([data, MPI.INT], dest=1, tag=77)
# elif rank == 1:
#     data = numpy.empty(1000, dtype='i')
#     comm.Recv([data, MPI.INT], source=0, tag=77)

# automatic MPI datatype discovery
if rank == 0:
    data = numpy.arange(100, dtype=numpy.float64)
    print("Sending rank "+str(rank))
    comm.Send(data, dest=1, tag=13)
    data = numpy.empty(100, dtype=numpy.float64)
    comm.Recv(data, source=1, tag=13)
    print("Recieved rank "+str(rank))
    print(data)
elif rank == 1:
    data = numpy.empty(100, dtype=numpy.float64)
    comm.Recv(data, source=0, tag=13)
    print("Recieved rank "+str(rank))
    print(data)
    data = numpy.arange(100, dtype=numpy.float64)
    print("Sending rank "+str(rank))
    comm.Send(data, dest=0, tag=13)

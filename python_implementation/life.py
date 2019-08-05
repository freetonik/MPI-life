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
    for dest in range(size):
        #Break the board into slices for each processor to handle.
        start=dest*s
        end=dest*s+s
        comm.send(the_board[start:end], dest=dest, tag=1)


info = comm.recv(source=0, tag=1)
print("I am process "+ str(rank) + " with info: " + str(info))

slice = comm.recv(source=0, tag=1)
print("I am process "+ str(rank) + " with slice: " + str(slice) + " \n" + str(len(slice)))

for g in range(1): #generations for loop
    if rank!=size-1: # all except for last send down
        comm.send(slice[info[1]-1],dest=rank+1,tag=1) #sending data up
        print("Process " + str(rank) + " sent data to "+str(rank+1));
    else:
        fromup = [0] * info[0] # last one generates empty stripe "from up"
    if rank!=0: # all except for first receive from down
        print("Process " + str(rank) + " wait on data from "+str(rank-1))
        # println!("Process {} wait data from {}",rank, rank-1);
        # io::stdout().flush().unwrap();
        # let (msg, _status) = world.process_at_rank(rank-1).receive_vec_with_tag::<i32>(1);
        # fromup=msg;
        fromdown = comm.recv(source=rank-1,tag=1)
    else:
        fromdown = [0] * info[0] # first one generats empty line "from down"
    if rank!=0: # all except for first send up
        comm.send(slice[0],dest=rank-1,tag=1) #sending data down
        print("Process " + str(rank) + " sent data to "+str(rank-1));
    if rank!=size-1: # all except for last receive from up
        print("Process " + str(rank) + " wait on data from "+str(rank+1))
        # # println!("Process {} wait data from {}",rank, rank+1);
        # io::stdout().flush().unwrap();
        # let (msg, _status) = world.process_at_rank(rank+1).receive_vec_with_tag::<i32>(0);
        # fromdown=msg;
        fromup = comm.recv(source=rank+1,tag=1)

    print("Process "+ str(rank) +" fromup: "+ str(fromup) + " \nfromdown: " + str(fromdown) + "\n")

    sum=0 #sum of neighbours
    s=info[1]
    n=info[0]
    newslice = [[0] * n for i in range(s)] #newslice[s][N];
    print(newslice)

    for x in range(s): #for each row
        for y in range(n): #for each column
            if x==0 and y==0: #upper-left cell
              sum = slice[x+1][y]+slice[x+1][y+1]+slice[0][y+1]+fromup[0]+fromup[1]
            elif x==0 and y==n-1: #upper-right cell
              sum = slice[x][y-1]+slice[x+1][y-1]+slice[x+1][y]+fromup[n-1]+fromup[n-2]
            elif x==s-1 and y==0: #lower-left cell
              sum = slice[x][y+1]+slice[x-1][y+1]+slice[x-1][y]+fromdown[0]+fromdown[1]
            elif x==s-1 and y==n-1: #lower-right cell
              sum = slice[x-1][y]+slice[x-1][y-1]+slice[x][y-1]+fromdown[n-1]+fromdown[n-2];
            else: #not corner cells
              if y==0: #leftmost line, not corner
                sum=slice[x-1][y]+slice[x-1][y+1]+slice[x][y+1]+slice[x+1][y+1]+slice[x+1][y];
              elif y==n-1: #rightmost line, not corner
                sum=slice[x-1][y]+slice[x-1][y-1]+slice[x][y-1]+slice[x+1][y-1]+slice[x+1][y];
              elif x==0: #uppermost line, not corner
                sum=slice[x][y-1]+slice[x+1][y-1]+slice[x+1][y]+slice[x+1][y+1]+slice[x][y+1]+fromup[y-1]+fromup[y]+fromup[y+1];
              elif x==s-1: #lowermost line, not corner
                sum=slice[x-1][y-1]+slice[x-1][y]+slice[x-1][y+1]+slice[x][y+1]+slice[x][y-1]+fromdown[y-1]+fromdown[y]+fromdown[y+1];
              else: #general case, any cell within
                sum=slice[x-1][y-1]+slice[x-1][y]+slice[x-1][y+1]+slice[x][y+1]+slice[x+1][y+1]+slice[x+1][y]+slice[x+1][y-1]+slice[x][y-1];
            #PUT THE NEW VALUE OF A CELL
            test = sum==2 or sum==3
            if slice[x][y]==1 and test:
                newslice[x][y]=1
            elif slice[x][y]==1 and sum>3:
                newslice[x][y]=0
            elif slice[x][y]==1 and sum<1:
                newslice[x][y]=0
            elif slice[x][y]==0 and sum==3:
                newslice[x][y]=1
            else:
                newslice[x][y]=0
    slice = newslice
    print("I am process "+ str(rank) + " after generation " + str(g) + " with slice: " + str(slice))

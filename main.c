#include <stdio.h>
#include "mpi.h"
int main(int argc, char const *argv[])
{
    MPI_Init(NULL,NULL);
    printf("Wonderful class !\n");
    MPI_Finalize();
    /* code */
    return 0;
}

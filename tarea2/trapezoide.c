#include <mpi.h>
#include <stdio.h>

double exp3(double x){
	return x*x*x;
};
double trapeze( double left_endpt,double right_endpt,int trap_count,double base_len){
	double estimate, x;
	int i;
	if(right_endpt){
        return;
	}
	estimate=(exp3(left_endpt)+exp3(right_endpt))/2.0;
	for(i=0;i<=trap_count-1;i++){
		x=left_endpt+i;
		estimate += exp3(x);
	}
	estimate=estimate*base_len;
	return estimate;
}

void compute(int my_rank,int comm_sz,int n,double a,double b,double h){
	int local_n;
	double local_a;
	double local_b;
	double local_int;
	double total_int;
	int source;
    h= (b-a)/n;
	if((n%comm_sz)!=0)
		local_n = (n/comm_sz)+1;
	else
		local_n = n/comm_sz;
	local_a   = a + my_rank*local_n*h;
	local_b   = local_a + local_n*h;
	if(local_b > b)
		local_int = trapeze(local_a, b ,local_n, h);	
	else
		local_int = trapeze(local_a,local_b,local_n, h);
    
	if(my_rank!=0){
		printf("Proceso = %d \n", my_rank);
		MPI_Send(&local_int,1,MPI_DOUBLE,0,0,MPI_COMM_WORLD);
	}else{
		total_int = local_int;
		for(source=1;source<comm_sz;source++){
			MPI_Recv(&local_int,1,MPI_DOUBLE,source,0,MPI_COMM_WORLD,MPI_STATUS_IGNORE);
			printf("Proceso %d recibido, con %.5e de valor \n", source,local_int);
			total_int += local_int;
			printf("%d",total_int);
 		}
	}
	if(my_rank == 0){
		printf("con n = %d trapazoides, el aproximado es \n", n);
		printf("de la integral de  %f a %f = %.15e\n", a,b,total_int);
	}	

}

int main(int argc, char const *argv[])
{
  	int my_rank;
	int comm_sz;
	int n = 2024;

	double a=10;
	double b=30;
	double h=10;

	MPI_Init(NULL,NULL);
	MPI_Comm_rank(MPI_COMM_WORLD,&my_rank);
	MPI_Comm_size(MPI_COMM_WORLD,&comm_sz);
    compute(my_rank,comm_sz,n,a,b,h);

	MPI_Finalize();
    return 0;
}


#gcc -g -Wall -o pth_pi pth_pi.c -lpthread
#./pth_pi 2 1000000000 >> file.txt 
# gcc -g -Wall -o pth_pi_busy2 pth_pi_busy2.c -lm -lpthread
# ./pth_pi_busy2 2 100000000
# ./pth_pi_busy2 4 100000000 
# ./pth_pi_busy2 8 100000000
# ./pth_pi_busy2 16 100000000
# ./pth_pi_busy2 32 100000000
# ./pth_pi_busy2 64 100000000
# ./pth_pi_busy2 128 100000000

gcc -g -Wall -o pth_pi_mutex pth_pi_mutex.c -lm -lpthread
./pth_pi_mutex 2 100000000
./pth_pi_mutex 4 100000000
./pth_pi_mutex 8 100000000
./pth_pi_mutex 16 100000000
./pth_pi_mutex 32 100000000
./pth_pi_mutex 64 100000000
./pth_pi_mutex 128 100000000

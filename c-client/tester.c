#include <stdio.h>
#include <stdlib.h>

#include "tcp_client.c"

#define ITER 17

int main(int argc, char *argv[])
{
    int nid = atoi(argv[1]);
    printf("%d", nid);

    long int n = 99999999;
    // monitorInit();

    for (float i = 1; i <= ITER; i++)
    {
        bool is_prime = false;

        while (!is_prime)
        {
            n++;

            for (int j = 2; j < n; j++)
            {
                if (n % j == 0)
                {
                    is_prime = false;
                    break;
                }

                is_prime = true;
            }

            if (is_prime) {
                printf("Sending %7.3f...\n", 100*i/ITER);
                monitorInit(nid);
                monitorSend(100*i/ITER);
                monitorDestroy();
            }
        }

    }

    printf("Ending...\n");
    // monitorDestroy();
    monitorInit(nid);
    monitorSend(-1);
    monitorDestroy();
    printf("Ended.");
}
   
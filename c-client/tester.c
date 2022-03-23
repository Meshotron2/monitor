#include "tcp_client.c"

#define ITER 56

int main(void)
{
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
                monitorInit();
                monitorSend(100*i/ITER);
                monitorDestroy();
            }
        }
    }

    printf("Ending...");
    // monitorDestroy();
    printf("Ended.");
}
   
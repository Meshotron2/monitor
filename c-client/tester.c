#include "tcp_client.c"

int main(void)
{
    long int n = 99999999;

    for (int i = 1; i <= 100; i++)
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
                printf("Sending %d...\n", i);
                send_to_monitor(i);
            }
        }
    }
}
   
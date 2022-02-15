#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <stdbool.h>
#define MAX 9
#define PORT 49152
#define SA struct sockaddr

// https://www.geeksforgeeks.org/tcp-server-client-implementation-in-c/

void fetch_message(char *buff, int percent)
{
    int pid = getpid();
    
    int i = 0;
    for (; i < 5; i++)
    {
        buff[5-i] = '0' + (pid % 10);
        pid /= 10;
    }

    buff[5] = ':';
    i=0;

    for (; i < 3; i++)
    {
        buff[MAX-1-i] = '0' + (percent % 10);
        percent /= 10;
    }

    printf("Message: ");
    for (int j = 0; j < MAX; j++)
        printf("%c", buff[j]);

    printf("\n");
}

int send_to_monitor(int percent)
{
    char buff[MAX];
    fetch_message(buff, percent);

    int sockfd, connfd;
    struct sockaddr_in servaddr, cli;
   
    // socket create and verification
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1) {
        printf("socket creation failed...\n");
        exit(0);
    }
    else
        printf("Socket successfully created..\n");
    bzero(&servaddr, sizeof(servaddr));
   
    // assign IP, PORT
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = inet_addr("127.0.0.1");
    servaddr.sin_port = htons(PORT);
   
    // connect the client socket to server socket
    if (connect(sockfd, (SA*)&servaddr, sizeof(servaddr)) != 0) {
        printf("connection with the server failed...\n");
        exit(0);
    }
    else
        printf("connected to the server..\n");
   
    // function for chat
    //func(sockfd);
    write(sockfd, buff, sizeof(buff));
    bzero(buff, sizeof(buff));
   
    // close the socket
    close(sockfd);
}

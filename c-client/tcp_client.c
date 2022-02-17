#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <stdbool.h>

#define MAX 13
#define PORT 49152
#define SA struct sockaddr

static int sockfd, connfd, pid;
static struct sockaddr_in servaddr, cli;
static char buff[MAX];


// https://www.geeksforgeeks.org/tcp-server-client-implementation-in-c/
void monitorInit()
{  
    pid = getpid();
    
    // socket create and verification
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1) {
        printf("socket creation failed...\n");
        exit(0);
    }
    else
        printf("Socket successfully created..\n");
    memset(&servaddr, 0, sizeof(servaddr));
   
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
}

void monitorDestroy()
{
    close(sockfd);
}

void monitorSend(float percentage)
{    
    //memset(buff, 0, MAX);
    snprintf(buff, MAX+1, "%05d:%7.3f", pid, percentage);
    write(sockfd, buff, MAX);
}
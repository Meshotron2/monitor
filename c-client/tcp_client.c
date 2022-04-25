#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <stdbool.h>

#define MAX 8
#define PORT 49152
#define SA struct sockaddr

static int sockfd, connfd, pid;
static struct sockaddr_in servaddr, cli;
static char buff[MAX];

typedef struct MonitorData
{
    int32_t pid;
    float percentage;
    float t_envio;
    float t_rececao; 
    float t_delay_pass; 
    float t_scatter_pass;
};

void monitorInit(int idfk)
{
    pid = idfk;
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

// https://www.geeksforgeeks.org/tcp-server-client-implementation-in-c/
void monitorInitPID()
{  
    pid = getpid();
    monitorInit(pid);
}

void monitorDestroy()
{
    close(sockfd);
}

// void serialize(struct MonitorData *md) 
// {
//     buff[0] = md->pid >> 24;
//     buff[1] = md->pid >> 16;
//     buff[2] = md->pid >> 8;
//     buff[3] = md->pid;
    
//     buff[4] = md->percentage >> 24;
//     buff[5] = md->percentage >> 16;
//     buff[6] = md->percentage >> 8;
//     buff[7] = md->percentage;
// }

void monitorSend(float percentage)
{    
    //memset(buff, 0, MAX);
    // snprintf(buff, MAX+1, "%05d:%7.3f", pid, percentage);
    struct MonitorData md = {pid, percentage, 1, 2, 3, 4};
    // serialize(&md);
    write(sockfd, &md, sizeof(struct MonitorData));
}
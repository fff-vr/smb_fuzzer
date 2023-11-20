#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <pthread.h>

#define PORT1 8080
#define PORT2 8082
#define BUFFER_SIZE 1024

int client_socket_8082 = -1; // Socket for the client on port 8082
pthread_mutex_t client_mutex = PTHREAD_MUTEX_INITIALIZER;

char buffer[BUFFER_SIZE];
unsigned int buffer_size ; 
void *handle_relay(void *arg);
void *handle_client_8082(void *arg);
int setup_server_socket(int port);
void send_to_client_8082(char *msg, int len);

int main() {
    int server_fd1, server_fd2;
    pthread_t relay_thread, client_thread_8082;

    // Setup server sockets
    server_fd1 = setup_server_socket(PORT1);
    server_fd2 = setup_server_socket(PORT2);

    // Start a thread to handle the relay of data from port 8080
    if (pthread_create(&relay_thread, NULL, handle_relay, (void *)(intptr_t)server_fd1) != 0) {
        perror("Failed to create relay thread");
        return 1;
    }

    // Start a thread to handle a single client connection on port 8082
    if (pthread_create(&client_thread_8082, NULL, handle_client_8082, (void *)(intptr_t)server_fd2) != 0) {
        perror("Failed to create client thread for port 8082");
        return 1;
    }

    pthread_join(relay_thread, NULL);
    pthread_join(client_thread_8082, NULL);

    return 0;
}

int setup_server_socket(int port) {
    int server_fd, opt = 1;
    struct sockaddr_in address;
    int addrlen = sizeof(address);

    // Create socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("Socket failed");
        exit(EXIT_FAILURE);
    }

    // Attach socket to the port
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt))) {
        perror("setsockopt");
        exit(EXIT_FAILURE);
    }

    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(port);

    // Bind the socket
    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("Bind failed");
        exit(EXIT_FAILURE);
    }

    // Listen
    if (listen(server_fd, 3) < 0) {
        perror("Listen");
        exit(EXIT_FAILURE);
    }

    return server_fd;
}

void *handle_relay(void *arg) {
    //8080
    int server_fd = (intptr_t)arg;
    int new_socket = accept(server_fd, NULL, NULL);
    if (new_socket < 0) {
        perror("Accept failed");
        exit(EXIT_FAILURE);
    }

    while (1) {

        ssize_t bytes_read = read(new_socket, &buffer_size, 4);
        if (bytes_read != 4 || buffer_size > 0x100000) {
            perror("Read failed");
            break;
        }
        bytes_read = read(new_socket, buffer, buffer_size);
        if (bytes_read <= 0) {
            perror("Read failed");
            break;
        }

        pthread_mutex_lock(&client_mutex);
        send_to_client_8082(buffer, bytes_read);
        pthread_mutex_unlock(&client_mutex);
    }

    close(new_socket);
    return NULL;
}

void *handle_client_8082(void *arg) {
    int server_fd = (intptr_t)arg;
    while (1) {
        if (client_socket_8082 == -1) { // Check if no client is connected
            int new_socket = accept(server_fd, NULL, NULL);
            if (new_socket < 0) {
                perror("Accept failed");
                continue;
            }

            pthread_mutex_lock(&client_mutex);
            client_socket_8082 = new_socket; // Update the client socket
            pthread_mutex_unlock(&client_mutex);
        }

        // Wait for the client to disconnect
        char buffer[BUFFER_SIZE];
        if (recv(client_socket_8082, buffer, BUFFER_SIZE, 0) <= 0) {
            close(client_socket_8082);
            client_socket_8082 = -1; // Reset the client socket
        }
    }

    return NULL;
}

void send_to_client_8082(char *msg, int len) {
    if (client_socket_8082 != -1) {
        send(client_socket_8082,&buffer_size,4,0)
        send(client_socket_8082, msg, len, 0);
    }
}


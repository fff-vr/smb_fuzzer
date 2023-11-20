#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT1 8080
#define PORT2 8082
#define BUFFER_SIZE 1024 * 20

char buffer[BUFFER_SIZE] = {0};
int main() {
    int server_fd, new_socket, sending_socket;
    struct sockaddr_in server_addr, client_addr, dest_addr;
    int opt = 1;
    int addrlen = sizeof(server_addr);

    // Creating socket file descriptor
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("Socket failed");
        exit(EXIT_FAILURE);
    }

    // Forcefully attaching socket to the port 8080
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt))) {
        perror("setsockopt");
        exit(EXIT_FAILURE);
    }

    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT1);

    // Bind the socket to the port 8080
    if (bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("Bind failed");
        exit(EXIT_FAILURE);
    }

    // Listen for incoming connections
    if (listen(server_fd, 3) < 0) {
        perror("Listen");
        exit(EXIT_FAILURE);
    }

    while (1) {
        printf("Waiting for new connection...\n");

        // Accept a connection
        if ((new_socket = accept(server_fd, (struct sockaddr *)&client_addr, (socklen_t*)&addrlen)) < 0) {
            perror("Accept");
            continue; // Continue to next iteration on error
        }

        ssize_t total_bytes_read = 0;
        ssize_t bytes_read;
        while ((bytes_read = read(new_socket, buffer + total_bytes_read, BUFFER_SIZE - total_bytes_read)) > 0) {
            total_bytes_read += bytes_read;
            if (total_bytes_read >= BUFFER_SIZE) break;
        }

        if (bytes_read < 0) {
            perror("Read error");
            close(new_socket);
            continue;
        }

        printf("Received data: %s\n", buffer);

        // Creating a socket for sending data
        if ((sending_socket = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
            perror("Socket creation error");
            close(new_socket);
            continue; // Continue to next iteration on error
        }

        dest_addr.sin_family = AF_INET;
        dest_addr.sin_port = htons(PORT2);

        // Convert IPv4 addresses from text to binary form
        if (inet_pton(AF_INET, "127.0.0.1", &dest_addr.sin_addr) <= 0) {
            perror("Invalid address/ Address not supported");
            close(sending_socket);
            close(new_socket);
            continue; // Continue to next iteration on error
        }

        // Connect to the server on port 8082
        if (connect(sending_socket, (struct sockaddr *)&dest_addr, sizeof(dest_addr)) < 0) {
            perror("Connection Failed");
            close(sending_socket);
            close(new_socket);
            continue; // Continue to next iteration on error
        }

        ssize_t total_bytes_sent = 0;
        ssize_t bytes_sent;
        while (total_bytes_sent < total_bytes_read) {
            bytes_sent = send(sending_socket, buffer + total_bytes_sent, total_bytes_read - total_bytes_sent, 0);
            if (bytes_sent < 0) {
                perror("Send error");
                break;
            }
            total_bytes_sent += bytes_sent;
        }

        if (bytes_sent >= 0) {
            printf("Data forwarded to port 8082\n");
        }

        close(new_socket);
        close(sending_socket);
    }

    // The following line will never be reached in this example
    // close(server_fd);

    return 0;
}


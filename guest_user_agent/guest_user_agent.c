#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/types.h>
#include <sys/mount.h>
#include <unistd.h>
#include <sys/types.h> 
#include <sys/socket.h>
#include <netinet/in.h>
#define KCOV_INIT_TRACE                     _IOR('c', 1, unsigned long)
#define KCOV_ENABLE                 _IO('c', 100)
#define KCOV_DISABLE                        _IO('c', 101)
#define COVER_SIZE                  (64<<10)

#define KCOV_TRACE_PC  0
#define KCOV_TRACE_CMP 1


void mount_cifs(){

    const char* source = "//127.0.0.1/data"; // SMB 공유 경로
	const char* target = "/root/smb_fuzzer/guest_user_agent/tmp"; // 마운트 포인트
	const char* filesystemtype = "cifs";
	unsigned long mountflags = NULL;
	const char* data = "username=data,password=data"; // 사용자 이름과 비밀번호
    if (mount(source, target, filesystemtype, mountflags, data) != 0) {
        //fprintf(stderr, "Error mounting cifs filesystem: %s\n", strerror(errno));
        //TODO if refuse -> retry
        return -1;
    }
    umount("//127.0.0.1/data");
}
void start_coverage(int fd,unsigned long * cover){
 /* Mmap buffer shared between kernel- and user-space. */
    /* Enable coverage collection on the current thread. */
    if (ioctl(fd, KCOV_ENABLE, KCOV_TRACE_PC))
            perror("ioctl"), exit(1);
    /* Reset coverage from the tail of the ioctl() call. */
    __atomic_store_n(&cover[0], 0, __ATOMIC_RELAXED);
    /* Call the target syscall call. */
}

void end_coverage(int fd, unsigned long * cover, int master){
     /* Read number of PCs collected. */
    int n = __atomic_load_n(&cover[0], __ATOMIC_RELAXED);
    if (ioctl(fd, KCOV_DISABLE, 0))
            perror("ioctl"), exit(1);
    unsigned long *current_ptr = cover;
    ssize_t total_written = 0;
    ssize_t to_write = n * 8;
    while (total_written < to_write) {
        ssize_t written = write(master, current_ptr, to_write - total_written);

        if (written == -1) {
            perror("write");
            // Decide how to handle the error. You might want to retry, or exit, etc.
            // For now, let's just exit.
            exit(1);
        }

        total_written += written;
        current_ptr += written / sizeof(unsigned long);
    }
    return;
}
int accept_fuzzer_master(){
    int sockfd, newsockfd, portno;
    socklen_t clilen;
    struct sockaddr_in serv_addr, cli_addr;
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) 
       abort();

    bzero((char *) &serv_addr, sizeof(serv_addr));
    portno = 8081;

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = INADDR_ANY;
    serv_addr.sin_port = htons(portno);

    if (bind(sockfd, (struct sockaddr *) &serv_addr, sizeof(serv_addr)) < 0) 
             error("ERROR on binding");

    listen(sockfd, 5);
    clilen = sizeof(cli_addr);

    newsockfd = accept(sockfd, (struct sockaddr *) &cli_addr, &clilen);
    if (newsockfd < 0) 
          abort();
    return newsockfd;
}
int main(int argc, char **argv)
{
    int fd,master;
    unsigned long *cover, n, i;
    char buffer[0x10];
    /* A single fd descriptor allows coverage collection on a single
     * thread.
     */
    master = accept_fuzzer_master();
    fd = open("/sys/kernel/debug/kcov", O_RDWR);
    if (fd == -1)
            perror("open"), exit(1);
    /* Setup trace mode and trace size. */
    if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
            perror("ioctl"), exit(1);

    cover = (unsigned long*)mmap(NULL, COVER_SIZE * sizeof(unsigned long),PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if ((void*)cover == MAP_FAILED)
            perror("mmap"), exit(1);
    while(1){
        usleep(50000);
        start_coverage(fd,cover);
        int ret =0;

        int retry =0;
        while(ret != 1){
            ret = read(master,buffer,1);
            if(ret!=1){
                printf("wait for recv command from Master. %d\n",retry++);
                sleep(1);
            }
        }
        //more command for status? 
        mount_cifs();
        end_coverage(fd,cover,master);
    }
       /* Free resources. */
    if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
            perror("munmap"), exit(1);
    if (close(fd))
            perror("close"), exit(1);
    return 0;
}

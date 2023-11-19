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
#define KCOV_INIT_TRACE                     _IOR('c', 1, unsigned long)
#define KCOV_ENABLE                 _IO('c', 100)
#define KCOV_DISABLE                        _IO('c', 101)
#define COVER_SIZE                  (64<<10)

#define KCOV_TRACE_PC  0
#define KCOV_TRACE_CMP 1


void mount_cifs(){

    const char* source = "//127.0.0.1/data"; // SMB 공유 경로
	const char* target = "/mnt"; // 마운트 포인트
	const char* filesystemtype = "cifs";
	unsigned long mountflags = MS_MGC_VAL;
	const char* data = "username=data,password=data,vers=1.0"; // 사용자 이름과 비밀번호
    if (mount(source, target, filesystemtype, mountflags, data) != 0) {
        //fprintf(stderr, "Error mounting cifs filesystem: %s\n", strerror(errno));
        //TODO if refuse -> retry
        return -1;
    }
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

void end_coverage(int fd, unsigned long * cover){
     /* Read number of PCs collected. */
    int n = __atomic_load_n(&cover[0], __ATOMIC_RELAXED);
    for (int i = 0; i < n; i++)
            printf("0x%lx\n", cover[i + 1]);
    /* Disable coverage collection for the current thread. After this call
     * coverage can be enabled for a different thread.
     */
    if (ioctl(fd, KCOV_DISABLE, 0))
            perror("ioctl"), exit(1);


}
int main(int argc, char **argv)
{
    int fd;
    unsigned long *cover, n, i;

    /* A single fd descriptor allows coverage collection on a single
     * thread.
     */
    fd = open("/sys/kernel/debug/kcov", O_RDWR);
    if (fd == -1)
            perror("open"), exit(1);
    /* Setup trace mode and trace size. */
    if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
            perror("ioctl"), exit(1);

    cover = (unsigned long*)mmap(NULL, COVER_SIZE * sizeof(unsigned long),PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if ((void*)cover == MAP_FAILED)
            perror("mmap"), exit(1);
    start_coverage(fd,cover);    
    mount_cifs();
    end_coverage(fd,cover);
       /* Free resources. */
    if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
            perror("munmap"), exit(1);
    if (close(fd))
            perror("close"), exit(1);
    return 0;
}
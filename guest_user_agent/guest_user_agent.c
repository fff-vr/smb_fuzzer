#define _GNU_SOURCE
#include "file_operation.h"
#include <dirent.h>
#include <fcntl.h>
#include <linux/types.h>
#include <netinet/in.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/mount.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#define KCOV_INIT_TRACE _IOR('c', 1, unsigned long)
#define KCOV_ENABLE _IO('c', 100)
#define KCOV_DISABLE _IO('c', 101)
#define COVER_SIZE (64 << 10)

#define KCOV_TRACE_PC 0
#define KCOV_TRACE_CMP 1

#define MAX_BUF 256

int is_process_dir(const struct dirent *dir) {
  // Check if directory name is a number (process ID)
  for (const char *p = dir->d_name; *p; p++) {
    if (*p < '0' || *p > '9')
      return 0;
  }
  return 1;
}

int check_thread_name(const char *path, const char *thread_name) {
  FILE *fp = fopen(path, "r");
  if (!fp)
    return 0;

  char comm[MAX_BUF];
  int found = 0;
  if (fgets(comm, sizeof(comm), fp) && strstr(comm, thread_name)) {
    found = 1; // Thread found
  }
  fclose(fp);
  return found;
}

int check_thread_exists(const char *thread_name) {
  DIR *d = opendir("/proc");
  if (!d)
    return 0;

  struct dirent *dir;
  while ((dir = readdir(d)) != NULL) {
    if (dir->d_type == DT_DIR && is_process_dir(dir)) {
      char task_path[MAX_BUF];
      snprintf(task_path, sizeof(task_path), "/proc/%s/task", dir->d_name);

      DIR *td = opendir(task_path);
      if (td) {
        struct dirent *tdir;
        while ((tdir = readdir(td)) != NULL) {
          if (tdir->d_type == DT_DIR) {
            char comm_path[MAX_BUF];
            snprintf(comm_path, sizeof(comm_path), "%s/%s/comm", task_path,
                     tdir->d_name);

            if (check_thread_name(comm_path, thread_name)) {
              closedir(td);
              closedir(d);
              return 1;
            }
          }
        }
        closedir(td);
      }
    }
  }
  closedir(d);
  return 0;
}

<<<<<<< HEAD
int mount_cifs(int proxy_port){

    const char* source = "//10.0.2.10/data"; // SMB 공유 경로
	const char* target = "/root/smb_fuzzer/guest_user_agent/tmp"; // 마운트 포인트
	const char* filesystemtype = "cifs";
	unsigned long mountflags = 0;
	char data[0x1000]; 
    sprintf(data,"username=data,password=data,vers=3.0,sync,port=%d,soft", proxy_port); // 사용자 이름과 비밀번호
    if (mount(source, target, filesystemtype, mountflags, data) != 0) {
        return -1;
    }
    return 0;
=======
int mount_cifs(int proxy_port, char *id, char *pass) {

  const char *target = "/root/smb_fuzzer/guest_user_agent/tmp"; // 마운트 포인트
  const char *filesystemtype = "cifs";
  unsigned long mountflags = 0;
  char data[0x100];
  char mount_point[0x100];
  sprintf(mount_point,"//10.0.2.10/%s",id);
  sprintf(data, "username=%s,password=%s,vers=3.0,sync,port=%d", id, pass,
          proxy_port); // 사용자 이름과 비밀번호
  if (mount(source, target, filesystemtype, mountflags, data) != 0) {
    return -1;
  }
  return 0;
>>>>>>> 74d40b7bf7892da16d061e5fe54cdd644beafe07
}
void start_coverage(int fd, unsigned long *cover) {
  /* Mmap buffer shared between kernel- and user-space. */
  /* Enable coverage collection on the current thread. */
  if (ioctl(fd, KCOV_ENABLE, KCOV_TRACE_PC))
    perror("ioctl"), exit(1);
  /* Reset coverage from the tail of the ioctl() call. */
  __atomic_store_n(&cover[0], 0, __ATOMIC_RELAXED);
  /* Call the target syscall call. */
}

void end_coverage(int fd, unsigned long *cover, int master) {
  /* Read number of PCs collected. */
  int n = __atomic_load_n(&cover[0], __ATOMIC_RELAXED);
  if (ioctl(fd, KCOV_DISABLE, 0))
    perror("ioctl"), exit(1);
  unsigned long *current_ptr = cover;
  ssize_t total_written = 0;
  ssize_t to_write = n * 8;
  printf("coverage len = %d\n", n);
  if (to_write == 0) {
    unsigned long tmp_buf = 0;
    write(master, &tmp_buf, 8);
    return;
  }
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
int accept_fuzzer_master(int master_port) {

  int sock;
  struct sockaddr_in serv_addr;

  // 소켓 생성
  sock = socket(PF_INET, SOCK_STREAM, 0);
  if (sock == -1) {
    perror("socket() error");
    exit(1);
  }

  // 서버 주소 및 포트 설정
  memset(&serv_addr, 0, sizeof(serv_addr));
  serv_addr.sin_family = AF_INET;
  serv_addr.sin_addr.s_addr = inet_addr("10.0.2.10");
  serv_addr.sin_port = htons(master_port);
  while (1) {
    usleep(10000);
    if (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) != -1) {
      return sock;
    }
  }
}
int main(int argc, char **argv) {
  int fd, master;
  unsigned long *cover, n, i;
  char buffer[0x10];
  /* A single fd descriptor allows coverage collection on a single
   * thread.
   */
  master = accept_fuzzer_master(atoi(argv[1]));
  fd = open("/sys/kernel/debug/kcov", O_RDWR);
  if (fd == -1)
    perror("open"), exit(1);
  /* Setup trace mode and trace size. */
  if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
    perror("ioctl"), exit(1);

  cover = (unsigned long *)mmap(NULL, COVER_SIZE * sizeof(unsigned long),
                                PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
  if ((void *)cover == MAP_FAILED)
    perror("mmap"), exit(1);
  while (1) {
    while (1) {
      if (check_thread_exists("cifsd") == 0) {
        break;
      }
      usleep(10000);
    }
    start_coverage(fd, cover);
    int ret = 0;
    ret = read(master, buffer, 1);
    if (ret == -1) {
      exit(1);
    }

    if (mount_cifs(atoi(argv[2]), argv[3], argv[4]) == 0) {
      switch (buffer[0]) {
      case 1:
        file_operation1("/root/smb_fuzzer/guest_user_agent/tmp");
        break;
      case 2:
        file_operation2("/root/smb_fuzzer/guest_user_agent/tmp");
        break;
      case 3:
        file_operation3("/root/smb_fuzzer/guest_user_agent/tmp");
        break;
      }
    }
    umount("/root/smb_fuzzer/guest_user_agent/tmp");
    end_coverage(fd, cover, master);
  }
  /* Free resources. */
  if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
    perror("munmap"), exit(1);
  if (close(fd))
    perror("close"), exit(1);
  return 0;
}

#define _GNU_SOURCE
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/uio.h>
#include <sys/vfs.h>
#include <unistd.h>
int file_operation1(char *path) {
  char link_path[0x300];
  char symlink_path[0x300];
  char file_path[0x300];

  sprintf(file_path, "%s/test", path);
  sprintf(link_path, "%s/test_link", path);
  sprintf(symlink_path, "%s/test_symlink", path);

  int fd = open(file_path, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
  if (fd == -1) {
    perror("open");
    return 1;
  }

  const char *msg = "Hello, world!";
  if (write(fd, msg, strlen(msg)) == -1) {
    perror("write");
  }

  if (fchmod(fd, S_IRUSR | S_IWUSR | S_IRGRP) == -1) {
    perror("fchmod");
  }

  if (lseek(fd, 3, SEEK_SET) == -1) {
    perror("lseek");
  }

  char buffer[20];
  if (read(fd, buffer, sizeof(buffer)) == -1) {
    perror("read");
  }

  struct stat statbuf;
  if (fstat(fd, &statbuf) == -1) {
    perror("fstat");
  }

  umask(S_IWGRP | S_IWOTH);

  if (fchown(fd, getuid(), getgid()) == -1) {
    perror("fchown");
  }

  if (close(fd) == -1) {
    perror("close");
  }

  if (access(file_path, R_OK | W_OK) == -1) {
    perror("access");
  }

  if (link(file_path, link_path) == -1) {
    perror("link");
  }

  if (lstat(link_path, &statbuf) == -1) {
    perror("lstat");
  }

  return 0;
}

void file_operation2(const char *path) {
  char linkPath[1024];
  char folderPath[1024];
  char filePath[1024];
  char newfilePath[1024];
  char symlinkPath[1024];
  char tmpPath[1024];
  sprintf(folderPath, "%s/testdir", path);
  sprintf(filePath, "%s/testfile", path);
  sprintf(newfilePath, "%s/testfile_new", path);
  sprintf(linkPath, "%s/testfile_link", path);
  sprintf(symlinkPath, "%s/testfile_symlink", path);

  printf("folderPath = %s\n", folderPath);
  printf("filePath = %s\n", filePath);
  printf("newfilePath = %s\n", newfilePath);
  printf("linkPath = %s\n", linkPath);
  printf("symlinkPath = %s\n", symlinkPath);

  if (mkdir(folderPath, 0755) == -1) {
    perror("mkdir");
  }
  int fd = open(filePath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);

  if (link(filePath, linkPath) == -1) {
    perror("link");
  }

  if (symlink(filePath, symlinkPath) == -1) {
    perror("symlink");
  }

  ssize_t len = readlink(symlinkPath, tmpPath, sizeof(tmpPath) - 1);
  if (len == -1) {
    perror("readlink");
  }

  if (rename(filePath, newfilePath) == -1) {
    perror("rename");
  }

  if (unlink(linkPath) == -1) {
    perror("unlink");
  }
  if (unlink(symlinkPath) == -1) {
    perror("unlink");
  }
  if (rmdir(folderPath) == -1) {
    perror("rmdir");
  }
}

void file_operation3(const char *path) {
  char file_path[0x300];
  sprintf(file_path, "%s/test.txt", path);
  int fd = open(file_path, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
  if (fd == -1) {
    perror("open");
    return;
  }

  if (ftruncate(fd, 1024) == -1) {
    perror("ftruncate");
  }

  if (posix_fallocate(fd, 0, 1024) != 0) {
    perror("posix_fallocate");
  }

  void *map = mmap(NULL, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
  if (map == MAP_FAILED) {
    perror("mmap");
  }

  if (msync(map, 1024, MS_SYNC) == -1) {
    perror("msync");
  }

  if (mlock(map, 1024) == -1) {
    perror("mlock");
  }

  if (munlock(map, 1024) == -1) {
    perror("munlock");
  }

  if (munmap(map, 1024) == -1) {
    perror("munmap");
  }

  if (fsync(fd) == -1) {
    perror("fsync");
  }
  struct statfs buf;
  if (fstatfs(fd, &buf) == -1) {
    perror("fstatfs");
  }
  struct statx statxbuf;
  if (statx(fd, "", AT_EMPTY_PATH, STATX_ALL, &statxbuf) == -1) {
    perror("statx");
  }
  if (close(fd) == -1) {
    perror("close");
    return;
  }
}

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
  // 파일 생성 및 열기
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

  // 파일에 쓰기
  const char *msg = "Hello, world!";
  if (write(fd, msg, strlen(msg)) == -1) {
    perror("write");
  }

  // 파일 속성 변경
  if (fchmod(fd, S_IRUSR | S_IWUSR | S_IRGRP) == -1) {
    perror("fchmod");
  }

  // 파일 위치 변경
  if (lseek(fd, 3, SEEK_SET) == -1) {
    perror("lseek");
  }

  // 파일 읽기
  char buffer[20];
  if (read(fd, buffer, sizeof(buffer)) == -1) {
    perror("read");
  }

  // 파일 상태 조회
  struct stat statbuf;
  if (fstat(fd, &statbuf) == -1) {
    perror("fstat");
  }

  // umask 설정
  umask(S_IWGRP | S_IWOTH);

  // 파일 소유자 변경
  if (fchown(fd, getuid(), getgid()) == -1) {
    perror("fchown");
  }

  // 파일 닫기
  if (close(fd) == -1) {
    perror("close");
  }

  // 파일 접근 권한 확인
  if (access(file_path, R_OK | W_OK) == -1) {
    perror("access");
  }

  // 파일 링크 생성
  if (link(file_path, link_path) == -1) {
    perror("link");
  }

  // 파일 상태 조회 (심볼릭 링크)
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

  // 새 디렉토리 생성
  if (mkdir(folderPath, 0755) == -1) {
    perror("mkdir");
  }
  int fd = open(filePath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);

  // 하드 링크 생성
  if (link(filePath, linkPath) == -1) {
    perror("link");
  }

  // 심볼릭 링크 생성
  if (symlink(filePath, symlinkPath) == -1) {
    perror("symlink");
  }

  // 심볼릭 링크 읽기
  ssize_t len = readlink(symlinkPath, tmpPath, sizeof(tmpPath) - 1);
  if (len == -1) {
    perror("readlink");
  }

  // 파일 이름 변경
  if (rename(filePath, newfilePath) == -1) {
    perror("rename");
  }

  // 디렉토리 및 링크 제거
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
  // 파일 열기
  char file_path[0x300];
  sprintf(file_path, "%s/test.txt", path);
  int fd = open(file_path, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
  if (fd == -1) {
    perror("open");
    return;
  }

  // 파일 크기 조정
  if (ftruncate(fd, 1024) == -1) {
    perror("ftruncate");
  }

  // 공간 할당
  if (posix_fallocate(fd, 0, 1024) != 0) {
    perror("posix_fallocate");
  }

  // 메모리 매핑
  void *map = mmap(NULL, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
  if (map == MAP_FAILED) {
    perror("mmap");
  }

  // 메모리 동기화
  if (msync(map, 1024, MS_SYNC) == -1) {
    perror("msync");
  }

  // 메모리 잠금
  if (mlock(map, 1024) == -1) {
    perror("mlock");
  }

  // 메모리 잠금 해제
  if (munlock(map, 1024) == -1) {
    perror("munlock");
  }

  // 메모리 언매핑
  if (munmap(map, 1024) == -1) {
    perror("munmap");
  }

  // 파일 동기화
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
  // 파일 닫기
  if (close(fd) == -1) {
    perror("close");
    return;
  }
}

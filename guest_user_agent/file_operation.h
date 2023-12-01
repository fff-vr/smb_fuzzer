#define _GNU_SOURCE
#include <sys/vfs.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/uio.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
int file_operation1(char* path) {
    // 파일 생성 및 열기
    char link_path [0x300];
    char symlink_path [0x300];
    sprintf(link_path,"%s_link",path);
    sprintf(symlink_path,"%s_symlink",path);

    int fd = open(path, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
    if (fd == -1) {
        perror("open");
        return 1;
    }

    // 파일에 쓰기
    const char *msg = "Hello, world!";
    if (write(fd, msg, strlen(msg)) == -1) {
        perror("write");
        close(fd);
        return 1;
    }

    // 파일 속성 변경
    if (fchmod(fd, S_IRUSR | S_IWUSR | S_IRGRP) == -1) {
        perror("fchmod");
        close(fd);
        return 1;
    }

    // 파일 위치 변경
    if (lseek(fd, 0, SEEK_SET) == -1) {
        perror("lseek");
        close(fd);
        return 1;
    }

    // 파일 읽기
    char buffer[20];
    if (read(fd, buffer, sizeof(buffer)) == -1) {
        perror("read");
        close(fd);
        return 1;
    }

    // 파일 상태 조회
    struct stat statbuf;
    if (fstat(fd, &statbuf) == -1) {
        perror("fstat");
        close(fd);
        return 1;
    }

    // umask 설정
    umask(S_IWGRP | S_IWOTH);

    // 파일 소유자 변경
    if (fchown(fd, getuid(), getgid()) == -1) {
        perror("fchown");
        close(fd);
        return 1;
    }

    // 파일 닫기
    if (close(fd) == -1) {
        perror("close");
        return 1;
    }

    // 파일 접근 권한 확인
    if (access(path, R_OK | W_OK) == -1) {
        perror("access");
        return 1;
    }

    // 파일 링크 생성
    if (link(path, link_path) == -1) {
        perror("link");
        return 1;
    }

    // 파일 상태 조회 (심볼릭 링크)
    if (lstat(link_path, &statbuf) == -1) {
        perror("lstat");
        return 1;
    }

    // 심볼릭 링크 생성
    if (symlink(path, symlink_path) == -1) {
        perror("symlink");
        return 1;
    }

    // 파일 상태 조회 (원본 파일)
    if (stat(symlink_path, &statbuf) == -1) {
        perror("stat");
        return 1;
    }

    // 파일 링크 삭제
    if (unlink(link_path) == -1) {
        perror("unlink");
        return 1;
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
    sprintf(folderPath,"%s/testdir", path);
    sprintf(filePath,"%s/testdir/testfile", path);
    sprintf(newfilePath,"%s/testdir/testfile_new", path);
    sprintf(linkPath,"%s/testdir/testfile_link", path);
    sprintf(symlinkPath,"%s/testdir/testfile_symlink", path);
    // 새 디렉토리 생성
    if (mkdir(folderPath, 0755) == -1) {
        perror("mkdir");
        return;
    }

    // 파일 생성
    int fd = creat(filePath, 0644);
    if (fd == -1) {
        perror("creat");
        return;
    }
    close(fd);

    // 하드 링크 생성
    if (link(filePath, linkPath) == -1) {
        perror("link");
        return;
    }

    // 심볼릭 링크 생성
    if (symlink(filePath, symlinkPath) == -1) {
        perror("symlink");
        return;
    }

    // 심볼릭 링크 읽기
    ssize_t len = readlink(symlinkPath, tmpPath, sizeof(tmpPath) - 1);
    if (len == -1) {
        perror("readlink");
        return;
    }

    // 파일 이름 변경
    if (rename(filePath, newfilePath) == -1) {
        perror("rename");
        return;
    }

    // 디렉토리 및 링크 제거
    if (unlink(linkPath) == -1) {
        perror("unlink");
        return;
    }
    if (unlink(symlinkPath) == -1) {
        perror("unlink");
        return;
    }
    if (rmdir(folderPath) == -1) {
        perror("rmdir");
        return;
    }
}

void file_operation3(const char *path) {
    // 파일 열기
    int fd = open(path, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
    if (fd == -1) {
        perror("open");
        return;
    }

    // 파일 크기 조정
    if (ftruncate(fd, 1024) == -1) {
        perror("ftruncate");
        return;
    }

    // 공간 할당
    if (posix_fallocate(fd, 0, 1024) != 0) {
        perror("posix_fallocate");
        return;
    }

    // 메모리 매핑
    void *map = mmap(NULL, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (map == MAP_FAILED) {
        perror("mmap");
        return;
    }

    // 메모리 동기화
    if (msync(map, 1024, MS_SYNC) == -1) {
        perror("msync");
        return;
    }

    // 메모리 잠금
    if (mlock(map, 1024) == -1) {
        perror("mlock");
        return;
    }

    // 메모리 잠금 해제
    if (munlock(map, 1024) == -1) {
        perror("munlock");
        return;
    }

    // 메모리 언매핑
    if (munmap(map, 1024) == -1) {
        perror("munmap");
        return;
    }

    // 파일 동기화
    if (fsync(fd) == -1) {
        perror("fsync");
        return;
    }
    struct statfs buf;
    if (fstatfs(fd, &buf) == -1) {
        perror("fstatfs");
        close(fd);
        return;
    }
    struct statx statxbuf;
    if (statx(fd, "", AT_EMPTY_PATH, STATX_ALL, &statxbuf) == -1) {
        perror("statx");
        close(fd);
        return ;
    }
    // 파일 닫기
    if (close(fd) == -1) {
        perror("close");
        return;
    }
}



#!/bin/bash

# Samba 공유 폴더의 기본 경로 설정
base_dir="/samba/users"

# Samba 설정 파일
smb_conf="/etc/samba/smb.conf"

# 사용자 생성 및 설정
for i in {1..32}
do
    username="user$i"

    # 사용자 계정 생성
    sudo adduser --disabled-password --gecos "" $username

    # Samba 비밀번호 설정 (여기서는 사용자 이름과 동일하게 설정)
    echo -e "$username\n$username" | sudo smbpasswd -a $username

    # 공유 폴더 생성 및 권한 설정
    user_dir="$base_dir/$username"
    sudo mkdir -p $user_dir
    sudo chown $username:$username $user_dir
    sudo chmod 700 $user_dir

    # Samba 설정에 공유 추가
    echo "[$username]" | sudo tee -a $smb_conf
    echo "   path = $user_dir" | sudo tee -a $smb_conf
    echo "   browseable = no" | sudo tee -a $smb_conf
    echo "   read only = no" | sudo tee -a $smb_conf
    echo "   valid users = $username" | sudo tee -a $smb_conf
    echo "   root preexec = rm -rf $user_dir/* ; mkdir -p $user_dir" | sudo tee -a $smb_conf
    echo "" | sudo tee -a $smb_conf
done

# Samba 서비스 재시작
sudo systemctl restart smbd


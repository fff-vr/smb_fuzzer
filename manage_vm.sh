#!/bin/bash

# 입력된 회수만큼 반복
for i in $(seq 1 $1); do
    echo "copy vm"
    # bullseye.img를 bullseye$i.img로 복사
    cp vm/bullseye.img vm/bullseye$i.img

    # 마운트할 디렉토리 생성
    sudo mkdir -p /mnt/bullseye$i

    # bullseye$i.img를 마운트
    sudo mount vm/bullseye$i.img /mnt/bullseye$i
    abc_value=$((12345 + i*2))
    def_value=$((12346 + i*2))
    # run.sh 파일에 'ABC'와 'DEF'를 $i로 대체
    sudo mkdir -p /mnt/bullseye$i/root/smb_fuzzer/guest_user_agent/tmp
    sudo sed -i "s/AGENT_PORT/$abc_value/g; s/PROXY_PORT/$def_value/g" /mnt/bullseye$i//etc/systemd/system/agent.service

    # bullseye$i.img 언마운트
    sudo umount /mnt/bullseye$i

    # 마운트 디렉토리 삭제 (선택 사항)
    sudo rm -rf /mnt/bullseye$i
done


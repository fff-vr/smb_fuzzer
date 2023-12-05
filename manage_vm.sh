#!/bin/bash

for i in $(seq 1 $1); do
    echo "copy vm"
    cp vm/bullseye.img vm/bullseye$i.img

    sudo mkdir -p /mnt/bullseye$i

    sudo mount vm/bullseye$i.img /mnt/bullseye$i
    abc_value=$((12345 + i*2))
    def_value=$((12346 + i*2))
    USER_ID="user$i"
    sudo mkdir -p /mnt/bullseye$i/root/smb_fuzzer/guest_user_agent/tmp
    sudo sed -i "s/SAMBA_ID/$USER_ID/g ; s/SAMBA_PASS/$USER_ID/g ; s/AGENT_PORT/$abc_value/g; s/PROXY_PORT/$def_value/g" /mnt/bullseye$i//etc/systemd/system/agent.service

    sudo umount /mnt/bullseye$i

    sudo rm -rf /mnt/bullseye$i
done


#!/bin/bash

base_dir="/samba/users"

smb_conf="/etc/samba/smb.conf"

for i in {1..32}
do
    username="user$i"

    sudo adduser --disabled-password --gecos "" $username

    echo -e "$username\n$username" | sudo smbpasswd -a $username

    user_dir="$base_dir/$username"
    sudo mkdir -p $user_dir
    sudo chown $username:$username $user_dir
    sudo chmod 700 $user_dir

    echo "[$username]" | sudo tee -a $smb_conf
    echo "   path = $user_dir" | sudo tee -a $smb_conf
    echo "   browseable = no" | sudo tee -a $smb_conf
    echo "   read only = no" | sudo tee -a $smb_conf
    echo "   valid users = $username" | sudo tee -a $smb_conf
    echo "   root preexec = rm -rf $user_dir/* ; mkdir -p $user_dir" | sudo tee -a $smb_conf
    echo "" | sudo tee -a $smb_conf
done

sudo systemctl restart smbd


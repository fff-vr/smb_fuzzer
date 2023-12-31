KERNEL=/home/$USER/target/linux_ori/
IMAGE=./tools
qemu-system-x86_64 \
	-m 4G \
	-smp 2,sockets=2,cores=1 \
	-kernel $KERNEL/arch/x86/boot/bzImage \
	-append "console=ttyS0 root=/dev/sda earlyprintk=serial net.ifnames=0" \
	-drive file=./vm/bullseye$1.img \
	-net user,host=10.0.2.10,hostfwd=tcp:0.0.0.0:10021-:22,hostfwd=tcp:0.0.0.0:10022-:8080,hostfwd=tcp:0.0.0.0:10023-:8081 \
	-net nic,model=virtio \
	-enable-kvm \
	-nographic -s

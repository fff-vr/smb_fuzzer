use crate::config;
use std::process::Stdio;
use tokio::process::Child;
use tokio::process::Command;
pub async fn execute_linux_vm(i: u32) -> Child {
    // Create a unique command for each thread
    let vm_path = format!("{}/bullseye{}.img", config::get_vm_path(), i);
    let child = Command::new("/usr/bin/qemu-system-x86_64")
        .arg("-m")
        .arg(config::get_ram())
        .arg("-smp")
        .arg("1,sockets=1,cores=1")
        .arg("-kernel")
        .arg(config::get_kernel_path())
        .arg("-append")
        .arg("console=ttyS0 root=/dev/sda earlyprintk=serial net.ifnames=0")
        .arg("-drive")
        .arg(format!("file={}", vm_path))
        .arg("-net")
        .arg("user,host=10.0.2.10")
        .arg("-net")
        .arg("nic,model=virtio")
        .arg("-enable-kvm")
        .arg("-nographic")
        .arg("-serial")
        .arg(format!("file:../workdir/test{}.txt", i))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("fail to spawn qemu");
    child
}

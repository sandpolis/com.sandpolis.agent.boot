
source "qemu" "bootagent" {
  communicator         = "ssh"
  ssh_username         = "root"
  ssh_password         = "packer"
  disk_size            = "2034123776b"
  format               = "qcow2"
  boot_command         = [
                            "root<enter>",
                            "echo 'root:packer' | chpasswd<enter><wait>",
                            "apk add openssh<enter><wait>",
                            "echo 'PermitRootLogin yes' >>/etc/ssh/sshd_config<enter>",
                            "echo 'auto eth0' >>/etc/network/interfaces<enter>",
                            "echo 'iface eth0 inet dhcp' >>/etc/network/interfaces<enter>",
                            "/etc/init.d/networking restart<enter>", 
                            "service sshd start<enter><wait>"
                          ]
  boot_wait            = "10s"
  qemuargs             = [["-smp", "4,sockets=1,cores=4,threads=1"], ["-m", "4096"], ["-bios", "/usr/share/ovmf/x64/OVMF.fd"]]
  disk_interface       = "ide"
  headless             = false
  iso_checksum         = "sha256:4bf69f1d96384bd88574e6c64583f40d3c6ae07af4c96772900492ba0f0b9126"
  iso_url              = "https://dl-cdn.alpinelinux.org/alpine/v3.14/releases/x86_64/alpine-standard-3.14.0-x86_64.iso"
  shutdown_command     = ""
  net_device           = "rtl8139"
}

build {
  sources = ["source.qemu.bootagent"]

  provisioner "shell" {
    script = "install.sh"
  }

  post-processor "shell-local" {
    inline = [
      "qemu-img convert -f qcow2 -O raw output-bootagent/packer-bootagent output-bootagent/bootagent.img"
    ]
  }
}

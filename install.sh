#!/bin/ash
## Install Alpine and all boot agent dependencies

# Skip password configuration
sed -i 's/passwd/true/g' /sbin/setup-alpine

(
	export KEYMAPOPTS="us us"
	export HOSTNAMEOPTS="-n s7s-boot-agent"
	export DNSOPTS="-d localhost 8.8.8.8"
	export TIMEZONEOPTS="-z UTC"
	export PROXYOPTS="none"
	export APKREPOSOPTS="-r"
	export SSHDOPTS="-c none"
	export NTPOPTS="-c none"
	export BOOT_SIZE="10"
	export DISKOPTS="-s 0 -m sys /dev/sda"
	export INTERFACESOPTS="auto lo
iface lo inet loopback"

	yes | setup-alpine
)

# Remount root filesystem
mount /dev/sda2 /mnt

# Install ncurses
(
	apk add ncurses-dev
	cp /usr/lib/libncursesw.so /mnt/usr/lib/libncursesw.so
)

# Install yaft framebuffer terminal
(
	export LANG=en_US.UTF-8

	apk add git make gcc musl-dev linux-headers
	git clone https://github.com/uobikiemukot/yaft

	cd yaft
	make
	cp yaft /mnt/usr/bin
)

# Install boot agent (actually just a specially built micro agent)
(
	apk add wget

	wget 'https://maven.pkg.github.com/sandpolis/com.sandpolis.agent.micro/com.sandpolis.agent.micro/0.1.0/bootagent-0.1.0' /mnt/usr/bin/bootagent
)

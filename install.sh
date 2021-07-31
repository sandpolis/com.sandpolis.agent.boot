#!/bin/ash

export KEYMAPOPTS="us us"
export HOSTNAMEOPTS="-n s7s-boot-agent"
export DNSOPTS="-n 8.8.8.8"
export TIMEZONEOPTS="-z UTC"
export PROXYOPTS="none"
export APKREPOSOPTS="-r"
export SSHDOPTS="-c none"
export NTPOPTS="-c none"
export BOOT_SIZE="10"
export DISKOPTS="-s 0 -m sys /dev/sda"
export INTERFACESOPTS="auto lo
iface lo inet loopback
"

yes | setup-alpine

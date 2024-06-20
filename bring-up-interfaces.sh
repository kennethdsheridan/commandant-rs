#!/bin/sh

sudo ip link set ens11np0 up
sudo ip addr add 192.168.11.1/24 dev ens11np0

sudo ip link set ens12np0 up
sudo ip addr add 192.168.12.1/24 dev ens12np0

sudo ip link set ens21np0 up
sudo ip addr add 192.168.21.1/24 dev ens21np0

sudo ip link set ens22np0 up
sudo ip addr add 192.168.22.1/24 dev ens22np0

sudo ip link set ens31np0 up
sudo ip addr add 192.168.31.1/24 dev ens31np0

sudo ip link set ens32np0 up
sudo ip addr add 192.168.32.1/24 dev ens32np0

sudo ip link set ens41np0 up
sudo ip addr add 192.168.41.1/24 dev ens41np0

sudo ip link set ens42np0 up
sudo ip addr add 192.168.42.1/24 dev ens42np0

sudo ip link set ens50f0 up
sudo ip addr add 192.168.50.1/24 dev ens50f0
sudo ip link set ens50f1 up
sudo ip addr add 192.168.51.1/24 dev ens50f1



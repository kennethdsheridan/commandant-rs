#!/bin/sh
ip link set ens11np0 up
ip addr add 192.168.11.1/24 dev ens11np0
ip link set dev ens11np0 mtu 9100
ip route add default via 192.168.11.254 dev ens11np0 table 101
ip rule add from 192.168.11.1/24 table 101

ip link set ens12np0 up
ip addr add 192.168.12.1/24 dev ens12np0
ip link set dev ens12np0 mtu 9100
ip route add default via 192.168.12.254 dev ens12np0 table 102
ip rule add from 192.168.12.1/24 table 102

ip link set ens21np0 up
ip addr add 192.168.21.1/24 dev ens21np0
ip link set dev ens21np0 mtu 9100
ip route add default via 192.168.21.254 dev ens21np0 table 103
ip rule add from 192.168.21.1/24 table 103

ip link set ens22np0 up
ip addr add 192.168.22.1/24 dev ens22np0
ip link set dev ens22np0 mtu 9100
ip route add default via 192.168.22.254 dev ens22np0 table 104
ip rule add from 192.168.22.1/24 table 104

ip link set ens31np0 up
ip addr add 192.168.31.1/24 dev ens31np0
ip link set dev ens31np0 mtu 9100
ip route add default via 192.168.31.254 dev ens31np0 table 105
ip rule add from 192.168.31.1/24 table 105

ip link set ens32np0 up
ip addr add 192.168.32.1/24 dev ens32np0
ip link set dev ens32np0 mtu 9100
ip route add default via 192.168.32.254 dev ens32np0 table 106
ip rule add from 192.168.32.1/24 table 106

ip link set ens41np0 up
ip addr add 192.168.41.1/24 dev ens41np0
ip link set dev ens41np0 mtu 9100
ip route add default via 192.168.41.254 dev ens41np0 table 107
ip rule add from 192.168.41.1/24 table 107

ip link set ens42np0 up
ip addr add 192.168.42.1/24 dev ens42np0
ip link set dev ens42np0 mtu 9100
ip route add default via 192.168.42.254 dev ens42np0 table 108
ip rule add from 192.168.42.1/24 table 108

ip link set ens50f0 up
ip addr add 192.168.50.1/24 dev ens50f0
ip link set dev ens50f0 mtu 9100
ip route add default via 192.168.50.254 dev ens50f0 table 109
ip rule add from 192.168.50.1/24 table 109

ip link set ens50f1 up
ip addr add 192.168.51.1/24 dev ens50f1
ip link set dev ens50f1 mtu 9100
ip route add default via 192.168.51.254 dev ens50f1 table 110
ip rule add from 192.168.51.1/24 table 110


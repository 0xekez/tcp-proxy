#!/bin/bash

# Script for testing tcp proxy performance over network
# namespaces. Creates three namespaces, left, right, and proxy and
# sets up a client in left, proxy in proxy, and server in right.

#       10.0.0.1              10.0.0.2                 10.0.0.3
#    +------------+        +------------+           +------------+
#    |            |        |            |           |            |
#    |            |        |            |           |            |
#    |   left     |        |  proxy     |           |   right    |
#    |            |        |            |           |            |
#    |            |        |            |           |            |
#    +------------+        +------------+           +------------+

function setup_ns() {
    local nsname=$1
    local pubnet=$2
    local bridge=$3

    ip netns add $nsname

    ip -n $nsname link set dev lo up

    ip link add ${nsname}_outer type veth peer name ${nsname}_inner
    ip link set ${nsname}_inner netns $nsname

    ip link set dev ${nsname}_outer up
    ip link set ${nsname}_outer master $bridge

    ip -n $nsname link set ${nsname}_inner up
    ip -n $nsname addr add $pubnet dev ${nsname}_inner
}

function setup_bridge() {
    local name=$1

    ip link add $name type bridge
    ip link set $name up
}

function setup() {
    setup_bridge br0
    setup_ns left 10.0.0.1/24 br0
    setup_ns proxy 10.0.0.2/24 br0
    setup_ns right 10.0.0.3/24 br0
}

function benchmark() {
    ip netns exec proxy /home/admin/NETWORK/proxy/tcp-proxy/target/debug/proxy -c 0.0.0.0:1212 -s 10.0.0.3:1313 &

    echo "Using the proxy"
    echo "---------------"

    ip netns exec right nc -n -l -s 10.0.0.3 -p 1313 > /dev/null &
    ip netns exec left bash -c 'dd if=/dev/zero bs=1024 count=900000 | nc -q 1 10.0.0.2 1212'

    echo "Direct connection"
    echo "---------------"

    ip netns exec right nc -n -l -s 10.0.0.3 -p 1313 > /dev/null &
    ip netns exec left bash -c 'dd if=/dev/zero bs=1024 count=900000 | nc -q 1 10.0.0.3 1313'
}

function teardown() {
    ip netns del left
    ip netns del right
    ip netns del proxy
    ip link delete dev br0
}

case $1 in
    up)
	setup
	;;
    down)
	teardown
	;;
    bench)
	setup
	benchmark
	teardown
	;;
    *)
	echo "usage: $0 <up/down/bench>"
	;;
esac

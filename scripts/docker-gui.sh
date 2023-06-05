#!/bin/sh

set -eux

IP=$(ifconfig en0 | grep inet | awk '$1=="inet" {print $2}')
XSOCK=/tmp/.X11-unix
#XAUTH=${HOME}/.Xauthority
XAUTH=/tmp/.docker.xauth

xauth nlist $DISPLAY | sed -e 's/^..../ffff/' | xauth -f $XAUTH nmerge -
chmod 777 $XAUTH

docker run \
--platform linux/ppc64le \
--net host \
-e DISPLAY=${IP}:0 \
-e XAUTHORITY=${XAUTH} \
-v ${XSOCK}:${XSOCK} \
-v ${XAUTH}:${XAUTH} \
-it debian \
bash


#!/bin/bash
# update sidecar
# assume istio is installed
SCRIPTDIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
set -x
mkdir -p $SCRIPTDIR/generated
NGINMESH_IMAGE_HUB=${1:-docker.io}
NGX_LOG_LEVEL=${4:-warn}
NGINMESH_VERSION=0.9.0
ISTIO_PROXY_INIT=docker.io/istio/proxy_init:0.8.0
echo "generating sidecar config"
NGINMESH_CONFIG_NAME=nginmesh-sidecar-injector-configmap.yaml
GEN_NGINMESH=$SCRIPTDIR/generated/$NGINMESH_CONFIG_NAME
echo "# GENERATED FILE. Use with Istio 0.6" > $GEN_NGINMESH
cat $SCRIPTDIR/templates/$NGINMESH_CONFIG_NAME.tmpl >> $GEN_NGINMESH
sed -i .bak "s|{NGINMESH_IMAGE_HUB}|${NGINMESH_IMAGE_HUB}|" $GEN_NGINMESH
sed -i .bak "s|{NGX_LOG_LEVEL}|${NGX_LOG_LEVEL}|" $GEN_NGINMESH
sed -i .bak "s|{NGINMESH_VERSION}|${NGINMESH_VERSION}|" $GEN_NGINMESH
sed -i .bak "s|{ISTIO_PROXY_INIT}|${ISTIO_PROXY_INIT}|" $GEN_NGINMESH
#!/bin/bash
# list new message
# assume testclient has been installed
set -x
TOPIC_NAME=$1
CLIENT=testclient
KAFKA_NAME=my-kafka
kubectl -n kafka exec -it  $CLIENT -- /usr/bin/kafka-console-producer \
    --broker-list bootstrap.kafka.svc.cluster.local:9092 --topic $TOPIC_NAME 
 

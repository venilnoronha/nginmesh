#!/bin/bash
# list new message
# assume testclient has been installed
# headless is same as bootstrap
set -x
TOPIC_NAME=$1
CLIENT=testclient
KAFKA_NAME=my-kafka
kubectl -n kafka exec $CLIENT -- /usr/bin/kafka-console-consumer \
    --bootstrap-server bootstrap.kafka.svc.cluster.local:9092 --topic $TOPIC_NAME --from-beginning

#!/bin/bash
# add new topic
# assume testclient has been installed
TOPIC_NAME=$1
PARTIONS=${2:-1}
REPLICATION=${3:-1}
CLIENT=testclient
KAFKA_NAME=my-kafka
kubectl -n kafka exec $CLIENT -- /usr/bin/kafka-topics --zookeeper zookeeper.kafka.svc.cluster.local:2181 \
    --topic $TOPIC_NAME --create --partitions $PARTIONS --replication-factor $REPLICATION

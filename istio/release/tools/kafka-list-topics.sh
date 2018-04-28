#!/bin/bash
# list topic
CLIENT=testclient
KAFKA_NAME=kafka
kubectl -n kafka exec -ti $CLIENT -- /usr/bin/kafka-topics --zookeeper zookeeper.kafka.svc.cluster.local:2181 --list

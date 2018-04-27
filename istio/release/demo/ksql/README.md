# KSQL Demo

## Install

### Install Connect
```
kubectl create -f ../../install/kafka/connectl.yml'
```
This will create Kafka connect and services

### Install KSQL and create SQL STREAM
```
kubectl create -f ../../install/kafka/ksql.yml
```
This will create KSQL server pods

Download Apache Kafaka from https://www.confluent.io/download/.
Update your path to include bin directory of Kafka.

Start KSQL port-forward so that KSQL CLI client connect to server
```
../../install/kafka/ksql-portforward.sh'
```
Start KSQL CLI and run sql script to create nginmesh stream and tables
``
ksql
run script 'create.sql';
```

### Install Elastic Search
```
./install-elastic.sh
```
This will install elastic cluster in the namespace 'elastic'

### Install Grafana
```
./install-grafana.sh
```
This will install grafana in the namespace 'kafka'

``
### Start following port-forwarding, this is required in order to connect kafka to elastic search to grafana
```
./elastic-portforward.sh
./connect-portforward.sh
./grafana-portforward.sh
```

### Connect following tables to Elastic Search and Set up data source to Grafana

```
./ksql-tables-to-grafana.sh request_path_stat
```

### Grafana

Now open grafana at: http://localhost:3000,

Run following script to get password for grafana.  

```
./grafana-password.sh
```

Login in and create dashboard



TBD
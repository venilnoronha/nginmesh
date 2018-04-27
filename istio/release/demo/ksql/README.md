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
### Run following script set up port-forwarding to ksql, connect, elastic search and grafana
```
./all-portforward.sh
```

### For each of the KSQL table, run following script to connect KSQL -> connect -> elastic search -> grafna

```
./ksql-tables-to-grafana.sh request_per_min
./ksql-tables-to-grafana.sh request_per_min_max_avg
./ksql-tables-to-grafana.sh request_activity
```

### Grafana

Now open grafana at: http://localhost:3000,

Run following script to get password for grafana.  

```
./grafana-password.sh
```

Then import dashboard using 'grafana-dashboard.json'



TBD
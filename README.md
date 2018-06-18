# NGINX Architecture with Istio Service Mesh
This repository provides an implementation of a NGINX based service mesh (nginMesh).  nginMesh is compatible with Istio.  It leverages NGINX as a sidecar proxy. 

## What is Service Mesh and Istio?
Please check https://istio.io for a detailed explanation of the service mesh.  

## Production Status
The current version of nginMesh is designed to work with Istio release 0.8.0. It should not be used in production environments.  

## Architecture
The diagram below depicts how an NGINX sidecar proxy is implemented. Sidecar uses the open source version of NGINX compiled with modules for tracing and monitoring.

![Alt text](/images/nginx_sidecar.png?raw=true "NGINX Sidecar")

To learn more about the sidecar implementation, see [this document](istio/agent).

## Quick Start
Below are instructions to quickly install and configure nginMesh.  Currently, only Kubernetes environment is supported.

### Prerequisites
Make sure you have a Kubernetes cluster with at least 1.9 or greater due to fact only automatic sidecar injection is supported and no alpha feature is enabled. Please see [Prerequisites](https://istio.io/docs/setup/kubernetes/quick-start.html) for setting up a kubernetes cluster.

### Installing Istio and nginMesh
nginMesh requires installation of Istio first.

1. Download and install Istio 0.8.0:
```
curl -L https://git.io/getLatestIstio | ISTIO_VERSION=0.8.0 sh -
```
2. Download nginMesh release 0.8.0:
```
curl -L https://github.com/nginmesh/nginmesh/releases/download/0.8.0/nginmesh-0.8.0.tar.gz | tar zx
```

3. Deploy Istio between sidecars:

```
kubectl create -f istio-0.8.0/install/kubernetes/istio-demo.yaml
```


4. Ensure the following Kubernetes services are deployed: istio-pilot, istio-mixer, istio-ingress:
```
kubectl get svc  -n istio-system  
```
```
istio-citadel              ClusterIP      10.27.242.159   <none>           8060/TCP,9093/TCP                          2h
istio-egressgateway        ClusterIP      10.27.249.220   <none>           80/TCP,443/TCP                             2h
istio-ingressgateway       LoadBalancer   10.27.252.17    35.232.83.174    80:31380/TCP,443:31390/TCP,31400:31400/TCP 1h
istio-pilot                ClusterIP      10.27.243.203   <none>           15003/TCP,15005/TCP,15007/TCP,15010/TCP,15011/TCP,8080/TCP,9093/TCP   2h
istio-policy               ClusterIP      10.27.243.198   <none>           9091/TCP,15004/TCP,9093/TCP                2h
istio-sidecar-injector     ClusterIP      10.27.254.74    <none>           443/TCP                                    2h
istio-statsd-prom-bridge   ClusterIP      10.27.246.55    <none>           9102/TCP,9125/UDP                          2h
istio-telemetry            ClusterIP      10.27.246.232   <none>           9091/TCP,15004/TCP,9093/TCP,42422/TCP      2h
prometheus                 ClusterIP      10.27.250.35    <none>           9090/TCP                                   2h
servicegraph               ClusterIP      10.27.240.49    <none>           8088/TCP                                   2h
tracing                    LoadBalancer   10.27.255.156   35.224.195.224   80:32664/TCP                               2h
zipkin                     ClusterIP      10.27.240.80    <none>           9411/TCP                                   2h
grafana                    ClusterIP      10.27.253.24    <none>           3000/TCP                                   2h
```

5. Ensure the following Kubernetes pods are up and running: istio-pilot-* , istio-mixer-* , istio-ingress-*  and istio-initializer-* :
```
kubectl get pods -n istio-system    
```
```
istio-citadel-7bdc7775c7-jgjwb             1/1       Running   0          2h
istio-egressgateway-795fc9b47-zhvd4        1/1       Running   0          2h
istio-ingressgateway-7d89dbf85f-vgxgg      1/1       Running   0          2h
istio-pilot-66f4dd866c-6xhkw               2/2       Running   0          2h
istio-policy-76c8896799-glzhc              2/2       Running   0          2h
istio-sidecar-injector-645c89bc64-kzpsh    1/1       Running   0          1h
istio-statsd-prom-bridge-949999c4c-qmwlr   1/1       Running   0          2h
istio-telemetry-6554768879-mzzq4           2/2       Running   0          2h
istio-tracing-754cdfd695-pz755             1/1       Running   0          2h
prometheus-86cb6dd77c-lq22z                1/1       Running   0          2h
servicegraph-5849b7d696-s574j              1/1       Running   2          2h
grafana-6f6dff9986-68p7c                   1/1       Running   0          2h
```


6. Automatic sidecar:
To set up sidecar injection, please run following script which will install Istio webhook with nginMesh customization.
```
nginmesh-0.8.0/install/kubernetes/install-sidecar.sh
```

7. Verify that istio-injection label is not labeled for the default namespace :
```
kubectl get namespace -L istio-injection
```
```
NAME           STATUS        AGE       ISTIO-INJECTION
default        Active        1h        
istio-system   Active        1h        
kube-public    Active        1h        
kube-system    Active        1h
```


### Deploy a Sample Application
In this section we deploy the Bookinfo application, which is taken from the Istio samples. Please see [Bookinfo](https://istio.io/docs/guides/bookinfo.html)  for more details.

1. Label the default namespace with istio-injection=enabled:

```
kubectl label namespace default istio-injection=enabled
```

2. Deploy the application:

```
kubectl apply -f  istio-0.8.0/samples/bookinfo/kube/bookinfo.yaml
```
3. Apply ingress rules for the application:

```
kubectl apply -f  istio-0.8.0/samples/bookinfo/kube/bookinfo-gateway.yaml
```


4. Confirm that all application services are deployed: productpage, details, reviews, ratings:

```
kubectl get services
```
```
NAME                       CLUSTER-IP   EXTERNAL-IP   PORT(S)              AGE
details                    10.0.0.31    <none>        9080/TCP             6m
kubernetes                 10.0.0.1     <none>        443/TCP              7d
productpage                10.0.0.120   <none>        9080/TCP             6m
ratings                    10.0.0.15    <none>        9080/TCP             6m
reviews                    10.0.0.170   <none>        9080/TCP             6m
```

5. Confirm that all application pods are running --details-v1-* , productpage-v1-* , ratings-v1-* , reviews-v1-* , reviews-v2-* and reviews-v3-* :
```
kubectl get pods
```
```
NAME                                        READY     STATUS    RESTARTS   AGE
details-v1-1520924117-48z17                 2/2       Running   0          6m
productpage-v1-560495357-jk1lz              2/2       Running   0          6m
ratings-v1-734492171-rnr5l                  2/2       Running   0          6m
reviews-v1-874083890-f0qf0                  2/2       Running   0          6m
reviews-v2-1343845940-b34q5                 2/2       Running   0          6m
reviews-v3-1813607990-8ch52                 2/2       Running   0          6m
```

6. Get the public IP of the Istio Ingress controller. If the cluster is running in an environment that supports external load balancers:

```
kubectl get svc -n istio-system | grep -E 'EXTERNAL-IP|istio-ingress'
```

OR

```
kubectl get ingress -o wide       
```

7. Open the Bookinfo application in a browser using the following link:
```
http://<Public-IP-of-the-Ingress-Controller>/productpage
```

Note: For E2E routing rules and performace testing you could refer to [E2E Test](istio/tests/README.md).


### Uninstalling the Application
1. To uninstall application, run:

```
./istio-0.8.0/samples/bookinfo/kube/cleanup.sh
```

### Uninstalling Istio
1. To uninstall the Istio core components:

```
kubectl delete -f istio-0.8.0/install/kubernetes/istio-demo.yaml
```


2. To uninstall the initializer, run:

```
nginmesh-0.8.0/install/kubernetes/delete-sidecar.sh
```

## Limitations
nginMesh has the following limitations:
* TCP and gRPC traffic is not supported.
* Quota Check is not supported.
* Only Kubernetes is supported.

All sidecar-related limitations and supported traffic management rules are described [here](istio/agent).

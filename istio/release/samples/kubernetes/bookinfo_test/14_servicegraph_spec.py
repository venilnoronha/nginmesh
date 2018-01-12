import requests
import subprocess
import time
import configuration
import performance
from mamba import description, context, it
from expects import expect, be_true, have_length, equal, be_a, have_property, be_none

headers = {'content-type': 'application/json','accept': 'application/json'}
with description('nginmesh Test 14'):
    with before.all:
         #Read Config file
         configuration.setenv(self)

    with context('Set environment'):
         with it('Add Servicegraph feature'):
            subprocess.call("kubectl apply -f ../istio-"+self.VERSION+"/install/kubernetes/addons/prometheus.yaml > /dev/null 2>&1 | exit 0",universal_newlines=True,shell=True)
            subprocess.call("kubectl apply -f ../istio-"+self.VERSION+"/install/kubernetes/addons/servicegraph.yaml > /dev/null 2>&1 | exit 0",universal_newlines=True,shell=True)
            time.sleep(5)
            subprocess.call("kubectl -n istio-system port-forward $(kubectl -n istio-system get pod -l app=prometheus -o jsonpath='{.items[0].metadata.name}') 9090:9090 & > /dev/null 2>&1 | exit 0",universal_newlines=True,shell=True)
            subprocess.call("kubectl -n istio-system port-forward $(kubectl -n istio-system get pod -l app=servicegraph -o jsonpath='{.items[0].metadata.name}') 8088:8088 & > /dev/null 2>&1 | exit 0",universal_newlines=True,shell=True)
            time.sleep(5)


    with context('Starting Test'):
        with it('Bookinfo Servicegraph feature'):
            for _ in range(5):
                r = requests.get(self.url)
                r.status_code
                expect(r.status_code).to(equal(200))
            r1=requests.get(self.servicegraph)
            r1.status_code
            expect(r1.status_code).to(equal(200))
            if 'productpage' in r1.text:
                expect(0).to(equal(0))
            else:
                expect(0).not_to(equal(0))
            if self.performance=='on':
                print performance.wrecker(self.GATEWAY_URL)
            else:
                pass
    with context('Clean Environment'):
        with it('Delete Servicegraph feature'):
            subprocess.call(["kubectl delete -f ../istio-"+self.VERSION+"/install/kubernetes/addons/prometheus.yaml > /dev/null 2>&1 | exit 0"],universal_newlines=True,shell=True)
            subprocess.call(["kubectl delete -f ../istio-"+self.VERSION+"/install/kubernetes/addons/servicegraph.yaml > /dev/null 2>&1 | exit 0"],universal_newlines=True,shell=True)
            subprocess.call("ps -ef | grep prometheus | grep -v grep | awk '{print $2}' | xargs kill -9 > /dev/null 2>&1",universal_newlines=True,shell=True)
            subprocess.call("ps -ef | grep servicegraph | grep -v grep | awk '{print $2}' | xargs kill -9 > /dev/null 2>&1",universal_newlines=True,shell=True)








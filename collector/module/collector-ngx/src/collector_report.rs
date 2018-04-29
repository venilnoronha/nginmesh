
use std::sync::Mutex;
use std::collections::HashMap;
use std::mem;
use std::ptr;
use std::os::raw::c_void;
use std::ffi::CString;


use ngx_rust::bindings::*;

use nginmesh_collector_transport::attribute::attr_wrapper::AttributeWrapper;
use nginmesh_collector_transport::attribute::global_dict::{ RESPONSE_DURATION };

use ngx::main_config::ngx_http_collector_main_conf_t;
use ngx::server_config::ngx_http_collector_srv_conf_t;
use ngx::location_config::ngx_http_collector_loc_conf_t;
use ngx::config::CollectorConfig;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::fmt::Write;
use std::time::Duration;


// initialize channel that can be shared
/*
lazy_static! {
    static ref CHANNELS: Channels<MixerInfo> = {
        let (tx, rx) = channel();

        Channels {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }
    };
}
*/

lazy_static!  {
    static ref PRODUCER_CACHE: Mutex<HashMap<String,Producer>> = Mutex::new(HashMap::new());
}



#[no_mangle]
pub  extern "C" fn ngx_http_collector_create_loc_conf(cf: &ngx_conf_s)  -> *mut c_void {

    ngx_event_debug!("create collector loc conf");
    unsafe {
        ngx_pcalloc((*cf).pool,mem::size_of::<ngx_http_collector_loc_conf_t>())
    }  
}


#[no_mangle]
pub  extern "C" fn ngx_http_collector_create_srv_conf(cf: &ngx_conf_s) -> *mut ngx_http_collector_srv_conf_t {

    ngx_event_debug!("create collector srv conf");

    unsafe {
        let srv_conf_ptr = ngx_pcalloc((*cf).pool,mem::size_of::<ngx_http_collector_srv_conf_t>()) as *mut ngx_http_collector_srv_conf_t;
         if srv_conf_ptr.is_null() {
            return srv_conf_ptr;
        }
        let srv_conf: &mut ngx_http_collector_srv_conf_t = &mut * srv_conf_ptr;
        srv_conf.init();
        return srv_conf_ptr;
    } 
}



macro_rules! ngx_conf_merge_str_value {
    ( $conf:expr,$prev:expr,$default:expr ) => (
        
        if $conf.data.is_null() {                                                 
            if !$prev.data.is_null()  {                                                    
                $conf.len = $prev.len;                                            
                $conf.data = $prev.data;                                          
            } else {
                let c_string = CString::new($default).unwrap();                                                             
                $conf.len = $default.len() ;                         
                $conf.data = c_string.into_raw() as *mut u_char;                             
            }                                               
        }
        
    )
}

macro_rules! NGX_CONF_OK {
    () =>  ( ptr::null() )
}
 
#[no_mangle] 
pub extern "C" fn ngx_http_collector_merge_loc_conf(_cf: &mut ngx_conf_s,
        parent: &mut ngx_http_collector_loc_conf_t, 
        child: &mut ngx_http_collector_loc_conf_t) -> *const u_char {
 
    ngx_event_debug!("rust: merging collector loc conf");
    ngx_conf_merge_str_value!(child.topic,parent.topic, "");
    ngx_conf_merge_str_value!(child.destination_service,parent.destination_service,"");
    NGX_CONF_OK!() 
}


// send to background thread using channels
#[no_mangle]
pub extern fn nginmesh_set_collector_server_config(server: &ngx_str_t)  {

    let server_name = server.to_str();
    ngx_event_debug!("set collector server config: {}",server_name);

    let new_producer = Producer::from_hosts(vec!(server_name.to_owned()))
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create();

    if new_producer.is_err() {
        ngx_event_debug!("server not founded: {}",server_name);
        return
    } 
             
    PRODUCER_CACHE.lock().unwrap().insert(server_name.to_owned(),new_producer.unwrap());
    
    ngx_event_debug!("add to server cache")

}

fn send_stat(message: &str,server_name: &str,topic: &str) {

    // test comment
    let mut cache = PRODUCER_CACHE.lock().unwrap();
    let producer_result = cache.get_mut(server_name);
    if producer_result.is_none()  {
         ngx_event_debug!("server: {} is not founded",server_name);
         return 
    }
    let mut buf = String::with_capacity(2);
    let _ = write!(&mut buf, "{}", message); 
    let producer = producer_result.unwrap();
    producer.send(&Record::from_value(topic, buf.as_bytes())).unwrap();
    ngx_event_debug!("send event to kafka topic: {}",topic);

}


/*
pub fn collector_report_background()  {

    let rx = CHANNELS.rx.lock().unwrap();
    let mut producer: Producer  = Producer::from_hosts(vec!("broker.kafka:9092".to_owned()))
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create()
                .unwrap();


    loop {
        ngx_event_debug!("mixer report  thread waiting");
        let info = rx.recv().unwrap();
        ngx_event_debug!("mixer report thread woke up");

        
        let mut buf = String::with_capacity(2);
        let _ = write!(&mut buf, "{}", info.attributes); 
        producer.send(&Record::from_value("test", buf.as_bytes())).unwrap();
        ngx_event_debug!("send event to kafka topic test");

        ngx_event_debug!("mixer report thread: finished sending to kafka");
    }
}
*/


// Total Upstream response Time Calculation Function Start

fn upstream_response_time_calculation( upstream_states: *const ngx_array_t ) -> i64 {

    unsafe {

        let upstream_value = *upstream_states;
        let upstream_response_time_list = upstream_value.elts;
        let upstream_response_time_n = upstream_value.nelts as isize;
        let upstream_response_time_size = upstream_value.size as isize;
        let mut upstream_response_time_total:i64 = 0;
        for i in 0..upstream_response_time_n as isize {

            let upstream_response_time_ptr = upstream_response_time_list.offset(i*upstream_response_time_size) as *mut ngx_http_upstream_state_t;
            let upstream_response_time_value = (*upstream_response_time_ptr).response_time as i64;
            upstream_response_time_total = upstream_response_time_total + upstream_response_time_value;

        }

        return upstream_response_time_total;
    }
}


#[no_mangle]
pub extern fn nginmesh_collector_report_handler(request: &ngx_http_request_s,
    main_config: &ngx_http_collector_main_conf_t,
    srv_conf: &ngx_http_collector_srv_conf_t,
    loc_conf: &ngx_http_collector_loc_conf_t)  {

    let topic = loc_conf.topic.to_str();
    let server_name = main_config.collector_server.to_str();
    let mut attr = AttributeWrapper::new();
    srv_conf.process_istio_attr(&mut attr);
    request.process_istio_attr(&mut attr);
    attr.insert_int64_attribute(RESPONSE_DURATION, upstream_response_time_calculation(request.upstream_states));
    
    let headers_out =  &request.headers_out;
    headers_out.process_istio_attr(&mut attr);

    send_stat(&attr.to_string(),&server_name,&topic);
    ngx_http_debug!(request,"finish sending to kafer");

}




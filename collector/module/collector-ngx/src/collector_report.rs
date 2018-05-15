
use std::sync::Mutex;
use std::collections::HashMap;
use std::mem;
use std::ptr;
use std::os::raw::c_void;
use std::ffi::CString;
use std::ffi::CStr;
use std::slice;


use ngx_rust::bindings::*;

use nginmesh_collector_transport::attribute::attr_wrapper::AttributeWrapper;
use nginmesh_collector_transport::attribute::global_dict::{ RESPONSE_DURATION };

use ngx::main_config::ngx_http_collector_main_conf_t;
use ngx::server_config::ngx_http_collector_srv_conf_t;
use ngx::location_config::ngx_http_collector_loc_conf_t;
use ngx::config::CollectorConfig;
use ngx::config::ServerConfig;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::fmt::Write;
use std::time::Duration;

lazy_static!  {
    static ref PRODUCER_CACHE: Mutex<HashMap<String,Producer>> = Mutex::new(HashMap::new());
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

#[no_mangle]
pub extern "C" fn ngx_http_collector_merge_srv_conf(_cf: &ngx_conf_s,
        parent: &mut ngx_http_collector_srv_conf_t,
        child:  &mut ngx_http_collector_srv_conf_t) -> *const u_char {

    ngx_event_debug!("merging srv conf");
  
    ngx_conf_merge_str_value!(child.destination_service,parent.destination_service,"");
    ngx_conf_merge_str_value!(child.destination_uid,parent.destination_uid,"");
    ngx_conf_merge_str_value!(child.source_ip,parent.source_ip,"");
    ngx_conf_merge_str_value!(child.source_uid,parent.source_uid,"");
    ngx_conf_merge_str_value!(child.source_service,parent.source_service,"");
    ngx_conf_merge_uint_value!(child.source_port, parent.source_port, 0);

    NGX_CONF_OK!()
}

#[no_mangle]
pub  extern "C" fn ngx_http_collector_create_loc_conf(cf: &ngx_conf_s)  -> *mut c_void {

    ngx_event_debug!("create collector loc conf");
    unsafe {
        ngx_pcalloc((*cf).pool,mem::size_of::<ngx_http_collector_loc_conf_t>())
    }  
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

#[no_mangle] 
pub extern "C" fn ngx_http_collector_create_main_conf(cf: &ngx_conf_s) -> *mut c_void {

    ngx_event_debug!("setting up main config");

    unsafe {
        ngx_pcalloc((*cf).pool,mem::size_of::<ngx_http_collector_main_conf_t>())
    }  
}




extern "C" {

    #[link_name = "ngx_http_collector_module"]
    static mut NGX_HTTP_COLLECTOR_MODULE: ngx_module_s;
}


/**
 * collector report handler.
 *
 */
#[no_mangle]
pub extern "C" fn  ngx_http_collector_report_handler(request: &ngx_http_request_s) -> ngx_int_t {

   // ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "start invoking collector report handler");
    ngx_http_debug!(request,"start report handler");
   /*
    let ctx_ptr: *const c_void = ptr::null();
    let commands_ptr: *const ngx_command_t = ptr::null();
    let COLLECTOR_MODULE = ngx_module_s {
        ctx_index: NGX_MODULE_UNSET_INDEX!(),
        index: NGX_MODULE_UNSET_INDEX!(),                         
        name: CString::new("test").unwrap().into_raw(),
        spare0: 0,
        spare1: 0, 
        version: nginx_version as usize, 
        signature: NGX_NULL_PTR!(),
        ctx: ctx_ptr as *mut c_void,
        commands: commands_ptr as *mut ngx_command_t,
        type_: NGX_HTTP_MODULE as usize,
        init_master: None,
        init_module: None,
        init_process: None,
        init_thread: None,
        exit_thread: None,
        exit_process: None,
        exit_master: None,
        spare_hook0: 0usize,
        spare_hook1: 0usize,
        spare_hook2: 0usize,
        spare_hook3: 0usize,
        spare_hook4: 0usize,
        spare_hook5: 0usize,
        spare_hook6: 0usize,
        spare_hook7: 0usize
    };
    */

    unsafe {
        
        let c_str: &CStr = CStr::from_ptr(NGX_HTTP_COLLECTOR_MODULE.name);
        let str_slice: &str = c_str.to_str().unwrap();
        ngx_http_debug!(request,"collector module name: {}",str_slice);
        let index = NGX_HTTP_COLLECTOR_MODULE.ctx_index;
         ngx_http_debug!(request,"collector module index: {}",index);
         /*
        let ptr = request.loc_conf.offset(index as isize);
        if ptr.is_null() {
              ngx_http_debug!(request,"loc  conf is null");
        } */
        let loc_conf_ptr = ngx_http_get_module_loc_conf!(request, NGX_HTTP_COLLECTOR_MODULE);
        let loc_conf: &mut ngx_http_collector_loc_conf_t = &mut * ( loc_conf_ptr as *mut ngx_http_collector_loc_conf_t);
        let src_conf_ptr = ngx_http_get_module_srv_conf!(request,NGX_HTTP_COLLECTOR_MODULE);
        let srv_conf: &mut ngx_http_collector_srv_conf_t = &mut * (src_conf_ptr as *mut ngx_http_collector_srv_conf_t);
        let main_conf_ptr = ngx_http_get_module_main_conf!(request, NGX_HTTP_COLLECTOR_MODULE);
        let main_conf: &mut ngx_http_collector_main_conf_t = &mut * (main_conf_ptr as *mut ngx_http_collector_main_conf_t);
    
    //    ngx_log_debug2(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "using collector server: %*s",main_conf->collector_server.len,main_conf->collector_server.data);

        ngx_http_debug!(request,"invoking nginmesh report handler");
        // invoke mix client
        nginmesh_collector_report_handler(request,main_conf,srv_conf,loc_conf);

        //ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "finish calling collector report handler");
        NGX_OK!()

    }
    

} 


// install log phase handler for collector

#[no_mangle]
pub extern "C" fn ngx_http_collector_filter_init(cf: &ngx_conf_s) -> ngx_int_t {

    unsafe {
        let cmcf: &mut ngx_http_core_main_conf_t = &mut * (ngx_http_conf_get_module_main_conf!(cf, ngx_http_core_module) as *mut ngx_http_core_main_conf_t);
        let h1 = ngx_array_push(&mut cmcf.phases[ngx_http_phases::NGX_HTTP_LOG_PHASE as usize].handlers);
        if h1.is_null() {
            return NGX_ERROR!();
         }

        let fn_ptrz =  ngx_http_collector_report_handler as *const fn() -> () as usize;
        
        *(h1 as *mut usize) = *(&fn_ptrz as *const usize);

        NGX_OK!() 
    
    }
    
}

// set loca conf topic
#[no_mangle]
pub extern "C" fn  collector_conf_set_topic(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        loc_conf: &mut ngx_http_collector_loc_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let topic = value_array[1];
        loc_conf.topic.len = topic.len;
        loc_conf.topic.data = topic.data;
    }

    NGX_CONF_OK!()
}

// set main conf server address
#[no_mangle]
pub extern "C" fn  collector_conf_set_server(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        main_conf: &mut ngx_http_collector_main_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        main_conf.collector_server.len = value.len;
        main_conf.collector_server.data = value.data;
        nginmesh_set_collector_server_config(&main_conf.collector_server);
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_destination_service(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.destination_service.len = value.len;
        srv_conf.destination_service.data = value.data;
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_destination_uid(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.destination_uid.len = value.len;
        srv_conf.destination_uid.data = value.data;
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_destination_ip(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.destination_ip.len = value.len;
        srv_conf.destination_ip.data = value.data;
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_source_ip(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.source_ip.len = value.len;
        srv_conf.source_ip.data = value.data;
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_source_uid(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.source_uid.len = value.len;
        srv_conf.source_uid.data = value.data;
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_source_service(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.source_service.len = value.len;
        srv_conf.source_service.data = value.data;
    }

    NGX_CONF_OK!()
}

#[no_mangle]
pub extern "C" fn  collector_conf_set_source_port(cf: &ngx_conf_s, 
        cmd: &ngx_command_s, 
        srv_conf: &mut ngx_http_collector_srv_conf_t) -> *const char {

    unsafe {
        let args: &ngx_array_t = & *cf.args;
        let value_array: &[ngx_str_t] = slice::from_raw_parts(args.elts as *const ngx_str_t,args.nelts);
        let value = value_array[1];
        srv_conf.source_port = ngx_atoi(value.data, value.len) as ngx_uint_t;
    }

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
    ngx_http_debug!(request,"using topic: {}",topic);
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




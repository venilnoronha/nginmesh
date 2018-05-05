
use std::mem;
use std::ptr;
use ngx_rust::bindings::*;
use nginmesh_collector_transport::attribute::attr_wrapper::AttributeWrapper;

#[macro_export]
macro_rules! NGX_CONF_UNSET_UINT {
    () =>  ( !0 )
}

#[macro_export]
macro_rules! NGX_MODULE_UNSET_INDEX {
    () =>  ( !0 )
}


#[macro_export]
macro_rules! NGX_OK_PTR {
    () =>  ( ptr::null() )
}

#[macro_export]
macro_rules! NGX_NULL_PTR {
    () =>  ( ptr::null() )
}


#[macro_export]
macro_rules! NGX_OK {
    () =>  ( 0 )
}

#[macro_export]
macro_rules! NGX_ERROR {
    () =>  ( -1 )
}      


#[macro_export]
macro_rules! NGX_CONF_OK {
    () =>  ( ptr::null() )
}

/*
macro_rules! NGX_MODULE_SIGNATURE {
    () => (

         NGX_MODULE_SIGNATURE_0 NGX_MODULE_SIGNATURE_1 NGX_MODULE_SIGNATURE_2      \
    NGX_MODULE_SIGNATURE_3 NGX_MODULE_SIGNATURE_4 NGX_MODULE_SIGNATURE_5      \
    NGX_MODULE_SIGNATURE_6 NGX_MODULE_SIGNATURE_7 NGX_MODULE_SIGNATURE_8      \
    NGX_MODULE_SIGNATURE_9 NGX_MODULE_SIGNATURE_10 NGX_MODULE_SIGNATURE_11    \
    NGX_MODULE_SIGNATURE_12 NGX_MODULE_SIGNATURE_13 NGX_MODULE_SIGNATURE_14   \
    NGX_MODULE_SIGNATURE_15 NGX_MODULE_SIGNATURE_16 NGX_MODULE_SIGNATURE_17   \
    NGX_MODULE_SIGNATURE_18 NGX_MODULE_SIGNATURE_19 NGX_MODULE_SIGNATURE_20   \
    NGX_MODULE_SIGNATURE_21 NGX_MODULE_SIGNATURE_22 NGX_MODULE_SIGNATURE_23   \
    NGX_MODULE_SIGNATURE_24 NGX_MODULE_SIGNATURE_25 NGX_MODULE_SIGNATURE_26   \
    NGX_MODULE_SIGNATURE_27 NGX_MODULE_SIGNATURE_28 NGX_MODULE_SIGNATURE_29   \
    NGX_MODULE_SIGNATURE_30 NGX_MODULE_SIGNATURE_31 NGX_MODULE_SIGNATURE_32   \
    NGX_MODULE_SIGNATURE_33 NGX_MODULE_SIGNATURE_34


    )
}
*/
   



#[macro_export]
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

#[macro_export]
macro_rules! ngx_conf_merge_uint_value {
    ( $conf:expr,$prev:expr,$default:expr ) => (
        if $conf == NGX_CONF_UNSET_UINT!()  {
            if $prev == NGX_CONF_UNSET_UINT!() {
                $conf = $default;
            } else {
                $conf = $prev;
            }                                  
        }
    )
}

#[macro_export]
macro_rules! ngx_http_conf_get_module_main_conf {
    ( $cf:expr,$module:expr) =>  ( 
        {
            let ctx: &ngx_http_conf_ctx_t = & *($cf.ctx as *mut ngx_http_conf_ctx_t) ;
            ctx.main_conf.offset($module.ctx_index as isize)           
        }
        
    )
}

#[macro_export]
macro_rules! ngx_http_get_module_srv_conf {
    ( $r:expr,$module:expr) =>  ( {
        let ptr = $r.srv_conf;
        *ptr.offset($module.ctx_index as isize)
    })
        // #define ngx_http_get_module_srv_conf(r, module)  (r)->srv_conf[module.ctx_index]
}

#[macro_export]
macro_rules! ngx_http_get_module_loc_conf {
    ( $r:expr,$module:expr) =>  ( {
        let ptr = $r.loc_conf;
        *ptr.offset($module.ctx_index as isize)
    })
        //#define ngx_http_get_module_loc_conf(r, module)  (r)->loc_conf[module.ctx_index]

}

#[macro_export]
macro_rules! ngx_http_get_module_main_conf {
    ( $r:expr,$module:expr) =>  ( {
        let ptr = $r.main_conf;
        *ptr.offset($module.ctx_index as isize)
    })
}



// server config
pub trait ServerConfig {

    fn init(&mut self)  {
    }
}

pub trait CollectorConfig {

    // convert and migrate values to istio attributes
    fn process_istio_attr(&self, attr: &mut AttributeWrapper);

}

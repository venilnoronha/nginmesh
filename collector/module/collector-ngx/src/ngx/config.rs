
use ngx_rust::bindings::*;
use nginmesh_collector_transport::attribute::attr_wrapper::AttributeWrapper;

#[macro_export]
macro_rules! NGX_CONF_UNSET_UINT {
    () =>  ( !0 )
}

pub trait CollectorConfig {

    // convert and migrate values to istio attributes
    fn process_istio_attr(&self, attr: &mut AttributeWrapper);

}

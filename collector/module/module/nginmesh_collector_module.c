/**
 * @file   nginmesh_collector_module.c
 * @author Sehyo Chang <sehyo@nginx.com>
 * @date   Wed Aug 19 2017
 *
 * @brief  Collector module for nginmesh
 *
 * @section LICENSE
 *
 * Copyright (C) 2017,2018 by Nginx
 *
 */
#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>

ngx_int_t ngx_http_collector_report_handler(ngx_http_request_t *r);


ngx_int_t ngx_http_collector_filter_init(ngx_conf_t *cf);

// create configuration
void *ngx_http_collector_create_loc_conf(ngx_conf_t *cf);
char *ngx_http_collector_merge_loc_conf(ngx_conf_t *cf, void *parent,void *child);

void *ngx_http_collector_create_srv_conf(ngx_conf_t *cf);
char *ngx_http_collector_merge_srv_conf(ngx_conf_t *cf, void *parent, void *child);

void *ngx_http_collector_create_main_conf(ngx_conf_t *cf);

// handlers in rust
void  nginmesh_set_collector_server_config(ngx_str_t *server);

ngx_int_t  nginmesh_collector_init(ngx_cycle_t *cycle);
void  nginmesh_collector_exit();
char *collector_conf_set_topic(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_server(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_destination_service(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_destination_uid(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_destination_ip(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_source_ip(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_source_uid(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_source_service(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *collector_conf_set_source_port(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);


/**
 * This module provide callback to istio for http traffic
 *
 */
static ngx_command_t ngx_http_collector_commands[] = {

    { 
      ngx_string("collector_report"),   
      NGX_HTTP_LOC_CONF | NGX_CONF_FLAG, 
      collector_conf_set_topic,
      NGX_HTTP_LOC_CONF_OFFSET, 
      0,
      NULL
    },
    {
       ngx_string("collector_destination_service"),
       NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
       collector_conf_set_destination_service, 
       NGX_HTTP_SRV_CONF_OFFSET,
       0,
       NULL
     },
    {
        ngx_string("collector_destination_uid"), 
        NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
        collector_conf_set_destination_uid, 
        NGX_HTTP_SRV_CONF_OFFSET,
        0,
        NULL
     },
     {
      ngx_string("collector_destination_ip"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      collector_conf_set_destination_ip,
      NGX_HTTP_SRV_CONF_OFFSET,
      0,
      NULL
    },
    {
      ngx_string("collector_source_ip"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      collector_conf_set_source_ip,
      NGX_HTTP_SRV_CONF_OFFSET,
      0,
      NULL
    },

    {
      ngx_string("collector_source_uid"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      collector_conf_set_source_uid,
      NGX_HTTP_SRV_CONF_OFFSET,
      0,
      NULL
    },
    {
      ngx_string("collector_source_service"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      collector_conf_set_source_service,
      NGX_HTTP_SRV_CONF_OFFSET,
      0,
      NULL
    },
    {
      ngx_string("collector_source_port"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      collector_conf_set_source_port,
      NGX_HTTP_SRV_CONF_OFFSET,
      0,
      NULL
    },
    { 
      ngx_string("collector_server"), /* directive */
      NGX_HTTP_MAIN_CONF|NGX_CONF_TAKE1,  // server takes 1 //
      collector_conf_set_server, /* configuration setup function */
      NGX_HTTP_MAIN_CONF_OFFSET, 
      0,
      NULL
    },
    ngx_null_command /* command termination */
};


/* The module context. */
static ngx_http_module_t ngx_http_collector_module_ctx = {
    NULL, /* preconfiguration */
    ngx_http_collector_filter_init, /* postconfiguration */
    ngx_http_collector_create_main_conf, /* create main configuration */
    NULL, /* init main configuration */

    ngx_http_collector_create_srv_conf, /* create server configuration */
    ngx_http_collector_merge_srv_conf, /* merge server configuration */

    ngx_http_collector_create_loc_conf, /* create location configuration */
    ngx_http_collector_merge_loc_conf /* merge location configuration */
};

/* Module definition. */
ngx_module_t ngx_http_collector_module = {
    NGX_MODULE_V1,
    &ngx_http_collector_module_ctx, /* module context */
    ngx_http_collector_commands, /* module directives */
    NGX_HTTP_MODULE, /* module type */
    NULL, /* init master */
    NULL, /* init module */
    nginmesh_collector_init, /* init process */
    NULL, /* init thread */
    NULL, /* exit thread */
    NULL, /* exit process */
    NULL, /* exit master */
    NGX_MODULE_V1_PADDING
};



use crate::config::Config;
use jsonrpc_core::IoHandler;
use jsonrpc_http_server::{Server, ServerBuilder};
use jsonrpc_server_utils::cors::AccessControlAllowOrigin;
use jsonrpc_server_utils::hosts::DomainsValidation;
use std::net::ToSocketAddrs;

pub struct RpcServer {
    pub(crate) server: Server,
}

impl RpcServer {
    pub fn new(config: Config, io_handler: IoHandler) -> RpcServer {
        let server = ServerBuilder::new(io_handler)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Null,
                AccessControlAllowOrigin::Any,
            ]))
            .threads(config.threads.unwrap_or_else(num_cpus::get))
            .max_request_body_size(config.max_request_body_size)
            .health_api(("/ping", "ping"))
            .start_http(
                &config
                    .listen_address
                    .to_socket_addrs()
                    .expect("config listen_address parsed")
                    .next()
                    .expect("config listen_address parsed"),
            )
            .expect("Jsonrpc initialize");

        RpcServer { server }
    }

    pub fn close(self) {
        self.server.close()
    }
}

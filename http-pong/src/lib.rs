use proxy_wasm::traits::RootContext;
use proxy_wasm::types::LogLevel;
use crate::root_context::PongRootContext;

mod root_context;
mod http_context;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(PongRootContext {})
    });
}}
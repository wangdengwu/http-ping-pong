use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;

use crate::http_context::PongHttpContext;

pub struct PongRootContext {}

impl Context for PongRootContext {}

impl RootContext for PongRootContext {
    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(PongHttpContext::default()))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
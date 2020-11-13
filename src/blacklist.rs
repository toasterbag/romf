use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use tide::{Middleware, Next, Request, Response, StatusCode};
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
#[derive(Debug)]
pub struct BlacklistMiddleware {
    blacklist: HashSet<String>,
}

impl BlacklistMiddleware {
    pub fn new() -> Self {
        let file = std::fs::read_to_string(crate::BLACKLIST_PATH).unwrap_or_default();
        let blacklist = file.split_terminator('\n').map(|s| s.to_string()).collect();
        BlacklistMiddleware { blacklist }
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for BlacklistMiddleware {
    fn handle<'a>(
        &'a self,
        req: Request<State>,
        next: Next<'a, State>,
    ) -> BoxFuture<'a, tide::Result<Response>> {
        Box::pin(async move {
            if let Some(ip) = req.remote() {
                if self.blacklist.contains(ip) {
                    return Ok(Response::new(StatusCode::Forbidden));
                }
            }
            let res = next.run(req).await;
            return res;
        })
    }
}

use router::Router;
use mount::OriginalUrl;
use handlers::event;

pub fn routes() -> Router {
    let mut router = Router::new();
    router.get("/event/:event", event::handler, "event");
    router
}

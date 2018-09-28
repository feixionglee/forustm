extern crate forustm;
extern crate sapper;
extern crate sapper_std;

use std::sync::Arc;
use sapper::{Request, Response, Result as SapperResult, SapperApp, SapperAppShell};
use forustm::{create_pg_pool, create_redis_pool, Postgresql, Redis};
use forustm::web::*;
use forustm::{get_identity_and_web_context, Permissions, WebContext};

struct WebApp;

impl SapperAppShell for WebApp {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        sapper_std::init(req, Some("forustm_session"))?;
        let (identity, web) = get_identity_and_web_context(req);
        req.ext_mut().insert::<Permissions>(identity);
        req.ext_mut().insert::<WebContext>(web);
        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

fn main() {
    let redis_pool = Arc::new(create_redis_pool(None));
    let pg_pool = create_pg_pool();
    let mut app = SapperApp::new();
    let port = 8081;
    app.address("0.0.0.0")
        .port(port)
        .init_global(Box::new(move |req: &mut Request| {
            req.ext_mut().insert::<Redis>(Arc::clone(&redis_pool));
            req.ext_mut().insert::<Postgresql>(Arc::clone(&pg_pool));
            Ok(())
        }))
        .with_shell(Box::new(WebApp))
        .add_module(Box::new(Index))
        .add_module(Box::new(WebSection))
        .add_module(Box::new(WebArticle))
        .add_module(Box::new(Home))
        .add_module(Box::new(WebAdminSection))
        .static_service(true);

    println!("Start listen on http://{}:{}", "0.0.0.0", port);
    app.run_http();
}

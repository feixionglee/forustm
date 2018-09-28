extern crate forustm;
extern crate sapper;
extern crate sapper_std;

use sapper::{Request, Response, Result as SapperResult, SapperApp, SapperAppShell};
use forustm::proxy::ProxyModule;

struct WebApp;

impl SapperAppShell for WebApp {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        sapper_std::init(req, Some("forustm_session"))?;
        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

fn main() {
    let mut app = SapperApp::new();
    let port = 7777;
    app.address("0.0.0.0")
        .port(port)
        .with_shell(Box::new(WebApp))
        .add_module(Box::new(ProxyModule))
        .static_service(false);

    println!("Start listen on http://{}:{}", "0.0.0.0", port);
    app.run_http();
}

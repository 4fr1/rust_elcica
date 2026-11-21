
use askama::Template;
use worker::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}

#[event(fetch)]
pub async fn main(
    _req: Request,
    _env: Env,
    _ctx: worker::Context,
) -> Result<Response> {
    let page = IndexTemplate {
        title: "ELCICA",
    };

    match page.render() {
        Ok(html) => Response::from_html(html),
        Err(_err) => Response::error("Internal Server Error", 500),
    }
}


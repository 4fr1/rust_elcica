use askama::Template;
use worker::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    domain: &'a str,
    message: &'a str,
}

#[event(fetch)]
pub async fn main(_req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    let template = IndexTemplate {
        domain: "elcica.xyz",
        message: "Selamat datang di server Rust Wasm ultra-cepat!",
    };

    match template.render() {
        Ok(html) => Response::from_html(html),
        Err(_) => Response::error("Internal Server Error", 500),
    }
}





use askama::Template;
use worker::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}

#[event(fetch)]
pub async fn main(
    req: Request,
    _env: Env,
    _ctx: worker::Context,
) -> Result<Response> {
    let url = req.url()?;

    match (req.method(), url.path()) {
        (Method::Get, "/health") => Response::from_json(&serde_json::json!({
            "app": "elcica",
            "status": "ok",
            "version": "0.1.0"
        })),

        // Halaman admin sekarang dilayani auth.elcica.xyz.
        (Method::Get, "/admin") => {
            let mut res = Response::empty()?;
            res.headers_mut()
                .set("Location", "https://auth.elcica.xyz/login")?;
            Ok(res.with_status(302))
        }

        _ => {
            let page = IndexTemplate {
                title: "ELCICA",
            };

            match page.render() {
                Ok(html) => Response::from_html(html),
                Err(_err) => Response::error("Internal Server Error", 500),
            }
        }
    }
}

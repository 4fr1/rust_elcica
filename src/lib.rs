use askama::Template;
use worker::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        // 1. Route Utama (HTML Askama)
        .get_async("/", |_req, _ctx| async move {
            let template = IndexTemplate { title: "Home" };
            match template.render() {
                Ok(html) => Response::from_html(html),
                Err(_) => Response::error("Internal Server Error", 500),
            }
        })
        
        // 2. Route PWA Manifest (`/manifest.json`)
        .get("/manifest.json", |_req, _ctx| {
            let manifest = r#"{
  "name": "ELCICA Ecosystem",
  "short_name": "ELCICA",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0a0e14",
  "theme_color": "#0a0e14",
  "icons": [
    {
      "src": "https://via.placeholder.com/192/0a0e14/00ff66?text=ELCICA",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "https://via.placeholder.com/512/0a0e14/00ff66?text=ELCICA",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}"#;
            let mut headers = Headers::new();
            let _ = headers.set("content-type", "application/json");
            Ok(Response::from_body(ResponseBody::Body(manifest.as_bytes().to_vec()))?.with_headers(headers))
        })
        
        // 3. Route PWA Service Worker (`/sw.js`)
        .get("/sw.js", |_req, _ctx| {
            let sw_script = r#"
const CACHE_NAME = "elcica-v1";
const ASSETS_TO_CACHE = [
  "/",
  "/manifest.json"
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(ASSETS_TO_CACHE);
    })
  );
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cache) => {
          if (cache !== CACHE_NAME) {
            return caches.delete(cache);
          }
        })
      );
    })
  );
  self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }
      return fetch(event.request).catch(() => {
        return caches.match("/");
      });
    })
  );
});
"#;
            let mut headers = Headers::new();
            let _ = headers.set("content-type", "application/javascript");
            Ok(Response::from_body(ResponseBody::Body(sw_script.as_bytes().to_vec()))?.with_headers(headers))
        })
        .run(req, &env)
        .await
}


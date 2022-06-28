use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Result, Server, StatusCode,
};
use image_text::PngTextBuilder;
use json::{self, object};
use std::{convert::Infallible, net::SocketAddr};
use tokio::runtime::Builder;


macro_rules! response {
    ($b: expr) => {
        Ok(Response::builder().status(StatusCode::OK).body($b).unwrap())
    };
}

async fn png_text_req(req: Request<Body>) -> Result<Response<Body>> {
    let body = req.into_body();
    let body = to_bytes(body).await?;
    let body_str = unsafe { std::str::from_utf8_unchecked(&body) };
    let error = object!{
        "code": -1i32,
        "msg": "invalid message"
    };
    let error = json::stringify(error);
    let json_obj = match json::parse(body_str) {
        Ok(json::JsonValue::Object(obj)) => obj,
        _ => {
            return response!(error.into());
        }
    };
    let text: &str = match json_obj["text"].as_str() {
        Some(o) => o,
        _ => {
            return response!(error.into());
        }
    };
    let id = match json_obj["id"].as_str() {
        Some(o) => o,
        _ => {
            return response!(error.into());
        }
    };
    let png_text = PngTextBuilder::new()
        .font_path("./STHeiti.ttf")
        .font_size((45., 45.))
        .png_path("default.png")
        .point((360., 65.))
        .text(&text)
        .build()
        .unwrap();
    png_text.save(format!("{}.png", id)).unwrap();
    response!(Body::empty())
}

fn main() {
    let rt = Builder::new_multi_thread()
        .enable_io()
        .worker_threads(5)
        .build()
        .unwrap();
    rt.block_on(async move {
        let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(png_text_req)) });
        let serv = Server::bind(&addr).serve(make_svc);
        serv.await.unwrap();
    });
}

mod structs;
mod api;

use std::path::{Path, PathBuf};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::header::HeaderValue;
use mysql::{Pool, PooledConn};

fn get_mysql_connection() -> PooledConn {
    let pool = Pool::new("mysql://md:memedump6969@localhost:3306/memedump").unwrap();
    pool.try_get_conn(500).expect("Could not get connection from pool")
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/api/addmeme") => api::addmeme(req).await,

        (&Method::GET, "/api/getmemes") => api::getmemes(req).await,

        (&Method::GET, "/api/get") => api::getmeme(req).await,

        (&Method::GET, "/debug") => Ok(Response::new(Body::from("ass"))),

        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // return 404 for all other requests
        _ => {
            let filepath = Path::new("public/")
                .join(req.uri().path()
                    .to_string().strip_prefix("/").unwrap());

            let mut res_path:PathBuf = PathBuf::new();

            let found:bool;

            if filepath.is_file() {
                found = true;
                res_path = filepath;
            } else if filepath.is_dir() {
                res_path = filepath.join("index.html");

                if res_path.is_file() {
                    found = true;
                } else {
                    found = false;
                }
            } else {
                found = false;
            }

            let mut response:Response<Body>;

            if found {
                let idk = res_path.to_str().unwrap();
                let body = std::fs::read(idk).unwrap();

                response = Response::new(Body::from(body));

                // i am crying that is how cursed this is
                let mime_guess = mime_guess::from_path(idk);
                let mut str:String = "".parse().unwrap();
                str = str + mime_guess.first().unwrap().to_string().as_str();
                let yes = HeaderValue::from_str(str.as_str()).unwrap();
                let yes2 = HeaderValue::from_str(str.as_str()).unwrap();

                *response.headers_mut()
                    .entry("Content-Type")
                    .or_insert(yes) = yes2;
            } else {
                response = Response::new("404 Not Found".into());
                *response.status_mut() = StatusCode::NOT_FOUND;
            }

            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let http_addr = ([127, 0, 0, 1], 3000).into();

    let http_service = make_service_fn(move |_| { // first move it into the closure
        // closure can be called multiple times, so for each call, we must
        // clone it and move that clone into the async block
        async move {
            // async block is only executed once, so just pass it on to the closure
            Ok::<_, hyper::Error>(service_fn(|_req| {
                // but this closure may also be called multiple times, so make
                // a clone for each call, and move the clone into the async block
                async move {
                    let result: Result<Response<Body>, hyper::Error> = echo(_req).await;
                    result
                }
            }))
        }
    });

    let http_server = Server::bind(&http_addr).serve(http_service);

    println!("memedump-rs | Listening on http://{}", http_addr);

    http_server.await?;

    Ok(())
}
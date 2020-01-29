use crate::commands::bootstrap::configure;
use crate::config::dfinity::ConfigDefaultsBootstrap;
use crate::lib::environment::Environment;
use crate::lib::error::{DfxError, DfxResult};
use actix_files::Files;
use actix_web::client::Client;
use actix_web::http::uri::{Authority, PathAndQuery, Scheme, Uri};
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use atomic_counter::{AtomicCounter, RelaxedCounter};
use bytes::Bytes;
use clap::ArgMatches;
use futures::future::{ok, Either, Future};
use futures::stream::Stream;
use std::default::Default;
use std::time::Duration;

/// Defines the state associated with the bootstrap server.
struct State {
    config: ConfigDefaultsBootstrap,
    counter: RelaxedCounter,
}

/// Runs the bootstrap server.
pub fn execute(env: &dyn Environment, args: &ArgMatches<'_>) -> DfxResult {
    let config = configure::get_config(env, args)?;
    let ip = config.ip.unwrap();
    let port = config.port.unwrap();
    HttpServer::new(move || {
        let root = config.root.as_ref().unwrap();
        let state = State {
            config: config.clone(),
            counter: RelaxedCounter::new(0),
        };
        App::new()
            .wrap(Logger::default())
            .data(state)
            .service(web::scope("/api").default_service(web::post().to_async(serve_upstream)))
            .default_service(Files::new("/", &root).index_file("index.html"))
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .map_err(|err| DfxError::Io(err))
}

/// TODO (enzo): Documentation.
fn serve_upstream(
    request: HttpRequest,
    payload: web::Payload,
    state: web::Data<State>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // TODO (enzo): Documentation.
    let providers = state.get_ref().config.providers.clone().unwrap();
    let timeout = state.get_ref().config.timeout.clone().unwrap();
    let i = state.get_ref().counter.inc();
    let n = providers.len();
    if i >= n {
        state.get_ref().counter.reset();
    };
    let provider = providers[i % n].to_string();
    // TODO (enzo): Documentation.
    match build(request, provider) {
        Err(err) => Either::A(ok(HttpResponse::InternalServerError().body(err.to_string()))),
        Ok(uri) => Either::B(
            payload
                .map_err(Error::from)
                .fold(web::BytesMut::new(), move |mut body, chunk| {
                    body.extend_from_slice(&chunk);
                    Ok::<_, Error>(body)
                })
                .and_then(move |body| {
                    Client::new()
                        .post(uri)
                        .content_type("application/cbor")
                        .timeout(Duration::from_secs(timeout))
                        .send_body(body)
                        .map_err(Error::from)
                        .and_then(|response| {
                            response.concat2().map_err(Error::from).map(|data| {
                                HttpResponse::Ok()
                                    .content_type("application/cbor")
                                    .body(data)
                            })
                        })
                }),
        ),
    }
}

/// TODO (enzo): Documentation.
fn build(request: HttpRequest, provider: String) -> Result<Uri, String> {
    let uri = provider.parse::<Uri>().map_err(|err| err.to_string())?;
    let parts = uri.into_parts();
    let default_scheme = Scheme::HTTP;
    let scheme = parts.scheme.unwrap_or(default_scheme);
    let default_authority = Authority::from_static("127.0.0.1");
    let authority = parts.authority.unwrap_or(default_authority);
    let path_and_query =
        PathAndQuery::from_shared(Bytes::from(request.path())).map_err(|err| err.to_string())?;
    Uri::builder()
        .scheme(scheme)
        .authority(authority)
        .path_and_query(path_and_query)
        .build()
        .map_err(|err| err.to_string())
}

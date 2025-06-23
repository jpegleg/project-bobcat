use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::env;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, http::header::ContentType, middleware, web,
};
use log::debug;
use notify::{Event, RecursiveMode, Watcher as _};
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};
use tokio::sync::mpsc;
use unveil::unveil;
use pledge::pledge_promises;


#[derive(Debug)]
struct TlsUpdated;

async fn index(req: HttpRequest) -> HttpResponse {
    debug!("{req:?}");

    HttpResponse::Ok().content_type(ContentType::html()).body(
        "<!DOCTYPE html><html><body>\
            <p>Morphobsd Template</p>\
        </body></html>",
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {

    pledge_promises![Stdio Inet Rpath Getpw Unveil].unwrap();

    let webpath = "./static/";

    unveil(webpath, "r")
      .or_else(unveil::Error::ignore_platform)
      .unwrap();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let (reload_tx, mut reload_rx) = mpsc::channel(1);

    let mut file_watcher =
        notify::recommended_watcher(move |res: notify::Result<Event>| match res {
            Ok(ev) => {
                log::info!("files changed: {:?}", ev.paths);
                reload_tx.blocking_send(TlsUpdated).unwrap();
            }
            Err(err) => {
                log::error!("file watch error: {err}");
            }
        })
        .unwrap();


    let certpath = "/opt/morpho/cert.pem";
    let keypath = "/opt/morpho/key.pem";

    unveil(certpath, "r")
      .or_else(unveil::Error::ignore_platform)
      .unwrap();
    unveil(keypath, "r")
      .or_else(unveil::Error::ignore_platform)
      .unwrap();

    file_watcher
      .watch(Path::new("/opt/morpho/cert.pem"), RecursiveMode::NonRecursive)
      .unwrap();
    file_watcher
      .watch(Path::new("/opt/morpho/key.pem"), RecursiveMode::NonRecursive)
      .unwrap();

    log::info!("starting morphobsd with libressl on 3443");

    loop {

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        builder.set_private_key(&load_encrypted_private_key()).unwrap();
        builder.set_certificate_chain_file("/opt/morpho/cert.pem").unwrap();

        let mut server = HttpServer::new(|| {
            App::new()
                .service(web::resource("/").to(index))
                .wrap(middleware::Logger::default())
        })
        .workers(2)
        .bind_openssl("0.0.0.0:3443", builder)?
        .run();

        let server_hnd = server.handle();

        tokio::select! {
            res = &mut server => {
                log::info!("server shutdown arrived");
                res?;
                break;
            },

            Some(_) = reload_rx.recv() => {
                log::info!("TLS cert or key updated");
                drop(server_hnd.stop(true));
                server.await?;
                continue;
            }
        }
    }

    Ok(())
}

fn load_encrypted_private_key() -> PKey<Private> {
    let keypath = "/opt/morpho/key.pem";

    unveil(keypath, "r")
      .or_else(unveil::Error::ignore_platform)
      .unwrap();

    let mut file = File::open("/opt/morpho/key.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let binding = env::var("MORPHOP").expect("failed to read MORPHOP");
    let pem_password = binding.as_bytes();
    PKey::private_key_from_pem_passphrase(&buffer, pem_password).unwrap()
}

use std::path::Path;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, http::header::ContentType, middleware, web,
};
use log::debug;
use notify::{Event, RecursiveMode, Watcher as _};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use tokio::sync::mpsc;

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

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

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

    file_watcher
        .watch(Path::new("cert.pem"), RecursiveMode::NonRecursive)
        .unwrap();
    file_watcher
        .watch(Path::new("key.pem"), RecursiveMode::NonRecursive)
        .unwrap();

    log::info!("starting morphobsd with rustls on 3443");
  
    loop {
        let config = load_rustls_config()?;
    
        let mut server = HttpServer::new(|| {
            App::new()
                .service(web::resource("/").to(index))
                .wrap(middleware::Logger::default())
        })
        .workers(2)
        .bind_rustls_0_23("0.0.0.0:3443", config)?
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

fn load_rustls_config() -> eyre::Result<rustls::ServerConfig> {
    let certpath = "./cert.pem";
    let keypath = "./key.pem";
  
    unveil(certpath, "r")
      .or_else(unveil::Error::ignore_platform)
      .unwrap();
    unveil(keypath, "r")
      .or_else(unveil::Error::ignore_platform)
      .unwrap();

    let cert_chain = CertificateDer::pem_file_iter(certpath)
      .unwrap()
      .flatten()
      .collect();

    let key_der = PrivateKeyDer::from_pem_file(keypath).expect("Could not locate PKCS 8 private keys.");

    Ok(ServerConfig::builder()
      .with_no_client_auth()
      .with_single_cert(cert_chain, key_der)?)
}

use std::io;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Result};
use bytes::Bytes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (sender, receiver) = channel::<char>(1);
    let sender: Data<Sender<char>> = Data::new(sender);
    let receiver = Data::new(receiver);

    HttpServer::new(move || {
        App::new()
            .service(stream)
            .service(wake)
            .app_data(sender.clone())
            .app_data(receiver.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

struct Guard {
    pub receiver: Receiver<char>,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropped");
    }
}

impl Guard {
    pub fn into_stream(self) -> impl futures_util::Stream<Item = Result<Bytes>> {
        futures_util::stream::unfold(self, move |mut this| async move {
            let c = this.receiver.recv().await.unwrap();

            Some((Ok(vec![c as u8].into()), this))
        })
    }
}

#[get("/stream")]
async fn stream(sender: Data<Sender<char>>) -> Result<HttpResponse> {
    println!("stream called");
    let receiver = sender.subscribe();
    // response
    Ok(HttpResponse::Ok().streaming(Guard { receiver }.into_stream()))
}

#[get("/wake")]
async fn wake(sender: Data<Sender<char>>) -> Result<HttpResponse> {
    println!("wake called");

    sender.send('a').unwrap();

    // response
    Ok(HttpResponse::NoContent().into())
}

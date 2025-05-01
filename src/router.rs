use crate::chat;
use crate::tch_cpu;
use axum::{
    Router,
    response::sse::{Event, Sse},
    routing::{get, post},
};
use clap::Parser;
use std::{convert::Infallible, path::PathBuf, time::Duration};

use axum_extra::TypedHeader;
use futures::stream::{self, Stream};

use tokio_stream::StreamExt as _;
use tower_http::{services::ServeDir, trace::TraceLayer};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub async fn hello_world() -> String {
    let args = chat::Config::parse();
    let g = tch_cpu::run(args);
    g.text
}

pub async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let (tx, rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        let args = chat::Config::parse();
        let g = tch_cpu::run(args);
        for chunk in g.text.split(" ").into_iter() {
            tx.send(Ok(Event::default().data(chunk))).await.unwrap();
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

use axum::{
    response::sse::{Event, Sse},
    routing::{get, post},
    Router,
};
use clap::Parser;
use crate::chat;
use crate::tch_cpu;
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

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
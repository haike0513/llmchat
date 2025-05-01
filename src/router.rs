use crate::{chat, server::state::AppState};
use crate::ext::LlamaExtension;
use crate::tch_cpu;
use axum::{
    extract::State, response::sse::{Event, Sse}, routing::{get, post}, Router
};
use clap::Parser;
use std::{convert::Infallible, path::PathBuf, time::Duration};

use axum_extra::TypedHeader;
use futures::stream::{self, Stream};

use tokio_stream::StreamExt as _;
use tower_http::{services::ServeDir, trace::TraceLayer};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub async fn hello_world() -> String {
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let args = chat::Config::parse();
    let g = tch_cpu::run(args, tx);
    g.text
}

pub async fn sse_handler(
    State(s) : State<AppState>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let models = s.models.lock();
    
    if let Ok(models) = models {
        let model = models.get("llm");
        if let Some(llm) = model {
            let result = llm.generate();
        }
    }

    let (tx, rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        let args = chat::Config::parse();
        let g = tch_cpu::run(args, tx);
        // for chunk in g.text.split(" ").into_iter() {
        //     tx.send(Ok(Event::default().data(chunk))).await.unwrap();
        // }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    )
}

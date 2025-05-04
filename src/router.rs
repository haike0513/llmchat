use crate::{chat, server::state::AppState};
use crate::ext::LlamaExtension;
// use crate::tch_cpu;
use axum::{
    extract::State, response::sse::{Event, Sse}, routing::{get, post}, Router
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, path::PathBuf, time::Duration};

use axum_extra::TypedHeader;
use futures::stream::{self, Stream};

use tokio_stream::StreamExt as _;
use tower_http::{services::ServeDir, trace::TraceLayer};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub async fn hello_world() -> String {
    "Hello World".to_string()
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseParam {
    pub prompt: String,
}

use axum::debug_handler;

#[debug_handler]
pub async fn sse_handler(
    State(s) : State<AppState>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    axum::Json(sse_param): axum::Json<SseParam>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::debug!("`{}` connected", user_agent.as_str());

    let models = s.models.lock();
    
    let result_stream = if let Ok(models) = models {
        let model = models.get("llm");
        let s = if let Some(llm) = model {
            tracing::debug!("llm: {:?}", llm.name());
            let result = llm.generate(sse_param.prompt.clone());
            let final_r = result.map(|s|  {
                tracing::debug!("llm: {:?}", s);
                let ev = Event::default().data(s);
                Ok(ev)
            });
            Some(final_r)
        } else {
            None
        };
        s
    } else {
        None
    };

    let rs = result_stream.unwrap();



    // let (tx, rx) = tokio::sync::mpsc::channel(32);

    // tokio::spawn(async move {
    //     let args = chat::Config::parse();
    //     let g = tch_cpu::run(args, tx);
    //     // for chunk in g.text.split(" ").into_iter() {
    //     //     tx.send(Ok(Event::default().data(chunk))).await.unwrap();
    //     // }
    // });

    // let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    Sse::new(rs).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    )
}

use crate::{exec, tasks::get_task, IResponse};

use std::sync::Arc;

use amber_dchat_rpc_utils::structs::{PartialRpcStatus, RpcStatus};
use tokio::{net::TcpStream, spawn};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use futures_util::{pin_mut, SinkExt, StreamExt, TryStreamExt};

pub fn accept<T: IResponse + Send + Sync + 'static>(stream: TcpStream, sender: Arc<T>) {
  spawn(async move {
    let task_id = get_task();
    run(stream, sender, task_id).await
  });
}

async fn run<T: IResponse + Send + Sync + 'static>(
  stream: TcpStream,
  sender: Arc<T>,
  task_id: usize,
) -> Option<()> {
  let socket = accept_async(stream).await.ok()?;

  let (mut socket, response) = socket.split();

  let _ = socket.send(Message::text(r#"{"__rpc_rs":"polling","format0":"https://docs.rs/amber_dchat_rpc_utils/latest/amber_dchat_rpc_utils/structs/struct.PartialRpcStatus.html"}"#)).await;

  let broadcast = response
    .try_for_each(|res| {
      let sender = sender.clone();
      async move {
        if res.is_close() {
          return Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed);
        } else if res.is_text() {
          let txt = res.to_text().unwrap();

          let Ok(rpc) = serde_json::from_str::<PartialRpcStatus>(txt) else {
            return Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed);
          };

          let rpc = RpcStatus::from(rpc, task_id);

          sender.send_listener(&serde_json::to_string(&rpc).unwrap());
        }

        Ok(())
      }
    });

  pin_mut!(broadcast);
  let _ = broadcast.await;

  let _ = socket.close().await;

  sender.send_listener(&format!("{{\"_clear\": {task_id}}}"));

  exec! {
    println!("Task {task_id} finished");
  }

  Some(())
}

use crate::{exec, tasks::get_task, IResponse};

use std::sync::Arc;

use amber_dchat_rpc_utils::structs::{PartialRpcStatus, RpcStatus};
use tokio::{net::TcpStream, spawn};
use tokio_tungstenite::accept_async;

use futures_util::{SinkExt, StreamExt, TryStreamExt};

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

  response
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
    })
    .await
    .ok()?;

  let _ = socket.close().await;

  exec! {
    println!("Task {task_id} finished");
  }

  Some(())
}

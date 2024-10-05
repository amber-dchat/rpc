use std::{sync::Arc, thread};

use tokio::{net::TcpListener, runtime::Builder};

pub(crate) use amber_dchat_rpc_utils as utils;
use ws::accept;

pub mod structs;

pub(crate) mod tasks;
mod ws;

pub trait IResponse {
  fn send_listener(&self, data: &str) -> () {
    let _ = data;
  }

  fn submit(&self, success: bool) -> () {
    let _ = success;
  }
}

/// THIS IS AN INTERNAL MACRO
///
/// DO NOT USE
/// THE ABI CAN CHANGE ANI TIME
#[macro_export]
macro_rules! exec {
    ($($a:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $($a)*
        }
    };
}

pub fn bootstrap<T: IResponse + Send + Sync + 'static>(a: T) {
  thread::spawn(move || {
    let rt = Builder::new_multi_thread()
      .enable_all()
      .worker_threads(2)
      .build()
      .unwrap();

    rt.block_on(async {
      launch(a).await;
    });
  });
}

async fn launch<T: IResponse + Send + Sync + 'static>(a: T) {
  exec! {
      println!("Launching server");
  }

  if let Some(port) = utils::find_port().await {
    let listener = Arc::new(a);

    let report = listener.clone();
    let res: Option<()> = async move {
      exec! {
          println!("Using port {port}");
      }

      let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await.ok()?;

      while let Ok((stream, _)) = listener.accept().await {
        exec! {
            println!("New connection");
        }
        accept(stream, report.clone());
      }

      Some(())
    }
    .await;

    listener.submit(res.is_some());
  } else {
    a.submit(false);
  }
}

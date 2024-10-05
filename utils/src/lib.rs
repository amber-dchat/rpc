use tokio::net::TcpListener;

pub static LOWEST_PORT: u16 = 64_000;
pub static HIGHEST_PORT: u16 = 65_535;

pub async fn find_port() -> Option<u16> {
    for port in LOWEST_PORT..=HIGHEST_PORT {
        match TcpListener::bind(format!("127.0.0.1:{port}")).await {
            Ok(_) => {
                return Some(port);
            }
            _ => continue
        }
    }

    None
}

pub async fn get_rpc_port() -> Option<u16> {
    None
}

pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Default, Deserialize)]
    pub enum RpcType {
        #[default]
        None,
        NoPrefix,
        Playing,
        Watching,
        Listenting,
        Streaming,
        Coding,
        Reading,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct PartialRpcStatus {
        pub prefix: RpcType,
        pub title: String,
        pub description: String,
        pub from: u64
    }


    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct RpcStatus {
        pub id: usize,
        pub prefix: RpcType,
        pub title: String,
        pub description: String,
        pub from: u64
    }

    impl RpcStatus {
        pub fn from(PartialRpcStatus { prefix, title, description, from }: PartialRpcStatus, id: usize) -> Self {
            Self {
                id,
                prefix,
                description,
                title,
                from
            }
        }

        pub fn degrade(self) -> PartialRpcStatus {
            PartialRpcStatus {
                prefix: self.prefix,
                title: self.title,
                description: self.description,
                from: self.from
            }
        }
    }
}
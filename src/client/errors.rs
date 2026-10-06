use std::io;

use crate::protocol;
use crate::server::socket_paths::client_socket_path;

/// Errors that can occur during client operation.
#[derive(Debug)]
pub enum ClientError {
    /// Could not connect to the server's client socket.
    ConnectionFailed(io::Error),
    /// Server rejected our handshake.
    HandshakeRejected { version: u32, error: String },
    /// Server shut down.
    ServerShutdown { reason: Option<String> },
    /// Lost connection to the server.
    ConnectionLost(io::Error),
    /// Protocol error (framing, deserialization).
    Protocol(protocol::FramingError),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ConnectionFailed(err) => {
                write!(f, "無法連線至伺服器：{err}")?;
                let path = client_socket_path();
                write!(
                    f,
                    "\nherdr_zh 伺服器是否正在執行？請使用 `herdr server` 啟動。"
                )?;
                write!(f, "\nSocket 路徑：{}", path.display())
            }
            ClientError::HandshakeRejected { version, error } => {
                write!(f, "伺服器拒絕交握（版本 {version}）：{error}")
            }
            ClientError::ServerShutdown { reason } => {
                match reason.as_deref() {
                    Some("detached") => {
                        if let Ok(reattach_command) =
                            std::env::var(crate::remote::REATTACH_COMMAND_ENV_VAR)
                        {
                            write!(f, "已與遠端伺服器中斷連線")?;
                            write!(f, "\n執行 `{reattach_command}` 以重新連線")?;
                        } else {
                            write!(f, "已與伺服器中斷連線")?;
                            write!(
                                f,
                                "\n執行 `{}` 以重新連線",
                                crate::session::local_attach_command()
                            )?;
                        }
                    }
                    _ => {
                        write!(f, "伺服器已關閉")?;
                        if let Some(reason) = reason {
                            write!(f, ": {reason}")?;
                        }
                    }
                }
                Ok(())
            }
            ClientError::ConnectionLost(err) => {
                if let Ok(reattach_command) = std::env::var(crate::remote::REATTACH_COMMAND_ENV_VAR)
                {
                    write!(f, "與遠端 herdr_zh 的連線已中斷：{err}")?;
                    write!(f, "\n若遠端伺服器在 SSH 或網路中斷後仍存活，其窗格可能仍在執行。")?;
                    write!(f, "\n執行 `{reattach_command}` 以重新連線")
                } else {
                    write!(f, "與伺服器的連線已中斷：{err}")
                }
            }
            ClientError::Protocol(err) => write!(f, "協定錯誤：{err}"),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientError::ConnectionFailed(err) => Some(err),
            ClientError::ConnectionLost(err) => Some(err),
            ClientError::Protocol(err) => Some(err),
            _ => None,
        }
    }
}

impl From<protocol::FramingError> for ClientError {
    fn from(err: protocol::FramingError) -> Self {
        match err {
            protocol::FramingError::UnexpectedEof => ClientError::ConnectionLost(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "server closed connection",
            )),
            protocol::FramingError::Io(err) => ClientError::ConnectionLost(err),
            err => ClientError::Protocol(err),
        }
    }
}

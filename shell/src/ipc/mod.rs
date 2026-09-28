use crate::core::event::Event;
use anyhow::{Context, Result};
use color_engine::Theme;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::thread;
use tracing::{debug, error, info, warn};

pub struct IpcServer {
    socket_path: PathBuf,
}

impl IpcServer {
    pub fn new() -> Self {
        let socket_path = Self::get_socket_path();
        Self { socket_path }
    }

    fn get_socket_path() -> PathBuf {
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
            .unwrap_or_else(|| "/tmp".into());
        Path::new(&runtime_dir).join("adaptive-shell-ipc.sock")
    }

    pub fn start(&self, sender: Sender<Event>) -> Result<()> {
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)
                .with_context(|| format!("removing old socket at {}", self.socket_path.display()))?;
        }

        let listener = UnixListener::bind(&self.socket_path)
            .with_context(|| format!("binding to socket at {}", self.socket_path.display()))?;

        info!("IPC server listening on {}", self.socket_path.display());

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        debug!("new IPC connection established");
                        let reader = BufReader::new(stream);
                        for line in reader.lines() {
                            match line {
                                Ok(content) => {
                                    match serde_json::from_str::<Theme>(&content) {
                                        Ok(theme) => {
                                            if let Err(e) = sender.send(Event::ThemeUpdated(theme)) {
                                                error!("Failed to send ThemeUpdated event: {}", e);
                                            } else {
                                                info!("Theme update received and queued via IPC");
                                            }
                                        }
                                        Err(e) => {
                                            warn!("Invalid theme JSON received over IPC: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Error reading from IPC stream: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("IPC connection failed: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
}

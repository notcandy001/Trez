use anyhow::Result;
use std::collections::HashMap;
use tracing::{debug, info};
use wayland_client::protocol::wl_output::WlOutput;

#[derive(Debug)]
pub struct Monitor {
    pub output: WlOutput,
    pub name: String,
    pub description: String,
    pub size: (i32, i32),
    pub scale: f64,
}

pub struct DesktopState {
    pub monitors: HashMap<u32, Monitor>,
    pub active_workspace: u32,
    pub total_workspaces: u32,
}

impl DesktopState {
    pub fn new() -> Self {
        Self {
            monitors: HashMap::new(),
            active_workspace: 1,
            total_workspaces: 1,
        }
    }

    pub fn add_monitor(&mut self, id: u32, monitor: Monitor) {
        info!("Monitor attached: {} ({}x{} @ {})", monitor.name, monitor.size.0, monitor.size.1, monitor.scale);
        self.monitors.insert(id, monitor);
    }

    pub fn remove_monitor(&mut self, id: u32) {
        if let Some(monitor) = self.monitors.remove(&id) {
            info!("Monitor detached: {}", monitor.name);
        }
    }

    pub fn update_workspace(&mut self, active: u32, total: u32) {
        if self.active_workspace != active || self.total_workspaces != total {
            debug!("Workspace state updated: {}/{}", active, total);
            self.active_workspace = active;
            self.total_workspaces = total;
        }
    }
}

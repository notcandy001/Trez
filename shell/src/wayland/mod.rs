use anyhow::{Context, Result};
use tracing::{debug, info};
use wayland_client::{
    protocol::{wl_output, wl_registry},
    Connection, Dispatch, EventQueue, QueueHandle,
};
use crate::desktop::{DesktopState, Monitor};

pub struct ShellState {
    pub desktop: DesktopState,
}

impl Dispatch<wl_registry::WlRegistry, ()> for ShellState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            if interface == "wl_output" {
                registry.bind::<wl_output::WlOutput, _, _>(name, version.min(4), qh, ());
            }
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for ShellState {
    fn event(
        state: &mut Self,
        output: &wl_output::WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // Handle output events to build the monitor state
        // In a real implementation we would accumulate events until `Done` is received.
        // For now, we stub out the events we care about.
        match event {
            wl_output::Event::Name { name } => {
                debug!("Discovered output name: {}", name);
            }
            wl_output::Event::Description { description } => {
                debug!("Discovered output description: {}", description);
            }
            wl_output::Event::Geometry { transform: _, .. } => {}
            wl_output::Event::Mode { width, height, .. } => {
                // Here we would capture the mode (resolution)
                debug!("Output mode: {}x{}", width, height);
            }
            wl_output::Event::Scale { factor } => {
                debug!("Output scale: {}", factor);
            }
            wl_output::Event::Done => {
                info!("Output fully configured");
            }
            _ => {}
        }
    }
}

pub struct WaylandRuntime {
    connection: Connection,
    event_queue: EventQueue<ShellState>,
    state: ShellState,
}

impl WaylandRuntime {
    pub fn connect() -> Result<Self> {
        let connection = Connection::connect_to_env().context("connecting to Wayland compositor")?;
        let mut event_queue = connection.new_event_queue();
        let qh = event_queue.handle();

        let display = connection.display();
        display.get_registry(&qh, ());

        let state = ShellState {
            desktop: DesktopState::new(),
        };

        Ok(Self {
            connection,
            event_queue,
            state,
        })
    }

    pub fn dispatch_pending(&mut self) -> Result<()> {
        self.event_queue
            .dispatch_pending(&mut self.state)
            .context("dispatching Wayland events")?;
        self.connection
            .flush()
            .context("flushing Wayland requests")?;
        Ok(())
    }

    pub fn state(&self) -> &ShellState {
        &self.state
    }
    
    pub fn state_mut(&mut self) -> &mut ShellState {
        &mut self.state
    }
}

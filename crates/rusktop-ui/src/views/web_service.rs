use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputState};
use gpui_component::v_flex;
use rusktop_core::{ServiceStatus, WebServiceConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct WebServiceView {
    config: WebServiceConfig,
    status: ServiceStatus,
    port_input: Entity<InputState>,
    server_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl WebServiceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let app_config = &cx.global::<crate::GlobalAppConfig>().0;
        let config = WebServiceConfig::new(app_config.server.port);

        let port_input = cx.new(|cx| InputState::new(window, cx));

        Self {
            config,
            status: ServiceStatus::Stopped,
            port_input,
            server_handle: Arc::new(Mutex::new(None)),
        }
    }

    fn toggle_service(
        &mut self,
        _event: &gpui::ClickEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.status {
            ServiceStatus::Stopped | ServiceStatus::Error(_) => {
                let port_text = self.port_input.read(cx).text().to_string();

                if let Ok(port) = port_text.parse::<u16>() {
                    if !WebServiceConfig::is_valid_port(port) {
                        self.status = ServiceStatus::Error("Port must be >= 1024".to_string());
                        cx.notify();
                        return;
                    }

                    self.config.port = port;
                    self.status = ServiceStatus::Starting;
                    cx.notify();

                    let server_handle = self.server_handle.clone();
                    let addr = format!("127.0.0.1:{}", port);
                    let db_url = cx.global::<crate::GlobalAppConfig>().database.url.clone();

                    tokio::spawn(async move {
                        let handle = tokio::spawn(async move {
                            if let Err(e) = rusktop_core::run(&db_url, addr.parse().unwrap()).await {
                                eprintln!("Web server error: {}", e);
                            }
                        });

                        let mut lock = server_handle.lock().await;
                        *lock = Some(handle);
                    });

                    self.status = ServiceStatus::Running { port };
                    cx.notify();
                } else {
                    self.status = ServiceStatus::Error("Invalid port number".to_string());
                    cx.notify();
                }
            }
            ServiceStatus::Running { .. } => {
                self.status = ServiceStatus::Stopping;
                cx.notify();

                let server_handle = self.server_handle.clone();
                tokio::spawn(async move {
                    let mut lock = server_handle.lock().await;
                    if let Some(handle) = lock.take() {
                        handle.abort();
                    }
                });

                self.status = ServiceStatus::Stopped;
                cx.notify();
            }
            _ => {}
        }
    }
}

impl Render for WebServiceView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let button_label = match &self.status {
            ServiceStatus::Stopped | ServiceStatus::Error(_) => "Start Web Server",
            ServiceStatus::Running { .. } => "Stop Web Server",
            ServiceStatus::Starting => "Starting...",
            ServiceStatus::Stopping => "Stopping...",
        };

        let status_text = match &self.status {
            ServiceStatus::Stopped => "Status: Stopped".to_string(),
            ServiceStatus::Starting => "Status: Starting...".to_string(),
            ServiceStatus::Running { port } => format!("Status: Running on port {}", port),
            ServiceStatus::Stopping => "Status: Stopping...".to_string(),
            ServiceStatus::Error(msg) => format!("Status: Error - {}", msg),
        };

        v_flex()
            .p_4()
            .gap_3()
            .child(
                Button::new("toggle")
                    .label(button_label)
                    .primary()
                    .on_click(cx.listener(Self::toggle_service)),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(div().child("Port:"))
                    .child(Input::new(&self.port_input)),
            )
            .child(div().child(status_text))
    }
}

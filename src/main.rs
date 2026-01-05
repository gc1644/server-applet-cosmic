use cosmic::app::{Core, Task};
use cosmic::iced::Subscription;
use cosmic::task;
use cosmic::{Application, Element};

use std::time::Duration;
use tokio::process::Command as TokioCommand;

#[derive(Default)]
struct ServerStatusApplet {
    core: Core,
    is_up: bool,
}

#[derive(Clone, Debug)]
enum Message {
    CheckServer,
    ServerStatus(bool),
}

impl Application for ServerStatusApplet {
    const APP_ID: &'static str = "com.example.ServerStatusApplet"; // Match your App ID

    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let applet = ServerStatusApplet {
            core,
            is_up: false,
        };
        // Trigger immediate first check
        (applet, task::future(async move { Message::CheckServer }))
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            async_stream::stream! {
                loop {
                    tokio::time::sleep(Duration::from_secs(30)).await;
                    yield Message::CheckServer;
                }
            }
        })
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::CheckServer => task::future(async {
                let output = TokioCommand::new("ping")
                    .arg("-c")
                    .arg("1")
                    .arg("-W")
                    .arg("2")
                    .arg("195.96.156.220")
                    .status()
                    .await;

                Message::ServerStatus(matches!(output, Ok(status) if status.success()))
            }),
            Message::ServerStatus(up) => {
                self.is_up = up;
                task::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let icon_name = if self.is_up {
            "emblem-ok-symbolic" // Green check
        } else {
            "process-error-symbolic" // Red error
        };

        self.core.applet.icon_button(icon_name).into()
    }
}

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<ServerStatusApplet>(())
}

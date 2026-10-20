/*

    MIT License

    Copyright (c) 2026 Stevens Benavides

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

*/

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

use futures_lite::StreamExt;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use zbus::{Connection, MatchRule, MessageStream, Proxy};

use crate::error::Result;

const NOTIFICATIONS_BUS: &str = "org.freedesktop.Notifications";
const NOTIFICATIONS_PATH: &str = "/org/freedesktop/Notifications";
const NOTIFICATIONS_INTERFACE: &str = "org.freedesktop.Notifications";
const FILE_MANAGER_BUS: &str = "org.freedesktop.FileManager1";
const FILE_MANAGER_PATH: &str = "/org/freedesktop/FileManager1";
const FILE_MANAGER_INTERFACE: &str = "org.freedesktop.FileManager1";

const FILE_URI_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'/')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[derive(Debug, Clone)]
pub enum NotifierEvent {
    ActionInvoked { id: u32, action: String },
    Closed { id: u32 },
    NotificationsAvailable,
}

#[derive(Debug)]
pub struct Notifier {
    app_name: String,
    icon: String,
    expire_ms: i32,
    action_label: String,
    connection: Option<Connection>,
    events: Option<Receiver<NotifierEvent>>,
    active: HashMap<u32, PathBuf>,
}

impl Notifier {
    #[inline]
    pub fn new(app_name: &str, icon: &str, expire_ms: i32, action_label: &str) -> Notifier {
        Notifier {
            app_name: app_name.to_string(),
            icon: icon.to_string(),
            expire_ms,
            action_label: action_label.to_string(),
            connection: None,
            events: None,
            active: HashMap::new(),
        }
    }
}

impl Notifier {
    #[inline]
    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }

    #[inline]
    pub fn get_icon(&self) -> &str {
        &self.icon
    }

    #[inline]
    pub fn get_expire_ms(&self) -> i32 {
        self.expire_ms
    }

    #[inline]
    pub fn get_action_label(&self) -> &str {
        &self.action_label
    }

    #[inline]
    pub fn get_connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }

    #[inline]
    pub fn get_events(&self) -> Option<&Receiver<NotifierEvent>> {
        self.events.as_ref()
    }

    #[inline]
    pub fn get_active(&self) -> &HashMap<u32, PathBuf> {
        &self.active
    }
}

impl Notifier {
    #[inline]
    pub fn set_app_name(&mut self, app_name: String) {
        self.app_name = app_name;
    }

    #[inline]
    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon;
    }

    #[inline]
    pub fn set_expire_ms(&mut self, expire_ms: i32) {
        self.expire_ms = expire_ms;
    }

    #[inline]
    pub fn set_action_label(&mut self, action_label: String) {
        self.action_label = action_label;
    }

    #[inline]
    pub fn set_connection(&mut self, connection: Option<Connection>) {
        self.connection = connection;
    }

    #[inline]
    pub fn set_events(&mut self, events: Option<Receiver<NotifierEvent>>) {
        self.events = events;
    }

    #[inline]
    pub fn set_active(&mut self, active: HashMap<u32, PathBuf>) {
        self.active = active;
    }
}

impl Notifier {
    #[inline]
    pub fn get_mut_app_name(&mut self) -> &mut String {
        &mut self.app_name
    }

    #[inline]
    pub fn get_mut_icon(&mut self) -> &mut String {
        &mut self.icon
    }

    #[inline]
    pub fn get_mut_expire_ms(&mut self) -> &mut i32 {
        &mut self.expire_ms
    }

    #[inline]
    pub fn get_mut_action_label(&mut self) -> &mut String {
        &mut self.action_label
    }

    #[inline]
    pub fn get_mut_connection(&mut self) -> &mut Option<Connection> {
        &mut self.connection
    }

    #[inline]
    pub fn get_mut_events(&mut self) -> &mut Option<Receiver<NotifierEvent>> {
        &mut self.events
    }

    #[inline]
    pub fn get_mut_active(&mut self) -> &mut HashMap<u32, PathBuf> {
        &mut self.active
    }
}

impl Notifier {
    pub fn ensure_connection(&mut self) -> Result<()> {
        if self.connection.is_some() {
            return Ok(());
        }

        let connection: Connection = zbus::block_on(Connection::session())?;

        let (sender, receiver) = mpsc::channel::<NotifierEvent>();

        std::thread::Builder::new()
            .name("LLVMNotificator-signals".to_string())
            .stack_size(256 * 1024)
            .spawn(move || {
                let result = zbus::block_on(async move {
                    let signal_connection: Connection = Connection::session().await?;

                    let notifications_rule: MatchRule<'_> = MatchRule::builder()
                        .msg_type(zbus::message::Type::Signal)
                        .interface(NOTIFICATIONS_INTERFACE)
                        .build()?;

                    let names_rule: MatchRule<'_> = MatchRule::builder()
                        .msg_type(zbus::message::Type::Signal)
                        .sender("org.freedesktop.DBus")
                        .interface("org.freedesktop.DBus")
                        .member("NameOwnerChanged")
                        .add_arg(NOTIFICATIONS_BUS)
                        .build()?;

                    let mut notifications: MessageStream =
                        MessageStream::for_match_rule(notifications_rule, &signal_connection, None)
                            .await?;

                    let mut names: MessageStream =
                        MessageStream::for_match_rule(names_rule, &signal_connection, None).await?;

                    loop {
                        let message: Option<std::prelude::v1::Result<zbus::Message, zbus::Error>> =
                            futures_lite::future::or(notifications.next(), names.next()).await;

                        let Some(result) = message else {
                            break;
                        };

                        let Ok(message) = result else {
                            eprintln!("warning: failed to read a D-Bus signal");

                            break;
                        };

                        let header: zbus::message::Header<'_> = message.header();

                        let Some(member) = header.member() else {
                            continue;
                        };

                        match member.as_str() {
                            "ActionInvoked" => {
                                if let Ok((id, action)) =
                                    message.body().deserialize::<(u32, String)>()
                                    && sender
                                        .send(NotifierEvent::ActionInvoked { id, action })
                                        .is_err()
                                {
                                    break;
                                }
                            }
                            "NotificationClosed" => {
                                if let Ok((id, _reason)) =
                                    message.body().deserialize::<(u32, u32)>()
                                    && sender.send(NotifierEvent::Closed { id }).is_err()
                                {
                                    break;
                                }
                            }
                            "NameOwnerChanged" => {
                                if let Ok((name, _old_owner, new_owner)) =
                                    message.body().deserialize::<(String, String, String)>()
                                    && name == NOTIFICATIONS_BUS
                                    && !new_owner.is_empty()
                                    && sender.send(NotifierEvent::NotificationsAvailable).is_err()
                                {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    Ok::<(), zbus::Error>(())
                });

                if let Err(error) = result {
                    eprintln!("warning: D-Bus listener stopped: {error}");
                }
            })?;

        self.connection = Some(connection);
        self.events = Some(receiver);

        Ok(())
    }

    pub fn send(&mut self, version: &str, commits: usize, folder: &Path) -> Result<u32> {
        self.ensure_connection()?;

        let Some(connection) = self.connection.as_ref() else {
            return Err(std::io::Error::other("D-Bus session bus unavailable").into());
        };

        let summary: String = format!("{} v{version}", self.app_name);

        let body: String = format!("{commits} new commits - use the button to open the folder");

        let actions: Vec<&str> = vec!["open-folder", self.action_label.as_str()];

        let mut hints: HashMap<String, zbus::Variant<'static>> =
            HashMap::<String, zbus::Variant<'static>>::new();

        hints.insert("urgency".to_string(), zbus::Variant::from(1u8));

        hints.insert(
            "sender-pid".to_string(),
            zbus::Variant::from(std::process::id() as i64),
        );

        let app_name: &str = self.app_name.as_str();
        let icon: &str = self.icon.as_str();
        let expire_ms: i32 = self.expire_ms;

        let id: u32 = zbus::block_on(async {
            let proxy: Proxy<'_> = Proxy::new(
                connection,
                NOTIFICATIONS_BUS,
                NOTIFICATIONS_PATH,
                NOTIFICATIONS_INTERFACE,
            )
            .await?;

            let id: u32 = proxy
                .call(
                    "Notify",
                    &(
                        app_name, 0u32, icon, summary, body, actions, hints, expire_ms,
                    ),
                )
                .await?;

            Ok::<u32, zbus::Error>(id)
        })?;

        self.active.insert(id, folder.to_path_buf());

        Ok(id)
    }

    pub fn next_event(&mut self, timeout: Duration) -> Option<NotifierEvent> {
        let Some(receiver) = self.events.as_ref() else {
            std::thread::sleep(timeout);

            return None;
        };

        match receiver.recv_timeout(timeout) {
            Ok(event) => Some(event),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                self.connection = None;
                self.events = None;
                self.active.clear();

                None
            }
        }
    }

    pub fn handle_event(&mut self, event: NotifierEvent) -> Result<()> {
        match event {
            NotifierEvent::ActionInvoked { id, action } => {
                let Some(folder) = self.active.get(&id).cloned() else {
                    return Ok(());
                };

                if action == "open-folder" {
                    self.open_folder(&folder)?;

                    self.active.remove(&id);
                }

                Ok(())
            }
            NotifierEvent::Closed { id } => {
                self.active.remove(&id);

                Ok(())
            }
            NotifierEvent::NotificationsAvailable => Ok(()),
        }
    }

    pub fn open_folder(&self, folder: &Path) -> Result<()> {
        if !folder.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("folder not found: {}", folder.display()),
            )
            .into());
        }

        let canonical: PathBuf = std::fs::canonicalize(folder)?;

        let encoded: String =
            self::utf8_percent_encode(&canonical.to_string_lossy(), FILE_URI_SET).to_string();

        let uri: String = format!("file://{encoded}");

        let dbus_result = zbus::block_on(async {
            let connection: Connection = Connection::session().await?;

            let proxy: Proxy<'_> = Proxy::new(
                &connection,
                FILE_MANAGER_BUS,
                FILE_MANAGER_PATH,
                FILE_MANAGER_INTERFACE,
            )
            .await?;

            let uris: Vec<String> = vec![uri];

            let _: () = proxy.call("ShowFolders", &(uris, "")).await?;

            Ok::<(), zbus::Error>(())
        });

        if dbus_result.is_ok() {
            eprintln!("opened folder via FileManager1: {}", folder.display());

            return Ok(());
        }

        let spawned = std::process::Command::new("xdg-open")
            .arg(folder)
            .spawn()
            .or_else(|_| {
                std::process::Command::new("gio")
                    .arg("open")
                    .arg(folder)
                    .spawn()
            });

        match spawned {
            Ok(mut child) => {
                eprintln!("opened folder: {}", folder.display());

                std::thread::spawn(move || {
                    let _ = child.wait();
                });

                Ok(())
            }
            Err(error) => Err(std::io::Error::other(format!(
                "could not open folder {}: {error}",
                folder.display()
            ))
            .into()),
        }
    }
}

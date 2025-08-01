use std::{net::SocketAddr, sync::atomic::Ordering};

use packet::{ClientboundPacket, Packet, PacketError, ServerboundPacket};
use pumpkin_config::{RCONConfig, advanced_config};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    select,
};

use crate::{SHOULD_STOP, STOP_INTERRUPT, server::Server};

mod packet;

pub struct RCONServer;

impl RCONServer {
    pub async fn run(config: &RCONConfig, server: Arc<Server>) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(config.address).await.unwrap();

        let password = Arc::new(config.password.clone());

        let mut connections = 0;
        while !SHOULD_STOP.load(Ordering::Relaxed) {
            let await_new_client = || async {
                let t1 = listener.accept();
                let t2 = STOP_INTERRUPT.notified();

                select! {
                    client = t1 => Some(client),
                    () = t2 => None,
                }
            };
            // Asynchronously wait for an inbound socket.

            let Some(result) = await_new_client().await else {
                break;
            };
            let (connection, address) = result?;

            if config.max_connections != 0 && connections >= config.max_connections {
                continue;
            }

            connections += 1;
            let mut client = RCONClient::new(connection, address);

            let password = password.clone();
            let server = server.clone();
            tokio::spawn(async move { while !client.handle(&server, &password).await {} });
            log::debug!("closed RCON connection");
            connections -= 1;
        }
        Ok(())
    }
}

pub struct RCONClient {
    connection: tokio::net::TcpStream,
    address: SocketAddr,
    logged_in: bool,
    incoming: Vec<u8>,
    closed: bool,
}

impl RCONClient {
    #[must_use]
    pub const fn new(connection: tokio::net::TcpStream, address: SocketAddr) -> Self {
        Self {
            connection,
            address,
            logged_in: false,
            incoming: Vec::new(),
            closed: false,
        }
    }

    /// Returns whether the client is closed or not.
    pub async fn handle(&mut self, server: &Arc<Server>, password: &str) -> bool {
        if !self.closed {
            match self.read_bytes().await {
                // The stream is closed, so we can't reply, so we just close everything.
                Ok(true) => return true,
                Ok(false) => {}
                Err(e) => {
                    log::error!("Could not read packet: {e}");
                    return true;
                }
            }
            // If we get a close here, we might have a reply, which we still want to write.
            let _ = self.poll(server, password).await.map_err(|e| {
                log::error!("RCON error: {e}");
                self.closed = true;
            });
        }
        self.closed
    }

    async fn poll(&mut self, server: &Arc<Server>, password: &str) -> Result<(), PacketError> {
        let Some(packet) = self.receive_packet().await? else {
            return Ok(());
        };
        let config = &advanced_config().networking.rcon;
        match packet.get_type() {
            ServerboundPacket::Auth => {
                if packet.get_body() == password {
                    self.send(ClientboundPacket::AuthResponse, packet.get_id(), "")
                        .await?;
                    if config.logging.logged_successfully {
                        log::info!("RCON ({}): Client logged in successfully", self.address);
                    }
                    self.logged_in = true;
                } else {
                    if config.logging.wrong_password {
                        log::info!("RCON ({}): Client tried the wrong password", self.address);
                    }
                    self.send(ClientboundPacket::AuthResponse, -1, "").await?;
                    self.closed = true;
                }
            }
            ServerboundPacket::ExecCommand => {
                if self.logged_in {
                    let output = Arc::new(tokio::sync::Mutex::new(Vec::<String>::new()));

                    let server_clone = server.clone();
                    let output_clone = output.clone();
                    let packet_body = packet.get_body().to_owned();
                    tokio::spawn(async move {
                        let dispatcher = server_clone.command_dispatcher.read().await;
                        dispatcher
                            .handle_command(
                                &mut crate::command::CommandSender::Rcon(output_clone),
                                &server_clone,
                                &packet_body,
                            )
                            .await;
                    });

                    let output = output.lock().await;
                    for line in output.iter() {
                        if config.logging.commands {
                            log::info!("RCON ({}): {}", self.address, line);
                        }
                        self.send(ClientboundPacket::Output, packet.get_id(), line)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn read_bytes(&mut self) -> std::io::Result<bool> {
        let mut buf = [0; 1460];
        let n = self.connection.read(&mut buf).await?;
        if n == 0 {
            return Ok(true);
        }
        self.incoming.extend_from_slice(&buf[..n]);
        Ok(false)
    }

    async fn send(
        &mut self,
        packet: ClientboundPacket,
        id: i32,
        body: &str,
    ) -> Result<(), PacketError> {
        let buf = packet.write_buf(id, body);
        self.connection
            .write(&buf)
            .await
            .map_err(PacketError::FailedSend)?;
        Ok(())
    }

    async fn receive_packet(&mut self) -> Result<Option<Packet>, PacketError> {
        Packet::deserialize(&mut self.incoming).await
    }
}

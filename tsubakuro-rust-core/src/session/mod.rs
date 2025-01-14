use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use endpoint::Endpoint;
use log::{debug, trace};
use option::ConnectionOption;
use tcp::TcpConnector;
use wire::Wire;

use crate::{
    error::TgError,
    illegal_argument_error,
    job::Job,
    prelude::{core::CoreService, ServiceClient, ShutdownType},
    tateyama::proto::endpoint::request::ClientInformation,
    util::string_to_prost_string,
};

pub mod endpoint;
pub mod option;
pub(crate) mod tcp;
pub(crate) mod wire;

#[derive(Debug)]
pub struct Session {
    wire: Arc<Wire>,
    default_timeout: Duration,
    shutdowned: AtomicBool,
    fail_on_drop_error: AtomicBool,
}

impl Session {
    pub async fn connect(connection_option: &ConnectionOption) -> Result<Arc<Session>, TgError> {
        let timeout = connection_option.default_timeout();
        Self::connect_for(connection_option, timeout).await
    }

    pub async fn connect_for(
        connection_option: &ConnectionOption,
        timeout: Duration,
    ) -> Result<Arc<Session>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;
        let default_timeout = connection_option.default_timeout();

        match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect(
                    endpoint,
                    connection_option,
                    client_information,
                    timeout,
                    default_timeout,
                )
                .await
            }
            _ => Err(illegal_argument_error!("unsupported endpoint")),
        }
    }

    pub async fn connect_async(
        connection_option: &ConnectionOption,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;
        let default_timeout = connection_option.default_timeout();

        let job = match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect_async(
                    endpoint,
                    connection_option,
                    client_information,
                    default_timeout,
                )
                .await?
            }
            _ => return Err(illegal_argument_error!("unsupported endpoint")),
        };

        Ok(job)
    }

    fn create_information(
        option: &ConnectionOption,
    ) -> Result<(&Endpoint, ClientInformation), TgError> {
        let endpoint = option
            .endpoint()
            .ok_or(illegal_argument_error!("endpoint not specified"))?;

        let client_information = ClientInformation {
            connection_label: string_to_prost_string(option.label()),
            application_name: string_to_prost_string(option.application_name()),
            credential: None, // TODO Crendential
        };
        Ok((endpoint, client_information))
    }

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    pub fn make_client<T: ServiceClient>(self: &Arc<Session>) -> T {
        T::new(self.clone())
    }

    pub async fn update_expiration_time(
        &self,
        expiration_time: Option<Duration>,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.update_expiration_time_for(expiration_time, timeout)
            .await
    }

    pub async fn update_expiration_time_for(
        &self,
        expiration_time: Option<Duration>,
        timeout: Duration,
    ) -> Result<(), TgError> {
        CoreService::update_expiration_time(&self.wire, expiration_time, timeout).await
    }

    pub async fn update_expiration_time_async(
        &self,
        expiration_time: Option<Duration>,
    ) -> Result<Job<()>, TgError> {
        CoreService::update_expiration_time_async(
            &self.wire,
            expiration_time,
            self.default_timeout,
            self.fail_on_drop_error(),
        )
        .await
    }

    pub async fn shutdown(&self, shutdown_type: ShutdownType) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.shutdown_for(shutdown_type, timeout).await
    }

    pub async fn shutdown_for(
        &self,
        shutdown_type: ShutdownType,
        timeout: Duration,
    ) -> Result<(), TgError> {
        self.set_shutdown();
        CoreService::shutdown(&self.wire, shutdown_type, timeout).await
    }

    pub async fn shutdown_async(&self, shutdown_type: ShutdownType) -> Result<Job<()>, TgError> {
        self.set_shutdown();
        CoreService::shutdown_async(
            &self.wire,
            shutdown_type,
            self.default_timeout,
            self.fail_on_drop_error(),
        )
        .await
    }

    fn set_shutdown(&self) {
        self.shutdowned
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_shutdowned(&self) -> bool {
        self.shutdowned.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn close(&self) -> Result<(), TgError> {
        self.wire.close().await
    }

    pub fn is_closed(&self) -> bool {
        self.wire.is_closed()
    }

    // for debug
    pub fn set_fail_on_drop_error(&self, value: bool) {
        self.fail_on_drop_error
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub(crate) fn fail_on_drop_error(&self) -> bool {
        self.fail_on_drop_error
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Session {
    fn new(wire: Arc<Wire>, keep_alive: Duration, default_timeout: Duration) -> Arc<Self> {
        let session = Arc::new(Session {
            wire,
            default_timeout,
            shutdowned: AtomicBool::new(false),
            fail_on_drop_error: AtomicBool::new(false),
        });

        if !keep_alive.is_zero() {
            let wire = session.wire();
            tokio::spawn(async move {
                trace!("session.keep_alive start");
                loop {
                    tokio::time::sleep(keep_alive).await;

                    if wire.is_closed() {
                        trace!("session.keep_alive end");
                        break;
                    }

                    let result =
                        CoreService::update_expiration_time(&wire, None, default_timeout).await;
                    if let Err(error) = result {
                        trace!("session.keep_alive end. {}", error);
                        break;
                    }
                }
            });
        }

        session
    }

    pub(crate) fn wire(&self) -> Arc<Wire> {
        self.wire.clone()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.is_closed() {
            return;
        }

        std::thread::scope(|scope| {
            scope.spawn(move || {
                trace!("Session.drop() start");
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            debug!("Session.drop() runtime::new error. {}", e);
                            if self.fail_on_drop_error() {
                                panic!("Session.drop() runtime::new error. {}", e);
                            }
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    if let Err(e) = self.close().await {
                        debug!("Session.drop() close error. {}", e);
                        if self.fail_on_drop_error() {
                            panic!("Session.drop() close error. {}", e);
                        }
                    }
                });
                trace!("Session.drop() end");
            });
        });
    }
}

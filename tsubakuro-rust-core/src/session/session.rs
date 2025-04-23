use std::{
    sync::{atomic::AtomicBool, Arc, RwLock},
    time::Duration,
};

use log::{error, trace, warn};

use crate::{
    error::TgError,
    illegal_argument_error,
    job::Job,
    prelude::{
        r#type::large_object::{LargeObjectRecvPathMapping, LargeObjectSendPathMapping},
        Endpoint, ShutdownType,
    },
    service::{core::core_service::CoreService, ServiceClient},
    tateyama::proto::endpoint::request::ClientInformation,
    util::string_to_prost_string,
};

use super::{option::ConnectionOption, tcp::connector::TcpConnector, wire::Wire};

/// Represents a connection to Tsurugi server.
///
/// Note: Should invoke [`Self::close`] before [`Self::drop`] to dispose the session.
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example() -> Result<(), TgError> {
///     let mut connection_option = ConnectionOption::new();
///     connection_option.set_endpoint_url("tcp://localhost:12345");
///     connection_option.set_application_name("Tsubakuro/Rust example");
///     connection_option.set_session_label("example session");
///     connection_option.set_default_timeout(std::time::Duration::from_secs(10));
///
///     let session = Session::connect(&connection_option).await?;
///     let client: SqlClient = session.make_client();
///
///     session.close().await;
///     Ok(())
/// }
/// ```
///
/// See [SqlClient](crate::prelude::SqlClient).
#[derive(Debug)]
pub struct Session {
    wire: Arc<Wire>,
    lob_send_path_mapping: LargeObjectSendPathMapping,
    lob_recv_path_mapping: LargeObjectRecvPathMapping,
    default_timeout: RwLock<Duration>,
    shutdowned: AtomicBool,
    fail_on_drop_error: AtomicBool,
}

impl Session {
    /// Establishes a connection to the Tsurugi server.
    ///
    /// Note: Should invoke [`Self::close`] before [`Self::drop`] to dispose the session.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(connection_option: ConnectionOption) -> Result<(), TgError> {
    ///     let session = Session::connect(&connection_option).await?;
    ///     let client: SqlClient = session.make_client();
    ///
    ///     session.close().await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(connection_option: &ConnectionOption) -> Result<Arc<Session>, TgError> {
        let timeout = connection_option.default_timeout();
        Self::connect_for(connection_option, timeout).await
    }

    /// Establishes a connection to the Tsurugi server.
    ///
    /// Note: Should invoke [`Self::close`] before [`Self::drop`] to dispose the session.
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

    /// Establishes a connection to the Tsurugi server.
    ///
    /// Note: Should invoke [`Self::close`] before [`Self::drop`] to dispose the session.
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
            connection_label: string_to_prost_string(option.session_label()),
            application_name: string_to_prost_string(option.application_name()),
            credential: None, // TODO Crendential
        };
        Ok((endpoint, client_information))
    }

    /// Set default timeout.
    pub fn set_default_timeout(&self, timeout: Duration) {
        let mut default_timeout = self.default_timeout.write().unwrap();
        *default_timeout = timeout;
    }

    /// Get default timeout.
    pub fn default_timeout(&self) -> Duration {
        let default_timeout = self.default_timeout.read().unwrap();
        *default_timeout
    }

    /// Creates a service client.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// fn example(session: &Arc<Session>) {
    ///     let client: SqlClient = session.make_client();
    /// }
    /// ```
    pub fn make_client<T: ServiceClient>(self: &Arc<Session>) -> T {
        T::new(self.clone())
    }

    /// Requests to update the session expiration time.
    ///
    /// The resources underlying this session will be disposed after this session was expired.
    /// To extend the expiration time, clients should continue to send requests in this session, or update expiration time explicitly by using this method.
    ///
    /// If the specified expiration time is too long, the server will automatically shorten it to its limit.
    /// Or, if the time is too short or less than zero, the server sometimes omits the request.
    pub async fn update_expiration_time(
        &self,
        expiration_time: Option<Duration>,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout();
        self.update_expiration_time_for(expiration_time, timeout)
            .await
    }

    /// Requests to update the session expiration time.
    ///
    /// The resources underlying this session will be disposed after this session was expired.
    /// To extend the expiration time, clients should continue to send requests in this session, or update expiration time explicitly by using this method.
    ///
    /// If the specified expiration time is too long, the server will automatically shorten it to its limit.
    /// Or, if the time is too short or less than zero, the server sometimes omits the request.
    pub async fn update_expiration_time_for(
        &self,
        expiration_time: Option<Duration>,
        timeout: Duration,
    ) -> Result<(), TgError> {
        CoreService::update_expiration_time(&self.wire, expiration_time, timeout).await
    }

    /// Requests to update the session expiration time.
    ///
    /// The resources underlying this session will be disposed after this session was expired.
    /// To extend the expiration time, clients should continue to send requests in this session, or update expiration time explicitly by using this method.
    ///
    /// If the specified expiration time is too long, the server will automatically shorten it to its limit.
    /// Or, if the time is too short or less than zero, the server sometimes omits the request.
    pub async fn update_expiration_time_async(
        &self,
        expiration_time: Option<Duration>,
    ) -> Result<Job<()>, TgError> {
        CoreService::update_expiration_time_async(
            &self.wire,
            expiration_time,
            self.default_timeout(),
            self.fail_on_drop_error(),
        )
        .await
    }

    pub(crate) fn large_object_path_mapping_on_send(&self) -> &LargeObjectSendPathMapping {
        &self.lob_send_path_mapping
    }

    pub(crate) fn large_object_path_mapping_on_recv(&self) -> &LargeObjectRecvPathMapping {
        &self.lob_recv_path_mapping
    }

    /// Request to shutdown the current session and wait for the running requests were finished.
    pub async fn shutdown(&self, shutdown_type: ShutdownType) -> Result<(), TgError> {
        let timeout = self.default_timeout();
        self.shutdown_for(shutdown_type, timeout).await
    }

    /// Request to shutdown the current session and wait for the running requests were finished.
    pub async fn shutdown_for(
        &self,
        shutdown_type: ShutdownType,
        timeout: Duration,
    ) -> Result<(), TgError> {
        self.set_shutdown();
        CoreService::shutdown(&self.wire, shutdown_type, timeout).await
    }

    /// Request to shutdown the current session and wait for the running requests were finished.
    pub async fn shutdown_async(&self, shutdown_type: ShutdownType) -> Result<Job<()>, TgError> {
        self.set_shutdown();
        CoreService::shutdown_async(
            &self.wire,
            shutdown_type,
            self.default_timeout(),
            self.fail_on_drop_error(),
        )
        .await
    }

    fn set_shutdown(&self) {
        self.shutdowned
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Check if the session is shut down.
    pub fn is_shutdowned(&self) -> bool {
        self.shutdowned.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Disposes the current session.
    ///
    /// This may not wait for complete the ongoing requests, and it may cause the requests may still be in progress after disconnected from the session.
    /// You can use [Self::shutdown] to safely close this session with waiting for complete the ongoing requests, if any.
    ///
    /// Note: Should invoke `close` before [`Self::drop`] to dispose the session.
    pub async fn close(&self) -> Result<(), TgError> {
        self.wire.close().await
    }

    /// Check if the session is closed.
    pub fn is_closed(&self) -> bool {
        self.wire.is_closed()
    }

    /// for debug
    #[doc(hidden)]
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
    pub(crate) fn new(
        wire: Arc<Wire>,
        connection_option: &ConnectionOption,
        default_timeout: Duration,
    ) -> Arc<Self> {
        let session = Arc::new(Session {
            wire,
            lob_send_path_mapping: connection_option
                .large_object_path_mapping_on_send()
                .clone(),
            lob_recv_path_mapping: connection_option
                .large_object_path_mapping_on_recv()
                .clone(),
            default_timeout: RwLock::new(default_timeout),
            shutdowned: AtomicBool::new(false),
            fail_on_drop_error: AtomicBool::new(false),
        });

        let keep_alive = connection_option.keep_alive();
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
                        error!("session.keep_alive end. {}", error);
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
                            error!("Session.drop() runtime::new error. {}", e);
                            if self.fail_on_drop_error() {
                                panic!("Session.drop() runtime::new error. {}", e);
                            }
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    if let Err(e) = self.close().await {
                        warn!("Session.drop() close error. {}", e);
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

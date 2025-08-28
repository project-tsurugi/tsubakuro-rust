use std::sync::Arc;
use std::time::Duration;

use crate::error::TgError;
use crate::job::Job;
use crate::prelude::{ConnectionOption, Credential};
use crate::session::wire::DelegateWire;
use crate::session::{wire::Wire, Session};
use crate::tateyama::proto::endpoint::request::{
    wire_information::StreamInformation, wire_information::WireInformation as WireInformationType,
    ClientInformation, Credential as ProtoCredential, WireInformation,
};

use crate::service::endpoint::endpoint_broker::EndpointBroker;
use crate::util::string_to_prost_string;

use super::link::TcpLink;
use super::wire::TcpWire;

pub(crate) struct TcpConnector {}

impl TcpConnector {
    pub(crate) async fn connect(
        connection_option: &ConnectionOption,
        timeout: Duration,
        default_timeout: Duration,
    ) -> Result<Arc<Session>, TgError> {
        let wire = TcpConnector::create_wire(connection_option).await?;

        let client_information =
            Self::create_client_information(&wire, connection_option, timeout).await?;
        let wire_information = Self::create_wire_information();
        let result =
            EndpointBroker::handshake(&wire, client_information, wire_information, timeout).await?;

        wire.initialize(result)?;

        Ok(Session::new(wire, connection_option, default_timeout))
    }

    pub(crate) async fn connect_async(
        connection_option: &ConnectionOption,
        default_timeout: Duration,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let wire = TcpConnector::create_wire(connection_option).await?;

        let client_information =
            Self::create_client_information(&wire, connection_option, default_timeout).await?;
        let wire_information = Self::create_wire_information();
        let connection_option = connection_option.clone();
        let job = EndpointBroker::handshake_async(
            &wire.clone(),
            client_information,
            wire_information,
            move |result| {
                wire.initialize(result)?;
                Ok(Session::new(
                    wire.clone(),
                    &connection_option,
                    default_timeout,
                ))
            },
            default_timeout,
            false,
        )
        .await?;
        Ok(job)
    }

    async fn create_wire(connection_option: &ConnectionOption) -> Result<Arc<Wire>, TgError> {
        let link = TcpLink::connect(connection_option).await?;
        let wire = TcpWire::new(link);
        let wire = Wire::new(DelegateWire::Tcp(Arc::new(wire)));
        Ok(wire)
    }

    async fn create_client_information(
        wire: &Arc<Wire>,
        option: &ConnectionOption,
        timeout: Duration,
    ) -> Result<ClientInformation, TgError> {
        Ok(ClientInformation {
            connection_label: string_to_prost_string(option.session_label()),
            application_name: string_to_prost_string(option.application_name()),
            credential: to_proto_credential(
                wire,
                option.credential(),
                option.validity_period(),
                timeout,
            )
            .await?,
        })
    }

    fn create_wire_information() -> WireInformation {
        let stream_information = StreamInformation {
            maximum_concurrent_result_sets: 127, // TODO maximum_concurrent_result_sets
        };

        WireInformation {
            wire_information: Some(WireInformationType::StreamInformation(stream_information)),
        }
    }
}

async fn to_proto_credential(
    wire: &Arc<Wire>,
    credential: &Credential,
    validity_period: Duration,
    timeout: Duration,
) -> Result<Option<ProtoCredential>, TgError> {
    use crate::tateyama::proto::endpoint::request::credential::CredentialOpt;

    match credential {
        Credential::Null => Ok(None),
        Credential::UserPassword { .. } => {
            let validity_period_in_seconds = validity_period.as_secs() as i64;
            let expiration_date = if validity_period_in_seconds > 0 {
                Some(chrono::Utc::now() + chrono::Duration::seconds(validity_period_in_seconds))
            } else {
                None
            };
            let json_text = credential.to_json_text(expiration_date)?;
            match wire.encrypt(&json_text, timeout).await? {
                Some(encrypted) => Ok(Some(ProtoCredential {
                    credential_opt: Some(CredentialOpt::EncryptedCredential(encrypted)),
                })),
                None => Ok(None),
            }
        }
        Credential::AuthToken(token) => Ok(Some(ProtoCredential {
            credential_opt: Some(CredentialOpt::RememberMeCredential(token.into())),
        })),
        Credential::File { encrypted, .. } => Ok(Some(ProtoCredential {
            credential_opt: Some(CredentialOpt::EncryptedCredential(encrypted.into())),
        })),
    }
}

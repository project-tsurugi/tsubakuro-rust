#![allow(dead_code)]

use crate::dbc::connect::{
    connect_tsurugi::{TsurugiOdbcConnectArguments, TsurugiOdbcCredentialType},
    connection_string::ConnectionAttributes,
    dsn::read_dsn,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DsnDialogCredentialRadio {
    Nothing,
    Null,
    UserPassword,
    AuthToken,
    File,
}

impl From<TsurugiOdbcCredentialType> for DsnDialogCredentialRadio {
    fn from(value: TsurugiOdbcCredentialType) -> Self {
        use TsurugiOdbcCredentialType::*;
        match value {
            Null => DsnDialogCredentialRadio::Null,
            UserPassword => DsnDialogCredentialRadio::UserPassword,
            AuthToken => DsnDialogCredentialRadio::AuthToken,
            File => DsnDialogCredentialRadio::File,
        }
    }
}

pub(crate) struct DsnDialogValue {
    dsn: Option<String>,
    is_add: bool,
    data_source_name: String,
    data_source_name_editable: bool,
    need_check_data_source_name: bool,
    endpoint: String,
    user: String,
    password: String,
    auth_token: String,
    credential_file: String,
    credential_radio: DsnDialogCredentialRadio,
}

impl std::fmt::Debug for DsnDialogValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DsnDialogValue")
            .field("dsn", &self.dsn)
            .field("is_add", &self.is_add)
            .field("data_source_name", &self.data_source_name)
            .field("data_source_name_editable", &self.data_source_name_editable)
            .field(
                "need_check_data_source_name",
                &self.need_check_data_source_name,
            )
            .field("endpoint", &self.endpoint)
            .field("user", &self.user)
            .field("password", &"****")
            .field("auth_token", &"****")
            .field("credential_file", &self.credential_file)
            .field("credential_radio", &self.credential_radio)
            .finish()
    }
}

impl DsnDialogValue {
    pub(crate) fn new() -> Self {
        Self {
            dsn: None,
            is_add: false,
            data_source_name: String::new(),
            data_source_name_editable: true,
            need_check_data_source_name: true,
            endpoint: String::new(),
            user: String::new(),
            password: String::new(),
            auth_token: String::new(),
            credential_file: String::new(),
            credential_radio: DsnDialogCredentialRadio::Nothing,
        }
    }

    pub(crate) fn from(attributes: ConnectionAttributes, is_add: bool) -> Self {
        let mut dialog_value = Self::new();
        dialog_value.is_add = is_add;

        if let Some(dsn) = attributes.dsn() {
            dialog_value.dsn = Some(dsn.into());
            dialog_value.data_source_name = dsn.into();
            if is_add {
                dialog_value.data_source_name_editable = false;
            }

            if dsn == "Default" && is_add {
                dialog_value.endpoint = "tcp://localhost:12345".into();
            }

            let arg = read_dsn(dsn);
            if let Some(endpoint) = arg.endpoint() {
                dialog_value.endpoint = endpoint.into();
            }
            if let Some(user) = arg.user() {
                dialog_value.user = user.into();
            }
            if let Some(password) = arg.password() {
                dialog_value.password = password.into();
            }
            if let Some(token) = arg.auth_token() {
                dialog_value.auth_token = token.into();
            }
            if let Some(path) = arg.credentials() {
                dialog_value.credential_file = path.into();
            }
            dialog_value.credential_radio = arg.credential_type().into();
        }

        if let Some(endpoint) = attributes.endpoint() {
            dialog_value.endpoint = endpoint.into();
        }
        if let Some(user) = attributes.user() {
            dialog_value.user = user.into();
        }
        if let Some(password) = attributes.password() {
            dialog_value.password = password.into();
        }
        if let Some(token) = attributes.auth_token() {
            dialog_value.auth_token = token.into();
        }
        if let Some(path) = attributes.credentials() {
            dialog_value.credential_file = path.into();
        }

        if dialog_value.credential_radio == DsnDialogCredentialRadio::Nothing {
            dialog_value.credential_radio = if attributes.user().is_some() {
                DsnDialogCredentialRadio::UserPassword
            } else if attributes.auth_token().is_some() {
                DsnDialogCredentialRadio::AuthToken
            } else if attributes.credentials().is_some() {
                DsnDialogCredentialRadio::File
            } else {
                DsnDialogCredentialRadio::Null
            };
        }

        dialog_value
    }

    pub(crate) fn from_connect_arguments(arg: &TsurugiOdbcConnectArguments) -> Self {
        let mut dialog_value = Self::new();

        dialog_value.dsn = arg.dsn().cloned();
        dialog_value.data_source_name = arg.dsn().cloned().unwrap_or_default();
        dialog_value.endpoint = arg.endpoint().cloned().unwrap_or_default();
        dialog_value.user = arg.user().cloned().unwrap_or_default();
        dialog_value.password = arg.password().cloned().unwrap_or_default();
        dialog_value.auth_token = arg.auth_token().cloned().unwrap_or_default();
        dialog_value.credential_file = arg.credentials().cloned().unwrap_or_default();
        dialog_value.credential_radio = arg.credential_type().into();

        dialog_value.data_source_name_editable = false;
        dialog_value.need_check_data_source_name = false;

        dialog_value
    }

    pub fn dsn(&self) -> Option<&String> {
        self.dsn.as_ref()
    }

    pub fn is_add(&self) -> bool {
        self.is_add
    }

    pub fn data_source_name(&self) -> &String {
        &self.data_source_name
    }

    pub fn data_source_name_editable(&self) -> bool {
        self.data_source_name_editable
    }

    pub fn need_check_data_source_name(&self) -> bool {
        self.need_check_data_source_name
    }

    pub fn set_data_source_name(&mut self, name: String) {
        self.data_source_name = name;
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    pub fn user(&self) -> &String {
        &self.user
    }

    pub fn set_user(&mut self, user: String) {
        self.user = user;
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn auth_token(&self) -> &String {
        &self.auth_token
    }

    pub fn set_auth_token(&mut self, auth_token: String) {
        self.auth_token = auth_token;
    }

    pub fn credential_file(&self) -> &String {
        &self.credential_file
    }

    pub fn set_credential_file(&mut self, credential_file: String) {
        self.credential_file = credential_file;
    }

    pub fn credential_radio(&self) -> DsnDialogCredentialRadio {
        self.credential_radio
    }

    pub fn set_credential_radio(&mut self, credential_radio: DsnDialogCredentialRadio) {
        self.credential_radio = credential_radio;
    }

    pub fn is_new_dsn(&self) -> bool {
        if self.dsn.is_none() {
            return true;
        }

        self.is_rename_dsn()
    }

    pub fn is_rename_dsn(&self) -> bool {
        match self.dsn {
            Some(ref old_dsn) => {
                &self.data_source_name != old_dsn && !self.data_source_name.is_empty()
            }
            _ => false,
        }
    }
}

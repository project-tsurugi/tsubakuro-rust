#![allow(dead_code)]

use crate::dbc::connect::{connection_string::ConnectionAttributes, dsn::read_dsn};

#[derive(Debug)]
pub(crate) struct DsnDialogValue {
    driver: String,
    dsn: Option<String>,
    is_add: bool,
    data_source_name: String,
    data_source_name_editable: bool,
    endpoint: String,
}

impl DsnDialogValue {
    fn new(driver: String, is_add: bool) -> Self {
        Self {
            driver,
            dsn: None,
            is_add,
            data_source_name: String::new(),
            data_source_name_editable: true,
            endpoint: String::new(),
        }
    }

    pub fn from(driver: String, attributes: ConnectionAttributes, is_add: bool) -> Self {
        let mut dialog_value = Self::new(driver, is_add);

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
            if let Some(endpoint) = arg.endpoint {
                dialog_value.endpoint = endpoint;
            }
        }

        if let Some(endpoint) = attributes.endpoint() {
            dialog_value.endpoint = endpoint.clone();
        }

        dialog_value
    }

    pub fn driver(&self) -> &String {
        &self.driver
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

    pub fn set_data_source_name(&mut self, name: String) {
        self.data_source_name = name;
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TcpRequestInfo {
    RequestSessionPayload,
    RequestResultSetByeOk,
}

impl From<TcpRequestInfo> for u8 {
    fn from(value: TcpRequestInfo) -> Self {
        match value {
            TcpRequestInfo::RequestSessionPayload => 2,
            TcpRequestInfo::RequestResultSetByeOk => 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum TcpResponseInfo {
    ResponseSessionPayload,
    ResponseResultSetPayload,
    ResponseResultSetHello,
    ResponseResultSetBye,
    ResponseSessionBodyhead,
    ResponseUnknown(u8),
}

impl From<u8> for TcpResponseInfo {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ResponseSessionPayload,
            2 => Self::ResponseResultSetPayload,
            // 3, 4 are nolonger used
            5 => Self::ResponseResultSetHello,
            6 => Self::ResponseResultSetBye,
            7 => Self::ResponseSessionBodyhead,
            v => Self::ResponseUnknown(v),
        }
    }
}

impl TcpResponseInfo {
    pub fn value(self) -> u8 {
        match self {
            Self::ResponseSessionPayload => 1,
            Self::ResponseResultSetPayload => 2,
            // 3, 4 are nolonger used
            Self::ResponseResultSetHello => 5,
            Self::ResponseResultSetBye => 6,
            Self::ResponseSessionBodyhead => 7,
            Self::ResponseUnknown(value) => value,
        }
    }
}

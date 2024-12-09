#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TcpRequestInfo {
    RequestSessionPayload,
    RequestResultSetByeOk,
}

impl Into<u8> for TcpRequestInfo {
    fn into(self) -> u8 {
        match self {
            Self::RequestSessionPayload => 2,
            Self::RequestResultSetByeOk => 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

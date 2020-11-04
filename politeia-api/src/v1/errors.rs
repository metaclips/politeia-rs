#[repr(u8)]
pub enum ErrorCode {
    Invalid = 0,
    StatusProposalNotFound = 6,
    StatusInvalidCensorshipToken = 58,
    StatusMaxProposalsExceededPolicy = 61,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorCode::Invalid => write!(f, "Invalid error code."),
            ErrorCode::StatusProposalNotFound => write!(f, "Proposal not found."),
            ErrorCode::StatusInvalidCensorshipToken => write!(f, "Invalid censorship token."),
            ErrorCode::StatusMaxProposalsExceededPolicy => {
                write!(f, "Max proposal per request exceed.")
            }
        }
    }
}

impl std::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorCode::Invalid => write!(f, "ErrorCode(Invalid error code)"),
            ErrorCode::StatusProposalNotFound => write!(f, "ErrorCode(Proposal not found)"),
            ErrorCode::StatusInvalidCensorshipToken => {
                write!(f, "ErrorCode(Invalid censorship token)")
            }
            ErrorCode::StatusMaxProposalsExceededPolicy => {
                write!(f, "ErrorCode(Max proposal per request exceed)")
            }
        }
    }
}

impl Into<ErrorCode> for u8 {
    fn into(self) -> ErrorCode {
        match self {
            6 => ErrorCode::StatusProposalNotFound,
            58 => ErrorCode::StatusInvalidCensorshipToken,
            61 => ErrorCode::StatusMaxProposalsExceededPolicy,
            _ => ErrorCode::Invalid,
        }
    }
}

impl From<ErrorCode> for u8 {
    fn from(val: ErrorCode) -> Self {
        match val {
            ErrorCode::Invalid => 0,
            ErrorCode::StatusProposalNotFound => 6,
            ErrorCode::StatusInvalidCensorshipToken => 58,
            ErrorCode::StatusMaxProposalsExceededPolicy => 61,
        }
    }
}

/// Proposal state codes
///
/// PropStateUnvetted includes proposals with a status of:
///   * PropStatusNotReviewed
///   * PropStatusUnreviewedChanges
///   * PropStatusCensored
/// PropStateVetted includes proposals with a status of:
///   * PropStatusPublic
///   * PropStatusAbandoned
///
/// Proposal states correspond to the unvetted and vetted politeiad
/// repositories.
#[derive(serde::Deserialize, Debug)]
pub enum ProposalState {
    Invalid,
    Unvetted,
    Vetted,
}

impl Into<ProposalState> for u8 {
    fn into(self) -> ProposalState {
        match self {
            1 => ProposalState::Unvetted,
            2 => ProposalState::Vetted,
            _ => ProposalState::Invalid,
        }
    }
}
impl From<ProposalState> for u8 {
    fn from(val: ProposalState) -> Self {
        match val {
            ProposalState::Unvetted => 1,
            ProposalState::Vetted => 2,
            ProposalState::Invalid => 0,
        }
    }
}

impl Default for ProposalState {
    fn default() -> Self {
        ProposalState::Invalid
    }
}

/// Proposal status codes.
#[derive(serde::Deserialize, Debug)]
pub enum ProposalStatus {
    Invalid,
    NotFound,
    NotReviewed,
    Censored,
    Public,
    UnreviewedChanges,
    Abandoned,
}

impl Default for ProposalStatus {
    fn default() -> Self {
        ProposalStatus::Invalid
    }
}

impl Into<ProposalStatus> for u8 {
    fn into(self) -> ProposalStatus {
        match self {
            1 => ProposalStatus::NotFound,
            2 => ProposalStatus::NotReviewed,
            3 => ProposalStatus::Censored,
            4 => ProposalStatus::Public,
            5 => ProposalStatus::UnreviewedChanges,
            6 => ProposalStatus::Abandoned,
            _ => ProposalStatus::Invalid,
        }
    }
}

impl From<ProposalStatus> for u8 {
    fn from(val: ProposalStatus) -> Self {
        match val {
            ProposalStatus::NotFound => 1,
            ProposalStatus::NotReviewed => 2,
            ProposalStatus::Censored => 3,
            ProposalStatus::Public => 4,
            ProposalStatus::UnreviewedChanges => 5,
            ProposalStatus::Abandoned => 6,
            ProposalStatus::Invalid => 0,
        }
    }
}

#[derive(serde::Deserialize, PartialEq, Debug)]
pub enum ProposalVoteStatus {
    Invalid,
    NotStarted,
    Started,
    Finished,
    DoesntExist,
}

impl Into<ProposalVoteStatus> for u8 {
    fn into(self) -> ProposalVoteStatus {
        match self {
            1 => ProposalVoteStatus::NotStarted,
            2 => ProposalVoteStatus::Started,
            3 => ProposalVoteStatus::Finished,
            4 => ProposalVoteStatus::DoesntExist,
            _ => ProposalVoteStatus::Invalid,
        }
    }
}

impl From<ProposalVoteStatus> for u8 {
    fn from(val: ProposalVoteStatus) -> u8 {
        match val {
            ProposalVoteStatus::NotStarted => 1,
            ProposalVoteStatus::Started => 2,
            ProposalVoteStatus::Finished => 3,
            ProposalVoteStatus::DoesntExist => 4,
            ProposalVoteStatus::Invalid => 0,
        }
    }
}

impl Default for ProposalVoteStatus {
    fn default() -> Self {
        ProposalVoteStatus::Invalid
    }
}

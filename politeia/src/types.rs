use politeia_api::v1::types;

#[derive(Default)]
pub struct Proposals {
    pub pre: types::ProposalsResult,
    pub abandoned: types::ProposalsResult,
    pub active: types::ProposalsResult,
    pub approved: types::ProposalsResult,
    pub censored: types::ProposalsResult,
    pub unreviewed: types::ProposalsResult,
    pub rejected: types::ProposalsResult,
}

#[derive(serde::Deserialize, Debug)]
pub struct Tokens {
    pub tokens: Vec<String>,
}

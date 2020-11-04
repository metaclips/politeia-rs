use politeia_api as api;
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::RwLock, time};

pub struct Client {
    client: reqwest::Client,

    csrf_token: String,
    csrf_expiry: std::time::Instant,

    pub policy: api::v1::types::Policy,
}

const CSRF_TOKEN: &str = "X-CSRF-Token";
const MAX_CSRF_EXPIRY_SECS: u64 = 20 * 60 * 60;
const MAX_TIME_CACHE_UPDATE_SECS: u64 = 10 * 60;

impl Client {
    pub fn new() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
        let req_client = reqwest::ClientBuilder::default()
            .connection_verbose(true)
            .cookie_store(true)
            .build()?;

        let client = Client {
            client: req_client,
            csrf_token: String::new(),
            csrf_expiry: std::time::Instant::now(),
            policy: api::v1::types::Policy::default(),
        };

        Ok(client)
    }

    async fn get_request(
        &self,
        url: String,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.client.get(&url).send().await?;
        match response.error_for_status() {
            Ok(res) => Ok(res.bytes().await?.to_vec()),

            Err(e) => Err(e.into()),
        }
    }

    async fn post_request(
        &mut self,
        url: String,
        params: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        if self.csrf_token == ""
            || self.csrf_expiry.elapsed() > std::time::Duration::from_secs(MAX_CSRF_EXPIRY_SECS)
        {
            log::info!("Updating csrf.");
            self.version().await?;
            self.policy = self.fetch_policy().await?;
            self.csrf_expiry = std::time::Instant::now();
        }

        let response = self
            .client
            .post(&url)
            .header(CSRF_TOKEN, &self.csrf_token)
            .body(params)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(res) => Ok(res.bytes().await?.to_vec()),

            Err(e) => Err(e.into()),
        }
    }

    async fn version(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}{}",
            api::POLITEIA_HOST,
            api::v1::routes::REQUEST_GET_VERSION
        );

        let response = self.client.get(&url).send().await?;

        match response.headers().get(CSRF_TOKEN) {
            Some(e) => {
                self.csrf_token = e.to_str()?.to_string();
            }

            None => return Err("Error retrieving CSRF-Token".into()),
        }

        Ok(())
    }

    pub async fn fetch_policy(
        &self,
    ) -> Result<api::v1::types::Policy, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}{}",
            api::POLITEIA_HOST,
            api::v1::routes::REQUEST_GET_POLICY
        );

        let response = self.get_request(url).await?;
        let policy: api::v1::types::Policy = serde_json::from_slice(&response)?;
        Ok(policy)
    }

    pub async fn fetch_tokens(
        &self,
    ) -> Result<politeia_api::v1::types::TokenInventory, Box<dyn std::error::Error + Send + Sync>>
    {
        let url = format!(
            "{}{}",
            api::POLITEIA_HOST,
            api::v1::routes::REQUEST_GET_TOKEN_INVENTORY
        );

        let response = self.get_request(url).await?;
        let tokens: api::v1::types::TokenInventory = serde_json::from_slice(&response)?;
        Ok(tokens)
    }

    pub async fn fetch_batch_proposal(
        &mut self,
        tokens: Vec<String>,
    ) -> Result<politeia_api::v1::types::ProposalsResult, Box<dyn std::error::Error + Send + Sync>>
    {
        let url = format!(
            "{}{}",
            api::POLITEIA_HOST,
            api::v1::routes::REQUEST_POST_BATCH_PROPOSALS
        );

        log::info!("Fetching batch proposal");

        let val = serde_json::json!({ "tokens": tokens });
        let params = serde_json::to_vec(&val)?;

        let response = self.post_request(url, params).await?;

        let policy: api::v1::types::ProposalsResult = serde_json::from_slice(&response)?;
        Ok(policy)
    }

    pub async fn fetch_all_proposals(
        &mut self,
        mut tokens: api::v1::types::TokenInventory,
    ) -> Result<super::types::Proposals, Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Fetching all proposals");

        let mut proposals = super::types::Proposals::default();

        self.fetch_proposal(&mut tokens.pre, &mut proposals.pre)
            .await?;
        self.fetch_proposal(&mut tokens.abandoned, &mut proposals.abandoned)
            .await?;
        self.fetch_proposal(&mut tokens.active, &mut proposals.active)
            .await?;
        self.fetch_proposal(&mut tokens.approved, &mut proposals.approved)
            .await?;
        self.fetch_proposal(&mut tokens.censored, &mut proposals.censored)
            .await?;
        self.fetch_proposal(&mut tokens.unreviewed, &mut proposals.unreviewed)
            .await?;
        self.fetch_proposal(&mut tokens.rejected, &mut proposals.rejected)
            .await?;

        Ok(proposals)
    }

    async fn fetch_proposal(
        &mut self,
        tokens: &mut Vec<String>,
        proposals: &mut api::v1::types::ProposalsResult,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Fetch proposals also not exceeding max proposal list to fetch.
        while !tokens.is_empty() {
            let split_at = if (tokens.len() as i64 - self.policy.proposal_list_page_size as i64) < 0
            {
                tokens.len() - 1
            } else {
                tokens.len() - self.policy.proposal_list_page_size
            };

            let a = self
                .fetch_batch_proposal(tokens.split_off(split_at))
                .await?;
            let iter = a.proposals.iter();

            for prop in iter {
                proposals.proposals.push(prop.clone())
            }
        }

        Ok(())
    }
}

/// Update proposals at 10minute intervals.
pub(crate) async fn update_proposals(
    tokens: Arc<RwLock<api::v1::types::TokenInventory>>,
    policy: Arc<RwLock<api::v1::types::Policy>>,
    proposal_mapper: Arc<RwLock<HashMap<String, api::v1::types::Proposal>>>,
) {
    log::trace!("Starting cache store updater.");

    let mut client = Client::new().unwrap();

    loop {
        log::trace!("Updating cache store.");
        let new_tokens = match client.fetch_tokens().await {
            Ok(e) => e,
            Err(e) => {
                log::error!("Error fetching proposal tokens, error: {}", e);
                log::trace!(
                    "Retrying update cache in {} seconds",
                    MAX_TIME_CACHE_UPDATE_SECS
                );
                time::delay_for(time::Duration::from_secs(MAX_TIME_CACHE_UPDATE_SECS)).await;
                continue;
            }
        };

        let old_tokens = tokens.read().await;
        if *old_tokens != new_tokens {
            drop(old_tokens);

            let mut tokens = tokens.write().await;
            *tokens = new_tokens.clone();
        }

        let new_proposals = match client.fetch_all_proposals(new_tokens).await {
            Ok(e) => e,
            Err(e) => {
                log::error!("Error fetching proposals, error: {}", e);
                log::trace!(
                    "Retrying update cache in {} seconds",
                    MAX_TIME_CACHE_UPDATE_SECS
                );
                time::delay_for(time::Duration::from_secs(MAX_TIME_CACHE_UPDATE_SECS)).await;
                continue;
            }
        };

        let is_proposal_same = proposals_is_same(new_proposals.clone(), &proposal_mapper).await;

        if !is_proposal_same {
            set_proposals(new_proposals, &proposal_mapper).await;
        }

        if client.policy != *policy.read().await {
            *policy.write().await = client.policy.clone();
        }

        log::trace!("Updating cache in {} seconds", MAX_TIME_CACHE_UPDATE_SECS);
        time::delay_for(time::Duration::from_secs(MAX_TIME_CACHE_UPDATE_SECS)).await;
    }
}

async fn proposals_is_same(
    new_proposal: super::types::Proposals,
    proposal_mapper: &Arc<RwLock<HashMap<String, api::v1::types::Proposal>>>,
) -> bool {
    let proposal_mapper = proposal_mapper.read().await;

    // Get proposal from mapper and check if same.
    let getter = |proposal: api::v1::types::ProposalsResult| -> bool {
        for proposal in proposal.proposals {
            let token = &proposal.censorship_record.token;

            match proposal_mapper.get(token.as_str()) {
                Some(e) => {
                    if *e != proposal {
                        return false;
                    }

                    continue;
                }

                // Token not found so a new proposal has to be retrieved.
                None => return false,
            };
        }

        true
    };

    if !getter(new_proposal.abandoned) {
        return false;
    };
    if !getter(new_proposal.active) {
        return false;
    };
    if !getter(new_proposal.approved) {
        return false;
    };
    if !getter(new_proposal.censored) {
        return false;
    };
    if !getter(new_proposal.pre) {
        return false;
    };
    if !getter(new_proposal.rejected) {
        return false;
    };
    if !getter(new_proposal.unreviewed) {
        return false;
    };

    true
}

async fn set_proposals(
    new_proposal: super::types::Proposals,
    proposal_mapper: &Arc<RwLock<HashMap<String, api::v1::types::Proposal>>>,
) {
    let mut proposal_mapper = proposal_mapper.write().await;

    let mut setter = |proposal: api::v1::types::ProposalsResult| {
        for proposal in proposal.proposals {
            let token = proposal.censorship_record.token.clone();

            proposal_mapper.insert(token, proposal);
        }
    };

    setter(new_proposal.abandoned);
    setter(new_proposal.active);
    setter(new_proposal.approved);
    setter(new_proposal.censored);
    setter(new_proposal.pre);
    setter(new_proposal.rejected);
    setter(new_proposal.unreviewed);
}

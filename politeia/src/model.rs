use politeia_api as api;

pub struct Client {
    client: reqwest::Client,

    csrf_token: String,

    pub policy: api::v1::types::Policy,
}

const CSRF_TOKEN: &str = "X-CSRF-Token";

pub fn new() -> Result<Client, Box<dyn std::error::Error>> {
    let req_client = reqwest::ClientBuilder::default()
        .connection_verbose(true)
        .build()?;

    let client = Client {
        client: req_client,
        csrf_token: String::new(),
        policy: api::v1::types::Policy::default(),
    };

    Ok(client)
}

impl Client {
    async fn get_request(&self, url: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.version().await?;
        self.policy = self.fetch_policy().await?;

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

    async fn version(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn fetch_policy(&self) -> Result<api::v1::types::Policy, Box<dyn std::error::Error>> {
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
    ) -> Result<politeia_api::v1::types::TokenInventory, Box<dyn std::error::Error>> {
        let url = format!(
            "{}{}",
            api::POLITEIA_HOST,
            api::v1::routes::REQUEST_GET_TOKEN_INVENTORY
        );

        let response = self.get_request(url).await?;
        let policy: api::v1::types::TokenInventory = serde_json::from_slice(&response)?;
        Ok(policy)
    }

    pub async fn fetch_batch_proposal(
        &mut self,
        tokens: Vec<String>,
    ) -> Result<politeia_api::v1::types::ProposalsResult, Box<dyn std::error::Error>> {
        let url = format!(
            "{}{}",
            api::POLITEIA_HOST,
            api::v1::routes::REQUEST_POST_BATCH_PROPOSALS
        );

        let val = serde_json::json!({ "tokens": tokens });
        let params = serde_json::to_vec(&val)?;

        let response = self.post_request(url, params).await?;

        let policy: api::v1::types::ProposalsResult = serde_json::from_slice(&response)?;
        Ok(policy)
    }

    pub async fn fetch_all_proposals(
        &mut self,
        mut tokens: api::v1::types::TokenInventory,
    ) -> Result<super::types::Proposals, Box<dyn std::error::Error>> {
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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

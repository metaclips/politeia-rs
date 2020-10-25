/// Retrieves the vote status of all public proposals.
pub const REQUEST_GET_VOTE_STATUS: &str = "/v1/proposals/votestatus";
/// Retrieve the censorship record tokens of all proposals in the inventory.
pub const REQUEST_GET_TOKEN_INVENTORY: &str = "/v1/proposals/tokeninventory";
/// Retrieve server policy.
pub const REQUEST_GET_POLICY: &str = "/v1/policy";
/// Retrieve version, route information and signing identity.
pub const REQUEST_GET_VERSION: &str = "/v1/version";
/// Retrieve politeia proposals provided tokens.
pub const REQUEST_POST_BATCH_PROPOSALS: &str = "/v1/proposals/batch";

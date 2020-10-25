#![allow(deprecated)]
/// Obtain version, route information and signing identity from server.
///
/// This call shall ALWAYS be the first contact with the server.
/// This is done in order to get the CSRF token for the session
/// and to ensure API compatibility.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Version {
    pub version: String,
    pub route: String,
    pub pubkey: String,
    pub testnet: bool,
    pub mode: String,
    #[serde(rename = "activeusersesstion")]
    pub active_user_sesstion: String,
}

/// Describes a single vote option.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct VoteOption {
    /// Single unique word identifying vote (e.g. yes)
    pub id: String,
    /// Longer description of the vote.
    pub description: String,
    /// Bits used for this option
    pub bits: u64,
}

/// Describes a [VotingOption] along with the number of votes it has received.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct VoteOptionResult {
    /// Vote Option.
    pub option: VoteOption,
    /// Number of votes received by the option
    #[serde(rename = "votesreceived")]
    pub votes_received: u64,
}

/// Describes the vote status for a given proposal.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct VoteStatus {
    /// Token that identifies vote.
    pub token: String,
    /// Proposal status.
    pub status: u8,
    /// Proposal's total number of votes.
    #[serde(rename = "totalvotes")]
    pub total_votes: u64,
    /// Vote end .
    #[serde(rename = "endheight")]
    pub end_height: String,
    /// Current best block height.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Total number of eligible votes.
    #[serde(rename = "numofeligiblevotes")]
    pub number_of_eligible_votes: u64,
    /// Percent of eligible votes required for quorum.
    #[serde(rename = "quorumpercentage")]
    pub quorom_percentage: u32,
    /// Percent of total votes required to pass.
    #[serde(rename = "passpercentage")]
    pub pass_percentage: u32,
    /// Vote options.
    #[serde(rename = "optionsresult")]
    pub options_result: Vec<VoteOptionResult>,
}

/// Returns the [VoteStatus] of all public proposals.
#[deprecated = "This type has been deprecated, use Batch Vote type"]
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct VoteStatusResult {
    #[serde(rename = "votesstatus")]
    pub vote_status: Vec<VoteStatus>,
}

/// Retrieve the censorship record tokens of all proposals in the inventory.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct TokenInventory {
    /// Tokens of all vetted proposals that are pre-vote/in-discussion.
    pub pre: Vec<String>,
    /// Tokens of all vetted proposals with an active voting period.
    pub active: Vec<String>,
    /// Tokens of all vetted proposals that have been approved by a vote.
    pub approved: Vec<String>,
    /// Tokens of all vetted proposals that have been rejected by a vote.
    pub rejected: Vec<String>,
    /// Tokens of all vetted proposals that have been abandoned.
    pub abandoned: Vec<String>,
    /// Tokens of all unreviewed proposals.
    pub unreviewed: Vec<String>,
    /// Tokens of all censored proposals.
    pub censored: Vec<String>,
}

/// Describes an individual file that is part of the proposal.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct File {
    /// File name.
    pub name: String,
    /// Mime type.
    pub mime: String,
    /// Digest of unencoded payload in SHA256.
    pub digest: String,
    /// File content, base64 encoded.
    pub payload: String,
}

/// Describes user specified metadata.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Metadata {
    /// SHA256 digest of JSON encoded payload.
    pub digest: String,
    /// Hint that describes the payload.
    pub hint: String,
    /// Base64 encoded metadata content.
    pub payload: String,
}

/// Contains the proof that a proposal was accepted for review.
/// The proof is verifiable on the client side.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct CensorshipRecord {
    /// Censorship token.
    token: String,
    /// Merkle root of proposal.
    merkle: String,
    /// Server side signature of byte array representation of Merkle+Token.
    signature: String,
}

/// Politeia proposals.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Proposal {
    /// Short proposal name.
    pub name: String,
    /// Current state of proposal.
    #[serde(rename = "state")]
    pub proposal_state: i64,
    /// Current status of proposal.
    #[serde(rename = "status")]
    pub proposal_status: i64,
    /// Last update of proposal.
    pub timestamp: i64,
    /// ID of user who submitted proposal.
    #[serde(rename = "userid")]
    pub user_id: String,
    /// Username of user who submitted proposal.
    pub username: String,
    /// Key used for signature.
    #[serde(rename = "publickey")]
    pub public_key: String,
    /// Signature of merkle root.
    pub signature: String,
    /// Number of comments on the proposal.
    #[serde(rename = "numcomments")]
    pub number_of_comments: usize,
    /// Record version.
    pub version: String,
    /// Message associated to the status change.
    #[serde(rename = "statuschangemessage")]
    pub status_change_message: String,
    /// UNIX timestamp of when proposal was published.
    #[serde(rename = "publishedat")]
    pub pubished_at: i64,
    /// UNIX timestamp of when proposal was censored.
    #[serde(rename = "censoredat")]
    pub censored_at: i64,
    /// UNIX timestamp of when proposal was abandoned.
    #[serde(rename = "abandonedat")]
    pub abandoned_at: i64,
    /// Token of linked parent proposal.
    #[serde(rename = "linkto")]
    pub link_to: String,
    /// UNIX timestamp of RFP deadline.
    #[serde(rename = "linkby")]
    pub link_by: i64,
    /// Proposal files.
    pub files: Vec<File>,
    /// Proposal metadata.
    pub metadata: Vec<Metadata>,
    /// Proposal censorship record.
    #[serde(rename = "censorshiprecord")]
    pub censorship_record: CensorshipRecord,
}

/// Returns all Politeia [Proposal]s.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct ProposalsResult {
    pub proposals: Vec<Proposal>,
}

/// Retrieve server policy.
///
/// The returned values contain various maxima that the client SHALL observe.
#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Policy {
    /// Minimum number of characters accepted for user passwords.
    #[serde(rename = "minpasswordlength")]
    pub minimum_password_length: usize,
    /// Minimum number of characters accepted for username.
    #[serde(rename = "minusernamelength")]
    pub minimum_username_length: usize,
    /// Maximum number of characters accepted for username.
    #[serde(rename = "maxusernamelength")]
    pub maximum_username_length: usize,
    /// The regular expression of a valid username.
    #[serde(rename = "usernamesupportedchars")]
    pub username_supported_chars: Vec<String>,
    /// Is paywall enabled.
    #[serde(rename = "paywallenabled")]
    pub paywall_enabled: bool,
    /// Maximum number of proposals returned for the routes that return lists of proposals.
    #[serde(rename = "proposallistpagesize")]
    pub proposal_list_page_size: usize,
    /// Maximum number of users returned for the routes that return lists of users.
    #[serde(rename = "userlistpagesize")]
    pub user_list_page_size: usize,
    /// Maximum number of images accepted when creating a new proposal.
    #[serde(rename = "maximages")]
    pub max_images: usize,
    /// Maximum image file size (in bytes) accepted when creating a new proposal.
    #[serde(rename = "maximagesize")]
    pub max_image_size: usize,
    /// Maximum number of markdown files accepted when creating a new proposal.
    #[serde(rename = "maxmds")]
    pub max_markdowns: usize,
    /// Maximum markdown file size (in bytes) accepted when creating a new proposal.
    #[serde(rename = "maxmdsize")]
    pub max_markdown_size: usize,
    /// List of all acceptable MIME types that can be communicated between client and server.
    #[serde(rename = "validmimetypes")]
    pub valid_mime_types: Vec<String>,
    /// Max length of a proposal name.
    #[serde(rename = "maxproposalnamelength")]
    pub max_proposal_name_length: usize,
    /// Min length of a proposal name.
    #[serde(rename = "minproposalnamelength")]
    pub min_proposal_name_length: usize,
    /// The regular expression of a valid proposal name.
    #[serde(rename = "proposalnamesupportedchars")]
    pub proposal_name_supported_chars: Vec<String>,
    /// Maximum number of characters accepted for comments.
    #[serde(rename = "maxcommentlength")]
    pub max_comment_length: usize,
    /// The length of token prefix needed.
    #[serde(rename = "backendpublickey")]
    pub backend_public_key: String,
    /// The length of token prefix needed.
    #[serde(rename = "tokenprefixlength")]
    pub token_prefix_length: usize,
    /// Build information including module commit hashes.
    #[serde(rename = "buildinformation")]
    pub build_information: Vec<String>,
    /// Required filename for the proposal index.md file.
    #[serde(rename = "indexfilename")]
    pub index_filename: String,
    /// Minimum required period, in seconds, for the proposal linkby period.
    #[serde(rename = "minlinkbyperiod")]
    pub min_link_by_period: usize,
    /// Maximum allowed period, in seconds, for the proposal linkby period.
    #[serde(rename = "maxlinkbyperiod")]
    pub max_link_by_period: usize,
    /// Minimum allowed vote duration.
    #[serde(rename = "minvoteduration")]
    pub min_vote_duration: usize,
    /// Maximum allowed vote duration.
    #[serde(rename = "maxvoteduration")]
    pub max_vote_duration: usize,
}

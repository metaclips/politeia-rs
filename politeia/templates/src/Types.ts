/// Politeia proposals.
export interface Proposal {
    /// Short proposal name.
    name: String;
    /// Current state of proposal.
    state: number;
    /// Current status of proposal.
    status: number;
    /// Last update of proposal.
    timestamp: number;
    /// ID of user who submitted proposal.
    userid: string;
    /// Username of user who submitted proposal.
    username: string;
    /// Key used for signature.
    publickey: String;
    /// Signature of merkle root.
    signature: string;
    /// Number of comments on the proposal.
    numcomments: number;
    /// Record version.
    version: string;
    /// Message associated to the status change.
    statuschangemessage: string;
    /// UNIX timestamp of when proposal was published.
    publishedat: number;
    /// UNIX timestamp of when proposal was censored.
    censoredat: number;
    /// UNIX timestamp of when proposal was abandoned.
    abandonedat: number;
    /// Token of linked parent proposal.
    linkto: string;
    /// UNIX timestamp of RFP deadline.
    linkby: number;
    /// Proposal files.
    files: [File];
    /// Proposal metadata.
    metadata: [Metadata];
    /// Proposal censorship record.
    censorshiprecord: CensorshipRecord;
}

export interface File {
    /// File name.
    name: string;
    /// Mime type.
    mime: string;
    /// Digest of unencoded payload in SHA256.
    digest: string;
    /// File content, base64 encoded.
    payload: string;
}

/// Describes user specified metadata
export interface Metadata {
    /// SHA256 digest of JSON encoded payload.
    digest: string;
    /// Hint that describes the payload.
    hint: string;
    /// Base64 encoded metadata content.
    payload: string;
}

/// Contains the proof that a proposal was accepted for review.
/// The proof is verifiable on the client side.
export interface CensorshipRecord {
    /// Censorship token.
    token: string;
    /// Merkle root of proposal.
    merkle: string;
    /// Server side signature of byte array representation of Merkle+Token.
    signature: string;
}
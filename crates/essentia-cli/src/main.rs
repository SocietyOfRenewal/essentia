use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use reqwest::Client;

use essentia_core::{
    sign_transaction, AccountView, Block, BlocksQuery, KeyFile, NodeStatus, PeerRegistration,
    ProposalKind, ProposeBlockResponse, ReviewScore, RiskBand, Role, RubricDimension,
    SignedTransaction, StateDump, SubmitTxRequest, SubmitTxResponse, Transaction,
    TransactionKind, generate_key_file,
};

#[derive(Debug, Parser)]
#[command(name = "essentia-cli", version, about = "Essentia v0.1.0 prototype CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Keygen {
        #[arg(long)]
        out: String,
    },
    InspectKey {
        #[arg(long)]
        key: String,
    },
    Query {
        #[command(subcommand)]
        command: QueryCommand,
    },
    Peer {
        #[command(subcommand)]
        command: PeerCommand,
    },
    Block {
        #[command(subcommand)]
        command: BlockCommand,
    },
    Tx {
        #[command(subcommand)]
        command: TxCommand,
    },
}

#[derive(Debug, Subcommand)]
enum QueryCommand {
    Status {
        #[arg(long)]
        node: String,
    },
    State {
        #[arg(long)]
        node: String,
    },
    Account {
        #[arg(long)]
        node: String,
        #[arg(long)]
        did: String,
    },
    Blocks {
        #[arg(long)]
        node: String,
        #[arg(long)]
        from: Option<u64>,
    },
}

#[derive(Debug, Subcommand)]
enum PeerCommand {
    Register {
        #[arg(long)]
        node: String,
        #[arg(long)]
        peer: String,
    },
}

#[derive(Debug, Subcommand)]
enum BlockCommand {
    Propose {
        #[arg(long)]
        node: String,
    },
}

#[derive(Debug, Subcommand)]
enum TxCommand {
    RegisterDid {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        metadata: Option<String>,
    },
    IssuePersonhood {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        did: String,
        #[arg(long)]
        nullifier: String,
        #[arg(long)]
        eligible: bool,
    },
    IssueRole {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        did: String,
        #[arg(long)]
        role: String,
    },
    CreateBudget {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        global_essent_cap: u64,
        #[arg(long)]
        freedom_floor_units: u64,
        #[arg(long)]
        vendor_redemption_pool: u64,
        #[arg(long)]
        civic_gas_pool: u64,
    },
    CreatePurpose {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        purpose_id: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        name: String,
        #[arg(long)]
        description_hash: String,
        #[arg(long)]
        essent_budget: u64,
    },
    CreateQuest {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        quest_id: String,
        #[arg(long)]
        purpose_id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        reward_ceiling: u64,
        #[arg(long)]
        challenge_window_blocks: u64,
        #[arg(long)]
        audit_tail_bps: u16,
        #[arg(long)]
        audit_tail_blocks: u64,
        #[arg(long)]
        risk_band: String,
        #[arg(long = "rubric")]
        rubric: Vec<String>,
    },
    SubmitClaim {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        claim_id: String,
        #[arg(long)]
        quest_id: String,
        #[arg(long)]
        evidence_root: String,
        #[arg(long)]
        metadata_hash: String,
        #[arg(long)]
        bond: u64,
    },
    SubmitReview {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        claim_id: String,
        #[arg(long)]
        summary_hash: String,
        #[arg(long = "score")]
        scores: Vec<String>,
    },
    ChallengeClaim {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        claim_id: String,
        #[arg(long)]
        challenge_hash: String,
        #[arg(long)]
        bond: u64,
    },
    ResolveChallenge {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        claim_id: String,
        #[arg(long)]
        accepted: bool,
        #[arg(long)]
        note_hash: String,
    },
    FinalizeClaim {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        claim_id: String,
    },
    TransferEssent {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        memo: Option<String>,
    },
    IssueFreedomFloor {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        expires_at: u64,
    },
    SpendUnits {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        vendor: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        memo: Option<String>,
    },
    RedeemVendor {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        amount: u64,
    },
    CreateProposal {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        proposal_id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        body_hash: String,
        #[arg(long)]
        kind: String,
        #[arg(long)]
        opens_at: u64,
        #[arg(long)]
        closes_at: u64,
    },
    CastVote {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        proposal_id: String,
        #[arg(long)]
        seq: u64,
        #[arg(long)]
        yes: bool,
    },
    TallyProposal {
        #[arg(long)]
        node: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        proposal_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let http = Client::new();

    match cli.command {
        Command::Keygen { out } => {
            let key_file = generate_key_file();
            write_json(&out, &key_file)?;
            print_json(&key_file)?;
        }
        Command::InspectKey { key } => {
            let key_file: KeyFile = read_json(&key)?;
            print_json(&key_file)?;
        }
        Command::Query { command } => match command {
            QueryCommand::Status { node } => {
                let status = fetch_status(&http, &node).await?;
                print_json(&status)?;
            }
            QueryCommand::State { node } => {
                let state = fetch_state(&http, &node).await?;
                print_json(&state)?;
            }
            QueryCommand::Account { node, did } => {
                let account = fetch_account(&http, &node, &did).await?;
                print_json(&account)?;
            }
            QueryCommand::Blocks { node, from } => {
                let blocks = fetch_blocks(&http, &node, from).await?;
                print_json(&blocks)?;
            }
        },
        Command::Peer { command } => match command {
            PeerCommand::Register { node, peer } => {
                let response = register_peer(&http, &node, &peer).await?;
                print_json(&response)?;
            }
        },
        Command::Block { command } => match command {
            BlockCommand::Propose { node } => {
                let response = propose_block(&http, &node).await?;
                print_json(&response)?;
            }
        },
        Command::Tx { command } => match command {
            TxCommand::RegisterDid { node, key, metadata } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::RegisterDid { metadata },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::IssuePersonhood {
                node,
                key,
                did,
                nullifier,
                eligible,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::IssuePersonhood {
                        did,
                        nullifier_commitment: nullifier,
                        eligible,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::IssueRole { node, key, did, role } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let role = role.parse::<Role>().map_err(anyhow::Error::msg)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::IssueRole { did, role },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::CreateBudget {
                node,
                key,
                epoch,
                global_essent_cap,
                freedom_floor_units,
                vendor_redemption_pool,
                civic_gas_pool,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::CreateEpochBudget {
                        epoch,
                        global_essent_mint_cap: global_essent_cap,
                        freedom_floor_units_pool: freedom_floor_units,
                        vendor_redemption_pool_e: vendor_redemption_pool,
                        civic_gas_pool_e: civic_gas_pool,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::CreatePurpose {
                node,
                key,
                purpose_id,
                epoch,
                name,
                description_hash,
                essent_budget,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::CreatePurpose {
                        purpose_id,
                        epoch,
                        name,
                        description_hash,
                        essent_budget,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::CreateQuest {
                node,
                key,
                quest_id,
                purpose_id,
                title,
                reward_ceiling,
                challenge_window_blocks,
                audit_tail_bps,
                audit_tail_blocks,
                risk_band,
                rubric,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let risk_band = risk_band.parse::<RiskBand>().map_err(anyhow::Error::msg)?;
                let rubric = parse_rubric(&rubric)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::CreateQuest {
                        quest_id,
                        purpose_id,
                        title,
                        reward_ceiling,
                        challenge_window_blocks,
                        audit_tail_bps,
                        audit_tail_blocks,
                        risk_band,
                        rubric,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::SubmitClaim {
                node,
                key,
                claim_id,
                quest_id,
                evidence_root,
                metadata_hash,
                bond,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::SubmitClaim {
                        claim_id,
                        quest_id,
                        evidence_root,
                        metadata_hash,
                        claimant_bond: bond,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::SubmitReview {
                node,
                key,
                claim_id,
                summary_hash,
                scores,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let scores = parse_scores(&scores)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::SubmitReview {
                        claim_id,
                        summary_hash,
                        scores,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::ChallengeClaim {
                node,
                key,
                claim_id,
                challenge_hash,
                bond,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::ChallengeClaim {
                        claim_id,
                        challenge_hash,
                        bond,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::ResolveChallenge {
                node,
                key,
                claim_id,
                accepted,
                note_hash,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::ResolveChallenge {
                        claim_id,
                        accepted,
                        note_hash,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::FinalizeClaim { node, key, claim_id } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::FinalizeClaim { claim_id },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::TransferEssent { node, key, to, amount, memo } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::TransferEssent { to, amount, memo },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::IssueFreedomFloor {
                node,
                key,
                to,
                epoch,
                amount,
                expires_at,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::IssueFreedomFloor {
                        to,
                        epoch,
                        amount,
                        expires_at,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::SpendUnits { node, key, vendor, amount, memo } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::SpendEssentialUnits { vendor, amount, memo },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::RedeemVendor { node, key, epoch, amount } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::RedeemVendorSettlement { epoch, amount },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::CreateProposal {
                node,
                key,
                proposal_id,
                title,
                body_hash,
                kind,
                opens_at,
                closes_at,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let kind = parse_proposal_kind(&kind)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::CreateProposal {
                        proposal_id,
                        title,
                        body_hash,
                        kind,
                        opens_at,
                        closes_at,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::CastVote {
                node,
                key,
                proposal_id,
                seq,
                yes,
            } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::CastVote {
                        proposal_id,
                        seq,
                        yes,
                    },
                )
                .await?;
                print_json(&response)?;
            }
            TxCommand::TallyProposal { node, key, proposal_id } => {
                let key_file = read_json::<KeyFile>(&key)?;
                let response = send_kind(
                    &http,
                    &node,
                    &key_file,
                    TransactionKind::TallyProposal { proposal_id },
                )
                .await?;
                print_json(&response)?;
            }
        },
    }

    Ok(())
}

async fn fetch_status(http: &Client, node: &str) -> Result<NodeStatus> {
    let endpoint = format!("{}/v1/status", normalize_node(node));
    let response = http.get(endpoint).send().await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<NodeStatus>().await?)
}

async fn fetch_state(http: &Client, node: &str) -> Result<StateDump> {
    let endpoint = format!("{}/v1/state", normalize_node(node));
    let response = http.get(endpoint).send().await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<StateDump>().await?)
}

async fn fetch_account(http: &Client, node: &str, did: &str) -> Result<AccountView> {
    let endpoint = format!("{}/v1/accounts/{}", normalize_node(node), did);
    let response = http.get(endpoint).send().await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<AccountView>().await?)
}

async fn fetch_blocks(http: &Client, node: &str, from: Option<u64>) -> Result<Vec<Block>> {
    let endpoint = format!("{}/v1/blocks", normalize_node(node));
    let response = http
        .get(endpoint)
        .query(&BlocksQuery { from })
        .send()
        .await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<Vec<Block>>().await?)
}

async fn propose_block(http: &Client, node: &str) -> Result<ProposeBlockResponse> {
    let endpoint = format!("{}/v1/blocks/propose", normalize_node(node));
    let response = http.post(endpoint).send().await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<ProposeBlockResponse>().await?)
}

async fn register_peer(http: &Client, node: &str, peer: &str) -> Result<PeerRegistration> {
    let endpoint = format!("{}/v1/peers/register", normalize_node(node));
    let response = http
        .post(endpoint)
        .json(&PeerRegistration {
            url: peer.to_string(),
        })
        .send()
        .await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<PeerRegistration>().await?)
}

async fn send_kind(
    http: &Client,
    node: &str,
    key_file: &KeyFile,
    kind: TransactionKind,
) -> Result<SubmitTxResponse> {
    let tx = build_signed_tx(http, node, key_file, kind).await?;
    submit_transaction(http, node, &tx).await
}

async fn build_signed_tx(
    http: &Client,
    node: &str,
    key_file: &KeyFile,
    kind: TransactionKind,
) -> Result<SignedTransaction> {
    let status = fetch_status(http, node).await?;
    let account = fetch_account(http, node, &key_file.did).await?;
    let tx = Transaction {
        chain_id: status.chain_id,
        nonce: account.next_nonce,
        created_at_unix_ms: Utc::now().timestamp_millis(),
        kind,
    };
    Ok(sign_transaction(key_file, tx)?)
}

async fn submit_transaction(
    http: &Client,
    node: &str,
    tx: &SignedTransaction,
) -> Result<SubmitTxResponse> {
    let endpoint = format!("{}/v1/transactions", normalize_node(node));
    let response = http
        .post(endpoint)
        .json(&SubmitTxRequest {
            tx: tx.clone(),
            propagate: true,
        })
        .send()
        .await?;
    if !response.status().is_success() {
        bail!(response.text().await?);
    }
    Ok(response.json::<SubmitTxResponse>().await?)
}

fn normalize_node(node: &str) -> String {
    node.trim_end_matches('/').to_string()
}

fn parse_rubric(input: &[String]) -> Result<Vec<RubricDimension>> {
    if input.is_empty() {
        bail!("at least one --rubric name=weight is required");
    }
    input.iter().map(|item| {
        let (name, value) = split_kv(item)?;
        Ok(RubricDimension {
            name,
            weight_bps: value.parse::<u16>()?,
        })
    }).collect()
}

fn parse_scores(input: &[String]) -> Result<Vec<ReviewScore>> {
    if input.is_empty() {
        bail!("at least one --score name=value is required");
    }
    input.iter().map(|item| {
        let (dimension, value) = split_kv(item)?;
        Ok(ReviewScore {
            dimension,
            score_bps: value.parse::<u16>()?,
        })
    }).collect()
}

fn split_kv(input: &str) -> Result<(String, String)> {
    let Some((left, right)) = input.split_once('=') else {
        bail!("expected key=value pair, got {input}");
    };
    if left.trim().is_empty() || right.trim().is_empty() {
        bail!("expected non-empty key=value pair, got {input}");
    }
    Ok((left.trim().to_string(), right.trim().to_string()))
}

fn parse_proposal_kind(input: &str) -> Result<ProposalKind> {
    match input.trim().to_ascii_lowercase().as_str() {
        "public_signal" | "public-signal" | "publicsignal" => Ok(ProposalKind::PublicSignal),
        "budget_signal" | "budget-signal" | "budgetsignal" => Ok(ProposalKind::BudgetSignal),
        other => bail!("unknown proposal kind: {other}"),
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let bytes = fs::read(path.as_ref())
        .with_context(|| format!("failed to read {}", path.as_ref().display()))?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn write_json<T: serde::Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path.as_ref(), bytes)
        .with_context(|| format!("failed to write {}", path.as_ref().display()))?;
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

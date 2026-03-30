use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::extract::{Path as AxumPath, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::Parser;
use parking_lot::RwLock;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{info, warn};

use essentia_core::{
    build_status, AccountView, Block, BlocksQuery, Chain, GenesisConfig, ImportBlockRequest,
    KeyFile, NodeConfig, NodeStatus, PeerRegistration, ProposeBlockResponse, SignedTransaction,
    StateDump, SubmitTxRequest, SubmitTxResponse,
};

#[derive(Debug, Parser)]
#[command(name = "essentia-node", version, about = "Essentia v0.1.0 prototype node")]
struct Args {
    #[arg(long, default_value = "examples/bootstrap/node1.json")]
    config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedNode {
    chain: Chain,
    peers: BTreeSet<String>,
}

#[derive(Clone)]
struct AppState {
    config: NodeConfig,
    chain: Arc<RwLock<Chain>>,
    peers: Arc<RwLock<BTreeSet<String>>>,
    http: Client,
    validator_key: Option<KeyFile>,
}

impl AppState {
    fn load(config: NodeConfig) -> Result<Self> {
        fs::create_dir_all(&config.data_dir)
            .with_context(|| format!("failed to create data dir {}", config.data_dir))?;
        let snapshot_path = snapshot_path(&config.data_dir);
        let persisted = if snapshot_path.exists() {
            Some(read_json::<PersistedNode>(&snapshot_path).with_context(|| {
                format!("failed to read snapshot {}", snapshot_path.display())
            })?)
        } else {
            None
        };

        let chain = if let Some(snapshot) = &persisted {
            snapshot.chain.clone()
        } else {
            let genesis: GenesisConfig = read_json(&config.genesis_file)
                .with_context(|| format!("failed to read genesis {}", config.genesis_file))?;
            Chain::from_genesis(genesis)
        };

        let peers = if let Some(snapshot) = persisted {
            snapshot.peers
        } else {
            config.peers.iter().cloned().collect()
        };

        let validator_key = config
            .validator_key_file
            .as_ref()
            .map(|path| read_json::<KeyFile>(path))
            .transpose()
            .with_context(|| "failed to read validator key file".to_string())?;

        Ok(Self {
            config,
            chain: Arc::new(RwLock::new(chain)),
            peers: Arc::new(RwLock::new(peers)),
            http: Client::new(),
            validator_key,
        })
    }

    fn persist(&self) -> Result<()> {
        let snapshot = PersistedNode {
            chain: self.chain.read().clone(),
            peers: self.peers.read().clone(),
        };
        let path = snapshot_path(&self.config.data_dir);
        let bytes = serde_json::to_vec_pretty(&snapshot)?;
        fs::write(&path, bytes)
            .with_context(|| format!("failed to persist snapshot {}", path.display()))?;
        Ok(())
    }

    fn peer_list(&self) -> Vec<String> {
        self.peers.read().iter().cloned().collect()
    }

    fn add_peer(&self, url: &str) -> Result<bool> {
        if url == self.config.public_url {
            return Ok(false);
        }
        let inserted = self.peers.write().insert(url.to_string());
        if inserted {
            self.persist()?;
        }
        Ok(inserted)
    }

    async fn broadcast_tx(&self, tx: SignedTransaction) {
        let peers = self.peer_list();
        for peer in peers {
            let endpoint = format!("{}/v1/transactions", peer.trim_end_matches('/'));
            let request = SubmitTxRequest {
                tx: tx.clone(),
                propagate: false,
            };
            if let Err(error) = self.http.post(endpoint).json(&request).send().await {
                warn!(%error, "failed to broadcast transaction to peer");
            }
        }
    }

    async fn broadcast_block(&self, block: Block) {
        let peers = self.peer_list();
        for peer in peers {
            let endpoint = format!("{}/v1/blocks/import", peer.trim_end_matches('/'));
            let request = ImportBlockRequest {
                block: block.clone(),
                propagate: false,
            };
            if let Err(error) = self.http.post(endpoint).json(&request).send().await {
                warn!(%error, "failed to broadcast block to peer");
            }
        }
    }

    async fn register_with_peers(&self) {
        let peers = self.peer_list();
        for peer in peers {
            let endpoint = format!("{}/v1/peers/register", peer.trim_end_matches('/'));
            let payload = PeerRegistration {
                url: self.config.public_url.clone(),
            };
            if let Err(error) = self.http.post(endpoint).json(&payload).send().await {
                warn!(%error, peer = %peer, "failed to register with peer");
            }
        }
    }

    async fn maybe_propose(&self, max_txs: usize) -> Result<Option<Block>> {
        let Some(key_file) = self.validator_key.clone() else {
            return Ok(None);
        };
        let proposed = {
            let mut chain = self.chain.write();
            if chain.mempool.is_empty() {
                None
            } else {
                match chain.propose_block(&key_file, max_txs) {
                    Ok(block) => Some(block),
                    Err(error) => {
                        let text = error.to_string();
                        if text.contains("scheduled proposer") || text.contains("validator set is empty") {
                            None
                        } else {
                            return Err(error);
                        }
                    }
                }
            }
        };
        if let Some(block) = proposed.clone() {
            self.persist()?;
            self.broadcast_block(block.clone()).await;
        }
        Ok(proposed)
    }

    async fn sync_once(&self) -> Result<()> {
        let peers = self.peer_list();
        for peer in peers {
            let status_endpoint = format!("{}/v1/status", peer.trim_end_matches('/'));
            let remote_status = match self.http.get(&status_endpoint).send().await {
                Ok(response) => match response.json::<NodeStatus>().await {
                    Ok(status) => status,
                    Err(error) => {
                        warn!(%error, peer = %peer, "failed to decode peer status");
                        continue;
                    }
                },
                Err(error) => {
                    warn!(%error, peer = %peer, "failed to fetch peer status");
                    continue;
                }
            };
            let local_height = self.chain.read().state.height;
            if remote_status.height <= local_height {
                continue;
            }
            let blocks_endpoint = format!("{}/v1/blocks", peer.trim_end_matches('/'));
            let response = match self
                .http
                .get(&blocks_endpoint)
                .query(&BlocksQuery {
                    from: Some(local_height + 1),
                })
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    warn!(%error, peer = %peer, "failed to fetch remote blocks");
                    continue;
                }
            };
            let blocks = match response.json::<Vec<Block>>().await {
                Ok(blocks) => blocks,
                Err(error) => {
                    warn!(%error, peer = %peer, "failed to decode remote blocks");
                    continue;
                }
            };
            if blocks.is_empty() {
                continue;
            }
            let mut imported_any = false;
            {
                let mut chain = self.chain.write();
                for block in blocks {
                    if let Err(error) = chain.import_block(block) {
                        warn!(%error, peer = %peer, "failed to import block during sync");
                        break;
                    }
                    imported_any = true;
                }
            }
            if imported_any {
                self.persist()?;
            }
        }
        Ok(())
    }
}

fn snapshot_path(data_dir: &str) -> PathBuf {
    Path::new(data_dir).join("snapshot.json")
}

fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let bytes = fs::read(path.as_ref())?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn http_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, error.to_string())
}

async fn health() -> &'static str {
    "ok"
}

async fn status_handler(State(app): State<AppState>) -> Result<Json<NodeStatus>, (StatusCode, String)> {
    let chain = app.chain.read();
    let peers = app.peer_list();
    let status = build_status(&chain, &app.config.node_id, &app.config.public_url, &peers)
        .map_err(http_error)?;
    Ok(Json(status))
}

async fn state_handler(State(app): State<AppState>) -> Result<Json<StateDump>, (StatusCode, String)> {
    let chain = app.chain.read();
    Ok(Json(chain.state_dump().map_err(http_error)?))
}

async fn account_handler(
    State(app): State<AppState>,
    AxumPath(did): AxumPath<String>,
) -> Result<Json<AccountView>, (StatusCode, String)> {
    let chain = app.chain.read();
    Ok(Json(chain.account_view(&did)))
}

async fn blocks_handler(
    State(app): State<AppState>,
    Query(query): Query<BlocksQuery>,
) -> Result<Json<Vec<Block>>, (StatusCode, String)> {
    let from = query.from.unwrap_or(1);
    let chain = app.chain.read();
    let blocks: Vec<Block> = chain
        .blocks
        .iter()
        .filter(|block| block.height >= from)
        .cloned()
        .collect();
    Ok(Json(blocks))
}

async fn submit_transaction_handler(
    State(app): State<AppState>,
    Json(request): Json<SubmitTxRequest>,
) -> Result<Json<SubmitTxResponse>, (StatusCode, String)> {
    let tx_hash = request.tx.tx_hash.clone();
    {
        let mut chain = app.chain.write();
        chain.submit_transaction(request.tx.clone()).map_err(http_error)?;
    }
    app.persist().map_err(http_error)?;
    if request.propagate {
        let broadcast_state = app.clone();
        let tx = request.tx.clone();
        tokio::spawn(async move {
            broadcast_state.broadcast_tx(tx).await;
        });
    }
    Ok(Json(SubmitTxResponse {
        accepted: true,
        tx_hash,
        reason: None,
    }))
}

async fn propose_block_handler(
    State(app): State<AppState>,
) -> Result<Json<ProposeBlockResponse>, (StatusCode, String)> {
    let max_txs = app
        .config
        .auto_propose
        .as_ref()
        .map(|auto| auto.max_txs)
        .unwrap_or(256);
    let proposed = app.maybe_propose(max_txs).await.map_err(http_error)?;
    if let Some(block) = proposed {
        Ok(Json(ProposeBlockResponse {
            proposed: true,
            height: Some(block.height),
            block_hash: Some(block.block_hash),
            reason: None,
        }))
    } else {
        Ok(Json(ProposeBlockResponse {
            proposed: false,
            height: None,
            block_hash: None,
            reason: Some("node is not scheduled proposer or mempool is empty".to_string()),
        }))
    }
}

async fn import_block_handler(
    State(app): State<AppState>,
    Json(request): Json<ImportBlockRequest>,
) -> Result<Json<ProposeBlockResponse>, (StatusCode, String)> {
    let block = request.block.clone();
    {
        let mut chain = app.chain.write();
        chain.import_block(block.clone()).map_err(http_error)?;
    }
    app.persist().map_err(http_error)?;
    if request.propagate {
        let broadcast_state = app.clone();
        tokio::spawn(async move {
            broadcast_state.broadcast_block(block).await;
        });
    }
    Ok(Json(ProposeBlockResponse {
        proposed: true,
        height: Some(request.block.height),
        block_hash: Some(request.block.block_hash),
        reason: None,
    }))
}

async fn register_peer_handler(
    State(app): State<AppState>,
    Json(peer): Json<PeerRegistration>,
) -> Result<Json<PeerRegistration>, (StatusCode, String)> {
    app.add_peer(&peer.url).map_err(http_error)?;
    Ok(Json(peer))
}

fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/status", get(status_handler))
        .route("/v1/state", get(state_handler))
        .route("/v1/accounts/{did}", get(account_handler))
        .route("/v1/blocks", get(blocks_handler))
        .route("/v1/transactions", post(submit_transaction_handler))
        .route("/v1/blocks/propose", post(propose_block_handler))
        .route("/v1/blocks/import", post(import_block_handler))
        .route("/v1/peers/register", post(register_peer_handler))
        .with_state(app_state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,hyper=warn,reqwest=warn".into()),
        )
        .init();

    let args = Args::parse();
    let config: NodeConfig = read_json(&args.config)
        .with_context(|| format!("failed to load config {}", args.config))?;
    let app_state = AppState::load(config)?;
    app_state.persist()?;
    app_state.register_with_peers().await;

    let sync_state = app_state.clone();
    tokio::spawn(async move {
        loop {
            if let Err(error) = sync_state.sync_once().await {
                warn!(%error, "background sync failed");
            }
            sleep(Duration::from_secs(3)).await;
        }
    });

    if let Some(auto_propose) = app_state.config.auto_propose.clone() {
        if auto_propose.enabled {
            let proposal_state = app_state.clone();
            tokio::spawn(async move {
                loop {
                    if let Err(error) = proposal_state.maybe_propose(auto_propose.max_txs).await {
                        warn!(%error, "auto-propose failed");
                    }
                    sleep(Duration::from_millis(auto_propose.interval_ms)).await;
                }
            });
        }
    }

    let listener = tokio::net::TcpListener::bind(&app_state.config.listen_addr)
        .await
        .with_context(|| format!("failed to bind {}", app_state.config.listen_addr))?;
    info!(
        node_id = %app_state.config.node_id,
        listen_addr = %app_state.config.listen_addr,
        public_url = %app_state.config.public_url,
        "essentia node listening"
    );
    axum::serve(listener, router(app_state)).await?;
    Ok(())
}

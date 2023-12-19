use crate::{request::send_request, url::Url, *};
use blockfrost_openapi::models::{
    address_transactions_content_inner::AddressTransactionsContentInner, tx_content::TxContent,
    tx_content_delegations_inner::TxContentDelegationsInner,
    tx_content_metadata_cbor_inner::TxContentMetadataCborInner,
    tx_content_metadata_inner::TxContentMetadataInner, tx_content_mirs_inner::TxContentMirsInner,
    tx_content_pool_retires_inner::TxContentPoolRetiresInner,
    tx_content_redeemers_inner::TxContentRedeemersInner,
    tx_content_stake_addr_inner::TxContentStakeAddrInner,
    tx_content_withdrawals_inner::TxContentWithdrawalsInner,
};
use reqwest::{header::HeaderValue, Body, Method};
use serde_json::from_str as json_from;

impl BlockfrostAPI {
    /// Obtain information about Move Instantaneous Rewards (MIRs) of a specific transaction.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/mirs`].
    ///
    /// [`/accounts/{stake_address}/mirs`]: https://docs.blockfrost.io/#tag/Cardano-Transactions/paths/~1tx~1submit/post
    pub async fn transactions_submit(&self, transaction_data: Vec<u8>) -> BlockfrostResult<String> {
        let body = Body::from(transaction_data);
        let endpoint_suffix = "/tx/submit";
        let url = Url::from_endpoint(self.base_url.as_str(), endpoint_suffix)?;

        let request = self
            .client
            .request(Method::POST, &url)
            .header("Content-Type", HeaderValue::from_static("application/cbor"))
            .body(body);

        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| BlockfrostError::Reqwest {
                url: url.clone(),
                reason,
            })?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }
        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    pub async fn transaction_by_hash(
        &self, hash: &str,
    ) -> BlockfrostResult<AddressTransactionsContentInner> {
        self.call_endpoint(format!("/txs/{}", hash).as_str()).await
    }

    pub async fn transactions_utxos(&self, hash: &str) -> BlockfrostResult<TxContent> {
        self.call_endpoint(format!("/txs/{}/utxos", hash).as_str())
            .await
    }

    pub async fn transactions_stakes(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentStakeAddrInner>> {
        self.call_endpoint(format!("/txs/{}/stakes", hash).as_str())
            .await
    }

    pub async fn transactions_delegations(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentDelegationsInner>> {
        self.call_endpoint(format!("/txs/{}/delegations", hash).as_str())
            .await
    }

    pub async fn transactions_withdrawals(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentWithdrawalsInner>> {
        self.call_endpoint(format!("/txs/{}/withdrawals", hash).as_str())
            .await
    }

    pub async fn transactions_mirs(&self, hash: &str) -> BlockfrostResult<Vec<TxContentMirsInner>> {
        self.call_endpoint(format!("/txs/{}/mirs", hash).as_str())
            .await
    }

    pub async fn transactions_pool_updates(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentMirsInner>> {
        self.call_endpoint(format!("/txs/{}/pool_updates", hash).as_str())
            .await
    }

    pub async fn transactions_pool_retires(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentPoolRetiresInner>> {
        self.call_endpoint(format!("/txs/{}/pool_retires", hash).as_str())
            .await
    }

    pub async fn transactions_metadata(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentMetadataInner>> {
        self.call_endpoint(format!("/txs/{}/metadata", hash).as_str())
            .await
    }

    pub async fn transactions_metadata_cbor(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentMetadataCborInner>> {
        self.call_endpoint(format!("/txs/{}/metadata/cbor", hash).as_str())
            .await
    }

    pub async fn transactions_redeemers(
        &self, hash: &str,
    ) -> BlockfrostResult<Vec<TxContentRedeemersInner>> {
        self.call_endpoint(format!("/txs/{}/redeemers", hash).as_str())
            .await
    }
}
<<<<<<< HEAD
=======

/// Created by [`transaction_by_hash`](BlockFrostApi::transaction_by_hash) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    /// Transaction hash.
    pub hash: String,
    /// Block hash.
    pub block: String,
    /// Block number.
    pub block_height: Integer,
    /// Block creation time in UNIX time.
    pub block_time: Integer,
    /// Slot number.
    pub slot: Integer,
    /// Transaction index within the block.
    pub index: Integer,
    /// Amounts of the transaction.
    pub output_amount: Vec<Amount>,
    /// Fees of the transaction in Lovelaces.
    pub fees: String,
    /// Deposit within the transaction in Lovelaces.
    pub deposit: String,
    /// Size of the transaction in Bytes.
    pub size: Integer,
    /// Left (included) endpoint of the timelock validity intervals.
    pub invalid_before: Option<String>,
    /// Right (excluded) endpoint of the timelock validity intervals.
    pub invalid_hereafter: Option<String>,
    /// Count of UTXOs within the transaction.
    pub utxo_count: Integer,
    /// Count of the withdrawals within the transaction.
    pub withdrawal_count: Integer,
    /// Count of the MIR certificates within the transaction.
    pub mir_cert_count: Integer,
    /// Count of the delegations within the transaction.
    pub delegation_count: Integer,
    /// Count of the stake keys (de)registration and delegation certificates within the transaction.
    pub stake_cert_count: Integer,
    /// Count of the stake pool registration and update certificates within the transaction.
    pub pool_update_count: Integer,
    /// Count of the stake pool retirement certificates within the transaction.
    pub pool_retire_count: Integer,
    /// Count of asset mints and burns within the transaction.
    pub asset_mint_or_burn_count: Integer,
    /// Count of redeemers within the transaction.
    pub redeemer_count: Integer,
}

/// Created by [`transactions_utxos`](BlockFrostApi::transactions_utxos) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionUtxos {
    /// Transaction hash.
    pub hash: String,
    pub inputs: Vec<TransactionUtxosInput>,
    pub outputs: Vec<TransactionUtxosOutput>,
}

/// Created by [`transactions_stakes`](BlockFrostApi::transactions_stakes) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionStake {
    /// Index of the certificate within the transaction.
    pub cert_index: Integer,
    /// Delegation stake address.
    pub address: String,
    /// Registration boolean, false if deregistration.
    pub registration: bool,
}

/// Created by [`transactions_delegations`](BlockFrostApi::transactions_delegations) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionDelegation {
    /// Index of the certificate within the transaction".
    pub index: Integer,
    /// Index of the certificate within the transaction.
    pub cert_index: Integer,
    /// Bech32 delegation stake address.
    pub address: String,
    /// Bech32 ID of delegated stake pool.
    pub pool_id: String,
    /// Epoch in which the delegation becomes active.
    pub active_epoch: Integer,
}

/// Created by [`transactions_withdrawals`](BlockFrostApi::transactions_withdrawals) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionWithdrawal {
    /// Bech32 withdrawal address.
    pub address: String,
    /// Withdrawal amount in Lovelaces.
    pub amount: String,
}

/// Created by [`transactions_mirs`](BlockFrostApi::transactions_mirs) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionMir {
    /// Source of MIR funds.
    pub pot: MirFundsSource,
    /// Index of the certificate within the transaction.
    pub cert_index: Integer,
    /// Bech32 stake address.
    pub address: String,
    /// MIR amount in Lovelaces.
    pub amount: String,
}

/// Created by [`transactions_pool_updates`](BlockFrostApi::transactions_pool_updates) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionPoolUpdate {
    /// Index of the certificate within the transaction
    pub cert_index: Integer,
    /// Bech32 encoded pool ID
    pub pool_id: String,
    /// VRF key hash
    pub vrf_key: String,
    /// Stake pool certificate pledge in Lovelaces
    pub pledge: String,
    /// Margin tax cost of the stake pool
    pub margin_cost: Float,
    /// Fixed tax cost of the stake pool in Lovelaces
    pub fixed_cost: String,
    /// Bech32 reward account of the stake pool
    pub reward_account: String,
    /// Bech32 accounts of the pool owners.
    pub owners: Vec<String>,
    pub metadata: Option<PoolUpdateMetadata>,
    pub relays: Vec<Relay>,
    /// Epoch that the delegation becomes active.
    pub active_epoch: Integer,
}

/// Created by [`transactions_pool_retires`](BlockFrostApi::transactions_pool_retires) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionPoolRetire {
    /// Index of the certificate within the transaction.
    pub cert_index: Integer,
    /// Bech32 stake pool ID.
    pub pool_id: String,
    /// Retiring epoch.
    pub retiring_epoch: Integer,
}

/// Created by [`transactions_metadata`](BlockFrostApi::transactions_metadata) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionMetadata {
    /// Metadata label.
    pub label: String,
    /// Content of the metadata.
    ///
    /// Can be either a Json Object or String.
    pub json_metadata: JsonValue,
}

/// Created by [`transactions_metadata_cbor`](BlockFrostApi::transactions_metadata_cbor) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionMetadataCbor {
    /// Metadata label.
    pub label: String,
    /// Content of the CBOR metadata.
    pub cbor_metadata: Option<String>,
}

/// Created by [`transactions_redeemers`](BlockFrostApi::transactions_redeemers) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRedeemer {
    /// Index of the redeemer within the transaction.
    pub tx_index: Integer,
    /// Validation purpose.
    pub purpose: RedeemerPurpose,
    /// The budget in Memory to run a script.
    pub unit_mem: String,
    /// The budget in CPU steps to run a script.
    pub unit_steps: String,
    /// The fee consumed to run the script.
    pub fee: String,
}

/// Inner member of [`TransactionUtxos`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionUtxosInput {
    /// Input address.
    pub address: String,
    pub amount: Vec<Amount>,
    /// Hash of the UTXO transaction.
    pub tx_hash: String,
    /// UTXO index in the transaction.
    pub output_index: Integer,
    /// The hash of the transaction output datum.
    pub data_hash: Option<String>,
    // Whether the input is a collateral consumed on script validation failure.
    pub collateral: bool,
}

/// Inner member of [`TransactionUtxos`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionUtxosOutput {
    /// Output address.
    pub address: String,
    pub amount: Vec<Amount>,
    /// The hash of the transaction output datum.
    pub data_hash: Option<String>,
    /// UTXO index in the transaction.
    pub output_index: Integer,
    pub inline_datum: Option<String>,
    pub collateral: bool,
    pub reference_script_hash: Option<String>,
}

/// Inner enum for [`TransactionMir`].
///
/// Source of MIR funds.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MirFundsSource {
    Reserve,
    Treasury,
}

/// Inner enum for [`TransactionRedeemer`].
///
/// Validation purpose.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RedeemerPurpose {
    Spend,
    Mint,
    Cert,
    Reward,
}

/// Inner member of [`TransactionPoolUpdate`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolUpdateMetadata {
    /// URL to the stake pool metadata.
    pub url: Option<String>,
    /// Hash of the metadata file.
    pub hash: Option<String>,
    /// Ticker of the stake pool.
    pub ticker: Option<String>,
    /// Name of the stake pool.
    pub name: Option<String>,
    /// Description of the stake pool.
    pub description: Option<String>,
    /// Home page of the stake pool.
    pub homepage: Option<String>,
}

/// Inner member of [`TransactionPoolUpdate`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relay {
    /// IPv4 address of the relay.
    pub ipv4: Option<String>,
    /// IPv6 address of the relay.
    pub ipv6: Option<String>,
    /// DNS name of the relay.
    pub dns: Option<String>,
    /// DNS SRV entry of the relay.
    pub dns_srv: Option<String>,
    /// Network port of the relay.
    pub port: Integer,
}

>>>>>>> c7f0f36 (add missing TransactionOutputs fields)
#[cfg(test)]
mod tests {
    use super::*;
    use blockfrost_openapi::models::{
        tx_content_metadata_inner_json_metadata::TxContentMetadataInnerJsonMetadata,
        tx_content_utxo::TxContentUtxo,
    };
    use serde_json::json;

<<<<<<< HEAD
    #[tokio::test]
    async fn test_transaction() {
        let json_value = json!({
            "hash": "1e043f100dce12d107f679685acd2fc0610e10f72a92d412794c9773d11d8477",
            "block": "356b7d7dbb696ccd12775c016941057a9dc70898d87a63fc752271bb46856940",
            "block_height": 123456,
            "block_time": 2,
            "slot": 42000000,
            "index": 1,
            "output_amount": [
                {
                    "unit": "lovelace",
                    "quantity": "42000000"
                },
                {
                    "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
                    "quantity": "12"
                }
            ],
            "fees": "182485",
            "deposit": "0",
            "size": 433,
            "invalid_before": null,
            "invalid_hereafter": "13885913",
            "utxo_count": 4,
            "withdrawal_count": 0,
            "mir_cert_count": 0,
            "delegation_count": 0,
            "stake_cert_count": 0,
            "pool_update_count": 0,
            "pool_retire_count": 0,
            "asset_mint_or_burn_count": 0,
            "redeemer_count": 0,
            "valid_contract": false
        });
        serde_json::from_value::<TxContent>(json_value).unwrap();
=======
    test_example! { test_transaction, Transaction, r#"
    {
      "hash": "1e043f100dce12d107f679685acd2fc0610e10f72a92d412794c9773d11d8477",
      "block": "356b7d7dbb696ccd12775c016941057a9dc70898d87a63fc752271bb46856940",
      "block_height": 123456,
      "slot": 42000000,
      "index": 1,
      "output_amount": [
        {
          "unit": "lovelace",
          "quantity": "42000000"
        },
        {
          "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
          "quantity": "12"
        }
      ],
      "fees": "182485",
      "deposit": "0",
      "size": 433,
      "invalid_before": null,
      "invalid_hereafter": "13885913",
      "utxo_count": 4,
      "withdrawal_count": 0,
      "mir_cert_count": 0,
      "delegation_count": 0,
      "stake_cert_count": 0,
      "pool_update_count": 0,
      "pool_retire_count": 0,
      "asset_mint_or_burn_count": 0,
      "redeemer_count": 0,
      "block_time": 0
>>>>>>> 2ee7a77 (chore: update deps)
    }

    #[tokio::test]
    async fn test_transaction_null_invalid_hereafter() {
        let json_value = json!({
            "hash": "78e5821367cd4e2fbfce4d32c3ecededd388f13118f06b959015c2aad19b5cd8",
            "block": "bb4003096f11eaaca11d3705db62637f311f93671fcb4d4b3cf8749a90ea4b74",
            "block_height": 6352717,
            "block_time": 1633861069,
            "slot": 42294778,
            "index": 9,
            "output_amount": [
                {
                    "unit": "lovelace",
                    "quantity": "4828735"
                },
                {
                    "unit": "29d222ce763455e3d7a09a665ce554f00ac89d2e99a1a83d267170c64d494e",
                    "quantity": "25000000000000"
                }
            ],
            "fees": "171265",
            "deposit": "0",
            "size": 357,
            "invalid_before": null,
            "invalid_hereafter": null,
            "utxo_count": 3,
            "withdrawal_count": 0,
            "mir_cert_count": 0,
            "delegation_count": 0,
            "stake_cert_count": 0,
            "pool_update_count": 0,
            "pool_retire_count": 0,
            "asset_mint_or_burn_count": 1,
            "redeemer_count": 0,
            "valid_contract": true
        });
        serde_json::from_value::<TxContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_utxos() {
        let json_value = json!({
            "hash": "1e043f100dce12d107f679685acd2fc0610e10f72a92d412794c9773d11d8477",
            "inputs": [
                {
                    "address": "addr1q9ld26v2lv8wvrxxmvg90pn8n8n5k6tdst06q2s856rwmvnueldzuuqmnsye359fqrk8hwvenjnqultn7djtrlft7jnq7dy7wv",
                    "amount": [
                        {
                            "unit": "lovelace",
                            "quantity": "42000000"
                        },
                        {
                            "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
                            "quantity": "12"
                        }
                    ],
                    "inline_datum": "12",
                    "reference_script_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                    "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                    "output_index": 0,
                    "data_hash": null,
                    "collateral": false
                }
            ],
            "outputs": [
                {
                    "address": "addr1q9ld26v2lv8wvrxxmvg90pn8n8n5k6tdst06q2s856rwmvnueldzuuqmnsye359fqrk8hwvenjnqultn7djtrlft7jnq7dy7wv",
                    "output_index": 0,
                    "data_hash": null,
                    "reference_script_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                    "inline_datum": null,
                    "collateral": false,
                    "amount": [
                        {
                            "unit": "lovelace",
                            "quantity": "42000000"
                        },
                        {
                            "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
                            "quantity": "12"
                        }
                    ]
                }
            ]
        });

        serde_json::from_value::<TxContentUtxo>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_stakes() {
        let json_value = json!([
            {
                "cert_index": 0,
                "address": "stake1u9t3a0tcwune5xrnfjg4q7cpvjlgx9lcv0cuqf5mhfjwrvcwrulda",
                "registration": true
            }
        ]);

        serde_json::from_value::<Vec<TxContentStakeAddrInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_delegations() {
        let json_value = json!([
            {
                "index": 0,
                "cert_index": 0,
                "address": "stake1u9r76ypf5fskppa0cmttas05cgcswrttn6jrq4yd7jpdnvc7gt0yc",
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "active_epoch": 210
            }
        ]);
        serde_json::from_value::<Vec<TxContentDelegationsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_withdrawals() {
        let json_value = json!([
            {
                "address": "stake1u9r76ypf5fskppa0cmttas05cgcswrttn6jrq4yd7jpdnvc7gt0yc",
                "amount": "431833601"
            }
        ]);

        serde_json::from_value::<Vec<TxContentWithdrawalsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_mirs() {
        let json_value = json!([
            {
                "pot": "reserve",
                "cert_index": 0,
                "address": "stake1u9r76ypf5fskppa0cmttas05cgcswrttn6jrq4yd7jpdnvc7gt0yc",
                "amount": "431833601"
            }
        ]);

        serde_json::from_value::<Vec<TxContentMirsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_pool_updates() {
        let json_value = json!([
            {
                "cert_index": 0,
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "vrf_key": "0b5245f9934ec2151116fb8ec00f35fd00e0aa3b075c4ed12cce440f999d8233",
                "pledge": "5000000000",
                "margin_cost": 0.05,
                "fixed_cost": "340000000",
                "reward_account": "stake1uxkptsa4lkr55jleztw43t37vgdn88l6ghclfwuxld2eykgpgvg3f",
                "owners": [
                    "stake1u98nnlkvkk23vtvf9273uq7cph5ww6u2yq2389psuqet90sv4xv9v"
                ],
                "metadata": {
                    "url": "https://stakenuts.com/mainnet.json",
                    "hash": "47c0c68cb57f4a5b4a87bad896fc274678e7aea98e200fa14a1cb40c0cab1d8c",
                    "ticker": "NUTS",
                    "name": "Stake Nuts",
                    "description": "The best pool ever",
                    "homepage": "https://stakentus.com/"
                },
                "relays": [
                    {
                        "ipv4": "4.4.4.4",
                        "ipv6": "https://stakenuts.com/mainnet.json",
                        "dns": "relay1.stakenuts.com",
                        "dns_srv": "_relays._tcp.relays.stakenuts.com",
                        "port": 3001
                    }
                ],
                "active_epoch": 210
            }
        ]);

        serde_json::from_value::<Vec<TxContentMetadataInnerJsonMetadata>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_pool_retires() {
        let json_value = json!([
            {
                "cert_index": 0,
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "retiring_epoch": 216
            }
        ]);

        serde_json::from_value::<Vec<TxContentMetadataInnerJsonMetadata>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_metadata() {
        let json_value = json!([
            {
                "label": "1967",
                "json_metadata": {
                    "metadata": "https://nut.link/metadata.json",
                    "hash": "6bf124f217d0e5a0a8adb1dbd8540e1334280d49ab861127868339f43b3948af"
                }
            },
            {
                "label": "1968",
                "json_metadata": {
                    "ADAUSD": [
                        {
                            "value": "0.10409800535729975",
                            "source": "ergoOracles"
                        }
                    ]
                }
            }
        ]);

        serde_json::from_value::<Vec<TxContentMetadataInnerJsonMetadata>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_metadata_cbor() {
        let json_value = json!([
            {
                "label": "1968",
                "cbor_metadata": "\\xa100a16b436f6d62696e6174696f6e8601010101010c",
                "metadata": null
            }
        ]);

        serde_json::from_value::<Vec<TxContentMetadataCborInner>>(json_value).unwrap();
    }
}

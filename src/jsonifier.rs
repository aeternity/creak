use serde_json::*;
use crate::rlp_val::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref OBJECT_TAG_SIGNED_TRANSACTION = 11;
    static ref OBJECT_TAG_SPEND_TRANSACTION = 12;
    static ref OBJECT_TAG_ORACLE_REGISTER_TRANSACTION = 22;
    static ref OBJECT_TAG_ORACLE_QUERY_TRANSACTION = 23;
    static ref OBJECT_TAG_ORACLE_RESPONSE_TRANSACTION = 24;
    static ref OBJECT_TAG_ORACLE_EXTEND_TRANSACTION = 25;
    static ref OBJECT_TAG_NAME_SERVICE_CLAIM_TRANSACTION = 32;
    static ref OBJECT_TAG_NAME_SERVICE_PRECLAIM_TRANSACTION = 33;
    static ref OBJECT_TAG_NAME_SERVICE_UPDATE_TRANSACTION = 34;
    static ref OBJECT_TAG_NAME_SERVICE_REVOKE_TRANSACTION = 35;
    static ref OBJECT_TAG_NAME_SERVICE_TRANSFER_TRANSACTION = 36;
    static ref OBJECT_TAG_CONTRACT_CREATE_TRANSACTION = 42;
    static ref OBJECT_TAG_CONTRACT_CALL_TRANSACTION = 43;
}

pub enum TxType {
    SignedTx,
    Spend,
    ContractCreate,
    ContractCall,
    NamePreClaim,
    NameClaim,
    NameUpdate,
    NameTransfer,
    NameRevoke,
    OracleRegister,
    OracleExtend,
    OracleQuery,
    OracleRespond,
}

impl TxType {
    pub fn from_tag(s: &i16) -> Option<TxType> {
        match s {
            OBJECT_TAG_CONTRACT_CALL_TRANSACTION => Some(TxType::ContractCall),
            OBJECT_TAG_CONTRACT_CREATE_TRANSACTION => Some(TxType::ContractCreate),
            OBJECT_TAG_NAME_SERVICE_CLAIM_TRANSACTION => Some(TxType::NameClaim),
            OBJECT_TAG_NAME_SERVICE_PRECLAIM_TRANSACTION => Some(TxType::NamePreClaim),
            OBJECT_TAG_NAME_SERVICE_REVOKE_TRANSACTION => Some(TxType::NameRevoke),
            OBJECT_TAG_NAME_SERVICE_TRANSFER_TRANSACTION => Some(TxType::NameTransfer),
            OBJECT_TAG_NAME_SERVICE_UPDATE_TRANSACTION => Some(TxType::NameUpdate),
            OBJECT_TAG_SIGNED_TRANSACTION => Some(TxType::SignedTx),
            OBJECT_TAG_SPEND_TRANSACTION => Some(TxType::Spend),
            OBJECT_TAG_ORACLE_REGISTER_TRANSACTION => Some(TxType::OracleRegister),
            OBJECT_TAG_ORACLE_EXTEND_TRANSACTION => Some(TxType::OracleExtend),
            OBJECT_TAG_ORACLE_QUERY_TRANSACTION => Some(TxType::OracleQuery),
            OBJECT_TAG_ORACLE_RESPONSE_TRANSACTION => Some(TxType::OracleRespond),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match s {
            TxType::SignedTx => "signedTx",
            TxType::Spend => "spendTx",
            TxType::ContractCall => "contractCallTx",
            TxType::ContractCreate => "contractCreateTx",
            TxType::NameClaim => "nameClaimTx",
            TxType::NamePreClaim => "namePreClaimTx",
            TxType::NameTransfer => "nameTransferTx",
            TxType::NameUpdate => "nameUpdateTx",
            TxType::NameRevoke => "nameRevokeTx",
            TxType::OracleRegister => "oracleRegisterTx",
            TxType::OracleExtend => "oracleExtendTx",
            TxType::OracleQuery => "oracleQueryTx",
            TxType::OracleRespond => "oracleRespondTx"
        }
    }
}

fn tx_type(tag: u32) -> String {
    String::from("SpendTx")
}

pub fn signed_tx(stx: &RlpVal, tx: &Value) -> Value
{
    let _type = tx_type(u32::convert(&stx[0]));
    json!(
        {
            "type": _type,
            "signatures": SignatureList::convert(&stx[2]),
            "tx": tx,
        })
}

pub fn spend_tx(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "SpendTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn name_claim(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "NameClaimTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn name_update(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "NameUpdateTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn name_transfer(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "NameTransferTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn name_pre_claim(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "NamePreClaimTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn name_revoke(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "NameRevokeTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn contract_create(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "ContractCreateTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn contract_call(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "ContractCallTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn oracle_register(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "ContractCallTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn oracle_extend(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "ContractCallTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn oracle_query(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "ContractCallTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

pub fn oracle_respond(rlp: &RlpVal) -> Value
{
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": "ContractCallTx",
            "nonce": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[4]),
            "payload": String::convert(&rlp[8]),
            "version": 1,
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "recipient_id":AeIdentifier::convert(&rlp[3]),
        })
}

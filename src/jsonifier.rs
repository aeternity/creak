use serde::ser::{Serialize, Serializer};
use serde_json::Value;
use crate::rlp_val::*;

type RlpError = Box<std::error::Error>;

const OBJECT_TAG_SIGNED_TRANSACTION: u32 = 11;
const OBJECT_TAG_SPEND_TRANSACTION: u32 = 12;
const OBJECT_TAG_ORACLE_REGISTER_TRANSACTION: u32 = 22;
const OBJECT_TAG_ORACLE_QUERY_TRANSACTION: u32 = 23;
const OBJECT_TAG_ORACLE_RESPONSE_TRANSACTION: u32 = 24;
const OBJECT_TAG_ORACLE_EXTEND_TRANSACTION: u32 = 25;
const OBJECT_TAG_NAME_SERVICE_CLAIM_TRANSACTION: u32 = 32;
const OBJECT_TAG_NAME_SERVICE_PRECLAIM_TRANSACTION: u32 = 33;
const OBJECT_TAG_NAME_SERVICE_UPDATE_TRANSACTION: u32 = 34;
const OBJECT_TAG_NAME_SERVICE_REVOKE_TRANSACTION: u32 = 35;
const OBJECT_TAG_NAME_SERVICE_TRANSFER_TRANSACTION: u32 = 36;
const OBJECT_TAG_CONTRACT_CREATE_TRANSACTION: u32 = 42;
const OBJECT_TAG_CONTRACT_CALL_TRANSACTION: u32 = 43;


pub enum TxType {
    Signed,
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
    pub fn from_tag(s: u32) -> Option<TxType> {
        match s {
            OBJECT_TAG_CONTRACT_CALL_TRANSACTION => Some(TxType::ContractCall),
            OBJECT_TAG_CONTRACT_CREATE_TRANSACTION => Some(TxType::ContractCreate),
            OBJECT_TAG_NAME_SERVICE_CLAIM_TRANSACTION => Some(TxType::NameClaim),
            OBJECT_TAG_NAME_SERVICE_PRECLAIM_TRANSACTION => Some(TxType::NamePreClaim),
            OBJECT_TAG_NAME_SERVICE_REVOKE_TRANSACTION => Some(TxType::NameRevoke),
            OBJECT_TAG_NAME_SERVICE_TRANSFER_TRANSACTION => Some(TxType::NameTransfer),
            OBJECT_TAG_NAME_SERVICE_UPDATE_TRANSACTION => Some(TxType::NameUpdate),
            OBJECT_TAG_SIGNED_TRANSACTION => Some(TxType::Signed),
            OBJECT_TAG_SPEND_TRANSACTION => Some(TxType::Spend),
            OBJECT_TAG_ORACLE_REGISTER_TRANSACTION => Some(TxType::OracleRegister),
            OBJECT_TAG_ORACLE_EXTEND_TRANSACTION => Some(TxType::OracleExtend),
            OBJECT_TAG_ORACLE_QUERY_TRANSACTION => Some(TxType::OracleQuery),
            OBJECT_TAG_ORACLE_RESPONSE_TRANSACTION => Some(TxType::OracleRespond),
            _ => None,
        }
    }

    pub fn as_str(s: &TxType) -> &'static str {
        match s {
            TxType::Signed => "signedTx",
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

impl Serialize for TxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(TxType::as_str(self))
    }
}

pub fn process_tx(tag: u32, stx: &RlpVal) -> Value {
    match TxType::from_tag(tag) {
        Some(txType) => parse_tx(stx, txType),
        None => panic!("Wrong Transaction type")
    }
}

fn parse_tx(stx: &RlpVal, tx_type: TxType) -> Value {
    match tx_type {
        TxType::Signed => signed_tx(stx).unwrap(), // TODO
        TxType::Spend => spend_tx(stx),
        TxType::ContractCall => contract_call(stx),
        TxType::ContractCreate => contract_create(stx),
        TxType::NameClaim => name_claim(stx),
        TxType::NamePreClaim => name_pre_claim(stx),
        TxType::NameUpdate => name_update(stx),
        TxType::NameTransfer => name_transfer(stx),
        TxType::NameRevoke => name_revoke(stx),
        TxType::OracleRegister => oracle_register(stx),
        TxType::OracleExtend => oracle_extend(stx),
        TxType::OracleQuery => oracle_query(stx),
        TxType::OracleRespond => oracle_respond(stx),
    }
}

pub fn signed_tx(stx: &RlpVal) -> ::std::result::Result<Value, RlpError>
{
    let tx_rlp_val = match &stx[3] {
        RlpVal::Val { data } => rlp::Rlp::new(&data),
        _ => return Err("Wrong type of RlpVal".into()),
    };
    let tx = RlpVal::from_rlp(&tx_rlp_val)?;
    let tx_json = process_tx(u32::convert(&tx[0]), &tx);
    Ok(json!(
        {
            "type": TxType::Signed,
            "signatures": SignatureList::convert(&stx[2]),
            "tx": tx_json,
        }))
}

pub fn spend_tx(rlp: &RlpVal) -> Value
{
    println!("spend_tx: {:?}", rlp);
    json!(
        {
            "fee": u64::convert(&rlp[5]),
            "type": TxType::Spend,
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
            "fee": u64::convert(&rlp[6]),
            "ttl": u64::convert(&rlp[7]),
            "type": TxType::NameClaim,
            "nonce": u64::convert(&rlp[3]),
            "name": encode(&rlp[4], "nm"),
            "account_id": AeIdentifier::convert(&rlp[2]),
            "name_salt": u64::convert(&rlp[5]),
            "version": 1,
        })
}

pub fn name_update(rlp: &RlpVal) -> Value
{
    json!(
        {
            "type": TxType::NameUpdate,
            "fee": u64::convert(&rlp[8]),
            "ttl": u64::convert(&rlp[9]),
            "nonce": u64::convert(&rlp[3]),
            "version": 1,
            "account_id": AeIdentifier::convert(&rlp[2]),
            "name_id": AeIdentifier::convert(&rlp[4]),
            "name_ttl": u64::convert(&rlp[5]),
            "client_ttl": u64::convert(&rlp[7]),
            "pointers": String::convert(&rlp[6]), // TODO read pointers
        })
}

pub fn name_transfer(rlp: &RlpVal) -> Value
{
    json!(
        {
            "type": TxType::NameTransfer,
            "fee": u64::convert(&rlp[6]),
            "ttl": u64::convert(&rlp[7]),
            "nonce": u64::convert(&rlp[3]),
            "version": 1,
            "account_id": AeIdentifier::convert(&rlp[2]),
            "name_id": AeIdentifier::convert(&rlp[4]),
            "recipient_id": AeIdentifier::convert(&rlp[5])
        })
}

pub fn name_pre_claim(rlp: &RlpVal) -> Value
{
    json!(
        {
            "account_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "commitment_id": u64::convert(&rlp[4]),
            "fee": u64::convert(&rlp[5]),
            "ttl": u64::convert(&rlp[6]),
            "type": TxType::NamePreClaim,
            "version": 1
        })
}

pub fn name_revoke(rlp: &RlpVal) -> Value
{
    json!(
        {
            "account_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "name_id": AeIdentifier::convert(&rlp[4]),
            "fee": u64::convert(&rlp[5]),
            "fee": u64::convert(&rlp[6]),
            "type": TxType::NameRevoke,
            "version": 1
        })
}

pub fn contract_create(rlp: &RlpVal) -> Value
{
    json!(
        {
            "owner_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "code": encode(&rlp[4], "cb"),
            "vm_version": u64::convert(&rlp[5]),
            "fee": u64::convert(&rlp[6]),
            "ttl": u64::convert(&rlp[7]),
            "deposit": u128::convert(&rlp[8]),
            "amount": u128::convert(&rlp[9]),
            "gas": u128::convert(&rlp[10]),
            "gas_price": u128::convert(&rlp[11]),
            "call_data": encode(&rlp[12], "cb"),
            "type": TxType::ContractCreate,
            "version": 1
        })
}

pub fn contract_call(rlp: &RlpVal) -> Value
{
    json!(
        {
            "caller_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "contract_id": AeIdentifier::convert(&rlp[4]),
            "vm_version": u64::convert(&rlp[5]),
            "fee": u64::convert(&rlp[6]),
            "ttl": u64::convert(&rlp[7]),
            "amount": u128::convert(&rlp[8]),
            "gas": u128::convert(&rlp[9]),
            "gas_price": u128::convert(&rlp[10]),
            "call_data": encode(&rlp[11], "cb"),
            "type": TxType::ContractCall,
            "version": 1
        })
}

pub fn oracle_register(rlp: &RlpVal) -> Value
{
    json!(
        {
            "account_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "query_format": String::convert(&rlp[4]),
            "response_format": String::convert(&rlp[5]),
            "query_fee": u128::convert(&rlp[6]),
            "oracle_ttl_type": u128::convert(&rlp[7]),
            "oracle_ttl_value": u128::convert(&rlp[8]),
            "fee": u64::convert(&rlp[9]),
            "ttl": u64::convert(&rlp[10]),
            "vm_version": u64::convert(&rlp[11]),
            "type": TxType::OracleRegister,
            "version": 1
        })
}

pub fn oracle_extend(rlp: &RlpVal) -> Value
{
    json!(
        {
            "oracle_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "oracle_ttl_type": u32::convert(&rlp[4]),
            "oracle_ttl_value": u32::convert(&rlp[5]),
            "fee": u64::convert(&rlp[6]),
            "ttl": u64::convert(&rlp[7]),
            "type": TxType::OracleExtend,
            "version": 1
        })
}

pub fn oracle_query(rlp: &RlpVal) -> Value
{
    json!(
        {
            "sender_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "oracle_id": AeIdentifier::convert(&rlp[4]),
            "query": String::convert(&rlp[5]),
            "query_fee": u128::convert(&rlp[6]),
            "query_ttl_type": u128::convert(&rlp[7]),
            "query_ttl_value": u128::convert(&rlp[8]),
            "response_ttl_value": u128::convert(&rlp[9]),
            "response_ttl_value": u128::convert(&rlp[10]),
            "fee": u64::convert(&rlp[11]),
            "ttl": u64::convert(&rlp[12]),
            "type": TxType::OracleQuery,
            "version": 1
        })
}

pub fn oracle_respond(rlp: &RlpVal) -> Value
{
    json!(
        {
            "oracle_id": AeIdentifier::convert(&rlp[2]),
            "nonce": u64::convert(&rlp[3]),
            "query_id": encode(&rlp[4], "oq"),
            "response": String::convert(&rlp[5]),
            "response_ttl_value": u128::convert(&rlp[6]),
            "response_ttl_value": u128::convert(&rlp[7]),
            "fee": u64::convert(&rlp[8]),
            "ttl": u64::convert(&rlp[9]),
            "type": TxType::OracleRespond,
            "version": 1
        })
}

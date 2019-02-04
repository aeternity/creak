use serde::ser::{Serialize, Serializer};
use serde_json::Value;
use crate::rlp_val::*;

type RlpError = Box<std::error::Error>;

const OBJECT_TAG_SIGNED_TRANSACTION: u32 = 11;
const OBJECT_TAG_SPEND_TRANSACTION :u32 = 12;
const OBJECT_TAG_ORACLE_REGISTER_TRANSACTION :u32 = 22;
const OBJECT_TAG_ORACLE_QUERY_TRANSACTION :u32 = 23;
const OBJECT_TAG_ORACLE_RESPONSE_TRANSACTION :u32 = 24;
const OBJECT_TAG_ORACLE_EXTEND_TRANSACTION :u32 = 25;
const OBJECT_TAG_NAME_SERVICE_CLAIM_TRANSACTION :u32 = 32;
const OBJECT_TAG_NAME_SERVICE_PRECLAIM_TRANSACTION :u32 = 33;
const OBJECT_TAG_NAME_SERVICE_UPDATE_TRANSACTION :u32 = 34;
const OBJECT_TAG_NAME_SERVICE_REVOKE_TRANSACTION :u32 = 35;
const OBJECT_TAG_NAME_SERVICE_TRANSFER_TRANSACTION :u32 = 36;
const OBJECT_TAG_CONTRACT_CREATE_TRANSACTION :u32 = 42;
const OBJECT_TAG_CONTRACT_CALL_TRANSACTION :u32 = 43;


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
    pub fn from_tag(s: u32) -> Option<TxType> {
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

    pub fn as_str(s: &TxType) -> &'static str {
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

impl Serialize for TxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(TxType::as_str(self))
    }
}

fn process_tx(stx: &RlpVal) -> Value {
    let _type = TxType::from_tag(u32::convert(&stx[0]));
    match _type {
        Some(txType) => parse_tx(stx, txType),
        None => panic!("Wrong Transaction type")
    }

}

fn parse_tx(stx: &RlpVal, tx_type: TxType) -> Value {
    match  tx_type {
        TxType::SignedTx => signed_tx(stx).unwrap(), // TODO
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

    let tx_json = process_tx(&RlpVal::from_rlp(&tx_rlp_val)?);
    Ok(json!(
        {
            "type": TxType::SignedTx,
            "signatures": SignatureList::convert(&stx[2]),
            "tx": tx_json,
        }))
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

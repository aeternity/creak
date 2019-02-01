use serde_json::*;
use crate::rlp_val::*;

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

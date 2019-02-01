use serde_json::*;
use crate::rlp_val::*;


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

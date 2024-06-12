use wasm_bindgen::prelude::*;
use serde::{ Serialize };
use serde_wasm_bindgen::to_value;
use ::{
    bitcoin::{ Transaction, consensus::{ self, Decodable, Encodable, encode::deserialize } },
    ordinals::{ Artifact, Runestone },
    hex
};     

#[wasm_bindgen]
pub fn get_runestone_from_transaction_hex(hex_tx: &str) -> JsValue {
    let tx_bytes = hex::decode(hex_tx).expect("Hex decoding failed");
    let transaction: Transaction = deserialize(&tx_bytes).expect("Transaction deserialization failed");
    let runestone = Runestone::decipher(&transaction);
    if runestone.is_some() {
        let js_value = to_value(&runestone).unwrap();
        return js_value;
    } else {
        return JsValue::undefined();
    }
}

use crate::hashmap;
use std::collections::HashMap;

/// The type-checking sequence in which we must compile the modules.
/// In a 'real' project, this is done using a dependency graph which
/// code lies under aiken-project -- not importable here.
pub const MODULES_SEQUENCE: [&str; 20] = [
    "aiken/crypto",
    "cardano/address",
    "aiken/math",
    "aiken/option",
    "aiken/primitive/bytearray",
    "aiken/primitive/int",
    "aiken/collection",
    "aiken/collection/dict",
    "aiken/collection/list",
    "aiken/math/rational",
    "cardano/assets",
    "cardano/governance/protocol_parameters",
    "aiken/cbor",
    "cardano/certificate",
    "aiken/primitive/string",
    "cardano/governance",
    "aiken/collection/pairs",
    "aiken/interval",
    "cardano/transaction",
    "cardano/script_context",
];

pub fn modules() -> HashMap<&'static str, &'static str> {
    hashmap! {
        "aiken/cbor" => include_str!("../../stdlib/lib/aiken/cbor.ak"),
        "aiken/collection" => include_str!("../../stdlib/lib/aiken/collection.ak"),
        "aiken/collection/dict" => include_str!("../../stdlib/lib/aiken/collection/dict.ak"),
        "aiken/collection/list" => include_str!("../../stdlib/lib/aiken/collection/list.ak"),
        "aiken/collection/pairs" => include_str!("../../stdlib/lib/aiken/collection/pairs.ak"),
        "aiken/crypto" => include_str!("../../stdlib/lib/aiken/crypto.ak"),
        "aiken/interval" => include_str!("../../stdlib/lib/aiken/interval.ak"),
        "aiken/math" => include_str!("../../stdlib/lib/aiken/math.ak"),
        "aiken/math/rational" => include_str!("../../stdlib/lib/aiken/math/rational.ak"),
        "aiken/option" => include_str!("../../stdlib/lib/aiken/option.ak"),
        "aiken/primitive/bytearray" => include_str!("../../stdlib/lib/aiken/primitive/bytearray.ak"),
        "aiken/primitive/int" => include_str!("../../stdlib/lib/aiken/primitive/int.ak"),
        "aiken/primitive/string" => include_str!("../../stdlib/lib/aiken/primitive/string.ak"),
        "cardano/address" => include_str!("../../stdlib/lib/cardano/address.ak"),
        "cardano/assets" => include_str!("../../stdlib/lib/cardano/assets.ak"),
        "cardano/certificate" => include_str!("../../stdlib/lib/cardano/certificate.ak"),
        "cardano/governance" => include_str!("../../stdlib/lib/cardano/governance.ak"),
        "cardano/governance/protocol_parameters" => include_str!("../../stdlib/lib/cardano/governance/protocol_parameters.ak"),
        "cardano/script_context" => include_str!("../../stdlib/lib/cardano/script_context.ak"),
        "cardano/transaction" => include_str!("../../stdlib/lib/cardano/transaction.ak"),
    }
}

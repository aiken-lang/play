use crate::hashmap;
use std::collections::HashMap;

pub const VERSION: &str = "v2.1.0";

pub const MODULES_SEQUENCE: [&str; 1] = ["aiken/fuzz"];

pub fn modules() -> HashMap<&'static str, &'static str> {
    hashmap! {
        "aiken/fuzz" => include_str!("../../fuzz/lib/aiken/fuzz.ak"),
    }
}

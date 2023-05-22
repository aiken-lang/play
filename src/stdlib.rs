pub const MODULES: [(&str, &str); 13] = [
    (
        "aiken/option",
        include_str!("../stdlib/lib/aiken/option.ak"),
    ),
    (
        "aiken/interval",
        include_str!("../stdlib/lib/aiken/interval.ak"),
    ),
    ("aiken/time", include_str!("../stdlib/lib/aiken/time.ak")),
    ("aiken/math", include_str!("../stdlib/lib/aiken/math.ak")),
    ("aiken/hash", include_str!("../stdlib/lib/aiken/hash.ak")),
    (
        "aiken/bytearray",
        include_str!("../stdlib/lib/aiken/bytearray.ak"),
    ),
    ("aiken/dict", include_str!("../stdlib/lib/aiken/dict.ak")),
    ("aiken/int", include_str!("../stdlib/lib/aiken/int.ak")),
    ("aiken/list", include_str!("../stdlib/lib/aiken/list.ak")),
    (
        "aiken/transaction/credential",
        include_str!("../stdlib/lib/aiken/transaction/credential.ak"),
    ),
    (
        "aiken/transaction/certificate",
        include_str!("../stdlib/lib/aiken/transaction/certificate.ak"),
    ),
    (
        "aiken/transaction/value",
        include_str!("../stdlib/lib/aiken/transaction/value.ak"),
    ),
    (
        "aiken/transaction",
        include_str!("../stdlib/lib/aiken/transaction.ak"),
    ),
];

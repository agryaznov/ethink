/// Lookup for a specific field in the provided json output,
/// and try to convert it with $as() to a type required.
#[macro_export]
macro_rules! json_get {
    ( $o:ident$([$k:literal])+.$as:ident() ) => {
           $o.into_iter::<serde_json::Value>()
           .next()
           .expect("blank json output")
           .expect("can't decode json output")$([$k])+
           .$as()
           .expect("can't parse needed value from json output")
    };
}

/// Spawn a node, deploy a contract specified with $path to it,
/// and make post request to its RPC.
#[macro_export]
macro_rules! rpc_rq {
    ( $env:ident, $rq:ident ) => {
        // make ETH RPC request
        Deserializer::from_reader(
            ureq::post($env.http_url().as_str())
                .send_json($rq)
                .expect("ETH RPC request failed")
                .into_reader(),
        )
    };

    ( $env:ident, $rq:tt ) => {
        // make ETH RPC request
        Deserializer::from_reader(
            ureq::post($env.http_url().as_str())
                .send_json(ureq::json!($rq))
                .expect("ETH RPC request failed")
                .into_reader(),
        )
    };
}

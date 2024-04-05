/// Lookup for a specific field in the provided json output,
/// and try to convert it with $as() to a type required.
#[macro_export]
macro_rules! json_get {
    ( $o:ident$([$k:literal])+ ) => {
        to_json_val!($o)$([$k])+
    };
}

#[macro_export]
macro_rules! to_json_val {
    ( $o:ident ) => {
        $o.into_iter::<serde_json::Value>()
            .next()
            .expect("blank json output")
            .expect("can't decode json output")
    };
}

#[macro_export]
macro_rules! ensure_no_err {
    ( &$j:ident ) => {
        if let Some(err) = &$j["error"].as_object() {
            panic!("RPC returned error: {:#?}", err)
        }
    };
}

#[macro_export]
macro_rules! extract_result {
    ( &$j:ident ) => {
        &$j["result"]
            .as_str()
            .expect("RPC returned no result string")
    };
}

/// Spawn a node, deploy a contract specified with $path to it,
/// and make post request to its RPC.
#[macro_export]
macro_rules! rpc_rq {
    ( $env:ident, $rq:ident ) => {
        // make ETH RPC request
        make_rq!($env, $rq)
    };

    ( $env:ident, $rq:tt ) => {
        // make ETH RPC request
        make_rq!($env, $rq)
    };
}

/// Prepare node and contract for testing env
#[macro_export]
macro_rules! prepare_node_and_contract {
    ( $manifest:ident ) => {
        prepare::node_and_contract($manifest, vec![], None).await
    };

    ( $manifest:ident, $args:expr ) => {
        prepare::node_and_contract($manifest, $args, None).await
    };

    ( $manifest:ident, $args:expr, $signer:ident ) => {
        prepare::node_and_contract($manifest, $args, Some($signer)).await
    };
}

#[macro_export]
macro_rules! make_rq {
    ($env:ident, $rq:tt) => {
        Deserializer::from_reader(
            ureq::post($env.http_url().as_str())
                .send_json(ureq::json!($rq))
                .expect("ETH RPC request failed")
                .into_reader(),
        )
    };
}

#[macro_export]
macro_rules! call {
    ($env:ident, $msg:literal) => {
        contracts::call(&$env, $msg, vec![], false, None)
    };
    ($env:ident, $msg:literal, $args:expr) => {
        contracts::call(&$env, $msg, $args, false, None)
    };
    ($env:ident, $msg:literal, $args:expr, $exec:literal) => {
        contracts::call(&$env, $msg, $args, $exec, None)
    };
    ($env:ident, $msg:literal, $args:expr, $exec:literal, $signer:expr) => {
        contracts::call(&$env, $msg, $args, $exec, $signer)
    };
}

#[macro_export]
macro_rules! encode {
    ($path:ident, $msg:literal) => {
        contracts::encode($path, $msg, vec![])
    };
    ($path:ident, $msg:literal, $args:expr) => {
        contracts::encode($path, $msg, $args)
    };
}

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

/// Make a call to contract, ensure success return,
/// and decode its output.
#[macro_export]
macro_rules! contract_call {
    ( $env:ident, $msg:literal, $exec:literal ) => {
        contracts::call(
            $env.ws_url().as_str(),
            &$env.contract_manifest(),
            &$env.contract_address(),
            $msg,
            $exec,
        )
    };
}

/// Prepare node and contract for testing env
#[macro_export]
macro_rules! prepare_node_and_contract {
    ( $manifest:ident ) => {
        prepare::node_and_contract($manifest, None).await
    };

    ( $manifest:ident, $signer:ident ) => {
        prepare::node_and_contract($manifest, Some($signer)).await
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

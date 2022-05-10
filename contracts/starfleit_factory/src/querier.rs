use cosmwasm_std::{Addr, Binary, Deps, Empty, QueryRequest, StdResult, WasmQuery};
use starfleit::asset::PairInfoRaw;

pub fn query_liquidity_token(deps: Deps<Empty>, contract_addr: Addr) -> StdResult<Addr> {
    // load pair_info form the pair contract
    let pair_info: PairInfoRaw = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Raw {
        contract_addr: contract_addr.to_string(),
        key: Binary::from("\u{0}\u{9}pair_info".as_bytes()),
    }))?;

    deps.api.addr_humanize(&pair_info.liquidity_token)
}

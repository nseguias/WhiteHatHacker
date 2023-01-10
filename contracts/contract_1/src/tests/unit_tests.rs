#[cfg(test)]
mod tests {
    use crate::{
        contract::{execute, instantiate, query},
        msg::{BoilerplateResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
    };
    use cosmwasm_std::{
        attr, coin, from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    pub const OWNER: &str = "owner";
    pub const USER: &str = "user";
    pub const DENOM: &str = "denom";
    pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(OWNER, &[coin(100, DENOM)]);
        let msg = InstantiateMsg {
            protocol_fee_bps: 0,
            cw721_code_id: INSTANTIATE_CW721_REPLY_ID,
            name: "NAME".to_string(),
            symbol: "SYMBOL".to_string(),
            minter: "minter".to_string(),
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("contract_owner", OWNER)]
        );
    }

    #[test]
    fn test_boilerplate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(OWNER, &[coin(1_000_000, DENOM)]);
        let msg = InstantiateMsg {
            protocol_fee_bps: 0,
            cw721_code_id: INSTANTIATE_CW721_REPLY_ID,
            name: "NAME".to_string(),
            symbol: "SYMBOL".to_string(),
            minter: "minter".to_string(),
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let execute_msg = ExecuteMsg::Boilerplate {};

        let execute_info = mock_info(USER, &[coin(1_000, DENOM)]);

        let res = execute(
            deps.as_mut(),
            env.clone(),
            execute_info,
            execute_msg.clone(),
        );

        assert_eq!(
            res.unwrap().attributes,
            vec![attr("action", "execute_boilerplate"),]
        );

        // query highest bidder should return new bidder addr2 & 9990000 (10_000_000 - 10_000)
        let query_msg = QueryMsg::Boilerplate {};
        let _res: BoilerplateResponse =
            from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
    }
}

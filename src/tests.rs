use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use crate::contract::{handle, init, query};
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::*;

//static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/stsp.wasm");

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};
    use crate::state::User;
    use crate::contract::{SYSTEM, SYSTEM_ORG, ENTERPRISE, GOVERNMENT, BANK};

    #[test]
    fn create_user() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));
        //let mut deps = mock_instance(WASM, &[]);

        let env = mock_env("creator", &coins(2, "token"));
        let msg = InitMsg { address: env.message.sender.to_string() };
        let _res = init(&mut deps, env.clone(), msg).unwrap();

        let user = User {
            user_id: "laowang".to_string(),
            address:"laowangaddress".to_string(),
            org: "xalhh".to_string(),
            role: vec![ENTERPRISE.to_string()]
        };

        let msg = HandleMsg::CreateUser {user};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();
        let log = _res.log.get(0).unwrap().clone().value;
        let _user: User = serde_json_wasm::from_str(log.as_str()).unwrap();
        assert_eq!("laowang", _user.user_id);
    }

    #[test]
    fn set_application() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let env = mock_env("creator", &coins(2, "token"));
        let msg = InitMsg { address: env.message.sender.to_string() };
        let _res = init(&mut deps, env.clone(), msg).unwrap();

        let user = User {
            user_id: "laowang".to_string(),
            address:"laowangaddress".to_string(),
            org: "xalhh".to_string(),
            role: vec![ENTERPRISE.to_string()]
        };

        let msg = HandleMsg::CreateUser {user};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();

        let env = mock_env("laowangaddress", &coins(2, "token"));

        let new_application = CreateApplication {
            enterprise: "baiyangdian".to_string(),
            time_stamp: 123,
            application_id: "1".to_string(),
            application_type: "xixi".to_string(),
            application_entity: "haha".to_string(),
            data: vec![Data {data_name: "shit".to_string(), data_hash: "wow".to_string()}],
            permission: vec!["laowangaddress".to_string()],
        };

        let msg = HandleMsg::CreateApplication {new_application};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();
        let log = _res.log.get(0).unwrap().clone().value;
        let _application: Application = serde_json_wasm::from_str(log.as_str()).unwrap();
        assert_eq!("1", _application.application_id);
    }

    #[test]
    fn get_application() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let env = mock_env("creator", &coins(2, "token"));
        let msg = InitMsg { address: env.message.sender.to_string() };
        let _res = init(&mut deps, env.clone(), msg).unwrap();

        let user = User {
            user_id: "laowang".to_string(),
            address:"laowangaddress".to_string(),
            org: "xalhh".to_string(),
            role: vec![ENTERPRISE.to_string()]
        };

        let msg = HandleMsg::CreateUser {user};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();

        let user = User {
            user_id: "laolv".to_string(),
            address: "laolvaddress".to_string(),
            org: "xiongxian".to_string(),
            role: vec![GOVERNMENT.to_string()]
        };
        let msg = HandleMsg::CreateUser {user};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();

        let env = mock_env("laowangaddress", &coins(2, "token"));

        let new_application = CreateApplication {
            enterprise: "baiyangdian".to_string(),
            time_stamp: 123,
            application_id: "1".to_string(),
            application_type: "xixi".to_string(),
            application_entity: "haha".to_string(),
            data: vec![Data {data_name: "shit".to_string(), data_hash: "wow".to_string()}],
            permission: vec!["laowangaddress".to_string()],
        };

        let msg = HandleMsg::CreateApplication {new_application};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();
        let log = _res.log.get(0).unwrap().clone().value;
        let _application: Application = serde_json_wasm::from_str(log.as_str()).unwrap();
        assert_eq!("1", _application.application_id);

        let env = mock_env("laowangaddress", &coins(2, "token"));

        let application = Application {
            enterprise: "baiyangdian".to_string(),
            time_stamp: 123,
            application_id: "1".to_string(),
            application_type: "xixi".to_string(),
            application_entity: "haha".to_string(),
            data: vec![Data {data_name: "shit".to_string(), data_hash: "wow".to_string()}],
            permission: vec!["laowangaddress".to_string()],
            result: 2,
            reason: "just don't".to_string(),
        };

        let msg = HandleMsg::AuditApplication {application};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();
        let log = _res.log.get(0).unwrap().clone().value;
        let _application: Application = serde_json_wasm::from_str(log.as_str()).unwrap();
        assert_eq!("xixi", _application.application_type);
    }
}
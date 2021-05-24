use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult,
    Storage, MigrateResponse, log
};
use cosmwasm_std::{Order, KV};
use crate::msg::{
    HandleMsg, InitMsg, MigrateMsg, QueryMsg
};
use crate::state::*;

pub const SYSTEM: &str = "ROLE_0001";// 系统
pub const ENTERPRISE: &str = "ROLE_0002";// 企业
pub const GOVERNMENT: &str = "ROLE_0003";// 政府
pub const BANK: &str = "ROLE_0004";// 银行
pub const SYSTEM_ORG: &str = "SYSTEM";// 系统组织
pub const SYSTEM_ID: &str = "SYSTEM";// 系统编号
pub const APPROVED: u8 = 1;// 通过
pub const REJECTED: u8 = 2;// 驳回

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    if msg.address.is_empty() {
        return  Err(StdError::generic_err("address is empty"));
    }
    let user = User {
        user_id: SYSTEM_ID.to_string(),
        address: msg.address,
        org: SYSTEM_ORG.to_string(),
        role: vec![SYSTEM.to_string()],
    };
    user_store(&mut deps.storage)
        .save(&user.address.as_bytes(), &user)?;
    Ok(InitResponse::default())
}

pub fn migrate<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: MigrateMsg,
) -> StdResult<MigrateResponse> {
    Ok(MigrateResponse::default())
}

/// 合约执行方法入口
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateUser {
            user
        } => create_user(deps, env, user),
        HandleMsg::CreateApplication {
            new_application
        } => create_application(deps, env, new_application),
        HandleMsg::AuditApplication {
            application
        } => audit_application(deps, env, application),
    }
}

/// 合约查询方法入口
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::from(vec![1]))
}

/// 创建用户
pub fn create_user<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    user: User,
) -> StdResult<HandleResponse> {
    if user.user_id.is_empty() {
        return Err(StdError::generic_err("user id is empty"));
    } else if user.address.is_empty() {
        return Err(StdError::generic_err("user address is empty"));
    } else if user.org.is_empty() {
        return Err(StdError::generic_err("user org is empty"));
    } else if user.role.is_empty() {
        return Err(StdError::generic_err("user role is empty"));
    }
    let roles = vec![SYSTEM, ENTERPRISE, GOVERNMENT, BANK];
    for item in user.role.clone() {
        if !roles.contains(&item.as_str()) {
            return Err(StdError::generic_err("user role is wrong"));
        }
    }
    let sender_option = user_store_read(&deps.storage)
        .may_load(_env.message.sender.to_string().as_bytes())?;
    if sender_option.is_none() {
        return Err(StdError::generic_err("message sender empty"));
    }
    if !sender_option.unwrap().role.contains(&SYSTEM.to_string()) {
        return Err(StdError::generic_err("message sender role error"));
    }
    if user.role.contains(&SYSTEM.to_string()) {
        if !user.org.eq(SYSTEM_ORG) {
            return Err(StdError::generic_err("user org is wrong"));
        }
    }
    let user_key = user.address.to_string();
    let user_option = user_store_read(&deps.storage)
        .may_load(user_key.as_bytes())?;
    if user_option.is_some() {
        return Err(StdError::generic_err("user exists"));
    }
    user_store(&mut deps.storage)
        .save(user_key.as_bytes(), &user)?;
    let log_value = serde_json_wasm::to_string(&user).unwrap();

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("create_user", &log_value)],
        data: None,
    })
}

/// 提交申请
pub fn create_application<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    new_application: CreateApplication,
) -> StdResult<HandleResponse> {
    if new_application.enterprise.is_empty() {
        return Err(StdError::generic_err("enterprise is empty"))
    } else if !(new_application.time_stamp > 0) {
        return Err(StdError::generic_err("time stamp is invalid"))
    } else if new_application.application_id.is_empty() {
        return Err(StdError::generic_err("application id is empty"));
    } else if new_application.application_entity.is_empty() {
        return Err(StdError::generic_err("application entity is empty"))
    } else if new_application.application_type.is_empty() {
        return Err(StdError::generic_err("application type is empty"));
    } else if new_application.data.is_empty() {
        return Err(StdError::generic_err("application data is empty"));
    } else if new_application.permission.is_empty() {
        return Err(StdError::generic_err("application permission is empty"));
    }
    let application_key = new_application.application_id.to_string();
    let application_option =  application_store_read(&deps.storage)
        .may_load(application_key.as_bytes())?;
    if application_option.is_some() {
        return Err(StdError::generic_err("application already exist"));
    }
    let user_option = user_store_read(&deps.storage)
        .may_load(_env.message.sender.to_string().as_bytes())?;
    if user_option.is_none() {
        return Err(StdError::generic_err("user not exist"));
    }
    if !user_option.unwrap().role.contains(&ENTERPRISE.to_string()) {
        return Err(StdError::generic_err("permission denied"));
    }
    let application = Application {
        enterprise: new_application.enterprise,
        time_stamp: new_application.time_stamp,
        application_id: new_application.application_id,
        application_type: new_application.application_type,
        application_entity: new_application.application_entity,
        data: new_application.data.clone(),
        permission: new_application.permission.clone(),
        result: 0,
        reason: "".to_string(),
    };
    application_store(&mut deps.storage)
        .save(application_key.as_bytes(), &application)?;
    let log_value = serde_json_wasm::to_string(&application).unwrap();

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("create_application", &log_value)],
        data: None,
    })
}

/// 审核申请
pub fn audit_application<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    application: Application
) -> StdResult<HandleResponse> {
    if !application.permission.contains(&_env.message.sender.to_string()) {
        return Err(StdError::generic_err("permission denied"));
    }
    let application_key = application.application_id.to_string();
    let application_option = application_store_read(&deps.storage)
        .may_load(application_key.as_bytes())?;
    if application_option.is_none() {
        return Err(StdError::generic_err("application not exist"));
    }
    application_store(&mut deps.storage)
        .save(application_key.as_bytes(), &application)?;
    let log_value = serde_json_wasm::to_string(&application).unwrap();

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("audit_application", &log_value)],
        data: None,
    })
}
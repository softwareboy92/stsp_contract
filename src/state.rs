use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub const USER_KEY: &[u8] = b"user";
pub const APPLICATION_KEY: &[u8] = b"application";

pub fn user_store<S: Storage>(storage: &mut S)
    -> Bucket<S, User> {
    bucket(USER_KEY, storage)
}

pub fn user_store_read<S: Storage>(storage: &S)
    -> ReadonlyBucket<S, User> {
    bucket_read(USER_KEY, storage)
}

pub fn application_store<S: Storage>(storage: &mut S)
    -> Bucket<S, Application> {
    bucket(APPLICATION_KEY, storage)
}

pub fn application_store_read<S: Storage>(storage: &S)
    -> ReadonlyBucket<S, Application> {
    bucket_read(APPLICATION_KEY, storage)
}

/// 用户数据项
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    // 用户编号
    pub user_id: String,
    // 用户地址
    pub address: String,
    // 用户组织
    pub org: String,
    // 用户角色
    pub role: Vec<String>,
}

/// 申请
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Application {
    // 申请编号
    pub application_id: String,
    // 申请类别
    pub application_type: String,
    // 数据条目
    pub data: Vec<Data>,
    // 访问权限
    pub permission: Vec<String>,
}

/// 共享数据项
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Data {
    // 数据项名字
    pub data_name: String,
    // 数据项哈希
    pub data_hash: String,
}
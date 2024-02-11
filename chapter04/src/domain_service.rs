// ドメインサービスとは

#![allow(dead_code)]

use anyhow::{bail, Error};

// user id の実装
#[derive(Debug, Clone, PartialEq, Eq)]
struct UserId {
    value: String,
}

impl UserId {
    fn new(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

// user name の実装
struct UserName {
    value: String,
}

impl UserName {
    fn new(s: &str) -> Result<Self, Error> {
        if s.len() >= 3 {
            Ok(Self {
                value: s.to_string(),
            })
        } else {
            bail!("ユーザー名は 3 文字以上である必要があります。")
        }
    }
}

// user の実装
struct User {
    user_id: UserId,
    user_name: UserName,
}

impl User {
    fn new(user_id: UserId, user_name: UserName) -> Self {
        Self { user_id, user_name }
    }

    fn change_username(&mut self, user_name: UserName) {
        self.user_name = user_name;
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for User {}

// user service の実装
#[derive(Debug)]
struct UserService {}

impl UserService {
    fn exists(_user: User) {
        // レポジトリ層での実装が必要。
        // SQL 文など冗長なため、5 章以降の内容を踏まえて実装したい。
        // ここでは、未実装に留めておく。
        unimplemented!()
    }
}

// 値オブジェクトを採用するモチベーション

#![allow(dead_code)]
use anyhow::Error;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct UserId {
    value: String,
}

impl FromStr for UserId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 1 {
            Ok(UserId {
                value: s.to_string(),
            })
        } else {
            Err(Error::msg(
                "ユーザー ID は 1 文字以上でなければいけません。",
            ))
        }
    }
}

#[derive(Debug, Clone)]
struct UserName {
    value: String,
}

impl FromStr for UserName {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 3 {
            Ok(UserName {
                value: s.to_string(),
            })
        } else {
            Err(Error::msg("ユーザー名は 3 文字以上でなければいけません。"))
        }
    }
}

struct User {
    user_id: UserId,
    user_name: UserName,
}

impl User {
    fn new(user_id: &UserId, user_name: &UserName) -> Result<Self, Error> {
        Ok(User {
            user_id: user_id.clone(),
            user_name: user_name.clone(),
        })
    }
}

fn create_user(name: &str) {
    let user_name = UserName {
        value: name.to_string(),
    };

    let user_id = UserId {
        value: "test".to_string(),
    };

    let _user = User::new(&user_id, &user_name);
}

fn update_user(name: &str) {
    let user_name = UserName {
        value: name.to_string(),
    };

    let user_id = UserId {
        value: "test".to_string(),
    };

    let _user = User::new(&user_id, &user_name);
}

// ライフサイクルのあるオブジェクト「エンティティ」
#![allow(dead_code)]

use std::str::FromStr;

use anyhow::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserId {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserName {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    user_id: UserId,
    user_name: UserName,
}

impl User {
    fn new(user_id: UserId, user_name: &str) -> Result<Self, Error> {
        Ok(User {
            user_id,
            user_name: user_name.parse::<UserName>()?,
        })
    }
}

impl FromStr for UserName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 3 {
            Ok(UserName {
                value: s.to_string(),
            })
        } else {
            Err(Error::msg("ユーザー名は 3 文字以上である必要があります。"))
        }
    }
}

fn check(left_user: &User, right_user: &User) -> String {
    if left_user.user_id == right_user.user_id {
        "同一のユーザーです。".to_string()
    } else {
        "別のユーザーです。".to_string()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn is_same_user() {
        let user_id = UserId {
            value: "1234".to_string(),
        };
        let user_name = UserName {
            value: "John".to_string(),
        };
        let user = User { user_id, user_name };

        assert_eq!(
            check(&user, &user.clone()),
            "同一のユーザーです。".to_string()
        );
    }

    #[test]
    fn is_invalid_user() {
        let user_id = UserId {
            value: "1234".to_string(),
        };
        let user_name = "J";
        match User::new(user_id, user_name) {
            Ok(_) => panic!("バリデーションが失敗すべきところ、成功しました。"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "ユーザー名は 3 文字以上である必要があります。"
                )
            }
        }
    }
}

// gen by @3-/i18n-rust ; DON'T EDIT

use conn::{
  fred::interfaces::{HashesInterface, RedisResult},
  KV,
};

pub const SIGN_UP: &'static [u8] = b"signUp";

pub const RESET_PASSWORD: &'static [u8] = b"resetPassword";

pub const MAIL: &'static [u8] = b"mail";

pub const INVALID: &'static [u8] = b"invalid";

pub const CODE: &'static [u8] = b"code";

pub const ACCOUNT_EXIST: &'static [u8] = b"accountExist";

pub const ACCOUNT_NOT_EXIST: &'static [u8] = b"accountNotExist";

pub const PASSWORD_ERROR: &'static [u8] = b"passwordError";

pub const VERIFY_MAIL: &'static [u8] = b"verifyMail";

lang::gen!(auth);

#[macro_export]
macro_rules! throw {

  ($header:ident,$id:ident,$key:ident) => {{
    crate::i18n::throw(&$header, stringify!($id), crate::i18n::$key).await?;
    unreachable!()
  }};

  ($header:ident,$id:ident,$($key:ident),+) => {{
    crate::i18n::throw_li(&$header, stringify!($id), &[
      $(crate::i18n::$key),+
    ]).await?;
    unreachable!()
  }};

}
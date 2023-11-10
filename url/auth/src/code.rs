use sk::sk;

pub fn code(account: impl AsRef<str>, password: impl AsRef<str>, hour: u64) -> String {
  util::hash::token(
    &[
      account.as_ref().as_bytes(),
      password.as_ref().as_bytes(),
      &hour.to_le_bytes()[..],
      sk(),
    ]
    .concat(),
  )
}

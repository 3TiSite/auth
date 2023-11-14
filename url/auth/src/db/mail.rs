use anyhow::Result;

use crate::{db::code, i18n, throw};

pub const CODE: &str = "${code}";

pub async fn send(
  kind: &[u8],
  lang: &str,
  host: &str,
  account: String,
  password: String,
) -> Result<()> {
  let li = i18n::get_li(lang, &[i18n::VERIFY_MAIL, kind]).await?;

  let code = code::gen(kind, &account, password, util::hours());

  let mail = li[0].replace("${action}", &li[1]);
  let txt = mail.replace(CODE, &code);
  let title = format!("[{host}] {}", &txt[0..txt.find('\n').unwrap()]);
  let htm = util::mail::htm(mark::htm(mail.replace(
                CODE,
                &format!("<b style=\"background:#ff0;border:1px dashed #f90;font-weight:bold;padding:8px;font-family:Consolas,Monaco,monospace\">{code}</b>")
    )));
  smtp::send(host, account, title, txt, htm);
  Ok(())
}

pub async fn host_send(
  kind: &[u8],
  header: &t3::HeaderMap,
  host: &str,
  account: String,
  password: String,
) -> t3::Result<()> {
  use email_address::EmailAddress;
  let lang = lang::header(&header);
  if EmailAddress::is_valid(&account) {
    send(kind, lang, &host, account, password).await?;
    return Ok(().into());
  }
  throw!(header, account, MAIL, INVALID);
}

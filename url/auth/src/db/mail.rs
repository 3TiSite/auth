use anyhow::Result;
use email_address::EmailAddress;

use crate::{db::code, i18n, throw};

pub const CODE: &str = "${code}";

pub async fn send_txt_htm(
  host: &str,
  account: impl Into<String>,
  txt: impl AsRef<str>,
  htm: impl AsRef<str>,
) -> anyhow::Result<()> {
  let account = account.into();
  let txt = txt.as_ref();
  let title = format!("[{host}] {}", &txt[0..txt.find('\n').unwrap()]);
  let htm = htm.as_ref();
  smtp::send(host, account, title, txt, htm);
  Ok(())
}

pub async fn send_code(
  kind: &[u8],
  li: Vec<String>,
  host: &str,
  account: impl AsRef<str>,
  token: impl AsRef<str>,
) -> Result<()> {
  let account = account.as_ref();
  let code = code::gen(kind, account, &token, util::hours());
  let mail = li[0].replace("${action}", &li[1]);

  let txt = mail.replace(CODE, &code);
  let mail = mail.replace(
        CODE,
        &format!("<b style=\"background:#ff0;border:1px dashed #f90;font-weight:bold;padding:8px;font-family:Consolas,Monaco,monospace\">{code}</b>")
    );
  let htm = util::mail::htm(mark::htm(mail));

  send_txt_htm(host, account, txt, htm).await
}

pub async fn email_vaild(header: &t3::HeaderMap, mail: impl AsRef<str>) -> t3::Result<String> {
  let mail = mail.as_ref();
  if !EmailAddress::is_valid(mail) {
    throw!(header, account, MAIL, INVALID)
  }
  Ok(mail.to_owned())
}

pub async fn host_send_with_suffix(
  kind: &[u8],
  header: &t3::HeaderMap,
  host: &str,
  account: impl AsRef<str>,
  token: impl AsRef<str>,
  suffix: &str,
) -> t3::Result<()> {
  let account = email_vaild(header, account).await?;
  let lang = lang::header(header);
  let mut li = i18n::get_li(lang, &[i18n::VERIFY_MAIL, kind]).await?;

  let txt = &li[0];

  if let Some(p) = txt.find('\n') {
    let title = &txt[..p];
    let title = format!("{title} ( {suffix} )");
    li[0] = title + &txt[p..];
  }

  send_code(kind, li, host, account, token).await?;
  Ok(())
}

pub async fn host_send(
  kind: &[u8],
  header: &t3::HeaderMap,
  host: &str,
  account: impl AsRef<str>,
  token: impl AsRef<str>,
) -> t3::Result<()> {
  let account = email_vaild(header, account).await?;
  let lang = lang::header(header);
  let li = i18n::get_li(lang, &[i18n::VERIFY_MAIL, kind]).await?;
  send_code(kind, li, host, account, token).await?;
  Ok(())
}

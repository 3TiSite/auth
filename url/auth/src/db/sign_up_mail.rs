use anyhow::Result;

use crate::{code, i18n};

pub const CODE: &str = "${code}";

pub async fn sign_up_mail(lang: &str, host: &str, account: String, password: String) -> Result<()> {
  let li = i18n::get_li(lang, &[i18n::VERIFY_MAIL, i18n::SIGN_UP]).await?;

  let code = code(&account, password, util::hours());

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

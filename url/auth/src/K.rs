use kfn::kfn;

pub const HOST_ID: &[u8] = b"hostId";
pub const MAIL_ID: &[u8] = b"mailId";
pub const NAME: &[u8] = b"name";
pub const UID: &[u8] = b"uid";
pub const UID_ACCOUNT: &[u8] = b"{uid}account";
pub const UID_HOST: &[u8] = b"{uid}host";
pub const UID_PASSWD: &[u8] = b"{uid}passwd";
pub const LANG: &[u8] = b"lang";

kfn!(
hostMail{uid} // prefix:host > mailId - userId
mail{uid}
clientUid
uidClient
);

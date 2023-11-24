pub fn truncate(s: impl AsRef<str>) -> String {
  let s = s.as_ref();
  s[..s.len().min(32)].into()
}

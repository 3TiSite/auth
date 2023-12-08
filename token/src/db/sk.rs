use rand::Rng;

pub fn bin(sk: u64, day: u64, uid: u64) -> Vec<u8> {
  vb::e([sk, day, uid])
}

pub fn day(uid: u64) -> (u64, u64, Vec<u8>) {
  let sk: u64 = rand::thread_rng().gen_range(0..9007199254740991);
  let day = sts::day();
  (sk, day, bin(sk, day, uid))
}

pub fn b64(token_id: u64, sk: u64, day: u64) -> String {
  ub64::b64e(vb::e([sk, day, token_id]))
}

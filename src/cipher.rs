use ring::aead;

const NONCE: [u8; 12] = *b"511520511520";
const JEY: [u8; 32] = *b"Q0U1Q0FGRjgzRDcxhFNjRjU1NjA4MDg=";

#[allow(dead_code)]
pub fn encode(raw: &str) -> String {
    let mut cipher_vec = String::from(raw).as_bytes().to_vec();

    let key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &JEY).unwrap();
    let nonce = aead::Nonce::assume_unique_for_key(NONCE);

    let sealing_key = aead::LessSafeKey::new(key);
    sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut cipher_vec).unwrap();

    let res: String = cipher_vec.iter()
        .map(|c| format!("-{}", c))
        .collect();
    format!("0o://{}", &res[1..res.len()])
}

pub fn decode(encoded: &str) -> String {
    let encoded_bytes: Vec<u8> = encoded[5..]
        .split('-')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut cipher_vec = encoded_bytes;

    let key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &JEY).unwrap();
    let nonce = aead::Nonce::assume_unique_for_key(NONCE);

    let opening_key = aead::LessSafeKey::new(key);
    let decrypted_data = opening_key
        .open_in_place(nonce, aead::Aad::empty(), &mut cipher_vec)
        .unwrap();

    std::str::from_utf8(decrypted_data).unwrap().to_string()
}

pub fn get_host() -> String {
    const PRV: &str = "0o://216-144-160-88-2-144-99-32-163-65-159-38-136-240-111-195-78-125-28-202-42-194-166-255-143-7-104-179-65-67-203-66-127-247-31-2-251-19-40-45-46-211-142-156-197";

    decode(&PRV)
}
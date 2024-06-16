use anyhow::Result;
use sha2::{Digest, Sha256};

pub fn hash<T: AsRef<[u8]>>(input: T) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_ref());
    let result = hasher.finalize();
    let mut res_str = String::new();
    for byte in result.iter() {
        res_str.push_str(&format!("{:x}", byte));
    }

    let _ = res_str.split_off(8);

    Ok(res_str)
}

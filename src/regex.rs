use anyhow::Result;
use regex::Regex;

const URL_REGEX: &str = "(http(s)?:\\/\\/.)?(www\\.)?[-a-zA-Z0-9@:%._\\+~#=]{2,256}\\.[a-z]{2,6}\\b([-a-zA-Z0-9@:%_\\+.~#?&//=]*)";

pub fn is_url<T: AsRef<str>>(url: T) -> Result<bool> {
    let re = Regex::new(URL_REGEX)?;
    let url = url.as_ref();
    Ok(re.is_match(url))
}

pub fn skkserv_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
    )
}

pub fn skkserv_host(address: &str) -> String {
    format!("{}", address)
}

pub fn convert(s: &str) -> String {
    // TODO: 実装
    "4\n".to_string()
}

pub fn complete(_req: &str) -> String {
    // 未対応
    let res = "4\n".to_string();
    return res;
}

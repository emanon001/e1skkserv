use crate::converter::{Emanon001Converter, SkkConverter};

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

pub fn convert(src: &str) -> String {
    let converters: Vec<Box<dyn SkkConverter>> = vec![Box::new(Emanon001Converter::new())];
    let candidates = converters
        .into_iter()
        .flat_map(|c| c.convert(src))
        .flatten()
        .collect::<Vec<String>>();
    if candidates.is_empty() {
        return "4\n".to_string();
    } else {
        // TODO: スラッシュが含まれている場合の対応
        return format!("1/{}/\n", candidates.join("/"));
    }
}

pub fn complete(_req: &str) -> String {
    // 未対応
    let res = "4\n".to_string();
    return res;
}

mod wget;
mod error;
mod ftd_lib;

fn main() {
    let data = std::fs::read_to_string("test-config/wget-final.ftd").unwrap();
    println!("{:?}", parse_ftd(data.as_str()));
}

fn parse_ftd(data: &str) -> wget::WgetConfig {
    match wget::WgetConfig::parse(data) {
        Ok(d) => d,
        Err(err) => {
            println!("{}", err.to_string());
            wget::WgetConfig::default()
        },
    }
}


#[cfg(test)]
mod tests {

    fn get_data(fp: &str) -> String {
        std::fs::read_to_string(fp).unwrap()
    }

    fn get_config() -> super::wget::Config {
        super::wget::Config::create(true, "mozilla/1.4".to_string())
    }

    fn get_header(key: Option<String>, value: Option<String>) -> super::wget::Header {
        super::wget::Header::create(key.unwrap_or("default-key".to_string()), value.unwrap_or("default-value".to_string()))
    }

    fn get_headers() -> Vec<super::wget::Header> {
        let mut headers = vec![];
        headers.push(get_header(Some("key1".to_string()), Some("value1".to_string())));
        headers.push(get_header(Some("key2".to_string()), Some("value2".to_string())));
        headers
    }

    fn get_proxy() -> super::wget::Proxy {
        super::wget::Proxy::create(
            "127.0.0.1".to_string(),
            "3000".to_string(),
            "user".to_string(),
            super::wget::PassOrPat::Password{password: "pass".to_string()},
        )
    }

    #[test]
    fn test_wget_config() {
        pretty_assertions::assert_eq!(
            super::parse_ftd(get_data("test-config/wget-final.ftd").as_str()),
            super::wget::WgetConfig::create(
                true,
                get_config(),
                get_proxy(),
                get_headers(),
            )
        )
    }
}

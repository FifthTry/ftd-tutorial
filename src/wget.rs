
#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    auto_resume: bool,
    user_agent: String,
}

impl Config {
    pub fn create (auto_resume: bool, user_agent: String) -> Self {
        Config {
            auto_resume,
            user_agent,
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum PassOrPat {
    #[serde(rename = "password")]
    Password { password: String },
    #[serde(rename = "pat")]
    Pat { pat: String },
    #[serde(rename = "empty")]
    Empty,
}

impl Default for PassOrPat {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Proxy {
    server: String,
    port: String,
    username: String,
    auth: Vec<PassOrPat>,
}

impl Proxy {
    pub fn create (server: String, port: String, username: String, auth: PassOrPat) -> Self {
        Proxy {
            server,
            port,
            username,
            auth: vec![auth],
        }
    }
}


#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    pub fn create (key: String, value: String) -> Self {
        Header {
            key,
            value,
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WgetConfig {
    verbose: bool,
    config: Config,
    proxy: Proxy,
    headers: Vec<Header>
}


impl WgetConfig {

    pub fn create(
        verbose: bool,
        config: Config,
        proxy: Proxy,
        headers: Vec<Header>,
    ) -> Self {
        WgetConfig {
            verbose,
            config,
            proxy,
            headers
        }
    }


    pub fn parse(
        body: &str
    ) -> Result<Self, crate::error::ParseError> {
        let id = "random-id";
        let library = crate::ftd_lib::Library {};
        let b = match ftd::p2::Document::without_render(id, body, &library) {
            Ok(v) => v,
            Err(e) => {
                println!("failed to parse {}: {:?}\n\n{}", id, &e, body);
                return Err(e.into());
            }
        };
        let verbose: bool = match b.get("wget#verbose")? {
            Some(v) => {
                v
            },
            None => false,
        };
        let config: Config = match b.only_instance("wget#config")? {
            Some(c) => {
                c
            },
            None => unimplemented!("no config present!")
        };
        let proxy: Proxy = match b.only_instance("wget#proxy")? {
            Some(p) => {
                p
            },
            None => unimplemented!("no proxy present!")
        };
        let headers: Vec<Header> = {
            let mut headers = vec![];
            for h in b.instances::<Header>("wget#header")?.into_iter() {
                headers.push(h);
            }
            headers
        };
        Ok(WgetConfig {
            verbose,
            config,
            proxy,
            headers,
        })
    }
}

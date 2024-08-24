use std::str::FromStr;

///HTTP Version
#[derive(Debug, Clone)]
pub enum Version {
    HTTP1_1,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(input: &str) -> Result<Version, Self::Err> {
        match input {
            "HTTP/1.1" => Ok(Version::HTTP1_1),
            _ => Err(()),
        }
    }
}
impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Version::HTTP1_1 => "HTTP/1.1",
        }
    }
}

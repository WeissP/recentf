use anyhow::{anyhow, Context, Error, Result};
use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

/// split tramp_path to tramp_prefix and file_path
pub fn split(tramp_path: &str) -> (&str, &str) {
    tramp_path.rsplit_once(':').unwrap_or(("", tramp_path))
}

pub struct TrampPath {
    pub prefix: Prefix,
    pub file_path: PathBuf,
}

impl FromStr for TrampPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (prefix, path) = split(s);
        if !prefix.starts_with('/') {
            return Err(anyhow!("tramp prefix must start with /"));
        }
        Ok(Self {
            prefix: Prefix::from_str(&prefix[1..])?,
            file_path: PathBuf::from(path),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Prefix(Vec<Piece>);

impl FromStr for Prefix {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let res = s
            .split('|')
            .map(Piece::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Prefix(res))
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<_> = self.0.iter().map(|x| x.to_string()).collect();
        f.write_str("/")?;
        f.write_str(&v.join("|"))
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    method: Method,
    user: String,
    host: String,
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (method, rest) = s.split_once(':').context("invalid tramp prefix")?;
        let method = Method::from_str(method)?;
        let (user, host) = rest.split_once('@').context("invalid tramp prefix")?;
        Ok(Piece {
            method,
            user: user.to_string(),
            host: host.to_string(),
        })
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}@{}", self.method, self.user, self.host)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Method {
    Ssh,
    Sudo,
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ssh" => Ok(Self::Ssh),
            "sudo" => Ok(Self::Sudo),
            _ => Err(anyhow::anyhow!("invalid mathod")),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Method::Ssh => "ssh",
            Method::Sudo => "sudo",
        };
        f.write_str(s)
    }
}

pub type Alias = String;
pub type AliasMap = HashMap<Alias, String>;

pub fn pretty_print(paths: Vec<TrampPath>) -> () {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn piece_parse_test() {
        assert_eq!(
            Piece::from_str("sudo:root@localhost").unwrap().to_string(),
            "sudo:root@localhost"
        )
    }

    #[test]
    fn parse_test() {
        let p = TrampPath::from_str(
            "/ssh:weiss@192.168.8.31|sudo:root@192.168.8.31:/lib/systemd/system/syncthing@.service",
        )
        .unwrap();
        assert_eq!(
            p.file_path.to_str().unwrap(),
            "/lib/systemd/system/syncthing@.service"
        );
        assert_eq!(p.prefix.0[0].to_string(), "ssh:weiss@192.168.8.31");
        assert_eq!(
            p.prefix.to_string(),
            "/ssh:weiss@192.168.8.31|sudo:root@192.168.8.31"
        );
    }
}

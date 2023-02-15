use anyhow::Result;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct Upsert<'a> {
    pub filepath: &'a str,
    pub tramp_type: &'a str,
    pub tramp_path: &'a str,
}

impl<'a> Upsert<'a> {
    pub fn new(filepath: &'a str, tramp_type: &'a str, tramp_path: &'a str) -> Self {
        Self {
            filepath,
            tramp_type,
            tramp_path,
        }
    }

    pub fn insert(&self) -> Result<()> {
        let id = hisfile::add_tramp(self.tramp_type, self.tramp_path)?;
        hisfile::upsert(hisfile::PK::new(id, self.filepath))
    }
}

fn is_emacs_path(s: &str) -> bool {
    s.contains(":/") && s.contains('@')
}
// /ssh:weiss@192.168.8.31|sudo:root@192.168.8.31:/lib/systemd/system/syncthing@.service
impl<'a> From<&'a str> for Upsert<'a> {
    fn from(s: &'a str) -> Self {
        assert!(s.starts_with('/'), "invalid path: {}", s);
        if is_emacs_path(s) {
            let (tramp, filepath) = s.rsplit_once(':').unwrap();
            let (tramp_type, tramp_path) = tramp[1..]
                .split_once(':')
                .expect(format!("invalid tramp head: {:?}", tramp).as_str());
            Upsert::new(filepath, tramp_type, tramp_path)
        } else {
            Upsert {
                filepath: s,
                ..Default::default()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Upsert;

    #[test]
    fn from() {
        assert_eq!(
            Upsert::from("/sudo:root@localhost:~/Downloads/test.txt"),
            Upsert::new("~/Downloads/test.txt", "sudo", "root@localhost")
        );
        assert_eq!(
            Upsert::from("~/Downloads/test.txt"),
            Upsert::new("~/Downloads/test.txt", "", "")
        );
    }
}

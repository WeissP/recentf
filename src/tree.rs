use hisfile::Tramp;
use pathtrie;

use crate::scored_paths::ScoredPaths;

#[derive(Debug, Clone)]
pub struct Fmt {
    pub indent: &'static str,
    pub prefix: String,
    pub width: usize,
    pub sep: &'static str,
    pub tramp: Tramp,
}

impl Fmt {
    pub fn new(
        indent: &'static str,
        prefix: String,
        width: usize,
        sep: &'static str,
        tramp: Tramp,
    ) -> Self {
        Self {
            indent,
            prefix,
            width,
            sep,
            tramp,
        }
    }

    pub fn update_tramp(&mut self, tramp: Tramp) {
        self.tramp = tramp;
        self.prefix = match self.tramp.tramp_type.as_str() {
            "sudo" => "♔ ".to_owned(),
            "ssh" => format!("[{}] ", self.tramp.id.0),
            _ => String::new(),
        }
    }
}

impl<'a> pathtrie::format::Format for Fmt {
    fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    fn single_indent(&self) -> &str {
        self.indent
    }

    fn max_width(&self) -> usize {
        self.width
    }

    fn suffix(&self, segs: pathtrie::Segs, _level_stack: &Vec<usize>) -> String {
        let sep = self.sep;
        let path = segs.join("/");
        if self.tramp.is_empty() {
            format!("{sep}/{path}")
        } else {
            let tramp_type = &self.tramp.tramp_type;
            let tramp_path = &self.tramp.tramp_path;
            format!("{sep}/{tramp_type}:{tramp_path}:/{path}")
        }
    }
}

impl<'a> Default for Fmt {
    fn default() -> Self {
        Fmt::new("ǁ    ", String::new(), 50, "』𝔰𝔢𝔭『", Tramp::default())
    }
}

#[derive(Default)]
pub struct Tree<'a>(pub pathtrie::Tree<'a, (), Fmt>);

impl<'a> TryFrom<&'a ScoredPaths<'a>> for Tree<'a> {
    type Error = hisfile::Error;

    fn try_from(sps: &'a ScoredPaths<'a>) -> Result<Self, Self::Error> {
        let mut fmt = Fmt::default();
        fmt.update_tramp(Tramp::try_from(sps.tramp_id)?.into());
        let vv = sps.paths.iter().map(|x| &x.fullpath).collect::<Vec<_>>();
        Ok(Tree(pathtrie::Tree {
            root: pathtrie::node::Node::from(vv),
            fmt,
        }))
    }
}

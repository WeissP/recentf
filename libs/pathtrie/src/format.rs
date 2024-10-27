pub use crate::Segs;

pub trait Format {
    fn prefix(&self) -> &str;
    fn single_indent(&self) -> &str;
    fn max_width(&self) -> usize;
    fn suffix(&self, segs: Segs, level_stack: &Vec<usize>) -> String;

    fn indent(&self, level: usize) -> String {
        self.single_indent().repeat(level)
    }

    fn print_seg<'a>(
        &self,
        path: Segs<'a>,
        level_stack: &Vec<usize>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let prefix = &self.prefix();
        let indent = self.indent(level_stack.len() - 1);
        let path_seg = path[*level_stack.last().unwrap()..].join("/");
        let suffix = self.suffix(path, level_stack);
        write!(f, "{prefix}{indent}{path_seg}{suffix}\n")
    }
}

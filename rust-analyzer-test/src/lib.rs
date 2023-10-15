use std::fmt;
use std::fmt::Debug;

/// https://rust-analyzer.github.io/manual.html#go-to-definition
/// Functions accessible by LSP server: https://github.dev/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/main_loop.rs#L656
/// Where GlobalState is defined: https://github.dev/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/main_loop.rs#L656
/// Arch: https://github.com/rust-lang/rust-analyzer/blob/853fb44a24b8d3341f52747caa949013121b24b4/docs/dev/architecture.md#cratesrust-analyzer
///
/// Example of LSP server: https://github.com/rust-lang/rust-analyzer/blob/853fb44a24b8d3341f52747caa949013121b24b4/lib/lsp-server/examples/goto_def.rs#L53
///
/// In macro: if server is not online, use Arc<> to turn on server (so multiple server are not started across macro invocations)
/// If cargo crate is updated: need to call `reload` on the server
///
/// Open Questions: How to start server with cargo context
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Example<'a> {
    field: usize,
    string: &'a str
}

impl<'a> fmt::Display for Example<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Example {{ field: {}, string: {} }}", self.field, self.string)
    }
}

impl<'a> Debug for Example<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Example {{ field: {}, string: {} }}", self.field, self.string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let example = Example { field: 1, string: "hello" };
        assert_eq!(result, 4);
    }
}

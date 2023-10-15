use syn::Path;

enum MacroQueryType { Mod, Struct, Enum, Fn }

// TODO: Set up Path parameter to work here ...
macro_rules! query_cargo_src {
    (mod $p:path) => {
        use cargo_src_query::macros::process_macro;
        process_macro(MacroQueryType::Mod, $p);
    };
    (struct $p:path) => {};
    (enum $p:path) => {};
    (fn $p:path) => { };
}

# Conteúdo de `mod.rs`

adicione essas configurações em typstlibrary/src/loading/mod.rs


#[path = "sqlite.rs"]
mod sqlite_;

pub use self::sqlite_::*;

global.define_func::<sqlite>();

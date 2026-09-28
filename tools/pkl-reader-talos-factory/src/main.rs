//! Pkl external resource reader binary, run by the `pkl` CLI (via this repo's root
//! `PklProject`) to service `read()` calls under custom URI schemes it can't handle itself.
//!
//! Currently serves just one scheme, [`factory::TalosFactoryReader`]; a new scheme is added by
//! implementing [`protocol::ResourceReader`] and passing it to [`protocol::serve`] here.

mod factory;
mod protocol;

fn main() -> anyhow::Result<()> {
    protocol::serve(&factory::TalosFactoryReader, std::io::stdin().lock(), std::io::stdout())
}

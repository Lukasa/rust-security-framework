//! OSX specific extensions.

pub mod certificate;
pub mod import_export;
mod identity;
mod item;
pub mod secure_transport;
pub mod keychain;

#[cfg(test)]
pub mod test {
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;

    use item::{ItemSearchOptions, ItemClass, Reference};
    use os::macos::keychain::SecKeychainExt;
    use identity::SecIdentity;
    use keychain::SecKeychain;

    pub fn identity(dir: &Path) -> SecIdentity {
        // FIXME https://github.com/rust-lang/rust/issues/30018
        let keychain = keychain(dir);
        let mut items = p!(ItemSearchOptions::new()
                               .class(ItemClass::Identity)
                               .keychains(&[keychain])
                               .search());
        match items.pop().unwrap().reference {
            Some(Reference::Identity(identity)) => identity,
            _ => panic!("expected identity"),
        }
    }

    pub fn keychain(dir: &Path) -> SecKeychain {
        let path = dir.join("server.keychain");
        let mut file = p!(File::create(&path));
        p!(file.write_all(include_bytes!("../../../test/server.keychain")));
        drop(file);

        let mut keychain = p!(SecKeychain::open(&path));
        p!(keychain.unlock(Some("password123")));
        keychain
    }
}

//! Keychain support

use core_foundation::base::TCFType;
use security_framework_sys::base::SecKeychainRef;
use security_framework_sys::keychain::*;
use std::mem;

make_wrapper! {
    /// A type representing a keychain.
    struct SecKeychain, SecKeychainRef, SecKeychainGetTypeID
}

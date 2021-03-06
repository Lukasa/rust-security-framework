//! Trust evaluation support.

use core_foundation_sys::base::Boolean;
use core_foundation::base::TCFType;
use core_foundation::array::CFArray;
use security_framework_sys::trust::*;
use std::mem;

use cvt;
use base::Result;
use certificate::SecCertificate;

/// The result of trust evaluation.
pub enum TrustResult {
    /// An invalid setting or result.
    Invalid,
    /// You may proceed.
    Proceed,
    /// Indicates a denial by the user, do not proceed.
    Deny,
    /// The certificate is implicitly trusted.
    Unspecified,
    /// Indicates a trust policy failure that the user can override.
    RecoverableTrustFailure,
    /// Indicates a trust policy failure that the user cannot override.
    FatalTrustFailure,
    /// An error not related to trust validation.
    OtherError,
}

impl TrustResult {
    fn from_raw(raw: SecTrustResultType) -> TrustResult {
        match raw {
            kSecTrustResultInvalid => TrustResult::Invalid,
            kSecTrustResultProceed => TrustResult::Proceed,
            kSecTrustResultDeny => TrustResult::Deny,
            kSecTrustResultUnspecified => TrustResult::Unspecified,
            kSecTrustResultRecoverableTrustFailure => TrustResult::RecoverableTrustFailure,
            kSecTrustResultFatalTrustFailure => TrustResult::FatalTrustFailure,
            kSecTrustResultOtherError => TrustResult::OtherError,
            raw => panic!("unexpected value {}", raw),
        }
    }

    /// Returns true if the result if "successful" - specifically `Proceed` or
    /// `Unspecified`.
    pub fn success(&self) -> bool {
        match *self {
            TrustResult::Proceed | TrustResult::Unspecified => true,
            _ => false,
        }
    }
}

make_wrapper! {
    /// A type representing a trust evaluation for a certificate.
    struct SecTrust, SecTrustRef, SecTrustGetTypeID
}

impl SecTrust {
    /// Sets additional anchor certificates used to validate trust.
    pub fn set_anchor_certificates(&mut self, certs: &[SecCertificate]) -> Result<()> {
        let certs = CFArray::from_CFTypes(&certs);

        unsafe { cvt(SecTrustSetAnchorCertificates(self.0, certs.as_concrete_TypeRef())) }
    }

    /// If set to `true`, only the certificates specified by
    /// `set_anchor_certificates` will be trusted, but not globally trusted
    /// certificates.
    pub fn set_trust_anchor_certificates_only(&mut self, only: bool) -> Result<()> {
        unsafe { cvt(SecTrustSetAnchorCertificatesOnly(self.0, only as Boolean)) }
    }

    /// Evaluates trust.
    pub fn evaluate(&self) -> Result<TrustResult> {
        unsafe {
            let mut result = kSecTrustResultInvalid;
            try!(cvt(SecTrustEvaluate(self.0, &mut result)));
            Ok(TrustResult::from_raw(result))
        }
    }
}

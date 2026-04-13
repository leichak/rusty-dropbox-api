#[macro_export]
macro_rules! implement_utils {
    ($req_type:ty, $payload_type:ty) => {
        impl Utils<'_> for $req_type {
            type T = $payload_type;
            fn payload(&self) -> Option<&Self::T> {
                if self.payload.is_some() {
                    return Some(self.payload.as_ref().unwrap());
                }
                None
            }
        }

        #[cfg(all(test, feature = "test-utils"))]
        impl $req_type {
            /// Returns a partial `Self` used only by `implement_tests!` as the
            /// `..spread` operand after `access_token` and `payload`. This
            /// struct has no extra fields, so the return is an empty literal.
            pub fn default_test_extras() -> Self {
                Self {
                    access_token: "",
                    payload: None,
                }
            }
        }
    };
}

/// Variant of `implement_utils!` for upload-side content endpoints.
/// The Request struct must have a `pub data: Option<Vec<u8>>` field
/// carrying the binary body; the generated impl forwards it through
/// `Utils::content_body()` so the service macro can attach it as the
/// HTTP body when `Headers::ContentTypeAppOctetStream` is set.
#[macro_export]
macro_rules! implement_content_upload_utils {
    ($req_type:ty, $payload_type:ty) => {
        impl Utils<'_> for $req_type {
            type T = $payload_type;
            fn payload(&self) -> Option<&Self::T> {
                self.payload.as_ref()
            }
            fn content_body(&self) -> Option<&[u8]> {
                self.data.as_deref()
            }
        }

        #[cfg(all(test, feature = "test-utils"))]
        impl $req_type {
            pub fn default_test_extras() -> Self {
                Self {
                    access_token: "",
                    payload: None,
                    data: None,
                }
            }
        }
    };
}

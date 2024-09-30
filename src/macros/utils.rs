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
    };
}

use base64::{engine::general_purpose, Engine as _};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// base64 encode
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        // 把pb转换过来的结构体重新转换成二进制数据，类似js的uint8Array
        let data = image_spec.encode_to_vec();
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = general_purpose::URL_SAFE_NO_PAD.decode(value)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

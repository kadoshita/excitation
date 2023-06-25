use super::common::AddressGenerator;
use rand::Rng;

pub struct TestNet1Generator {}

impl AddressGenerator for TestNet1Generator {
    fn generate(&self) -> String {
        let fourth_octet = rand::thread_rng().gen_range(0..255);
        format!("192.0.2.{}", fourth_octet)
    }
}

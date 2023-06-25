use super::common::AddressGenerator;
use rand::Rng;

pub struct TestNet3Generator {}

impl AddressGenerator for TestNet3Generator {
    fn generate(&self) -> String {
        let fourth_octet = rand::thread_rng().gen_range(0..255);
        format!("203.0.113.{}", fourth_octet)
    }
}

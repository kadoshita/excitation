use super::common::AddressGenerator;
use rand::Rng;

pub struct TestNet2Generator {}

impl AddressGenerator for TestNet2Generator {
    fn generate(&self) -> String {
        let fourth_octet = rand::thread_rng().gen_range(0..255);
        format!("198.51.100.{}", fourth_octet)
    }
}

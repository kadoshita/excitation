use super::common::AddressGenerator;
use rand::Rng;

pub struct TestNet1Generator {}

impl AddressGenerator for TestNet1Generator {
    fn generate(&self) -> String {
        let fourth_octet = rand::thread_rng().gen_range(0..255);
        format!("192.0.2.{}", fourth_octet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let generator = TestNet1Generator {};
        let address = generator.generate();
        assert!(address.starts_with("192.0.2."));
        let fourth_octet = address.split('.').nth(3).unwrap();
        let _: u8 = fourth_octet.parse().unwrap();
    }
}

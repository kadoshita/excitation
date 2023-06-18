use super::common::AddressGenerator;

pub struct TestNet3Generator {}

impl AddressGenerator for TestNet3Generator {
    fn generate(&self) -> String {
        String::from("TestNet3Generator::generate")
    }
}

use super::common::AddressGenerator;

pub struct TestNet2Generator {}

impl AddressGenerator for TestNet2Generator {
    fn generate(&self) -> String {
        String::from("TestNet2Generator::generate")
    }
}

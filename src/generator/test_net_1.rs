use super::common::AddressGenerator;

pub struct TestNet1Generator {}

impl AddressGenerator for TestNet1Generator {
    fn generate(&self) -> String {
        String::from("TestNet1Generator::generate")
    }
}

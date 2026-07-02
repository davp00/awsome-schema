use codegen::CodeGenerator;

#[test]
fn codegen_trait_is_available() {
    struct Stub;
    impl CodeGenerator for Stub {
        fn language(&self) -> &'static str {
            "stub"
        }

        fn generate(&self, _schema: &core::DatabaseSchema) -> Result<String, core::DomainError> {
            Ok(String::new())
        }
    }

    assert_eq!(Stub.language(), "stub");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        rust_core::run_app()
    }
    #[cfg(target_arch = "wasm32")]
    {
        Ok(())
    }
}

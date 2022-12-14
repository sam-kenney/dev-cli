use async_trait::async_trait;

/// Trait to ensure each language has an execute function.
#[async_trait]
pub trait Language {
    /// Execute the language specific project build.
    ///
    /// # Arguments
    /// * `dir` - The directory to create the project in.
    async fn execute(&self, dir: &String) -> ();
}

#[async_trait]
impl Foo for WrapBj {
    async fn foo(&self) {
        // Notice how there is a gap between `self.0.` and `id()`
        // removing this gap prevents the crash
        self.0. id().await;
    }
}

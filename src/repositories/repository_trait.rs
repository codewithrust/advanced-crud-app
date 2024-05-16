pub trait RepositoryTrait<'a, T> {
    fn create(&self, item: T) -> Result<T, String>;
    fn get_by_id(&self, id: String) -> Result<T, String>;
    fn get_all(&self) -> Result<Vec<T>, String>;
    fn update(&self, item: T) -> Result<(), String>;
    fn delete(&self, id: String) -> Result<(), String>;
}

// serviceTrait is a trait that defines the basic CRUD operations for a service
// it uses lifetimes and generics to define the type of the item that will be used
// to use lifetimes, we use syntax like this: 'a.
// to use generics, we use syntax like this: <T>
// lifetimes are used to ensure that the reference to the repository is valid for the lifetime of the service
pub trait ServiceTrait<'a, T> {
    fn create(&self, item: T) -> Result<T, String>;
    fn get_by_id(&self, id: String) -> Result<T, String>;
    fn get_all(&self) -> Result<Vec<T>, String>;
    fn update(&self, item: T) -> Result<T, String>;
    fn delete(&self, id: String) -> Result<(), String>; // empty tuple is used to indicate that the function does not return anything
}

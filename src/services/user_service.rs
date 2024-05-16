use super::service_trait::ServiceTrait;
use crate::models::user_model::User as UserModel;

// we should import below or will get error: no method named `{method name}` found for reference `&'a Repo` in the current scope
// items from traits can only be used if the trait is in scope
use crate::repositories::repository_trait::RepositoryTrait;
use crate::repositories::user_repository::Repository as UserRepository;

// Service is a struct that implements the ServiceTrait trait
pub struct Service<'a> {
    pub repository: &'a UserRepository<'a>,
}

// implementation of the Service struct without implementing the ServiceTrait trait
// because the new function return a Service struct
impl<'a> Service<'a> {
    pub fn new(repo: &'a UserRepository) -> Self {
        Service { repository: repo }
    }
}

// implementation of the Service struct with the ServiceTrait trait
impl<'a> ServiceTrait<'a, UserModel> for Service<'a> {
    fn create(&self, item: UserModel) -> Result<UserModel, String> {
        self.repository.create(item)
    }

    fn get_by_id(&self, id: String) -> Result<UserModel, String> {
        self.repository.get_by_id(id)
    }

    fn get_all(&self) -> Result<Vec<UserModel>, String> {
        self.repository.get_all()
    }

    fn update(&self, item: UserModel) -> Result<UserModel, String> {
        let id = item.clone().id;
        let result = self.repository.update(item);
        if result.is_err() {
            Err(result.err().unwrap())
        } else {
            self.repository.get_by_id(id)
        }
    }

    fn delete(&self, id: String) -> Result<(), String> {
        self.repository.delete(id)
    }
}

use async_trait::async_trait;

use crate::{entities::User, Error};

#[async_trait]
pub trait BaseDataSource<T>: std::fmt::Debug
where
    T: Sized,
{
    async fn insert(&self, user: &T) -> Result<T, Error>;
    async fn update(&self, user: &T) -> Result<T, Error>;
    async fn delete(&self, id: &str) -> Result<(), Error>;
    async fn get(&self, id: &str) -> Result<T, Error>;
}

#[async_trait]
pub trait UserDataSource: BaseDataSource<User> {
    async fn get_by_login(&self, login: &str) -> Result<User, Error>;
    async fn get_by_email(&self, email: &str) -> Result<User, Error>;
}

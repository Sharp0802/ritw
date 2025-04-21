use std::error::Error;
use crate::models::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait Model<TSelf, TDto, T>
where
    TSelf: Model<TSelf, TDto, T>,
    TSelf: From<TDto>,
    T: ?Sized,
{
    fn id(&self) -> &T;
    async fn up() -> Result<(), Box<dyn Error>>;
    async fn create(new: &Self) -> Result<TSelf, AppError>;
    async fn read(id: &T) -> Result<TSelf, AppError>;
    async fn update(old: &Self, new: &Self) -> Result<(), AppError>;
    async fn delete(id: &T) -> Result<(), AppError>;
}

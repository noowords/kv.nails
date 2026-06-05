mod domain;
mod infrastructure;
mod application;

use sqlx::mysql::{ MySqlPoolOptions };

use crate::domain::user::value_objects::{ UserPhone };
use crate::infrastructure::mysql::{ MySqlUnitOfWork };
use crate::application::use_cases::register_user::{ RegisterUserCommand, RegisterUserUseCase };

#[tokio::main]
async fn main() -> Result<(), String> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:root@localhost:3306/kvnails")
        .await
        .map_err(|e| format!("MySQL connection error: {}", e))?;

    let mut uow = MySqlUnitOfWork::begin(&pool).await?;

    let cmd = RegisterUserCommand::new(
        UserPhone::new(Some("+X (XXX) XXX-XX-XX".to_string()))
    );

    match RegisterUserUseCase::execute(&mut uow, cmd).await {
        Ok(()) => {
            uow.commit().await?;

            Ok(())
        }
        Err(e) => {
            uow.rollback().await?;

            Err(e)
        }
    }
}

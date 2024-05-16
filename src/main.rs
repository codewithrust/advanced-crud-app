use std::env;

use crate::repositories::user_repository::Repository as UserRepository;
use crate::services::service_trait::ServiceTrait;
use crate::services::user_service::Service as UserService;
use async_std::task;
use dotenv::dotenv;
use prettytable::{row, Table};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

fn main() {
    dotenv().ok();

    task::block_on(async {
        let db_result = connect_to_db().await;

        if db_result.is_err() {
            eprintln!("Failed to connect to DB: {:?}", db_result.err());
            std::process::exit(1);
        }

        let db_conn = db_result.unwrap();
        let user_repository = UserRepository::new(&db_conn);
        let user_service = UserService::new(&user_repository);

        example_crud(user_service);
    });
}

async fn connect_to_db() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Connected to database");

    Ok(pool)
}

fn example_crud(service: UserService) {
    // create user
    let user_id = generate_random_uuid();
    let create_user_result = service.create(models::user_model::User {
        id: user_id.clone(),
        username: generate_username(&user_id.as_str()),
        email: generate_email(&user_id.as_str()),
    });

    if create_user_result.is_err() {
        eprintln!("Failed to create user: {:?}", create_user_result.err());
        return;
    }

    let created_user = create_user_result.unwrap();
    println!("\n# Created User:\n");

    let mut users: Vec<models::user_model::User> = vec![];
    users.push(created_user.clone());
    show_table(users);

    // get user by id
    let get_user_by_id_result = service.get_by_id(created_user.id);
    if get_user_by_id_result.is_err() {
        eprintln!(
            "Failed to get user by id: {:?}",
            get_user_by_id_result.err()
        );
        return;
    }

    let get_user_by_id = get_user_by_id_result.unwrap();
    println!("\n# Get User By Id:\n");

    let mut users: Vec<models::user_model::User> = vec![];
    users.push(get_user_by_id.clone());
    show_table(users);

    // update user
    let update_user_result = service.update(models::user_model::User {
        id: get_user_by_id.id.clone(),
        username: format!("updated-{}", get_user_by_id.username),
        email: format!("updated-{}", get_user_by_id.email),
    });
    if update_user_result.is_err() {
        eprintln!("Failed to update user: {:#?}", update_user_result.err());
        return;
    }

    let updated_user = update_user_result.unwrap();
    println!("\n# Updated User:\n");

    let mut users: Vec<models::user_model::User> = vec![];
    users.push(updated_user.clone());
    show_table(users);

    // delete user
    let delete_user_result = service.delete(updated_user.id.clone());
    if delete_user_result.is_err() {
        eprintln!("Failed to delete user: {:?}", delete_user_result.err());
        return;
    }

    println!("\n# Deleted User ID: {:#?}", updated_user.id);

    // get all users
    let get_all_users_result = service.get_all();
    if get_all_users_result.is_err() {
        eprintln!("Failed to get all users: {:?}", get_all_users_result.err());
        return;
    }

    let all_users = get_all_users_result.unwrap();
    println!("\n# All Users:\n");

    show_table(all_users);
}

fn generate_random_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

fn generate_username(id: &str) -> String {
    format!("user-{}", id)
}

fn generate_email(id: &str) -> String {
    format!("{}@example.com", id)
}

fn show_table(users: Vec<models::user_model::User>) {
    let mut table = Table::new();
    table.add_row(row!["ID", "Username", "Email"]);
    for user in &users {
        table.add_row(row![user.id, user.username, user.email]);
    }

    // Print the table
    table.printstd();
}

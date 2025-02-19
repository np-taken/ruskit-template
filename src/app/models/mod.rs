use crate::framework::database::model::Model;
use crate::framework::database::migration::Migration;
use crate::app::entities::{User, Post};

// This file will be populated with models as they are created
mod user;
mod post;

pub use user::*;
pub use post::*;

// Register all models
pub fn register_models() {
    println!("Registering models...");
    let mut migrations = Vec::new();
    migrations.extend(User::migrations());
    migrations.extend(Post::migrations());
    println!("Models registered");
}

// Get all migrations from all models
pub fn get_all_model_migrations() -> Vec<Migration> {
    let mut migrations = Vec::new();
    migrations.extend(User::migrations());
    migrations
}
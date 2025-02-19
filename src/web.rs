use axum::{
    Router,
    routing::{get, post},
};
use crate::framework::middleware::{
    WithMiddleware,
    presets::{Cors, TrimStrings},
};
use crate::bootstrap::app::bootstrap;
use crate::app::controllers::{
    user_controller::UserController,
    docs_controller::DocsController,
    inertia_controller::InertiaController,
    pages::landing,
};
use axum_inertia::vite;

// Define routes with middleware
pub async fn routes() -> Router {
    // Initialize the application
    if let Err(e) = bootstrap().await {
        eprintln!("Failed to bootstrap application: {}", e);
        std::process::exit(1);
    }

    let inertia_config = vite::Development::default()
        .port(3000)
        .main("resources/js/app.tsx")
        .lang("en")
        .title("Ruskit")
        .react() 
        .into_config();
    
    let inertia_router = Router::new()
        .route("/", get(landing))
        .route("/about", get(InertiaController::about))
        .route("/docs", get(DocsController::index))
        .route("/docs/:page", get(DocsController::show))
        .with_state(inertia_config);

    let api_router = Router::new()
        .route(
            "/users", 
            get(UserController::index)
                .middleware(TrimStrings::new())
                .middleware(Cors::new("http://users.example.com"))
        )
        .route("/users/{id}", get(UserController::show))
        .route(
            "/users", 
            post(UserController::store).middleware(TrimStrings::new())
        );

    Router::new()
        .nest("/", inertia_router)
        .nest("/api", api_router)
} 
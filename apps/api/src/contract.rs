//! THE contract — aggregates all utoipa-annotated handlers into one OpenAPI doc.
//! The typegen test walks `ApiDoc::openapi()` to emit `apps/web/src/lib/contract.ts`.

use crate::features::auth::presentation::http::routes as auth;
use crate::features::todo::presentation::http::routes as todo;
use crate::shared::presentation;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        presentation::health::health,
        auth::sign_up,
        auth::sign_in,
        auth::sign_out,
        auth::session,
        todo::list_todos,
        todo::create_todo,
        todo::get_todo,
        todo::update_todo,
        todo::delete_todo,
    ),
    components(schemas(
        crate::features::auth::presentation::http::dtos::SignUpRequest,
        crate::features::auth::presentation::http::dtos::SignInRequest,
        crate::features::auth::presentation::http::dtos::UserResponse,
        crate::features::auth::presentation::http::dtos::SessionResponse,
        crate::features::auth::presentation::http::dtos::SessionDetails,
        crate::features::todo::presentation::http::dtos::CreateTodoRequest,
        crate::features::todo::presentation::http::dtos::CreateTodoResponse,
        crate::features::todo::presentation::http::dtos::UpdateTodoRequest,
        crate::features::todo::presentation::http::dtos::TodoResponse,
        crate::features::todo::domain::Status,
        presentation::health::Health,
    ))
)]
pub struct ApiDoc;

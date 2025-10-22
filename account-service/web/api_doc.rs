use utoipa::OpenApi;
use crate::web::handlers::{
    DeleteAccountRequest, DeleteAccountResponse
};
use crate::web::handlers::__path_delete_account;
use crate::web::handlers::__path_login_user;
use crate::web::handlers::__path_register_user;
use crate::web::handlers::__path_verify_mfa_user;


#[derive(OpenApi)]
#[openapi(
    paths(
        register_user,
        login_user,
        verify_mfa_user,
        delete_account
    ),
    components(schemas(
        DeleteAccountRequest,
        DeleteAccountResponse
    )),
    tags(
        (name = "Accounts", description = "Endpoints pour la gestion des comptes"),
    ),
    security(
        ("basicAuth" = [])
    )
)]
pub struct ApiDoc;

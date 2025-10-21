use utoipa::OpenApi;
use crate::web::handlers::{
    DepositRequest, DepositResponse,
    BalanceRequest, BalanceResponse,
    DeleteAccountRequest, DeleteAccountResponse
};
use crate::web::handlers::__path_delete_account;
use crate::web::handlers::__path_get_balance;
use crate::web::handlers::__path_login_user;
use crate::web::handlers::__path_register_user;
use crate::web::handlers::__path_verify_mfa_user;
use crate::web::handlers::__path_deposit_funds;


#[derive(OpenApi)]
#[openapi(
    paths(
        register_user,
        login_user,
        verify_mfa_user,
        deposit_funds,
        get_balance,
        delete_account
    ),
    components(schemas(
        DepositRequest,
        DepositResponse,
        BalanceRequest,
        BalanceResponse,
        DeleteAccountRequest,
        DeleteAccountResponse
    )),
    tags(
        (name = "Accounts", description = "Endpoints pour la gestion des comptes"),
        (name = "Wallet", description = "Endpoints pour les portefeuilles et transactions")
    ),
    security(
        ("basicAuth" = [])
    )
)]
pub struct ApiDoc;

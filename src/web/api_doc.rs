use utoipa::OpenApi;
use crate::web::handlers::{
    DepositRequest, DepositResponse,
    BalanceRequest, BalanceResponse
};
use crate::web::handlers::__path_get_balance;
use crate::web::handlers::__path_deposit_funds;


#[derive(OpenApi)]
#[openapi(
    paths(

        deposit_funds,
        get_balance,
    ),
    components(schemas(
        DepositRequest,
        DepositResponse,
        BalanceRequest,
        BalanceResponse,
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

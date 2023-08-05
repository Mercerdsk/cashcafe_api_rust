use actix_web::web;
use crate::api::endpoints::*;



pub fn init_routes_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(player_creation_handler);
    cfg.service(player_login_handler);
    cfg.service(get_balance_handler);
    cfg.service(available_games_handler);
    cfg.service(payment_init_handler);
    cfg.service(add_money_handler);
    cfg.service(withdraw_money_handler);
}
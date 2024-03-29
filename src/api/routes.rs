use actix_web::web;
use crate::api::endpoints::*;


pub fn init_routes_v1(cfg: &mut web::ServiceConfig) {
    let kyc_verify_cfg = web::JsonConfig::default().limit(25_097_152);
    let profile_upd_cfg = web::JsonConfig::default().limit(25_097_152);
    cfg.service(get_token_handler);
    cfg.service(protected_handler);
    cfg.service(player_creation_handler);
    cfg.service(player_login_handler);
    cfg.service(get_balance_handler);
    cfg.service(available_games_handler);
    cfg.service(payment_init_handler);
    cfg.service(add_money_handler);
    cfg.service(withdraw_money_handler);
    cfg.service(otp_validation_handler);
    cfg.service(otp_generation_handler);
    cfg.service(get_games_handler);
    cfg.service(get_fav_games_handler);
    cfg.service(get_server_time_handler);
    cfg.service(get_slot_games_handler);
    cfg.service(get_player_profile_handler);
    cfg.service(upd_player_profile_handler).app_data(profile_upd_cfg);
    cfg.service(buy_handler);
    cfg.service(kyc_verification_handler).app_data(kyc_verify_cfg);
    cfg.service(get_current_result_handler);
    cfg.service(get_latest_result_handler);
    cfg.service(transaction_history_handler);
    cfg.service(player_reports_handler);
    cfg.service(result_handler);
    cfg.service(password_change_handler);
    cfg.service(captcha_verify_handler);
    cfg.service(ticket_info_handler);
    cfg.service(get_odds_config_scheme_handler);
    cfg.service(player_login_image_handler);
    cfg.service(get_game_wise_bet_info_handler);
    cfg.service(get_available_race_handler);
    cfg.service(get_game_race_details_handler);
    cfg.service(get_country_handler);
    cfg.service(deposit_init_handler);
    cfg.service(addmoney_conformation_handler);
    cfg.service(vdr_vhr_handler);
    cfg.service(image_upload_handler);
    cfg.service(vdr_result_handler);
    cfg.service(withdraw_init_handler);
    cfg.service(withdraw_confirmation_handler);
    cfg.service(logout_handler);

}
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::*;
use std::env;

pub fn get_pattern_status(
    pattern: Option<&Pattern>,
    second_last_pattern_type: &PatternType,
) -> Status {
    let max_pattern_days = env::var("MAX_PATTERN_DAYS")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let max_pattern_activated_days = env::var("MAX_PATTERN_ACTIVATED_DAYS")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let max_pattern_date = DbDateTime::from_chrono(Local::now() - Duration::days(max_pattern_days));

    let max_activated_date =
        DbDateTime::from_chrono(Local::now() - Duration::days(max_pattern_activated_days));

    let super_date = DbDateTime::from_chrono(Local::now() - Duration::days(35));

    let fake_date = DbDateTime::from_chrono(Local::now() - Duration::days(1000));

    let local_pattern_status = match pattern {
        Some(_pat) => {
            let pattern_active = match pattern {
                Some(pat) => pat.active.active,
                None => false,
            };

            let pattern_date = match pattern {
                Some(val) => val.date,
                None => fake_date,
            };

            let pattern_active_date = match pattern {
                Some(val) => val.active.date,
                None => fake_date,
            };

            let pattern_type = match pattern {
                Some(val) => val.pattern_type.clone(),
                None => PatternType::None,
            };

            let pattern_status = match pattern {
                _x if pattern_active && pattern_active_date > max_activated_date => Status::Bullish,
                _x if second_last_pattern_type == &PatternType::ChannelDown
                    && &pattern_type != second_last_pattern_type =>
                {
                    Status::Bullish
                }
                _x if second_last_pattern_type == &PatternType::ChannelDown
                    && &pattern_type == second_last_pattern_type =>
                {
                    Status::Bearish
                }
                _x if pattern_date > max_pattern_date => Status::Neutral,
                _x if pattern_date > super_date => Status::Neutral,
                _x if pattern_type == PatternType::None => Status::Default,
                _ => Status::Default,
            };

            pattern_status
        }
        None => Status::Default,
    };
    local_pattern_status
}

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::pattern::{Pattern, PatternType};
use rs_algo_shared::models::status::Status;
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

    let max_pattern_date = to_dbtime(Local::now() - Duration::days(max_pattern_days));

    let max_activated_date = to_dbtime(Local::now() - Duration::days(max_pattern_activated_days));

    let super_date = to_dbtime(Local::now() - Duration::days(35));

    let fake_date = to_dbtime(Local::now() - Duration::days(1000));

    match pattern {
        Some(_pat) => {
            let pattern_type = match pattern {
                Some(pat) => pat.pattern_type.clone(),
                None => PatternType::None,
            };
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

            match pattern {
                _x if pattern_type == PatternType::ChannelUp
                    || pattern_type == PatternType::HigherHighsHigherLows
                    || pattern_type == PatternType::TriangleUp =>
                {
                    Status::Bullish
                }
                _x if pattern_type == PatternType::ChannelDown
                    || pattern_type == PatternType::LowerHighsLowerLows
                    || pattern_type == PatternType::TriangleDown =>
                {
                    Status::Bearish
                }
                _x if pattern_type == PatternType::Broadening
                    || pattern_type == PatternType::Rectangle
                    || pattern_type == PatternType::TriangleSym =>
                {
                    Status::Neutral
                }

                _x if pattern_active && pattern_active_date > max_activated_date => Status::Bullish,
                _x if (second_last_pattern_type == &PatternType::ChannelDown
                    || second_last_pattern_type == &PatternType::LowerHighsLowerLows)
                    && &pattern_type != second_last_pattern_type =>
                {
                    Status::Bullish
                }
                _x if (second_last_pattern_type == &PatternType::ChannelDown
                    || second_last_pattern_type == &PatternType::LowerHighsLowerLows)
                    && &pattern_type == second_last_pattern_type =>
                {
                    Status::Bearish
                }

                _x if pattern_date > max_pattern_date => Status::Neutral,
                _x if pattern_date > super_date => Status::Neutral,
                _x if pattern_type == PatternType::None => Status::Default,
                _ => Status::Default,
            }
        }
        None => Status::Default,
    }
}

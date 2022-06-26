use rs_algo_shared::models::status::Status;

pub fn get_status_class<'a>(status: &Status) -> &'a str {
    let class = match status {
        Status::Default => "",
        Status::Neutral => "has-background-warning-light",
        //Status::Neutral => "",
        Status::Bullish => "has-background-primary-light",
        Status::Bearish => "has-background-danger-light",
        Status::ChangeUp => "has-background-warning-light",
        Status::ChangeDown => "has-background-warning-light",
        Status::CancelUp => "has-background-cancel-up",
        Status::CancelDown => "has-background-cancel-down",
    };
    class
}

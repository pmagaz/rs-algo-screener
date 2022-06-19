use rs_algo_shared::models::status::Status;

pub fn get_status_class2<'a>(status: &Status) -> &'a str {
    let class = match status {
        Status::Default => "",
        Status::Neutral => "",
        //Status::Neutral => "",
        Status::Bullish => "has-background-primary-light",
        Status::Bearish => "has-background-danger-light",
        Status::ChangeUp => "has-background-primary-light2",
        Status::ChangeDown => "has-background-danger-light2",
    };
    class
}

pub fn get_status_class<'a>(status: &Status) -> &'a str {
    let class = match status {
        Status::Default => "",
        Status::Neutral => "",
        //Status::Neutral => "",
        Status::Bullish => "has-background-primary-light",
        Status::Bearish => "has-background-danger-light",
        Status::ChangeUp => "has-background-warning-light",
        Status::ChangeDown => "has-background-warning-light",
    };
    class
}

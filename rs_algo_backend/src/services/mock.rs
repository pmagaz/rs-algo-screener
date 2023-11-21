pub async fn find_mock(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let (symbol, time_frame) = path.into_inner();

    Ok(HttpResponse::Ok().json(instrument))
}

use std::path::PathBuf;

use axum::{
    routing::post, 
    Router, 
    response::Json,
    http::StatusCode, 
    body::Bytes,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart, FieldData};
use axum::routing::get_service;
use tower_http::services::ServeDir;
use spinning_ascii::*;
use tracing::info;

#[derive(TryFromMultipart)]
struct RequestData {
    animation_type: String,
    image: FieldData<Bytes>,
}

async fn upload(
    TypedMultipart(RequestData { animation_type, image }): TypedMultipart<RequestData>,
) -> Result<Json<Frames>, (StatusCode, String)> {

    info!(
        "file name = '{}', content type = '{}', size = '{}'",
        image.metadata.file_name.unwrap_or(String::new()),
        image.metadata.content_type.clone().unwrap_or(String::from("text/plain")),
        image.contents.len()
    );

    if image.metadata.content_type.clone().unwrap().to_string().contains("image") == false {
        return Err((StatusCode::BAD_REQUEST, "Not an image".to_string()));
    }

    let image_buffer = image::load_from_memory_with_format(
        &image.contents,
        image::ImageFormat::from_mime_type(image.metadata.content_type.clone().unwrap()).unwrap()
    ).map_err(|err| {
        println!("Error: {}", err);
        (StatusCode::BAD_REQUEST, "Error loading image".to_string())
    })?.to_luma8();

    let pixel_matrix = pixels_to_ascii(image_buffer, 20).map_err(|err| {
        println!("Error: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error converting image".to_string())
    })?;

    let ouput = match animation_type.as_str() {
        "rotate-cw" => create_rotate_cw_frames(pixel_matrix),
        "rotate-ccw" => create_rotate_ccw_frames(pixel_matrix),
        "shift-left" => create_shift_left_frames(pixel_matrix),
        "shift-right" => create_shift_right_frames(pixel_matrix),
        _ => create_rotate_cw_frames(pixel_matrix),
    };

    Ok(Json(ouput.map_err(|err| {
        println!("Error: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error processing image".to_string())
    })?))
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "assets")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .merge(routes_api())
        .fallback_service(routes_static(PathBuf::from(format!("{}/{}", static_folder.display(), "frames.json"))));

    Ok(app.into())
}

fn routes_api() -> Router {
    Router::new()
        .route("/api", post(upload))
}

fn routes_static(static_folder: PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(static_folder)))
}

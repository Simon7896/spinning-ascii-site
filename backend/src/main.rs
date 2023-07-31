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
use serde_json::{Value, json};
use tower_http::services::ServeDir;
use spinning_ascii::run_json;

#[derive(TryFromMultipart)]
struct RequestData {
    image: FieldData<Bytes>,
}

async fn upload(
    TypedMultipart(RequestData { image }): TypedMultipart<RequestData>,
) -> Result<Json<Value>, (StatusCode, String)> {


    println!(
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


    Ok(Json(json!(run_json(spinning_ascii::pixels_to_ascii(image_buffer, 20).unwrap()).unwrap().as_str())))
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "assets")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .merge(routes_upload())
        .fallback_service(routes_static(PathBuf::from(format!("{}/{}", static_folder.display(), "frames.json"))));

    Ok(app.into())
}

fn routes_upload() -> Router {
    Router::new().route("/upload", post(upload))
}

fn routes_static(static_folder: PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(static_folder)))
}

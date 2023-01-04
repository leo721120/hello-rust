use crate::app::App;
//use crate::app::RFC7807;
use crate::foo::service;
use crate::foo::FooAttributes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
//use axum::response::Response;
use axum::Json;

pub async fn create_foo(State(app): State<App>, Json(params): Json<FooAttributes>) -> impl IntoResponse {
    let res = service::create_foo(app, params).await.unwrap();
    return (StatusCode::CREATED, Json(res));

    /*match service::create_foo(app, params).await {
        Err(err) => {
            tracing::warn!("{:?}", err);
            //return err.into_response();
            //return String::from("Hello, world!").into_response();
            /*return Response::builder()
            .status(StatusCode::CONFLICT)
            .body(Json(err))
            .unwrap();*/
            //return (StatusCode::CONFLICT, Json(err));
        }
        Ok(res) => {
            tracing::info!("{:?}", res);
            let hello = FooAttributes {
                username: Some(String::from("world")),
            };
            return Json(hello).into_response();
            /*return Response::builder()
            .status(StatusCode::CREATED)
            .body(Json(res))
            .unwrap();*/
            //return (StatusCode::CREATED, Json(res));
        }
    };*/
}

//! Module containing all the actix request handlers for the `/api/v1/users/` endpoints

use super::PCResponder;
use crate::{
    error::PointercrateError,
    middleware::{auth::Token, cond::HttpResponseBuilderExt},
    model::user::{PatchUser, User, UserPagination},
    state::PointercrateState,
};
use actix_web::{AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Path};
use log::info;
use tokio::prelude::future::{Future, IntoFuture};

/// `GET /api/v1/users/` handler
pub fn paginate(req: &HttpRequest<PointercrateState>) -> PCResponder {
    info!("GET /api/v1/users/");

    let query_string = req.query_string();
    let pagination = serde_urlencoded::from_str(query_string)
        .map_err(|err| PointercrateError::bad_request(&err.to_string()));

    let req = req.clone();

    pagination
        .into_future()
        .and_then(move |pagination: UserPagination| {
            req.state()
                .paginate::<Token, User, _>(&req, pagination, "/api/v1/users/".to_string())
        })
        .map(|(users, links)| HttpResponse::Ok().header("Links", links).json(users))
        .responder()
}

get_handler!("/api/v1/users/[id]", i32, "User ID", User);
patch_handler!("/api/v1/users/[id]/", i32, "User ID", PatchUser, User);
delete_handler!("/api/v1/users/[user id]/", i32, "User ID", User);

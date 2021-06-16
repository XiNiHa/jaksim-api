mod schema;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_rocket::{Request, Response};
use rocket::{response::content::Html, State};
use schema::{AppSchema, QueryRoot};

extern crate rocket;

#[rocket::get("/graphql")]
fn graphql_playground_handler() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn post_graphql_handler(schema: &State<AppSchema>, request: Request) -> Response {
    request.execute(schema).await
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish())
        .mount(
            "/",
            rocket::routes![graphql_playground_handler, post_graphql_handler],
        )
}

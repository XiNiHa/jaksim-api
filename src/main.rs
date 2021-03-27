mod schema;

use juniper::{EmptyMutation, EmptySubscription};
use juniper_rocket_async::{GraphQLRequest, GraphQLResponse, playground_source};
use rocket::{State, response::content::Html};
use schema::{Context, Query, Schema};

extern crate rocket;

#[rocket::get("/graphql")]
fn graphql_playground_handler() -> Html<String> {
    playground_source("/graphql", None)
}

#[rocket::post("/graphql", data="<request>")]
async fn post_graphql_handler(
    request: GraphQLRequest,
    schema: State<'_, Schema>
) -> GraphQLResponse {
    request.execute(&schema, &Context{}).await
}

#[rocket::launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()))
        .mount("/", rocket::routes![
            graphql_playground_handler,
            post_graphql_handler
        ])
}

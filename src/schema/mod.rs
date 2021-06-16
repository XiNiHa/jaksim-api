use async_graphql::*;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn echo(&self, message: String) -> Echo {
        Echo { message }
    }
}

#[derive(SimpleObject)]
struct Echo {
    message: String,
}

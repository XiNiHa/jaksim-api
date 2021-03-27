use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode,
};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub struct Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn echo(_ctx: &Context, message: String) -> FieldResult<Echo> {
      Ok(Echo { message })
    }
}

#[derive(GraphQLObject)]
struct Echo {
    message: String,
}

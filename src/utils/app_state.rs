use std::sync::Arc;

use actix_web::web;
use aws_sdk_dynamodb::Client;

use crate::RedisClient;

pub struct AppState {
    //pub redis_client: web::Data<RedisClient>,
    pub dynamo_client: Arc<Client>,
}

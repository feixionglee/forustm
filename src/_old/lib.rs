#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![recursion_limit = "128"]
#![deny(warnings)]

extern crate ammonia;
extern crate chrono;
extern crate comrak;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate dotenv;
extern crate hyper;
extern crate hyper_native_tls;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate redis;
extern crate sapper;
#[macro_use]
extern crate sapper_std;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tiny_keccak;
extern crate uuid;

pub mod api;
pub mod schema;
pub mod util;
pub mod models;
pub mod web;
pub mod web_wechat;
pub mod proxy;

pub(crate) use models::{Article, ArticleBrief, DeleteArticle, EditArticle, NewArticle,
                        SimpleArticle};
pub(crate) use models::NewArticleStats;
pub(crate) use models::UserNotify;
pub(crate) use models::{ChangStatus, ChangePassword, ChangePermission, EditUser, LoginUser, RUser,
                        RegisteredUser};
pub(crate) use models::{DeleteComment, NewComment};
pub(crate) use models::{InsertSection, PubNotice, Section};
pub(crate) use schema::{article, article_stats, comment, ruser, section};
pub(crate) use util::{inner_get_github_nickname_and_address, inner_get_github_primary_email,
                      inner_get_github_token, markdown_render, random_string,
                      send_reset_password_email, sha3_256_encode};
pub(crate) use util::{get_github_nickname_and_address, get_github_primary_email, get_github_token};
pub(crate) use util::{get_real_ip_from_req, get_ruser_from_session, get_user_agent_from_req};

pub use api::{AdminSection, AdminUser, User, Visitor};
pub use proxy::ProxyModule;
pub use util::{create_pg_pool, create_redis_pool, get_identity_and_web_context, Permissions,
               Postgresql, Redis, RedisPool, WebContext};

pub fn page_size() -> i64 {
    20
}

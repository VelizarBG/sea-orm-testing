mod entity;

use crate::entity::*;
use sea_orm::entity::prelude::*;
use sea_orm::{ConnectOptions, Database, EntityLoaderTrait, IntoActiveModel, JoinType, QuerySelect, Set};
use std::sync::OnceLock;
use crate::entity::role_operator_server::RoleToOperatorServer;
use crate::entity::role_whitelisted_server::RoleToWhitelistedServer;

static IDK: &str = "A_id";
static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub fn db() -> &'static DatabaseConnection {
    DB.get().expect("Database connection not initialized")
}

async fn check(db: &DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    db.clone().close().await.expect("TODO: panic message");
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let idk = IDK;
    println!("{idk}");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // let mut opt = ConnectOptions::new("sqlite://testing.db?mode=rwc");
    let mut opt = ConnectOptions::new("sqlite::memory:");
    opt
        // .max_connections(100)
        // .min_connections(5)
        // .connect_timeout(Duration::from_secs(8))
        // .acquire_timeout(Duration::from_secs(8))
        // .idle_timeout(Duration::from_secs(8))
        // .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false) // disable SQLx logging
        .sqlx_logging_level(log::LevelFilter::Info);
    {
        let db = Database::connect(opt).await?;
        /*let mut entities_module = module_path!().split("::").next().unwrap().to_owned();
        entities_module.push_str("::entity::*");*/
        db.get_schema_registry("sea_orm_testing::entity::*").sync(&db).await?;
        DB.set(db).unwrap();
    }
    // check(db()).await;

    let db = db();

    let test_server = server::ActiveModel {
        name: Set("test_server".to_owned()),
        ..Default::default()
    };
    let test_server = test_server.insert(db).await?;
    let test_server2 = server::ActiveModel {
        name: Set("test_server2".to_owned()),
        ..Default::default()
    };
    let test_server2 = test_server2.insert(db).await?;

    let test_server_ex = test_server.clone().into_ex();
    let test_server2_ex = test_server2.clone().into_ex();
    role::ActiveModel {
        id: Set("783762870860251148".to_owned()),
    }.insert(db).await?;
    let test_role = role::ActiveModel::builder()
        .set_id("783762870860251148")
        .add_whitelisted_server(test_server_ex.clone())
        .add_whitelisted_server(test_server2_ex.clone())
        .add_operator_server(test_server_ex.clone())
        .save(db)
        .await;
    let test_role = test_role?;
    /*role_whitelisted_server::ActiveModel {
        role_id: Set(test_role.id.clone()),
        server_id: Set(test_server_ex.clone().id),
    }.insert(db).await.unwrap();*/
    let rw = "rw";
    let test_role = role::Entity::load()
        .with(RoleToWhitelistedServer)
        // .left_join_linked(RoleToWhitelistedServer)
        // .join_as(JoinType::LeftJoin, role_whitelisted_server::Relation::Role.def().rev(), rw)
        // .join(JoinType::LeftJoin, role_whitelisted_server::Relation::Server.def().from_alias(rw))
        // .find_with_linked(RoleToWhitelistedServer)
        // .filter(role::Column::Id.eq("783762870860251148"))
        // .with(role_whitelisted_server::Relation::Server)
        // .into_model::<RoleIdk2>()
        .one(db).await?;

    dbg!(test_role);

    // entity::server::Entity::load().with(entity::operator_task::Entity).one(db()).await.unwrap();
    Ok(())
}

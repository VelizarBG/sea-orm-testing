use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "server")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    #[sea_orm(has_many, relation_enum = "WhitelistedRole", via = "role_whitelisted_server")]
    pub whitelisted_roles: HasMany<super::role::Entity>,
    #[sea_orm(has_many, relation_enum = "OperatorRole", via = "role_operator_server")]
    pub operator_roles: HasMany<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;
use sea_orm::{ColIdx, FromQueryResult, TryGetableArray};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "role")]
pub struct Model {
	#[sea_orm(primary_key, alias = "A_id")]
	pub id: String,
	#[sea_orm(has_many, relation_enum = "WhitelistedServer", via = "role_whitelisted_server")]
	pub whitelisted_servers: HasMany<super::server::Entity>,
	#[sea_orm(has_many, relation_enum = "OperatorServer", via = "role_operator_server")]
	pub operator_servers: HasMany<super::server::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

/*#[derive(Debug, DerivePartialModel)]
#[sea_orm(entity = "Entity")]
pub struct RoleIdk2 {
	pub id: String,
	pub whitelisted_servers: Vec<super::server::Entity>,
}*/

/*#[derive(Clone, Debug, FromQueryResult)]
pub struct RoleIdk {
	#[sea_orm(from_alias = "A_id")]
	pub id: String,
	#[sea_orm(from_alias = "B_id")]
	pub whitelisted_servers: Vec<super::server::Entity>,
}

impl TryGetableArray for super::server::Entity {
	fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Vec<Self>, TryGetError> {
		match res.try_get_many(index) {
			Ok(entities) => Ok(entities),
			Err(err) => Err(err),
		}
	}
}*/


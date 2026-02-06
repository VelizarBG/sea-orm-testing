use sea_orm::entity::prelude::*;

use super::super::{role, server};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "role_whitelisted_server")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub role_id: String,
	#[sea_orm(primary_key, auto_increment = false)]
	pub server_id: i64,
	#[sea_orm(belongs_to, from = "role_id", to = "id")]
	pub role: Option<role::Entity>,
	#[sea_orm(belongs_to, from = "server_id", to = "id")]
	pub server: Option<server::Entity>,
}

pub struct RoleToWhitelistedServer;

impl Linked for RoleToWhitelistedServer {
	type FromEntity = role::Entity;
	type ToEntity = server::Entity;

	fn link(&self) -> Vec<RelationDef> {
		vec![
			Relation::Role.def().rev(), // cake -> cake_filling
			Relation::Server.def(),    // cake_filling -> filling
		]
	}
}

impl ActiveModelBehavior for ActiveModel {}

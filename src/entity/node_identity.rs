//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "node_identity")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub address: String,
	#[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
	pub private_key: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

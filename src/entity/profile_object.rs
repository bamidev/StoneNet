//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

use crate::common::IdType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "profile_object")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub object_id: i64,
	pub name: Option<IdType>,
	pub avatar_file_hash: Option<IdType>,
	pub wallpaper_file_hash: Option<IdType>,
	pub description_file_hash: Option<IdType>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

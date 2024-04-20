//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

use crate::{common::IdType, core::ActorAddress};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post_object")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub object_id: i64,
	pub in_reply_to_actor_address: Option<ActorAddress>,
	pub in_reply_to_object_hash: Option<IdType>,
	pub file_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::object::Entity",
		from = "Column::ObjectId",
		to = "super::object::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Object,
	#[sea_orm(has_many = "super::post_tag::Entity")]
	PostTag,
}

impl Related<super::object::Entity> for self::Entity {
	fn to() -> RelationDef { Relation::Object.def() }
}

impl Related<super::post_tag::Entity> for self::Entity {
	fn to() -> RelationDef { Relation::PostTag.def() }
}

impl ActiveModelBehavior for ActiveModel {}

use chrono::{NaiveDateTime, Local};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::visitors;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "visitors"]
pub struct Visitors {
    pub visitor_id: i32,
    pub prison_id: String,
    pub gender: String,
    pub first_name: String,
    pub last_name: String,
    pub relations: String,
    pub phone_num: String,
    pub line_id: String,
    pub remark: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "visitors"]
pub struct VisitorsInsert {
    pub gender: String,
    pub prison_id: String,
    pub first_name: String,
    pub last_name: String,
    pub relations: String,
    pub phone_num: String,
    pub line_id: String,
    pub remark: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "visitors"]
pub struct VisitorsMessage {
    pub gender: String,
    pub prison_id: String,
    pub first_name: String,
    pub last_name: String,
    pub relations: String,
    pub phone_num: String,
    pub line_id: String,
    pub remark: String,
}

impl Visitors {

    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let visitor_list = visitors::table
            .load::<Visitors>(&conn)?;

        Ok(visitor_list)
    }

    pub fn find(id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visitor = visitors::table
            .filter(visitors::visitor_id.eq(id))
            .first(&conn)?;

        Ok(visitor)
    }

    pub fn find_family_and_friend(id: String) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let visitors = visitors::table
            .filter(visitors::prison_id.eq(id))
            .load::<Visitors>(&conn)?;
        Ok(visitors)
    }

    pub fn find_by_first_name(first_name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visitor = visitors::table
            .filter(visitors::first_name.like(format!("%{}%", first_name)))
            .first(&conn)?;

        Ok(visitor)
    }


    pub fn find_by_last_name(last_name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visitor = visitors::table
            .filter(visitors::last_name.like(format!("%{}%", last_name)))
            .first(&conn)?;

        Ok(visitor)
    }


    pub fn create(visitors_message: VisitorsMessage) -> Result<Self, ApiError> {
        let conn = db::connection().unwrap();
        let visitors_message = VisitorsInsert::from(visitors_message);
        let new_visitor = diesel::insert_into(visitors::table)
            .values(&visitors_message)
            .get_result(&conn)?;

        Ok(new_visitor)
    }

    pub fn update(id: i32, visitors_message: VisitorsMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visitor = diesel::update(visitors::table)
            .filter(visitors::visitor_id.eq(id))
            .set(visitors_message)
            .get_result(&conn)?;

        Ok(visitor)
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            visitors::table
                .filter(visitors::visitor_id.eq(id))
        )
            .execute(&conn)?;

        Ok(res)
    }
}



impl From<VisitorsMessage> for VisitorsInsert {
    fn from(visitors_message: VisitorsMessage) -> Self {
        VisitorsInsert {
            gender: visitors_message.gender,
            prison_id: visitors_message.prison_id,
            first_name:visitors_message.first_name,
            last_name: visitors_message.last_name,
            relations: visitors_message.relations,
            phone_num: visitors_message.phone_num,
            line_id: visitors_message.line_id,
            remark: visitors_message.remark,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
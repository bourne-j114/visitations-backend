use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::visitors;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "visitors"]
pub struct Visitors {
    pub id: i32,
    name: String,
    address1: String,
    address2: String,
    post_code: String,
    id_number: String,
    gender: String,
    phone_number: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Visitors {
    pub fn new(name: String,
               address1: String,
               address2: String,
               post_code: String,
               id_number: String,
               gender: String,
               phone_number: String,) -> VisitorsMessage {
        VisitorsMessage {
            name,
            address1,
            address2,
            post_code,
            id_number,
            gender,
            phone_number
        }
    }

    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection().unwrap();

        let visitor_list = visitors::table
            .load::<Visitors>(&conn)?;

        Ok(visitor_list)
    }

    pub fn find(id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visitor = visitors::table
            .filter(visitors::id.eq(id))
            .first(&conn)?;

        Ok(visitor)
    }

    pub fn find_by_name(name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visitor = visitors::table
            .filter(visitors::name.like(format!("%{}%", name)))
            .first(&conn)?;

        Ok(visitor)
    }


    pub fn create(visitors_message: VisitorsMessage) -> Result<Self, ApiError> {
        visitors_message.insert()
    }

    pub fn update(id: i32, visitors_message: VisitorsMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(visitors::table)
            .filter(visitors::id.eq(id))
            .set(visitors_message)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            visitors::table
                .filter(visitors::id.eq(id))
        )
            .execute(&conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "visitors"]
pub struct VisitorsMessage {
    pub name: String,
    pub address1: String,
    pub address2: String,
    pub post_code: String,
    pub id_number: String,
    pub gender: String,
    pub phone_number: String,
}

impl VisitorsMessage {
    pub fn insert(&self) -> Result<Visitors, ApiError> {
        let conn = db::connection().unwrap();
        let visitors_message = self.clone();
        let new_visitor = diesel::insert_into(visitors::table)
            .values(&visitors_message)
            .get_result(&conn)?;
        
        Ok(new_visitor)
        
    }
}
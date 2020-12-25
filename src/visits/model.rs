use chrono::{NaiveDateTime, Local};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::{visits};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "visits"]
pub struct Visits {
    pub id: Uuid,
    pub prison_id: String,
    pub visitor_id: i32,
    pub visit_date: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub stop_time: NaiveDateTime,
    pub round: i16,
    pub allow: i16,
    pub remark: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "visits"]
pub struct VisitsMessage {
    pub prison_id: String,
    pub visitor_id: i32,
    pub visit_date: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub stop_time: NaiveDateTime,
    pub round: i16,
    pub allow: i16,
    pub remark: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}


impl Visits {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let visit_list = visits::table
            .load::<Visits>(&conn)?;

        Ok(visit_list)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visit = visits::table
            .filter(visits::id.eq(id))
            .first(&conn)?;

        Ok(visit)
    }

    pub fn insert(visits_message: VisitsMessage) -> Result<Visits, ApiError> {
        let conn = db::connection()?;
        let visit_message = Visits::from(visits_message);
        let new_visit = diesel::insert_into(visits::table)
            .values(visit_message)
            .get_result(&conn)?;

        Ok(new_visit)
    }

    pub fn update(id: Uuid, visit_message: VisitsMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let visit = diesel::update(visits::table)
            .filter(visits::id.eq(id))
            .set(visit_message)
            .get_result(&conn)?;

        Ok(visit)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            visits::table
                .filter(visits::id.eq(id))
        ).execute(&conn)?;

        Ok(res)
    }
}

impl From<VisitsMessage> for Visits {
    fn from(visits_message: VisitsMessage) -> Visits {
        Visits {
            id: Uuid::new_v4(),
            prison_id: visits_message.prison_id,
            visitor_id: visits_message.visitor_id,
            visit_date: Local::now().naive_local(),
            start_time: visits_message.start_time,
            stop_time: visits_message.start_time,
            round: visits_message.round,
            allow: visits_message.allow,
            remark: visits_message.remark,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
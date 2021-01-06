use chrono::{NaiveDateTime, Local, Datelike, Duration};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::{visits};
use uuid::Uuid;
use crate::schema::visits::columns::visit_date;
use std::ops::Add;
use crate::visitors::Visitors;
use crate::prisons::Prisons;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "visits"]
pub struct Visits {
    pub id: Uuid,
    pub prison_id: String,
    pub visitor_name: String,
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
    pub visitor_name: String,
    pub allow: i16
}


impl Visits {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let visit_list = visits::table
            .load::<Visits>(&conn)?;

        Ok(visit_list)
    }
    pub fn is_visited(prison_id: String)-> Result<bool, ApiError>{
        let conn = db::connection()?;
        let prison = Prisons::find(prison_id.clone())?;
        let mut rs = true;
        let visit:Visits = visits::table
            .filter(visits::prison_id.eq(prison_id))
            .order(visit_date.desc())
            .first(&conn)?;
        let visited_date= visit.visit_date.format("%Y-%m-%d").to_string();
        if prison.prison_type == 0 {
            rs = verify(visited_date);
        }else{
            let today= Local::now().naive_local().format("%Y-%m-%d").to_string();
            if today == visited_date {
                rs = true;
            }
        }
        Ok(rs)
    }

    pub fn find_by_prison_id(prison_id: String) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let visits = visits::table
            .filter(visits::prison_id.eq(prison_id))
            .order(visit_date.desc())
            .limit(5)
            .load::<Visits>(&conn)?;

        Ok(visits)
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
            visitor_name: visits_message.visitor_name,
            visit_date: Local::now().naive_local(),
            start_time: Local::now().naive_local(),
            stop_time: Local::now().naive_local(),
            round: 0,
            allow: visits_message.allow,
            remark: "-".to_string(),
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}

fn verify(visited: String) -> bool{
    let today= Local::now().naive_local();
    let mut rs = false;
    for i in 0..6 {
        let dt = today - Duration::days(i);
        let temp = dt.weekday().to_string();
        if temp == "Sun".to_string() {
            break;
        }
        let test = dt.format("%Y-%m-%d").to_string();
        if visited == test {
            rs = true;
        }
    }
    rs
}
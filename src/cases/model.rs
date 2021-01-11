use chrono::{NaiveDateTime, Local, Datelike, Duration, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::{cases};

use std::ops::Add;
use crate::visitors::Visitors;
use crate::prisons::Prisons;
use crate::schema::cases::columns::receive_date;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "cases"]
pub struct Cases {
    pub id: Uuid,
    pub prison_id: String,
    pub court_order: String,
    pub case_no: String,
    pub case_detail: String,
    pub police_station: String,
    pub catch_date: NaiveDate,
    pub receive_date: NaiveDate,
    pub jail_date: NaiveDate,
    pub jail_status: i16,
    pub scheduled_release15: Option<NaiveDate>,
    pub scheduled_release45: Option<NaiveDate>,
    pub cause_release: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct CasesMessage {
    pub prison_id: String,
    pub court_order: String,
    pub case_no: String,
    pub case_detail: String,
    pub police_station: String,
    pub catch_date: String,
    pub receive_date: String,
}

#[derive(Debug,Serialize, Deserialize,AsChangeset)]
#[table_name = "cases"]
pub struct CasesUpdate {
    pub prison_id: String,
    pub court_order: String,
    pub case_no: String,
    pub case_detail: String,
    pub police_station: String,
    pub catch_date: NaiveDate,
    pub receive_date: NaiveDate,
    pub jail_date: NaiveDate,
    pub jail_status: i16,
    pub scheduled_release15: Option<NaiveDate>,
    pub scheduled_release45: Option<NaiveDate>,
    pub cause_release: String,
}


impl Cases {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let cases_list = cases::table
            .load::<Cases>(&conn)?;

        Ok(cases_list)
    }

    pub fn find(prison_id: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let case = cases::table
            .filter(cases::prison_id.eq(prison_id))
            .order(receive_date.desc())
            .first(&conn)?;
        Ok(case)
    }

    pub fn find_by_prison_id(prison_id: String) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let cases = cases::table
            .filter(cases::prison_id.eq(prison_id))
            .order(receive_date.desc())
            .limit(5)
            .load::<Cases>(&conn)?;

        Ok(cases)
    }

    pub fn insert(cases_message: CasesMessage) -> Result<Cases, ApiError> {
        let conn = db::connection()?;
        let cases_message = Cases::from(cases_message);
        let new_case = diesel::insert_into(cases::table)
            .values(cases_message)
            .get_result(&conn)?;

        Ok(new_case)
    }

    pub fn update(id: Uuid, cases_message: CasesUpdate) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let update_case = diesel::update(cases::table)
            .filter(cases::id.eq(id))
            .set(cases_message)
            .get_result(&conn)?;

        Ok(update_case)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            cases::table
                .filter(cases::id.eq(id))
        ).execute(&conn)?;

        Ok(res)
    }
}

impl From<CasesMessage> for Cases {
    fn from(cases_message: CasesMessage) -> Cases {
        Cases {
            id: Uuid::new_v4(),
            prison_id: cases_message.prison_id,
            court_order: cases_message.court_order,
            case_no: cases_message.case_no,
            case_detail: cases_message.case_detail,
            police_station: cases_message.police_station,
            catch_date: NaiveDate::parse_from_str(cases_message.catch_date.as_str(),"%Y-%m-%d").unwrap(),
            receive_date: NaiveDate::parse_from_str(cases_message.receive_date.as_str(),"%Y-%m-%d").unwrap(),
            jail_date: Local::now().naive_local().date(),
            jail_status: 1,
            scheduled_release15: None,
            scheduled_release45: None,
            cause_release: "-".to_string(),
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}

use chrono::{NaiveDateTime, Local};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::prisons;
use crate::schema::visitors;
use crate::visitors::Visitors;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "prisons"]
pub struct Prisons {
    pub prison_id: String,
    pub gender: String,
    pub first_name: String,
    pub last_name: String,
    pub location: String,
    pub case_detail: String,
    pub punish: String,
    pub remark: String,
    pub id_card: String,
    pub jail_date: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "prisons"]
pub struct PrisonsMessage {
    pub prison_id: String,
    pub gender: String,
    pub first_name: String,
    pub last_name: String,
    pub location: String,
    pub case_detail: String,
    pub punish: String,
    pub remark: String,
    pub id_card: String,
    pub jail_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "prisons"]
pub struct PrisonLocation {
    pub location: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrisonLocationMessage {
    pub prison_id: String,
    pub location: String,

}

impl Prisons {

    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let prison_list = prisons::table
            .load::<Prisons>(&conn)?;

        Ok(prison_list)
    }

    pub fn find(id: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let prison = prisons::table
            .filter(prisons::prison_id.eq(id))
            .first(&conn)?;

        Ok(prison)
    }

    pub fn find_by_first_name(first_name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let prison = prisons::table
            .filter(prisons::first_name.like(format!("%{}%", first_name)))
            .first(&conn)?;

        Ok(prison)
    }

    pub fn find_by_last_name(last_name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let prison = prisons::table
            .filter(prisons::last_name.like(format!("%{}%", last_name)))
            .first(&conn)?;

        Ok(prison)
    }


    pub fn create(prisons_message: PrisonsMessage) -> Result<Self, ApiError> {
        let conn = db::connection().unwrap();
        let prisons_message = Prisons::from(prisons_message);
        let new_prison = diesel::insert_into(prisons::table)
            .values(&prisons_message)
            .get_result(&conn)?;

        Ok(new_prison)
    }

    pub fn update(id: String, prison_message: PrisonsMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let prison = diesel::update(prisons::table)
            .filter(prisons::prison_id.eq(id))
            .set(prison_message)
            .get_result(&conn)?;

        Ok(prison)
    }

    pub fn delete(id: String) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            prisons::table
                .filter(prisons::prison_id.eq(id))
        ).execute(&conn)?;

        Ok(res)
    }
}


impl PrisonLocation {
    pub fn update(prison_location_message: PrisonLocationMessage) -> Result<Prisons, ApiError> {
        let conn = db::connection()?;
        let prison_location = PrisonLocation::from(prison_location_message.clone());
        let update_prison = diesel::update(prisons::table)
            .filter(prisons::prison_id.eq(prison_location_message.prison_id))
            .set(prison_location)
            .get_result(&conn)?;

        Ok(update_prison)
    }
}


impl PrisonsMessage {
    pub fn insert(&self) -> Result<Prisons, ApiError> {
        let conn = db::connection()?;
        let prison_message = self.clone();
        let new_prison = diesel::insert_into(prisons::table)
            .values(&prison_message)
            .get_result(&conn)?;
        
        Ok(new_prison)
    }
}
impl From<PrisonLocationMessage> for PrisonLocation {
    fn from(prison_location_message: PrisonLocationMessage) -> Self {
        PrisonLocation {
            location: prison_location_message.location,
            updated_at: Some(Local::now().naive_local()),
        }
    }
}

impl From<PrisonsMessage> for Prisons {
    fn from(prisons_message: PrisonsMessage) -> Self {
        Prisons {
            prison_id: prisons_message.prison_id,
            gender: prisons_message.gender,
            first_name: prisons_message.first_name,
            last_name: prisons_message.last_name,
            location: prisons_message.location,
            case_detail: prisons_message.case_detail,
            punish: prisons_message.punish,
            remark: prisons_message.remark,
            id_card: prisons_message.id_card,
            jail_date: prisons_message.jail_date,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
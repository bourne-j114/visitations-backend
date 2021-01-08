use chrono::{NaiveDateTime, Local};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::{db};
use crate::schema::prisons;
use crate::schema::visitors;
use crate::visitors::Visitors;
use diesel::dsl::sql;
use diesel::sql_types::Bool;
use diesel::sql_query;

#[derive(PartialEq,QueryableByName, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "prisons"]
pub struct Prisons {
    pub prison_id: String,
    pub gender: String,
    pub first_name: String,
    pub last_name: String,
    pub nick_name: String,
    pub birth_day: String,
    pub picture_paht: String,
    pub location: String,
    pub prison_type: i16,
    pub remark: String,
    pub id_card: String,
    pub address_no: String,
    pub moo: String,
    pub subdistric: String,
    pub distric: String,
    pub province: String,
    pub race: String,
    pub nationality: String,
    pub religion: String,
    pub blame: String,
    pub education: String,
    pub edu_institution: String,
    pub status: String,
    pub child: i16,
    pub sibling: i16,
    pub child_in_a_child: i16,
    pub home_owner: String,
    pub stay_address_no: String,
    pub stay_moo: String,
    pub stay_subdistric: String,
    pub stay_distric: String,
    pub stay_province: String,
    pub occupation: String,
    pub income: String,
    pub history_punish: String,
    pub history_punish_year: i16,
    pub history_punish_month: i16,
    pub history_punish_day: i16,
    pub prove_pass_num: i16,
    pub cur_num: i16,
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
    pub nick_name: String,
    pub birth_day: String,
    pub prison_type: i16,
    pub id_card: String,
    pub address_no: String,
    pub moo: String,
    pub subdistric: String,
    pub distric: String,
    pub province: String,
    pub race: String,
    pub nationality: String,
    pub religion: String,
    pub blame: String,
    pub education: String,
    pub edu_institution: String,
    pub status: String,
    pub child: i16,
    pub sibling: i16,
    pub child_in_a_child: i16,
    pub home_owner: String,
    pub stay_address_no: String,
    pub stay_moo: String,
    pub stay_subdistric: String,
    pub stay_distric: String,
    pub stay_province: String,
    pub occupation: String,
    pub income: String,
    pub history_punish: String,
    pub history_punish_year: i16,
    pub history_punish_month: i16,
    pub history_punish_day: i16,
    pub prove_pass_num: i16,
    pub cur_num: i16,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrisonsSlim {
    pub prison_id: String,
    pub full_name: String,
}



impl Prisons {

    pub fn find_all() -> Result<Vec<PrisonsSlim>, ApiError> {
        let conn = db::connection()?;
        let prison_list: Vec<Prisons> = sql_query("SELECT * FROM prisons ORDER BY prison_id")
            .load(&conn)?;
        let mut prison_vec = vec![];
        for v in prison_list{
            let tmp = PrisonsSlim{
                prison_id: v.prison_id,
                full_name: format!("{} {}",v.first_name,v.last_name),
            };
            prison_vec.push(tmp);
        }
        Ok(prison_vec)
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

    pub fn create(prisons_message: PrisonsMessage,prison_id: String) -> Result<bool, ApiError> {
        let conn = db::connection().unwrap();
        let mut rs = false;
        match Self::find(prison_id){
            Ok(r) => {
                Self::update(r.prison_id,prisons_message)?;
            }
            Err(_) => {
                let prisons_message = Prisons::from(prisons_message);
                 diesel::insert_into(prisons::table)
                    .values(&prisons_message)
                    .execute(&conn)?;
                rs = true;
            }
        }
        Ok(rs)
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
            nick_name: prisons_message.nick_name,
            birth_day: prisons_message.birth_day,
            picture_paht: "temp".to_string(),
            location: "-".to_string(),
            prison_type: prisons_message.prison_type,
            remark: "-".to_string(),
            id_card: prisons_message.id_card,
            address_no: prisons_message.address_no,
            moo: prisons_message.moo,
            subdistric: prisons_message.subdistric,
            distric: prisons_message.distric,
            province: prisons_message.province,
            race: prisons_message.race,
            nationality: prisons_message.nationality,
            religion: prisons_message.religion,
            blame: prisons_message.blame,
            education: prisons_message.education,
            edu_institution: prisons_message.edu_institution,
            status: "1".to_string(),
            child: prisons_message.child,
            sibling: prisons_message.sibling,
            child_in_a_child: prisons_message.child_in_a_child,
            home_owner: prisons_message.home_owner,
            stay_address_no: prisons_message.stay_address_no,
            stay_moo: prisons_message.stay_moo,
            stay_subdistric: prisons_message.stay_subdistric,
            stay_distric: prisons_message.stay_distric,
            stay_province: prisons_message.stay_province,
            occupation: prisons_message.occupation,
            income: prisons_message.income,
            history_punish: prisons_message.history_punish,
            history_punish_year: prisons_message.history_punish_year,
            history_punish_month: prisons_message.history_punish_month,
            history_punish_day: prisons_message.history_punish_day,
            prove_pass_num: prisons_message.prove_pass_num,
            cur_num: prisons_message.cur_num,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
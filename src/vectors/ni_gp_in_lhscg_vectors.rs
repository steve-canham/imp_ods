use crate::import::rec_structs::LinkedNIGPRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct LinkedNIGPVecs {
	pub codes: Vec<String>,
	pub parent_orgs: Vec<String>,
	pub parent_org_types: Vec<String>,
	pub join_parent_dates: Vec<Option<NaiveDate>>,
	pub left_parent_dates: Vec<Option<NaiveDate>>,
}


impl LinkedNIGPVecs{
    pub fn new(vsize: usize) -> Self {
        LinkedNIGPVecs {
            codes: Vec::with_capacity(vsize),
            parent_orgs: Vec::with_capacity(vsize),
            parent_org_types: Vec::with_capacity(vsize),
            join_parent_dates: Vec::with_capacity(vsize),
            left_parent_dates: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &LinkedNIGPRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.parent_orgs.push(r.parent_org.clone());
        self.parent_org_types.push(r.parent_org_type.clone());
        self.join_parent_dates.push(r.join_parent_date.clone());
        self.left_parent_dates.push(r.left_parent_date.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.ni_gps_in_lhscg (ods_code, parent_org, parent_org_type, 
                        join_parent_date, left_parent_date) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::date[], $5::date[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.parent_orgs).bind(&self.parent_org_types)
        .bind(&self.join_parent_dates).bind(&self.left_parent_dates)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

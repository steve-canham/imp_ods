use crate::import::rec_structs::NIGPRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct NIGPVecs {
	pub codes: Vec<String>,
	pub names: Vec<String>,
	pub health_geogs: Vec<String>,
	pub cities: Vec<String>,
	pub postcodes: Vec<String>,
	pub postal_adds: Vec<String>,
	pub open_dates: Vec<Option<NaiveDate>>,
	pub close_dates: Vec<Option<NaiveDate>>,
	pub subtype_codes: Vec<String>,
	pub statuses: Vec<String>,
}


impl NIGPVecs{
    pub fn new(vsize: usize) -> Self {
        NIGPVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            health_geogs: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            subtype_codes: Vec::with_capacity(vsize),
            statuses: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &NIGPRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.health_geogs.push(r.health_geog.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.postal_adds.push(r.postal_add.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.subtype_codes.push(r.subtype_code.clone());
        self.statuses.push(r.status.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.ni_gps (ods_code, ods_name, health_geog, 
                      city, postcode, postal_add, open_date, close_date, subtype_code, status) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::text[], $7::date[], $8::date[], $9::text[], $10::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names).bind(&self.health_geogs)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .bind(&self.open_dates).bind(&self.close_dates).bind(&self.subtype_codes).bind(&self.statuses)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

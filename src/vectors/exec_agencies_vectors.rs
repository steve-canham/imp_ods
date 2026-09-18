use crate::import::rec_structs::EARec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct EAVecs {
	pub codes: Vec<String>,
	pub names: Vec<String>,
	pub cities: Vec<String>,
	pub postcodes: Vec<String>,
	pub postal_adds: Vec<String>,
	pub open_dates: Vec<Option<NaiveDate>>,
	pub close_dates: Vec<Option<NaiveDate>>,
	pub parent_orgs: Vec<String>,
}


impl EAVecs{
    pub fn new(vsize: usize) -> Self {
        EAVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            parent_orgs: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &EARec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.postal_adds.push(r.postal_add.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.parent_orgs.push(r.parent_org.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.exec_agencies (ods_code, ods_name, 
                      city, postcode, postal_add, open_date, close_date, parent_org) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::date[], $7::date[], $8::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .bind(&self.open_dates).bind(&self.close_dates).bind(&self.parent_orgs)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

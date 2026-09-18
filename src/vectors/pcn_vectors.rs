use crate::import::rec_structs::PCNRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct PCNVecs {
	pub codes: Vec<String>,
	pub names: Vec<String>,
	pub subicb_locs: Vec<String>,
	pub subicb_names: Vec<String>,
	pub open_dates: Vec<Option<NaiveDate>>,
	pub close_dates: Vec<Option<NaiveDate>>,
    pub cities: Vec<String>,
	pub postcodes: Vec<String>,
	pub postal_adds: Vec<String>,
}


impl PCNVecs{
    pub fn new(vsize: usize) -> Self {
        PCNVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            subicb_locs: Vec::with_capacity(vsize),
            subicb_names: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize), 
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &PCNRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.subicb_locs.push(r.subicb_loc.clone());
        self.subicb_names.push(r.subicb_name.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.postal_adds.push(r.postal_add.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.pcns (ods_code, ods_name, 
                      subicb_loc, subicb_name, open_date, close_date, city, postcode, postal_add) 
                      SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], 
                               $5::date[], $6::date[], $7::text[], $8::text[], $9::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names).bind(&self.subicb_locs).bind(&self.subicb_names)
        .bind(&self.open_dates).bind(&self.close_dates)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

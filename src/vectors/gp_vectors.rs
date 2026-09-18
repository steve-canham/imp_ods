use crate::import::rec_structs::GPRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct GPVecs {
	pub codes: Vec<String>,
	pub names: Vec<String>,
	pub groupings: Vec<String>,
	pub health_geogs: Vec<String>,
	pub cities: Vec<String>,
	pub postcodes: Vec<String>,
	pub postal_adds: Vec<String>,
	pub open_dates: Vec<Option<NaiveDate>>,
	pub close_dates: Vec<Option<NaiveDate>>,
    pub statuses: Vec<String>,
	pub subtype_codes: Vec<String>,
    pub commissioners: Vec<String>,
	pub join_parent_dates: Vec<Option<NaiveDate>>,
	pub left_parent_dates: Vec<Option<NaiveDate>>,
    pub provpurchs: Vec<String>,
	pub prescribing_settings: Vec<String>,
}


impl GPVecs{
    pub fn new(vsize: usize) -> Self {
        GPVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            groupings: Vec::with_capacity(vsize),
            health_geogs: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            statuses: Vec::with_capacity(vsize),
            subtype_codes: Vec::with_capacity(vsize),
            commissioners: Vec::with_capacity(vsize),
            join_parent_dates: Vec::with_capacity(vsize),
            left_parent_dates: Vec::with_capacity(vsize),
            provpurchs: Vec::with_capacity(vsize),
            prescribing_settings: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &GPRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.groupings.push(r.grouping.clone());
        self.health_geogs.push(r.health_geog.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.postal_adds.push(r.postal_add.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.statuses.push(r.status.clone());
        self.subtype_codes.push(r.subtype_code.clone());
        self.commissioners.push(r.commissioner.clone());
        self.join_parent_dates.push(r.join_parent_date.clone());
        self.left_parent_dates.push(r.left_parent_date.clone());
        self.provpurchs.push(r.provpurch.clone());
        self.prescribing_settings.push(r.prescribing_setting.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.gps (ods_code, ods_name, grouping, health_geog, 
                      city, postcode, postal_add, open_date, close_date, status, 
                      subtype_code, commissioner, join_parent_date, left_parent_date,
                      provpurch, prescribing_setting) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::text[], $7::text[], $8::date[], $9::date[], $10::text[], $11::text[], 
                $12::text[], $13::date[], $14::date[], $15::text[], $16::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names).bind(&self.groupings).bind(&self.health_geogs)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .bind(&self.open_dates).bind(&self.close_dates).bind(&self.statuses)
        .bind(&self.subtype_codes).bind(&self.commissioners)
        .bind(&self.join_parent_dates).bind(&self.left_parent_dates)
        .bind(&self.provpurchs).bind(&self.prescribing_settings)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}


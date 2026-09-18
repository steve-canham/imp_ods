use crate::import::rec_structs::SuccRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct SuccVecs {
	pub codes: Vec<String>,
	pub succ_ods_codes: Vec<String>,
	pub succ_reason_codes: Vec<String>,
	pub effective_dates: Vec<Option<NaiveDate>>,
	pub succession_indicators: Vec<String>,
}


impl SuccVecs{
    pub fn new(vsize: usize) -> Self {
        SuccVecs {
            codes: Vec::with_capacity(vsize),
            succ_ods_codes: Vec::with_capacity(vsize),
            succ_reason_codes: Vec::with_capacity(vsize),
            effective_dates: Vec::with_capacity(vsize),
            succession_indicators: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &SuccRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.succ_ods_codes.push(r.succ_ods_code.clone());
        self.succ_reason_codes.push(r.succ_reason_code.clone());
        self.effective_dates.push(r.effective_date.clone());
        self.succession_indicators.push(r.succession_indicator.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.successions (ods_code, succ_ods_code, succ_reason_code, 
                      effective_date, succession_indicator) 
                SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::date[], $5::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.succ_ods_codes).bind(&self.succ_reason_codes)
        .bind(&self.effective_dates).bind(&self.succession_indicators)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

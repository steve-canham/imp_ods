use crate::AppError;
use crate::utils;

use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use log::info;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct SuccLine {
	ods_code: String,
	succ_ods_code: String,
	succ_reason_code: String,
	effective_date: String,
	succession_indicator: String,
}

#[derive(Debug)]
struct SuccRec {
	pub ods_code: String,
	pub succ_ods_code: String,
	pub succ_reason_code: String,
	pub effective_date:  Option<NaiveDate>, 
	pub succession_indicator: String,
}

struct SuccVecs {
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

pub async fn import_data(data_folder: &PathBuf, source_file_name: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    let source_file_path: PathBuf = [data_folder, &PathBuf::from(source_file_name)].iter().collect();
    let file = File::open(source_file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .quote(b'"')
        .from_reader(buf_reader);
    
    let mut i = 0;
    let vector_size = 10000;
    let mut dv: SuccVecs = SuccVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: SuccLine = result?;
        let eff_date = utils::convert_to_date(&source.effective_date);
         
        let succ_rec = SuccRec {
            ods_code: source.ods_code,
            succ_ods_code: source.succ_ods_code,
            succ_reason_code: source.succ_reason_code,
            effective_date: eff_date,
            succession_indicator: source.succession_indicator,
         };

        dv.add_data(&succ_rec);   // transfer data to vectors
        i+=1;    
    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.succ_rec", i, source_file_name);
    Ok(())
}
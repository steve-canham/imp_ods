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
struct LinkedGPLine {
	ods_code: String,
	parent_org: String,
	parent_org_type: String,
	join_parent_date: String,
	left_parent_date: String,
	amended_record: String,
}

#[derive(Debug)]
struct LinkedGPRec {
	pub ods_code: String,
	pub parent_org: String,
	pub parent_org_type: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>,
}

struct LinkedGPVecs {
	pub codes: Vec<String>,
	pub parent_orgs: Vec<String>,
    pub parent_org_types: Vec<String>,
	pub join_parent_dates: Vec<Option<NaiveDate>>,
	pub left_parent_dates: Vec<Option<NaiveDate>>,
}

impl LinkedGPVecs{
    pub fn new(vsize: usize) -> Self {
        LinkedGPVecs {
            codes: Vec::with_capacity(vsize),
            parent_orgs: Vec::with_capacity(vsize),
            parent_org_types: Vec::with_capacity(vsize),
            join_parent_dates: Vec::with_capacity(vsize),
            left_parent_dates: Vec::with_capacity(vsize),
        }
    }

    pub fn add_data(&mut self, r: &LinkedGPRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.parent_orgs.push(r.parent_org.clone());
        self.parent_org_types.push(r.parent_org_type.clone());
        self.join_parent_dates.push(r.join_parent_date.clone());
        self.left_parent_dates.push(r.left_parent_date.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.gpmem (ods_code, parent_org, parent_org_type, join_parent_date, left_parent_date) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::date[], $5::date[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.parent_orgs).bind(&self.parent_org_types)
        .bind(&self.join_parent_dates).bind(&self.left_parent_dates)
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
    let mut dv: LinkedGPVecs = LinkedGPVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: LinkedGPLine = result?;
        let joined = utils::convert_to_date(&source.join_parent_date);
        let left = utils::convert_to_date(&source.left_parent_date);
        
        let gpmem_rec = LinkedGPRec {
            ods_code: source.ods_code,
            parent_org: source.parent_org,
            parent_org_type: source.parent_org_type,
            join_parent_date: joined,
            left_parent_date: left,
        };

        dv.add_data(&gpmem_rec);   // transfer data to vectors
        i+=1;    
    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.gpmem", i, source_file_name);

    Ok(())
}
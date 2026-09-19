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
struct CSULine {
	ods_code: String,
	ods_name: String,
	column3: String,
	column4 : String,
	aline1: String,
	aline2: String,
	aline3: String,
	aline4: String,
	aline5: String,
	postcode: String,
	open_date: String,
	close_date: String,
    column13: String,
	subtype_code: String,
	column15: String,
	column16: String,
	column17 : String,
	column18: String,
	column19: String,
	column20: String,
	column21: String,
	amended_record: String,
	column23: String,
	column24: String,
	column25: String,
	column26: String,
	column27: String,
}

#[derive(Debug)]
struct CSURec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

struct CSUVecs {
    pub codes: Vec<String>,
    pub names: Vec<String>,
    pub cities: Vec<String>,
    pub postcodes: Vec<String>,
    pub postal_adds: Vec<String>,
    pub open_dates: Vec<Option<NaiveDate>>,
    pub close_dates: Vec<Option<NaiveDate>>,
    pub subtype_codes: Vec<String>,
}

impl CSUVecs{
    pub fn new(vsize: usize) -> Self {
        CSUVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            subtype_codes: Vec::with_capacity(vsize),
        }
    }

    pub fn add_data(&mut self, r: &CSURec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.postal_adds.push(r.postal_add.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.subtype_codes.push(r.subtype_code.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.csus (ods_code, ods_name, 
                      city, postcode, postal_add, open_date, close_date, subtype_code) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                      $6::date[], $7::date[], $8::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .bind(&self.open_dates).bind(&self.close_dates).bind(&self.subtype_codes)
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
    let mut dv: CSUVecs = CSUVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: CSULine = result?;
        let site_name =  utils::capitalise_site_name(&source.ods_name);
        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);        
        let opened = utils::convert_to_date(&source.open_date);
        let closed = utils::convert_to_date(&source.close_date);
        
        let csu_rec = CSURec {
            ods_code: source.ods_code,
            ods_name: site_name,
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,
            open_date: opened,
            close_date: closed,
            subtype_code: source.subtype_code,
        };

        dv.add_data(&csu_rec);   // transfer data to vectors
        i+=1;    
    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.csus", i, source_file_name);
    Ok(())
}
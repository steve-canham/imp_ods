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
struct CTLine {
	ods_code: String,
	ods_name: String,
	grouping: String,
	health_geog : String,
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
	contact_tel: String,
	column19: String,
	column20: String,
	column21: String,
	amended_record: String,
	column23: String,
	column25: String,
	column26: String,
	column27: String,
}

#[derive(Debug)]
struct CTRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

struct CTVecs {
	pub codes: Vec<String>,
	pub names: Vec<String>,
	pub groupings: Vec<String>,
	pub health_geogs: Vec<String>,
	pub cities: Vec<String>,
	pub postcodes: Vec<String>,
	pub postal_adds: Vec<String>,
	pub open_dates: Vec<Option<NaiveDate>>,
	pub close_dates: Vec<Option<NaiveDate>>,
	pub subtype_codes: Vec<String>,
}

impl CTVecs{
    pub fn new(vsize: usize) -> Self {
        CTVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            groupings: Vec::with_capacity(vsize),
            health_geogs: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            subtype_codes: Vec::with_capacity(vsize),
        }
    }

    pub fn add_data(&mut self, r: &CTRec) 
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
        self.subtype_codes.push(r.subtype_code.clone());
    }

    pub fn shrink_to_fit(&mut self) 
    {
        self.codes.shrink_to_fit();
        self.names.shrink_to_fit();
        self.groupings.shrink_to_fit();
        self.health_geogs.shrink_to_fit();
        self.cities.shrink_to_fit();
        self.postcodes.shrink_to_fit();
        self.postal_adds.shrink_to_fit();
        self.open_dates.shrink_to_fit();
        self.close_dates.shrink_to_fit();
        self.subtype_codes.shrink_to_fit();
    }
    
    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.care_trusts (ods_code, ods_name, grouping, health_geog, 
                      city, postcode, postal_add, open_date, close_date, subtype_code) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::text[], $7::text[], $8::date[], $9::date[], $10::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names).bind(&self.groupings).bind(&self.health_geogs)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .bind(&self.open_dates).bind(&self.close_dates).bind(&self.subtype_codes)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

// Only about 11 care trusts listed

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
    let vector_size = 100;
    let mut dv: CTVecs = CTVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: CTLine = result?;
        let site_name =  utils::capitalise_site_name(&source.ods_name);
        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);        
        let opened = utils::convert_to_date(&source.open_date);
        let closed = utils::convert_to_date(&source.close_date);
        
        let care_trust_rec = CTRec {
            ods_code: source.ods_code,
            ods_name: site_name,
            grouping: source.grouping,
            health_geog: source.health_geog,
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,
            open_date: opened,
            close_date: closed,
            subtype_code: source.subtype_code,
        };

        dv.add_data(&care_trust_rec);   // transfer data to vectors
        i+=1;    
    }
           
    dv.shrink_to_fit(); 
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.care_trusts", i, source_file_name);
    Ok(())
}
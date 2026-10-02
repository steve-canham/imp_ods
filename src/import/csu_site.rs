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
struct CSUSiteLine {
	ods_code: String,
	ods_name: String,
	column3: String,
	column4: String,
	aline1: String,
	aline2: String,
	aline3: String,
	aline4: String,
	aline5: String,
	postcode: String,
	open_date: String,
	close_date: String,
    column13: String,
	column14: String,
	parent_org: String,
	join_parent_date: String,
	left_parent_date: String,
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
pub struct CSUSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub parent_org: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>, 
}

pub struct CSUSiteVecs {
    pub codes: Vec<String>,
    pub names: Vec<String>,
    pub cities: Vec<String>,
    pub postcodes: Vec<String>,
    pub postal_adds: Vec<String>,
    pub open_dates: Vec<Option<NaiveDate>>,
    pub close_dates: Vec<Option<NaiveDate>>,
    pub parent_orgs: Vec<String>,
    pub join_parent_dates: Vec<Option<NaiveDate>>,
	pub left_parent_dates: Vec<Option<NaiveDate>>,
}

impl CSUSiteVecs{
    pub fn new(vsize: usize) -> Self {
        CSUSiteVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            parent_orgs: Vec::with_capacity(vsize),
            join_parent_dates: Vec::with_capacity(vsize),
            left_parent_dates: Vec::with_capacity(vsize),
        }
    }

    pub fn add_data(&mut self, r: &CSUSiteRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.postal_adds.push(r.postal_add.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.parent_orgs.push(r.parent_org.clone());
        self.join_parent_dates.push(r.join_parent_date.clone());
        self.left_parent_dates.push(r.left_parent_date.clone());
    }

    pub fn shrink_to_fit(&mut self) 
    {
        self.codes.shrink_to_fit();
        self.names.shrink_to_fit();
        self.cities.shrink_to_fit();
        self.postcodes.shrink_to_fit();
        self.postal_adds.shrink_to_fit();
        self.open_dates.shrink_to_fit();
        self.close_dates.shrink_to_fit();
        self.parent_orgs.shrink_to_fit();
        self.join_parent_dates.shrink_to_fit();
        self.left_parent_dates.shrink_to_fit();
    }
       
    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.csu_sites (ods_code, ods_name, 
                      city, postcode, postal_add, open_date, close_date, parent_org,
                      join_parent_date, left_parent_date) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::date[], $7::date[], $8::text[], $9::date[], $10::date[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names)
        .bind(&self.cities).bind(&self.postcodes).bind(&self.postal_adds)
        .bind(&self.open_dates).bind(&self.close_dates).bind(&self.parent_orgs)
        .bind(&self.join_parent_dates).bind(&self.left_parent_dates)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

// approx 150 records created

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
    let vector_size = 200;
    let mut dv: CSUSiteVecs = CSUSiteVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: CSUSiteLine = result?;
        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);        
        let csu_site_rec = CSUSiteRec {
            ods_code: source.ods_code,
            ods_name: utils::capitalise_site_name(&source.ods_name),
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,
            open_date: NaiveDate::parse_from_str(&source.open_date, "%Y%m%d").ok(),
            close_date: NaiveDate::parse_from_str(&source.close_date, "%Y%m%d").ok(),
            parent_org: source.parent_org,
            join_parent_date: NaiveDate::parse_from_str(&source.join_parent_date, "%Y%m%d").ok(),
            left_parent_date: NaiveDate::parse_from_str(&source.left_parent_date, "%Y%m%d").ok(),
        };

        dv.add_data(&csu_site_rec);   // transfer data to vectors
        i+=1;    
    }

    dv.shrink_to_fit(); 
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.csu_sites", i, source_file_name);
    Ok(())
}
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
struct GPLine {
	ods_code : String,
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
    status: String,
	subtype_code: String,
	commissioner: String,
	join_provpurch_date: String,
	left_provpurch_date : String,
	contact_tel: String,
	column19: String,
	column20: String,
	column21: String,
	amended_record: String,
	column23: String,
	provpurch: String,
	column25: String,
	prescribing_setting: String,
	column27: String,
}

#[derive(Debug)]
struct GPRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub status: String,
	pub subtype_code: String,
	pub commissioner: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>, 
	pub provpurch: String,
	pub prescribing_setting: String,
}

struct GPVecs {
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
    let mut dv: GPVecs = GPVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: GPLine = result?;
        let site_name =  utils::capitalise_site_name(&source.ods_name);
        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);        
        let opened = utils::convert_to_date(&source.open_date);
        let closed = utils::convert_to_date(&source.close_date);
        let joined = utils::convert_to_date(&source.join_provpurch_date);
        let left = utils::convert_to_date(&source.left_provpurch_date);
        
        let ccg_site_rec = GPRec {
            ods_code: source.ods_code,
            ods_name: site_name,
            grouping: source.grouping,
            health_geog: source.health_geog,
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,
            open_date: opened,
            close_date: closed,
            status: source.status,
            subtype_code: source.subtype_code,
            commissioner: source.commissioner,
            join_parent_date: joined,
            left_parent_date: left,
            provpurch: source.provpurch,
            prescribing_setting: source.prescribing_setting,
        };

        dv.add_data(&ccg_site_rec);   // transfer data to vectors
        i+=1;    
    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.gps", i, source_file_name);

    Ok(())
}
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
struct PCNPartnerLine {
	ods_code: String,
	ods_name: String,
	parent_subicb_loc: String,
	parent_subicb_name: String,
    pcn_code: String,
	pcn_name: String,
	pcn_parent_subicb_loc: String,
	pcn_parent_subicb_name: String,
    start_date: String,
	end_date: String,
	icbs_match: String,
}

#[derive(Debug)]
struct PCNPartnerRec {
	pub ods_code: String,
	pub ods_name: String,
	pub parent_subicb_loc: String,
	pub parent_subicb_name: String,
    pub pcn_code: String,
	pub pcn_name: String,
	pub pcn_parent_subicb_loc: String,
	pub pcn_parent_subicb_name: String,
    pub start_date: Option<NaiveDate>,
	pub end_date: Option<NaiveDate>,
	pub icbs_match: bool,
}

struct PCNPartnerVecs {
	pub codes: Vec<String>,
	pub names: Vec<String>,
	pub parent_subicb_locs: Vec<String>,
	pub parent_subicb_names: Vec<String>,
    pub pcn_codes: Vec<String>,
	pub pcn_names: Vec<String>,
	pub pcn_parent_subicb_locs: Vec<String>,
	pub pcn_parent_subicb_names: Vec<String>,
	pub start_dates: Vec<Option<NaiveDate>>,
	pub end_dates: Vec<Option<NaiveDate>>,
	pub icbs_matches: Vec<bool>,
}

impl PCNPartnerVecs{
    pub fn new(vsize: usize) -> Self {
        PCNPartnerVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            parent_subicb_locs: Vec::with_capacity(vsize),
            parent_subicb_names: Vec::with_capacity(vsize),
            pcn_codes: Vec::with_capacity(vsize),
            pcn_names: Vec::with_capacity(vsize),
            pcn_parent_subicb_locs: Vec::with_capacity(vsize),
            pcn_parent_subicb_names: Vec::with_capacity(vsize),
            start_dates: Vec::with_capacity(vsize),
            end_dates: Vec::with_capacity(vsize),
            icbs_matches: Vec::with_capacity(vsize),
        }
    }

    pub fn add_data(&mut self, r: &PCNPartnerRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.parent_subicb_locs.push(r.parent_subicb_loc.clone());
        self.parent_subicb_names.push(r.parent_subicb_name.clone());
        self.pcn_codes.push(r.pcn_code.clone());
        self.pcn_names.push(r.pcn_name.clone());
        self.pcn_parent_subicb_locs.push(r.pcn_parent_subicb_loc.clone());
        self.pcn_parent_subicb_names.push(r.pcn_parent_subicb_name.clone());
        self.start_dates.push(r.start_date.clone());
        self.end_dates.push(r.end_date.clone());
        self.icbs_matches.push(r.icbs_match);
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO ods.pcn_partners (ods_code, ods_name, 
                      parent_subicb_loc, parent_subicb_name, pcn_code, pcn_name, 
                      pcn_par_subicb_loc, pcn_par_subicb_name, start_date, end_date, icbs_match) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                      $6::text[], $7::text[], $8::text[], $9::date[], $10::date[], $11::bool[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names)
        .bind(&self.parent_subicb_locs).bind(&self.parent_subicb_names)
        .bind(&self.pcn_codes).bind(&self.pcn_names)
        .bind(&self.pcn_parent_subicb_locs).bind(&self.pcn_parent_subicb_names)
        .bind(&self.start_dates).bind(&self.end_dates).bind(&self.icbs_matches)
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
    let mut dv: PCNPartnerVecs = PCNPartnerVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: PCNPartnerLine = result?;
        let site_name =  utils::capitalise_site_name(&source.ods_name);
        let started = utils::convert_to_date(&source.start_date);
        let ended = utils::convert_to_date(&source.end_date);
        let icbsmatch = if source.icbs_match == "TRUE" {true} else {false};
      
        let pcn_partner_rec = PCNPartnerRec {
            ods_code: source.ods_code,
            ods_name: site_name,
            parent_subicb_loc: source.parent_subicb_loc,
            parent_subicb_name: utils::capitalise_field(&source.parent_subicb_name),
            pcn_code: source.pcn_code,
            pcn_name: utils::capitalise_field(&source.pcn_name),
            pcn_parent_subicb_loc: source.pcn_parent_subicb_loc,
            pcn_parent_subicb_name: utils::capitalise_field(&source.pcn_parent_subicb_name),
            start_date: started,
            end_date: ended,
            icbs_match: icbsmatch,
        };

        dv.add_data(&pcn_partner_rec);   // transfer data to vectors
        i+=1;    
    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.pcn_partners", i, source_file_name);
    Ok(())
}
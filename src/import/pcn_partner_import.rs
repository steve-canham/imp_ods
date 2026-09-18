use crate::AppError;
use crate::utils;
use crate::vectors::pcn_partner_vectors::PCNPartnerVecs;
use super::rec_structs::PCNPartnerRec;

use sqlx::{Pool, Postgres};
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
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.pcn_partners", i, source_file_name);

    Ok(())
}
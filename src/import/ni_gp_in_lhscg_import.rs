use crate::AppError;
use crate::utils;
use crate::vectors::ni_gp_in_lhscg_vectors::LinkedNIGPVecs;
use super::rec_structs::LinkedNIGPRec;

use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use log::info;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct LinkedNIGPLine {
	ods_code: String,
	parent_org: String,
	parent_org_type: String,
	join_parent_date: String,
	left_parent_date: String,
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
    let mut dv: LinkedNIGPVecs = LinkedNIGPVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: LinkedNIGPLine = result?;
        let joined = utils::convert_to_date(&source.join_parent_date);
        let left = utils::convert_to_date(&source.left_parent_date);
        
        let ccg_site_rec = LinkedNIGPRec {
            ods_code: source.ods_code,
            parent_org: source.parent_org,
            parent_org_type: source.parent_org_type,
            join_parent_date: joined,
            left_parent_date: left,
        };

        dv.add_data(&ccg_site_rec);   // transfer data to vectors
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.ni_gps_in_lhscg", i, source_file_name);

    Ok(())
}
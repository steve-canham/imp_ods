use crate::AppError;
use crate::utils;
use crate::vectors::succession_vectors::SuccVecs;
use super::rec_structs::SuccRec;

use sqlx::{Pool, Postgres};
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
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.succ_rec", i, source_file_name);

    Ok(())
}
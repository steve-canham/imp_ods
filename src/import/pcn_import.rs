use crate::AppError;
use crate::utils;
use crate::vectors::pcn_vectors::PCNVecs;
use super::rec_structs::PCNRec;

use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use log::info;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct PCNLine {
	ods_code: String,
	ods_name: String,
	subicb_loc: String,
	subicb_name: String,
    open_date: String,
	close_date: String,
	aline1: String,
	aline2: String,
	aline3: String,
	aline4: String,
	aline5: String,
	postcode: String,
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
    let mut dv: PCNVecs = PCNVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: PCNLine = result?;
        let site_name =  utils::capitalise_site_name(&source.ods_name);
        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);       
        let opened = utils::convert_to_date(&source.open_date);
        let closed = utils::convert_to_date(&source.close_date);
           
        let pcn_rec = PCNRec {
            ods_code: source.ods_code,
            ods_name: site_name,
            subicb_loc: source.subicb_loc,
            subicb_name: utils::capitalise_field(&source.subicb_name),
            open_date: opened,
            close_date: closed,
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,

        };

        dv.add_data(&pcn_rec);   // transfer data to vectors
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.pcn", i, source_file_name);

    Ok(())
}
use crate::AppError;
use crate::utils;
use crate::vectors::gp_vectors::GPVecs;
use super::rec_structs::GPRec;

use sqlx::{Pool, Postgres};
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
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.gps", i, source_file_name);

    Ok(())
}
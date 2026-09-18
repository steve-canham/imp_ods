use crate::import::rec_structs::PCNPartnerRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct PCNPartnerVecs {
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
                      pcn_parent_subicb_loc, pcn_parent_subicb_name, start_date, end_date, icbs_match) 
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

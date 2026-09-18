pub mod rec_structs;

pub mod auth_import;
pub mod ccg_import;
pub mod ccg_site_import;
pub mod csu_import;
pub mod csu_site_import;
pub mod care_trust_import;
pub mod care_trust_site_import;
pub mod hospice_import;
pub mod iom_org_import;
pub mod non_nhs_import;
pub mod supp_agencies_import;
pub mod exec_agencies_import;
pub mod gpmem_import;
pub mod pcn_import;
pub mod pcn_partner_import;
pub mod php_provider_import;
pub mod php_provider_site_import;
pub mod gp_import;
pub mod sha_import;
pub mod treat_centre_import;
pub mod ni_org_import;
pub mod ni_gp_in_lhscg_import;
pub mod ni_gp_import;
pub mod succession_import;
pub mod wlhb_import;
pub mod wlhb_site_import;


pub mod trust_import;
pub mod trust_site_import;


use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;


pub async fn import_data(data_folder: &PathBuf, pool: &Pool<Postgres>) -> Result<(), AppError> {

    let file_name = "eauth.csv";
    auth_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "eccg.csv";
    ccg_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "eccgsite.csv";
    ccg_site_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ecsu.csv";
    csu_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ecsusite.csv";
    csu_site_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ect.csv";
    care_trust_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ectsite.csv";
    care_trust_site_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ehospice.csv";
    hospice_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "eiom.csv";
    iom_org_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "enonnhs.csv";
    non_nhs_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ensa.csv";
    supp_agencies_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "eother.csv";
    exec_agencies_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "epcmem.csv";
    gpmem_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "epcn.csv";
    pcn_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "epcncorepartnerdetails.csv";
    pcn_partner_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ephp.csv";
    php_provider_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ephpsite.csv";
    php_provider_site_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "epraccur.csv";
    gp_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "espha.csv";
    sha_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "etr.csv";
    trust_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "etreat.csv";
    treat_centre_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ets.csv";
    trust_site_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "niorg.csv";
    ni_org_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "nlhscgpr.csv";
    ni_gp_in_lhscg_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "npraccur.csv";
    ni_gp_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "succ.csv";
    succession_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "wlhb.csv";
    wlhb_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "wlhbsite.csv";
    wlhb_site_import::import_data(data_folder, file_name, pool).await?;

    Ok(())
}

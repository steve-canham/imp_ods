//pub mod rec_structs;

pub mod auth;
pub mod ccg;
pub mod ccg_site;
pub mod csu;
pub mod csu_site;
pub mod care_trust;
pub mod care_trust_site;
pub mod hospice;
pub mod iom_org;
pub mod non_nhs;
pub mod supp_agencies;
pub mod exec_agencies;
pub mod gpmem;
pub mod pcn;
pub mod pcn_partner;
pub mod php_provider;
pub mod php_provider_site;
pub mod gp;
pub mod sha;
pub mod treat_centre;
pub mod ni_org;
pub mod ni_gp_in_lhscg;
pub mod ni_gp;
pub mod path;
pub mod succession;
pub mod wlhb;
pub mod wlhb_site;
pub mod trust;
pub mod trust_site;


use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;

pub async fn import_data(data_folder: &PathBuf, pool: &Pool<Postgres>) -> Result<(), AppError> {

    auth::import_data(data_folder, "eauth.csv", pool).await?;
    ccg::import_data(data_folder, "eccg.csv", pool).await?;
    ccg_site::import_data(data_folder, "eccgsite.csv", pool).await?;
    csu::import_data(data_folder, "ecsu.csv", pool).await?;
    csu_site::import_data(data_folder, "ecsusite.csv", pool).await?;
    path::import_data(data_folder, "eplab.csv", pool).await?;

    care_trust::import_data(data_folder, "ect.csv", pool).await?;
    care_trust_site::import_data(data_folder, "ectsite.csv", pool).await?;
    hospice::import_data(data_folder, "ehospice.csv", pool).await?;
    iom_org::import_data(data_folder, "eiom.csv", pool).await?;
    non_nhs::import_data(data_folder, "enonnhs.csv", pool).await?;
    supp_agencies::import_data(data_folder, "ensa.csv", pool).await?;

    exec_agencies::import_data(data_folder, "eother.csv", pool).await?;
    gpmem::import_data(data_folder, "epcmem.csv", pool).await?;
    pcn::import_data(data_folder, "epcn.csv", pool).await?;
    pcn_partner::import_data(data_folder, "epcncorepartnerdetails.csv", pool).await?;
    php_provider::import_data(data_folder, "ephp.csv", pool).await?;
    php_provider_site::import_data(data_folder, "ephpsite.csv", pool).await?;

    gp::import_data(data_folder, "epraccur.csv", pool).await?;
    sha::import_data(data_folder, "espha.csv", pool).await?;
    trust::import_data(data_folder, "etr.csv", pool).await?;
    treat_centre::import_data(data_folder, "etreat.csv", pool).await?;
    trust_site::import_data(data_folder, "ets.csv", pool).await?;
    ni_org::import_data(data_folder, "niorg.csv", pool).await?;

    ni_gp_in_lhscg::import_data(data_folder, "nlhscgpr.csv", pool).await?;
    ni_gp::import_data(data_folder, "npraccur.csv", pool).await?;
    succession::import_data(data_folder, "succ.csv", pool).await?;
    wlhb::import_data(data_folder, "wlhb.csv", pool).await?;
    wlhb_site::import_data(data_folder, "wlhbsite.csv", pool).await?;
    

    Ok(())
}

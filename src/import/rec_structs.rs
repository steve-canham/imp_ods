use chrono::NaiveDate;

#[derive(Debug)]
pub struct AuthRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CCGRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CCGSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub parent_org: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct CSURec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CSUSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub parent_org: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>, 
}


#[derive(Debug)]
pub struct CTRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CTSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
	pub parent_org: String,
}

#[derive(Debug)]
pub struct HospiceRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct IoMRec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct NonNHSRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct SARec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct EARec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub parent_org: String,
}

#[derive(Debug)]
pub struct LinkedGPRec {
	pub ods_code: String,
	pub parent_org: String,
	pub parent_org_type: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>,
}


#[derive(Debug)]
pub struct PCNRec {
	pub ods_code: String,
	pub ods_name: String,
	pub subicb_loc: String,
	pub subicb_name: String,
    pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
}

#[derive(Debug)]
pub struct PCNPartnerRec {
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

#[derive(Debug)]
pub struct PHPRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct PHPSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
	pub parent_org: String,
}

#[derive(Debug)]
pub struct GPRec {
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

#[derive(Debug)]
pub struct SHARec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
}


#[derive(Debug)]
pub struct TrustRec {
    pub ods_code : String,
    pub ods_name: String,
    pub grouping: String,
    pub health_geog : String,
    pub city: String,
    pub postcode: String,
    pub postal_add: String,
    pub open_date: Option<NaiveDate>,
    pub close_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct TreatRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct TrustSiteRec {
    pub ods_code : String,
    pub ods_name: String,
    pub grouping: String,
    pub health_geog : String,
    pub city: String,
    pub postcode: String,
    pub postal_add: String,
    pub open_date: Option<NaiveDate>,
    pub close_date: Option<NaiveDate>, 
    pub subtype_code: String,
    pub parent_org: String,
}

#[derive(Debug)]
pub struct NIOrgRec {
	pub ods_code : String,
	pub ods_name: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
    pub status: String,
	pub parent_org: String,
}

#[derive(Debug)]
pub struct LinkedNIGPRec {
	pub ods_code: String,
	pub parent_org: String,
	pub parent_org_type: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>,
}

#[derive(Debug)]
pub struct NIGPRec {
	pub ods_code : String,
	pub ods_name: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
	pub status: String,
}

#[derive(Debug)]
pub struct SuccRec {
	pub ods_code: String,
	pub succ_ods_code: String,
	pub succ_reason_code: String,
	pub effective_date:  Option<NaiveDate>, 
	pub succession_indicator: String,
}

#[derive(Debug)]
pub struct WOrgRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct WSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
}

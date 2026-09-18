
pub fn get_ods_sql_1<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    create schema if not exists src;
    
    drop table if exists ods.auths;
    create table ods.auths (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.ccgs;
    create table ods.ccgs (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.ccg_sites;
    create table ods.ccg_sites (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , parent_org           varchar null
      , join_parent_date     date    null
      , left_parent_date     date    null
    );

    drop table if exists ods.csus;
    create table ods.csus (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.csu_sites;
    create table ods.csu_sites (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , parent_org           varchar null
      , join_parent_date     date    null
      , left_parent_date     date    null
    );

    drop table if exists ods.care_trusts;
    create table ods.care_trusts (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.care_trust_sites;
    create table ods.care_trust_sites (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
      , parent_org           varchar null
    );

    drop table if exists ods.hospices;
    create table ods.hospices (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.iom_orgs;
    create table ods.iom_orgs (
        ods_code varchar not null primary key
      , ods_name varchar not null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
    );

    drop table if exists ods.non_nhs;
    create table ods.non_nhs (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );
    
    SET client_min_messages TO NOTICE;"#
}

pub fn get_ods_sql_2<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
        
    drop table if exists ods.supp_agencies;
    create table ods.supp_agencies (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
    );

    drop table if exists ods.exec_agencies;
    create table ods.exec_agencies (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , parent_org           varchar null
    );

    drop table if exists ods.gpmem;
    create table ods.gpmem (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , parent_org_type      varchar null
      , join_parent_date     date    null
      , left_parent_date     date    null
    );

    drop table if exists ods.pcns;
    create table ods.pcns (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , subicb_loc           varchar null
      , subicb_name          varchar null
      , open_date            date    null
      , close_date           date    null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
    );

    drop table if exists ods.pcn_partners;
    create table ods.pcn_partners (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , parent_subicb_loc    varchar null
      , parent_subicb_name   varchar null
      , pcn_code             varchar null
      , pcn_name             varchar null
      , pcn_par_subicb_loc   varchar null
      , pcn_par_subicb_name  varchar null
      , start_date           date    null
      , end_date             date    null
      , icbs_match           bool    null
    );

    drop table if exists ods.php_providers;
    create table ods.php_providers (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
    );

    drop table if exists ods.php_provider_sites;
    create table ods.php_provider_sites (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
      , parent_org           varchar null
    );

    drop table if exists ods.gps;
    create table ods.gps (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , status               varchar null
      , subtype_code         varchar null
      , commissioner         varchar null
      , join_parent_date     date    null
      , left_parent_date     date    null
      , provpurch            varchar null
      , prescribing_setting  varchar null
    );

    drop table if exists ods.shas;
    create table ods.shas (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.trusts;
    create table ods.trusts (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
    );

    SET client_min_messages TO NOTICE;"#
}

pub fn get_ods_sql_3<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    
    drop table if exists ods.treat_centres;
    create table ods.treat_centres (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
    );

    drop table if exists ods.trust_sites;
    create table ods.trust_sites (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
      , parent_org           varchar null
    );

    drop table if exists ods.ni_orgs;
    create table ods.ni_orgs (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
      , status               varchar null
      , parent_org           varchar null
    );

    drop table if exists ods.ni_gps_in_lhscg;
    create table ods.ni_gps_in_lhscg (
        ods_code             varchar not null
      , parent_org           varchar not null
      , parent_org_type      varchar null
      , join_parent_date     date    null
      , left_parent_date     date    null
    );

    drop table if exists ods.ni_gps;
    create table ods.ni_gps (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
      , subtype_code         varchar null
      , status               varchar null
    );

    drop table if exists ods.successions;
    create table ods. successions (
        ods_code             varchar not null 
      , succ_ods_code        varchar null
      , succ_reason_code     varchar null
      , effective_date       date    null
      , succession_indicator varchar null
    );

    drop table if exists ods.wlhbs;
    create table ods. wlhbs (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
    );

    drop table if exists ods.wlhb_sites;
    create table ods.wlhb_sites (
        ods_code             varchar not null primary key
      , ods_name             varchar not null
      , grouping             varchar null
      , health_geog          varchar null
      , city                 varchar null
      , postcode             varchar null
      , postal_add           varchar null
      , open_date            date    null
      , close_date           date    null
    );
    
    SET client_min_messages TO NOTICE;"#
}
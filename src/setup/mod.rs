pub mod config_reader;
pub mod log_helper;
pub mod cli_reader;
pub mod db_pars;

use crate::sql::create_ods_tables;
use crate::err::AppError;
use std::ffi::OsString;
use directories::ProjectDirs;
use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use cli_reader::{CliPars, Flags};
use std::fs;
use config_reader::Config;
use std::sync::OnceLock;

pub struct InitParams {
    pub data_folder: PathBuf,
    pub log_folder: PathBuf,
    pub flags: Flags,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_command_line_args(args: Vec<OsString>) -> Result<CliPars, AppError> {
    
    // CLI parameters collected first, in this instance mainly to conform with other systems.
    
    cli_reader::fetch_valid_arguments(args)
}

pub fn get_config_file_args()-> Result<Config, AppError> {
    
    // The config data is then processed to create a Config object, which includes 
    // database connection parameters, and parent folders for logs and source data.

    let config_path = obtain_config_file_path()?;  // The OS dependent location of the config file.

    let config_string = fs::read_to_string(&config_path)
            .map_err(|e| AppError::IoReadErrorWithPath(e, config_path.to_owned()))?;

    config_reader::populate_config_vars(&config_string)
}

pub fn combine_args(cli_pars: CliPars, config: Config) -> Result<InitParams, AppError> {

    // CLI and config data are then combined into a single InitParams struct - the CLI's 
    // source file parameters will overrule any in the config file.

    let folder_pars = config.folders;  // guaranteed to exist

    let data_folder =  folder_pars.data_folder_path;
    if !folder_exists (&data_folder) && (cli_pars.flags.import_ods || cli_pars.flags.import_trusts)
    {   
        return Result::Err(AppError::MissingProgramParameter("data_folder".to_string()));
    }

    let log_folder = folder_pars.log_folder_path;
    if !folder_exists (&log_folder) {
        fs::create_dir_all(&log_folder)?;
    }

    // For flags read from the CLI variables
    
    Ok(InitParams {
        data_folder,
        log_folder,
        flags: cli_pars.flags,
    })

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    match folder_name.try_exists() {
        Ok(true) => true,
        _ => false,   // includes Ok(false) as well as Err
    }
}

pub fn establish_log(params: &InitParams) -> Result<(), AppError> {

    if !log_set_up() {  // may be called more than once in context of integration tests
        log_helper::setup_log(&params.log_folder)?;
        LOG_RUNNING.set(true).unwrap(); // should always work
        log_helper::log_startup_params(&params);
    }
    Ok(())
}

pub fn log_set_up() -> bool {
    match LOG_RUNNING.get() {
        Some(_) => true,
        None => false,
    }
}


pub async fn create_ods_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = create_ods_tables::get_ods_sql_1();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = create_ods_tables::get_ods_sql_2();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = create_ods_tables::get_ods_sql_3();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


fn obtain_config_file_path() -> Result<PathBuf, AppError> {

     if let Some(config) = ProjectDirs::from("eu", "canhamis", "imp_ods") {
         let config_folder = config.config_dir().to_path_buf();
         let file_name = "config.toml";
         Ok(config_folder.join(file_name))
 
         // Linux:   /home/<user name>/.config/imp_ods/config.toml
         // Windows: C:\Users\<user name>\AppData\Roaming\canhamis\imp_ods\config.toml
         // macOS:   /Users/<user name>/Library/Application Support/eu.canhamis.imp_ods/config.toml

     }   
     else {
         println!("Odd! - Unable to identify an OS-specific location for the configuration file");
         Err(AppError::ConfigurationError(
             "No folder for config file found".to_string(), 
             "Fatal error - unable to proceed".to_string()))
     }
}



// Tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn check_config_vars_read_correctly() {

        let config = r#"
[folders]
data_folder_path="/home/steve/Data/Resources - Data/NHS Org Data/ODS Source data/2025 October/"
log_folder_path="/home/steve/Data/MDR logs/nhs/"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
"#;
        let args : Vec<&str> = vec!["dummy target"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let cli = get_command_line_args(test_args).unwrap();
        let config_string = config.to_string();
        let config = config_reader::populate_config_vars(&config_string).unwrap();
        let res = combine_args(cli, config).unwrap();
        
        assert_eq!(res.flags.import_ods, true);
        assert_eq!(res.flags.process_ods, false);
        assert_eq!(res.data_folder, PathBuf::from("/home/steve/Data/Resources - Data/NHS Org Data/ODS Source data/2025 October/"));
        assert_eq!(res.log_folder, PathBuf::from("/home/steve/Data/MDR logs/nhs/"));
    }
   
    
    #[test]
    #[should_panic]
    fn check_wrong_data_folder_panics() {

        let config = r#"
[folders]
data_folder_path="/home/steve/Data/MDR source data/NHS Org Data/2025/"
log_folder_path="/home/steve/Data/MDR logs/nhs/"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();
        
        let args : Vec<&str> = vec!["dummy target", "-r"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();
        let config_string = config.to_string();
        let config = config_reader::populate_config_vars(&config_string).unwrap();
        let _res = combine_args(cli_pars, config).unwrap();
    }
}


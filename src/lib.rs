
pub mod setup;
pub mod err;
mod import;
mod sql;
mod utils;

use err::AppError;
use std::ffi::OsString;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let cli = setup::get_command_line_args(args)?;
    let config = setup::get_config_file_args()?;
    let params = setup::combine_args(cli, config)?;
        
    setup::establish_log(&params)?;
    let pool = setup::db_pars::get_db_pool().await?;

    let flags = params.flags;
    
    if flags.import_ods   
    {
        // recreate the tables
        setup::create_ods_tables(&pool).await?;

        // Import the data
        import::import_data(&params.data_folder, &pool).await?;
    }

    if flags.process_ods   
    {
        // process the imported ods data
        // TO DO
    }

    if flags.import_hosps   
    {
        // import the hospitals data
        // TO DO
    }

    if flags.import_trusts
    {
        // import the trusts data
        // TO DO
    }

    Ok(())  
}

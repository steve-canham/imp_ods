// Module uses clap crate to read command line arguments. 

use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;
 
pub struct CliPars {
     pub flags: Flags, 
}
 
#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub import_ods: bool,
    pub process_ods: bool,
    pub import_hosps: bool,
    pub import_trusts: bool,
}
 
 pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
 { 
     let parse_result = parse_args(args)?;
  
     // Flag values are false if not present, true if present.
 
     let mut i_flag = parse_result.get_flag("i_flag");
     let p_flag = parse_result.get_flag("p_flag");
     let s_flag = parse_result.get_flag("s_flag");
     let t_flag = parse_result.get_flag("t_flag");

     if !i_flag && !p_flag && !s_flag && !t_flag{
         i_flag = true;  // import ODS data is the default
     }
 
     let flags = Flags {
         import_ods: i_flag,
         process_ods: p_flag,
         import_hosps: s_flag,
         import_trusts: t_flag,
     };
 
     Ok(CliPars {
         flags: flags,
     })
 }
 
 
 fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {
 
     command!()
         .about("Imports data from csv filesand imports it into a database")
         .arg(
             Arg::new("i_flag")
            .short('i')
            .long("import-ods")
            .required(false)
            .help("A flag signifying import from ods files to ods schema tables")
            .action(clap::ArgAction::SetTrue)
         )
        .arg(
             Arg::new("p_flag")
             .short('p')
             .long("process-ods")
             .required(false)
             .help("A flag signifying process ods data")
             .action(clap::ArgAction::SetTrue)
        )
        .arg(
             Arg::new("s_flag")
             .short('s')
             .long("import-hosp")
             .required(false)
             .help("A flag signifying import hosp data")
             .action(clap::ArgAction::SetTrue)
        )
        .arg(
             Arg::new("t_flag")
             .short('t')
             .long("import-trust")
             .required(false)
             .help("A flag signifying import trust data")
             .action(clap::ArgAction::SetTrue)
        )
     .try_get_matches_from(args)
 
 }
 
 
 #[cfg(test)]
 mod tests {
     use super::*;
     
     // Ensure the parameters are being correctly extracted from the CLI arguments
 
     #[test]
     fn check_cli_no_explicit_params() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_ods, true);
         assert_eq!(res.flags.process_ods, false);
         assert_eq!(res.flags.import_hosps, false);
         assert_eq!(res.flags.import_trusts, false);
     }
 
     #[test]
     fn check_cli_with_i_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-i"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_ods, true);
         assert_eq!(res.flags.process_ods, false);
         assert_eq!(res.flags.import_hosps, false);
         assert_eq!(res.flags.import_trusts, false);
     }
 
 
     #[test]
     fn check_cli_with_s_flags() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-s"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_ods, false);
         assert_eq!(res.flags.process_ods, false);
         assert_eq!(res.flags.import_hosps, true);
         assert_eq!(res.flags.import_trusts, false);
     }
      
    
     #[test]
     fn check_cli_with_most_params_explicit() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-i", "-p", "-s", "-t"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_ods, true);
         assert_eq!(res.flags.process_ods, true);
         assert_eq!(res.flags.import_hosps, true);
         assert_eq!(res.flags.import_trusts, true);
     }
 
 }
 
 
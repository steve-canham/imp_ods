use std::char;
use chrono::NaiveDate;
use regex::{Captures, Regex};


pub fn convert_to_date(text: &str) -> Option<NaiveDate> {

    match NaiveDate::parse_from_str(text, "%Y%m%d")
        {
            Ok(d) => Some(d),
            Err(_) => None,
        }
}


pub fn capitalise_field(text: &str) -> String {
    
   // Preliminary corrections.
   // Ensure &s and brackets are properly spaced.

   let mut arg_text = text.replace("'","’");
   arg_text = arg_text.replace(".","");

   if arg_text.contains('&') {
       arg_text = repair_ampersands(&arg_text);
   }

   if arg_text.contains('(') {
       arg_text = repair_brackets(&arg_text);
   }
   
   // Put in lower case and remove hyphens from some names.

   let mut lower = arg_text.to_string().to_lowercase();
   
   if lower.contains('-') {
        lower = lower.replace("-on-", " on ");
        lower = lower.replace("-under-", " under ");
        lower = lower.replace("-in-", " in ");
        lower = lower.replace("-super-", " super ");
        lower = lower.replace("-upon-", " upon");
        lower = lower.replace("-by-the-", " by the ");
   }

   if lower.contains('-') {
       lower = repair_hyphens(&lower);
   }
   
   // Split lower case string into separate words 
   // Consider each word in turn

   let parts: Vec<_> = lower
      .split(|c: char| c.is_ascii_whitespace())
      .filter(|p| !p.is_empty())
      .collect();
   
   let mut new_text = "".to_string();

   for w in parts {
        
        if w.starts_with('(') {
            new_text = new_text + " " + &w;  // add bracketed text 'as is'
            continue;
        }
        else {
            
            new_text = new_text + " " + &capitalise_word(w)
        }
    }
        
    new_text = new_text.trim().to_string();

    if new_text.contains('(') {
       
       let re = Regex::new(r"\((?<content>[^)]+)\)").unwrap();
       new_text = re.replace_all(&new_text, |caps: &Captures| {
            format!("({})", capitalise_field(&caps["content"].replace("-", " ")))
        }).to_string();
    }

    new_text = new_text.replace(" - Y - ", "-y-");
    new_text = new_text.replace(" Y ", " y ");
    new_text = new_text.replace("A & E ", "A&E ");
    new_text = new_text.replace("O Cliff", "O’Cliff");

    new_text
    
}


pub fn capitalise_site_name(text: &str) -> String {

    let mut new_name = capitalise_field(text);

    new_name = new_name.replace("E Pact", "ePact");
    new_name = new_name.replace("Y AMH ", "YAMH ");
    new_name = new_name.replace("IST Floor", "1st Floor");
    new_name = new_name.replace(" TO A ", " to a ");
    new_name = new_name.replace(" C S U", " CSU");
    new_name = new_name.replace("in - Reach", "Inreach");
    new_name = new_name.replace("D - MHSOP", "DMHSOP");

    new_name
}


pub fn get_postal_address(a1: &str, a2: &str, a3: &str, city: &str, pcode: &str) -> (String, String) {

    let cap_city = capitalise_field(city);

    let a = capitalise_field(a1);
    let b = capitalise_field(a2);
    let c = capitalise_field(a3);

    let b = if b == "" {""} else {&(", ".to_string() + &b)};
    let c = if c == "" {""} else {&(", ".to_string() + &c)};
    let d = if cap_city == "" {""} else {&(", ".to_string() + &cap_city)};
    let e = if pcode == "" {""} else {&(", ".to_string() + pcode)};
    let postal_address = a + b + c + d + e;
    
    (cap_city, postal_address)
}


fn repair_ampersands(text: &str) -> String {

    let mut new_text = text.to_string(); 
    
    // find position of ampersand - if no space before add one

    let re = Regex::new(r"\S&").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace("&", " &");
    }

    let re = Regex::new(r"&\S").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace("&", "& ");
    }

    new_text
}


fn repair_brackets(text: &str) -> String {

    let mut new_text = text.to_string(); 
    
    // Left bracket - if no space before add one

    let re = Regex::new(r"\S\(").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace("(", " (");
    }

     // Right bracket - if no space after add one

    let re = Regex::new(r"\)\S").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace(")", ") ");
    }
        
    new_text
}


fn repair_hyphens(text: &str) -> String {

    let mut new_text = text.to_string(); 
    
    // If no space before add one

    let re = Regex::new(r"\S-").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace("-", " -");
    }

    // If no space after add one

    let re = Regex::new(r"-\S").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace("-", "- ");
    }

    // but some are extraneous matches and the spacing needs to be removed again

    new_text = new_text.replace(" - a ", "-A ");
    new_text = new_text.replace(" - b ", "-B ");
    if new_text.ends_with(" - a") {new_text = new_text.replace(" - a", "-A");}
    if new_text.ends_with(" - b") {new_text = new_text.replace(" - b", "-B");}

    new_text = new_text.replace(" - amh", "amh");
    new_text = new_text.replace(" - camhs", "camhs");
    new_text = new_text.replace(" - ca", "ca");

    new_text = new_text.replace("non - ", "non-");
    new_text = new_text.replace("pre - ", "pre-");

    // hyphens between numbers should be closed up again...

    let re = Regex::new(r"(?<numhyph>[0-9] - [0-9])").unwrap();
    new_text = re.replace_all(&new_text, 
        |caps: &Captures| {caps["numhyph"].replace(" - ", "-")}).to_string();

   new_text
}


fn capitalise_word(w: &str) -> String {

    let mut c = w.chars();      // turn word into a vector of characters
    let mut wcap = match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };

    if wcap.len() == 2 {
            wcap = check_2_letter_word(&wcap);
    }
    else if wcap.len() == 3 {
        wcap = check_3_letter_word(&wcap);
    }
    else if wcap.len() == 4 {
        wcap = check_4_letter_word(&wcap);
    }
    else if wcap.len() == 5 {
        wcap = check_5_letter_word(&wcap);
    }
    else if wcap.len() == 6 {
        wcap = check_6_letter_word(&wcap);
    }
    else if wcap.len() == 7 {
        wcap = check_7_letter_word(&wcap);
    }

    if wcap.contains("'") {
        wcap = wcap.replace("'", "’"); 
    }
        
    if (wcap.starts_with("O’") || wcap.starts_with("D’sou")) && wcap.chars().count() > 2 {
        let mut s = wcap.chars().collect::<Vec<char>>();
        s[2] = s[2].to_ascii_uppercase();
        wcap = s.iter().collect::<String>();
    }

    wcap

}


fn check_2_letter_word(wcap: &str) -> String {
    
    let short_word = wcap.to_uppercase();
    let short_word_slice = match short_word.as_str() {
                "AT" => "at",
                "BY" => "by",
                "DR" => "Dr",
                "DU" => "Du",   
                "IN" => "in",
                "LE" => "le",
                "NO" => "No",             
                "OF" => "of",
                "ON" => "on",
                "OZ" => "Oz",
                "ST" => "St",
                "TY" => "Ty",
                "YR" => "Yr",
                _ => &short_word
            };
    short_word_slice.to_string()
}


fn check_3_letter_word(wcap: &str) -> String {

    let short_word = wcap.to_uppercase();
    let mut short_word_slice = short_word.as_str();

    if short_word.starts_with(['1', '2', '3', '4', '5', '6']) {
        short_word_slice = match short_word.as_str() {
            "1ST" => "1st",
            "2ND" => "2nd",
            "3RD" => "3rd",
            "4TH" => "4th",
            "5TH" => "5th",
            "6TH" => "6th",
            _ => &short_word
        };
    }
    else if short_word.starts_with('A')
    {
        short_word_slice = match short_word.as_str() {
            "AND" => "and",
            "ANN" => "Ann",
            "ALL" => "All",
            "ASH" => "Ash",
            "ARK" => "Ark",
            "AMI" => "Ami",
            "AMY" => "Amy",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('B') {
        short_word_slice = match short_word.as_str() {
            "BAR" => "Bar",
            "BIG" => "Big",
            "BAY" => "Bay",
            "BEE" => "Bee",
            "BEN" => "Ben",
            "BOW" => "Bow",
            "BOX" => "Box",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('C') {
        short_word_slice = match short_word.as_str() {
            "CLU" => "Clu",
            "COX" => "Cox",
            "CTR" => "Ctr",
            "COW" => "Cow",
            "CWM" => "Cwm",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('D') {
        short_word_slice = match short_word.as_str() {
            "DAN" => "Dan",
            "DAY" => "Day",
            "DOL" => "Dol",
            "DON" => "Don",
            "DOT" => "Dot",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('E') {
        short_word_slice = match short_word.as_str() {
            "ELM" => "Elm",
            "EAR" => "Ear",
            "ELY" => "Ely",
            "ERW" => "Erw",
            "ESK" => "Esk",
            "EYE" => "Eye",
            "END" => "End",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('F') {
        short_word_slice = match short_word.as_str() {
            "FAI" => "Fai",
            "FIR" => "Fir",
            "FLR" => "Flr",
            "FOR" => "for",
            "FOX" => "Fox",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('G') {
        short_word_slice = match short_word.as_str() {
            "GAP" => "Gap",
            "GEN" => "Gen",
            "GET" => "Get",
            "GWY" => "Gwy",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with(['H', 'I']) {
        short_word_slice = match short_word.as_str() {
            "HAM" => "Ham",
            "HEN" => "Hen",
            "HEY" => "Hey",
            "HOB" => "Hob",
            "HUB" => "Hub",
            "HPL" => "Hpl",
            "IAN" => "Ian",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with(['L', 'M', 'N']) {
        short_word_slice = match short_word.as_str() {
            "LEA" => "Lea",
            "LEE" => "Lee",
            "LEY" => "Ley",
            "LON" => "Lon",
            "LOW" => "Low",
            "LTD" => "Ltd",
            "MAN" => "Man",
            "MED" => "Med",
            "MID" => "Mid",
            "MIN" => "Min",
            "MON" => "Mon",
            "MOR" => "Mor",
            "NEW" => "New",
            "NON" => "Non",
            "NOW" => "Now",
            "NUR" => "Nur",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with(['O', 'P']) {
        short_word_slice = match short_word.as_str() {
            "OBS" => "Obs",
            "OFF" => "Off",
            "OLD" => "Old",
            "OAK" => "Oak",
            "OUR" => "Our",
            "OUT" => "Out",
            "ONE" => "One",
            "OWN" => "Own",
            "PEN" => "Pen",
            "PRE" => "Pre",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('R') {
        short_word_slice = match short_word.as_str() {
            "RED" => "Red", 
            "RAY" => "Ray",
            "ROY" => "Roy",
            "ROM" => "Rom",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('S') {
        short_word_slice = match short_word.as_str() {
            "SAN" => "San",
            "SEA" => "Sea",
            "SIR" => "Sir",
            "SIX" => "Six",
            "SPA" => "Spa",
            "SUE" => "Sue",
            "SUB" => "Sub",
            "SWN" => "Swn",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with('T') {
        short_word_slice = match short_word.as_str() {
            "TAF" => "Taf",
            "TAN" => "Tan",
            "THE" => "The",
            "TIR" => "Tir",
            "TOR" => "Tor",
            "TOP" => "Top",
            "TYN" => "Tyn",
            _ => short_word_slice
        };
    }
    else if short_word.starts_with(['V', 'W', 'Y']) {
        short_word_slice = match short_word.as_str() {
            "VAN" => "Van",
            "VUE" => "Vue",
            "WAR" => "War",
            "WAT" => "Wat",
            "WAY" => "Way",
            "WYE" => "Wye",
            "WAX" => "Wax",
            "YEW" => "Yew", 
            _ => &short_word_slice
        };
    }

    short_word_slice.to_string()

}


fn check_4_letter_word(wcap: &str) -> String {
    
    let short_word = wcap.to_string();
    let mut short_word_slice = short_word.as_str();

    if wcap.starts_with('A') {
        short_word_slice = match wcap {
            "Adhd" => "ADHD",
            "Aecu" => "AECU",
            "Afrs" => "AFRS",
            _ => wcap
        };
    }
    if wcap.starts_with('B') {
        short_word_slice = match wcap {
            "Bcsp" => "BCSPO",
            "Bfwh" => "BFWH",
            "Bhft" => "BHFT",
            "Blmk" => "BLMK",
            "Bmec" => "BMEC",
            "Brid" => "BRID",
            _ => wcap
        };
    }
    else if wcap.starts_with('C')
    {
        short_word_slice = match wcap {
            "Camh" => "CAMH",
            "Ccgs" => "CCGS",
            "Ccht" => "CCHT",
            "Cdat" => "CDAT",
            "Ceds" => "CEDS",
            "Cert" => "CERT",
            "Crht" => "CRHT",
            "Chbt" => "CHBT",
            "Citt" => "CITT",
            "Cldt" => "CLDT",
            "Cmht" => "CMHT",
            "Cmit" => "CMIT",
            "Cnwl" => "CNWL",
            "Copd" => "COPD",
            "Cwmh" => "CWMH",
            "Cypd" => "CYPD",
            "Cyps" => "CYPS",
            _ => wcap
        };
    }
    else if wcap.starts_with('D')
    {
        short_word_slice = match wcap {
            "Daat" => "DAAT",
            "Damh" => "DAMH",
            "Dasa" => "DASA",
            "Ddtc" => "DDTC",
            "Dmrc" => "DMRC",
            _ => wcap
        };
    }
        else if wcap.starts_with('E')
    {
        short_word_slice = match wcap {
            "Efnp" => "EFNP",
            "Eip1" => "EIP1",
            "Eip2" => "EIP2",
            "Eip3" => "EIP3",
            "Elfs" => "ELFS",
            "Elft" => "ELFT",
            "Eltt" => "ELTT",
            "Enht" => "ENHT",
            "Enpt" => "ENPT",
            "Epma" => "EPMA",
            "Etct" => "ETCT",
            _ => wcap
        };
    }
        else if wcap.starts_with('F')
    {
        short_word_slice = match wcap {
            "Fp10" => "FP10",
            "Ftac" => "FTAC",
            "Fgpf" => "FGPF",
            _ => wcap
        };
    }
    else if wcap.starts_with(['G'])
    {
        short_word_slice = match wcap {
            "Gdpr" => "GDPR",
            "Gim1" => "GIM1",
            "Gmht" => "GMHT",
            "Gmsc" => "GMSC",
            "Gstt" => "GSTT",
            _ => wcap
        };
    }
    else if wcap.starts_with('H')
    {
        short_word_slice = match wcap {
            "Hbss" => "HBSS",
            "Hdft" => "HDFT",
            "Hlht" => "HLHT",
            "Hmls" => "HMLS",
            _ => wcap
        };
    }
    else if wcap.starts_with('I')
    {
        short_word_slice = match wcap {
            "Iapt" => "IAPT",
            "Icrt" => "ICRT",
            "Ipct" => "IPCT",
            _ => wcap
        };
    }
    else if wcap.starts_with('K')
    {
        short_word_slice = match wcap {
            "Kmes" => "KMES",
            _ => wcap
        };
    }
    else if wcap.starts_with('M')
    {
        short_word_slice = match wcap {
            "Mhlt" => "MHLT",
            "Miaa" => "MIAA",
            _ => wcap
        };
    }
    else if wcap.starts_with('N')
    {
        short_word_slice = match wcap {
            "Ncic" => "NCIC",
            "Necs" => "NECS",
            "Nhft" => "NHFT",
            "Nhnn" => "NHNN",
            "Nici" => "NICI",
            "Nifs" => "NIFS",
            "Nihr" => "NIHR",
            "Nlfs" => "NLFS",
            "Nmgh" => "NMGH",
            "Nmp1" => "NMP1",
            "Nmp2" => "NMP2",
            "Norf" => "NORF",
            "Ntbc" => "NTBC",
            "Ntei" => "NTEI",
            _ => wcap
        };
    }
    else if wcap.starts_with('O')
    {
        short_word_slice = match wcap {
            "Ohid" => "OHID",
            "Opmh" => "OPMH",
            _ => wcap
        };
    }
    else if wcap.starts_with('P')
    {
        short_word_slice = match wcap {
            "Pnsl" => "PNSL",
            _ => wcap
        };
    }
    else if wcap.starts_with('R')
    {
        short_word_slice = match wcap {
            "Rcht" => "RCHT",
            "Rhch" => "RHCH",
            "Rmch" => "RMCH",
            _ => wcap
        };
    }
    else if wcap.starts_with('S')
    {
        short_word_slice = match wcap {
            "Sais" => "SAIS",
            "Sdfs" => "SDFS",
            "Ssnc" => "SSNC",
            "Smht" => "SMHT",
            "Sney" => "SNEY",
            "Srft" => "SRFT",
            "Ssdu" => "SSDU",
            "Stct" => "STCT",
            _ => wcap
        };
    }
    else if wcap.starts_with('T')
    {
        short_word_slice = match wcap {
            "Tiaa" => "TIAA",
            "Tamh" => "TAMH",
            "Titm" => "TITM",
            _ => wcap
        };
    }
    else if wcap.starts_with('U')
    {
        short_word_slice = match wcap {
            "Uclh" => "UCLH",
            "Uhnd" => "UHND",
            "Ulht" => "ULHT",
            "Upon" => "upon",
            _ => wcap
        };
    }
    else if wcap.starts_with('W')
    {
        short_word_slice = match wcap {
            "Wecg" => "WECG",
            "Wtbc" => "WTBC",
            "Wtei" => "WTEI",
            _ => wcap
        };
    }
    else if wcap.starts_with('Y')
    {
        short_word_slice = match wcap {
           "Yamh" => "YAMH",
           "Ymca" => "YMCA",
            _ => wcap
        };
    }

    short_word_slice.to_string()
}


fn check_5_letter_word(wcap: &str) -> String {
    
    let short_word = wcap.to_string();
    let mut short_word_slice = short_word.as_str();

    if wcap.starts_with(['A', 'B']) {
        short_word_slice = match wcap {
            "Adash" => "ADASH",
            "Bnssg" => "BNSSG",
                _ => wcap
        };
    }
    if wcap.starts_with('C') {
        short_word_slice = match wcap {
            "Calds" => "CALDS",
            "Camhs" => "CAMHS",
            "Cofph" => "CPFPH",
            "Cowph" => "COWPH",  
            "Crhtt" => "CRHTT",
            "Ctaid" => "CTAID",
            "Ctpld" => "CTPLD",
            _ => wcap
        };
    }
    else if wcap.starts_with('D')
    {
        short_word_slice = match wcap {
            "Daart" => "DAART",
            "Dscro" => "DSCRO",
            "Dairs" => "DAIRS",
            "Driff" => "DRIFF",
            _ => wcap
        };
    }
    else if wcap.starts_with(['E', 'I'])
    {
        short_word_slice = match wcap {
            "Epact" => "ePact",
            "Idass" => "IDASS",
            "Icash" => "ICASH",
            "Icats" => "ICATS",
            "Isats" => "ISATS",
            _ => wcap
        };
    }
    else if wcap.starts_with(['M', 'N'])
    {
        short_word_slice = match wcap {
            "Mhcas" => "MHCAS",
            "Mhsop" => "MHSOP",
            "Ndamh" => "NDAMH",
            "Nelft" => "NELFT",
            "Nepts" => "NEPTS",
            "Nhsuk" => "NHSUK",
            _ => wcap
        };
    }
    else if wcap.starts_with(['S', 'L'])
    {
        short_word_slice = match wcap {
            "Smhpc" => "SMHPC",
            "Spfit" => "SPFIT",
            "Sprpk" => "SPRPK",
            "Suhft" => "SUHFT",
            "Lpfit" => "LPFIT",
            _ => wcap
        };
    }
    else if wcap.starts_with(['U', 'V'])
    {
        short_word_slice = match wcap {
            "Ukhsa" => "UKHSA",
            "Under" => "under",
            "Vhabc" => "VHABC",
            _ => wcap
        };
    }

    short_word_slice.to_string()
}


fn check_6_letter_word(wcap: &str) -> String {
       
    let short_word_slice = match wcap {
        "Cmht-a" => "CMHT-A",
        "Cmht-b" => "CMHT-B",
        "Cmht-c" => "CMHT-C",
        "Cypmhs" => "CYPMHS",
        "E-pact" => "ePact",
        "Opcmht" => "OPCMHT",
        "Ycamhs" => "YCAMHS",
        _ => wcap
    };

    short_word_slice.to_string()
}


fn check_7_letter_word(wcap: &str) -> String {
       
    let short_word_slice = match wcap {
        "Nmepfit" => "NMEPFIT",
        "Fp10hnc" =>"FP10HNC",
        "Sub-icb" =>"Sub-ICB",
        "Hst-can" =>"HST-CAN",
        _ => wcap
    };

    short_word_slice.to_string()
}
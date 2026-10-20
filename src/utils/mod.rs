use std::char;
use regex::{Captures, Regex};

pub fn capitalise_field(text: &str) -> String {
    
   // Remove periods, replace apostrophes with single quotes

   let mut arg_text = text.replace("'","’");
   arg_text = arg_text.replace(".","");

   // Ensure &s and brackets are properly spaced.
   
   if arg_text.contains('&') {
       arg_text = repair_ampersands(&arg_text);
   }

   if arg_text.contains('(') {
       arg_text = repair_brackets(&arg_text);
   }
   
   // Put field in lower case and remove hyphens from some names.
   // Try and ensure remaining hyphens are also regularly spaced

   let mut lower = arg_text.to_string().to_lowercase();
   
   if lower.contains('-') {
        lower = lower.replace("-on-", " on ");
        lower = lower.replace("-under-", " under ");
        lower = lower.replace("-in-", " in ");
        lower = lower.replace("-super-", " super ");
        lower = lower.replace("-upon-", " upon");
        lower = lower.replace("-by-the-", " by the ");
        lower = lower.replace("-le-", " le ");
   }

   if lower.contains('-') {
       lower = repair_hyphens(&lower);
   }
   
   // Split lower case string into separate words 

   let parts: Vec<_> = lower
      .split(|c: char| c.is_ascii_whitespace())
      .filter(|p| !p.is_empty())
      .collect();
   
   let mut new_text = "".to_string();

   // Then consider each word in turn
   
   for w in parts {
        
        if w.starts_with('(') {
            new_text = new_text + " " + &w;  // add bracketed text 'as is', i.e. lower case
        }
        else {
            new_text = new_text + " " + &capitalise_word(w)
        }
    }

    // Trim and consider field text again if it includes a '('.
    // This may indicate the first of several bracketed words / abbreviations.
    // Bracketed content is found by the regex, (which captures all 
    // characters not the ')' between brackets), and this is 
    // now capitalised in the same way as the field as a whole
    // Hyphens are replaced by spaces
        
    new_text = new_text.trim().to_string();

    if new_text.contains('(') {
       
       let re = Regex::new(r"\((?<content>[^)]+)\)").unwrap();
       new_text = re.replace_all(&new_text, |caps: &Captures| {
            format!("({})", capitalise_field(&caps["content"].replace("-", " ")))
        }).to_string();
    }

    //Some odd corrections still required

    new_text = new_text.replace(" - Y - ", "-y-");
    new_text = new_text.replace(" Y ", " y ");
    new_text = new_text.replace("A & E ", "A&E ");
    new_text = new_text.replace("O Cliff", "O’Cliff");

    new_text
    
}


pub fn capitalise_site_name(text: &str) -> String {

    let mut new_name = capitalise_field(text);

    // These corrections useful for site names

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
    let mut wcapped = match c.next() {     // Capitalise first character
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };

    //if wcapped.contains("'") {
    //    wcapped = wcapped.replace("'", "’"); 
    //}

    if wcapped.len() == 2 {
        wcapped = check_2_letter_word(&wcapped);
    }
    else if wcapped.len() == 3 {
        wcapped = check_3_letter_word(&wcapped);
    }
    else if wcapped.len() == 4 {
        wcapped = check_4_letter_word(&wcapped);
    }
    else if wcapped.len() == 5 {
        wcapped = check_5_letter_word(&wcapped);
    }
    else if wcapped.len() == 6 {
        wcapped = check_6_letter_word(&wcapped);
    }
    else if wcapped.len() == 7 {
        wcapped = check_7_letter_word(&wcapped);
    }
        
    if (wcapped.starts_with("O’") || wcapped.starts_with("Mc")
        || wcapped.starts_with("D’sou")
    ) && wcapped.chars().count() > 2 {
        let mut s = wcapped.chars().collect::<Vec<char>>();
        s[2] = s[2].to_ascii_uppercase();
        wcapped = s.iter().collect::<String>();
    }

    wcapped

}


fn check_2_letter_word(wcapped: &str) -> String {

    // Most 2 letter 'words' will be upper case rather than lower,
    // e.g. may be part of a name or abbreviation. 
    // But the ones below are real words
    // and would be in all lower or capitalised case.

    let short_word = wcapped.to_uppercase();
    let short_word_slice = match short_word.as_str() {
                "AN" => "an",
                "AT" => "at",
                "BY" => "by",
                "DR" => "Dr",
                "DE" => "de",
                "DU" => "Du", 
                "EN" => "en", 
                "GT" => "Gt", 
                "IN" => "in",
                "LE" => "le",
                "NO" => "No",             
                "OF" => "of",
                "ON" => "on",
                "OZ" => "Oz",
                "Pk" => "Park",
                "RD" => "Road",
                "ST" => "St",
                "TO" => "to",
                "TY" => "Ty",
                "UP" => "Up",
                "YR" => "Yr",
                _ => &short_word
            };
    short_word_slice.to_string()

    // In some cases 'le', 'en' or 'de' may start a name and will need to be recapitalised
}


fn check_3_letter_word(wcap: &str) -> String {

    // Most 3 letter 'words' will be upper case rather than lower,
    // e.g. usualy as part of an abbreviation. 
    // But the ones below are real words
    // and would be in all lower or capitalised case.
    
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
    else 
    {  
        let fl = short_word_slice.chars().next().unwrap();
        match fl {
            'A' => {
                short_word_slice = match short_word_slice {
                    "ACE" => "Ace",
                    "AGE" => "Age",
                    "AID" => "Aid",
                    "AIR" => "Air",
                    "AND" => "and",
                    "ANN" => "Ann",
                    "ALL" => "All",
                    "ASH" => "Ash",
                    "ARC" => "Arc",
                    "ARK" => "Ark",
                    "ARM" => "Arm",
                    "ART" => "Art",
                    "AMI" => "Ami",
                    "AMY" => "Amy",
                    _ => short_word_slice
                };
             },
            'B' => {
                short_word_slice = match short_word.as_str() {
                    "BAR" => "Bar",
                    "BAY" => "Bay",
                    "BEC" => "Bec",
                    "BED" => "Bed",
                    "BEE" => "Bee",
                    "BEN" => "Ben",
                    "BIG" => "Big",
                    "BIZ" => "Biz",
                    "BOB" => "Bob",
                    "BOG" => "Bog",
                    "BOW" => "Bow",
                    "BOX" => "Box",
                    "BUS" => "Bus",
                    _ => short_word_slice
                };
             },
            'C' => {
                short_word_slice = match short_word.as_str() {
                    "CAP" => "Cap",
                    "CAT" => "Cat",
                    "CAR" => "Car",
                    "CLU" => "Clu",
                    "COX" => "Cox",
                    "CTR" => "Ctr",
                    "COW" => "Cow",
                    "COY" => "Coy",
                    "CWM" => "Cwm",
                    _ => short_word_slice
                };
            },
            'D' => {
                short_word_slice = match short_word.as_str() {
                    "DAM" => "Dam",
                    "DAN" => "Dan",
                    "DAY" => "Day",
                    "DEE" => "Dee",
                    "DEN" => "Den",
                    "DOL" => "Dol",
                    "DON" => "Don",
                    "DOT" => "Dot",
                    "DRY" => "Dry",
                    _ => short_word_slice
                };
            },

            'E' => {
                short_word_slice = match short_word.as_str() {
                    "EAR" => "Ear",
                    "ELM" => "Elm",
                    "ELY" => "Ely",
                    "ENA" => "Ena",
                    "END" => "End",
                    "ERW" => "Erw",
                    "ESK" => "Esk",
                    "EST" => "Est",
                    "EYE" => "Eye",
                    _ => short_word_slice
                };
            },

            'F' => {
                short_word_slice = match short_word.as_str() {
                    "FAI" => "Fai",
                    "FAR" => "Far",
                    "FAX" => "Fax",
                    "FIR" => "Fir",
                    "FLR" => "Flr",
                    "FOR" => "for",
                    "FOX" => "Fox",
                    _ => short_word_slice
                };
            },

            'G' => {
                short_word_slice = match short_word.as_str() {
                    "GAP" => "Gap",
                    "GAY" => "Gay",
                    "GEN" => "Gen",
                    "GET" => "Get",
                    "GUT" => "Gut",
                    "GUY" => "Guy",
                    "GWY" => "Gwy",
                    "GYM" => "Gym",
                    _ => short_word_slice
                };
             },

            'H' => {
                short_word_slice = match short_word.as_str() {
                    "HAM" => "Ham",
                    "HAT" => "Hat",
                    "HEN" => "Hen",
                    "HEY" => "Hey",
                    "HIL" => "Hill",
                    "HOB" => "Hob",
                    "HPL" => "Hpl",
                    "HUB" => "Hub",
                    "HUT" => "Hut",
                    _ => short_word_slice
                };
             },
             
            'I' => {
                short_word_slice = match short_word.as_str() {
                    "IAN" => "Ian",
                    "ICE" => "Ice",
                    "IDA" => "Ida",
                    "IDE" => "Ide",
                    "IND" => "Ind",
                    "INN" => "Inn",
                    "IVY" => "Ivy",
                    _ => short_word_slice
                };
            },
            
            'J' => {
                short_word_slice = match short_word.as_str() {
                    "JON" => "Jon",
                    _ => short_word_slice
                };
            },

            'K' => {
                short_word_slice = match short_word.as_str() {
                    "KAO" => "Kao",
                    "KAY" => "Kay",
                    "KEN" => "Ken",
                    "KEW" => "Kew",
                    _ => short_word_slice
                };
            },

            'L' => {
                short_word_slice = match short_word.as_str() {
                    "LAW" => "Law",
                    "LEA" => "Lea",
                    "LEE" => "Lee",
                    "LEN" => "Len",
                    "LEY" => "Ley",
                    "LIZ" => "Liz",
                    "LOG" => "Log",
                    "LON" => "Lon",
                    "LOW" => "Low",
                    "LTD" => "Ltd",
                    "LYS" => "Lys",
                    _ => short_word_slice
                };      
            },

            'M' => {
                short_word_slice = match short_word.as_str() {
                    "MAN" => "Man",
                    "MAX" => "Max",
                    "MAY" => "May",
                    "MED" => "Med",
                    "MID" => "Mid",
                    "MIN" => "Min",
                    "MOA" => "Moa",
                    "MON" => "Mon",
                    "MOR" => "Mor",
                    _ => short_word_slice
                };      
             },
             
            'N' => {
                short_word_slice = match short_word.as_str() {
                    "NEW" => "New",
                    "NON" => "Non",
                    "NOW" => "Now",
                    "NUR" => "Nur",
                    _ => short_word_slice
                };      
             },
             
            'O' => {
                short_word_slice = match short_word.as_str() {
                    "OAK" => "Oak",
                    "OBS" => "Obs",
                    "OCK" => "Ock",
                    "OFF" => "Off",
                    "OLD" => "Old",
                    "OPS" => "Ops",
                    "OUR" => "Our",
                    "OUT" => "Out",
                    "ONE" => "One",
                    "OWN" => "Own",
                    _ => short_word_slice
                };
            },
            
            'P' => {
                short_word_slice = match short_word.as_str() {
                    "PAK" => "Pak",
                    "PAR" => "Par",
                    "PAY" => "Pay",
                    "PEN" => "Pen",
                    "PRE" => "Pre",
                    "PSY" => "Psy",
                    "PYE" => "Pye",
                    _ => short_word_slice
                };
            },

            'Q' => { },

            'R' => {
                short_word_slice = match short_word.as_str() {
                    "RAY" => "Ray", 
                    "REC" => "Rec",
                    "RED" => "Red", 
                    "ROY" => "Roy",
                    "ROM" => "Rom",
                    "ROE" => "Roe",
                    "ROW" => "Row",
                    "RYE" => "Rye",
                    _ => short_word_slice
                };
            },

            'S' => {
                short_word_slice = match short_word.as_str() {
                    "SAN" => "San",
                    "SEA" => "Sea",
                    "SIR" => "Sir",
                    "SIX" => "Six",
                    "SPA" => "Spa",
                    "STH" => "Sth",
                    "SUE" => "Sue",
                    "SUB" => "Sub",
                    "SUN" => "Sun",
                    "SWN" => "Swn",
                    _ => short_word_slice
                };
             },
             
            'T' => {
                short_word_slice = match short_word.as_str() {
                    "TAF" => "Taf",
                    "TAN" => "Tan",
                    "THE" => "The",
                    "TIN" => "Tin",
                    "TIR" => "Tir",
                    "TOM" => "Tom",
                    "TOR" => "Tor",
                    "TOP" => "Top",
                    "TWO" => "Two",
                    "TYE" => "Tye",
                    "TYN" => "Tyn",
                    _ => short_word_slice
                };
             },
             
            'U' => { },
            
            'V' => {
                short_word_slice = match short_word.as_str() {
                    "VAN" => "Van",
                    "VAZ" => "Vaz",
                    "VUE" => "Vue",
                    _ => &short_word_slice
                };
            },

            'W' => {
                short_word_slice = match short_word.as_str() {
                    "WAR" => "War",
                    "WAT" => "Wat",
                    "WAY" => "Way",
                    "WYE" => "Wye",
                    "WAX" => "Wax",
                    _ => &short_word_slice
                };
            },

            'X' => { },

            'Y' => {
                short_word_slice = match short_word.as_str() {
                    "YEW" => "Yew", 
                    _ => &short_word_slice
                };
            },

            'Z' => { },

            _ => { },
        }
    }

    /* 
     
    if short_word.starts_with('A')
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
    */

    /* 
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
            "PAY" => "Pay",
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
            "ROW" => "Row",
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
            "TYE" => "Tye",
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
    */

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
            "Cent" => "Centre",
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
            "Ctre" => "Centre",
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
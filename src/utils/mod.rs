mod word_maps;

use word_maps::*;
use std::char;
use regex::{Captures, Regex};
use std::sync::LazyLock;


pub fn capitalise_field(text: &str) -> String {
    
   // Remove periods, replace apostrophes with single quotes

   let mut arg_text = text.replace("'","’");
   arg_text = arg_text.replace(".","");

   // Ensure &s, any commas and brackets are properly spaced.
   
   if arg_text.contains('&') {
       arg_text = repair_ampersands(&arg_text);
   }

   if arg_text.contains(',') {
       arg_text = repair_commas(&arg_text);
   }

   if arg_text.contains('(') {
       arg_text = repair_brackets(&arg_text);
   }
  
   // Put field in lower case and remove hyphens from some names.

   let mut lower = arg_text.to_lowercase();
   
   if lower.contains('-') {
       lower = lower.replace("-by-the-", " by the ");
       lower = lower.replace("-on-the-", " on the ");
       lower = lower.replace("-en-le-", " en le ");
       lower = lower.replace("-by-", " by ");
       lower = lower.replace("-in-", " in ");      
       lower = lower.replace("-on-", " on ");
       lower = lower.replace("-le-", " le ");  
       lower = lower.replace("-de-", " de ");         
       lower = lower.replace("-super-", " super ");
       lower = lower.replace("-upon-", " upon ");
       lower = lower.replace("-under-", " under ");
       lower = lower.replace("-over-", " over ");
   }

   // Try and ensure remaining hyphens are also regularly spaced
   
   if lower.contains('-') {
       lower = repair_hyphens(&lower);
   }

   // Try and ensure remaining slashes are also regularly spaced

   if lower.contains('/') {
       lower = lower.replace("n/cle", "newcastle");
       lower = lower.replace("h/pool", "hartlepool");
       lower = lower.replace("m/brough", "middlesbrough");
       lower = lower.replace("h/pk", "health park");
   }

   if lower.contains('/') {
       lower = repair_slashes(&lower);
   }
      
   // Split lower case string into separate words 

   let parts: Vec<_> = lower
      .split(|c: char| c.is_ascii_whitespace())
      .filter(|p| !p.is_empty())
      .collect();
   
   let mut new_text = "".to_string();

   // Then consider each word in turn, create new string and trim
   
   for w in parts {
        
        if w.starts_with('(') {
            new_text = new_text + " " + &w;  // add bracketed text 'as is', i.e. lower case
        }
        else {
            new_text = new_text + " " + &capitalise_word(w)
        }
    }
    new_text = new_text.trim().to_string();
    
    // Consider field text again if it includes a '('.
    // This may indicate the first of several bracketed words / abbreviations.
    // Bracketed content is found by the regex, (which captures all 
    // characters not the ')' between brackets), and this is 
    // now capitalised in the same way as the field as a whole
    // Hyphens are replaced by spaces
    
    if new_text.contains('(') {
       
       let re = Regex::new(r"\((?<content>[^)]+)\)").unwrap();
       new_text = re.replace_all(&new_text, |caps: &Captures| {
            format!("({})", capitalise_field(&caps["content"].replace("-", " ")))
        }).to_string();
    }

    // Some odd corrections still required

    new_text = new_text.replace(" The ", " the ");
    new_text = new_text.replace(" ST,", " Street,");
    new_text = new_text.replace(" - Y - ", "-y-");
    new_text = new_text.replace(" - Yr- ", "-yr-");
    new_text = new_text.replace(" Y ", " y ");
    new_text = new_text.replace("A & E ", "A&E ");
    new_text = new_text.replace("O / P", "O/P");
    new_text = new_text.replace("P / C", "P/C");
    new_text = new_text.replace("C / O", "c/o");
    new_text = new_text.replace("Tri - Serv", "TriServ");
    new_text = new_text.replace("Tri Serv", "TriServ");
    new_text = new_text.replace("O Cliff", "O’Cliff");

    new_text
    
}


pub fn capitalise_site_name(text: &str) -> String {

    let mut new_name = capitalise_field(text);

    // These additional corrections useful for site names

    new_name = new_name.replace("E Pact", "ePact");
    new_name = new_name.replace("Y AMH ", "YAMH ");
    new_name = new_name.replace("IST Floor", "1st Floor");
    new_name = new_name.replace(" TO A ", " to a ");
    new_name = new_name.replace(" C S U", " CSU");
    new_name = new_name.replace("in - Reach", "Inreach");
    new_name = new_name.replace("D - MHSOP", "DMHSOP");
    new_name = new_name.replace("CAS - CP", "CAS-CP");
    new_name = new_name.replace("CAS - RP", "CAS-RP");

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

    static RE_A1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S&").unwrap());
    static RE_A2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"&\S").unwrap());
    let mut new_text = text.to_string(); 
    if RE_A1.is_match(text)    // If no space before add one
    {
        new_text = new_text.replace("&", " &");
    }
    if RE_A2.is_match(text)    // If no space after add one
    {
        new_text = new_text.replace("&", "& ");
    }
    new_text
}

fn repair_commas(text: &str) -> String {

    static RE_C1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r",\S").unwrap());
    let mut new_text = text.to_string(); 
    if RE_C1.is_match(text)    // If no space after add one
    {
        new_text = new_text.replace(",", ", ");
    }
    new_text
}


fn repair_brackets(text: &str) -> String {

    static RE_B1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S\(").unwrap());
    static RE_B2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\)\S").unwrap());
    let mut new_text = text.to_string(); 
    if RE_B1.is_match(text)     // If no space before left bracket add one
    {
        new_text = new_text.replace("(", " (");
    }
    if RE_B2.is_match(text)     // If no space after right bracket add one
    {
        new_text = new_text.replace(")", ") ");
    }
    new_text
}


fn repair_hyphens(text: &str) -> String {

    static RE_H1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S-").unwrap());
    static RE_H2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-\S").unwrap());
    static RE_HN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?<numhyph>[0-9] - [0-9])").unwrap());
    
    let mut new_text = text.to_string(); 
       
    // If no space before or after add one
    
    if RE_H1.is_match(text)
    {
        new_text = new_text.replace("-", " -");
    }
    if RE_H2.is_match(text)
    {
        new_text = new_text.replace("-", "- ");
    }

    // but some are extraneous matches and the spacing needs to be removed again

    new_text = new_text.replace(" - a ", "-A ");
    new_text = new_text.replace(" - b ", "-B ");
    if new_text.ends_with(" - a") {new_text = new_text.replace(" - a", "-A");}
    if new_text.ends_with(" - b") {new_text = new_text.replace(" - b", "-B");}

    new_text = new_text.replace(" - amh", "AMH");
    new_text = new_text.replace(" - camhs", "CAMHS");
    new_text = new_text.replace(" - ca", " CA");

    new_text = new_text.replace("Walk - in", "Walk-in");
    new_text = new_text.replace("CO - OP", "Co-op");
    new_text = new_text.replace("Mon - Ami", "Mon Ami");
    new_text = new_text.replace("non - ", "non-");
    new_text = new_text.replace("pre - ", "pre-");

    // hyphens between numbers should be closed up again...

    new_text = RE_HN.replace_all(&new_text, 
        |caps: &Captures| {caps["numhyph"].replace(" - ", "-")}).to_string();

   new_text
}


fn repair_slashes(text: &str) -> String {

    static RE_S1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S/").unwrap());
    static RE_S2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"/\S").unwrap());
    static RE_SN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?<numhyph>[0-9] / [0-9])").unwrap());
    
    let mut new_text = text.to_string(); 
       
    // If no space before or after add one
    
    if RE_S1.is_match(text)
    {
        new_text = new_text.replace("/", " /");
    }
    if RE_S2.is_match(text)
    {
        new_text = new_text.replace("/", "/ ");
    }
       
    // slashes between numbers should be closed up again and changed to hyphens...

    new_text = RE_SN.replace_all(&new_text, 
        |caps: &Captures| {caps["numhyph"].replace(" / ", "/")}).to_string();

    new_text
}


fn capitalise_word(w: &str) -> String {

    let char_length = w.chars().count();

    // The expectation is that - by default - small words, less than 
    // 4 characters, are in uppper case, while any 4 characters or 
    // longer have only the first letter capitalised. There are, however, many
    // exceptions to both rules, hence the use of mappings for each size word
    // to replace erroneous capitalisations with the correct one.
    
    let mut wcapped = if char_length < 4 {
        w.to_uppercase()  // Capitalise all characters
    }
    else {
        let mut c = w.chars();
        match c.next() {     // Capitalise first character
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    };

    if  char_length < 8 { 
        if char_length == 2 && let Some(s) = LETS2.get(&wcapped) {
            wcapped = s.to_string();
        }
        else if char_length == 3 && let Some(s) = LETS3.get(&wcapped) {
            wcapped = s.to_string();
        }
        else if char_length == 4 && let Some(s) = LETS4.get(&wcapped) {
            wcapped = s.to_string();
        }
        else if char_length == 5 && let Some(s) = LETS5.get(&wcapped) {
            wcapped = s.to_string();
        }
        else if char_length == 6 && let Some(s) = LETS6.get(&wcapped) {
            wcapped = s.to_string();
        }
        else if char_length == 7 && let Some(s) = LETS7.get(&wcapped) {
            wcapped = s.to_string();
        }
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

// Tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn check_phf_lets2_at() {
        let v = LETS2.get("AT").unwrap_or_else(|| &"AT").to_string();
        assert_eq!(v, "at");
    }

    #[test]
    fn check_phf_lets2_road() {
        let v = LETS2.get("RD").unwrap_or_else(|| &"RD").to_string();
        assert_eq!(v, "Road");
    }

    #[test]
    fn check_phf_lets2_zz() {
        let v = LETS2.get("ZZ").unwrap_or_else(|| &"ZZ").to_string();
        assert_eq!(v, "ZZ");
    }

    #[test]
    fn check_phf_lets2_at2() {
        let v = capitalise_word("AT");
        assert_eq!(v, "at");
    }

    #[test]
    fn check_phf_lets2_road2() {
        let v = capitalise_word("RD");
        assert_eq!(v, "Road");
    }

    #[test]
    fn check_phf_lets2_zz2() {
        let v = capitalise_word("ZZ");
        assert_eq!(v, "ZZ");
    }
}

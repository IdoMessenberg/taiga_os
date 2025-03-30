use std_alloc::vec::Vec;

pub fn parse(file: &[u8]) -> Result<Vec<TomlTable>,()> {
    let file_str = match core::str::from_utf8(file) {
        Ok(v) => v,
        Err(_) => return Err(())
    };
    let mut tables: Vec<TomlTable> = Vec::new();
    let mut current_table: TomlTable = TomlTable { name: "", values: Vec::new() };
    for line in file_str.lines() {
        let mut trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }
        trimmed_line = match trimmed_line.contains('#') {
            true => (&trimmed_line[0..trimmed_line.find('#').unwrap()]).trim(),
            false => trimmed_line
        };
        if line.starts_with('[') && line.ends_with(']') {
            if !current_table.name.is_empty() && !current_table.values.is_empty() {
                tables.push(current_table.clone());
            }
            current_table = TomlTable{name: &line[1..line.len()-1], values: Vec::new()};
        }
        else if let Some(p_line) = parse_line(trimmed_line){
            current_table.values.push(p_line);
        }
    }
    tables.push(current_table);
    Ok(tables)
}

#[derive(Clone)]
pub struct TomlTable<'a>{
    pub name: &'a str,
    pub values: Vec<(&'a str, TomlValue<'a>)>
}

#[derive(Clone)]
pub enum TomlValue<'a> {
    String(&'a str),
    Integer(u64),
    //Boolean(bool),
    Array(Vec<TomlValue<'a>>),
    //Table(Vec<(String, TomlValue)>)
}
impl<'a> TomlValue<'a> {
    pub fn as_string(&self) -> Option<&'a str> {
        match self {
            Self::String( v) => Some(v),
            _ => None
        }
    }
    pub fn as_integer(&self) -> Option<u64> {
        match self {
            Self::Integer( v) => Some(v.clone()),
            _ => None
        }
    }
    /*pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean( v) => Some(v.clone()),
            _ => None
        }
    }*/
    pub fn as_array(&self) -> Option<Vec<TomlValue>> {
        match self {
            Self::Array( v) => Some(v.clone()),
            _ => None
        }
    }
}

fn parse_line<'a>(line: &'a str) -> Option<(&'a str, TomlValue<'a>)>{
    let mut parts:Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2{
        return None;
    }
    parts[0] = parts[0].trim();
    parts[1] = parts[1].trim();
    let value: TomlValue = match parse_value(parts[1]){
        Some(v) => v,
        None => return None
    };

    Some((parts[0], value))
}

fn parse_value(value: &str) -> Option<TomlValue> {
    return if let Some(v) = parse_string(value) {
        Some(TomlValue::String(v))
    }else if let Some(v) = parse_integer(value) {
        Some(TomlValue::Integer(v))
    }/*else if let Some(v) = parse_boolean(value) {
        Some(TomlValue::Boolean(v))
    }*/
    else if let Some(v) = parse_array(value) {
        Some(TomlValue::Array(v))
    }else{
        return None;
    };
}

fn parse_string<'a>(value: &'a str) -> Option<&'a str> {
    if value.starts_with('"') && value.ends_with('"'){
        return Some(& value[1..value.len()-1])
    }
    None
}
 
fn parse_integer(value: &str) -> Option<u64> {
    let result: Result<u64, core::num::ParseIntError> = match value.starts_with("0x"){
        true => u64::from_str_radix(&value[2..value.len()], 16),
        false => value.parse()
    };
    match result {
        Ok(v) => Some(v),
        Err(_) => None
    }
}

/*fn parse_boolean(value: &str) -> Option<bool> {
    match value.to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None
    }
}*/

fn parse_array(value: &str) -> Option<Vec<TomlValue>>{
    let mut res: Vec<TomlValue> = Vec::new();
    if value.starts_with('[') && value.ends_with(']'){
        let items: Vec<&str> = value[1..value.len()-1].split(',').collect();
        for item in items {
            res.push(match parse_value(item.trim()) {
                Some(v) => v,
                None => return None
            });
        }
        return Some(res)
    }
    None
}
use std_alloc::vec::Vec;

#[derive(Default, Clone)]
pub struct File<'a>{
    pub graphics_theme: efi::ColourTheme,
    pub loader_paths: LoaderPaths<'a>
}

#[derive(Default, Clone)]
pub struct LoaderPaths<'a>{
    pub kernel_path: &'a str,
    pub font_path:  &'a str
}

pub fn parse_config_toml<'b>(file: &'b[u8]) -> Result<File<'b>, efi::Status>{
    //convert the u8 array to a string slice
    let file_str = match core::str::from_utf8(file) {
        Ok(str) => str,
        Err(_) => return Err(efi::Status::UnknownGlyph)
    };

    let mut tables: TomlTable = HashMap::new();
    let mut current_table_name = "";
    for line in file_str.lines(){
        let mut trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {continue;}
        trimmed_line = if trimmed_line.contains('#') {&trimmed_line[0..trimmed_line.find('#').unwrap()]}else{trimmed_line};
        if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
            current_table_name = &trimmed_line[1..trimmed_line.len()-1];
            tables.insert(current_table_name, Vec::new());
        }
        else {
            let parts: Vec<&str> = trimmed_line.split('=').map(|s| s.trim()).collect();
            if parts.len() != 2 {
                return Err(efi::Status::Unsupported)
            }
            let table = if let Some(t) = tables.get_mut(current_table_name) {t}
            else {
                return Err(efi::Status::NotFound);
            };
            match get_value_type(parts[1]) {
                TomlValue::Boolean(_) => table.push((parts[0], TomlValue::Boolean(parts[1] == "true"))),
                //TomlValue::UnsignedInteger(_) => tables.get_mut(current_table_name).unwrap().push((key, TomlValue::UnsignedInteger(value.trim_start_matches('-').parse::<usize>().unwrap()))),
                TomlValue::HexInteger(_) => {
                    table.push((parts[0], TomlValue::HexInteger(usize::from_str_radix(parts[1].trim_start_matches("0x"), 16).unwrap())))
                },
                TomlValue::String(_) => table.push((parts[0], TomlValue::String(&parts[1][1..parts[1].len()-1]))),
            }
        }
    }
    let mut file: File= File::default();

    if let Some(table) = tables.get("loader_paths") {
        for key_value in table {
            match key_value.0 {
                "kernel-path" => if let TomlValue::String(v) = key_value.1 {file.loader_paths.kernel_path = v},
                "font-path" => if let TomlValue::String(v) = key_value.1 {
                    file.loader_paths.font_path = v;
                } 
                _ => continue,
            }
        }

    }

    if let Some(table) = tables.get("graphics.theme") {
        for key_value in table {
            match key_value.0 {
                "dark-mode" => if let TomlValue::Boolean(v) = key_value.1 {file.graphics_theme.dark_mode = v;},
                "white" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.white = v as u32;}
                "black" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.black = v as u32;} 
                "red" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.red = v as u32;} 
                "green" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.green = v as u32;} 
                "blue" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.blue = v as u32;} 
                "yellow" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.yellow = v as u32;} 
                "orange" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.orange = v as u32;} 
                "purple" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.purple = v as u32;} 
                "gray" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.gray = v as u32;} 
                "dark-gray" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.dark_gray = v as u32;} 
                "light-red" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.light_red = v as u32;} 
                "light-green" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.light_green = v as u32;} 
                "light-blue" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.light_blue = v as u32;} 
                "light-yellow" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.light_yellow = v as u32;} 
                "light-orange" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.light_orange = v as u32;} 
                "light-purple" => if let TomlValue::HexInteger(v) = key_value.1 {file.graphics_theme.light_purple = v as u32;} 
                _ => continue,
            }
        }
    }

    Ok(file)
}


fn get_value_type(value: &str) -> TomlValue {
    //if value.parse::<usize>().is_ok(){
    //    TomlValue::UnsignedInteger(0)
    //}
    /*else*/ if value.starts_with("0x"){
        TomlValue::HexInteger(0)
    }
    else if value == "true" || value == "false" {
        TomlValue::Boolean(true)
    }
    else {
        TomlValue::String("")
    }
}


struct HashMap<K : Eq, T>(Vec<(K,T)>);

impl<K: Eq, T> HashMap<K, T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn insert(&mut self, key: K, value: T) {
        self.0.push((key, value));
    }

    fn get(&self, key: K) -> Option<&T> {
        for item in &self.0 {
            if item.0 == key {
                return Some(&item.1)
            }
        }
        None
    }
    
    fn get_mut(&mut self, key: K) -> Option<&mut T> {
        for item in &mut self.0 {
            if item.0 == key {
                return Some(&mut item.1)
            }
        }
        None
    }
}

enum TomlValue<'a> {
    String(&'a str), 
    //UnsignedInteger(usize),
    HexInteger(usize),
    Boolean(bool)
}

type TomlArgument<'a> = (&'a str, TomlValue<'a>);
type TomlTable<'a> = HashMap<&'a str, Vec<TomlArgument<'a>>>;
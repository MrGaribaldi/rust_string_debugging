use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Default)]
pub struct SosiStringObject<'a>{
    lines:Vec<&'a str>
}
impl<'a> SosiStringObject<'a>{
    pub fn new() -> SosiStringObject<'a>{
        SosiStringObject{
            lines: Vec::new()
        }
    }
}

fn get_sosi_string_objects(contents: &str) -> Vec<SosiStringObject> {
    let content_lines = contents.lines();
    let mut sosi_string_objects = Vec::new();

    let mut first:bool = true;
    let mut current_object= SosiStringObject::new();
    for l in content_lines{
        let mut chars = l.chars();
        if chars.next().unwrap() == '.' && chars.next().unwrap() != '.' {
            if !first {
                sosi_string_objects.push(current_object);
                current_object = SosiStringObject::new();
            }else{
                first = false;
            }

        }
        current_object.lines.push(l);
    }
    //need to subtract 1 since the counter updates and then exits.
    sosi_string_objects.push(current_object);

    println!("Parsing file, we found {} sosi-objects", sosi_string_objects.len());

    sosi_string_objects
}

fn get_string_without_bom(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut first_byte = vec![0u8, 1];

    let mut f = File::open(&filename)?;
    let mut contents: String = String::new();
    f.read_exact(&mut first_byte)?;
    if first_byte[0] > 127u8 {
        println!("BOM detected, seeking past it");
        //we have a BOM
        f.seek(SeekFrom::Start(3))?;
    } else {
        f.rewind()?;
    }

    f.read_to_string(&mut contents)?;

    Ok(contents)
}



fn main() {
    let input_file_path = "C:\\Users\\Erik W. Bjønnes\\Documents\\repos\\jobb\\faerder.sos";
    let contents = get_string_without_bom(input_file_path).unwrap();
    let sosi_string_objects = get_sosi_string_objects(&contents);

    println!("We found {} objects", sosi_string_objects.len());

}

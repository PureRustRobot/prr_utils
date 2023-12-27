use std::fs;
use yaml_rust::{Yaml, YamlLoader};

fn get_param(path:String, bin_name:&str)->Yaml
{
    let file = fs::read_to_string(path);
    let s = file.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();

    let doc = &docs[0];

    return doc["zenoh_params"][bin_name].clone()
}

pub fn get_str_param(path:String , bin_name:&str, key_name:&str, default:String)->String
{
    match get_param(path, bin_name)[key_name].as_str()
    {
        Some(value)=>{
            return value.to_string()
        },
        None=>{
            return default
        }
    }
}

pub fn get_f64_param(path:String , bin_name:&str, key_name:&str, default:f64)->f64
{
    match get_param(path, bin_name)[key_name].as_f64()
    {
        Some(value)=>{
            return value;
        },
        None =>{
            return default;
        }
    }
}

pub fn get_i64_param(path:String , bin_name:&str, key_name:&str, default:i64)->i64
{
    match get_param(path, bin_name)[key_name].as_i64()
    {
        Some(value)=>{
            return value;
        },
        None =>{
            return default;
        }
    }
}

pub fn get_bool_param(path:String , bin_name:&str, key_name:&str, default:bool)->bool
{
    match get_param(path, bin_name)[key_name].as_bool()
    {
        Some(value)=>{
            return value;
        },
        None =>{
            return default;
        }
    }
}
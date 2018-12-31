use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use xml::EventReader;
use xml::ParserConfig;
use xml::reader::XmlEvent;
use error::Error;

pub type PropertyListDict = HashMap<String, PropertyListValue>;

#[derive(Debug)]
pub enum PropertyListValue {
    Integer(i32),
    String(String),
    Date(String),
    Boolean(bool),
    Dict(PropertyListDict),
    Array(Vec<PropertyListValue>),
}

pub fn to_i32(value: &PropertyListValue) -> Option<i32> {
    match value {
        PropertyListValue::Integer(v) => Some(*v),
        _ => None
    }
}

pub fn to_string(value: &PropertyListValue) -> Option<String> {
    match value {
        PropertyListValue::String(text) => Some(text.clone()),
        PropertyListValue::Date(text) => Some(text.clone()),
        _ => None
    }
}

pub fn read_property_list<P: AsRef<Path>>(path: P) -> Result<PropertyListDict, Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut parser = ParserConfig::new()
        .trim_whitespace(true)
        .create_reader(file);

    read_dict(&mut parser)
}

fn read_dict<R: Read>(parser: &mut EventReader<R>) -> Result<PropertyListDict, Error> {
    let mut dict: PropertyListDict = HashMap::new();

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name.as_str() == "key" => {
                if let Some((key, value)) = parse_key_value_pair(parser)? {
                    dict.insert(key, value);
                }
            },
            Ok(XmlEvent::EndDocument) => break,
            Ok(XmlEvent::EndElement { ref name, .. }) if name.local_name.as_str() == "dict" => break,
            Ok(event) => {
                println!("unhandled event {:?}", event);
                // break;
            },
            Err(e) =>
                return Err(Error::Parse(e))
        }
    }

    Ok(dict)
}

fn read_array<R: Read>(parser: &mut EventReader<R>) -> Result<Vec<PropertyListValue>, Error> {
    let mut array: Vec<PropertyListValue> = Vec::new();

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name.as_str() == "dict" => {
                let dict = read_dict(parser)?;
                array.push(PropertyListValue::Dict(dict));
            },
            Ok(XmlEvent::EndDocument) => break,
            Ok(XmlEvent::EndElement { ref name, .. }) if name.local_name.as_str() == "array" => break,
            Ok(event) => {
                println!("unhandled event {:?}", event);
                // break;
            },
            Err(e) =>
                return Err(Error::Parse(e))
        }
    }

    Ok(array)
}

fn parse_key_value_pair<R: Read>(parser: &mut EventReader<R>) -> Result<Option<(String, PropertyListValue)>, Error> {
    let mut key: String = String::new();

    if let Ok(XmlEvent::Characters(text)) = parser.next() {
        key = text;
    }
    if let Ok(XmlEvent::EndElement { name, .. }) = parser.next() {
    }
    let result = parse_value(parser)?
        .map(|value| (key, value));

    Ok(result)
}

fn parse_value<R: Read>(parser: &mut EventReader<R>) -> Result<Option<PropertyListValue>, Error> {
    match parser.next() {
        Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_str() {
            "integer" => Ok(read_text(parser)
                .and_then(|text| i32::from_str_radix(&text, 10).ok())
                .map(|value| PropertyListValue::Integer(value))),
            "string" => Ok(read_text(parser).map(|text| PropertyListValue::String(text))),
            "date" => Ok(read_text(parser).map(|text| PropertyListValue::Date(text))),
            "true" => Ok(Some(PropertyListValue::Boolean(true))),
            "false" => Ok(Some(PropertyListValue::Boolean(false))),
            "dict" => {
                let dict = read_dict(parser)?;
                Ok(Some(PropertyListValue::Dict(dict)))
            },
            "array" => {
                let array = read_array(parser)?;
                Ok(Some(PropertyListValue::Array(array)))
            },
            k => {
                println!("unknown key {}", k);
                Ok(None)
            }
        },
        Err(e) => Err(Error::Parse(e)),
        _ => Ok(None)
    }
}

fn read_text<R: Read>(parser: &mut EventReader<R>) -> Option<String> {
    if let Ok(XmlEvent::Characters(text)) = parser.next() {
        Some(text)
    }else {
        println!("string element without characters");
        None
    }
}

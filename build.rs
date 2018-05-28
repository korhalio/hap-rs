extern crate uuid;
extern crate handlebars;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::{fs::File, io::Write, collections::HashMap};

use handlebars::{Handlebars, Helper, RenderContext, RenderError, Renderable};

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    #[serde(rename = "Categories")]
    pub categories: Vec<Category>,
    #[serde(rename = "Characteristics")]
    pub characteristics: Vec<Characteristic>,
    #[serde(rename = "Services")]
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Category")]
    pub number: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Characteristic {
    #[serde(rename = "UUID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<Constraints>,
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Properties")]
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Constraints {
    #[serde(rename = "ValidValues")]
    pub valid_values: Option<HashMap<String, String>>,
    #[serde(rename = "MaximumValue")]
    pub max_value: Option<serde_json::Value>,
    #[serde(rename = "MinimumValue")]
    pub min_value: Option<serde_json::Value>,
    #[serde(rename = "StepValue")]
    pub step_value: Option<serde_json::Value>,
    #[serde(rename = "MaximumLength")]
    pub max_len: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    #[serde(rename = "UUID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RequiredCharacteristics")]
    pub required_characteristics: Vec<String>,
    #[serde(rename = "OptionalCharacteristics")]
    pub optional_characteristics: Vec<String>,
}

fn if_eq_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let first = h.param(0).unwrap().value();
    let second = h.param(1).unwrap().value();
    let tmpl = if first == second { h.template() } else { h.inverse() };
    match tmpl {
        Some(ref t) => t.render(r, rc),
        None => Ok(()),
    }
}

fn trim_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let trim = s.replace(" ", "").replace(".", "_");
        try!(rc.writer.write(&trim.into_bytes()));
    }
    Ok(())
}

fn file_name_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let name = s.replace(" ", "_").replace(".", "_").to_lowercase();
        try!(rc.writer.write(&name.into_bytes()));
    }
    Ok(())
}

fn type_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => { try!(rc.writer.write(b"bool")); },
            "uint8" => { try!(rc.writer.write(b"u8")); },
            "uint16" => { try!(rc.writer.write(b"u16")); },
            "uint32" => { try!(rc.writer.write(b"u32")); },
            "uint64" => { try!(rc.writer.write(b"u64")); },
            "int" => { try!(rc.writer.write(b"i32")); },
            "int32" => { try!(rc.writer.write(b"i32")); },
            "float" => { try!(rc.writer.write(b"f32")); },
            "string" => { try!(rc.writer.write(b"String")); },
            "tlv8" => { try!(rc.writer.write(b"Vec<u8>")); },
            "data" => { try!(rc.writer.write(b"Vec<u8>")); },
            _ => { return Err(RenderError::new("Unknown Characteristic format")); },
        }
    }
    Ok(())
}

fn format_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => { try!(rc.writer.write(b"Format::Bool")); },
            "uint8" => { try!(rc.writer.write(b"Format::UInt8")); },
            "uint16" => { try!(rc.writer.write(b"Format::UInt16")); },
            "uint32" => { try!(rc.writer.write(b"Format::UInt32")); },
            "uint64" => { try!(rc.writer.write(b"Format::UInt64")); },
            "int" => { try!(rc.writer.write(b"Format::Int32")); },
            "int32" => { try!(rc.writer.write(b"Format::Int32")); },
            "float" => { try!(rc.writer.write(b"Format::Float")); },
            "string" => { try!(rc.writer.write(b"Format::String")); },
            "tlv8" => { try!(rc.writer.write(b"Format::Tlv8")); },
            "data" => { try!(rc.writer.write(b"Format::Data")); },
            _ => { return Err(RenderError::new("Unknown Characteristic format")); },
        }
    }
    Ok(())
}

fn unit_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "percentage" => { try!(rc.writer.write(b"Unit::Percentage")); },
            "arcdegrees" => { try!(rc.writer.write(b"Unit::ArcDegrees")); },
            "celsius" => { try!(rc.writer.write(b"Unit::Celsius")); },
            "lux" => { try!(rc.writer.write(b"Unit::Lux")); },
            "seconds" => { try!(rc.writer.write(b"Unit::Seconds")); },
            _ => { return Err(RenderError::new("Unknown Characteristic unit")); },
        }
    }
    Ok(())
}

fn uuid_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        try!(rc.writer.write(shorten_uuid(&s).as_bytes()));
    }
    Ok(())
}

fn valid_values_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_object().unwrap();
    let mut output = String::from("vec![\n");
    for (key, val) in param {
        output.push_str(&format!("\t\t\t{}, // {}\n", key, val));
    }
    output.push_str("\t\t]");
    try!(rc.writer.write(output.as_bytes()));
    Ok(())
}

fn perms_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let params = h.param(0).unwrap().value().as_array().unwrap();
    for param in params {
        match param.as_str() {
            Some("read") => { try!(rc.writer.write(b"\n\t\t\tPerm::PairedRead,")); },
            Some("write") => { try!(rc.writer.write(b"\n\t\t\tPerm::PairedWrite,")); },
            Some("cnotify") => { try!(rc.writer.write(b"\n\t\t\tPerm::Events,")); },
            _ => {},
        }
    }
    Ok(())
}

fn float_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let format = h.param(0).unwrap().value().as_str().unwrap();
    if format == "float" {
        try!(rc.writer.write(b" as f32"));
    }
    Ok(())
}

fn shorten_uuid(id: &str) -> String {
    id.split("-").collect::<Vec<&str>>()[0].trim_left_matches('0').to_owned()
}

fn characteristic_name_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let id = h.param(0).unwrap().value().as_str().unwrap();
    let characteristics: Vec<Characteristic> = serde_json::from_value(h.param(1).unwrap().value().clone()).unwrap();
    for c in characteristics {
        if &c.id == id {
            let name = c.name.replace(" ", "").replace(".", "_");
            try!(rc.writer.write(name.as_bytes()));
        }
    }
    Ok(())
}

fn characteristic_file_name_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let id = h.param(0).unwrap().value().as_str().unwrap();
    let characteristics: Vec<Characteristic> = serde_json::from_value(h.param(1).unwrap().value().clone()).unwrap();
    for c in characteristics {
        if &c.id == id {
            let name = c.name.replace(" ", "_").replace(".", "_").to_lowercase();
            try!(rc.writer.write(name.as_bytes()));
        }
    }
    Ok(())
}

static CATEGORIES: &'static str = "// THIS FILE IS AUTO-GENERATED\n
#[derive(Copy, Clone)]
pub enum Category {
{{#each Categories as |c|}}\
\t{{trim c.Name}} = {{c.Category}},
{{/each}}\
}
";

static CHARACTERISTIC: &'static str = "// THIS FILE IS AUTO-GENERATED\n
use characteristic::{Characteristic, Format, Perm{{#if characteristic.Unit}}, Unit{{/if}}};

pub type {{trim characteristic.Name}} = Characteristic<{{type characteristic.Format}}>;

pub fn new() -> {{trim characteristic.Name}} {
    {{trim characteristic.Name}} {
        hap_type: \"{{uuid characteristic.UUID}}\".into(),
        format: {{format characteristic.Format}},
        perms: vec![{{perms characteristic.Properties}}
        ],\
        {{#if characteristic.Unit}}\n\t\tunit: Some({{unit characteristic.Unit}}),{{/if}}\
        {{#if characteristic.Constraints.MaximumValue}}\n\t\tmax_value: Some({{characteristic.Constraints.MaximumValue}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.MinimumValue}}\n\t\tmin_value: Some({{characteristic.Constraints.MinimumValue}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.StepValue}}\n\t\tstep_value: Some({{characteristic.Constraints.StepValue}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.MaximumLength}}\n\t\tmax_len: Some({{characteristic.Constraints.MaximumLength}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.MaximumDataLength}}\n\t\tmax_data_len: Some({{characteristic.Constraints.MaximumDataLength}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.ValidValues}}\n\t\tvalid_values: Some({{valid_values characteristic.Constraints.ValidValues}}),{{/if}}
        ..Default::default()
    }
}
";

static CHARACTERISTIC_MOD: &'static str = "// THIS FILE IS AUTO-GENERATED
{{#each characteristics as |c|}}\npub mod {{c}};{{/each}}
";

static SERVICE: &'static str = "// THIS FILE IS AUTO-GENERATED\n
use service::{HapService, Service};
use characteristic::{
    HapCharacteristic,
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
};
use hap_type::HapType;

pub type {{trim service.Name}} = Service<{{trim service.Name}}Inner>;

impl Default for {{trim service.Name}} {
    fn default() -> {{trim service.Name}} { new() }
}

#[derive(Default)]
pub struct {{trim service.Name}}Inner {
    id: u64,
    hap_type: HapType,
    hidden: bool,
    primary: bool,
    
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\tpub {{characteristic_file_name r ../../this.characteristics}}: {{characteristic_file_name r ../../this.characteristics}}::{{characteristic_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
\n{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\tpub {{characteristic_file_name r ../../this.characteristics}}: Option<{{characteristic_file_name r ../../this.characteristics}}::{{characteristic_name r ../../this.characteristics}}>,
{{/if_eq}}\
{{/each}}\
{{/each}}\
}

impl HapService for {{trim service.Name}}Inner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> &HapType {
        &self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        let mut characteristics: Vec<&HapCharacteristic> = vec![
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t\t&self.{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\t];
{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\tif let Some(c) = &self.{{characteristic_file_name r ../../this.characteristics}} {
\t\t    characteristics.push(c);
\t\t}
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\tcharacteristics
    }
    
    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t\t&mut self.{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\t];
{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\tif let Some(c) = &mut self.{{characteristic_file_name r ../../this.characteristics}} {
\t\t    characteristics.push(c);
\t\t}
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\tcharacteristics
    }
}

pub fn new() -> {{trim service.Name}} {
    {{trim service.Name}}::new({{trim service.Name}}Inner {
        hap_type: \"{{uuid service.UUID}}\".into(),
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t{{characteristic_file_name r ../../this.characteristics}}: {{characteristic_file_name r ../../this.characteristics}}::new(),
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\t..Default::default()
    })
}
";

static SERVICE_MOD: &'static str = "// THIS FILE IS AUTO-GENERATED
{{#each services as |s|}}\npub mod {{s}};{{/each}}
";

fn main() {
    let metadata_file = File::open("default.metadata.json").unwrap();
    let metadata: Metadata = serde_json::from_reader(metadata_file).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("if_eq", Box::new(if_eq_helper));
    handlebars.register_helper("trim", Box::new(trim_helper));
    handlebars.register_helper("file_name", Box::new(file_name_helper));
    handlebars.register_helper("format", Box::new(format_helper));
    handlebars.register_helper("type", Box::new(type_helper));
    handlebars.register_helper("unit", Box::new(unit_helper));
    handlebars.register_helper("uuid", Box::new(uuid_helper));
    handlebars.register_helper("valid_values", Box::new(valid_values_helper));
    handlebars.register_helper("perms", Box::new(perms_helper));
    handlebars.register_helper("float", Box::new(float_helper));
    handlebars.register_helper("characteristic_name", Box::new(characteristic_name_helper));
    handlebars.register_helper("characteristic_file_name", Box::new(characteristic_file_name_helper));
    handlebars.register_template_string("categories", CATEGORIES).unwrap();
    handlebars.register_template_string("characteristic", CHARACTERISTIC).unwrap();
    handlebars.register_template_string("characteristic_mod", CHARACTERISTIC_MOD).unwrap();
    handlebars.register_template_string("service", SERVICE).unwrap();
    handlebars.register_template_string("service_mod", SERVICE_MOD).unwrap();

    let categories = handlebars.render("categories", &metadata).unwrap();
    let categories_path = "src/accessory/category.rs".to_owned();
    let mut categories_file = File::create(&categories_path).unwrap();
    categories_file.write_all(categories.as_bytes()).unwrap();

    let characteristics_base_path = "src/characteristic/includes/";
    let mut characteristsic_names = vec![];
    for c in &metadata.characteristics {
        let characteristic = handlebars.render("characteristic", &json!({"characteristic": c})).unwrap();
        let characteristic_file_name = c.name.replace(" ", "_").replace(".", "_").to_lowercase();
        let mut characteristic_path = String::from(characteristics_base_path);
        characteristic_path.push_str(&characteristic_file_name);
        characteristic_path.push_str(".rs");
        let mut characteristic_file = File::create(&characteristic_path).unwrap();
        characteristic_file.write_all(characteristic.as_bytes()).unwrap();
        characteristsic_names.push(characteristic_file_name);
    }
    let characteristic_mod = handlebars.render("characteristic_mod", &json!({"characteristics": characteristsic_names})).unwrap();
    let mut characteristic_mod_file = File::create(&format!("{}mod.rs", characteristics_base_path)).unwrap();
    characteristic_mod_file.write_all(characteristic_mod.as_bytes()).unwrap();

    let services_base_path = "src/service/includes/";
    let mut service_names = vec![];
    for s in &metadata.services {
        let service = handlebars.render("service", &json!({"service": s, "characteristics": &metadata.characteristics})).unwrap();
        let service_file_name = s.name.replace(" ", "_").replace(".", "_").to_lowercase();
        let mut service_path = String::from(services_base_path);
        service_path.push_str(&service_file_name);
        service_path.push_str(".rs");
        let mut service_file = File::create(&service_path).unwrap();
        service_file.write_all(service.as_bytes()).unwrap();
        service_names.push(service_file_name);
    }
    let service_mod = handlebars.render("service_mod", &json!({"services": service_names})).unwrap();
    let mut service_mod_file = File::create(&format!("{}mod.rs", services_base_path)).unwrap();
    service_mod_file.write_all(service_mod.as_bytes()).unwrap();
}
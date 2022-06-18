use serde::{
    de::DeserializeOwned,
    // de::Deserializer,
    Deserialize,
    Serialize,
};

use std::collections::HashMap;

use eosio_scale_info::{
    Type,
    Path
};

pub struct ActionInfo {
    pub name: String,
    pub info: Type,
}

pub struct TableInfo {
    pub name: String,
    pub info: Type,
}

pub struct ABIInfo {
    pub actions: Vec<ActionInfo>,
    pub tables: Vec<TableInfo>,
    pub structs: Vec<Type>,
    pub variants: Vec<Type>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIType {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIStruct {
    name: String,
    base: String,
    fields: Vec<ABIType>,
}

///
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIAction {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    ricardian_contract: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABITable {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    index_type: String,
    key_names: Vec<String>,
    key_types: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIVariant {
	name: String,
    // #[serde(deserialize_with = "string_or_seq_string")]
    types: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABI {
	version: String,
	types: Vec<String>,
    structs: Vec<ABIStruct>,
    actions: Vec<ABIAction>,
    tables: Vec<ABITable>,
    variants: Vec<ABIVariant>,
    abi_extensions: Vec<String>,
    error_messages: Vec<String>,
    ricardian_clauses: Vec<String>,
}

fn native_type_to_abi_type(tp: &str) -> &str {
    match tp {
        "bool" => "bool",
        "i8" => "int8",
        "u8" => "uint8",
        "i16" => "int16",
        "u16" => "uint16",
        "i32" => "int32",
        "u32" => "uint32",
        "i64" => "int64",
        "u64" => "uint64",
        "i128" => "int128",
        "u128" => "uint128",
        "Varint32" => "varint32",
        "VarUint32" => "varuint32",
        "f32" => "float32",
        "f64" => "float64",
        "Float128" => "float128",
        "TimePoint" => "time_point",
        "TimePointSec" => "time_point_sec",
        "BlockTimeStampType" => "block_timestamp_type",
        "Name" => "name",
        "&[u8]" => "bytes",
        "String" => "string",
        "Checksum160" => "checksum160",
        "Checksum256" => "checksum256",
        "Uint256" => "checksum256",
        "Checksum512" => "checksum512",
        "PublicKey" => "public_key",
        "Signature" => "signature",
        "Symbol" => "symbol",
        "SymbolCode" => "symbol_code",
        "Asset" => "asset",
        "ExtendedAsset" => "extended_asset",
        _ => tp,
    }
}

fn is_intrinsic_abi_type(name: &str) -> bool {
    match name {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" |
        "String" |
        "Varint32" | "VarUint32" | "Float128" | "TimePoint" | "TimePointSec" |
        "BlockTimeStampType" | "Name" | "Checksum160" | "Checksum256" | "Uint256" |
        "Checksum512" | "PublicKey" | "Signature" | "Symbol" | "SymbolCode" | "Asset" |
        "ExtendedAsset"  => {
            return true;
        }
        _=> {
            return false;
        }
    }
}

fn get_full_path_name(path: &Path) -> String {
    return path.segments().to_vec().join("::");
}

fn get_last_path_name(path: &Path) -> String {
    let len = path.segments().len();
    if len == 0 {
        return String::from("");
    }
    return String::from(path.segments()[len-1]);
}

pub fn verify_abi_structs(main_structs: &Vec<Type>) -> Vec<Type> {
    //
    let mut main_struct_map: HashMap<String, &Type> = HashMap::new();
    //<name, full_name>
    let mut struct_name_map: HashMap<String, &Type> = HashMap::new();

    for ty in main_structs {
        let last_name = get_last_path_name(ty.path());
        let full_name = get_full_path_name(ty.path());
        if let Some(ty) = main_struct_map.get(&last_name) {
            let full_name2 = get_full_path_name(ty.path());
            if full_name != full_name2 {
                panic!("Same struct name live in different modules is not supported by ABI\n{}\n<==>\n{}\n", full_name, full_name2);
            }
        } else {
            main_struct_map.insert(full_name.clone(), ty);
        }

        if let Some(ty2) = struct_name_map.get(&last_name) {
            let full_name2 = get_full_path_name(ty2.path());
            if full_name2 != full_name {
                panic!("Same struct name live in different modules is not supported by ABI\n{}\n<==>\n{}\n", full_name, full_name2);
            }
        } else {
            struct_name_map.insert(last_name, ty);
        }
    }

    let hashmap_mutex = eosio_scale_info::get_scale_type_map();
    let global_hash_map = &*hashmap_mutex.lock().unwrap();
    for (full_name, ty) in  global_hash_map {
        let last_name = get_last_path_name(ty.path());
        if let Some(ty) = struct_name_map.get(&last_name) {
            let full_name2 = get_full_path_name(ty.path());
            if full_name2 != *full_name {
                panic!("Same struct name live in different modules is not supported by ABI\n{}\n<==>\n{}\n", *full_name, full_name2);
            }
        } else {
            struct_name_map.insert(last_name, ty);
        }
    }

    let mut other_structs_map: HashMap<String, &Type> = HashMap::new();

    let mut check_rust_type = |rust_type: &str| {
        if is_intrinsic_abi_type(rust_type) {
            return;
        }

        if let Some(_) = main_struct_map.get(rust_type) {
            return
        }

        if let Some(ty) = struct_name_map.get(rust_type) {
            let name = String::from(rust_type);
            if let Some(ty2) = other_structs_map.get(&name) {
                //
            } else {
                other_structs_map.insert(name, *ty);
            }
            return
        }
        panic!("abi struct not found: {}", rust_type);
    };

    main_structs.iter().for_each(|item|{
        match item.type_def() {
            ::eosio_scale_info::TypeDef::Composite(x) => {
                let last_name = get_last_path_name(item.path());
                x.fields().iter().for_each(|field|{
                    let rust_type = *field.type_name().unwrap();
                    if let Some(pos) = rust_type.find("Option<") {
                        let inner_rust_type = &rust_type["Option<".len()..rust_type.len() -1];
                        check_rust_type(inner_rust_type);
                    } else if let Some(pos) = rust_type.find("Vec<") {
                        let inner_rust_type = &rust_type["Vec<".len()..rust_type.len() -1];
                        check_rust_type(inner_rust_type);
                    } else {
                        check_rust_type(rust_type);
                    }
                });
            }
            ::eosio_scale_info::TypeDef::Variant(x) => {
                x.variants().iter().for_each(|v|{
                    let rust_type = v.fields()[0].type_name().unwrap();
                    check_rust_type(*rust_type);
                });
            }
            _ => {
                println!("+++unknown abi type: {:?}", item);
            }
        }
    });

    let mut other_structs: Vec<Type> = Vec::new();
    for (_, ty) in other_structs_map {
        other_structs.push(ty.clone());
    }
    return other_structs;
}

pub fn parse_abi_info(info: &mut ABIInfo) -> String {
    let mut abi = ABI {
        version: String::from("eosio::abi/1.1"),
        types: Vec::new(),
        structs: Vec::new(),
        actions: Vec::new(),
        tables: Vec::new(),
        variants: Vec::new(),
        abi_extensions: Vec::new(),
        error_messages: Vec::new(),
        ricardian_clauses: Vec::new(),    
    };

    let other_structs = verify_abi_structs(&info.structs);
    info.structs.extend(other_structs);

    info.structs.iter().for_each(|item|{
        println!("+++++++++item:{:?}", item);
        match item.type_def() {
            ::eosio_scale_info::TypeDef::Composite(x) => {
                if is_intrinsic_abi_type(&get_last_path_name(item.path())) {
                    return;
                }

                let name = item.path().segments().last().unwrap();
                let mut s = ABIStruct{
                    name: String::from(*name),
                    base: String::from(""),
                    fields: Vec::new(),
                };
                x.fields().iter().for_each(|field|{
                    let mut ty: String;
                    let rust_type = *field.type_name().unwrap();
                    if let Some(pos) = rust_type.find("Option<") {
                        ty = String::from(native_type_to_abi_type(&rust_type["Option<".len()..rust_type.len() -1])) + "?";
                    } else if let Some(pos) = rust_type.find("Vec<") {
                        ty = String::from(native_type_to_abi_type(&rust_type["Vec<".len()..rust_type.len() -1])) + "[]";
                    } else {
                        ty = String::from(native_type_to_abi_type(rust_type));
                    }
                    s.fields.push(
                        ABIType{
                            name: String::from(*field.name().unwrap()),
                            ty,
                        }
                    )
                });
                abi.structs.push(s);
            }
            ::eosio_scale_info::TypeDef::Variant(x) => {
                let name = item.path().segments().last().unwrap();
                let mut abi_variant = ABIVariant{
                    name: String::from(*name),
                    types: Vec::new(),
                };
                x.variants().iter().for_each(|v|{
                    let rust_type = v.fields()[0].type_name().unwrap();
                    abi_variant.types.push(String::from(String::from(native_type_to_abi_type(rust_type))));
                });
                abi.variants.push(abi_variant);    
            }
            _ => {
                println!("+++unknown abi type: {:?}", item);
                // panic!("unknown abi type {:?}", item);
            }
        }
    });

    info.tables.iter().for_each(|table|{
		if let ::eosio_scale_info::TypeDef::Composite(x) = table.info.type_def() {
			let name = table.info.path().segments().last().unwrap();
            abi.tables.push(ABITable {
                name: table.name.clone(),
                ty: String::from(*name),
                index_type: String::from("i64"),
                key_names: Vec::new(),
                key_types: Vec::new(),
            });
        }
    });

    info.actions.iter().for_each(|action|{
		if let ::eosio_scale_info::TypeDef::Composite(x) = action.info.type_def() {
			let name = action.info.path().segments().last().unwrap();
            abi.actions.push(ABIAction {
                name: String::from(*name),
                ty: String::from(*name),
                ricardian_contract: String::from(""),
            });
        }
    });


    if let Ok(contents) = serde_json::to_string_pretty(&abi) {
        return contents;
    }
    return String::from("");
}
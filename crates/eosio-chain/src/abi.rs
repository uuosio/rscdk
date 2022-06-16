use serde::{
    de::DeserializeOwned,
    // de::Deserializer,
    Deserialize,
    Serialize,
};

use eosio_scale_info::Type;

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

pub fn parse_abi_info(info: &ABIInfo) -> String {
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

    info.structs.iter().for_each(|item|{
		if let ::eosio_scale_info::TypeDef::Composite(x) = item.type_def() {
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

    info.variants.iter().for_each(|variant|{
		if let ::eosio_scale_info::TypeDef::Variant(x) = variant.type_def() {
			let name = variant.path().segments().last().unwrap();
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
    });

    if let Ok(contents) = serde_json::to_string_pretty(&abi) {
        return contents;
    }
    return String::from("");
}
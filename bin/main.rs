//#![recursion_limit = "128"]
//#[macro_use]
//extern crate serde_derive;
//
//
//use std::borrow::Cow;
//use std::collections::HashMap;
//use std::sync::Arc;
//
//use mysql::{Error, from_row, Opts, OptsBuilder, Row, QueryResult};
//use mysql::consts::ColumnType;
//use mysql::Value as MyValue;
//
//
//use r2d2::PooledConnection;
//use r2d2_mysql::MysqlConnectionManager;
////use serde::{Deserialize, Serialize};
////use serde::private::de::missing_field;
////use serde_json::{json, Map};
////use serde_json::Value as JsonValue;
////use serde_json::Number;
////use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
////use rustc_serialize::json::Json;
////use rustc_serialize::base64::STANDARD;
////use rustc_serialize::base64::ToBase64;
////use rustc_serialize::base64::FromBase64;
//
//pub enum Value {
//    Bool(bool),
//    I8(i8),
//    I16(i16),
//    I32(i32),
//    I64(i64),
//    U8(u8),
//    U16(u16),
//    U32(u32),
//    U64(u64),
//    F32(f32),
//    F64(f64),
//    String(String),
//    VecU8(Vec<u8>),
////    Json(Json),
////    Uuid(Uuid),
////    DateTime(DateTime<FixedOffset>),
//}
//
//
//fn from_sql_to_rust_type(value: Option<MyValue>,
//                         index: usize,
//                         column_type: &ColumnType)
//                         -> Option<JsonValue> {
////    let value = row.get(index);
//    match value {
//        Some(value) => {
//            match *column_type {
////                ColumnType::MYSQL_TYPE_DECIMAL => {
////                    let v: f64 = mysql::from_value(value.clone());
////                    Some(Value::F64(v))
////                    Some(JsonValue::Number(Number::from_f64(123 as f64).unwrap()))
////                }
////                ColumnType::MYSQL_TYPE_TINY => {
////                    let v: i8 = mysql::from_value(value.clone());
////                    Some(Value::I8(v))
////                }
////                ColumnType::MYSQL_TYPE_SHORT => {
////                    let v: i16 = mysql::from_value(value.clone());
////                    Some(Value::I16(v))
////                }
//                ColumnType::MYSQL_TYPE_LONG => {
//                    let v: i64 = mysql::from_value(value.clone());
////                    Some(Value::I64(v))
//                    Some(JsonValue::Number(Number::from(v)))
//                }
////                ColumnType::MYSQL_TYPE_FLOAT => {
////                    let v: f32 = mysql::from_value(value.clone());
////                    Some(Value::F32(v))
////                }
////                ColumnType::MYSQL_TYPE_DOUBLE => {
////                    let v: f64 = mysql::from_value(value.clone());
////                    Some(Value::F64(v))
////                }
////                ColumnType::MYSQL_TYPE_NULL => None,
//////                ColumnType::MYSQL_TYPE_TIMESTAMP => {
//////                    let v: Timespec = mysql::from_value(value.clone());
//////                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
//////                    let fix = FixedOffset::east(1);
//////                    let t2 = DateTime::from_utc(t, fix);
//////                    Some(Value::DateTime(t2))
//////                }
////                ColumnType::MYSQL_TYPE_LONGLONG => {
////                    let v: i64 = mysql::from_value(value.clone());
////                    Some(Value::I64(v))
////                }
////                ColumnType::MYSQL_TYPE_INT24 => {
////                    let v: i32 = mysql::from_value(value.clone());
////                    Some(Value::I32(v))
////                }
////                ColumnType::MYSQL_TYPE_DATE => {
////                    let v: Timespec = mysql::from_value(value.clone());
////                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
////                    let fix = FixedOffset::east(1);
////                    let t2 = DateTime::from_utc(t, fix);
////                    Some(Value::DateTime(t2))
////                }
////                ColumnType::MYSQL_TYPE_TIME => {
////                    let v: Timespec = mysql::from_value(value.clone());
////                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
////                    let fix = FixedOffset::east(1);
////                    let t2 = DateTime::from_utc(t, fix);
////                    Some(Value::DateTime(t2))
////                }
////                ColumnType::MYSQL_TYPE_DATETIME => {
////                    let v: Timespec = mysql::from_value(value.clone());
////                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
////                    let fix = FixedOffset::east(1);
////                    let t2 = DateTime::from_utc(t, fix);
////                    Some(Value::DateTime(t2))
////                }
////                ColumnType::MYSQL_TYPE_YEAR => {
////                    let v: Timespec = mysql::from_value(value.clone());
////                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
////                    let fix = FixedOffset::east(1);
////                    let t2 = DateTime::from_utc(t, fix);
////                    Some(Value::DateTime(t2))
////                }
//                ColumnType::MYSQL_TYPE_VARCHAR => {
//                    let v: String = mysql::from_value(value.clone());
//                    Some(JsonValue::String(v))
//                }
////                ColumnType::MYSQL_TYPE_BIT => unimplemented!(),
////                ColumnType::MYSQL_TYPE_NEWDECIMAL => {
////                    let v: f64 = mysql::from_value(value.clone());
////                    Some(Value::F64(v))
////                }
////                ColumnType::MYSQL_TYPE_ENUM => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_SET => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_TINY_BLOB => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_MEDIUM_BLOB => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_LONG_BLOB => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_BLOB => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_VAR_STRING => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_STRING => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
////                ColumnType::MYSQL_TYPE_GEOMETRY => {
////                    let v: String = mysql::from_value(value.clone());
////                    Some(Value::String(v))
////                }
//                _ => {
//                    Some(JsonValue::String("".to_string()))
//                }
//            }//<--match column_type
//        } //<-- Some(Value)
//        None => None,
//    }
//}
//
//
//#[derive(ActiveRecord, Serialize, Deserialize, Debug, Default)]
//struct User {
//    id: i32,
//    phone: i32,
//    name: String,
//    fff: Option<i32>,
//}
//
////macro_rules! parse {
////    (@in[] $arg:expr)=>({
////            let mut ret = String::from("(");
////            for i in $arg{
////                if let Value::String(b)=i {
////                ret += format!("'{}'",b).as_ref();
////                } else {
////                ret += format!("{}",i).as_ref();
////                }
////                ret += ",";
////            }
////            ret = ret.trim_end_matches(",").to_string();
////            ret += ")";
////            ret
////    });
////    (@in $arg:expr)=>({
////        let mut ret = $arg.as_object().and_then(|x|{
////                    let mut ret = String::new();
////                    for i in x
////                    {
////                        let a = parse!(@in[] i.1.as_array().unwrap());
////                         ret += format!("`{}` in {} ",i.0,a).as_ref();
////                    }
////                    Some(ret)
////                });
////        ret.unwrap()
////    });
////    ($arg:expr) => ({
////        match $arg {
//////            Value::Null => {
//////                "".to_owned()
//////            }
//////            Value::Bool(x) => {
//////                println!("bool:{}",x);
//////                "".to_owned()
//////            }
//////            Value::Array(x)=>{
//////                "".to_owned()
//////            }
////            Value::String(x) => {
////                x
////            }
//////            Value::Number(x) => {
//////                "".to_owned()
//////            }
////            Value::Object(x) =>{
////
////                if $arg.get("=").is_some(){
////                     "".to_owned()
////                } else if $arg.get("in").is_some() {
////                    parse!(@in $arg.get("in").unwrap())
////                } else if $arg.get("not in").is_some(){
////                    parse!(@in $arg.get("not in").unwrap())
////                } else if $arg.get("like").is_some() {
////                     "".to_owned()
////                } else {
////                    let ret = $arg.as_object().and_then(|x|{
////                        let mut ret = String::new();
////                        for i in x {
////                            if let Value::String(a) = i.1 {
////                                ret += format!("`{}` = '{}' and ",i.0,a).as_ref();
////                            } else {
////                                ret += format!("`{}` = {} and ",i.0,i.1).as_ref();
////                            }
////                        }
////                        ret = ret.trim_end_matches("and ").to_string();
////                        Some(ret)
////                    });
////                    ret.unwrap()
////                }
////
////            }
////            _=>{
////                "".to_owned()
////            }
////        }
////
////    })
////}
////macro_rules! unwrap {
////($result:expr)=>{
////    if $result.is_some() {
////        $result.unwrap()
////    } else {
////        $result
////    }
////};
////    (Some($result:expr))=>{
////        println!("Some:{:?}",$result.unwrap());
////        return $result;
////    };
////    (None)=>{
////        println!("None");
////    }
////}
////macro_rules! cond {
////    (Some($result:tt))=>{
////        cond!($result);
////    };
////    (None)=>{
////        "";
////    };
////    ($arg:tt) => ({
////        parse!(json!($arg))
////    });
////    ($arg:expr) => (
////        println!("2x");
////    );
////}
//macro_rules! builder_struct_field {
//    ($name:tt,$key:stmt,$value:expr) => {
//        $name.$key = $value;
//    };
//}
//#[derive(Debug)]
//enum MyError {
//    Connection,
//    Abc,
//}
//
//use std::fmt;
//use std::panic::resume_unwind;
//use serde_json::map::Values;
//
//impl fmt::Display for MyError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            MyError::Connection => {
//                write!(f, "Connection Faild")
//            }
//            MyError::Abc => {
//                write!(f, "Abc:{}", 1)
//            }
//        }
//    }
//}
//
//
//trait ActiveRecordTrait {
//    fn find() -> ActiveRecord {
//        ActiveRecord {}
//    }
//    fn update_all() {}
//}
//
//struct ActiveRecord {}
//
//impl ActiveRecord {
//    fn and(&self, cond: String) -> &Self {
//        println!("{:?}", cond);
//        self
//    }
//    fn one(&self, db: &mut PooledConnection<MysqlConnectionManager>) -> Result<User, MyError> {
//        db.prepare("select t.id ,t.phone from t left join proxy as p on t.id = p.id limit 1").map_err(|x| {
//            MyError::Connection
//        }).and_then(|mut stmt| {
//            let mut columns = vec![];
//            for col in stmt.columns_ref().unwrap() {
//                let column_name = col.org_name_str().to_string();
//                columns.push((column_name, col.column_type()));
//            }
//            let param: Vec<i32> = vec![];
////            let mut daos = Vec::new();
//            let rows = stmt.execute(&param).unwrap();
//            let mut res = User::default();
//            for row in rows {
//                let row = row.unwrap();
//                let mut index = 0;
//                let mut dao: Map<String, JsonValue> = Map::new();
//                for &(ref column_name, ref column_type) in &columns {
//                    let rtype = from_sql_to_rust_type(row.get(index), index, column_type);
//                    match &**column_name {
//                        "id" => {
//                            if let Some(rtype) = rtype {
//                                match rtype {
//                                    JsonValue::String(a) => {
//                                        println!("string {}", a);
//                                    }
//                                    JsonValue::Number(a) => {
//                                        res.id = a.as_i64().unwrap() as i32;
////                                        res.id = a.
//                                    }
//                                    _ => {}
//                                }
//                            }
//                        }
//                        "phone" => {
//                            if let Some(rtype) = rtype {
//                                match rtype {
//                                    JsonValue::String(a) => {
//                                        println!("string {}", a);
//                                    }
//                                    JsonValue::Number(a) => {
//                                        res.phone = a.as_i64().unwrap() as i32;
//                                    }
//                                    _ => {}
//                                }
//                            }
//                        }
//                        "name" => {
//                            if let Some(rtype) = rtype {
//                                match rtype {
//                                    JsonValue::String(a) => {
//                                        res.name = a;
//                                    }
//                                    _ => {}
//                                }
//                            }
//                        }
//                        _ => {}
//                    }
//                    index += 1;
//                }
//            }
//            Ok(res)
//        })
//    }
//}
//
//impl ActiveRecordTrait for User {}
//
//fn main() {
//    let a = 1;
//    let db_url = "mysql://root:@localhost:3306/novel_wechat";
//    let manager = r2d2_mysql::MysqlConnectionManager::new(OptsBuilder::from_opts(Opts::from(db_url)));
//    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
//    let mut a = pool.get().unwrap();
//    let mut w = User::default();
////    let mut w = User::default();
////    builder_struct_field!(w,fff,Some(123123));
////    builder_struct_field!(w,id,123123);
////    builder_struct_field!(w,name,"adfadsf".to_string());
////    println!("{:?}", w);
////    println!("{}", cond!({"id":1,"b":"哈哈","C":true}));
////    println!("{}", cond!("id = 1 and b = 2"));
////    println!("{}", cond!(w));
////    println!("{}", cond!({ "in" :{"id":["adfadf",2,3,4,5]},"not in":{"id":[1,2,3,4,5]}}));
//
//    let u = User::find().one(&mut a);
//    u.and_then(|x| {
//        println!("{:?}", x);
//        Ok(())
//    });
//}

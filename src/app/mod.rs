use crate::err::Error;
use serde_json::Value as Json;
use serde_wasm_bindgen::from_value;
use surrealdb::rpc::format::cbor::Cbor;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Uint8Array;

#[wasm_bindgen(start)]
pub fn setup() {
    crate::log::init();
}

#[wasm_bindgen]
pub fn parse(sql: &str) -> Result<JsValue, Error> {
    let ast = surrealdb::sql::parse(sql)?;
    let res = serde_wasm_bindgen::to_value(&ast)?;
    Ok(res)
}

#[wasm_bindgen]
pub fn format(sql: &str, pretty: bool) -> Result<String, Error> {
    let ast = surrealdb::sql::parse(sql)?;
    Ok(match pretty {
        true => format!("{ast:#}"),
        false => format!("{ast}"),
    })
}

#[wasm_bindgen]
pub fn validate(sql: &str) -> Result<(), Error> {
    surrealdb::sql::parse(sql)?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_where(sql: &str) -> Result<(), Error> {
    let sql = format!("SELECT * FROM validate WHERE {sql}");
    surrealdb::sql::parse(&sql)?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_value(sql: &str) -> Result<(), Error> {
    surrealdb::sql::value(sql)?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_thing(sql: &str) -> Result<(), Error> {
    surrealdb::sql::thing(sql)?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_idiom(sql: &str) -> Result<(), Error> {
    surrealdb::sql::idiom(sql)?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_subquery(sql: &str) -> Result<(), Error> {
    surrealdb::sql::subquery(sql)?;
    Ok(())
}

#[wasm_bindgen]
pub struct Value {
    inner: surrealdb::sql::Value,
}

#[wasm_bindgen]
impl Value {
    /// Create a new SurrealQL value
    ///
    /// ```js
    /// const value = Value.from_json({ test: true });
    /// ```
    #[wasm_bindgen]
    pub fn from_json(val: JsValue) -> Result<Value, Error> {
        let val = surrealdb::sql::json(&from_value::<Json>(val)?.to_string())?;
        Ok(Value { inner: val })
    }

    /// Create a new SurrealQL value
    ///
    /// ```js
    /// const value = Value.from_cbor(<binary data>);
    /// ```
    #[wasm_bindgen]
    pub fn from_cbor(val: Uint8Array) -> Result<Value, Error> {
        let val: ciborium::Value =
            ciborium::from_reader::<ciborium::Value, _>(&mut val.to_vec().as_slice())
                .map_err(|_| Error::from("Received invalid binary data"))?;
        let val: surrealdb::sql::Value =
            <Cbor as TryInto<surrealdb::sql::Value>>::try_into(Cbor(val))
                .map_err(|_| Error::from("Received invalid binary data"))?;
        Ok(Value { inner: val })
    }

    /// Format a SurrealQL value
    ///
    /// ```js
    /// const value = Value.from_json({ test: true });
    /// const output = value.format(true);
    ///
    #[wasm_bindgen]
    pub fn format(&self, pretty: bool) -> Result<String, Error> {
        Ok(match pretty {
            true => format!("{:#}", self.inner),
            false => format!("{}", self.inner),
        })
    }
}

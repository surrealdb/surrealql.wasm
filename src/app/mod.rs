use crate::err::Error;
use serde_json::ser::PrettyFormatter;
use serde_json::Value as Json;
use serde_wasm_bindgen::from_value;
use surrealdb::dbs::capabilities::Targets;
use surrealdb::dbs::Capabilities;
use surrealdb::rpc::format::cbor::Cbor;
use surrealdb::sql::Statement;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Uint8Array;
use serde::ser::Serialize;

macro_rules! caps {
    () => {
        &Capabilities::all().with_experimental(Targets::All)
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    crate::log::init();
}

#[wasm_bindgen]
pub fn parse(sql: &str) -> Result<JsValue, Error> {
    let ast = surrealdb::syn::parse_with_capabilities(sql, caps!())?;
    let ser = serde_wasm_bindgen::Serializer::json_compatible().serialize_large_number_types_as_bigints(true);
    let res = ast.serialize(&ser)?;
    Ok(res)
}

#[wasm_bindgen]
pub fn format(sql: &str, pretty: bool) -> Result<String, Error> {
    let ast = surrealdb::syn::parse_with_capabilities(sql, caps!())?;
    Ok(match pretty {
        true => format!("{ast:#}"),
        false => format!("{ast}"),
    })
}

#[wasm_bindgen]
pub fn validate(sql: &str) -> Result<(), Error> {
    surrealdb::syn::parse_with_capabilities(sql, caps!())?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_where(sql: &str) -> Result<(), Error> {
    let sql = format!("SELECT * FROM validate WHERE {sql}");
    surrealdb::syn::parse_with_capabilities(&sql, caps!())?;
    Ok(())
}

#[wasm_bindgen]
pub fn validate_value(sql: &str) -> Result<(), Error> {
    surrealdb::syn::parse_with_capabilities(sql, caps!())?;
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
    let sql = format!("({sql})");
    let mut val = surrealdb::syn::parse_with_capabilities(&sql, caps!())?;

    // Validate the value
    let Statement::Value(val) = val.remove(0) else {
        return Err("Internal error: Expected a Statement::Value when parsing a subquery".into());
    };

    // Validate the value
    if !matches!(val, surrealdb::sql::Value::Subquery(_)) {
        return Err("Internal error: Expected a Value::Subquery when parsing a subquery".into());
    };

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
    /// const value = Value.from_string("{ test: true }");
    /// ```
    #[wasm_bindgen]
    pub fn from_string(val: String) -> Result<Value, Error> {
		let val = surrealdb::sql::value(&val).map_err(|_| "Failed to parse value from string")?;
        Ok(Value { inner: val })
    }
    /// Create a new SurrealQL value from a JSON string
	/// Errors when it encounters non-json values
    ///
    /// ```js
    /// const value = Value.from_json_string("{ test: true }");
    /// ```
    #[wasm_bindgen]
    pub fn from_json_string(val: String) -> Result<Value, Error> {
		let val = surrealdb::sql::json(&val).map_err(|_| "Failed to parse value from JSON string")?;
        Ok(Value { inner: val })
    }

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

    /// Format a SurrealQL value into JSON
    ///
    /// ```js
    /// const value = Value.from_json({ test: true });
    /// const output = value.json(true);
    ///
    #[wasm_bindgen]
    pub fn json(&self, pretty: bool) -> Result<String, Error> {
        Ok(match pretty {
            true => {
				let mut buf = Vec::new();
				let mut serializer = serde_json::Serializer::with_formatter(
					&mut buf,
					PrettyFormatter::with_indent(b"\t"),
				);
				self.inner.clone().into_json().serialize(&mut serializer).unwrap();
				String::from_utf8(buf).unwrap()
			}
            false => self.inner.clone().into_json().to_string(),
        })
    }

    /// Return a parsed SurrealQL value as CBOR
    ///
    /// ```js
    /// const value = Value.from_string("{ test: true }");
    /// const output = value.to_cbor();
    ///
    #[wasm_bindgen]
	pub fn to_cbor(&self) -> Result<Uint8Array, Error> {
		// Into CBOR value
		let cbor: Cbor = self.inner.clone().try_into().map_err(|_| "Failed to convert Value to CBOR")?;
		let mut res = Vec::new();
		ciborium::into_writer(&cbor.0, &mut res).unwrap();
		let out_arr: Uint8Array = res.as_slice().into();
		Ok(out_arr.into())
	}
}

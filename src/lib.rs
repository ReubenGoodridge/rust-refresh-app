use redis::Commands;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_redis_key(host: &str, port: &str, key: &str) -> Result<String, JsValue> {
    let client = redis::Client::open(format!("redis://{}:{}", host, port)).map_err(|err| {
        JsValue::from_str(&format!("Failed to connect to Redis: {:?}", err))
    })?;
    
    let mut connection = client.get_connection().map_err(|err| {
        JsValue::from_str(&format!("Failed to establish Redis connection: {:?}", err))
    })?;
    
    let value: String = connection.get(key).map_err(|err| {
        JsValue::from_str(&format!("Failed to retrieve key from Redis: {:?}", err))
    })?;
    
    Ok(value)
}

#[wasm_bindgen]
pub fn increment_redis_key(host: &str, port: &str, key: &str) -> Result<(), JsValue> {
    let client = redis::Client::open(format!("redis://{}:{}", host, port)).map_err(|err| {
        JsValue::from_str(&format!("Failed to connect to Redis: {:?}", err))
    })?;
    
    let mut connection = client.get_connection().map_err(|err| {
        JsValue::from_str(&format!("Failed to establish Redis connection: {:?}", err))
    })?;
    
    let _: () = connection.incr(key, 1).map_err(|err| {
        JsValue::from_str(&format!("Failed to increment key in Redis: {:?}", err))
    })?;
    
    Ok(())
}


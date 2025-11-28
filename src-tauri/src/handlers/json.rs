use serde_json::{to_value, Map, Value, Number}; // 引入 Number
use wz_reader::{property::WzSubProperty, WzNode, WzObjectType};

// === 新增辅助函数：尝试将字符串 Value 转换为数字 Value ===
fn try_convert_string_to_number(v: Value) -> Value {
    if let Value::String(ref s) = v {
        // 1. 尝试转换为整数 (i64)
        if let Ok(int_val) = s.parse::<i64>() {
            return Value::Number(int_val.into());
        }
        // 2. 尝试转换为浮点数 (f64)
        if let Ok(float_val) = s.parse::<f64>() {
            // Number::from_f64 返回 Option，因为 JSON 不支持 NaN 或 Infinity
            if let Some(n) = Number::from_f64(float_val) {
                return Value::Number(n);
            }
        }
    }
    // 如果转换失败或不是字符串，原样返回
    v
}

pub fn to_simple_json(node: &WzNode) -> Result<serde_json::Value, serde_json::Error> {
    if node.children.is_empty() {
        match &node.object_type {
            WzObjectType::Value(value_type) => {
                // 原逻辑：return Ok(value_type.clone().into());
                // 修改后：先转换成 Value，再尝试清洗数据
                let val: Value = value_type.clone().into();
                return Ok(try_convert_string_to_number(val));
            },
            WzObjectType::Property(WzSubProperty::PNG(inner)) => return to_value(inner),
            WzObjectType::Property(WzSubProperty::Sound(inner)) => return to_value(inner),
            _ => return Ok(Value::Null),
        }
    }

    let mut json = Map::new();
    let mut generate_extra_path = false;

    match &node.object_type {
        WzObjectType::Property(WzSubProperty::PNG(inner)) => {
            let dict = to_value(inner)?;
            generate_extra_path = true;

            if let Value::Object(dict) = dict {
                for (name, value) in dict {
                    // 对 PNG 属性内部的值也尝试转换 (可选，视情况而定)
                    json.insert(name, try_convert_string_to_number(value));
                }
            }
        }
        WzObjectType::Property(WzSubProperty::Sound(inner)) => {
            let dict = to_value(inner)?;

            if let Value::Object(dict) = dict {
                for (name, value) in dict {
                    json.insert(name, try_convert_string_to_number(value));
                }
            }
        }
        _ => {}
    }

    for (name, value) in node.children.iter() {
        let child = value.read().unwrap();
        // 递归调用会自动处理子节点的类型转换
        json.insert(name.to_string(), to_simple_json(&child)?);
    }

    if generate_extra_path && !json.contains_key("_outlink") {
        json.insert("path".to_string(), Value::String(node.get_full_path()));
    }

    Ok(Value::Object(json))
}
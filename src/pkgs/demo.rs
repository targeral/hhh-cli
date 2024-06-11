use serde_json::{json, Value};

fn main() {
    // 创建一个空的 JSON 对象
    let mut o = json!({});
    
    // 使用可变引用来模仿 JavaScript 中的对象赋值
    let mut c = &mut o;

    // 假设循环次数为5
    for i in 0..5 {
        // 检查当前的 c 是否是对象
        if !c.is_object() {
            continue;
        }

        // 如果当前索引不存在，则插入一个新的 JSON 对象
        if !c[i.to_string()].is_object() {
            c[i.to_string()] = json!({});
        }
        
        // 通过索引获取当前的 JSON 对象的可变引用
        c = c.get_mut(i.to_string()).unwrap();
    }

    // 打印结果
    println!("{}", o.to_string());
}
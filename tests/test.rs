/**
 * 文件名: "tests/test.rs" json解析器测试代码
 * Copyright (C) 2023 朱浚宇
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 * 
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. 
 */

#[cfg(test)]
#[test]
fn test_zjy_json(){
    use std::fs::read_to_string;

    use zjy_json::zjy::json::Json;

    let array_json_str=read_to_string("tests/testarray.json").unwrap();
    println!("Array:{}",Json::str_to_json(&array_json_str).unwrap());

    println!("-----------------------------------------------------");

    let object_json_str=read_to_string("tests/testobject.json").unwrap();
    println!("Object:{}",Json::str_to_json(&object_json_str).unwrap());

    println!("-----------------------------------------------------");

    let null_json_str=read_to_string("tests/testnull.json").unwrap();
    println!("Null:{}",Json::str_to_json(&null_json_str).unwrap());

    println!("-----------------------------------------------------");

    let string_json_str=read_to_string("tests/teststring.json").unwrap();
    println!("String:{}",Json::str_to_json(&string_json_str).unwrap());

}
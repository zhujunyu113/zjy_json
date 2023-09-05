/**
 * 文件名: "src/lib.rs" json解析器源代码
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


pub mod zjy;
#[cfg(test)]
#[test]
fn mai() {
    use crate::zjy::json::Json;
    let s=std::fs::read_to_string("tests/teststring.json").unwrap();
    println!("识别到的json:\n{}",Json::str_to_json(&s).unwrap());
    println!("识别到的json:\n{}",Json::str_to_json("-2323").unwrap());
    println!("识别到的json:\n{}",Json::str_to_json("-1E+10").unwrap());
}

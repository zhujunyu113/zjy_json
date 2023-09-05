/**
 * 文件名: "src/zjy/json/ast/null.rs" json解析器源代码
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
impl NullMode {
    #[allow(dead_code)]
    #[cold]
    pub fn new() -> NullMode{ NullMode::n }
    /**
     * 检查null语法
     * 单次只检查一个字符
     * 状态保存在结构体内
     * # 返回值: 如果传入的字符没有通过检查,返回错误,如果null语法检查完毕,就返回true,否则返回false
     * # 参数: 本次要检测的字符
     */
    #[allow(dead_code)]
    pub fn check_null(&mut self, c:char) -> Result<bool,()>{
        match self {
            NullMode::n => {
                match c {
                    'n'=>{
                        *self= NullMode::u;
                        return Ok(false);
                    }
                    '\n'|'\r'|'\x20'=>{
                        // 允许换行,空格
                        return Ok(false);
                    },
                    _=>{return Err(());},
                }

            },
            NullMode::u => {
                match c {
                    'u'=>{
                        *self=NullMode::l1;
                        return Ok(false);
                    }
                    _=>{
                        return Err(());
                    }
                }
            },
            NullMode::l1 => {
                match c {
                    'l'=>{
                        *self=NullMode::l2;
                        return Ok(false);
                    }
                    _=>{
                        return Err(());
                    }
                }

            },
            NullMode::l2 => {
                match c {
                    'l'=>{
                        *self=NullMode::n;
                        return Ok(true);
                    }
                    _=>{
                        return Err(());
                    }
                }

            },
        }

    }
}
/**
 * 用于保存应该检测什么的枚举
 */
#[allow(non_camel_case_types)]
pub enum NullMode {
    n,
    u,
    l1,
    l2,
}

#[test]
fn test_check_null(){
    let json_str=read_file();
    let mut connetc=NullMode::new();
    for c in json_str.chars(){
        match connetc.check_null(c){
            Ok(isfinsh) =>{
                if isfinsh {
                    println!("识别null完成");
                    break;
                }
            },
            Err(_err) => {
                panic!("识别null失败");
            },
        };
    }
}
#[cfg(test)]
fn read_file() -> String{
    use std::fs;
    return fs::read_to_string("tests/testnull.json").unwrap();
}
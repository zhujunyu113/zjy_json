/**
 * 文件名: "src/zjy/json/ast/false.rs" json解析器源代码
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

#[allow(non_camel_case_types)]
#[allow(dead_code)]
/**
 * 保存下一次该解析false字符串的第几位的句柄
 */
pub enum FalseMode {
    f,a,l,s,e
}
impl FalseMode {
    #[allow(dead_code)]
    #[cold]
    /**
     * 初始化一个解析false字符串的句柄
     */
    pub fn new() -> FalseMode{ FalseMode::f }
    #[allow(dead_code)]
    /**
     * 通过句柄解析false字符串
     * # 返回值: true代表已经解析完毕 false代表解析还未完成 ,返回Err代表false字符串包含非法的字符
     */
    pub fn check_false(&mut self, c:char) ->Result<bool,()>{
        match self {
            FalseMode::f => {
                match c {
                    'f' =>{
                        *self=FalseMode::a;
                        return Ok(false);
                    }
                    '\r'|'\n'|'\x20'=>{
                        return Ok(false);
                    }
                    _=>{
                        return Err(());
                    }
                }
            },
            FalseMode::a => {
                match c {
                    'a' => {
                        *self=FalseMode::l;
                        return Ok(false);
                    }
                    _=>{
                        return Err(());
                    }
                }
            },
            FalseMode::l => {
                match c {
                    'l' => {
                        *self=FalseMode::s;
                        return Ok(false);
                    }
                    _=>{
                        return Err(());
                    }
                }
            },
            FalseMode::s => {
                match c {
                    's' => {
                        *self=FalseMode::e;
                        return Ok(false);
                    }
                    _=>{
                        return Err(());
                    }
                }
            },
            FalseMode::e => {
                match c {
                    'e' => {
                        *self=FalseMode::f;
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

#[cfg(test)]
fn test_check_false(str:&str){
    let mut conntest=FalseMode::new();
    for c in str.chars(){
        match conntest.check_false(c) {
            Ok(isfinish) =>{
                if isfinish{
                    println!("false");
                    break;
                }
            },
            Err(_) => {
                panic!("识别false失败");
            },
        }
    }
}
#[cfg(test)]
#[test]
fn test_check_false_pass(){
    test_check_false("false");
}
#[cfg(test)]
#[test]
#[should_panic]
fn test_check_false_painc(){
    test_check_false("fAlse");
}
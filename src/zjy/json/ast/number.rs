/**
 * 文件名: "src/zjy/json/ast/number.rs" json解析器源代码
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
use std::num::ParseFloatError;
/**
 * 保存解析number时所需要的上下文的结构体
 */
pub struct NumberContext{
    number_str:String,
    number:f64,
}

impl NumberContext {
    /**
     * 获取NumberContext中number_str字段的可变借用
     */
    pub fn get_number_str_mut<'a>(&'a mut self) ->&'a mut String{ return &mut self.number_str; }
    /**
     * 获取NumberContext中number字段的值,这是已经解析的浮点数的值
     */
    pub fn get_number(&self) ->f64{self.number}

    #[allow(dead_code)]
    /**
     * 指定number_str字段初始化NumberContext
     */
    pub fn new_from_str (str:&str) ->NumberContext{NumberContext { number_str: String::from(str), number: 0.0 }}
    /**
     * 初始化NumberContext
     */
    pub fn new() -> NumberContext{NumberContext { number_str: String::new(), number: 0.0 }}
    /**
     * 将str参数加到number_str的后面,然后尝试将number_str转化成浮点数,成功会把转化
     * 之后的得到的浮点数保存在number字段中,如果转化失败,返回Err
     */
    pub fn check_number(&mut self, str:&str) ->Result<(),ParseFloatError>{
        self.number_str.push_str(str);
        self.number =  self.number_str.parse::<f64>()?;
        return Ok(());
    }
}
#[cfg(test)]
#[test]
fn test_f64tostr(){
    
    let str="3423000.01111";
    let f=str.parse::<f64>().unwrap();
    println!("str={},f={}",str,f);
    
}

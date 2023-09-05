/**
 * 文件名: "src/zjy/mod.rs" json解析器源代码
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

pub mod json{
    use std::fmt::{self};
    use std::collections::HashMap;

    use crate::zjy::json::ast::array::ArrayContext;
    use crate::zjy::json::ast::null::NullMode;
    use crate::zjy::json::ast::number::NumberContext;
    use crate::zjy::json::ast::object::{ObjectContext, ObjectMode};
    use crate::zjy::json::ast::r#false::FalseMode;
    use crate::zjy::json::ast::r#true::TrueMode;
    use crate::zjy::json::ast::string::{StringContext, StringMode};

    #[allow(dead_code)]
    /**
     * json实例对象
     */
    pub enum Json {
        /// 对应json中的Null
        Null,//null
        /// 对应json中的Boolen
        Boolean(bool),
        /// 对应json中的Number(64位浮点值)
        Number(f64),
        /// 对应json中的String(utf8字符串)
        String(String),
        /// 对应json中的Object(对象)
        Object(HashMap<String,Json>),
        /// 对应json中的Array(数字)
        Array(Vec<Json>)
    }
    impl Json {
            /**
             * 将字符串形式的json转换为json实例对象
             */
        pub fn str_to_json(str:&str) ->Result<Json,String>{
            if str.len()==0{
                return Err("str参数长度为0".to_string());
            }

            // 初始化计数器
            let mut char_index:usize=0;
            let mut json_type:Option<Json>=None;
            for c in str.chars(){
                char_index=char_index+1;
                match judgment_json_type_option(c) {
                    Ok(option) => {
                        if let Some(json)=option{
                            // 如果不为空
                            json_type=Some(json);
                            break;
                        }
                    },
                    Err(err) => {return Err(std::format!("第1个字符:{}",err));},
                }
            };
            // 判空
            let json_type=match json_type {
                Some(json) => json,
                None => {return Err(std::format!("没能解析出任何json对象"));},
            };
            
            // 重新指定迭代器的位置
            let mut chars=str.chars();
            {
                // 这会把字符指针指定到第一个json字符的位置
                let mut tmp:usize=0;
                while tmp < char_index {
                    chars.next();
                    tmp += 1;
                }
            }
            let mut chars_f=str.chars();
            {
                // 这会把字符指针指定到第一个json字符的前一个位置
                let mut tmp:usize=0;
                while tmp < char_index-1 {
                    chars_f.next();
                    tmp += 1;
                }
            }
            

            // 根据不同的类型,选择不同的上下文

            match json_type {
                Json::Null => {
                    let mut conntest=NullMode::u;
                    // 继续遍历
                    for c in chars{
                        char_index=char_index+1;
                        match conntest.check_null(c) {
                            Ok(isfinish) =>{
                                if isfinish{
                                    // 判断完毕
                                    return Ok(Json::Null);
                                }
                            },
                            Err(_err) => {return Err(std::format!("第{}个字符:{}'{}'",char_index,"此处不应是:",c));},
                        };
                    }
                    // for循环结束还没返回
                    return Err(std::format!("第{}个字符:null字符没有结束",char_index));
                },
                Json::Boolean(b) => {
                    if b{
                        let mut conntest=TrueMode::r;
                        for c in chars {
                            char_index=char_index+1;
                            match conntest.check_true(c) {
                                Ok(isfinish) => {
                                    if isfinish{
                                        return Ok(Json::Boolean(true));
                                    }
                                    
                                },
                                Err(_err) => {return Err(std::format!("第{}个字符:{}'{}'",char_index,"此处不应是:",c));},
                            }
                            
                        }
                        // for循环结束还没返回
                        return Err(std::format!("第{}个字符:true字符串没有结束",char_index));
                    }else {
                        let mut conntest =FalseMode::a;
                        for c in chars{
                            char_index=char_index+1;
                            match conntest.check_false(c) {
                                Ok(isfinish) => {
                                    if isfinish{
                                        return Ok(Json::Boolean(false));
                                    }
                                },
                                Err(_err) => {return Err(std::format!("第{}个字符:{}'{}'",char_index,"此处不应是:",c));},
                            }
                        }
                        // for循环结束还没返回
                        return Err(std::format!("第{}个字符:false字符串没有结束",char_index));
                    }
                },
                Json::Number(_) => {
                    // 直接将这个字符串判断
                    let mut context: NumberContext=NumberContext::new();
                    match context.check_number(str) {
                        Ok(_) => {},
                        Err(err) => {return Err(std::format!("字符串:{}不是一个合法的数字,错误对象:{}",str,err));},
                    }
                    return Ok(Json::Number(context.get_number()));
                },
                Json::String(_) => {
                    let mut context=StringContext::new();
                    // 需要前一个
                    for c in chars_f{
                        char_index=char_index+1;
                        match context.check_string(c) {
                            Ok(_) => {
                                if *(context.get_mode()) == StringMode::end{
                                    return Ok(Json::String(context.get_str().to_string()));
                                }
                            },
                            Err(_) =>{
                                {return Err(std::format!("第{}个字符:此处不应是:'{}'",char_index,c));}
                            },
                        }
                    }
                    // 如果for循环结束还没判断为一个完整的字符串
                    return Err(std::format!("第{}个字符:这个Sting类型缺少 '{}' 结束",char_index,'"'));
                },
                Json::Object(_) => {
                    let mut context=ObjectContext::new_key();
                    for c in chars {
                        char_index=char_index+1;
                        match context.check_object(c) {
                            Ok(_) => {
                                if let ObjectMode::End=context.get_mode_ptr(){
                                    // 如果已经完成
                                    return Ok(context.into_json());
                                }
                            },
                            Err(err) => {return Err(std::format!("第{}个字符:{}",char_index,err));},
                        }
                    }
                    // for循环结束还没返回
                    return Err(std::format!("第{}个字符:这个Object并没有结束",char_index));
                },
                Json::Array(_) => {
                    let mut context=ArrayContext::new_judgment_type();
                    for c in chars{
                        char_index=char_index+1;
                        match context.check_array(c) {
                            Ok(_) => {
                                // 判断是否完成
                                if let ast::array::ArrayMode::End=context.get_mode_ptr(){
                                    return Ok(context.into_json());
                                }
                            },
                            Err(err) => {
                                return Err(std::format!("第{}个字符:{}",char_index,err));
                            },
                        }
                    }
                    // for循环结束还没返回
                    return Err(std::format!("第{}个字符:此处需要']'结束数组",char_index));
    
                },
            }
        }
        

        
    }
    mod ast;

    pub(crate) fn judgment_json_type_option(c:char) ->Result<Option<Json>, String>{
        match c {
            'n'=>{
                return Ok(Option::Some(Json::Null));
            }
            't'=>{
                return Ok(Option::Some(Json::Boolean(true)));
            }
            'f'=>{
                return Ok(Option::Some(Json::Boolean(false)));
            }
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'-'=>{
                return Ok(Option::Some(Json::Number(0.0)));
            }
            '"'=>{
                return Ok(Option::Some(Json::String(String::new())));
            }
            '['=>{
                return Ok(Option::Some(Json::Array(Vec::new())));
            }
            '{'=>{
                return Ok(Option::Some(Json::Object(HashMap::new())));
            }
            // 允许空格回车
            '\r'|'\n'|'\x20'=>{
                return Ok(Option::None)
            }
            _=>{
                return Err(std::format!("此处不应该是: '{}'",c));
            }
        }
    }

    pub(crate) fn judgment_json_type(c:char) ->Result<Json, String>{
        match c {
            'n'=>{
                return Ok(Json::Null);
            }
            't'=>{
                return Ok(Json::Boolean(true));
            }
            'f'=>{
                return Ok(Json::Boolean(false));
            }
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'-'=>{
                return Ok(Json::Number(0.0));
            }
            '"'=>{
                return Ok(Json::String(String::new()));
            }
            '['=>{
                return Ok(Json::Array(Vec::new()));
            }
            '{'=>{
                return Ok(Json::Object(HashMap::new()));
            }
            _=>{
                return Err(std::format!("此处不应该是: '{}'",c));
            }
        }
    }

    impl fmt::Display for Json {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Json::Null => {
                    return write!(f,"null"); 
                },
                Json::Boolean(b) => {
                    if *b{
                        return write!(f,"true");
                    } else {
                        return write!(f, "false");
                    }
                },
                Json::Number(n) => {
                    return write!(f,"{}",*n);
                },
                Json::String(s) => {
                    return write!(f,"{}{}{}","\"",*s,"\"");
                },
                Json::Object(obj) => {
                    let mut result="{".to_string();
                    // 迭代
                    for (key,value) in obj.iter(){
                        result.push('"');
                        result.push_str(key);
                        result.push('"');
                        result.push(':');
                        result.push_str(&value.to_string());
                        result.push(',');
                    }
                    result.pop();
                    result.push('}');
                    return write!(f,"{}", result);
                },
                Json::Array(arr) => {
                    let mut tmp:String=String::new();
                    tmp.push('[');
                    for obj in arr{
                        tmp.push_str(&obj.to_string());
                        tmp.push(',');
                    }
                    tmp.pop();
                    tmp.push(']');
                    return write!(f,"{}",tmp);
                },
            }
        }
    }
    impl Clone for Json {
        fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Object(arg0) => Self::Object(arg0.clone()),
            Self::Array(arg0) => Self::Array(arg0.clone()),
        }
    }
    }


    #[cfg(test)]
    #[test]
    fn test_str_to_json_f64_pass(){
        println!("读取出的json:{}",Json::str_to_json("null").unwrap());
    }
    #[cfg(test)]
    #[test]
    #[should_panic]
    fn test_str_to_json_f64_painc(){
        println!("读取出的json:{}",Json::str_to_json("4324.0000242ff").unwrap());
    }


    #[cfg(test)]
    #[test]
    fn test_str_to_json_pass(){
        println!("{}",Json::str_to_json("null").unwrap());
    }
    #[cfg(test)]
    #[test]
    fn test_str_to_json_true_pass(){
        println!("测试:{}",Json::str_to_json("true").unwrap());
    }
    #[cfg(test)]
    #[test]
    #[should_panic]
    fn test_str_to_json_true_painc(){
        println!("测试:{}",Json::str_to_json("tfe").unwrap());
    }
    #[cfg(test)]
    #[test]
    fn test_str_to_json_false_pass(){
        println!("测试:{}",Json::str_to_json("false").unwrap());
    }
    #[cfg(test)]
    #[test]
    #[should_panic]
    fn test_str_to_json_false_painc(){
        println!("测试:{}",Json::str_to_json("falSE").unwrap());
    }
    #[cfg(test)]
    #[test]
    #[should_panic]
    fn test_str_to_json_painc(){
        println!("{}",Json::str_to_json("nulL").unwrap());
    }

}
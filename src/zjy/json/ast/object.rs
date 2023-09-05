/**
 * 文件名: "src/zjy/json/ast/object.rs" json解析器源代码
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
use std::collections::HashMap;

use crate::zjy::json::{ast::string::StringMode, Json, judgment_json_type};

use super::{string::StringContext, null::NullMode, r#true::TrueMode, r#false::FalseMode, array::{ArrayContext, ArrayMode}, number::NumberContext};

pub struct ObjectContext{
    obj:HashMap<String,Json>,
    mode:ObjectMode,
    key_tmp:String

}
pub enum ObjectMode {
    Key(StringContext),
    Maohao,
    Value,
    ValueNull(NullMode),ValueTrue(TrueMode),ValueFalse(FalseMode),ValueString(StringContext),
    ValueArray(ArrayContext),ValueNumber(NumberContext),ValueObject(Box<ObjectContext>),
    Douhao,
    End
    
}
#[cfg(test)]
#[test]
fn test_check_object(){
    let json=std::fs::read_to_string("tests/testobject.json").unwrap();
    println!("json:\n{}",Json::str_to_json(&json).unwrap());
}
impl ObjectContext {
    pub fn get_json(&self) -> Json{
        return Json::Object(self.obj.clone());
    }
    pub fn get_mode_ptr<'a>(&'a self) ->&'a ObjectMode{ return &self.mode; }
    pub fn into_json(self) ->Json {return Json::Object(self.obj);}
    pub fn new_key() ->ObjectContext{ ObjectContext { obj:HashMap::new(), mode: ObjectMode::Key(StringContext::new()), key_tmp:String::new() }}
    pub fn check_object(&mut self, c:char) ->Result<(),String>{
        match &mut self.mode {
            // object中的key,必须为string
            ObjectMode::Key(stringcontext) => {
                if let Err(_)=stringcontext.check_string(c){
                    return Err(std::format!("这个object中的name不合法"));
                }
                // 如果解析完成
                if let StringMode::end= stringcontext.get_mode(){
                    // 把name存进临时变量
                    self.key_tmp.push_str(stringcontext.get_str());
                    // 切换模式
                    self.mode=ObjectMode::Maohao;
                }
                // 返回
                return Ok(());
            },
            ObjectMode::Maohao => {
                // key和value的分隔符
                match c {
                    ':'=>{
                        // 切换模式
                        self.mode=ObjectMode::Value;
                        return Ok(());
                    }
                    '\r'|'\n'|'\x20'=>{
                        // 允许空格回车
                        return Ok(());
                    }
                    _=>{
                        // 不允许出现其他字符
                        return Err(std::format!("此处需要 ':' ,但实际为: '{}' ",c));
                    }
                }
            },
            // 需要判断json的类型
            ObjectMode::Value => {
                match c {
                    '\r'|'\n'|'\x20'=>{
                        // 允许空格回车
                        return Ok(());
                    }
                    _=>{
                        match judgment_json_type(c)? {
                            Json::Null => {
                                self.mode=ObjectMode::ValueNull(NullMode::u);
                                return Ok(());
                            },
                            Json::Boolean(b) => {
                                self.mode= if b{
                                    ObjectMode::ValueTrue(TrueMode::r)
                                } else {
                                    ObjectMode::ValueFalse(FalseMode::a)
                                };
                                return Ok(());
                            },
                            Json::Number(_) => {
                                // 切换模式
                                self.mode=ObjectMode::ValueNumber(NumberContext::new_from_str(&c.to_string()));
                                return Ok(());
                            },
                            Json::String(_) => {
                                // 切换模式
                                self.mode=ObjectMode::ValueString(StringContext { str: String::new(), mode: StringMode::str });
                                return Ok(());
                            },
                            Json::Object(_) => {
                                // 切换模式
                                self.mode=ObjectMode::ValueObject(Box::new(ObjectContext::new_key()));
                                return Ok(());
                            },
                            Json::Array(_) => {
                                // 切换模式
                                self.mode=ObjectMode::ValueArray(ArrayContext::new_judgment_type());
                                return Ok(());
                            },
                        }
                    }
                }

            },
            ObjectMode::ValueNull(null_context) => {
                match null_context.check_null(c) {
                    Ok(isfinish) => {
                        if isfinish{
                            // 如果已经识别完
                            // 存入hashmap
                            self.obj.insert(self.key_tmp.clone(), Json::Null);
                            // 清空tmp
                            self.key_tmp=String::new();
                            // 切换模式
                            self.mode=ObjectMode::Douhao;
                        }
                        // 返回
                        return Ok(());
                    },
                    Err(_) => {
                        return Err(std::format!("null解析失败,实际上的字符:{}",c));
                    },
                }
            },
            ObjectMode::ValueTrue(context) => {
                match context.check_true(c) {
                    Ok(isfinish) => {
                        if isfinish{
                            // 如果已经识别完
                            // 存入hashmap
                            self.obj.insert(self.key_tmp.clone(), Json::Boolean(true));
                            // 清空tmp
                            self.key_tmp=String::new();
                            // 切换模式
                            self.mode=ObjectMode::Douhao;
                        }
                        // 返回
                        return Ok(());
                    },
                    Err(_) => {return Err(std::format!("true解析失败,实际上的字符:{}",c));},
                }
            },
            ObjectMode::ValueFalse(context) => {
                match context.check_false(c) {
                    Ok(isfinish) => {
                        if isfinish{
                            // 如果已经识别完
                            // 存入hashmap
                            self.obj.insert(self.key_tmp.clone(), Json::Boolean(false));
                            // 清空tmp
                            self.key_tmp=String::new();
                            // 切换模式
                            self.mode=ObjectMode::Douhao;
                        }
                        // 返回
                        return Ok(());
                    },
                    Err(_) => {return Err(std::format!("false解析失败,实际上的字符:{}",c));},
                }

            },
            ObjectMode::End => {
                // 不应该调用
                return Err(std::format!("这个Object已经解析完毕,不应该再次调用check_object函数"));
            },
            ObjectMode::Douhao => {
                match c {
                    ','=>{
                        // 下一个元素
                        self.mode=ObjectMode::Key(StringContext::new());
                        return Ok(());
                    }
                    '\r'|'\n'|'\x20'=>{
                        // 允许空格回车
                        return Ok(());
                    }
                    '}'=>{
                        // Object已经结束
                        self.mode=ObjectMode::End;
                        return Ok(());
                    }
                    _=>{
                        // 其他字符
                        return Err(std::format!("这里需要 ',' 但实际为{}",c));
                    }
                    
                }
            },
            ObjectMode::ValueString(context) => {
                match context.check_string(c) {
                    Ok(_) => {
                        // 如果解析完成
                        if let StringMode::end=context.get_mode(){
                            // 存入hashmap
                            self.obj.insert(self.key_tmp.clone(), Json::String(context.get_str().to_string()));
                            // 清空tmp
                            self.key_tmp=String::new();
                            // 切换模式
                            self.mode=ObjectMode::Douhao;
                        }
                        // 返回
                        return Ok(());
                    },
                    Err(_) => {return Err(std::format!("string解析失败,实际上的字符:{}",c));},
                }
            },
            ObjectMode::ValueArray(context) => {
                match context.check_array(c) {
                    Ok(_) => {
                        // 如果解析完成
                        if let ArrayMode::End =context.get_mode_ptr(){
                            // 存入hashmap
                            self.obj.insert(self.key_tmp.clone(), Json::Array(context.get_arr()));
                            // 清空tmp
                            self.key_tmp=String::new();
                            // 切换模式
                            self.mode=ObjectMode::Douhao;
                        }
                        // 返回
                        return Ok(());
                    },
                    Err(err) => {return Err(std::format!("array解析失败,错误原因{}",err));},
                }
            },
            ObjectMode::ValueNumber(context) => {
                match c {
                    // 关于浮点数的字符全存进去
                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'-'|'E'|'e'|'.'=>{
                        context.get_number_str_mut().push(c);
                        return Ok(());
                    }
                    _=>{
                        // 如果出现其他字符,说明浮点数已经到头了
                        match context.check_number("") {
                            Ok(_) => {
                                // 浮点数合法
                                // 存入hashmap
                                self.obj.insert(self.key_tmp.clone(), Json::Number(context.get_number()));
                                // 清空tmp
                                self.key_tmp=String::new();
                                // 切换模式
                                self.mode=ObjectMode::Douhao;
                                // 立即执行一次检测
                                return self.check_object(c);
                            },
                            Err(err) => {Err(std::format!("浮点数解析失败,实际的字符为:{},异常对象:{}",c,err))},
                        }
                    }
                }
            },
            ObjectMode::ValueObject(context) => {
                // 解析n+1obj
                context.check_object(c)?;
                // 判断解析是否完成
                if let ObjectMode::End=context.mode{
                    // 子obj解析完成
                    self.obj.insert(self.key_tmp.clone(), Json::Object(context.obj.clone()));
                    // 切换模式
                    self.mode=ObjectMode::Douhao;
                }
                return Ok(());

            },
        }

    }

    
}
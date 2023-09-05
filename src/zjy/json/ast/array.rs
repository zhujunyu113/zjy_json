/**
 * 文件名: "src/zjy/json/ast/array.rs" json解析器源代码
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

use crate::zjy::json::{Json, judgment_json_type, ast::string::StringMode};

use super::{null::NullMode, r#true::TrueMode, r#false::FalseMode, number::NumberContext, string::StringContext, object::{ObjectContext, ObjectMode}};
/**
 * 解析Array需要保存的上下文
 */
pub struct ArrayContext{
    /// 存储解析出来的json对象的数组
    arr:Vec<Json>,
    /// 存储解析对象的行为枚举
    mode:ArrayMode
}
/**
 * 存储解析对象的行为枚举
 * String代表需要数组的开始符号  : [
 * JudgmentType代表需要判断接下来的对象是什么类型
 *      null(NullMode),JsonTrue(TrueMode),
 *      JsonFalse(FalseMode),NumberContext(NumberContext),String(StringContext)
 * 代表识别处理的类型,并且保存了解析该类型所需的上下文
 * Douhao 代表这一个数组的对象已经解析完毕, 需要判断这个数组是结束还是有下一个对象
 * end代表这一个数组已经解析完毕
 */
pub enum ArrayMode {

    JudgmentType,Null(NullMode),JsonTrue(TrueMode),
        JsonFalse(FalseMode),NumberContext(NumberContext),String(StringContext),Array(Box<ArrayContext>),Object(Box<ObjectContext>),
    Douhao,
    End
}
#[cfg(test)]
#[test]
fn test_check_array(){
    use std::fs::read_to_string;

    

    let json_str=read_to_string("tests/testarray.json").unwrap();
    let json=Json::str_to_json(&json_str).unwrap();
    println!("json:\n{}",json);

}

impl ArrayContext {
    /**
     * 获取当前模式的借用,一般用于判断array的解析是否完成
     */
    pub fn get_mode_ptr<'a>(&'a self) ->&'a ArrayMode{
        return &self.mode;
    }
    /**
     * 复制获取vec
     */
    pub fn get_arr(&self) -> Vec<Json>{
        let mut result =Vec::with_capacity(self.arr.len());
        for j in 0..self.arr.len(){
            result.push(self.arr[j].clone());
        }
        return result;
    }
    /**
     * 将array上下文句柄中保存的数组提取成json对象,这将会消费自身来提高效率
     */
    pub fn into_json(self) ->Json{
        return Json::Array(self.arr);
    }
    
    /**
     * 初始化一个array上下文句柄
     */
    // pub fn new() -> ArrayContext{ ArrayContext { arr: Vec::new(), mode: ArrayMode::Start }}
    /**
     * 初始化一个array上下文句柄,并将ArrayMode字段指定为判断类型模式
     */
    pub fn new_judgment_type() -> ArrayContext{ ArrayContext { arr: Vec::new(), mode: ArrayMode::JudgmentType }}
    /**
     * 通过循环调用该函数的方式解析json数组,将上下文保存在句柄中
     * 如果数组不合法,返回Err
     * 如果数组解析完毕,会将ArrayMode字段设置为end,
     * 可以通过get_mode_ptr()函数获取,
     * 并通过这个字段判断是否解析完成
     * 如果解析完成,再次调用check_array()函数会返回Err
     */
    pub fn check_array(&mut self, c:char) ->Result<(),String>{
        match self.mode {
            ArrayMode::JudgmentType => {
                match c {
                    '\r'|'\n'|'\x20'=>{
                        return Ok(())
                    }
                    _=>{ 
                        // 根据不同的类型,走不同的分支,将上下文存入
                        match judgment_json_type(c)? {
                            Json::Null => {
                                self.mode=ArrayMode::Null(NullMode::u);
                                return Ok(());
                            },
                            Json::Boolean(b) => {
                                if b{
                                    self.mode=ArrayMode::JsonTrue(TrueMode::r);
                                } else {
                                    self.mode=ArrayMode::JsonFalse(FalseMode::a);
                                }
                                return Ok(());
                            },
                            Json::Number(_) => {
                                self.mode=ArrayMode::NumberContext(NumberContext::new_from_str(&c.to_string()));
                                return Ok(());
                            },
                            Json::String(_) => {
                                self.mode=ArrayMode::String(StringContext{ str: String::new(), mode: StringMode::str });
                                return Ok(());

                            },
                            Json::Object(_) => {
                                self.mode=ArrayMode::Object(Box::new(ObjectContext::new_key()));
                                return Ok(());
                            },
                            Json::Array(_) => {
                                self.mode=ArrayMode::Array(Box::new(ArrayContext::new_judgment_type()));
                                return Ok(());
                            },
                        };
                    }
                }
            },
            ArrayMode::Null(ref mut context) => {
                match context.check_null(c) {
                    Ok(ok) => {
                        if ok{
                            // true代表null识别完毕
                            self.mode=ArrayMode::Douhao;
                            // 存入数组
                            self.arr.push(Json::Null);
                        }
                        return Ok(());
                    },
                    Err(_err) => {
                        return Err(std::format!("null识别错误:非法字符:{}",c));
                    },
                };
            },
            ArrayMode::JsonTrue(ref mut context) => {
                match context.check_true(c) {
                    Ok(ok) => {
                        // true代表true识别完毕
                        if ok{
                            self.mode=ArrayMode::Douhao;
                            // 存入数组
                            self.arr.push(Json::Boolean(true));
                        }
                        return Ok(());
                    },
                    Err(_err) => {
                        return Err(std::format!("true识别错误:非法字符:{}",c));
                    },
                }
            },
            ArrayMode::JsonFalse(ref mut context) => {
                match context.check_false(c) {
                    Ok(ok) => {
                        if ok{
                            // 解析完毕
                            self.mode=ArrayMode::Douhao;
                            // 存入数组
                            self.arr.push(Json::Boolean(false));
                        }
                        return Ok(());
                    },
                    Err(_) => {
                        return Err(std::format!("false识别错误:非法字符:{}",c));
                    },
                }
            },
            ArrayMode::NumberContext(ref mut context) => {
                match c {
                    // 关于浮点数的字符全存进去
                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'-'|'E'|'e'|'.'=>{
                        context.get_number_str_mut().push(c);
                        return Ok(());
                    }
                    // 如果出现其他字符,说明浮点数已经到头了
                    _=>{
                        match context.check_number("") {
                            Ok(_) => {
                                // 如果浮点数合法
                                // 存入浮点数
                                self.arr.push(Json::Number(context.get_number()));
                                // 立即执行一次逗号检测,并且返回
                                self.mode=ArrayMode::Douhao;
                                return self.check_array(c);
                            },
                            Err(err) => {
                                return Err(std::format!("{}",err));
                            },
                        };
                    }
                }
            },
            ArrayMode::String(ref mut context) =>{
                match context.check_string(c) {
                    Ok(_) => {
                        // 判断是否解析string完成
                        if (*context.get_mode())==StringMode::end{
                            // 存入数组
                            self.arr.push(Json::String(context.get_str().to_string()));
                            // 解析完成
                            self.mode=ArrayMode::Douhao;
                        }
                        return Ok(());
                    },
                    Err(_) => {
                        return Err(std::format!("string识别错误:非法字符:{}",c));
                    },
                }
            },
            ArrayMode::Douhao => {
                match c {
                    ','=>{
                        // 紧接着下一个内容
                        self.mode=ArrayMode::JudgmentType;
                        return Ok(());
                    }
                    '\n'|'\r'|'\x20' =>{
                        // 允许空格和换行
                        return Ok(());
                    }
                    ']'=>{
                        // 结束
                        self.mode=ArrayMode::End;
                        return Ok(());
                    }
                    _=>{
                        // 异常
                        return Err(std::format!("此处需要',' 但实际为{}",c));
                    }
                    
                }
            },
            ArrayMode::End => {
                return Err(std::format!("array已经解析完毕,不应该再次调用该函数"));
            },
            /*
            逐层传递字符串,递归调用,我觉得没必要优化成循环,
            一般不会有维度非常高的数组,一般最多3层封顶,循环不是很好组织代码
            相比循环,递归,每一维数组都需要压一个栈,并且需要将字符串逐层传递到最后一维
            只要数组维度不深,几乎不影响效率
             */
            ArrayMode::Array(ref mut context) => {
                // 解析1+n级数组
                context.check_array(c)?;
                // 判断这个1+n级数组解析完成了吗
                if let ArrayMode::End=context.get_mode_ptr(){
                    // 子数组解析完成,把这个数组里面的东西拿出来
                    self.arr.push(Json::Array(context.get_arr()));
                    self.mode=ArrayMode::Douhao;
                }
                return Ok(());
                // 否则继续解析子数组
            },
            ArrayMode::Object(ref mut context) => {
                // 解析obj
                match context.check_object(c) {
                    Ok(_) => {
                        if let ObjectMode::End=context.get_mode_ptr(){
                            // 解析完成
                            self.arr.push(context.get_json());
                            self.mode=ArrayMode::Douhao;
                        }
                        return Ok(());
                    },
                    Err(err) => {
                        return Err(err);
                    },
                }
            },
            
        }

    }
    
}
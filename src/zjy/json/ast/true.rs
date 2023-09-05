/**
 * 文件名: "src/zjy/json/ast/true.rs" json解析器源代码
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
 * 用于保存识别true的状态的结构体
 */
pub enum TrueMode{
    t,r,u,e
}
impl TrueMode {
    #[allow(dead_code)]
    #[cold]
    /**
     * 初始化一个TrueMode句柄
     */
    pub fn new() ->TrueMode{ TrueMode::t }
    #[allow(dead_code)]
    /**
     * 循环调用这个函数来判断true
     * # 参数 :c:char  要判断的"true"的下一个字符
     * # 返回值: ok(bool) :true代表true判断完毕 false代表true判断还没完成
     * # Err :true不合法就返回Err
     * 其中下一个要检测什么的值保存在枚举中
     */
    pub fn check_true(&mut self, c:char) ->Result<bool,()>{
        match self {
            TrueMode::t =>{
                match c {
                    't'=>{
                        *self=TrueMode::r;
                        return Ok(false);
                    },
                    // 允许换行和空格
                    '\r'|'\n'|'\x20'=>{
                        return Ok(false);
                    }
                    _=>{
                        *self=TrueMode::t;
                        return Err(());
                    },
                }
            },
            TrueMode::r => {
                match c {
                    'r'=>{
                        *self=TrueMode::u;
                        return Ok(false);
                    },
                    _=>{
                        *self=TrueMode::t;
                        return Err(());
                    },
                }
            },
            TrueMode::u => {
                match c {
                    'u'=>{
                        *self=TrueMode::e;
                        return Ok(false);
                    },
                    _=>{
                        *self=TrueMode::t;
                        return Err(());
                    },
                }
            },
            TrueMode::e => {
                match c {
                    'e'=>{
                        *self=TrueMode::t;
                        return Ok(true);
                    },
                    _=>{
                        *self=TrueMode::t;
                        return Err(());
                    },
                }
            },
        };

    }
    
}

#[cfg(test)]
fn test_check_true(t:&str){
    let mut true_conntest = TrueMode::new();
    for c in t.chars(){
        match true_conntest.check_true(c) {
            Ok(isfinish) =>{
                if isfinish{
                    println!("true");
                }
            },
            Err(_) => {
                panic!("true失败");
            },
        }
    }
}
#[cfg(test)]
#[test]
fn test_check_true_pass(){
    test_check_true("\n\r\n\n true");
}
#[cfg(test)]
#[test]
#[should_panic]
fn test_check_true_painc(){
    test_check_true("TRUE");
}

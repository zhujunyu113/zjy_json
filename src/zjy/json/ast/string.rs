/**
 * 文件名: "src/zjy/json/ast/string.rs" json解析器源代码
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


#[allow(dead_code)]
/**
 * 用于验证json中string类型合法性的上下文
 */
pub struct StringContext{
    /// 已经识别到的json中的sting
    pub(crate) str:String,
    /// 识别string的模式
    pub(crate) mode:StringMode
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
/**
 * 指定string识别模式的枚举
 * start表示这个string刚刚开始,应该由 引号:"  开始
 * str是表示string的正文
 * zhuanyi表示当前在转义模式
 * zhuanyi_utf表示当前在unicode转义模式,(u8,[u16;2])元组用于存储识别unicode所需的上下文
 * end表示字符串判断已经完成
 */
pub enum StringMode {
    start,str,zhuanyi,zhuanyi_utf(u8,[u16;2]),end
    
}
#[cfg(test)]
#[test]
fn test_check_string(){
    let str=std::fs::read_to_string("tests/teststring.json").unwrap();
    let mut context=StringContext::new();
    for c in str.chars(){
        context.check_string(c).unwrap();
        if context.mode==StringMode::end{
            // 结束
            println!("json:{}",context.str);
            return;
        }
    }
}
impl StringContext {
    /**
     * 获取str字段的借用
     */
    pub fn get_str<'a>(&'a self) -> &'a str { return &self.str; }
    /**
     * 获取mode字段的借用
     */
    pub fn get_mode<'a>(&'a self) -> &'a StringMode { return &self.mode; }
    /**
     * 初始化一个StringContext
     */
    pub fn new() -> StringContext{ StringContext { str: String::new(), mode: StringMode::start }}
    #[allow(dead_code)]
    /**
     * 通过循环调用的方式解析string,如果句柄中的mode字段是end值,代表已经解析完毕
     * 如果string格式错误,返回Err
     */
    pub fn check_string(&mut self, c:char) ->Result<(),()>{
        match self.mode {
            StringMode::start => {
                match c {
                    '"'=>{
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    // 允许空格回车
                    '\r'|'\n'|'\x20'=>{
                        return Ok(());
                    }
                    _=>{
                        return Err(());
                    }
                }
            },
            StringMode::str => {
                match c {
                    '"'=>{
                        // 引号代表结束
                        self.mode=StringMode::end;
                        return Ok(());
                    }
                    '\\'=>{
                        self.mode=StringMode::zhuanyi;
                        return Ok(());

                    }
                    
                    // \x08=\b  \x0c=\f  
                    '\x08'|'\x0c'|'\n'|'\r'|'\t'=>{
                        //必须通过转义的方式表示这些字符
                        self.mode=StringMode::end;
                        return Err(());
                    }
                    _=>{
                        // 正常存入
                        self.str.push(c);
                        return Ok(());
                    }
                    
                }
            }, 
            StringMode::zhuanyi => {
                match c {
                    '"'|'\\'|'/'=>{
                        // 转义"
                        self.str.push(c);
                        // 切回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    'b'=>{
                        // 转义"
                        self.str.push('\x08');
                        // 切回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    'f'=>{
                        // 转义"
                        self.str.push('\x0c');
                        // 切回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    'n'=>{
                        // 转义"
                        self.str.push('\n');
                        // 切回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    'r'=>{
                        // 转义"
                        self.str.push('\r');
                        // 切回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    't'=>{
                        // 转义"
                        self.str.push('\t');
                        // 切回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                    'u'=>{
                        // 切换到Unicode模式
                        self.mode=StringMode::zhuanyi_utf(0,[0;2]);
                        return Ok(());

                    }
                    _=>{
                        // 切回去
                        self.mode=StringMode::end;
                        // 报错
                        return Err(());
                    }
                    
                }
            },
            StringMode::zhuanyi_utf(ref mut ptr,ref mut point) => {
                // 第1至4个16进制字符
                if (0==*ptr)||(1==*ptr)||(2==*ptr)||(3==*ptr){
                    let mut tmp:u16=match char_to_u8_16hx(c) {
                        Ok(i) => {i.into()},
                        Err(_) => {return  Err(());},
                    };
                    // 循环乘,一次比一次少
                    for _x in 0..(3-(*ptr)){
                        tmp=tmp*16;
                    }
                    //加进来
                    point[0] = point[0] + tmp;
                    // 如果已经是第4个,需要判断有没有下一个低4位
                    // 如果码点不在U+D800到U+DBFF
                    if (3==*ptr) && (!((0xD800 <= point[0]) && (point[0] <= 0xDBFF))){
                        // rust char类型其实就是unicode的裸码点
                        self.str.push(unsafe { char::from_u32_unchecked(point[0].into()) });
                        // 切换回去
                        self.mode=StringMode::str;
                        return Ok(());
                    }
                } 
                // ptr=4
                if *ptr==4{
                    if c!='\\'{
                        self.mode=StringMode::end;
                        return Err(());
                    }
                }
                // ptr==5
                if *ptr==5{
                    if c!='u'{
                        self.mode=StringMode::end;
                        return Err(());
                    }
                }
                // ptr==6,这里是第二个转义序列
                if (6==*ptr)||(7==*ptr)||(8==*ptr)||(9==*ptr){
                    // 换成数字
                    let mut tmp:u16=match char_to_u8_16hx(c) {
                        Ok(i) => {i.into()},
                        Err(_) => {
                            self.mode=StringMode::end;
                            return Err(());
                        },
                    };
                    // 循环乘16
                    for _x in 0..9-*ptr{
                        tmp=tmp*16;
                    }
                    // 加进去
                    point[1] = point[1] + tmp;
                    // 判断点
                    if 9==*ptr{
                        // 低代理项必须在0xDC00到0xDFFF之间
                        if (0xDC00 <= point[1]) && (point[1] <=0xDFFF){
                            // 计算码点
                            let h:u32=point[0].into();
                            let l:u32=point[1].into();
                            let c_point:u32=0x10000 + ((h-0xD800) * 0x400) + (l-0xDC00);
                            // 由于已经检测代理码点的范围,码点是合法的,不需要检查
                            let ch=unsafe { char::from_u32_unchecked(c_point) };
                            // 存入
                            self.str.push(ch);
                            // 切换
                            self.mode=StringMode::str;
                            // 返回
                            return Ok(());
                        } else {
                            self.mode=StringMode::end;
                            return Err(());
                        }
                    }
                }
                if 10 <= *ptr{
                    // 不应该大于10
                    panic!("{}{}","ptr不应该大于10,如果大于10应该有bug,实际ptr=",*ptr);
                }

                // 循环计数器加一
                *ptr=*ptr+1;
                return Ok(());
            }
            StringMode::end => {
                // 已经判断完毕,不应该再次调用
                return Err(());
            },
            
        }


    }
    
}
/**
 * 将16进制字符串转换为整形数字
 */
fn char_to_u8_16hx(c:char)->Result<u8,()>{
    match c {
        '0'=>{Ok(0)},
        '1'=>{Ok(1)},
        '2'=>{Ok(2)},
        '3'=>{Ok(3)},
        '4'=>{Ok(4)},
        '5'=>{Ok(5)},
        '6'=>{Ok(6)},
        '7'=>{Ok(7)},
        '8'=>{Ok(8)},
        '9'=>{Ok(9)},
        'a'|'A'=>{Ok(10)},
        'b'|'B'=>{Ok(11)},
        'c'|'C'=>{Ok(12)},
        'd'|'D'=>{Ok(13)},
        'e'|'E'=>{Ok(14)},
        'f'|'F'=>{Ok(15)},
        _=>{Err(())},
    }
}
/**
 * 实现StringMode枚举的 == != 运算符
 */
impl PartialEq for StringMode {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
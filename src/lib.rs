//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

// 判断是否不启用标准库
#![cfg_attr(any(feature = "no_std",features = "alloc"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(not(any(feature = "no_std",features = "alloc")))]
extern crate std as core;

// 单元测试
#[cfg(test)]
mod tests;

/// Guuid 唯一标识符
/// 
/// **注意**
/// 如果想要储存，有两种方式
/// - 储存为字符串
/// - 储存为字节数组
/// 
/// 即使用
/// 
/// - 使用 `to_string` 及其对应函数 `from_string`
/// - 使用 `to_bytes` 及其对应函数 `from_bytes`
#[derive(Clone,Eq,Debug)]
pub struct Guuid{
    /// 以毫秒为单位的UNIX时间戳
    time: u64,

    /// 随机数
    random : u64
}

impl PartialEq for Guuid {
    /// 比较两个Guuid是否是相同
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time 
        && 
        self.random == other.random
    }
}


impl Guuid{

    /// 转换到u128
    /// 
    /// - 高64位:时间戳
    /// - 低64位:随机数
    pub fn as_u128(&self) -> u128 {
        let mut result = self.time as u128;
        result = (result << 64) + (self.random as u128);
        result
    }

    /// 获取时间戳
    pub fn time(&self) -> u64 {
        self.time
    }

    /// 获取随机数
    pub fn random(&self) -> u64 {
        self.random
    }

    /// 从时间戳和随机数中构造Guuid
    pub fn new(time: u64, random: u64) -> Guuid{
        Guuid{
            time: time,
            random: random
        }
    }

    /// 从u128中构造Guuid
    /// 
    /// | 值 | 定位 |
    /// | :---: | :---: |
    /// | time | 高64位 |
    /// | random | 低64位 |
    pub fn from_u128(u128: u128) -> Guuid {
        Guuid{
            time: (u128 >> 64) as u64,
            random: u128 as u64
        }
    }

    /// 从16进制字符串中构造Guuid
    /// 
    /// 仅在std或者alloc环境下可用
    #[cfg(not(feature = "no_std"))]
    pub fn from_string(str: &str) -> Option<Guuid> {
        use base32::decode;
        use core::convert::TryInto;

        match decode(base32::Alphabet::Crockford, str){
            Some(ok) => Some(Guuid::from_bytes(ok.try_into().unwrap())),

            None => None
        }
    }

    /// 将Guuid转换为字符串
    /// 
    /// 仅在std或alloc环境下可用
    #[cfg(not(feature = "no_std"))]
    pub fn to_string(&self) -> String {
        use base32::encode;

        encode(base32::Alphabet::Crockford, &self.to_bytes())
    }
    /// 生成guuid
    /// 
    /// 使用给定时间戳
    /// 
    /// 仅在std环境下使用
    #[cfg(feature = "std")]
    pub fn gen_guuid_from_time(time : u64) -> Guuid{
        use rand::RngCore;

        // 使用操作系统产生的随机数
        let random = rand::rngs::OsRng.next_u64();

        // 构造Guuid
        Guuid::new(time,random)
    }

    /// 生成guuid
    /// 
    /// 使用系统当前时间戳
    /// 
    /// 仅在std环境下使用
    #[cfg(feature = "std")]
    pub fn gen_guuid() -> Result<Guuid,std::time::SystemTimeError>{
        use core::convert::TryInto;

        match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Guuid::gen_guuid_from_time(n.as_millis().try_into().unwrap())),
            Err(err) => Err(err),
        }
    }

    /// 转换为字节数组
    pub fn to_bytes(&self) -> [u8;16]{
        // 大端序
        self.as_u128().to_be_bytes()
    }

    /// 从字节数组中构造Guuid
    pub fn from_bytes(bytes: [u8;16]) -> Guuid {
        // 大端序
        Guuid::from_u128(u128::from_be_bytes(bytes))
    }
}
//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 单元测试文件

use super::*;

    /// 检查from u128一致性
    #[test]
    fn eq_test_from_u128() {
        let first = Guuid::from_u128(100);
        let second = Guuid::from_u128(first.as_u128());

        assert_eq!(first, second);
    }

    /// 检查from bytes一致性
    #[test]
    fn eq_test_from_bytes() {
        let first = Guuid::from_bytes(*b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let second = Guuid::from_bytes(first.to_bytes());

        assert_eq!(first, second);
    }
    

    /// 检查from str一致性
    #[test]
    fn eq_test_from_str() {
        let first = Guuid::from_string("1234567890abcdef").unwrap();
        let second = Guuid::from_string(&first.to_string()).unwrap();

        assert_eq!(first, second);
    }

    /// 检查new一致性
    #[test]
    fn eq_test_new(){
        let first = Guuid::new(100,200);
        let second = Guuid::new(first.time(),first.random());

        assert_eq!(first, second);
    }




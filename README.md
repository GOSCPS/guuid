# GUUID Project
GUUID是GUID和UUID的完美替代品。

 - 简单易用 ✔
 - 易于移植 ✔
 - 跨平台   ✔
 - UUID、GUID的128位兼容 ✔
 - 不在简单的问题，如大小写上设置绊脚石 ✔

## 标准
```
  |  00002YPM7N3N9  | A5XP0NRR46QB4 |
  |  Unix毫秒级时间戳 | 安全随机数       |
  |  64位无符号整数   | 64位无符号整数  |
  |  大端序储存       | 大端序储存      |
  |  字符串使用Crockford Base32储存   |
```

## Features
 - std
   - 请在可使用标准库的情况下开启
 - alloc
   - 请在可以使用alloc的情况下开启
 - no_std
   -  请在不能使用alloc和std的情况下开启

以上三者只能同时开启一个。
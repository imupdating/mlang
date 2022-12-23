# mlang
## 一个解释型**编程语言**！

>特性
- 语法简单
- 有现代的第三方库管理工具(尚未开发)
- 编译器代码简单，可被嵌入
- 解释型语言，即时运行

> 运行

您可以尝试*cargo run*
来测试test.ms，您可以查看src\main.rs中的逻辑，test.ms是固定的(为了方便测试)，您可以随意更改，但不能商用

> 语法示例

部分的语法展示：
```
main {
    print("mlang Test");
    print_twice(1+1);
    result = plus(1, 2);
    print(result);
}

do print_twice(e) {
    repeat 2 {
        print(e);
    }
}

fn plus(a, b) = a + b;
```

只有这个例子可以跑起来...(作者太懒了(不是))
```
let b = 1;

fn plus(a, b) {
    return a;
}

fn add(b, d) {
    return b;
}

fn minus (k, h) {}

fn print(e) {
    
}
```

-----------
> 联系方式：QQ:3272295385

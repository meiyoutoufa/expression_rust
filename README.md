## expression-rust

实现表达式判断


使用window编译c,将expression.dll置于在c项目的main.c在同一目录下
main.c代码参考
```c

#include <stdio.h>
#include <stdbool.h>

// 声明 Rust 中的函数
extern bool evaluate_expression(const char* expr);

// 声明 Rust 函数
extern char** infix_to_postfix(const char* expr);
extern void free_postfix_result(char** result);

void test_infix_to_postfix();

void test_evaluate_expression();

int main() {
    //测试表达式
    test_evaluate_expression();

    //测试中序赚后续的字符串
    test_infix_to_postfix();



    return 0;
}

void test_evaluate_expression() {
    const char* expr = "10 + 5 * ( 20 - 10 / 2 + 1 ) - 100 / 10 > 30 && 30 < 100 || 20 * 3 > 100 || 1 != 100";  // 示例表达式
    bool result = evaluate_expression(expr);

    printf("Result: %s\n", result ? "true" : "false");
}

void test_infix_to_postfix() {
    const char* expr2 = "10 + 5 * ( 20 - 10 / 2 + 1 ) - 100 / 10 > 30 && 30 < 100 || 20 * 3 > 100 || 1 != 100";
    char** result2 = infix_to_postfix(expr2);

    // 假设打印结果
    int i = 0;
    while (result2[i] != NULL) {
        printf("%s\n", result2[i]);
        i++;
    }

    // 释放内存
    free_postfix_result(result2);
}


```

1.计算表达式的结果，引入extern bool evaluate_expression(const char* expr);，会返回true或者false

2.表达式中序转成后序，引入extern char** infix_to_postfix(const char* expr);和extern void free_postfix_result(char** result);
infix_to_postfix会返回转成后续的字符，同时之后要调用free_postfix_result()，来释放内存。

编译
```shell
##编译
gcc main.c -L. -lexpression -o fnrun

##运行
./fnrun
```
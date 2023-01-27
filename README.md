### Lisp
个人实现的Lisp语言解释器，可以在我们本地跑Lisp语法

### 什么是Lisp 
#### 关于Lisp有很多种说法
- 如果你会用Lisp语言，你就会拥有超人的编程能力

- Lisp和Fortran代表了编程语言发展的两大方向。前者的基础是数学，后者的基础是硬件架构

- Lisp这一类语言迎合的是数学家的本性，而不是程序员的本性

- Lisp实际上是数学，而数学是不会过时的

- Lisp并不只是说代码上的抽象，相对于你学习java之后，再去看Lisp，就会颠覆你对原本程序世界的认知，有种重塑三观的感觉

- Lisp是适合人工智能（AI）的语言

- Lisp是很好的思维训练的载体，以λ演算而不是图灵机作为计算模型对于初学者是一种思维上的飞跃，有很高的学习价值

- Lisp的语法是世界上最精炼、最美观，也是语法分析起来最高效的语法

- Lisp 是第一个可以在程序的任何位置定义函数，并且可以把函数作为值传递的语言。这样的设计使得它的表达能力非常强大。这种理念被Python，JavaScript，Ruby等语言所借鉴

- Lisp是世界上最强大的宏系统。这种宏系统的表达力几乎达到了理论所允许的极限。如果你只见过C语言的宏，那我可以告诉你它是完全无法与Lisp的宏系统相提并论的

- Lisp是世界上第一个使用垃圾回收的语言。这种超前的理念，后来被Java、C#等语言借鉴

安装步骤
1. 安装rust环境： $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
安装成功将会出现以下字幕：Rust is installed now. Great!

2. 安装c语言编译器(非必需) $ xcode-select --install
 检查是否安装成功 rustc -V cargo -V, 如果出现以下字幕，则安装成功
```rust
zhaozhenhang@zhaozhenhangdeMacBook-Pro rust % rustc -V
rustc 1.64.0-nightly (87588a2af 2022-07-13)
zhaozhenhang@zhaozhenhangdeMacBook-Pro rust % cargo -V
cargo 1.64.0-nightly (b1dd22e66 2022-07-09)
zhaozhenhang@zhaozhenhangdeMacBook-Pro rust %
```


3. 创建文件夹：mkdir -p /tmp/home/rust, 然后进入该目录下 cd /tmp/home/rust

4. clone项目：git@github.com:azhsmesos/lisp-interpreter.git
   branch是 main

5. 进入项目文件夹，编译：cd lisp-interpreter, 然后执行： cargo clean 和 cargo build构建项目

6. 添加环境变量: 我的电脑配置文件是 ～/.zshrc ,所以步骤这样：
   1. `vim ~/.zshrc` 
   2. 然后输入： `export PATH="/tmp/home/rust/lisp-interpreter/target/debug:$PATH" `
   3. 然后执行 `source ~/.zshrc`

注意，某些系统可能是profile文件,
其目录在/etc/profile下，输入数据
和zshrc一样。
7. 然后就可以使用命令了，heitx等同于我们平常使用的vim, 该terminal支持Ctrl+s保存，Ctrl+c退出，搜索、等功能，比较方便。
我们直接item2输入lisp-rs，就可以使用lisp语法了
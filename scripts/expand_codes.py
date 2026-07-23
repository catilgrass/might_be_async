# 0. 检查安装 cargo-expand 并快速失败
# 1. 检查并创建目录
#
# .temp/Cargo.toml 内容只有一个简单的[package]和一个空[workspace]（这很关键），还有相对依赖（../）的might_be_async
#
# 然后读取 doc/usage 下所有结尾不是_expand.rs的.rs文件，执行循环，过程如下：
#
# 1. 将内容写入.temp/src/lib.rs，并加入内容
#
# const EXPAND_BEGIN: () = ();
# 源码
# const EXPAND_END: () = ();
#
# 2. 执行 cargo expand，将const EXPAND_BEGIN: () = (); ... const EXPAND_END: () = ();的内容截取
#
# 3. trim 字符串，写入 doc/usage/源文件名_expand.rs

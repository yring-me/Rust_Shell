## 项目说明

本项目用于学习Rust语言，使用Rust完成一个简单的shell终端



## 目前进展

> ls    命令 支持 -l -a
>
> cd    命令
>
> mkdir 命令 支持 -p
>
> pwd   命令
>
> rm    命令
> 
> clear 命令
> 
> touch 命令


## 新增命令步骤

1. 在src/config.rs中找到此数组，在该数组中增加命令名字及命令对应处理函数

![](https://yring-me.oss-cn-beijing.aliyuncs.com/test/%E6%88%AA%E5%B1%8F2024-02-16%2016.32.33.png)

2. 命令处理函数应新增一个文件，名字为`cmd.rs`，在此文件中完成命令实现

![](https://yring-me.oss-cn-beijing.aliyuncs.com/test/%E6%88%AA%E5%B1%8F2024-02-16%2016.35.03.png)
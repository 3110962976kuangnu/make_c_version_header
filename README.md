# 生成一个 c语言头文件 用于提供编译时间戳和版本号信息

用法：
运行程序时 会检查当前目录下是否存储 一个名为“build_info.h”的文件，如果不存在   程序会在当前目录下生成一个 build_info.h 的文件 文件内容包括运行时的时间戳（秒数）和一个默认的 语义化版本号的三个值 1.0.0。
如果已经存在这个文件了，那么程序会将文件中第三行的内容替换成 `#define BUILD_TIME_STAMP 时间戳数字` 

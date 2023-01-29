# xlsx2doc
将xlsx表格中的每一条数据按照模版转换成doc文件的批处理程序，支持图片链接下载。

## 使用
1. 打包程序为可执行文件
2. 将需要处理的xlsx文件命名为target.xlsx并放置于可执行文件同级目录
3. 编写config.json并放置于可执行文件同级目录
4. 编写template.docx并放置于可执行文件同级目录
5. 执行可执行文件，并得到输出文件夹output

> `target.xlsx`、`config.json`、`template.docx` 必须要有，格式和作用直接看example就懂了。

## 其他
docx-rs这个库有点问题，以本地引入的方式引入

``` toml
docx-rs = { path = "./docx-rs-0.3.4" }
```

resolve-xlsx-link是一个单独的包，用来处理xlsx文件中HYPERLINK的问题（将点击跳转的目标地址转换为显示的文本）

这个项目是很久之前的了，只是最近才放到github上，所以代码中有一些问题没有处理的很好，但正常使用是没问题的，如果遇到问题可以提issue或者邮箱联系我

编译release的时候会卡在一个库上，一直没找到原因，记录一下

## 许可

[MIT](./LICENSE)

Copyright (c) 2022 Boda Lü

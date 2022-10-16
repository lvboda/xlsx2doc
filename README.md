# xlsx2doc

xlsx2doc is a program that converts xlsx file data into documents.

## Explain

There are some problems with this version of the docx-rs,so I downloaded it locally and made some changes and finally it's introduced this way.

``` toml
docx-rs = { path = "./docx-rs-0.3.4" }
```

resolve-xlsx-link is a separate package,that resolves HYPERLINK to xlsx files.

How to use please refer to the example.

## License

[MIT](https://github.com/lvboda/xlsx2doc/blob/master/LICENSE)

Copyright (c) 2022 Boda Lü

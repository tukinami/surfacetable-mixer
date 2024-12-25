# surfacetable-mixer

[GitHub repository](https://github.com/tukinami/surfacetable-mixer)

## これは何?

YAMLから、デスクトップマスコット「伺か」の設定ファイルの一つである`surfacetable.txt`を生成するCLIアプリケーションです。

YAMLの書式は、apxxxxxxeさんの[surfaces-mixer](https://github.com/apxxxxxxe/surfaces-mixer)のものに従います。

## 使い方

```
Usage: surfacetable-mixer.exe [OPTIONS]

Options:
  -i, --input <INPUT>          Path to input file [default: ./surfaces.yaml]
  -o, --output <OUTPUT>        Path to output file [default: ./surfacetable.txt]
  -f, --force                  Flag of force overwriting
  -w, --whitelist <WHITELIST>  Whitelist for surfaces, separated by comma
  -s, --separator <SEPARATOR>  Separator string for a parts of the surface [default: -]
  -h, --help                   Print help
  -V, --version                Print version
```

## 使用ライブラリ

いずれも敬称略。ありがとうございます。

+ [clap](https://github.com/clap-rs/clap) / rust-cli/Maintainers, clap-rs/Admins, Kevin K.
+ [serde](https://github.com/serde-rs/serde) / Erick Tryzelaar,David Tolnay
+ [serde\_yml](https://github.com/sebastienrousseau/serde_yml) / Serde YML Contributors


## コード参考・引用

敬称略。ありがとうございます。

+ [surfaces-mixer](https://github.com/apxxxxxxe/surfaces-mixer) / apxxxxxxe

## ライセンス

MITにて配布いたします。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)

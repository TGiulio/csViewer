# csViewer

csViewer is a super simple csv file reader which shows the content of a csv file in a table layout. It is my first exercise with Tauri.

I came up with it after having some problems with csv formatting of excel, so I made it an excuse to start exploring Tauri.

The style is super simple, it does not help you navigating folders, you have to write the path yourself.
It works only with Unicode characters, the separator is ";" (I'm sorry, this is the standard I have).

It adds rows' numbers as first column

![csViewer](https://i.ibb.co/278Cwhw/csViewer.png)

## installation

I highly recommend to visit [tauri](https://tauri.app/v1/guides/getting-started/prerequisites) website and documentation before using this repo, I assume you have the tauri cli installed when downloading.

Once downloaded you can run

```
cargo tauri dev
```

from the src-tauri/ directory of this repository to compile it and run it unoptimized in debug mode.

To have the installation file, the only thing you should need is executing

```
cargo tauri build
```

from the src-tauri/ directory.
This will compile the installation file you can find inside the src-tauri/target/release/ directory.

## Todos

- allow customizable separator/auto detection of separator
- allow folder navigation to select files

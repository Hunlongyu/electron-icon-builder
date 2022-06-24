electron-icon-builder
=================

[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![Version](https://img.shields.io/npm/v/oclif-hello-world.svg)](https://npmjs.org/package/oclif-hello-world)
[![CircleCI](https://circleci.com/gh/oclif/hello-world/tree/main.svg?style=shield)](https://circleci.com/gh/oclif/hello-world/tree/main)
[![Downloads/week](https://img.shields.io/npm/dw/oclif-hello-world.svg)](https://npmjs.org/package/oclif-hello-world)
[![License](https://img.shields.io/npm/l/oclif-hello-world.svg)](https://github.com/oclif/hello-world/blob/main/package.json)

# Global Usage
```sh-session
npm i -g @hunlongyu/electron-icon-builder
```

```bash
eib -i relative/path/file.png -o relative/path/folder
```

# Local Usage
```bash
npm i @hunlongyu/electron-icon-builder

// package

"scripts": {
    "icon": "eib  -i relative/path/file.png -o relative/path/folder"
}

npm run icon
```

# Flags
```
--input,    -i [String]   [Default: './icon.png'] Path to PNG file, 
--output,   -o [String]   [Default: './build/']   Folder to create files, 
--flatten,  -f [Boolean]  [Default: false]        Flatten output struture
```

# Recommendations
Input file should be 1024px x 1024px or larger. Make sure it is a 1 to 1 aspect ratio on width to height.

# Output structure
```
[output dir]
    -[build]
        -[win]
            -icon.ico
        -[mac]
            - icon.icns
        -[png]
            - 16x16.png
            - 32x32.png
            - 18x18.png
            - 64x64.png
            - 128x128.png
            - 512x512.png
```

When flatten option is enabled

```
[output dir]
    -[build]
        - icon.icns
        - icon.ico
        - 16x16.png
        - 32x32.png
        - 48x48.png
        - 64x64.png
        - 128x128.png
        - 512x512.png
```

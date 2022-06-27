electron-icon-builder
=================

<p align="center">
	<img src="https://forthebadge.com/images/badges/built-with-love.svg">
<p>
<p align="center">
<img src="https://github.com/aleen42/badges/raw/master/src/visual_studio_code_flat_square.svg?sanitize=true">
<img src="https://github.com/aleen42/badges/raw/master/src/typescript_flat_square.svg?sanitize=true">
</p>
<p align="center">
<img alt="npm" src="https://img.shields.io/npm/dt/@hunlongyu/electron-icon-builder?style=for-the-badge">
<img alt="GitHub" src="https://img.shields.io/github/license/Hunlongyu/eib?style=for-the-badge">
<p>

# Global Usage
```bash
// install
npm i -g @hunlongyu/electron-icon-builder

// usage
eib -i relative/path/file.png -o relative/path/folder
```

# Local Usage
```bash
// install
npm i -D @hunlongyu/electron-icon-builder

// add a srcipt in package.json file
"scripts": {
    "icon": "eib  -i relative/path/file.png -o relative/path/folder"
}

// usage
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

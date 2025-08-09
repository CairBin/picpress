# picpress

## Description

A command tool for image compression and conversion.

## Supproted Formats

* png
* jpeg/jpg
* webp
* avif

## Usage

Convert the format

```sh
picpress -i a.png -o b.jpeg
```

Compress the image quality(percentage)
```sh
picpress -i a.png -o b.jpeg -q 80
```

Resize the picture(default style is fit)
```sh
picpress -i a.png -o b.jpeg -r 800x600
```

Specify the style for picture resize, supported fill/fit/exact
```
picpress -i a.png -o b.jpeg -r 800*600 -m fill
```

For more details:
```
picpress --help
```
<h1 align="center">Roll-It</h1>

| ![Screenshot](data/screenshot_light.png) | ![Screenshot](data/screenshot_dark.png) |
|------------------------------------------|-----------------------------------------|

## Roll the dice

Simple dice-rolling app designed for [elementary OS]. Simulate the roll of a die with a configurable maximum value.

## Building

Roll-It is designed for and developed on [elementary OS]. You'll need the following dependencies to build it yourself:

* libgranite-dev
* libgtk-3-dev
* meson
* valac

To install them on [elementary OS]:

```shell
sudo apt install elementary-sdk
```

To build and install Roll-It:

```shell
meson build --prefix=/usr
cd build
ninja
sudo ninja install
```

[elementary OS]: https://elementary.io

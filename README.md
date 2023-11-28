# Pixman-rs

This project contains rust bindings for [`pixman`](https://www.pixman.org/).

From the official pixman docs:

> Pixman is a low-level software library for pixel manipulation, providing features such as image compositing and trapezoid rasterization. 
> Important users of pixman are the cairo graphics library and the X server.
> 
> Pixman is implemented as a library in the C programming language. It runs on many platforms, including Linux, BSD Derivatives, MacOS X, and Windows.
> 
> Pixman is free and open source software. It is available to be redistributed and/or modified under the terms of the MIT license. 

## Currently unsupported features

- Indexed image
- Glyph cache
- Separable convolution filter
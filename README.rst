Rust X264 
================


.. contents::


rust-bindgen
-----------------

.. code:: bash

	/Users/luozijun/.multirust/toolchains/stable/cargo/bin/bindgen --link x264 --builtins --convert-macros  /usr/local/Cellar/x264/r2601/include/x264.h > src/ffi/x264.rs
	/Users/luozijun/.multirust/toolchains/stable/cargo/bin/bindgen --link png  --builtins --convert-macros /usr/local/Cellar/libpng/1.6.23/include/png.h > src/ffi/png.rs
	/Users/luozijun/.multirust/toolchains/stable/cargo/bin/bindgen --link vpx  --builtins --convert-macros /usr/local/Cellar/libvpx/1.5.0/include/vpx/vp8.h > src/ffi/vp8.rs
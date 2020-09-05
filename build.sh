#!/bin/bash
export CMAKE_MODULE_PATH=deps/opencv-4.3.0/installed/lib/cmake/opencv4
export LD_LIBRARY_PATH="deps/opencv-4.3.0/build/lib:$LD_LIBRARY_PATH"

cargo build

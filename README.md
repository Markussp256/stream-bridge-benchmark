# stream-bridge-benchmark

A benchmark project evaluating different strategies for including C++ stream headers in large codebases.

## Overview

This repository contains a benchmark of three different approaches to managing stream-related includes in a demo project. The demo consists of **100 classes**, each with a single method, simulating a typical mid-sized C++ codebase.

### Benchmark Configurations

1. **Baseline**
   - Each class header includes `<iosfwd>`.
   - Each class source file includes `<ostream>` directly.

2. **Precompiled Header (PCH)**
   - Each class header includes `<iosfwd>`.
   - A single precompiled header includes `<ostream>` and is used in each `.cpp` file.

3. **Stream Bridge**
   - Each class header includes `iosfwd-bridge.h`.
   - Each source file includes `ostream-bridge.h` from the [stream-bridge](https://github.com/Markussp256/stream-bridge) library.

## Results (as of June 2025)

When building all libraries and executables from scratch:

- **Stream Bridge** builds in roughly **half the time** compared to the other two approaches.
- **Baseline** and **PCH** configurations show similar build times.
- Note: The stream-bridge library is still relatively small at this point, so further benchmarking may be needed as it grows.

## Stream Bridge Library

The stream-bridge library is designed to reduce the overhead of including heavyweight standard stream headers by wrapping them in lighter, modular bridge headers.

You can find the library here:
[https://github.com/Markussp256/stream-bridge](https://github.com/Markussp256/stream-bridge)

## Build Instructions

```bash
git clone https://github.com/your_username/stream-bridge-benchmark.git
cd stream-bridge-benchmark
mkdir build && cd build
cmake ..
cmake --build .

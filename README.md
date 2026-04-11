# How to build

```
$ cargo build --release
```

# How to run

```
$ ./target/release/nstats -h
Statistics of ninja log file
args:
    -h --help        print this help message
    -f <path>        path to ninja log file
    --gui            run gui for view ninja log (default: false)
    --sort-by-name   sort stats by filename (default: by time)
    -m <int>         maximum lines of top slow files (default: 4096)
```

# Example

```
$ ./target/release/nstats -f data/.ninja_log -m 25
115775 ms |    modules/features2d/CMakeFiles/opencv_features2d.dir/src/agast_score.cpp.o
 83394 ms |    modules/python3/CMakeFiles/opencv_python3.dir/__/src2/cv2.cpp.o
 50396 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin.cpp.o
 49664 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx512_skx.cpp.o
 49640 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx2.cpp.o
 49325 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin_emulator.cpp.o
 47067 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx512_skx.cpp.o
 46770 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin512.avx512_skx.cpp.o
 45918 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse2.cpp.o
 45549 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.fp16.cpp.o
 45261 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx.cpp.o
 44767 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse4_1.cpp.o
 44688 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse3.cpp.o
 44475 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx2.cpp.o
 44125 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse4_2.cpp.o
 43868 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.ssse3.cpp.o
 41737 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_imgproc_tests_cpu.cpp.o
 39551 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_core_tests_cpu.cpp.o
 39321 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/common/gapi_core_perf_tests.cpp.o
 39140 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/common/gapi_core_tests.cpp.o
 38247 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/common/gapi_imgproc_tests.cpp.o
 36978 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/cpu/gapi_imgproc_perf_tests_cpu.cpp.o
 36967 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/common/gapi_imgproc_perf_tests.cpp.o
 35598 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_arithm.cpp.o
 35044 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/cpu/gapi_core_perf_tests_cpu.cpp.o
   ...

Stats:
cpu time (1T) : 02:09:54.715 (     7794715 ms)
compile time  : 00:05:33.859 (      333859 ms)
speed ratio   : 23.35
avg build time: 3710.00 ms
files per secs: 6.29
files         : 2101
```

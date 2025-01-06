# How to build

```
$ cargo build --release
```

# How to run

```
$ ./target/release/nstats -h
Statistics of ninja log file
args:
        -h              print this help message
        -f      <path>  path to ninja log file
        -g              print group stats
        -t      <int>   amount print lines of top slowly files
```

# Example

```
$ ./target/release/nstats -f data/.ninja_log_opencv -t 30
   91931 ms |    modules/features2d/CMakeFiles/opencv_features2d.dir/src/agast_score.cpp.o (ext: o)
   67339 ms |    modules/python3/CMakeFiles/opencv_python3.dir/__/src2/cv2.cpp.o (ext: o)
   40941 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin.cpp.o (ext: o)
   38699 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx2.cpp.o (ext: o)
   37688 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx512_skx.cpp.o (ext: o)
   36897 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin_emulator.cpp.o (ext: o)
   35379 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin512.avx512_skx.cpp.o (ext: o)
   34961 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.fp16.cpp.o (ext: o)
   34673 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx2.cpp.o (ext: o)
   34641 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx512_skx.cpp.o (ext: o)
   33937 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse3.cpp.o (ext: o)
   33820 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx.cpp.o (ext: o)
   33250 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse2.cpp.o (ext: o)
   32221 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/common/gapi_core_tests.cpp.o (ext: o)
   32188 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse4_2.cpp.o (ext: o)
   31957 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_imgproc_tests_cpu.cpp.o (ext: o)
   31945 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.ssse3.cpp.o (ext: o)
   31669 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/common/gapi_imgproc_tests.cpp.o (ext: o)
   31453 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_core_tests_cpu.cpp.o (ext: o)
   31304 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse4_1.cpp.o (ext: o)
   28627 ms |    modules/core/CMakeFiles/opencv_perf_core.dir/perf/opencl/perf_arithm.cpp.o (ext: o)
   27500 ms |    modules/imgproc/CMakeFiles/opencv_imgproc.dir/src/connectedcomponents.cpp.o (ext: o)
   27047 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_arithm.cpp.o (ext: o)
   25511 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_operations.cpp.o (ext: o)
   24341 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_core_tests_fluid.cpp.o (ext: o)
   23891 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/common/gapi_core_perf_tests.cpp.o (ext: o)
   22043 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/common/gapi_imgproc_perf_tests.cpp.o (ext: o)
   21853 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/gpu/gapi_core_tests_gpu.cpp.o (ext: o)
   21051 ms |    modules/gapi/CMakeFiles/opencv_perf_gapi.dir/perf/cpu/gapi_imgproc_perf_tests_cpu.cpp.o (ext: o)
     ...

Stats:
  cpu time      : "01:42:52.742 (     6172742 ms)"
  compile time  : "00:04:31.855 (      271855 ms)"
  speed ratio   : 22.71
  avg build time: 4142.8 ms
  files         : 1490
  files per secs: 5.48
```

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
    -g               print group stats
    -n --sort-name   sort stats by filename
    -t <int>         amount print lines of top slowly files
```

# Example

```
$ ./target/release/nstats -f data/.ninja_log_opencv -t 20
   91931 ms |    modules/features2d/CMakeFiles/opencv_features2d.dir/src/agast_score.cpp.o
   67339 ms |    modules/python3/CMakeFiles/opencv_python3.dir/__/src2/cv2.cpp.o
   40941 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin.cpp.o
   38699 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx2.cpp.o
   37688 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx512_skx.cpp.o
   36897 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin_emulator.cpp.o
   35379 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin512.avx512_skx.cpp.o
   34961 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.fp16.cpp.o
   34673 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx2.cpp.o
   34641 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx512_skx.cpp.o
   33937 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse3.cpp.o
   33820 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx.cpp.o
   33250 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse2.cpp.o
   32221 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/common/gapi_core_tests.cpp.o
   32188 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.sse4_2.cpp.o
   31957 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_imgproc_tests_cpu.cpp.o
   31945 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.ssse3.cpp.o
   31669 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/common/gapi_imgproc_tests.cpp.o
   31453 ms |    modules/gapi/CMakeFiles/opencv_test_gapi.dir/test/cpu/gapi_core_tests_cpu.cpp.o
     ...

Stats:
  cpu time (1T) : "01:42:52.742 (     6172742 ms)"
  compile time  : "00:04:31.855 (      271855 ms)"
  speed ratio   : 22.71
  avg build time: 4142.8 ms
  files         : 1490
  files per secs: 5.48
```

```
$ ./target/release/nstats -f data/.ninja_log_opencv -t 10 -g
   91931 ms |    modules/features2d/CMakeFiles/opencv_features2d.dir/src/agast_score.cpp.o
   67339 ms |    modules/python3/CMakeFiles/opencv_python3.dir/__/src2/cv2.cpp.o
   40941 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin.cpp.o
   38699 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx2.cpp.o
   37688 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin256.avx512_skx.cpp.o
   36897 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin_emulator.cpp.o
   35379 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin512.avx512_skx.cpp.o
   34961 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.fp16.cpp.o
   34673 ms |    modules/core/CMakeFiles/opencv_test_core.dir/test/test_intrin128.avx2.cpp.o
     ...
GroupStats:
o: 1358
        sum     : 01:41:46.487 (     6106487 ms)
        min     : 24 ms (3rdparty/protobuf/CMakeFiles/libprotobuf.dir/src/google/protobuf/io/io_win32.cc.o)
        max     : 91931 ms (modules/features2d/CMakeFiles/opencv_features2d.dir/src/agast_score.cpp.o)
        avg     : 4496.7 ms
cpp: 18
        sum     : 00:00:00.776 (         776 ms)
        min     : 17 ms (modules/stitching/opencl_kernels_stitching.cpp)
        max     : 97 ms (modules/dnn/opencl_kernels_dnn.cpp)
        avg     : 43.1 ms
app: 44
        sum     : 00:00:29.647 (       29647 ms)
        min     : 12 ms (modules/java/jar/CMakeFiles/opencv_java_jar.dir/java_class_filelist)
        max     : 4066 ms (modules/java/jar/CMakeFiles/opencv_java_jar.dir/java_compiled_opencv_java_jar)
        avg     : 673.8 ms
so: 17
        sum     : 00:00:00.663 (         663 ms)
        min     : 7 ms (lib/libopencv_core.so)
        max     : 237 ms (lib/python3/cv2.cpython-311-x86_64-linux-gnu.so)
        avg     : 39.0 ms

Stats:
  cpu time (1T) : "01:42:52.742 (     6172742 ms)"
  compile time  : "00:04:31.855 (      271855 ms)"
  speed ratio   : 22.71
  avg build time: 4142.8 ms
  files         : 1490
  files per secs: 5.48
```

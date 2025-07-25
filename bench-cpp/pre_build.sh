cmake -DCMAKE_BUILD_TYPE=Release \
-DCMAKE_MAKE_PROGRAM=${CPP_TOOLCHAIN_PATH}/bin/ninja \
-DCMAKE_C_COMPILER=${CPP_TOOLCHAIN_PATH}/bin/clang \
-DCMAKE_CXX_COMPILER=${CPP_TOOLCHAIN_PATH}/bin/clang++ \
-DCMAKE_TOOLCHAIN_FILE=${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake \
-G Ninja \
-S . \
-B build

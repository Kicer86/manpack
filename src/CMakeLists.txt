
find_package(Corrosion REQUIRED)
find_program(CBINDGEN cbindgen REQUIRED)

corrosion_import_crate(MANIFEST_PATH manpack/Cargo.toml)

add_custom_command(
    OUTPUT manpack_c.hpp
    COMMAND ${CBINDGEN}
    ARGS --config cbindgen.toml
         --crate manpack
         --output ${CMAKE_CURRENT_BINARY_DIR}/manpack_c.hpp
    DEPENDS
        manpack/src/codec.rs

    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/manpack
)

set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/manpack_c.hpp PROPERTIES GENERATED TRUE)

add_executable(test
    test.cpp
    ${CMAKE_CURRENT_BINARY_DIR}/manpack_c.hpp
)

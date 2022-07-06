

# create rust crate and cpp interface file

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
        manpack/src/c_interface.rs

    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/manpack
)

target_include_directories(manpack
    INTERFACE
        ${CMAKE_CURRENT_BINARY_DIR}
)

set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/manpack_c.hpp PROPERTIES GENERATED TRUE)

# c++ part

add_executable(test
    test.cpp
    ${CMAKE_CURRENT_BINARY_DIR}/manpack_c.hpp
)

target_link_libraries(test
    PRIVATE
        manpack
)

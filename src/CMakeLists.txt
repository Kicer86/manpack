

# create rust crate and cpp interface file

find_package(Corrosion REQUIRED)
find_program(CXXBRIDGE cxxbridge REQUIRED)

corrosion_import_crate(MANIFEST_PATH manpack/Cargo.toml)

add_custom_command(
    OUTPUT manpack_cpp.hpp
    COMMAND ${CXXBRIDGE}
    ARGS ${CMAKE_CURRENT_SOURCE_DIR}/manpack/src/cpp_interface.rs
         --header > ${CMAKE_CURRENT_BINARY_DIR}/manpack_cpp.hpp
    DEPENDS
        manpack/src/cpp_interface.rs

    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/manpack
)

add_custom_command(
    OUTPUT manpack_cpp.cpp
    COMMAND ${CXXBRIDGE}
    ARGS ${CMAKE_CURRENT_SOURCE_DIR}/manpack/src/cpp_interface.rs
         > ${CMAKE_CURRENT_BINARY_DIR}/manpack_cpp.cpp
    DEPENDS
        manpack/src/cpp_interface.rs
)

target_include_directories(manpack
    INTERFACE
        ${CMAKE_CURRENT_BINARY_DIR}
)

set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/manpack_cpp.cpp PROPERTIES GENERATED TRUE)
set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/manpack_cpp.hpp PROPERTIES GENERATED TRUE)

# c++ part

add_executable(test
    test.cpp
    ${CMAKE_CURRENT_BINARY_DIR}/manpack_cpp.cpp
    ${CMAKE_CURRENT_BINARY_DIR}/manpack_cpp.hpp
)

target_link_libraries(test
    PRIVATE
        manpack
)

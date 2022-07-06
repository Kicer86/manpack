
find_package(Corrosion REQUIRED)
find_program(CBINDGEN cbindgen REQUIRED)

corrosion_import_crate(MANIFEST_PATH codec/Cargo.toml)

add_custom_command(
    OUTPUT codec.h
    COMMAND ${CBINDGEN}
    ARGS --config cbindgen.toml
         --output ${CMAKE_CURRENT_BINARY_DIR}/codec.h
    DEPENDS
        codec/codec.rs

    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/codec
)

add_library(codec SHARED
    ${CMAKE_CURRENT_BINARY_DIR}/codec.h
)

target_include_directories(codec
    PRIVATE
        ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(codec
    PRIVATE
)

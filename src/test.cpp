
#include "manpack_cpp.hpp"

int main()
{
    const uintptr_t count = 12;
    std::uint32_t pixels[count] = { 1, 2, 3,  1, 2, 3,  5, 6, 7,  1, 5, 7 };

    rust::Slice<const std::uint32_t> rustPixels(pixels, count);
    auto compressed = rust_part::compress_image(rustPixels);

    return 0;
}

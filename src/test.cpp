
#include "manpack_c.hpp"

int main()
{
    const uintptr_t count = 12;
    std::uint32_t pixels[count];
    compressImage(pixels, count);

    return 0;
}

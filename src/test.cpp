
#include <QImage>
#include <QFile>

#include "manpack_cpp.hpp"

int main()
{
    /*
    const uintptr_t count = 12;
    std::uint32_t pixels[count] = { 1, 2, 3,  1, 2, 3,  5, 6, 7,  1, 5, 7 };

    rust::Slice<const std::uint32_t> rustPixels(pixels, count);
    auto compressed = rust_part::compress_image(rustPixels);
    */

    const QImage image("test.bmp");

    assert(image.format() == QImage::Format_RGB32 || image.format() == QImage::Format_ARGB32);
    const std::uint32_t* data = reinterpret_cast<const std::uint32_t*>(image.bits());
    const qsizetype data_size = image.sizeInBytes() / 4;

    rust::Slice<const std::uint32_t> rustPixels(data, data_size);
    auto compressed = rust_part::compress_image(image.width(), image.height(), rustPixels);

    QFile mpImage("test.mp");
    mpImage.open(QIODevice::WriteOnly);
    mpImage.write((const char *)compressed.data(), compressed.size());
    mpImage.close();

    mpImage.open(QIODevice::ReadOnly);
    const QByteArray mpImageRaw = mpImage.readAll();
    mpImage.close();

    Image decompressed = rust_part::decompress_image(rust::Slice((const unsigned char*)mpImageRaw.data(), mpImageRaw.size()));

    return 0;
}


#include <QImage>
#include <QFile>

#include "manpack_cpp.hpp"

int main()
{
    const QImage image("test.jpg");

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

    const QImage decompressedImage((const uchar*)decompressed.pixels.data(), decompressed.width, decompressed.height, QImage::Format_ARGB32);
    decompressedImage.save("testd.png");

    return 0;
}

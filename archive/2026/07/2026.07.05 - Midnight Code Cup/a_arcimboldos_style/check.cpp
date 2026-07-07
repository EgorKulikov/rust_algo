#include <algorithm>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>
#include <png.h>
#include "testlib.h"

namespace fs = std::filesystem;

int const MIN_N    =       1;
int const MAX_N    =  10'000;
int const MIN_PART =       0;
int const MAX_PART =   9'999;
int const MIN_ROW  = -10'000;
int const MAX_ROW  = +10'000;
int const MIN_COL  = -10'000;
int const MAX_COL  = +10'000;

struct ImageRGBA {
    int width = 0;
    int height = 0;
    std::vector<std::uint8_t> data;
};

struct Placement {
    int pic = 0;
    int row = 0;
    int col = 0;
};

struct TestInput {
    int limit_n = 0;
    std::string target_name;
    std::string clipart_collection;
};

[[nodiscard]] ImageRGBA load_rgba(const fs::path& path) {
    png_image png_image_data{};
    png_image_data.version = PNG_IMAGE_VERSION;

    const std::string filename = path.string();
    if (!png_image_begin_read_from_file(&png_image_data, filename.c_str())) {
        quitf(_fail, "Failed to open image: %s", filename.c_str());
    }

    png_image_data.format = PNG_FORMAT_RGBA;

    if (png_image_data.width > static_cast<png_uint_32>(std::numeric_limits<int>::max()) ||
        png_image_data.height > static_cast<png_uint_32>(std::numeric_limits<int>::max())) {
        png_image_free(&png_image_data);
        quitf(_fail, "Image is too large: %s", filename.c_str());
    }

    ImageRGBA image;
    image.width = static_cast<int>(png_image_data.width);
    image.height = static_cast<int>(png_image_data.height);
    image.data.resize(PNG_IMAGE_SIZE(png_image_data));

    if (!png_image_finish_read(&png_image_data, nullptr, image.data.data(), 0, nullptr)) {
        const std::string message = png_image_data.message[0] != '\0' ? png_image_data.message : "unknown libpng error";
        png_image_free(&png_image_data);
        quitf(_fail, "Failed to decode image pixels: %s (%s)", filename.c_str(), message.c_str());
    }

    png_image_free(&png_image_data);
    return image;
}

[[nodiscard]] TestInput read_test_input(InStream& f) {
    TestInput info;
    info.limit_n = f.readInt(MIN_N, MAX_N, "n");
    f.readEoln();
    info.target_name = f.readLine();
    info.clipart_collection = f.readLine();
    f.readLine(); // comment
    f.readEof();
    return info;
}

[[nodiscard]] std::vector<Placement> read_solution(InStream& f, int limit_n) {
    int m = f.readInt(0, limit_n, "m");

    std::vector<Placement> placements;
    placements.reserve(static_cast<std::size_t>(m));

    for (int i = 0; i < m; ++i) {
        Placement placement;
        placement.pic = f.readInt(MIN_PART, MAX_PART, "pic");
        placement.row = f.readInt(MIN_ROW,  MAX_ROW,  "row");
        placement.col = f.readInt(MIN_COL,  MAX_COL,  "col");
        placements.push_back(placement);
    }

    return placements;
}

[[nodiscard]] fs::path clipart_path(const fs::path& test_dir, int pic) {
    std::ostringstream name;
    name << std::setfill('0') << std::setw(4) << pic << ".png";
    return test_dir / name.str();
}

void draw_clipart(std::vector<int>& canvas_rgb, int canvas_h, int canvas_w, const ImageRGBA& clip, int top, int left) {
    for (int y = 0; y < clip.height; ++y) {
        const int dst_y = top + y;
        if (dst_y >= canvas_h || dst_y < 0) {
            continue;
        }
        for (int x = 0; x < clip.width; ++x) {
            const int dst_x = left + x;
            if (dst_x >= canvas_w || dst_x < 0) {
                continue;
            }

            const std::size_t src_idx = (static_cast<std::size_t>(y) * static_cast<std::size_t>(clip.width) + static_cast<std::size_t>(x)) * 4U;
            if (clip.data[src_idx + 3U] == 0) {
                continue;
            }

            const std::size_t dst_idx = (static_cast<std::size_t>(dst_y) * static_cast<std::size_t>(canvas_w) + static_cast<std::size_t>(dst_x)) * 3U;
            canvas_rgb[dst_idx + 0U] = static_cast<int>(clip.data[src_idx + 0U]);
            canvas_rgb[dst_idx + 1U] = static_cast<int>(clip.data[src_idx + 1U]);
            canvas_rgb[dst_idx + 2U] = static_cast<int>(clip.data[src_idx + 2U]);
        }
    }
}

[[nodiscard]] std::uint64_t compute_total_error(const ImageRGBA& target, const std::vector<int>& canvas_rgb) {
    if (target.width <= 0 || target.height <= 0) {
        quitf(_fail, "Target image has invalid size");
    }

    std::uint64_t total_error = 0;
    const std::size_t pixels = static_cast<std::size_t>(target.width) * static_cast<std::size_t>(target.height);
    for (std::size_t i = 0; i < pixels; ++i) {
        const int tr = static_cast<int>(target.data[i * 4U + 0U]);
        const int tg = static_cast<int>(target.data[i * 4U + 1U]);
        const int tb = static_cast<int>(target.data[i * 4U + 2U]);
        const int pr = canvas_rgb[i * 3U + 0U];
        const int pg = canvas_rgb[i * 3U + 1U];
        const int pb = canvas_rgb[i * 3U + 2U];

        const int dr = tr - pr;
        const int dg = tg - pg;
        const int db = tb - pb;
        total_error += static_cast<std::uint64_t>(dr * dr + dg * dg + db * db);
    }
    return total_error;
}

[[nodiscard]] fs::path resolve_assets_dir(int argc, char* argv[], const TestInput& input) {
    if (argc >= 2) {
        const fs::path input_path = fs::absolute(argv[1]);
        const fs::path candidate = input_path.has_parent_path() ? input_path.parent_path() : fs::current_path();
        if (fs::exists(candidate / input.target_name) && fs::exists(candidate / input.clipart_collection)) {
            return candidate;
        }
    }
    return fs::path("/app/assets");
}

int main(int argc, char *argv[]) {
    setName("checker for problem arcimboldo");
    registerTestlibCmd(argc, argv);

    const TestInput input = read_test_input(inf);
    const std::vector<Placement> placements = read_solution(ouf, input.limit_n);

    try {
        const fs::path test_dir = resolve_assets_dir(argc, argv, input);

        const ImageRGBA target = load_rgba(test_dir / input.target_name);

        std::vector<int> canvas_rgb(
            static_cast<std::size_t>(target.width) * static_cast<std::size_t>(target.height) * 3U,
            -255
        );

        std::unordered_map<int, ImageRGBA> clip_cache;
        clip_cache.reserve(placements.size());

        const fs::path clipart_dir = test_dir / fs::path(input.clipart_collection);

        for (const Placement& placement : placements) {
            auto it = clip_cache.find(placement.pic);
            if (it == clip_cache.end()) {
                const fs::path path = clipart_path(clipart_dir, placement.pic);
                if (!fs::exists(path)) {
                    quitf(_wa, "Clipart file not found: %s", path.string().c_str());
                }
                it = clip_cache.emplace(placement.pic, load_rgba(path)).first;
            }

            draw_clipart(canvas_rgb, target.height, target.width, it->second, placement.row, placement.col);
        }

        const std::uint64_t total_error = compute_total_error(target, canvas_rgb);

        ouf.quit(_points, (std::to_string(total_error) + " " + format("ok, total error is %e", static_cast<double>(total_error))).c_str());
    } catch (const std::exception& ex) {
        quitf(_wa, ex.what());
    }
}

#include "xlink_api.h"   // <-- first
#include "util/file.h"
#include "system.h"

#include <cstdint>
#include <cstddef>
#include <cstring>
#include <string>


XLINK_API const char* xlink_binary_to_yaml(const char* data, size_t size) {
    if (!data || size == 0)
        return nullptr;

    banana::System sys;
    if (!sys.initialize(const_cast<void*>(reinterpret_cast<const void*>(data)), size))
        return nullptr;

    std::string yaml = sys.dumpYAML();

    char* result = new char[yaml.size() + 1];
    std::memcpy(result, yaml.c_str(), yaml.size() + 1);
    return result;
}

XLINK_API char* xlink_yaml_to_binary(const char* data, size_t size, size_t* out_size) {
    if (!data || size == 0 || !out_size)
        return nullptr;

    banana::System sys;
    if (!sys.loadYAML({ reinterpret_cast<const char*>(data) }))
        return nullptr;

    auto bin = sys.serialize();
    char* result = new char[bin.size()];
    std::memcpy(result, bin.data(), bin.size());
    *out_size = bin.size();
    return result;
}

XLINK_API void free_xlink_binary(void* data) {
    delete[] reinterpret_cast<char*>(data);
}

XLINK_API void free_xlink_string(char* str) {
    delete[] str;
}


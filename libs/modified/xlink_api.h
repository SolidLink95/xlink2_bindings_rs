#pragma once
#include <cstdint>
#include <cstddef>

#ifdef _WIN32
    #ifdef XLINK_BUILD_DLL
        #define XLINK_API __declspec(dllexport)
    #else
        #define XLINK_API __declspec(dllimport)
    #endif
#else
    #define XLINK_API __attribute__((visibility("default")))
#endif

extern "C" {

XLINK_API const char* xlink_binary_to_yaml(const char* data, size_t size);
XLINK_API char* xlink_yaml_to_binary(const char* data, size_t size, size_t* out_size);
XLINK_API void free_xlink_binary(void* binary);
XLINK_API void free_xlink_string(char* str);

}

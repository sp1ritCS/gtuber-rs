// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from /__w/gtuber/gtuber/x86_64-redhat-linux/gtuber (@ f6573a461c85)
// from /usr/local/share/gir-1.0 (@ ???)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GTUBER_ADAPTIVE_STREAM_MANIFEST_DASH);
    PRINT_CONSTANT((gint) GTUBER_ADAPTIVE_STREAM_MANIFEST_HLS);
    PRINT_CONSTANT((gint) GTUBER_ADAPTIVE_STREAM_MANIFEST_UNKNOWN);
    PRINT_CONSTANT((gint) GTUBER_CLIENT_ERROR_MISSING_INFO);
    PRINT_CONSTANT((gint) GTUBER_CLIENT_ERROR_NO_PLUGIN);
    PRINT_CONSTANT((gint) GTUBER_FLOW_ERROR);
    PRINT_CONSTANT((gint) GTUBER_FLOW_OK);
    PRINT_CONSTANT((gint) GTUBER_FLOW_RESTART);
    PRINT_CONSTANT(GTUBER_MAJOR_VERSION);
    PRINT_CONSTANT((gint) GTUBER_MANIFEST_GENERATOR_ERROR_NO_DATA);
    PRINT_CONSTANT(GTUBER_MICRO_VERSION);
    PRINT_CONSTANT(GTUBER_MINOR_VERSION);
    PRINT_CONSTANT((gint) GTUBER_STREAM_MIME_TYPE_AUDIO_MP4);
    PRINT_CONSTANT((gint) GTUBER_STREAM_MIME_TYPE_AUDIO_WEBM);
    PRINT_CONSTANT((gint) GTUBER_STREAM_MIME_TYPE_UNKNOWN);
    PRINT_CONSTANT((gint) GTUBER_STREAM_MIME_TYPE_VIDEO_MP4);
    PRINT_CONSTANT((gint) GTUBER_STREAM_MIME_TYPE_VIDEO_WEBM);
    PRINT_CONSTANT(GTUBER_VERSION_S);
    PRINT_CONSTANT((gint) GTUBER_WEBSITE_ERROR_OTHER);
    PRINT_CONSTANT((gint) GTUBER_WEBSITE_ERROR_PARSE_FAILED);
    PRINT_CONSTANT((gint) GTUBER_WEBSITE_ERROR_REQUEST_CREATE_FAILED);
    return 0;
}

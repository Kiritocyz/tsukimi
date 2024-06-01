// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../../gir-files-gstreamer
// from ../../gir-files-gtk
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
    PRINT_CONSTANT((gint) CLAPPER_DISCOVERER_DISCOVERY_ALWAYS);
    PRINT_CONSTANT((gint) CLAPPER_DISCOVERER_DISCOVERY_NONCURRENT);
    PRINT_CONSTANT(CLAPPER_HAVE_DISCOVERER);
    PRINT_CONSTANT(CLAPPER_HAVE_MPRIS);
    PRINT_CONSTANT(CLAPPER_HAVE_SERVER);
    PRINT_CONSTANT(CLAPPER_MAJOR_VERSION);
    PRINT_CONSTANT(CLAPPER_MARKER_NO_END);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_CHAPTER);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_CUSTOM_1);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_CUSTOM_2);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_CUSTOM_3);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_TITLE);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_TRACK);
    PRINT_CONSTANT((gint) CLAPPER_MARKER_TYPE_UNKNOWN);
    PRINT_CONSTANT(CLAPPER_MICRO_VERSION);
    PRINT_CONSTANT(CLAPPER_MINOR_VERSION);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_SEEK_METHOD_ACCURATE);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_SEEK_METHOD_FAST);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_SEEK_METHOD_NORMAL);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_STATE_BUFFERING);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_STATE_PAUSED);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_STATE_PLAYING);
    PRINT_CONSTANT((gint) CLAPPER_PLAYER_STATE_STOPPED);
    PRINT_CONSTANT(CLAPPER_QUEUE_INVALID_POSITION);
    PRINT_CONSTANT((gint) CLAPPER_QUEUE_PROGRESSION_CAROUSEL);
    PRINT_CONSTANT((gint) CLAPPER_QUEUE_PROGRESSION_CONSECUTIVE);
    PRINT_CONSTANT((gint) CLAPPER_QUEUE_PROGRESSION_NONE);
    PRINT_CONSTANT((gint) CLAPPER_QUEUE_PROGRESSION_REPEAT_ITEM);
    PRINT_CONSTANT((gint) CLAPPER_QUEUE_PROGRESSION_SHUFFLE);
    PRINT_CONSTANT(CLAPPER_STREAM_LIST_INVALID_POSITION);
    PRINT_CONSTANT((gint) CLAPPER_STREAM_TYPE_AUDIO);
    PRINT_CONSTANT((gint) CLAPPER_STREAM_TYPE_SUBTITLE);
    PRINT_CONSTANT((gint) CLAPPER_STREAM_TYPE_UNKNOWN);
    PRINT_CONSTANT((gint) CLAPPER_STREAM_TYPE_VIDEO);
    PRINT_CONSTANT(CLAPPER_TIME_FORMAT);
    PRINT_CONSTANT(CLAPPER_TIME_MS_FORMAT);
    PRINT_CONSTANT(CLAPPER_VERSION_S);
    return 0;
}
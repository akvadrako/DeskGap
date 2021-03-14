#ifndef DG_GEO_H
#define DG_GEO_H

#include <stdint.h>

typedef struct {
    double height;
    double width;
} dg_size_t;

typedef struct {
    double x;
    double y;
} dg_point_t;

typedef struct {
    dg_size_t size;
    dg_point_t location;
} dg_frame_t;

#endif

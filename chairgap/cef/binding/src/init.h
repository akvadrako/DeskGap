#ifndef DGCEF_INIT_H
#define DGCEF_INIT_H

#ifdef __cplusplus
extern "C" {
#endif

int dgcef_init(
    const char* lib_path,
    int argc_arg, char** argv_arg
);

#ifdef __cplusplus
}
#endif

#endif
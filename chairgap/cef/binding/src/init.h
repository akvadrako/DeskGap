#ifndef DGCEF_INIT_H
#define DGCEF_INIT_H

#ifdef __cplusplus
extern "C" {
#endif

int dgcef_init(
    const void* cef_path,
    int argc_arg, char** argv_arg
);

void dgcef_run_message_loop();

#ifdef __cplusplus
}
#endif

#endif

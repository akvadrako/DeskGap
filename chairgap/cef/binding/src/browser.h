#ifndef DGCEF_BROWSER_H
#define DGCEF_BROWSER_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct dgcef_browser_s *dgcef_browser_t;
dgcef_browser_t dgcef_browser_new(void* parent_handle);
void dgcef_browser_free(dgcef_browser_t);

#ifdef __cplusplus
}
#endif

#endif

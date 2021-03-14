#include "object.h"

void DGRelease(const void* obj) {
    CFBridgingRelease(obj);
}

#endif

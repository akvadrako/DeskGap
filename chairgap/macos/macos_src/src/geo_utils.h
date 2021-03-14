#ifndef DG_GEO_UTILS
#define DG_GEO_UTILS

#include "geo.h"

inline static dg_size_t dg_size_from_ns(NSSize ns_size) {
	dg_size_t result;
	result.width = ns_size.width;
	result.height = ns_size.height;
	return result;
}


#endif

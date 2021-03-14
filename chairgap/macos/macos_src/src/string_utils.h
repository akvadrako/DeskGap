#ifndef DG_STRING_UTILS
#define DG_STRING_UTILS

#include "string.h"

// inline static dg_str_t dg_str_new_from_ns(NSString* ns_string) {
// 	NSUInteger max_len = [ns_string maximumLengthOfBytesUsingEncoding: NSUTF8StringEncoding];
// 	char* mem = malloc(max_len);
// 	NSUInteger len;
// 	[ns_string
// 		getBytes: (void*)mem
// 		maxLength: max_len usedLength: &len
// 		encoding: NSUTF8StringEncoding
// 		options: 0
// 	];
// }

inline static dg_str_t dg_str_borrow_from_ns(NSString* ns_string) {
	dg_str_t res;
	res.mem = [ns_string UTF8String];
	res.len = [ns_string lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
	return res;
}

inline static NSString* dg_str_to_ns(const dg_str_t* dg_str) {
	return [[NSString alloc] initWithBytes: dg_str->mem length: dg_str->len encoding: NSUTF8StringEncoding];
}

#endif

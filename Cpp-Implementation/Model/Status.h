#ifndef STATUS_H
#define STATUS_H

#include <cstdint>

struct __attribute__((packed)) Status
{
	int8_t msg[63];
}

#endif
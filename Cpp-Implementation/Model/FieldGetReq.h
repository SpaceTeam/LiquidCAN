#ifndef FIELDGETREQ_H
#define FIELDGETREQ_H

#include <cstdint>

struct __attribute__((packed)) FieldGetReq
{
	uint8_t fieldId;
}

#endif
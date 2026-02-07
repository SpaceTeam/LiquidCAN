#ifndef FIELDGETRES_H
#define FIELDGETRES_H

#include <cstdint>

struct FieldGetRes
{
	uint8_t fieldId;
	uint8_t value[62];
}

#endif
#ifndef PARAMETERSETLOCK_H
#define PARAMETERSETLOCK_H

#include <cstdint>

struct __attribute__((packed)) ParameterSetLock
{
	uint8_t parameterId;
	uint8_t lockStatus;
}

#endif
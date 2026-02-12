#ifndef PARAMETERSETLOCK_H
#define PARAMETERSETLOCK_H

#include <cstdint>

struct ParameterSetLock
{
	uint8_t parameterId;
	uint8_t lockStatus;
}

#endif
#ifndef PARAMETERSETSTATUS_H
#define PARAMETERSETSTATUS_H

#include <cstdint>

enum ParameterSetStatus : uint8_t
{
	Success = 0,
	InvalidParameterId = 1,
	ParameterLocked = 2,
	NodeToNodeModification = 3,
}

#endif
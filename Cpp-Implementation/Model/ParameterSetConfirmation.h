#ifndef PARAMETERSETCONFIRMATION_H
#define PARAMETERSETCONFIRMATION_H

#include <cstdint>

struct ParameterSetConfirmation
{
	uint8_t parameterId;
	ParameterSetStatus status;
	uint8_t value[61];
}

#endif
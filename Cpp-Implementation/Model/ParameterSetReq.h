#ifndef PARAMETERSETREQ_H
#define PARAMETERSETREQ_H

#include <cstdint>

struct ParameterSetReq
{
	uint8_t parameterId;
	uint8_t value[61];
}

#endif
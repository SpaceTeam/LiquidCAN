#ifndef PARAMETERSETREQ_H
#define PARAMETERSETREQ_H

#include <cstdint>

struct __attribute__((packed)) ParameterSetReq
{
	uint8_t parameterId;
	uint8_t value[61];
}

#endif
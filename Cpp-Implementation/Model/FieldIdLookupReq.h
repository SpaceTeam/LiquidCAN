#ifndef FIELDIDLOOKUPREQ_H
#define FIELDIDLOOKUPREQ_H

#include <cstdint>

struct __attribute__((packed)) FieldIdLookupReq
{
	uint8_t fieldName[61];
}

#endif
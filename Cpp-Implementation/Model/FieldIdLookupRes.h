#ifndef FIELDIDLOOKUPRES_H
#define FIELDIDLOOKUPRES_H

#include <cstdint>
#include <../Infrastructure/DataType.h>

struct __attribute__((packed)) FieldIdLookupRes
{
	uint8_t fieldId;
	DataType fieldType;
}

#endif
#ifndef FIELDREGISTRATION_H
#define FIELDREGISTRATION_H

#include <cstdint>

struct __attribute__((packed)) FieldRegistration
{
	uint8_t fieldId;
	uint8_t fieldType;
	int8_t fieldName[61];
}

#endif
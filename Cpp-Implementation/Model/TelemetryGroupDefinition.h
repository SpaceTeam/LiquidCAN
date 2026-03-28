#ifndef TELEMETRYGROUPDEFINITION_H
#define TELEMETRYGROUPDEFINITION_H

#include <cstdint>

struct __attribute__((packed)) TelemetryGroupDefinition
{
	uint8_t groupId;
	uint8_t fieldIds[62];
}

#endif
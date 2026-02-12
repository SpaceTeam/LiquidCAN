#ifndef TELEMETRYGROUPUPDATE_H
#define TELEMETRYGROUPUPDATE_H

#include <cstdint>

struct TelemetryGroupUpdate
{
	uint8_t groupId;
	uint8_t values[62];
}

#endif
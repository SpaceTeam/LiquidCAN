#ifndef NODEINFORES_H
#define NODEINFORES_H

#include <cstdint>

struct NodeInfoRes
{
	uint8_t telCnt;
	uint8_t parCnt;
	uint32_t firmwareHash;
	uint32_t liquidHash;
	int8_t deviceName[53];
}

#endif
#ifndef DATATYPE_H
#define DATATYPE_H

#include <cstdint>

enum DataType : uint8_t
{
	Float32 = 0,
	Int32 = 1,
	Int16 = 2,
	Int8 = 3,
	Uint32 = 4,
	Uint16 = 5,
	Uint8 = 6,
	Boolean = 7,
}

#endif
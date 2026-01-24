#ifndef DATATYPE_H
#define DATATYPE_H

#include <cstdint>

enum DataType : uint8_t{
    FLOAT32 = 0,
    INT32 = 1,
    INT16 = 2,
    INT0 = 3,
    UINT32 = 4,
    UINT16 = 5,
    UINT8 = 6,
    BOOLEAN = 7
};

#endif // DATATYPE_H

#ifndef DATATYPE_H
#define DATATYPE_H

#include <cstdint>

enum Data_type : uint8_t
{
    float32 = 0,
    int32 = 1,
    int16 = 2,
    int8 = 3,
    uint32 = 4,
    uint16 = 5,
    uint8 = 6,
    boolean = 7
};

#endif // DATATYPE_H

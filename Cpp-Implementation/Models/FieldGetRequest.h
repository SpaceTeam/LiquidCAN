//
// Created by tobias on 30.01.26.
//

#ifndef FIELDGETREQUEST_H
#define FIELDGETREQUEST_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Field_get_request : public Can_message
{
    uint8_t field_id_;    // Field identifier
};

#endif //FIELDGETREQUEST_H

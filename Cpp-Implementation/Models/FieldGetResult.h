//
// Created by tobias on 30.01.26.
//

#ifndef FIELDGETRESULT_H
#define FIELDGETRESULT_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Field_get_result : public Can_message
{
public:
    Field_get_result();
    Field_get_result(uint8_t field_id, const uint8_t* value, uint8_t length);

    bool set_field_id(uint8_t field_id);
    uint8_t get_field_id() const;

    bool set_value(const uint8_t* value, uint8_t length);
    const uint8_t* get_value() const;
    uint8_t get_value_length() const;
private:
    uint8_t field_id_;         // Field identifier
    uint8_t value_length_;     // Field value length
    uint8_t value_[62];        // Field value
};

#endif //FIELDGETRESULT_H

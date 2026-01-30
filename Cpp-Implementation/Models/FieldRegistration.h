#ifndef FIELDREGISTRATION_H
#define FIELDREGISTRATION_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>
#include <Infrastructure/DataType.h>

class Field_registration : public Can_message
{
public:
    Field_registration();
    Field_registration(uint8_t field_id, Data_type field_type, const uint8_t* device_name, uint8_t length);

    bool set_field_id(uint8_t field_id);
    uint8_t get_field_id() const;

    bool set_field_type(Data_type field_type);
    Data_type get_field_type() const;

    bool set_field_name(const uint8_t* field_name, uint8_t length);
    const uint8_t* get_field_name() const;
    uint8_t get_field_name_length() const;
private:
    uint8_t field_id_;             // Unique identifier for this field
    Data_type field_type_;         // Field data type
    uint8_t field_name_length_;    // Field name length
    uint8_t field_name_[63];       // Human-readable field name
};

#endif // FIELDREGISTRATION_H

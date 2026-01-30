//
// Created by tobias on 30.01.26.
//

#ifndef FIELDIDLOOKUPREQUEST_H
#define FIELDIDLOOKUPREQUEST_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Field_id_lookup_request : public Can_message
{
public:
    Field_id_lookup_request();
    Field_id_lookup_request(const uint8_t* field_name, uint8_t length);

    bool set_field_name(const uint8_t* field_name, uint8_t length);
    const uint8_t* get_field_name() const;
    uint8_t get_field_name_length() const;
private:
    uint8_t field_name_length_;    // Field name length
    uint8_t field_name_[61];       // Field name
};

#endif //FIELDIDLOOKUPREQUEST_H

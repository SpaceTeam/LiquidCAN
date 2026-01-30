#ifndef PARAMETERSETREQUEST_H
#define PARAMETERSETREQUEST_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Parameter_set_request : public Can_message
{
public:
    Parameter_set_request();
    Parameter_set_request(uint8_t parameter_id, const uint8_t* value, uint8_t length);

    bool set_parameter_id(uint8_t id);
    uint8_t get_parameter_id() const;

    bool set_parameter_value(const uint8_t* value, uint8_t value_length);
    const uint8_t* get_parameter_value() const;
    uint8_t get_parameter_value_length() const;
private:
    uint8_t parameter_id_;  // Parameter identifier
    uint8_t value_length_;  // Internal Counter to track length of the value field
    uint8_t value_[61];     // New value
};

#endif // PARAMETERSETREQUEST_H

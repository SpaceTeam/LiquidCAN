//
// Created by tobias on 30.01.26.
//

#ifndef PARAMETERSETCONFIRMATION_H
#define PARAMETERSETCONFIRMATION_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>
#include <Infrastructure/ParameterSetStatus.h>

class Parameter_set_confirmation : public Can_message
{
public:
    Parameter_set_confirmation();
    Parameter_set_confirmation(uint8_t parameter_id,
                               Parameter_set_status status,
                               const uint8_t* value,
                               uint8_t length);

    bool set_parameter_id(uint8_t parameter_id);
    uint8_t get_parameter_id() const;

    bool set_status(Parameter_set_status status);
    Parameter_set_status get_status() const;

    bool set_value(const uint8_t* value, uint8_t length);
    const uint8_t* get_value() const;
    uint8_t get_value_length() const;
private:
    uint8_t parameter_id_;            // Parameter identifier
    Parameter_set_status status_;     // Status code (Parameter_set_status enum)
    uint8_t value_length_;            // Confirmed value length
    uint8_t value_[61];               // Confirmed value after set operation
};

#endif //PARAMETERSETCONFIRMATION_H

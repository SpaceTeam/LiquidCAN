#ifndef PARAMETERSETREQUEST_H
#define PARAMETERSETREQUEST_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class ParameterSetReq : public CanMessage {
public:
    ParameterSetReq();
    ParameterSetReq(uint8_t parameter_id, const uint8_t* value, uint8_t value_length);

    bool setParameterId(uint8_t id);
    uint8_t getParameterId() const;

    bool setParameterValue(uint8_t value, uint8_t value_length);
    uint8_t* getParameterValue() const;
    uint8_t getParameterValueLength() const;

private:
    uint8_t parameter_id_;  // Parameter identifier
    uint8_t value_length_;  // Internal Counter to track length of the value field
    uint8_t value_[61];     // New value
};

#endif // PARAMETERSETREQUEST_H

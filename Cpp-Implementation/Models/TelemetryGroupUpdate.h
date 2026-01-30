#ifndef TELEMETRYGROUPUPDATE_H
#define TELEMETRYGROUPUPDATE_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Telemetry_group_update : public Can_message
{
public:
    Telemetry_group_update();
    Telemetry_group_update(uint8_t group_id, const uint8_t* values, uint8_t value_count);

    bool set_group_id(uint8_t group_id);
    uint8_t get_group_id() const;

    bool set_group_values(const uint8_t* values, uint8_t value_count);
    const uint8_t* get_group_values() const;
    uint8_t get_group_value_length() const;
private:
    uint8_t group_id_;      // Group identifier
    uint8_t value_length_;  // Internal Counter to track length of the value field
    uint8_t values_[62];    // Packed values of all telemetry value in the group
};

#endif // TELEMETRYGROUPUPDATE_H

#ifndef TELEMETRYGROUPDEFINITION_H
#define TELEMETRYGROUPDEFINITION_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Telemetry_group_definition : public Can_message
{
public:
    Telemetry_group_definition();
    Telemetry_group_definition(uint8_t group_id, const uint8_t* field_ids, uint8_t field_count);

    bool set_group_id(uint8_t group_id);
    uint8_t get_group_id() const;

    bool add_group_field_id(uint8_t field_id);
    bool set_group_field_ids(const uint8_t* field_ids, uint8_t field_count);
    const uint8_t* get_group_field_ids() const;
    uint8_t get_group_field_count() const;
private:
    uint8_t group_id_;          // Unique identifier for this group
    uint8_t field_count_;       // Internal Counter to track number of fields
    uint8_t field_ids_[62];     // Array of field IDs in this group
};

#endif // TELEMETRYGROUPDEFINITION_H

#ifndef TELEMETRYGROUPDEFINITION_H
#define TELEMETRYGROUPDEFINITION_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class TelemetryGroupDefinition : public CanMessage {
public:
    TelemetryGroupDefinition();
    TelemetryGroupDefinition(uint8_t group_id, const uint8_t* field_ids, uint8_t field_count);

    bool setGroupId(uint8_t group_id);
    uint8_t getGroupId() const;

    bool addGroupFieldId(uint8_t field_id);
    bool setGroupFieldIds(const uint8_t* field_ids, uint8_t field_count);
    uint8_t* getGroupFieldIds() const;
    uint8_t getGroupFieldCount() const;

private:
    uint8_t group_id_;      // Unique identifier for this group
    uint8_t field_count_;   // Internal Counter to track number of fields
    uint8_t field_ids_[62]; // Array of field IDs in this group
};

#endif // TELEMETRYGROUPDEFINITION_H

#ifndef TELEMETRYGROUPUPDATE_H
#define TELEMETRYGROUPUPDATE_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class TelemetryGroupUpdate : public CanMessage {
public:
    TelemetryGroupUpdate();
    TelemetryGroupUpdate(uint8_t group_id, const uint8_t* values, uint8_t value_count);

    bool setGroupId(uint8_t group_id);
    uint8_t getGroupId() const;

    bool setGroupValues(uint8_t* values, uint8_t value_count);
    uint8_t* getGroupValues() const;
    uint8_t getGroupValueLength() const;
private:
    uint8_t group_id_;      // Group identifier
    uint8_t value_length_;  // Internal Counter to track length of the value field
    uint8_t values_[62];    // Packed values of all telemetry value in the group
};

#endif // TELEMETRYGROUPUPDATE_H

//
// Created by tobias on 24.01.26.
//

#ifndef NODEINFOREQUEST_H
#define NODEINFOREQUEST_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class NodeInfoRequest : public CanMessage{
public:
    NodeInfoRequest();
    NodeInfoRequest(uint8_t tel_cnt,
                    uint8_t par_cnt,
                    uint32_t firmware_hash,
                    uint32_t liquid_hash,
                    const char* device_name);

    bool setTelemetryCount(uint8_t tel_cnt);
    uint8_t getTelemetryCount() const;

    bool setParameterCount(uint8_t par_cnt);
    uint8_t getParameterCount() const;

    bool setFirmwareHash(uint32_t firmware_hash);
    uint32_t getFirmwareHash() const;

    bool setLiquidHash(uint32_t liquid_hash);
    uint32_t getLiquidHash() const;

    bool setDeviceName(const char* device_name, uint8_t device_name_length);
    const char* getDeviceName() const;
    uint8_t getDeviceNameLength() const;

private:
    uint8_t tel_cnt_;           // Number of telemetryValues on this node
    uint8_t par_cnt_;           // Number of parameters ot his node
    uint32_t firmware_hash_;    // Hash of the firmware version
    uint32_t liquid_hash_;      // Hash of the LiquidCan protocol version
    uint8_t device_name_length; // Internal counter to track device_name length
    char device_name_[53];      // Human-readable device name
};

#endif //NODEINFOREQUEST_H

//
// Created by tobias on 24.01.26.
//

#ifndef NODEINFOREQUEST_H
#define NODEINFOREQUEST_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Node_info_request : public Can_message
{
public:
    Node_info_request();
    Node_info_request(uint8_t tel_cnt,
                      uint8_t par_cnt,
                      uint32_t firmware_hash,
                      uint32_t liquid_hash,
                      const uint8_t* device_name,
                      uint8_t length);

    bool set_telemetry_count(uint8_t tel_cnt);
    uint8_t get_telemetry_count() const;

    bool set_parameter_count(uint8_t par_cnt);
    uint8_t get_parameter_count() const;

    bool set_firmware_hash(uint32_t firmware_hash);
    uint32_t get_firmware_hash() const;

    bool set_liquid_hash(uint32_t liquid_hash);
    uint32_t get_liquid_hash() const;

    bool set_device_name(const uint8_t* device_name, uint8_t length);
    const uint8_t* get_device_name() const;
    uint8_t get_device_name_length() const;
private:
    uint8_t tel_cnt_;               // Number of telemetry values on this node
    uint8_t par_cnt_;               // Number of parameters on this node
    uint32_t firmware_hash_;        // Hash of the firmware version
    uint32_t liquid_hash_;          // Hash of the LiquidCan protocol version
    uint8_t device_name_length_;    // Internal counter to track device_name length
    uint8_t device_name_[53];       // Human-readable device name
};

#endif //NODEINFOREQUEST_H

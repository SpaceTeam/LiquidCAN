//
// Created by tobias on 30.01.26.
//

#ifndef STATUS_H
#define STATUS_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Status : public Can_message
{
public:
    Status();
    Status(const uint8_t* message, uint8_t length);

    bool set_message(const uint8_t *msg, uint8_t length);
    const uint8_t* get_message() const;
    uint8_t get_message_length() const;
private:
    uint8_t msg_length_;    // Length of message string
    uint8_t msg_[63];      // Message string
};

#endif //STATUS_H

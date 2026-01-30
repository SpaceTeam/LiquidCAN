//
// Created by tobias on 30.01.26.
//

#ifndef HEARTBEAT_H
#define HEARTBEAT_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Heart_beat : public Can_message
{
public:
    Heart_beat();
    Heart_beat(uint32_t counter);

    uint32_t set_counter(uint32_t value);
    uint32_t get_counter() const;

    Heart_beat& operator++();
private:
    uint32_t counter_;    // Incrementing counter value
};

#endif //HEARTBEAT_H

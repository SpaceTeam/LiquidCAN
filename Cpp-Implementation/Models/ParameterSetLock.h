//
// Created by tobias on 30.01.26.
//

#ifndef PARAMETERSETLOCK_H
#define PARAMETERSETLOCK_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>

class Parameter_set_lock : public Can_message
{
public:
    Parameter_set_lock();
    Parameter_set_lock(uint8_t parameter_id, uint8_t lock_status);

    bool set_parameter_id(uint8_t parameter_id);
    uint8_t get_parameter_id() const;

    bool set_lock_status(uint8_t lock_status);
    bool get_lock_status() const;
private:
    uint8_t parameter_id_;             // Parameter identifier to lock
    uint8_t parameter_lock_status_;    // Lock status (0=locked, 1=unlocked)
};

#endif //PARAMETERSETLOCK_H

//
// Created by tobias on 30.01.26.
//

#ifndef PARAMETERSETSTATUS_H
#define PARAMETERSETSTATUS_H

#include <cstdint>

enum Parameter_set_status : uint8_t
{
    success = 0,                    // Parameter was successfully set
    invalid_parameter_id = 1,       // The Parameter ID does not exist
    parameter_locked = 2,           // The parameter is locked and cannot be modified
    node_to_node_modification = 3   // The parameter was modified by another node
};

#endif //PARAMETERSETSTATUS_H

//
// Created by tobias on 19.01.26.
//

#ifndef MESSAGETYPE_H
#define MESSAGETYPE_H

enum Can_message_type
{
    node_info_request = 0,                    // Request node information
    node_info_announcement = 1,               // Response with node capabilities and identification
    info_status = 10,                         // Information status message
    warning_status = 11,                      // Warning status message
    error_status = 12,                        // Error status message
    telemetry_value_registration = 20,        // Register a telemetry Value field
    parameter_registration = 21,              // Register a parameter field
    telemetry_group_definition = 30,          // Define a group of fields for batch updates
    telemetry_group_update = 31,              // Update values for a field group
    heartbeat_request = 40,                   // Heartbeat request
    heartbeat_response = 41,                  // Heartbeat response
    parameter_set_request = 50,               // Request to set a parameter value
    parameter_set_confirmation = 51,          // Response with confirmed parameter value
    parameter_set_lock_request = 52,          // Request to lock a parameter
    parameter_set_lock_confirmation = 53,     // Response confirming parameter lock
    field_get_request = 60,                   // Request field value
    field_get_response = 61,                  // Response with field value
    field_id_lookup_request = 62,             // Request the fieldID of a field name
    field_id_lookup_response = 63             // Response with fieldID
};

#endif //MESSAGETYPE_H
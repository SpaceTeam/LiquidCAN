//
// Created by tobias on 30.01.26.
//

#ifndef FIELDIDLOOKUPRESULT_H
#define FIELDIDLOOKUPRESULT_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>
#include <Infrastructure/DataType.h>


class Field_id_lookup_result : public Can_message
{
public:
    Field_id_lookup_result();
    Field_id_lookup_result(uint8_t field_id, Data_type data_type);

    bool set_field_id(uint8_t field_id);
    uint8_t get_field_id() const;

    bool set_data_type(Data_type data_type);
    Data_type get_data_type() const;
private:
    uint8_t field_id_;        // Field ID
    Data_type field_type_;    // Field data type
};

#endif //FIELDIDLOOKUPRESULT_H

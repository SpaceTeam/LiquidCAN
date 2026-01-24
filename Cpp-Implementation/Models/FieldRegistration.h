#ifndef FIELDREGISTRATION_H
#define FIELDREGISTRATION_H

#include <cstdint>
#include <Infrastructure/CanMessage.h>
#include <Infrastructure/DataType.h>

class FieldRegistration : public CanMessage{
public:
    FieldRegistration();
    FieldRegistration(uint8_t field_id, DataType field_type, const char* device_name);

    bool setFieldId(uint8_t field_id);
    uint8_t getFieldId();

    bool setFieldType(DataType field_type);
    DataType getFieldType();

    bool setFieldName(const char* field_name);
    const char* getFieldName() const;

private:
    uint8_t field_id_;      // Unique identifier for this field
    DataType field_type_;    // Field DataType
    char field_name_[63];   // Human-readable field name
};

#endif // FIELDREGISTRATION_H

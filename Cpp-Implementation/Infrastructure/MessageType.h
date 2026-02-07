#ifndef MESSAGETYPE_H
#define MESSAGETYPE_H

#include <cstdint>

enum MessageType : uint8_t
{
	NodeInfoReq = 0,
	NodeInfoAnnouncement = 1,
	InfoStatus = 10,
	WarningStatus = 11,
	ErrorStatus = 12,
	TelemetryValueRegistration = 20,
	ParameterRegistration = 21,
	TelemetryGroupDefinition = 30,
	TelemetryGroupUpdate = 31,
	HeartbeatReq = 40,
	HeartbeatRes = 41,
	ParameterSetReq = 50,
	ParameterSetConfirmation = 51,
	ParameterSetLock = 52,
	ParameterSetLockConfirmation = 53,
	FieldGetReq = 60,
	FieldGetRes = 61,
	FieldIdLookupReq = 62,
	FieldIdLookupRes = 63,
}

#endif
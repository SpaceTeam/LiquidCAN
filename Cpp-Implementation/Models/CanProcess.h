//
// Created by tobias on 17.01.26.
//

#ifndef CANPROCESS_H
#define CANPROCESS_H

#include <CanProcessStatus.h>
#include <CanMessageType.h>

/*
 * @brief Template class for CAN-Process Management
 *
 * This class stores the status and value of a given CAN process and
 * allows querying the process type and value
 *
 * @tparam T Datatype of the expected value of the CAN-Process e.g. int or float
 */
template <typename T>
class CanProcess {
    public:
		/*
 		 * @brief Construct a CAN process with the specified MessageTypes
 		 *
         * @param processType is the CanMessageType of the message witch started the process
		 * @param responseType is the CanMessageType of the expected response message
		 * @param fieldId is the optinal field identifier used to further link the process to a message, defaults to 0
         *
         * @post Process is initialized with
		 * 		status = started
		 * 		value = default
		 *		processType, responseType and fieldId set
         */
        CanProcess(CanMessageType processType, CanMessageType responseType, int fieldId = 0);

		/*
		 * @brief Sets the status of the CAN process
		 *
		 * @param status is the new status to set
		 *
		 * @pre The status must be "NEW" in order to be canged
		 * @post The internal process status is updated
		 */
        virtual void setStatus(CanProcessStatus status);

		/*
		 * @brief Returns the current status of a the CAN process
		 *
		 * @return Current CanProcessStatus fo the process
		 */
        virtual CanProcessStatus getStatus();

		/*
		 * @brief Sets the value ot the CAN process
		 *
		 * @param value is the new value of type T
		 *
		 * @post The internal value is updated
		 */
        virtual void setValue(T value);

		/*
		 * @brief Return the value to the CAN process
		 *
		 * @pre status must be DONE
		 *
		 * @return Current value to type T
		 */
        virtual T getValue();

		/*
		 * @brief Returns the type of the CAN process
		 *
		 * @return CanMessageType that started the process
		 */
		virtual CanMessageType getProcessType();

		/*
		 * @brief Returns the expected type of the response message
		 *
		 * @return Expected CanMessageType ot the response
		 */
		virtual CanMessageType getProcessResponseType();

    private:
        CanProcessStatus status_;
        T value_;
		CanMessageType processType_;
		CanMessageType responseType_;
		int fieldId_;

        // Disable coping
        CanProcess(const CanProcess &);
        CanProcess &operator=(const CanProcess &);
};

#endif //CANPROCESS_H

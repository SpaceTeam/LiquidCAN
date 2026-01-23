
#ifndef LIQUIDCAN_H
#define LIQUIDCAN_H

/*
 * @brief Class profiding an interface to the Liquid CAN prodocoll
 *
 * This interface declass methodes to handle
 * 		initialization,
 *		external parameter access
 *		process management
 *		status reporting
 */
class LiquidCan {
   	public:
		/*
		 * @brief Stats the initialisation procedure for the CAN System
		 *
		 * @pre parameter and telemetrie fields must be registerd
		 */
    	virtual void startInitProcedure();

		/*
		 * @brief Updated an external parameter by id and sets a given value
		 *
		 * @tparam T datatype of the external parameter
		 * @param id identifier of the external parameter
		 * @param data Value to set the external parameter
		 *
		 * @pre LiquidCan system must bei initialized
		 * @pre id must be correspond to a existing external id
		 * @post A CanProcess is started witch awaits the verification of the parameter update
		 *
		 * @returns processId witch tracks the response status and value
		 */
    	template <typename T>
    	virtual int updateExternalParameter<T>(int id, T data);

		/*
		 * @brief Retreives the value ot an external parameter
		 *
		 * @tparam T datatype of the external parameter
		 * @param id identifier of the external parameter
		 * @param data reference to store retreived data
		 *
		 * @pre LiquidCan system must bei initialized
		 * @pre id must be correspond to a existing external id
		 * @post A CanProcess is started witch awaits the verification of the parameter update
		 *
		 * @returns processId witch tracks the response status
		 */
    	template <typename T>
    	virtual int getExternalParameter(int id, T &data);

		/*
		 * @brief Locks an external parameter
		 *
		 * @param id identifier of the external parameter
		 *
		 * @pre LiquidCan system must bei initialized
		 * @pre id must be correspond to a existing external id
		 * @post A CanProcess is started witch awaits the verification of the parameter update
		 *
		 * @returns processId witch tracks the response status
		 */
    	virtual int lockExternalParameter(int id)

		/*
		 * @brief Unlocks an external parameter
		 *
		 * @param id identifier of the external parameter
		 *
		 * @pre LiquidCan system must bei initialized
		 * @pre id must be correspond to a existing external id
		 * @post A CanProcess is started witch awaits the verification of the parameter update
		 *
		 * @returns processId witch tracks the response status
		 */
    	virtual int unlockExternalParameter(int id);

		/*
		 * @brief Sends an information status message
		 *
		 * @param message as string
		 *
		 * @pre LiquidCan system must bei initialized
		 */
    	virtual void sendInfoStatus(std::string message);

		/*
		 * @brief Sends an warning status message
		 *
		 * @param message as string
		 *
		 * @pre LiquidCan system must bei initialized
		 */
    	virtual void sendWarningStatus(std::string message);

		/*
		 * @brief Sends an error status message
		 *
		 * @param message as string
		 *
		 * @pre LiquidCan system must bei initialized
		 */
    	virtual void sendErrorStatus(std::string message);

		/*
		 * @brief Retreives external fieldId by name
		 *
		 * @param name Name of the external field
		 *
		 * @pre LiquidCan system must bei initialized
		 * @post A CanProcess is started witch awaits the verification of the parameter update
		 *
		 * @returns processId witch tracks the response status
		 */
    	virtual int lookupFieldIdByName(std::string name);

		/*
		 * @brief Returns the value of a finished CanProcess
		 *
		 * @tparam T datatype of the response
		 * @param id identifier of the CanProcess
		 *
		 * @pre id must be correspond to a existing CanProcess
		 * @pre CanProcess muss have the status DONE
		 *
		 * @returns Value to CanProcess or nullptr (if not ready or failed)
		 */
    	template <typename T>
    	virtual T* getProcessResponse(int processId);

		/*
		 * @brief Checks if the response of a CanProcess is read
		 *
		 * @param id identifier of the CanProcess
		 *
		 * @pre id must be correspond to a existing CanProcess
		 *
		 * @returns Returns true if the CanProcess has status Done
		 */
    	virtual bool isProcessReady(int processId);

		/*
		 * @brief Checks if a CanProcess is active
		 *
		 * @param id identifier of the CanProcess
		 *
		 * @pre id must be correspond to a existing CanProcess
		 *
		 * @returns Returns true if the CanProcess has status Started
		 */
    	virtual bool isProcessActive(int processId);

		/*
		 * @brief Checks if a CanProcess is falied
		 *
		 * @param id identifier of the CanProcess
		 *
		 * @pre id must be correspond to a existing CanProcess
		 *
		 * @returns Returns true if the CanProcess has status Error
		 */
    	virtual bool isProcessFailed(int processId);

    	private:
        // Disable coping
    	LiquidCan(LiquidCan const &);
    	LiquidCan& operator=(LiquidCan const &);
}
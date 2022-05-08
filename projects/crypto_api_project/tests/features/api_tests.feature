Feature: Test Public REST API Consumption
    Users should be able to submit GET requests to the public web service,

    Background:
        Given an authorized http GET request to the public service

    Scenario: User is able to retrieve server time from web service
        When user wants to get server time from the service
        Then the server time request status is OK
        And the user retrieves the server time successfully
    
    Scenario: User is able to retrieve asset data for XBT/USD pairing
        When user wants to get XBT/USD asset data from the service
        Then the asset data request status is OK
        And the user retrieves the asset data successfully

    Background:
        Given an authorized http POST request to the private service
    

    Scenario: User is able to retrieve orders from the web service
        When user retrieves open orders from the open orders endpoint
        Then the open order request status is OK
        And the user retrieves the open orders successfully
    
    Scenario: 2FA on the user profile is active
        When the user tries to access a private endpoint with 2FA enabled without otp in payload   
        Then user recieves an unauthorized status code
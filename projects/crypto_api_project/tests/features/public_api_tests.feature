Feature: Test public REST API Consumption
    Users should be able to submit successful requests to the public web service,
    for information.

    Scenario: User is able to retrieve server time from web service
        When user wants to get server time from the service
        Then the server time request status is OK
        And the user retrieves the server time successfully
    
    Scenario: User is able to retrieve asset data for XBT/USD pairing
        When user wants to get XBT/USD asset data from the service
        Then the asset data request status is OK
        And the user retrieves the asset data successfully


Feature: Test public REST API Consumption
    Users should be able to submit successful requests to the public web service,
    for information.

    Background: Given a http GET request to the public web service

    Scenario: User retrieves server time from web service
        When user wants to get server time from the service
        Then the server time request status is OK
        And the user retrieves the server time successfully
    
    Scenario: User retrieves asset data for XBT/USD pairing
        When user wants to get XBT/USD asset data from the service
        Then the asset data request status is OK
        And the user retrieves the asset data successfully

    Scenario: User retrieves asset data with valid query params
        When the user wants to get XXBTZUSD,XETHXXBT asset pair data from the service
        Then the asset data request status is OK    
        And the user retrieves the query asset data successfully

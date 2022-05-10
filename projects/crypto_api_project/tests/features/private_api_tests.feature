Feature: Test private REST API Consumption
  
    Scenario: User is trying to retrieve open orders with valid key and sign
        Given an authorized http POST request to the private service
        When user retrieves open orders from the open orders endpoint with valid key and sign
        Then the open order request status is OK
        And the user retrieves the open orders successfully
    
    Scenario: User is trying to retrieve open orders with invalid key
        Given an unauthorized http POST request to the private service
        When user retrieves open orders from the open orders endpoint with invalid key
        Then the open order request status is OK
        And the user receives an invalid key error
    
    Scenario: User is trying to retrieve open orders with invalid sign
        Given an unauthorized http POST request to the private service
        When user retrieves open orders from the open orders endpoint with invalid sign
        Then the open order request status is OK
        And the user recieves an invalid sign error
    
    # Scenario: 2FA on the user profile is active
    #     When the user tries to access a private endpoint with 2FA enabled without otp in payload   
    #     Then user recieves an unauthorized status code
# service-api-testing
Tests a REST API implementation, by consuming the REST API and returning values.

# Known Issues
Due to a failure to get git-crypt or git-secret on to my Windows machine key-obfuscation is not ideal.
For this reason I didn't want to include a real Google Auth secret in the file so, for the private 
endpoints while 2FA logic is there it is not functional.
Lastly, the cucumber feature for output-json gave me some trouble, while the flag works not sure why I can't see
an output.json file after runnning the tests.

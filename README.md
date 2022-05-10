# service-api-testing
Tests a REST API implementation, by consuming the REST API and returning values.

# Known Issues
Due to a failure to get git-crypt or git-secret on to my Windows machine key-obfuscation is not ideal.

For this reason I didn't want to include a real Google Auth secret in the file so, for the private 
endpoints 2FA logic is there didn't enable it for my endpoints though.

Lastly, the cucumber feature for output-json gave me some trouble. Therefore, reports are busted for now, should find a
fix soon, but sadly I'm out of time.

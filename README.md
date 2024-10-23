# Name
## Status of developmenent
**Work in Progress**
## About
library to manage different backend for SaaS with the same methods.
## Objective
Allows to write services without relying on a specific backend.
## Features
clients for:  
- Product API
- Tasks tracker API
- Cache API
## Installation
## Usage
Implement the trait for clients, allowing program to plug backends without the need for different implementation.  
The client library can use a struct Client(reqwest::Client) and implement the trait of the corresponding backend.  
Deref can be automatically implemented with the crate derive_more to make the implementation more clean.  
## Bug Reporting
## Contributing
## Security
## Documentation
## License
GNU GPL v3

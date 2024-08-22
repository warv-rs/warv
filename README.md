# WARV
Warv [/var:v/] or Webapplikations Ramverk is a lightweight web application framework designed for high-performance and concurrent web services. Built on top of the [may](https://github.com/Xudong-Huang/may), which provides coroutine-based concurrency in Rust, Warv leverages this capability to efficiently handle multiple connections with minimal overhead. 

### Key features:
#### Concurrency with may:
Warv utilizes may's coroutine-based concurrency model, enabling it to handle a large number of concurrent connections efficiently. may's lightweight coroutines allow Warv to avoid the overhead associated with traditional threading models, making it highly scalable.

#### Flexible Routing:
Warv supports flexible routing with both stateless and stateful handlers. Developers can define routes that either require or do not require shared state, providing flexibility in how web requests are processed.
The framework supports method-specific routing (e.g., GET, POST) and allows the use of a default handler for unmatched routes.

#### Middleware Support:
Warv includes a robust middleware system, enabling developers to insert custom logic (e.g., logging, authentication, CORS) before and after request handling.

#### State Management:
Warv's routing system allows for optional state injection, meaning handlers can operate with or without state depending on the use case.
State is managed and passed around using Arc\<dyn State\>, providing flexibility while maintaining thread safety.

#### Request Handling:
Warv supports complex request handling scenarios, including path parameter extraction and query parsing. This allows for dynamic and customizable request processing, catering to various web application needs.

#### TLS Support:
Warv integrates with [rustls](https://github.com/rustls/rustls) to provide secure connections via TLS. This ensures that data transmitted over the network is encrypted, enhancing security for web applications.

#### Static File Serving:
Warv includes functionality to serve static files and directories, making it suitable for use cases like hosting static websites or serving assets in a dynamic application.
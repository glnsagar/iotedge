/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release of Docker, so API calls are versioned to ensure that clients don't break.  For Docker Engine 17.10, the API version is 1.33. To lock to this version, you prefix the URL with `/v1.33`. For example, calling `/info` is the same as calling `/v1.33/info`.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  In previous versions of Docker, it was possible to access the API without providing a version. This behaviour is now deprecated will be removed in a future version of Docker.  If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer Docker daemons.  This documentation is for version 1.34 of the API. Use this table to find documentation for previous versions of the API:  Docker version  | API version | Changes ----------------|-------------|--------- 17.10.x | [1.33](https://docs.docker.com/engine/api/v1.33/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-33-api-changes) 17.09.x | [1.32](https://docs.docker.com/engine/api/v1.32/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-32-api-changes) 17.07.x | [1.31](https://docs.docker.com/engine/api/v1.31/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-31-api-changes) 17.06.x | [1.30](https://docs.docker.com/engine/api/v1.30/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-30-api-changes) 17.05.x | [1.29](https://docs.docker.com/engine/api/v1.29/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-29-api-changes) 17.04.x | [1.28](https://docs.docker.com/engine/api/v1.28/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-28-api-changes) 17.03.1 | [1.27](https://docs.docker.com/engine/api/v1.27/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-27-api-changes) 1.13.1 & 17.03.0 | [1.26](https://docs.docker.com/engine/api/v1.26/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-26-api-changes) 1.13.0 | [1.25](https://docs.docker.com/engine/api/v1.25/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-25-api-changes) 1.12.x | [1.24](https://docs.docker.com/engine/api/v1.24/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-24-api-changes) 1.11.x | [1.23](https://docs.docker.com/engine/api/v1.23/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-23-api-changes) 1.10.x | [1.22](https://docs.docker.com/engine/api/v1.22/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-22-api-changes) 1.9.x | [1.21](https://docs.docker.com/engine/api/v1.21/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-21-api-changes) 1.8.x | [1.20](https://docs.docker.com/engine/api/v1.20/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-20-api-changes) 1.7.x | [1.19](https://docs.docker.com/engine/api/v1.19/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-19-api-changes) 1.6.x | [1.18](https://docs.docker.com/engine/api/v1.18/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-18-api-changes)  # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a Base64 encoded (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.34
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResourceObject : An object describing the resources which can be advertised by a node and requested by a task

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResourceObject {
    #[serde(rename = "NanoCPUs", skip_serializing_if = "Option::is_none")]
    nano_cp_us: Option<i64>,
    #[serde(rename = "MemoryBytes", skip_serializing_if = "Option::is_none")]
    memory_bytes: Option<i64>,
    #[serde(rename = "GenericResources", skip_serializing_if = "Option::is_none")]
    generic_resources: Option<crate::models::GenericResources>,
}

impl ResourceObject {
    /// An object describing the resources which can be advertised by a node and requested by a task
    pub fn new() -> Self {
        ResourceObject {
            nano_cp_us: None,
            memory_bytes: None,
            generic_resources: None,
        }
    }

    pub fn set_nano_cp_us(&mut self, nano_cp_us: i64) {
        self.nano_cp_us = Some(nano_cp_us);
    }

    pub fn with_nano_cp_us(mut self, nano_cp_us: i64) -> Self {
        self.nano_cp_us = Some(nano_cp_us);
        self
    }

    pub fn nano_cp_us(&self) -> Option<i64> {
        self.nano_cp_us
    }

    pub fn reset_nano_cp_us(&mut self) {
        self.nano_cp_us = None;
    }

    pub fn set_memory_bytes(&mut self, memory_bytes: i64) {
        self.memory_bytes = Some(memory_bytes);
    }

    pub fn with_memory_bytes(mut self, memory_bytes: i64) -> Self {
        self.memory_bytes = Some(memory_bytes);
        self
    }

    pub fn memory_bytes(&self) -> Option<i64> {
        self.memory_bytes
    }

    pub fn reset_memory_bytes(&mut self) {
        self.memory_bytes = None;
    }

    pub fn set_generic_resources(&mut self, generic_resources: crate::models::GenericResources) {
        self.generic_resources = Some(generic_resources);
    }

    pub fn with_generic_resources(
        mut self,
        generic_resources: crate::models::GenericResources,
    ) -> Self {
        self.generic_resources = Some(generic_resources);
        self
    }

    pub fn generic_resources(&self) -> Option<&crate::models::GenericResources> {
        self.generic_resources.as_ref()
    }

    pub fn reset_generic_resources(&mut self) {
        self.generic_resources = None;
    }
}

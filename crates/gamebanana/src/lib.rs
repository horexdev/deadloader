//! The only code that knows GameBanana's URLs, query parameters and JSON fields. It maps API responses into
//! deadloader's own types and reports an incompatible API as a distinct error. It talks to the network through a
//! transport trait, never directly.

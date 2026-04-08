// @generated
/// Generated client implementations.
pub mod access_code_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages early access codes for organization creation gating.
 All RPCs require API key authentication (platform-level, not org-scoped).
*/
    #[derive(Debug, Clone)]
    pub struct AccessCodeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccessCodeServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccessCodeServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccessCodeServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AccessCodeServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Generate one or more access codes with an optional label.
 Authorization: Requires API key auth (service-level).
*/
        pub async fn generate_access_codes(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAccessCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAccessCodesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.AccessCodeService/GenerateAccessCodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.AccessCodeService", "GenerateAccessCodes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List all access codes (active, redeemed, and revoked).
 Authorization: Requires API key auth (service-level).
*/
        pub async fn list_access_codes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessCodesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.AccessCodeService/ListAccessCodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.AccessCodeService", "ListAccessCodes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Revoke an access code. Revoked codes cannot be used for organization creation.
 Authorization: Requires API key auth (service-level).
*/
        pub async fn revoke_access_code(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeAccessCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeAccessCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.AccessCodeService/RevokeAccessCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.AccessCodeService", "RevokeAccessCode"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod access_code_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccessCodeServiceServer.
    #[async_trait]
    pub trait AccessCodeService: std::marker::Send + std::marker::Sync + 'static {
        /** Generate one or more access codes with an optional label.
 Authorization: Requires API key auth (service-level).
*/
        async fn generate_access_codes(
            &self,
            request: tonic::Request<super::GenerateAccessCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAccessCodesResponse>,
            tonic::Status,
        >;
        /** List all access codes (active, redeemed, and revoked).
 Authorization: Requires API key auth (service-level).
*/
        async fn list_access_codes(
            &self,
            request: tonic::Request<super::ListAccessCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessCodesResponse>,
            tonic::Status,
        >;
        /** Revoke an access code. Revoked codes cannot be used for organization creation.
 Authorization: Requires API key auth (service-level).
*/
        async fn revoke_access_code(
            &self,
            request: tonic::Request<super::RevokeAccessCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeAccessCodeResponse>,
            tonic::Status,
        >;
    }
    /** Manages early access codes for organization creation gating.
 All RPCs require API key authentication (platform-level, not org-scoped).
*/
    #[derive(Debug)]
    pub struct AccessCodeServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AccessCodeServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccessCodeServiceServer<T>
    where
        T: AccessCodeService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.AccessCodeService/GenerateAccessCodes" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateAccessCodesSvc<T: AccessCodeService>(pub Arc<T>);
                    impl<
                        T: AccessCodeService,
                    > tonic::server::UnaryService<super::GenerateAccessCodesRequest>
                    for GenerateAccessCodesSvc<T> {
                        type Response = super::GenerateAccessCodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateAccessCodesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccessCodeService>::generate_access_codes(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GenerateAccessCodesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.AccessCodeService/ListAccessCodes" => {
                    #[allow(non_camel_case_types)]
                    struct ListAccessCodesSvc<T: AccessCodeService>(pub Arc<T>);
                    impl<
                        T: AccessCodeService,
                    > tonic::server::UnaryService<super::ListAccessCodesRequest>
                    for ListAccessCodesSvc<T> {
                        type Response = super::ListAccessCodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAccessCodesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccessCodeService>::list_access_codes(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListAccessCodesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.AccessCodeService/RevokeAccessCode" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeAccessCodeSvc<T: AccessCodeService>(pub Arc<T>);
                    impl<
                        T: AccessCodeService,
                    > tonic::server::UnaryService<super::RevokeAccessCodeRequest>
                    for RevokeAccessCodeSvc<T> {
                        type Response = super::RevokeAccessCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeAccessCodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccessCodeService>::revoke_access_code(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RevokeAccessCodeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AccessCodeServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.AccessCodeService";
    impl<T> tonic::server::NamedService for AccessCodeServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod action_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Handles user actions on delivered messages.
 Actions drive workflow progression (e.g. ACK completes a wait step).
*/
    #[derive(Debug, Clone)]
    pub struct ActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ActionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ActionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ActionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Submit an action for a specific delivery, advancing the campaign workflow.
 Backend MUST verify the authenticated user is the delivery recipient.
*/
        pub async fn submit_action(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitActionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.ActionService/SubmitAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.ActionService", "SubmitAction"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod action_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ActionServiceServer.
    #[async_trait]
    pub trait ActionService: std::marker::Send + std::marker::Sync + 'static {
        /** Submit an action for a specific delivery, advancing the campaign workflow.
 Backend MUST verify the authenticated user is the delivery recipient.
*/
        async fn submit_action(
            &self,
            request: tonic::Request<super::SubmitActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitActionResponse>,
            tonic::Status,
        >;
    }
    /** Handles user actions on delivered messages.
 Actions drive workflow progression (e.g. ACK completes a wait step).
*/
    #[derive(Debug)]
    pub struct ActionServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ActionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ActionServiceServer<T>
    where
        T: ActionService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.ActionService/SubmitAction" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitActionSvc<T: ActionService>(pub Arc<T>);
                    impl<
                        T: ActionService,
                    > tonic::server::UnaryService<super::SubmitActionRequest>
                    for SubmitActionSvc<T> {
                        type Response = super::SubmitActionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitActionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ActionService>::submit_action(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SubmitActionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ActionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.ActionService";
    impl<T> tonic::server::NamedService for ActionServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod api_key_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages scoped API keys for programmatic access.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug, Clone)]
    pub struct ApiKeyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiKeyServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ApiKeyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ApiKeyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ApiKeyServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new scoped API key with specific permissions.
 The full secret key is only returned in the response — store it securely.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn create_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApiKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.ApiKeyService/CreateApiKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.ApiKeyService", "CreateApiKey"));
            self.inner.unary(req, path, codec).await
        }
        /** List all active API keys in the organization (metadata only, no secrets).
 Authorization: Requires PERMISSION_ORG_READ.
*/
        pub async fn list_api_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApiKeysResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.ApiKeyService/ListApiKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.ApiKeyService", "ListApiKeys"));
            self.inner.unary(req, path, codec).await
        }
        /** Revoke an API key. The key becomes immediately unusable.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn revoke_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeApiKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.ApiKeyService/RevokeApiKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.ApiKeyService", "RevokeApiKey"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod api_key_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ApiKeyServiceServer.
    #[async_trait]
    pub trait ApiKeyService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new scoped API key with specific permissions.
 The full secret key is only returned in the response — store it securely.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn create_api_key(
            &self,
            request: tonic::Request<super::CreateApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApiKeyResponse>,
            tonic::Status,
        >;
        /** List all active API keys in the organization (metadata only, no secrets).
 Authorization: Requires PERMISSION_ORG_READ.
*/
        async fn list_api_keys(
            &self,
            request: tonic::Request<super::ListApiKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApiKeysResponse>,
            tonic::Status,
        >;
        /** Revoke an API key. The key becomes immediately unusable.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn revoke_api_key(
            &self,
            request: tonic::Request<super::RevokeApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeApiKeyResponse>,
            tonic::Status,
        >;
    }
    /** Manages scoped API keys for programmatic access.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug)]
    pub struct ApiKeyServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ApiKeyServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiKeyServiceServer<T>
    where
        T: ApiKeyService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.ApiKeyService/CreateApiKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateApiKeySvc<T: ApiKeyService>(pub Arc<T>);
                    impl<
                        T: ApiKeyService,
                    > tonic::server::UnaryService<super::CreateApiKeyRequest>
                    for CreateApiKeySvc<T> {
                        type Response = super::CreateApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiKeyService>::create_api_key(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateApiKeySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.ApiKeyService/ListApiKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListApiKeysSvc<T: ApiKeyService>(pub Arc<T>);
                    impl<
                        T: ApiKeyService,
                    > tonic::server::UnaryService<super::ListApiKeysRequest>
                    for ListApiKeysSvc<T> {
                        type Response = super::ListApiKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListApiKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiKeyService>::list_api_keys(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListApiKeysSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.ApiKeyService/RevokeApiKey" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeApiKeySvc<T: ApiKeyService>(pub Arc<T>);
                    impl<
                        T: ApiKeyService,
                    > tonic::server::UnaryService<super::RevokeApiKeyRequest>
                    for RevokeApiKeySvc<T> {
                        type Response = super::RevokeApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiKeyService>::revoke_api_key(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RevokeApiKeySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ApiKeyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.ApiKeyService";
    impl<T> tonic::server::NamedService for ApiKeyServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod privacy_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** PrivacyService handles GDPR/LGPD data subject rights.
 All RPCs extract org_id from the JWT — it is never in request messages.
*/
    #[derive(Debug, Clone)]
    pub struct PrivacyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PrivacyServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PrivacyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PrivacyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            PrivacyServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Export all personal data associated with a user as a downloadable ZIP.
 Async operation — returns immediately with PENDING status, sends push
 notification when the export is ready.
 Auth: Requires JWT. Callable by the user themselves or an org admin.
*/
        pub async fn export_user_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportUserDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportUserDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/ExportUserData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.PrivacyService", "ExportUserData"));
            self.inner.unary(req, path, codec).await
        }
        /** Delete or anonymize all personal data associated with a user.
 Deletion has a 30-day grace period during which processing is restricted
 and the request can be cancelled. After 30 days, deletion is irreversible.
 Auth: Requires JWT. Admin only.
*/
        pub async fn delete_user_data(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/DeleteUserData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.PrivacyService", "DeleteUserData"));
            self.inner.unary(req, path, codec).await
        }
        /** Correct personal data for a user. Propagates corrections to all stored
 locations (profile, delivery records, analytics metadata).
 Auth: Requires JWT. Callable by the user themselves or an org admin.
*/
        pub async fn rectify_user_data(
            &mut self,
            request: impl tonic::IntoRequest<super::RectifyUserDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RectifyUserDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/RectifyUserData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.PrivacyService", "RectifyUserData"));
            self.inner.unary(req, path, codec).await
        }
        /** Restrict or unrestrict processing for a user. When restricted, the API
 skips this user in campaigns, analytics, and session replay.
 Auth: Requires JWT. Admin only.
*/
        pub async fn restrict_processing(
            &mut self,
            request: impl tonic::IntoRequest<super::RestrictProcessingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RestrictProcessingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/RestrictProcessing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.PrivacyService", "RestrictProcessing"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Confirm whether personal data exists for a user and list data categories.
 LGPD-specific: confirmação de existência (Art. 18, I).
 Auth: Requires JWT. Admin only.
*/
        pub async fn get_data_existence_confirmation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataExistenceConfirmationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataExistenceConfirmationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/GetDataExistenceConfirmation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.PrivacyService",
                        "GetDataExistenceConfirmation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List privacy requests for the organization, with optional filters.
 Used by the admin UI to show scheduled deletions table.
 Auth: Requires JWT. Admin only.
*/
        pub async fn list_privacy_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrivacyRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrivacyRequestsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/ListPrivacyRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.PrivacyService", "ListPrivacyRequests"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Cancel a pending deletion request. Reactivates the user and aborts
 the deletion workflow. Only valid during the 30-day grace period.
 Auth: Requires JWT. Admin only.
*/
        pub async fn cancel_deletion(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelDeletionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelDeletionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/CancelDeletion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.PrivacyService", "CancelDeletion"));
            self.inner.unary(req, path, codec).await
        }
        /** Skip the grace period and delete immediately. Signals the deletion
 workflow to proceed without waiting for the 30-day timer.
 Auth: Requires JWT. Admin only.
*/
        pub async fn immediate_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ImmediateDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImmediateDeleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/ImmediateDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.PrivacyService", "ImmediateDelete"));
            self.inner.unary(req, path, codec).await
        }
        /** List the calling user's own privacy requests (export, rectify).
 The server extracts user_id from the JWT — no admin permission required.
 Auth: Requires JWT. Any authenticated user.
*/
        pub async fn list_my_privacy_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyPrivacyRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyPrivacyRequestsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.PrivacyService/ListMyPrivacyRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.PrivacyService", "ListMyPrivacyRequests"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod privacy_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PrivacyServiceServer.
    #[async_trait]
    pub trait PrivacyService: std::marker::Send + std::marker::Sync + 'static {
        /** Export all personal data associated with a user as a downloadable ZIP.
 Async operation — returns immediately with PENDING status, sends push
 notification when the export is ready.
 Auth: Requires JWT. Callable by the user themselves or an org admin.
*/
        async fn export_user_data(
            &self,
            request: tonic::Request<super::ExportUserDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportUserDataResponse>,
            tonic::Status,
        >;
        /** Delete or anonymize all personal data associated with a user.
 Deletion has a 30-day grace period during which processing is restricted
 and the request can be cancelled. After 30 days, deletion is irreversible.
 Auth: Requires JWT. Admin only.
*/
        async fn delete_user_data(
            &self,
            request: tonic::Request<super::DeleteUserDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserDataResponse>,
            tonic::Status,
        >;
        /** Correct personal data for a user. Propagates corrections to all stored
 locations (profile, delivery records, analytics metadata).
 Auth: Requires JWT. Callable by the user themselves or an org admin.
*/
        async fn rectify_user_data(
            &self,
            request: tonic::Request<super::RectifyUserDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RectifyUserDataResponse>,
            tonic::Status,
        >;
        /** Restrict or unrestrict processing for a user. When restricted, the API
 skips this user in campaigns, analytics, and session replay.
 Auth: Requires JWT. Admin only.
*/
        async fn restrict_processing(
            &self,
            request: tonic::Request<super::RestrictProcessingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RestrictProcessingResponse>,
            tonic::Status,
        >;
        /** Confirm whether personal data exists for a user and list data categories.
 LGPD-specific: confirmação de existência (Art. 18, I).
 Auth: Requires JWT. Admin only.
*/
        async fn get_data_existence_confirmation(
            &self,
            request: tonic::Request<super::GetDataExistenceConfirmationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataExistenceConfirmationResponse>,
            tonic::Status,
        >;
        /** List privacy requests for the organization, with optional filters.
 Used by the admin UI to show scheduled deletions table.
 Auth: Requires JWT. Admin only.
*/
        async fn list_privacy_requests(
            &self,
            request: tonic::Request<super::ListPrivacyRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrivacyRequestsResponse>,
            tonic::Status,
        >;
        /** Cancel a pending deletion request. Reactivates the user and aborts
 the deletion workflow. Only valid during the 30-day grace period.
 Auth: Requires JWT. Admin only.
*/
        async fn cancel_deletion(
            &self,
            request: tonic::Request<super::CancelDeletionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelDeletionResponse>,
            tonic::Status,
        >;
        /** Skip the grace period and delete immediately. Signals the deletion
 workflow to proceed without waiting for the 30-day timer.
 Auth: Requires JWT. Admin only.
*/
        async fn immediate_delete(
            &self,
            request: tonic::Request<super::ImmediateDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImmediateDeleteResponse>,
            tonic::Status,
        >;
        /** List the calling user's own privacy requests (export, rectify).
 The server extracts user_id from the JWT — no admin permission required.
 Auth: Requires JWT. Any authenticated user.
*/
        async fn list_my_privacy_requests(
            &self,
            request: tonic::Request<super::ListMyPrivacyRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyPrivacyRequestsResponse>,
            tonic::Status,
        >;
    }
    /** PrivacyService handles GDPR/LGPD data subject rights.
 All RPCs extract org_id from the JWT — it is never in request messages.
*/
    #[derive(Debug)]
    pub struct PrivacyServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PrivacyServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PrivacyServiceServer<T>
    where
        T: PrivacyService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.PrivacyService/ExportUserData" => {
                    #[allow(non_camel_case_types)]
                    struct ExportUserDataSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::ExportUserDataRequest>
                    for ExportUserDataSvc<T> {
                        type Response = super::ExportUserDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportUserDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::export_user_data(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ExportUserDataSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/DeleteUserData" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserDataSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::DeleteUserDataRequest>
                    for DeleteUserDataSvc<T> {
                        type Response = super::DeleteUserDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::delete_user_data(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteUserDataSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/RectifyUserData" => {
                    #[allow(non_camel_case_types)]
                    struct RectifyUserDataSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::RectifyUserDataRequest>
                    for RectifyUserDataSvc<T> {
                        type Response = super::RectifyUserDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RectifyUserDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::rectify_user_data(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RectifyUserDataSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/RestrictProcessing" => {
                    #[allow(non_camel_case_types)]
                    struct RestrictProcessingSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::RestrictProcessingRequest>
                    for RestrictProcessingSvc<T> {
                        type Response = super::RestrictProcessingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestrictProcessingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::restrict_processing(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RestrictProcessingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/GetDataExistenceConfirmation" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataExistenceConfirmationSvc<T: PrivacyService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<
                        super::GetDataExistenceConfirmationRequest,
                    > for GetDataExistenceConfirmationSvc<T> {
                        type Response = super::GetDataExistenceConfirmationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDataExistenceConfirmationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::get_data_existence_confirmation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetDataExistenceConfirmationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/ListPrivacyRequests" => {
                    #[allow(non_camel_case_types)]
                    struct ListPrivacyRequestsSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::ListPrivacyRequestsRequest>
                    for ListPrivacyRequestsSvc<T> {
                        type Response = super::ListPrivacyRequestsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPrivacyRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::list_privacy_requests(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListPrivacyRequestsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/CancelDeletion" => {
                    #[allow(non_camel_case_types)]
                    struct CancelDeletionSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::CancelDeletionRequest>
                    for CancelDeletionSvc<T> {
                        type Response = super::CancelDeletionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelDeletionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::cancel_deletion(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CancelDeletionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/ImmediateDelete" => {
                    #[allow(non_camel_case_types)]
                    struct ImmediateDeleteSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::ImmediateDeleteRequest>
                    for ImmediateDeleteSvc<T> {
                        type Response = super::ImmediateDeleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImmediateDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::immediate_delete(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ImmediateDeleteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.PrivacyService/ListMyPrivacyRequests" => {
                    #[allow(non_camel_case_types)]
                    struct ListMyPrivacyRequestsSvc<T: PrivacyService>(pub Arc<T>);
                    impl<
                        T: PrivacyService,
                    > tonic::server::UnaryService<super::ListMyPrivacyRequestsRequest>
                    for ListMyPrivacyRequestsSvc<T> {
                        type Response = super::ListMyPrivacyRequestsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMyPrivacyRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivacyService>::list_my_privacy_requests(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListMyPrivacyRequestsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for PrivacyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.PrivacyService";
    impl<T> tonic::server::NamedService for PrivacyServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod audit_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** AuditService provides read access to the append-only audit trail.
 All RPCs extract org_id from the JWT — it is never in request messages.
*/
    #[derive(Debug, Clone)]
    pub struct AuditServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuditServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuditServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AuditServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AuditServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** List audit events with optional filtering by event type, actor, and date range.
 Results are ordered by created_at descending (newest first).
 Auth: Requires JWT. Admin only.
*/
        pub async fn list_audit_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuditEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuditEventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.AuditService/ListAuditEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.AuditService", "ListAuditEvents"));
            self.inner.unary(req, path, codec).await
        }
        /** Export the audit trail to S3 in CSV, JSON, or Parquet format.
 Creates a persistent record and starts an async Temporal workflow.
 Auth: Requires JWT. Admin only.
*/
        pub async fn export_audit_trail(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAuditTrailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportAuditTrailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.AuditService/ExportAuditTrail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.AuditService", "ExportAuditTrail"));
            self.inner.unary(req, path, codec).await
        }
        /** List audit export history for the organization.
 Auth: Requires JWT. Admin only.
*/
        pub async fn list_audit_exports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuditExportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuditExportsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.AuditService/ListAuditExports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.AuditService", "ListAuditExports"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod audit_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuditServiceServer.
    #[async_trait]
    pub trait AuditService: std::marker::Send + std::marker::Sync + 'static {
        /** List audit events with optional filtering by event type, actor, and date range.
 Results are ordered by created_at descending (newest first).
 Auth: Requires JWT. Admin only.
*/
        async fn list_audit_events(
            &self,
            request: tonic::Request<super::ListAuditEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuditEventsResponse>,
            tonic::Status,
        >;
        /** Export the audit trail to S3 in CSV, JSON, or Parquet format.
 Creates a persistent record and starts an async Temporal workflow.
 Auth: Requires JWT. Admin only.
*/
        async fn export_audit_trail(
            &self,
            request: tonic::Request<super::ExportAuditTrailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportAuditTrailResponse>,
            tonic::Status,
        >;
        /** List audit export history for the organization.
 Auth: Requires JWT. Admin only.
*/
        async fn list_audit_exports(
            &self,
            request: tonic::Request<super::ListAuditExportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuditExportsResponse>,
            tonic::Status,
        >;
    }
    /** AuditService provides read access to the append-only audit trail.
 All RPCs extract org_id from the JWT — it is never in request messages.
*/
    #[derive(Debug)]
    pub struct AuditServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AuditServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuditServiceServer<T>
    where
        T: AuditService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.AuditService/ListAuditEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListAuditEventsSvc<T: AuditService>(pub Arc<T>);
                    impl<
                        T: AuditService,
                    > tonic::server::UnaryService<super::ListAuditEventsRequest>
                    for ListAuditEventsSvc<T> {
                        type Response = super::ListAuditEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAuditEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuditService>::list_audit_events(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListAuditEventsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.AuditService/ExportAuditTrail" => {
                    #[allow(non_camel_case_types)]
                    struct ExportAuditTrailSvc<T: AuditService>(pub Arc<T>);
                    impl<
                        T: AuditService,
                    > tonic::server::UnaryService<super::ExportAuditTrailRequest>
                    for ExportAuditTrailSvc<T> {
                        type Response = super::ExportAuditTrailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportAuditTrailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuditService>::export_audit_trail(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ExportAuditTrailSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.AuditService/ListAuditExports" => {
                    #[allow(non_camel_case_types)]
                    struct ListAuditExportsSvc<T: AuditService>(pub Arc<T>);
                    impl<
                        T: AuditService,
                    > tonic::server::UnaryService<super::ListAuditExportsRequest>
                    for ListAuditExportsSvc<T> {
                        type Response = super::ListAuditExportsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAuditExportsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuditService>::list_audit_exports(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListAuditExportsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AuditServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.AuditService";
    impl<T> tonic::server::NamedService for AuditServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod campaign_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages the full lifecycle of communication campaigns — creation,
 execution, monitoring, and cancellation.
*/
    #[derive(Debug, Clone)]
    pub struct CampaignServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CampaignServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CampaignServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CampaignServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CampaignServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new campaign with a template, audience, and workflow.
 Authorization: Requires MANAGER+ role.
*/
        pub async fn create_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCampaignResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/CreateCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "CreateCampaign"));
            self.inner.unary(req, path, codec).await
        }
        /** Start a created campaign, triggering its workflow execution via the orchestration engine.
 Authorization: Requires MANAGER+ role.
*/
        pub async fn start_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::StartCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartCampaignResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/StartCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "StartCampaign"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve a single campaign by ID.
 Authorization: Authenticated user within the organization.
*/
        pub async fn get_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCampaignResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/GetCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "GetCampaign"));
            self.inner.unary(req, path, codec).await
        }
        /** List campaigns for the organization with pagination.
 Authorization: Authenticated user within the organization.
*/
        pub async fn list_campaigns(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCampaignsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCampaignsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/ListCampaigns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "ListCampaigns"));
            self.inner.unary(req, path, codec).await
        }
        /** Update a draft campaign (CREATED status only). Non-empty fields overwrite existing values.
 Authorization: Requires MANAGER+ role.
*/
        pub async fn update_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCampaignResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/UpdateCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "UpdateCampaign"));
            self.inner.unary(req, path, codec).await
        }
        /** Cancel a running campaign, stopping further deliveries and reminders.
 Authorization: Requires MANAGER+ role.
*/
        pub async fn cancel_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelCampaignResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/CancelCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "CancelCampaign"));
            self.inner.unary(req, path, codec).await
        }
        /** List delivery records for a campaign, optionally filtered by status.
 Authorization: Authenticated user within the organization.
*/
        pub async fn list_deliveries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeliveriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveriesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.CampaignService/ListDeliveries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.CampaignService", "ListDeliveries"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod campaign_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CampaignServiceServer.
    #[async_trait]
    pub trait CampaignService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new campaign with a template, audience, and workflow.
 Authorization: Requires MANAGER+ role.
*/
        async fn create_campaign(
            &self,
            request: tonic::Request<super::CreateCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCampaignResponse>,
            tonic::Status,
        >;
        /** Start a created campaign, triggering its workflow execution via the orchestration engine.
 Authorization: Requires MANAGER+ role.
*/
        async fn start_campaign(
            &self,
            request: tonic::Request<super::StartCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartCampaignResponse>,
            tonic::Status,
        >;
        /** Retrieve a single campaign by ID.
 Authorization: Authenticated user within the organization.
*/
        async fn get_campaign(
            &self,
            request: tonic::Request<super::GetCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCampaignResponse>,
            tonic::Status,
        >;
        /** List campaigns for the organization with pagination.
 Authorization: Authenticated user within the organization.
*/
        async fn list_campaigns(
            &self,
            request: tonic::Request<super::ListCampaignsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCampaignsResponse>,
            tonic::Status,
        >;
        /** Update a draft campaign (CREATED status only). Non-empty fields overwrite existing values.
 Authorization: Requires MANAGER+ role.
*/
        async fn update_campaign(
            &self,
            request: tonic::Request<super::UpdateCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCampaignResponse>,
            tonic::Status,
        >;
        /** Cancel a running campaign, stopping further deliveries and reminders.
 Authorization: Requires MANAGER+ role.
*/
        async fn cancel_campaign(
            &self,
            request: tonic::Request<super::CancelCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelCampaignResponse>,
            tonic::Status,
        >;
        /** List delivery records for a campaign, optionally filtered by status.
 Authorization: Authenticated user within the organization.
*/
        async fn list_deliveries(
            &self,
            request: tonic::Request<super::ListDeliveriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveriesResponse>,
            tonic::Status,
        >;
    }
    /** Manages the full lifecycle of communication campaigns — creation,
 execution, monitoring, and cancellation.
*/
    #[derive(Debug)]
    pub struct CampaignServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CampaignServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CampaignServiceServer<T>
    where
        T: CampaignService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.CampaignService/CreateCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCampaignSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::CreateCampaignRequest>
                    for CreateCampaignSvc<T> {
                        type Response = super::CreateCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::create_campaign(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateCampaignSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.CampaignService/StartCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct StartCampaignSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::StartCampaignRequest>
                    for StartCampaignSvc<T> {
                        type Response = super::StartCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::start_campaign(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartCampaignSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.CampaignService/GetCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct GetCampaignSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::GetCampaignRequest>
                    for GetCampaignSvc<T> {
                        type Response = super::GetCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::get_campaign(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCampaignSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.CampaignService/ListCampaigns" => {
                    #[allow(non_camel_case_types)]
                    struct ListCampaignsSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::ListCampaignsRequest>
                    for ListCampaignsSvc<T> {
                        type Response = super::ListCampaignsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCampaignsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::list_campaigns(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListCampaignsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.CampaignService/UpdateCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCampaignSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::UpdateCampaignRequest>
                    for UpdateCampaignSvc<T> {
                        type Response = super::UpdateCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::update_campaign(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateCampaignSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.CampaignService/CancelCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct CancelCampaignSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::CancelCampaignRequest>
                    for CancelCampaignSvc<T> {
                        type Response = super::CancelCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::cancel_campaign(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CancelCampaignSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.CampaignService/ListDeliveries" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeliveriesSvc<T: CampaignService>(pub Arc<T>);
                    impl<
                        T: CampaignService,
                    > tonic::server::UnaryService<super::ListDeliveriesRequest>
                    for ListDeliveriesSvc<T> {
                        type Response = super::ListDeliveriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDeliveriesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CampaignService>::list_deliveries(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListDeliveriesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CampaignServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.CampaignService";
    impl<T> tonic::server::NamedService for CampaignServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod device_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages push notification device registration.
 Used by the mobile app to register push tokens and manage device lifecycle.
*/
    #[derive(Debug, Clone)]
    pub struct DeviceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeviceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeviceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            DeviceServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Register a device with its push token for receiving notifications.
 Authorization: Authenticated user (own devices only).
*/
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.DeviceService/Register",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.DeviceService", "Register"));
            self.inner.unary(req, path, codec).await
        }
        /** Deactivate a device, preventing further push notifications.
 Authorization: Authenticated user (own devices only).
*/
        pub async fn deactivate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.DeviceService/Deactivate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.DeviceService", "Deactivate"));
            self.inner.unary(req, path, codec).await
        }
        /** List all devices registered to the authenticated user.
 Authorization: Authenticated user (own devices only).
*/
        pub async fn list_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDevicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDevicesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.DeviceService/ListDevices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.DeviceService", "ListDevices"));
            self.inner.unary(req, path, codec).await
        }
        /** List all devices for a specific organization member.
 Authorization: Requires MEMBERS_READ permission.
*/
        pub async fn list_member_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMemberDevicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMemberDevicesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.DeviceService/ListMemberDevices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.DeviceService", "ListMemberDevices"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod device_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DeviceServiceServer.
    #[async_trait]
    pub trait DeviceService: std::marker::Send + std::marker::Sync + 'static {
        /** Register a device with its push token for receiving notifications.
 Authorization: Authenticated user (own devices only).
*/
        async fn register(
            &self,
            request: tonic::Request<super::RegisterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterResponse>,
            tonic::Status,
        >;
        /** Deactivate a device, preventing further push notifications.
 Authorization: Authenticated user (own devices only).
*/
        async fn deactivate(
            &self,
            request: tonic::Request<super::DeactivateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateResponse>,
            tonic::Status,
        >;
        /** List all devices registered to the authenticated user.
 Authorization: Authenticated user (own devices only).
*/
        async fn list_devices(
            &self,
            request: tonic::Request<super::ListDevicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDevicesResponse>,
            tonic::Status,
        >;
        /** List all devices for a specific organization member.
 Authorization: Requires MEMBERS_READ permission.
*/
        async fn list_member_devices(
            &self,
            request: tonic::Request<super::ListMemberDevicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMemberDevicesResponse>,
            tonic::Status,
        >;
    }
    /** Manages push notification device registration.
 Used by the mobile app to register push tokens and manage device lifecycle.
*/
    #[derive(Debug)]
    pub struct DeviceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> DeviceServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DeviceServiceServer<T>
    where
        T: DeviceService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.DeviceService/Register" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::RegisterRequest>
                    for RegisterSvc<T> {
                        type Response = super::RegisterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::register(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RegisterSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.DeviceService/Deactivate" => {
                    #[allow(non_camel_case_types)]
                    struct DeactivateSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeactivateRequest>
                    for DeactivateSvc<T> {
                        type Response = super::DeactivateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeactivateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::deactivate(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeactivateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.DeviceService/ListDevices" => {
                    #[allow(non_camel_case_types)]
                    struct ListDevicesSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::ListDevicesRequest>
                    for ListDevicesSvc<T> {
                        type Response = super::ListDevicesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDevicesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::list_devices(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListDevicesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.DeviceService/ListMemberDevices" => {
                    #[allow(non_camel_case_types)]
                    struct ListMemberDevicesSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::ListMemberDevicesRequest>
                    for ListMemberDevicesSvc<T> {
                        type Response = super::ListMemberDevicesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMemberDevicesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::list_member_devices(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListMemberDevicesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for DeviceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.DeviceService";
    impl<T> tonic::server::NamedService for DeviceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod group_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages groups and group membership within an organization.
 Groups are recipient collections used for campaign audience targeting.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug, Clone)]
    pub struct GroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GroupServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GroupServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GroupServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            GroupServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new group in the organization.
 Authorization: Requires PERMISSION_GROUPS_WRITE or PERMISSION_GROUPS_ALL_WRITE.
*/
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateGroupResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/CreateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "CreateGroup"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve a group by ID.
 Authorization: Caller must be a member of the group, or have
 PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE.
*/
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGroupResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/GetGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "GetGroup"));
            self.inner.unary(req, path, codec).await
        }
        /** List groups in the organization with pagination.
 Without PERMISSION_GROUPS_ALL_READ/ALL_WRITE, returns only groups the caller belongs to.
*/
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/ListGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "ListGroups"));
            self.inner.unary(req, path, codec).await
        }
        /** Update a group's name and/or description.
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGroupResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/UpdateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "UpdateGroup"));
            self.inner.unary(req, path, codec).await
        }
        /** Delete a group and all its membership records. Default groups cannot be deleted.
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        pub async fn delete_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteGroupResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/DeleteGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "DeleteGroup"));
            self.inner.unary(req, path, codec).await
        }
        /** Add one or more users to a group (idempotent).
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        pub async fn add_group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGroupMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/AddGroupMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "AddGroupMembers"));
            self.inner.unary(req, path, codec).await
        }
        /** Remove one or more users from a group (idempotent).
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        pub async fn remove_group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveGroupMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/RemoveGroupMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "RemoveGroupMembers"));
            self.inner.unary(req, path, codec).await
        }
        /** List members of a group with pagination.
 Authorization: Caller must be a member of the group, or have
 PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE.
*/
        pub async fn list_group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/ListGroupMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.GroupService", "ListGroupMembers"));
            self.inner.unary(req, path, codec).await
        }
        /** Get group memberships for a batch of users.
 Used by campaign audience to show group badges.
 Authorization: Requires PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE.
*/
        pub async fn get_user_group_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserGroupMembershipsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGroupMembershipsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.GroupService/GetUserGroupMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.GroupService", "GetUserGroupMemberships"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod group_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GroupServiceServer.
    #[async_trait]
    pub trait GroupService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new group in the organization.
 Authorization: Requires PERMISSION_GROUPS_WRITE or PERMISSION_GROUPS_ALL_WRITE.
*/
        async fn create_group(
            &self,
            request: tonic::Request<super::CreateGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateGroupResponse>,
            tonic::Status,
        >;
        /** Retrieve a group by ID.
 Authorization: Caller must be a member of the group, or have
 PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE.
*/
        async fn get_group(
            &self,
            request: tonic::Request<super::GetGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGroupResponse>,
            tonic::Status,
        >;
        /** List groups in the organization with pagination.
 Without PERMISSION_GROUPS_ALL_READ/ALL_WRITE, returns only groups the caller belongs to.
*/
        async fn list_groups(
            &self,
            request: tonic::Request<super::ListGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupsResponse>,
            tonic::Status,
        >;
        /** Update a group's name and/or description.
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        async fn update_group(
            &self,
            request: tonic::Request<super::UpdateGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGroupResponse>,
            tonic::Status,
        >;
        /** Delete a group and all its membership records. Default groups cannot be deleted.
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        async fn delete_group(
            &self,
            request: tonic::Request<super::DeleteGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteGroupResponse>,
            tonic::Status,
        >;
        /** Add one or more users to a group (idempotent).
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        async fn add_group_members(
            &self,
            request: tonic::Request<super::AddGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGroupMembersResponse>,
            tonic::Status,
        >;
        /** Remove one or more users from a group (idempotent).
 Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any).
*/
        async fn remove_group_members(
            &self,
            request: tonic::Request<super::RemoveGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveGroupMembersResponse>,
            tonic::Status,
        >;
        /** List members of a group with pagination.
 Authorization: Caller must be a member of the group, or have
 PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE.
*/
        async fn list_group_members(
            &self,
            request: tonic::Request<super::ListGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupMembersResponse>,
            tonic::Status,
        >;
        /** Get group memberships for a batch of users.
 Used by campaign audience to show group badges.
 Authorization: Requires PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE.
*/
        async fn get_user_group_memberships(
            &self,
            request: tonic::Request<super::GetUserGroupMembershipsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGroupMembershipsResponse>,
            tonic::Status,
        >;
    }
    /** Manages groups and group membership within an organization.
 Groups are recipient collections used for campaign audience targeting.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug)]
    pub struct GroupServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> GroupServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GroupServiceServer<T>
    where
        T: GroupService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.GroupService/CreateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::CreateGroupRequest>
                    for CreateGroupSvc<T> {
                        type Response = super::CreateGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::create_group(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateGroupSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/GetGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GetGroupRequest>
                    for GetGroupSvc<T> {
                        type Response = super::GetGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::get_group(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetGroupSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/ListGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupsSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::ListGroupsRequest>
                    for ListGroupsSvc<T> {
                        type Response = super::ListGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::list_groups(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListGroupsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/UpdateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::UpdateGroupRequest>
                    for UpdateGroupSvc<T> {
                        type Response = super::UpdateGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::update_group(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateGroupSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/DeleteGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::DeleteGroupRequest>
                    for DeleteGroupSvc<T> {
                        type Response = super::DeleteGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::delete_group(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteGroupSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/AddGroupMembers" => {
                    #[allow(non_camel_case_types)]
                    struct AddGroupMembersSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::AddGroupMembersRequest>
                    for AddGroupMembersSvc<T> {
                        type Response = super::AddGroupMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddGroupMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::add_group_members(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddGroupMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/RemoveGroupMembers" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGroupMembersSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::RemoveGroupMembersRequest>
                    for RemoveGroupMembersSvc<T> {
                        type Response = super::RemoveGroupMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveGroupMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::remove_group_members(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveGroupMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/ListGroupMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupMembersSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::ListGroupMembersRequest>
                    for ListGroupMembersSvc<T> {
                        type Response = super::ListGroupMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGroupMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::list_group_members(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListGroupMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.GroupService/GetUserGroupMemberships" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserGroupMembershipsSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GetUserGroupMembershipsRequest>
                    for GetUserGroupMembershipsSvc<T> {
                        type Response = super::GetUserGroupMembershipsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUserGroupMembershipsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GroupService>::get_user_group_memberships(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetUserGroupMembershipsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for GroupServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.GroupService";
    impl<T> tonic::server::NamedService for GroupServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod heatmap_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages touch event ingestion, heatmap data aggregation, and screen screenshots
 for mobile app interaction analytics.
*/
    #[derive(Debug, Clone)]
    pub struct HeatmapServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HeatmapServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> HeatmapServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> HeatmapServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            HeatmapServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Ingest a batch of touch events from the mobile app.
 Authorization: Authenticated mobile user.
*/
        pub async fn ingest_touch_events(
            &mut self,
            request: impl tonic::IntoRequest<super::IngestTouchEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IngestTouchEventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.HeatmapService/IngestTouchEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.HeatmapService", "IngestTouchEvents"));
            self.inner.unary(req, path, codec).await
        }
        /** Query aggregated touch data for heatmap rendering.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        pub async fn query_heatmap_data(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryHeatmapDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryHeatmapDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.HeatmapService/QueryHeatmapData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.HeatmapService", "QueryHeatmapData"));
            self.inner.unary(req, path, codec).await
        }
        /** List available screen screenshots for heatmap backgrounds.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        pub async fn list_screenshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScreenshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListScreenshotsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.HeatmapService/ListScreenshots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.HeatmapService", "ListScreenshots"));
            self.inner.unary(req, path, codec).await
        }
        /** Upload a screenshot captured from the mobile app for heatmap backdrops.
 Authorization: Authenticated mobile user.
*/
        pub async fn upload_screenshot(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadScreenshotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UploadScreenshotResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.HeatmapService/UploadScreenshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.HeatmapService", "UploadScreenshot"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod heatmap_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with HeatmapServiceServer.
    #[async_trait]
    pub trait HeatmapService: std::marker::Send + std::marker::Sync + 'static {
        /** Ingest a batch of touch events from the mobile app.
 Authorization: Authenticated mobile user.
*/
        async fn ingest_touch_events(
            &self,
            request: tonic::Request<super::IngestTouchEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IngestTouchEventsResponse>,
            tonic::Status,
        >;
        /** Query aggregated touch data for heatmap rendering.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        async fn query_heatmap_data(
            &self,
            request: tonic::Request<super::QueryHeatmapDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryHeatmapDataResponse>,
            tonic::Status,
        >;
        /** List available screen screenshots for heatmap backgrounds.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        async fn list_screenshots(
            &self,
            request: tonic::Request<super::ListScreenshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListScreenshotsResponse>,
            tonic::Status,
        >;
        /** Upload a screenshot captured from the mobile app for heatmap backdrops.
 Authorization: Authenticated mobile user.
*/
        async fn upload_screenshot(
            &self,
            request: tonic::Request<super::UploadScreenshotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UploadScreenshotResponse>,
            tonic::Status,
        >;
    }
    /** Manages touch event ingestion, heatmap data aggregation, and screen screenshots
 for mobile app interaction analytics.
*/
    #[derive(Debug)]
    pub struct HeatmapServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> HeatmapServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for HeatmapServiceServer<T>
    where
        T: HeatmapService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.HeatmapService/IngestTouchEvents" => {
                    #[allow(non_camel_case_types)]
                    struct IngestTouchEventsSvc<T: HeatmapService>(pub Arc<T>);
                    impl<
                        T: HeatmapService,
                    > tonic::server::UnaryService<super::IngestTouchEventsRequest>
                    for IngestTouchEventsSvc<T> {
                        type Response = super::IngestTouchEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IngestTouchEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as HeatmapService>::ingest_touch_events(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = IngestTouchEventsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.HeatmapService/QueryHeatmapData" => {
                    #[allow(non_camel_case_types)]
                    struct QueryHeatmapDataSvc<T: HeatmapService>(pub Arc<T>);
                    impl<
                        T: HeatmapService,
                    > tonic::server::UnaryService<super::QueryHeatmapDataRequest>
                    for QueryHeatmapDataSvc<T> {
                        type Response = super::QueryHeatmapDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryHeatmapDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as HeatmapService>::query_heatmap_data(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = QueryHeatmapDataSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.HeatmapService/ListScreenshots" => {
                    #[allow(non_camel_case_types)]
                    struct ListScreenshotsSvc<T: HeatmapService>(pub Arc<T>);
                    impl<
                        T: HeatmapService,
                    > tonic::server::UnaryService<super::ListScreenshotsRequest>
                    for ListScreenshotsSvc<T> {
                        type Response = super::ListScreenshotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListScreenshotsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as HeatmapService>::list_screenshots(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListScreenshotsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.HeatmapService/UploadScreenshot" => {
                    #[allow(non_camel_case_types)]
                    struct UploadScreenshotSvc<T: HeatmapService>(pub Arc<T>);
                    impl<
                        T: HeatmapService,
                    > tonic::server::UnaryService<super::UploadScreenshotRequest>
                    for UploadScreenshotSvc<T> {
                        type Response = super::UploadScreenshotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadScreenshotRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as HeatmapService>::upload_screenshot(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UploadScreenshotSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for HeatmapServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.HeatmapService";
    impl<T> tonic::server::NamedService for HeatmapServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod inbox_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Provides the mobile app's inbox experience — syncing messages,
 tracking read status, and retrieving individual entries.
*/
    #[derive(Debug, Clone)]
    pub struct InboxServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InboxServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InboxServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InboxServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            InboxServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Sync inbox entries since a given timestamp for incremental updates.
 Authorization: Authenticated user (own inbox only).
*/
        pub async fn sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncRequest>,
        ) -> std::result::Result<tonic::Response<super::SyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InboxService/Sync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.InboxService", "Sync"));
            self.inner.unary(req, path, codec).await
        }
        /** Mark a delivered message as read (analytics-only, does not affect workflow).
 Authorization: Authenticated user (own inbox only).
*/
        pub async fn mark_read(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkReadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InboxService/MarkRead",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.InboxService", "MarkRead"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve a single inbox entry by delivery ID.
 Authorization: Authenticated user (own inbox only).
*/
        pub async fn get_message(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InboxService/GetMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.InboxService", "GetMessage"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod inbox_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InboxServiceServer.
    #[async_trait]
    pub trait InboxService: std::marker::Send + std::marker::Sync + 'static {
        /** Sync inbox entries since a given timestamp for incremental updates.
 Authorization: Authenticated user (own inbox only).
*/
        async fn sync(
            &self,
            request: tonic::Request<super::SyncRequest>,
        ) -> std::result::Result<tonic::Response<super::SyncResponse>, tonic::Status>;
        /** Mark a delivered message as read (analytics-only, does not affect workflow).
 Authorization: Authenticated user (own inbox only).
*/
        async fn mark_read(
            &self,
            request: tonic::Request<super::MarkReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkReadResponse>,
            tonic::Status,
        >;
        /** Retrieve a single inbox entry by delivery ID.
 Authorization: Authenticated user (own inbox only).
*/
        async fn get_message(
            &self,
            request: tonic::Request<super::GetMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMessageResponse>,
            tonic::Status,
        >;
    }
    /** Provides the mobile app's inbox experience — syncing messages,
 tracking read status, and retrieving individual entries.
*/
    #[derive(Debug)]
    pub struct InboxServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> InboxServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InboxServiceServer<T>
    where
        T: InboxService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.InboxService/Sync" => {
                    #[allow(non_camel_case_types)]
                    struct SyncSvc<T: InboxService>(pub Arc<T>);
                    impl<T: InboxService> tonic::server::UnaryService<super::SyncRequest>
                    for SyncSvc<T> {
                        type Response = super::SyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyncRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InboxService>::sync(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InboxService/MarkRead" => {
                    #[allow(non_camel_case_types)]
                    struct MarkReadSvc<T: InboxService>(pub Arc<T>);
                    impl<
                        T: InboxService,
                    > tonic::server::UnaryService<super::MarkReadRequest>
                    for MarkReadSvc<T> {
                        type Response = super::MarkReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InboxService>::mark_read(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MarkReadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InboxService/GetMessage" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessageSvc<T: InboxService>(pub Arc<T>);
                    impl<
                        T: InboxService,
                    > tonic::server::UnaryService<super::GetMessageRequest>
                    for GetMessageSvc<T> {
                        type Response = super::GetMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InboxService>::get_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetMessageSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for InboxServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.InboxService";
    impl<T> tonic::server::NamedService for InboxServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod insights_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Provides cohort-level AI insights for campaign planning and optimization.
 All predictions are aggregate — no individual-level profiling.
 Data is derived from k-anonymized, differential-privacy-noised behavioral features.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug, Clone)]
    pub struct InsightsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InsightsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InsightsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InsightsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            InsightsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Retrieve behavioral archetypes for a group based on anonymous feature vectors.
 Returns empty archetypes if insufficient data (cold start).
 Authorization: Requires PERMISSION_CAMPAIGNS_READ.
*/
        pub async fn get_group_archetypes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupArchetypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGroupArchetypesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InsightsService/GetGroupArchetypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InsightsService", "GetGroupArchetypes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Predict cohort-level ACK rate for a campaign targeting a specific group.
 Returns a confidence interval that narrows as more campaign data accumulates.
 Authorization: Requires PERMISSION_CAMPAIGNS_READ.
*/
        pub async fn predict_campaign_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictCampaignAckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PredictCampaignAckResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InsightsService/PredictCampaignACK",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InsightsService", "PredictCampaignACK"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get campaign configuration advisory (prediction + suggested escalation + archetypes).
 Advisory is informational only — never drives automated decisions.
 Authorization: Requires PERMISSION_CAMPAIGNS_READ.
*/
        pub async fn get_campaign_advisory(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignAdvisoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCampaignAdvisoryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InsightsService/GetCampaignAdvisory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InsightsService", "GetCampaignAdvisory"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod insights_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InsightsServiceServer.
    #[async_trait]
    pub trait InsightsService: std::marker::Send + std::marker::Sync + 'static {
        /** Retrieve behavioral archetypes for a group based on anonymous feature vectors.
 Returns empty archetypes if insufficient data (cold start).
 Authorization: Requires PERMISSION_CAMPAIGNS_READ.
*/
        async fn get_group_archetypes(
            &self,
            request: tonic::Request<super::GetGroupArchetypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGroupArchetypesResponse>,
            tonic::Status,
        >;
        /** Predict cohort-level ACK rate for a campaign targeting a specific group.
 Returns a confidence interval that narrows as more campaign data accumulates.
 Authorization: Requires PERMISSION_CAMPAIGNS_READ.
*/
        async fn predict_campaign_ack(
            &self,
            request: tonic::Request<super::PredictCampaignAckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PredictCampaignAckResponse>,
            tonic::Status,
        >;
        /** Get campaign configuration advisory (prediction + suggested escalation + archetypes).
 Advisory is informational only — never drives automated decisions.
 Authorization: Requires PERMISSION_CAMPAIGNS_READ.
*/
        async fn get_campaign_advisory(
            &self,
            request: tonic::Request<super::GetCampaignAdvisoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCampaignAdvisoryResponse>,
            tonic::Status,
        >;
    }
    /** Provides cohort-level AI insights for campaign planning and optimization.
 All predictions are aggregate — no individual-level profiling.
 Data is derived from k-anonymized, differential-privacy-noised behavioral features.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug)]
    pub struct InsightsServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> InsightsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InsightsServiceServer<T>
    where
        T: InsightsService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.InsightsService/GetGroupArchetypes" => {
                    #[allow(non_camel_case_types)]
                    struct GetGroupArchetypesSvc<T: InsightsService>(pub Arc<T>);
                    impl<
                        T: InsightsService,
                    > tonic::server::UnaryService<super::GetGroupArchetypesRequest>
                    for GetGroupArchetypesSvc<T> {
                        type Response = super::GetGroupArchetypesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGroupArchetypesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InsightsService>::get_group_archetypes(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetGroupArchetypesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InsightsService/PredictCampaignACK" => {
                    #[allow(non_camel_case_types)]
                    struct PredictCampaignACKSvc<T: InsightsService>(pub Arc<T>);
                    impl<
                        T: InsightsService,
                    > tonic::server::UnaryService<super::PredictCampaignAckRequest>
                    for PredictCampaignACKSvc<T> {
                        type Response = super::PredictCampaignAckResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PredictCampaignAckRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InsightsService>::predict_campaign_ack(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PredictCampaignACKSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InsightsService/GetCampaignAdvisory" => {
                    #[allow(non_camel_case_types)]
                    struct GetCampaignAdvisorySvc<T: InsightsService>(pub Arc<T>);
                    impl<
                        T: InsightsService,
                    > tonic::server::UnaryService<super::GetCampaignAdvisoryRequest>
                    for GetCampaignAdvisorySvc<T> {
                        type Response = super::GetCampaignAdvisoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCampaignAdvisoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InsightsService>::get_campaign_advisory(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCampaignAdvisorySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for InsightsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.InsightsService";
    impl<T> tonic::server::NamedService for InsightsServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod invite_link_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages shareable invite links for organization self-join.
 Create, List, and Revoke require JWT + PERMISSION_MEMBERS_INVITE.
 ValidateInviteLink is unauthenticated — the token IS the authorization.
 RedeemInviteLink requires a valid JWT.
*/
    #[derive(Debug, Clone)]
    pub struct InviteLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InviteLinkServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InviteLinkServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InviteLinkServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            InviteLinkServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new shareable invite link for the organization.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        pub async fn create_invite_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInviteLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InviteLinkService/CreateInviteLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InviteLinkService", "CreateInviteLink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List all invite links for the organization.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        pub async fn list_invite_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInviteLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInviteLinksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InviteLinkService/ListInviteLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InviteLinkService", "ListInviteLinks"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Revoke an invite link, making it immediately unusable. Idempotent.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        pub async fn revoke_invite_link(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeInviteLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InviteLinkService/RevokeInviteLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InviteLinkService", "RevokeInviteLink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Validate an invite link and provision a user account if needed.
 This is a pre-authentication endpoint — no JWT required.
 Rate limited to 10 requests per minute per IP.
 Authorization: None (token-based).
*/
        pub async fn validate_invite_link(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateInviteLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InviteLinkService/ValidateInviteLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InviteLinkService", "ValidateInviteLink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Redeem an invite link to join an organization.
 Requires a valid JWT — the email is extracted from the token.
 Authorization: JWT required.
*/
        pub async fn redeem_invite_link(
            &mut self,
            request: impl tonic::IntoRequest<super::RedeemInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RedeemInviteLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.InviteLinkService/RedeemInviteLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.InviteLinkService", "RedeemInviteLink"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod invite_link_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InviteLinkServiceServer.
    #[async_trait]
    pub trait InviteLinkService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new shareable invite link for the organization.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        async fn create_invite_link(
            &self,
            request: tonic::Request<super::CreateInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInviteLinkResponse>,
            tonic::Status,
        >;
        /** List all invite links for the organization.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        async fn list_invite_links(
            &self,
            request: tonic::Request<super::ListInviteLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInviteLinksResponse>,
            tonic::Status,
        >;
        /** Revoke an invite link, making it immediately unusable. Idempotent.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        async fn revoke_invite_link(
            &self,
            request: tonic::Request<super::RevokeInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeInviteLinkResponse>,
            tonic::Status,
        >;
        /** Validate an invite link and provision a user account if needed.
 This is a pre-authentication endpoint — no JWT required.
 Rate limited to 10 requests per minute per IP.
 Authorization: None (token-based).
*/
        async fn validate_invite_link(
            &self,
            request: tonic::Request<super::ValidateInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateInviteLinkResponse>,
            tonic::Status,
        >;
        /** Redeem an invite link to join an organization.
 Requires a valid JWT — the email is extracted from the token.
 Authorization: JWT required.
*/
        async fn redeem_invite_link(
            &self,
            request: tonic::Request<super::RedeemInviteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RedeemInviteLinkResponse>,
            tonic::Status,
        >;
    }
    /** Manages shareable invite links for organization self-join.
 Create, List, and Revoke require JWT + PERMISSION_MEMBERS_INVITE.
 ValidateInviteLink is unauthenticated — the token IS the authorization.
 RedeemInviteLink requires a valid JWT.
*/
    #[derive(Debug)]
    pub struct InviteLinkServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> InviteLinkServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InviteLinkServiceServer<T>
    where
        T: InviteLinkService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.InviteLinkService/CreateInviteLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInviteLinkSvc<T: InviteLinkService>(pub Arc<T>);
                    impl<
                        T: InviteLinkService,
                    > tonic::server::UnaryService<super::CreateInviteLinkRequest>
                    for CreateInviteLinkSvc<T> {
                        type Response = super::CreateInviteLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInviteLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InviteLinkService>::create_invite_link(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateInviteLinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InviteLinkService/ListInviteLinks" => {
                    #[allow(non_camel_case_types)]
                    struct ListInviteLinksSvc<T: InviteLinkService>(pub Arc<T>);
                    impl<
                        T: InviteLinkService,
                    > tonic::server::UnaryService<super::ListInviteLinksRequest>
                    for ListInviteLinksSvc<T> {
                        type Response = super::ListInviteLinksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInviteLinksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InviteLinkService>::list_invite_links(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListInviteLinksSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InviteLinkService/RevokeInviteLink" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeInviteLinkSvc<T: InviteLinkService>(pub Arc<T>);
                    impl<
                        T: InviteLinkService,
                    > tonic::server::UnaryService<super::RevokeInviteLinkRequest>
                    for RevokeInviteLinkSvc<T> {
                        type Response = super::RevokeInviteLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeInviteLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InviteLinkService>::revoke_invite_link(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RevokeInviteLinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InviteLinkService/ValidateInviteLink" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateInviteLinkSvc<T: InviteLinkService>(pub Arc<T>);
                    impl<
                        T: InviteLinkService,
                    > tonic::server::UnaryService<super::ValidateInviteLinkRequest>
                    for ValidateInviteLinkSvc<T> {
                        type Response = super::ValidateInviteLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidateInviteLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InviteLinkService>::validate_invite_link(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ValidateInviteLinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.InviteLinkService/RedeemInviteLink" => {
                    #[allow(non_camel_case_types)]
                    struct RedeemInviteLinkSvc<T: InviteLinkService>(pub Arc<T>);
                    impl<
                        T: InviteLinkService,
                    > tonic::server::UnaryService<super::RedeemInviteLinkRequest>
                    for RedeemInviteLinkSvc<T> {
                        type Response = super::RedeemInviteLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RedeemInviteLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InviteLinkService>::redeem_invite_link(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RedeemInviteLinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for InviteLinkServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.InviteLinkService";
    impl<T> tonic::server::NamedService for InviteLinkServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod member_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages members (users) within an organization.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug, Clone)]
    pub struct MemberServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MemberServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MemberServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MemberServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            MemberServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Invite a new user to the organization via email.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        pub async fn invite_user(
            &mut self,
            request: impl tonic::IntoRequest<super::InviteUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InviteUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/InviteUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "InviteUser"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve a user by ID within the organization.
 Self-lookup (empty user_id) is allowed for any authenticated user.
 Authorization: Requires PERMISSION_MEMBERS_READ for other users.
*/
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/GetUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "GetUser"));
            self.inner.unary(req, path, codec).await
        }
        /** List all users in the organization with pagination.
 Authorization: Requires PERMISSION_MEMBERS_READ.
*/
        pub async fn list_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "ListUsers"));
            self.inner.unary(req, path, codec).await
        }
        /** Change a user's role within the organization.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn update_user_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserRoleResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/UpdateUserRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "UpdateUserRole"));
            self.inner.unary(req, path, codec).await
        }
        /** Deactivate a user within the organization.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn deactivate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/DeactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "DeactivateUser"));
            self.inner.unary(req, path, codec).await
        }
        /** Reactivate a deactivated user, restoring their status to INVITED.
 The user must complete the invite link flow again to become ACTIVE.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn reactivate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/ReactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "ReactivateUser"));
            self.inner.unary(req, path, codec).await
        }
        /** Revoke an invitation for a user who has not yet completed registration.
 Hard-deletes the user record (INVITED users have no data to preserve).
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn revoke_invite(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeInviteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/RevokeInvite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "RevokeInvite"));
            self.inner.unary(req, path, codec).await
        }
        /** Update a user's profile attributes (department, title, etc.).
 Self-update (empty user_id or matching JWT sub) requires no special permission.
 Updating another user requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn update_user_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserProfileResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/UpdateUserProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "UpdateUserProfile"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve the caller's platform settings (theme, etc.).
 Authorization: Any authenticated user (self-only).
*/
        pub async fn get_user_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/GetUserSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "GetUserSettings"));
            self.inner.unary(req, path, codec).await
        }
        /** Update the caller's platform settings.
 Only fields with non-default values are applied; others are left unchanged.
 Authorization: Any authenticated user (self-only).
*/
        pub async fn update_user_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/UpdateUserSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "UpdateUserSettings"));
            self.inner.unary(req, path, codec).await
        }
        /** Invite multiple users to the organization in a single call.
 Emails are deduplicated. Each email is processed independently — individual
 failures do not abort the batch. Identity provider calls are parallelized (bounded concurrency).
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        pub async fn bulk_invite_users(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkInviteUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkInviteUsersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/BulkInviteUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "BulkInviteUsers"));
            self.inner.unary(req, path, codec).await
        }
        /** Confirm passkey enrollment after client-side WebAuthn registration.
 Verifies the caller has at least one registered credential server-side,
 then marks the user as passkey-enrolled in the identity provider.
 Authorization: Any authenticated user (self-only, no permission required).
*/
        pub async fn confirm_passkey_enrollment(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmPasskeyEnrollmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmPasskeyEnrollmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/ConfirmPasskeyEnrollment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.MemberService", "ConfirmPasskeyEnrollment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update the data governance region for a user. Triggers a data migration
 workflow if the region changed. Admin-only operation.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn update_user_region(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserRegionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserRegionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.MemberService/UpdateUserRegion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.MemberService", "UpdateUserRegion"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod member_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MemberServiceServer.
    #[async_trait]
    pub trait MemberService: std::marker::Send + std::marker::Sync + 'static {
        /** Invite a new user to the organization via email.
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        async fn invite_user(
            &self,
            request: tonic::Request<super::InviteUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InviteUserResponse>,
            tonic::Status,
        >;
        /** Retrieve a user by ID within the organization.
 Self-lookup (empty user_id) is allowed for any authenticated user.
 Authorization: Requires PERMISSION_MEMBERS_READ for other users.
*/
        async fn get_user(
            &self,
            request: tonic::Request<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::GetUserResponse>, tonic::Status>;
        /** List all users in the organization with pagination.
 Authorization: Requires PERMISSION_MEMBERS_READ.
*/
        async fn list_users(
            &self,
            request: tonic::Request<super::ListUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsersResponse>,
            tonic::Status,
        >;
        /** Change a user's role within the organization.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn update_user_role(
            &self,
            request: tonic::Request<super::UpdateUserRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserRoleResponse>,
            tonic::Status,
        >;
        /** Deactivate a user within the organization.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn deactivate_user(
            &self,
            request: tonic::Request<super::DeactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserResponse>,
            tonic::Status,
        >;
        /** Reactivate a deactivated user, restoring their status to INVITED.
 The user must complete the invite link flow again to become ACTIVE.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn reactivate_user(
            &self,
            request: tonic::Request<super::ReactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateUserResponse>,
            tonic::Status,
        >;
        /** Revoke an invitation for a user who has not yet completed registration.
 Hard-deletes the user record (INVITED users have no data to preserve).
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn revoke_invite(
            &self,
            request: tonic::Request<super::RevokeInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeInviteResponse>,
            tonic::Status,
        >;
        /** Update a user's profile attributes (department, title, etc.).
 Self-update (empty user_id or matching JWT sub) requires no special permission.
 Updating another user requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn update_user_profile(
            &self,
            request: tonic::Request<super::UpdateUserProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserProfileResponse>,
            tonic::Status,
        >;
        /** Retrieve the caller's platform settings (theme, etc.).
 Authorization: Any authenticated user (self-only).
*/
        async fn get_user_settings(
            &self,
            request: tonic::Request<super::GetUserSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserSettingsResponse>,
            tonic::Status,
        >;
        /** Update the caller's platform settings.
 Only fields with non-default values are applied; others are left unchanged.
 Authorization: Any authenticated user (self-only).
*/
        async fn update_user_settings(
            &self,
            request: tonic::Request<super::UpdateUserSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserSettingsResponse>,
            tonic::Status,
        >;
        /** Invite multiple users to the organization in a single call.
 Emails are deduplicated. Each email is processed independently — individual
 failures do not abort the batch. Identity provider calls are parallelized (bounded concurrency).
 Authorization: Requires PERMISSION_MEMBERS_INVITE.
*/
        async fn bulk_invite_users(
            &self,
            request: tonic::Request<super::BulkInviteUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkInviteUsersResponse>,
            tonic::Status,
        >;
        /** Confirm passkey enrollment after client-side WebAuthn registration.
 Verifies the caller has at least one registered credential server-side,
 then marks the user as passkey-enrolled in the identity provider.
 Authorization: Any authenticated user (self-only, no permission required).
*/
        async fn confirm_passkey_enrollment(
            &self,
            request: tonic::Request<super::ConfirmPasskeyEnrollmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmPasskeyEnrollmentResponse>,
            tonic::Status,
        >;
        /** Update the data governance region for a user. Triggers a data migration
 workflow if the region changed. Admin-only operation.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn update_user_region(
            &self,
            request: tonic::Request<super::UpdateUserRegionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserRegionResponse>,
            tonic::Status,
        >;
    }
    /** Manages members (users) within an organization.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug)]
    pub struct MemberServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> MemberServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MemberServiceServer<T>
    where
        T: MemberService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.MemberService/InviteUser" => {
                    #[allow(non_camel_case_types)]
                    struct InviteUserSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::InviteUserRequest>
                    for InviteUserSvc<T> {
                        type Response = super::InviteUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InviteUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::invite_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = InviteUserSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::GetUserRequest>
                    for GetUserSvc<T> {
                        type Response = super::GetUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::get_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetUserSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/ListUsers" => {
                    #[allow(non_camel_case_types)]
                    struct ListUsersSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::ListUsersRequest>
                    for ListUsersSvc<T> {
                        type Response = super::ListUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUsersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::list_users(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListUsersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/UpdateUserRole" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserRoleSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::UpdateUserRoleRequest>
                    for UpdateUserRoleSvc<T> {
                        type Response = super::UpdateUserRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::update_user_role(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateUserRoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/DeactivateUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeactivateUserSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::DeactivateUserRequest>
                    for DeactivateUserSvc<T> {
                        type Response = super::DeactivateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeactivateUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::deactivate_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeactivateUserSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/ReactivateUser" => {
                    #[allow(non_camel_case_types)]
                    struct ReactivateUserSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::ReactivateUserRequest>
                    for ReactivateUserSvc<T> {
                        type Response = super::ReactivateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReactivateUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::reactivate_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReactivateUserSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/RevokeInvite" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeInviteSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::RevokeInviteRequest>
                    for RevokeInviteSvc<T> {
                        type Response = super::RevokeInviteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeInviteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::revoke_invite(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RevokeInviteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/UpdateUserProfile" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserProfileSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::UpdateUserProfileRequest>
                    for UpdateUserProfileSvc<T> {
                        type Response = super::UpdateUserProfileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserProfileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::update_user_profile(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateUserProfileSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/GetUserSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSettingsSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::GetUserSettingsRequest>
                    for GetUserSettingsSvc<T> {
                        type Response = super::GetUserSettingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserSettingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::get_user_settings(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetUserSettingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/UpdateUserSettings" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserSettingsSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::UpdateUserSettingsRequest>
                    for UpdateUserSettingsSvc<T> {
                        type Response = super::UpdateUserSettingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserSettingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::update_user_settings(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateUserSettingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/BulkInviteUsers" => {
                    #[allow(non_camel_case_types)]
                    struct BulkInviteUsersSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::BulkInviteUsersRequest>
                    for BulkInviteUsersSvc<T> {
                        type Response = super::BulkInviteUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BulkInviteUsersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::bulk_invite_users(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BulkInviteUsersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/ConfirmPasskeyEnrollment" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmPasskeyEnrollmentSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::ConfirmPasskeyEnrollmentRequest>
                    for ConfirmPasskeyEnrollmentSvc<T> {
                        type Response = super::ConfirmPasskeyEnrollmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ConfirmPasskeyEnrollmentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::confirm_passkey_enrollment(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ConfirmPasskeyEnrollmentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.MemberService/UpdateUserRegion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserRegionSvc<T: MemberService>(pub Arc<T>);
                    impl<
                        T: MemberService,
                    > tonic::server::UnaryService<super::UpdateUserRegionRequest>
                    for UpdateUserRegionSvc<T> {
                        type Response = super::UpdateUserRegionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserRegionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MemberService>::update_user_region(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateUserRegionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for MemberServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.MemberService";
    impl<T> tonic::server::NamedService for MemberServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod organization_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages organizations (tenants) in the Pidgr platform.
 Most RPCs operate within the caller's org (extracted from JWT).
 CreateOrganization supports API key auth or JWT auth (self-service onboarding).
*/
    #[derive(Debug, Clone)]
    pub struct OrganizationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrganizationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrganizationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrganizationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            OrganizationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new organization with an initial admin user.
 Supports API key auth (service-to-service) and JWT auth (self-service onboarding).
*/
        pub async fn create_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/CreateOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.OrganizationService", "CreateOrganization"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve the organization for the authenticated user.
 Authorization: Requires PERMISSION_ORG_READ.
*/
        pub async fn get_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/GetOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.OrganizationService", "GetOrganization"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update organization settings (name, default workflow, industry, company size).
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn update_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/UpdateOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.OrganizationService", "UpdateOrganization"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Replace all SSO attribute mappings for the organization.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn update_sso_attribute_mappings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSsoAttributeMappingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSsoAttributeMappingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/UpdateSsoAttributeMappings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.OrganizationService",
                        "UpdateSsoAttributeMappings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Rotate the analytics salt and optionally increase the bucket count for k-anonymization.
 Authorization: Requires PERMISSION_PRIVACY_WRITE.
*/
        pub async fn rotate_analytics_salt(
            &mut self,
            request: impl tonic::IntoRequest<super::RotateAnalyticsSaltRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateAnalyticsSaltResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/RotateAnalyticsSalt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.OrganizationService",
                        "RotateAnalyticsSalt",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update the differential privacy epsilon parameter.
 Authorization: Requires PERMISSION_PRIVACY_WRITE.
*/
        pub async fn update_analytics_epsilon(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnalyticsEpsilonRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAnalyticsEpsilonResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/UpdateAnalyticsEpsilon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.OrganizationService",
                        "UpdateAnalyticsEpsilon",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a sandbox organization for testing configurations.
 Sandbox orgs auto-delete after expires_at. SCIM provisioning is allowed
 for IdP testing (users created in DB only, not in Cognito).
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn create_sandbox_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSandboxOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSandboxOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/CreateSandboxOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.OrganizationService",
                        "CreateSandboxOrganization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List all organizations the authenticated user belongs to.
 Org-exempt: callable without org context (only requires valid JWT).
 Used by the admin org switcher to discover available orgs.
 Excludes expired sandbox organizations.
 Authorization: Authenticated user (no specific permission required).
*/
        pub async fn list_user_organizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserOrganizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserOrganizationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.OrganizationService/ListUserOrganizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.OrganizationService",
                        "ListUserOrganizations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod organization_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrganizationServiceServer.
    #[async_trait]
    pub trait OrganizationService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new organization with an initial admin user.
 Supports API key auth (service-to-service) and JWT auth (self-service onboarding).
*/
        async fn create_organization(
            &self,
            request: tonic::Request<super::CreateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrganizationResponse>,
            tonic::Status,
        >;
        /** Retrieve the organization for the authenticated user.
 Authorization: Requires PERMISSION_ORG_READ.
*/
        async fn get_organization(
            &self,
            request: tonic::Request<super::GetOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationResponse>,
            tonic::Status,
        >;
        /** Update organization settings (name, default workflow, industry, company size).
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn update_organization(
            &self,
            request: tonic::Request<super::UpdateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationResponse>,
            tonic::Status,
        >;
        /** Replace all SSO attribute mappings for the organization.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn update_sso_attribute_mappings(
            &self,
            request: tonic::Request<super::UpdateSsoAttributeMappingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSsoAttributeMappingsResponse>,
            tonic::Status,
        >;
        /** Rotate the analytics salt and optionally increase the bucket count for k-anonymization.
 Authorization: Requires PERMISSION_PRIVACY_WRITE.
*/
        async fn rotate_analytics_salt(
            &self,
            request: tonic::Request<super::RotateAnalyticsSaltRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateAnalyticsSaltResponse>,
            tonic::Status,
        >;
        /** Update the differential privacy epsilon parameter.
 Authorization: Requires PERMISSION_PRIVACY_WRITE.
*/
        async fn update_analytics_epsilon(
            &self,
            request: tonic::Request<super::UpdateAnalyticsEpsilonRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAnalyticsEpsilonResponse>,
            tonic::Status,
        >;
        /** Create a sandbox organization for testing configurations.
 Sandbox orgs auto-delete after expires_at. SCIM provisioning is allowed
 for IdP testing (users created in DB only, not in Cognito).
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn create_sandbox_organization(
            &self,
            request: tonic::Request<super::CreateSandboxOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSandboxOrganizationResponse>,
            tonic::Status,
        >;
        /** List all organizations the authenticated user belongs to.
 Org-exempt: callable without org context (only requires valid JWT).
 Used by the admin org switcher to discover available orgs.
 Excludes expired sandbox organizations.
 Authorization: Authenticated user (no specific permission required).
*/
        async fn list_user_organizations(
            &self,
            request: tonic::Request<super::ListUserOrganizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserOrganizationsResponse>,
            tonic::Status,
        >;
    }
    /** Manages organizations (tenants) in the Pidgr platform.
 Most RPCs operate within the caller's org (extracted from JWT).
 CreateOrganization supports API key auth or JWT auth (self-service onboarding).
*/
    #[derive(Debug)]
    pub struct OrganizationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> OrganizationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrganizationServiceServer<T>
    where
        T: OrganizationService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.OrganizationService/CreateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<super::CreateOrganizationRequest>
                    for CreateOrganizationSvc<T> {
                        type Response = super::CreateOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::create_organization(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/GetOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<super::GetOrganizationRequest>
                    for GetOrganizationSvc<T> {
                        type Response = super::GetOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::get_organization(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/UpdateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<super::UpdateOrganizationRequest>
                    for UpdateOrganizationSvc<T> {
                        type Response = super::UpdateOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::update_organization(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/UpdateSsoAttributeMappings" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSsoAttributeMappingsSvc<T: OrganizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<
                        super::UpdateSsoAttributeMappingsRequest,
                    > for UpdateSsoAttributeMappingsSvc<T> {
                        type Response = super::UpdateSsoAttributeMappingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateSsoAttributeMappingsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::update_sso_attribute_mappings(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateSsoAttributeMappingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/RotateAnalyticsSalt" => {
                    #[allow(non_camel_case_types)]
                    struct RotateAnalyticsSaltSvc<T: OrganizationService>(pub Arc<T>);
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<super::RotateAnalyticsSaltRequest>
                    for RotateAnalyticsSaltSvc<T> {
                        type Response = super::RotateAnalyticsSaltResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RotateAnalyticsSaltRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::rotate_analytics_salt(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RotateAnalyticsSaltSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/UpdateAnalyticsEpsilon" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAnalyticsEpsilonSvc<T: OrganizationService>(pub Arc<T>);
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<super::UpdateAnalyticsEpsilonRequest>
                    for UpdateAnalyticsEpsilonSvc<T> {
                        type Response = super::UpdateAnalyticsEpsilonResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAnalyticsEpsilonRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::update_analytics_epsilon(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateAnalyticsEpsilonSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/CreateSandboxOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSandboxOrganizationSvc<T: OrganizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<
                        super::CreateSandboxOrganizationRequest,
                    > for CreateSandboxOrganizationSvc<T> {
                        type Response = super::CreateSandboxOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateSandboxOrganizationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::create_sandbox_organization(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateSandboxOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.OrganizationService/ListUserOrganizations" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserOrganizationsSvc<T: OrganizationService>(pub Arc<T>);
                    impl<
                        T: OrganizationService,
                    > tonic::server::UnaryService<super::ListUserOrganizationsRequest>
                    for ListUserOrganizationsSvc<T> {
                        type Response = super::ListUserOrganizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserOrganizationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::list_user_organizations(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListUserOrganizationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for OrganizationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.OrganizationService";
    impl<T> tonic::server::NamedService for OrganizationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod render_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Internal service for batch template rendering.
*/
    #[derive(Debug, Clone)]
    pub struct RenderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RenderServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RenderServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RenderServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RenderServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Render a template for multiple users, streaming results as each completes.
 Authorization: Internal server-to-server only. Not exposed to external clients.
*/
        pub async fn render_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::RenderBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::RenderBatchResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.RenderService/RenderBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.RenderService", "RenderBatch"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod render_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RenderServiceServer.
    #[async_trait]
    pub trait RenderService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the RenderBatch method.
        type RenderBatchStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::RenderBatchResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /** Render a template for multiple users, streaming results as each completes.
 Authorization: Internal server-to-server only. Not exposed to external clients.
*/
        async fn render_batch(
            &self,
            request: tonic::Request<super::RenderBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::RenderBatchStream>,
            tonic::Status,
        >;
    }
    /** Internal service for batch template rendering.
*/
    #[derive(Debug)]
    pub struct RenderServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RenderServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RenderServiceServer<T>
    where
        T: RenderService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.RenderService/RenderBatch" => {
                    #[allow(non_camel_case_types)]
                    struct RenderBatchSvc<T: RenderService>(pub Arc<T>);
                    impl<
                        T: RenderService,
                    > tonic::server::ServerStreamingService<super::RenderBatchRequest>
                    for RenderBatchSvc<T> {
                        type Response = super::RenderBatchResponse;
                        type ResponseStream = T::RenderBatchStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenderBatchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RenderService>::render_batch(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RenderBatchSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for RenderServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.RenderService";
    impl<T> tonic::server::NamedService for RenderServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod replay_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Proxies the analytics provider's session recording API, keeping credentials server-side.
 All data is fetched from the analytics provider on demand; no recording data is stored in pidgr.
*/
    #[derive(Debug, Clone)]
    pub struct ReplayServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReplayServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReplayServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ReplayServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ReplayServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** List recent session recordings with optional campaign and time range filters.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        pub async fn list_session_recordings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionRecordingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSessionRecordingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.ReplayService/ListSessionRecordings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.ReplayService", "ListSessionRecordings"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Fetch the full rrweb snapshot data for a single recording.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        pub async fn get_session_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionSnapshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSessionSnapshotsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.ReplayService/GetSessionSnapshots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pidgr.v1.ReplayService", "GetSessionSnapshots"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod replay_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReplayServiceServer.
    #[async_trait]
    pub trait ReplayService: std::marker::Send + std::marker::Sync + 'static {
        /** List recent session recordings with optional campaign and time range filters.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        async fn list_session_recordings(
            &self,
            request: tonic::Request<super::ListSessionRecordingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSessionRecordingsResponse>,
            tonic::Status,
        >;
        /** Fetch the full rrweb snapshot data for a single recording.
 Authorization: Requires CAMPAIGNS_READ permission.
*/
        async fn get_session_snapshots(
            &self,
            request: tonic::Request<super::GetSessionSnapshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSessionSnapshotsResponse>,
            tonic::Status,
        >;
    }
    /** Proxies the analytics provider's session recording API, keeping credentials server-side.
 All data is fetched from the analytics provider on demand; no recording data is stored in pidgr.
*/
    #[derive(Debug)]
    pub struct ReplayServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ReplayServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReplayServiceServer<T>
    where
        T: ReplayService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.ReplayService/ListSessionRecordings" => {
                    #[allow(non_camel_case_types)]
                    struct ListSessionRecordingsSvc<T: ReplayService>(pub Arc<T>);
                    impl<
                        T: ReplayService,
                    > tonic::server::UnaryService<super::ListSessionRecordingsRequest>
                    for ListSessionRecordingsSvc<T> {
                        type Response = super::ListSessionRecordingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSessionRecordingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReplayService>::list_session_recordings(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListSessionRecordingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.ReplayService/GetSessionSnapshots" => {
                    #[allow(non_camel_case_types)]
                    struct GetSessionSnapshotsSvc<T: ReplayService>(pub Arc<T>);
                    impl<
                        T: ReplayService,
                    > tonic::server::UnaryService<super::GetSessionSnapshotsRequest>
                    for GetSessionSnapshotsSvc<T> {
                        type Response = super::GetSessionSnapshotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSessionSnapshotsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReplayService>::get_session_snapshots(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetSessionSnapshotsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ReplayServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.ReplayService";
    impl<T> tonic::server::NamedService for ReplayServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod role_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages roles and their permissions within an organization.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug, Clone)]
    pub struct RoleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RoleServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RoleServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RoleServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RoleServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** List all roles in the organization with their permission sets.
 Authorization: Requires PERMISSION_ORG_READ.
*/
        pub async fn list_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRolesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.RoleService/ListRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.RoleService", "ListRoles"));
            self.inner.unary(req, path, codec).await
        }
        /** Create a new custom role with a name and initial permissions.
 Slug is auto-generated from the name and immutable after creation.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn create_role(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRoleResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.RoleService/CreateRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.RoleService", "CreateRole"));
            self.inner.unary(req, path, codec).await
        }
        /** Update a role's name and/or permissions.
 System roles (is_system=true) cannot be updated.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn update_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRoleResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.RoleService/UpdateRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.RoleService", "UpdateRole"));
            self.inner.unary(req, path, codec).await
        }
        /** Delete a role. Fails if any users are assigned to it.
 System roles (is_system=true) cannot be deleted.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        pub async fn delete_role(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRoleResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.RoleService/DeleteRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.RoleService", "DeleteRole"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod role_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RoleServiceServer.
    #[async_trait]
    pub trait RoleService: std::marker::Send + std::marker::Sync + 'static {
        /** List all roles in the organization with their permission sets.
 Authorization: Requires PERMISSION_ORG_READ.
*/
        async fn list_roles(
            &self,
            request: tonic::Request<super::ListRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRolesResponse>,
            tonic::Status,
        >;
        /** Create a new custom role with a name and initial permissions.
 Slug is auto-generated from the name and immutable after creation.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn create_role(
            &self,
            request: tonic::Request<super::CreateRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRoleResponse>,
            tonic::Status,
        >;
        /** Update a role's name and/or permissions.
 System roles (is_system=true) cannot be updated.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn update_role(
            &self,
            request: tonic::Request<super::UpdateRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRoleResponse>,
            tonic::Status,
        >;
        /** Delete a role. Fails if any users are assigned to it.
 System roles (is_system=true) cannot be deleted.
 Authorization: Requires PERMISSION_MEMBERS_MANAGE.
*/
        async fn delete_role(
            &self,
            request: tonic::Request<super::DeleteRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRoleResponse>,
            tonic::Status,
        >;
    }
    /** Manages roles and their permissions within an organization.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug)]
    pub struct RoleServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RoleServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RoleServiceServer<T>
    where
        T: RoleService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.RoleService/ListRoles" => {
                    #[allow(non_camel_case_types)]
                    struct ListRolesSvc<T: RoleService>(pub Arc<T>);
                    impl<
                        T: RoleService,
                    > tonic::server::UnaryService<super::ListRolesRequest>
                    for ListRolesSvc<T> {
                        type Response = super::ListRolesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRolesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoleService>::list_roles(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListRolesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.RoleService/CreateRole" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRoleSvc<T: RoleService>(pub Arc<T>);
                    impl<
                        T: RoleService,
                    > tonic::server::UnaryService<super::CreateRoleRequest>
                    for CreateRoleSvc<T> {
                        type Response = super::CreateRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoleService>::create_role(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateRoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.RoleService/UpdateRole" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRoleSvc<T: RoleService>(pub Arc<T>);
                    impl<
                        T: RoleService,
                    > tonic::server::UnaryService<super::UpdateRoleRequest>
                    for UpdateRoleSvc<T> {
                        type Response = super::UpdateRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoleService>::update_role(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateRoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.RoleService/DeleteRole" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRoleSvc<T: RoleService>(pub Arc<T>);
                    impl<
                        T: RoleService,
                    > tonic::server::UnaryService<super::DeleteRoleRequest>
                    for DeleteRoleSvc<T> {
                        type Response = super::DeleteRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoleService>::delete_role(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteRoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for RoleServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.RoleService";
    impl<T> tonic::server::NamedService for RoleServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod sso_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages SSO identity provider configuration for organizations.
*/
    #[derive(Debug, Clone)]
    pub struct SsoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SsoServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SsoServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SsoServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SsoServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Check if an email domain has SSO configured.
 This is a pre-authentication endpoint — no JWT required.
 Authorization: None (public).
*/
        pub async fn check_sso_by_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckSsoByDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckSsoByDomainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.SSOService/CheckSSOByDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.SSOService", "CheckSSOByDomain"));
            self.inner.unary(req, path, codec).await
        }
        /** Create an SSO provider for the organization.
 Validates the metadata URL before saving.
 Creates the corresponding identity provider in the auth service.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn create_sso_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSsoProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSsoProviderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.SSOService/CreateSSOProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.SSOService", "CreateSSOProvider"));
            self.inner.unary(req, path, codec).await
        }
        /** Get the organization's SSO provider configuration.
 Authorization: Requires PERMISSION_ORG_READ.
*/
        pub async fn get_sso_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSsoProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSsoProviderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.SSOService/GetSSOProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.SSOService", "GetSSOProvider"));
            self.inner.unary(req, path, codec).await
        }
        /** Delete the organization's SSO provider.
 Deletes the corresponding identity provider from the auth service.
 Users with that domain fall back to passkey/OTP.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        pub async fn delete_sso_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSsoProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteSsoProviderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.SSOService/DeleteSSOProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.SSOService", "DeleteSSOProvider"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod sso_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SsoServiceServer.
    #[async_trait]
    pub trait SsoService: std::marker::Send + std::marker::Sync + 'static {
        /** Check if an email domain has SSO configured.
 This is a pre-authentication endpoint — no JWT required.
 Authorization: None (public).
*/
        async fn check_sso_by_domain(
            &self,
            request: tonic::Request<super::CheckSsoByDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckSsoByDomainResponse>,
            tonic::Status,
        >;
        /** Create an SSO provider for the organization.
 Validates the metadata URL before saving.
 Creates the corresponding identity provider in the auth service.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn create_sso_provider(
            &self,
            request: tonic::Request<super::CreateSsoProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSsoProviderResponse>,
            tonic::Status,
        >;
        /** Get the organization's SSO provider configuration.
 Authorization: Requires PERMISSION_ORG_READ.
*/
        async fn get_sso_provider(
            &self,
            request: tonic::Request<super::GetSsoProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSsoProviderResponse>,
            tonic::Status,
        >;
        /** Delete the organization's SSO provider.
 Deletes the corresponding identity provider from the auth service.
 Users with that domain fall back to passkey/OTP.
 Authorization: Requires PERMISSION_ORG_WRITE.
*/
        async fn delete_sso_provider(
            &self,
            request: tonic::Request<super::DeleteSsoProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteSsoProviderResponse>,
            tonic::Status,
        >;
    }
    /** Manages SSO identity provider configuration for organizations.
*/
    #[derive(Debug)]
    pub struct SsoServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SsoServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SsoServiceServer<T>
    where
        T: SsoService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.SSOService/CheckSSOByDomain" => {
                    #[allow(non_camel_case_types)]
                    struct CheckSSOByDomainSvc<T: SsoService>(pub Arc<T>);
                    impl<
                        T: SsoService,
                    > tonic::server::UnaryService<super::CheckSsoByDomainRequest>
                    for CheckSSOByDomainSvc<T> {
                        type Response = super::CheckSsoByDomainResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckSsoByDomainRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SsoService>::check_sso_by_domain(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CheckSSOByDomainSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.SSOService/CreateSSOProvider" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSSOProviderSvc<T: SsoService>(pub Arc<T>);
                    impl<
                        T: SsoService,
                    > tonic::server::UnaryService<super::CreateSsoProviderRequest>
                    for CreateSSOProviderSvc<T> {
                        type Response = super::CreateSsoProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSsoProviderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SsoService>::create_sso_provider(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateSSOProviderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.SSOService/GetSSOProvider" => {
                    #[allow(non_camel_case_types)]
                    struct GetSSOProviderSvc<T: SsoService>(pub Arc<T>);
                    impl<
                        T: SsoService,
                    > tonic::server::UnaryService<super::GetSsoProviderRequest>
                    for GetSSOProviderSvc<T> {
                        type Response = super::GetSsoProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSsoProviderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SsoService>::get_sso_provider(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetSSOProviderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.SSOService/DeleteSSOProvider" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSSOProviderSvc<T: SsoService>(pub Arc<T>);
                    impl<
                        T: SsoService,
                    > tonic::server::UnaryService<super::DeleteSsoProviderRequest>
                    for DeleteSSOProviderSvc<T> {
                        type Response = super::DeleteSsoProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSsoProviderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SsoService>::delete_sso_provider(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteSSOProviderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for SsoServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.SSOService";
    impl<T> tonic::server::NamedService for SsoServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod team_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages organizational teams (departments, divisions) within an organization.
 Teams represent the organizational structure and can serve as sender identity.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug, Clone)]
    pub struct TeamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TeamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TeamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TeamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            TeamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new team in the organization.
 Authorization: Requires PERMISSION_TEAMS_WRITE or PERMISSION_TEAMS_ALL_WRITE.
*/
        pub async fn create_team(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTeamResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/CreateTeam",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "CreateTeam"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve a team by ID.
 Authorization: Caller must be a member of the team, or have
 PERMISSION_TEAMS_ALL_READ or PERMISSION_TEAMS_ALL_WRITE.
*/
        pub async fn get_team(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTeamResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/GetTeam",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "GetTeam"));
            self.inner.unary(req, path, codec).await
        }
        /** List teams in the organization with pagination.
 Without PERMISSION_TEAMS_ALL_READ/ALL_WRITE, returns only teams the caller belongs to.
*/
        pub async fn list_teams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTeamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTeamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/ListTeams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "ListTeams"));
            self.inner.unary(req, path, codec).await
        }
        /** Update a team's name and/or description.
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        pub async fn update_team(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTeamResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/UpdateTeam",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "UpdateTeam"));
            self.inner.unary(req, path, codec).await
        }
        /** Delete a team and all its membership records. Default teams cannot be deleted.
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        pub async fn delete_team(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTeamResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/DeleteTeam",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "DeleteTeam"));
            self.inner.unary(req, path, codec).await
        }
        /** Add one or more users to a team (idempotent).
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        pub async fn add_team_members(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTeamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddTeamMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/AddTeamMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "AddTeamMembers"));
            self.inner.unary(req, path, codec).await
        }
        /** Remove one or more users from a team (idempotent).
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        pub async fn remove_team_members(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveTeamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTeamMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/RemoveTeamMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "RemoveTeamMembers"));
            self.inner.unary(req, path, codec).await
        }
        /** List members of a team with pagination.
 Authorization: Caller must be a member of the team, or have
 PERMISSION_TEAMS_ALL_READ or PERMISSION_TEAMS_ALL_WRITE.
*/
        pub async fn list_team_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTeamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTeamMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TeamService/ListTeamMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TeamService", "ListTeamMembers"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod team_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TeamServiceServer.
    #[async_trait]
    pub trait TeamService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new team in the organization.
 Authorization: Requires PERMISSION_TEAMS_WRITE or PERMISSION_TEAMS_ALL_WRITE.
*/
        async fn create_team(
            &self,
            request: tonic::Request<super::CreateTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTeamResponse>,
            tonic::Status,
        >;
        /** Retrieve a team by ID.
 Authorization: Caller must be a member of the team, or have
 PERMISSION_TEAMS_ALL_READ or PERMISSION_TEAMS_ALL_WRITE.
*/
        async fn get_team(
            &self,
            request: tonic::Request<super::GetTeamRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTeamResponse>, tonic::Status>;
        /** List teams in the organization with pagination.
 Without PERMISSION_TEAMS_ALL_READ/ALL_WRITE, returns only teams the caller belongs to.
*/
        async fn list_teams(
            &self,
            request: tonic::Request<super::ListTeamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTeamsResponse>,
            tonic::Status,
        >;
        /** Update a team's name and/or description.
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        async fn update_team(
            &self,
            request: tonic::Request<super::UpdateTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTeamResponse>,
            tonic::Status,
        >;
        /** Delete a team and all its membership records. Default teams cannot be deleted.
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        async fn delete_team(
            &self,
            request: tonic::Request<super::DeleteTeamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTeamResponse>,
            tonic::Status,
        >;
        /** Add one or more users to a team (idempotent).
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        async fn add_team_members(
            &self,
            request: tonic::Request<super::AddTeamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddTeamMembersResponse>,
            tonic::Status,
        >;
        /** Remove one or more users from a team (idempotent).
 Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any).
*/
        async fn remove_team_members(
            &self,
            request: tonic::Request<super::RemoveTeamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTeamMembersResponse>,
            tonic::Status,
        >;
        /** List members of a team with pagination.
 Authorization: Caller must be a member of the team, or have
 PERMISSION_TEAMS_ALL_READ or PERMISSION_TEAMS_ALL_WRITE.
*/
        async fn list_team_members(
            &self,
            request: tonic::Request<super::ListTeamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTeamMembersResponse>,
            tonic::Status,
        >;
    }
    /** Manages organizational teams (departments, divisions) within an organization.
 Teams represent the organizational structure and can serve as sender identity.
 All RPCs operate within the caller's org (extracted from JWT).
*/
    #[derive(Debug)]
    pub struct TeamServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> TeamServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TeamServiceServer<T>
    where
        T: TeamService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.TeamService/CreateTeam" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTeamSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::CreateTeamRequest>
                    for CreateTeamSvc<T> {
                        type Response = super::CreateTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTeamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::create_team(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateTeamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/GetTeam" => {
                    #[allow(non_camel_case_types)]
                    struct GetTeamSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::GetTeamRequest>
                    for GetTeamSvc<T> {
                        type Response = super::GetTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTeamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::get_team(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetTeamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/ListTeams" => {
                    #[allow(non_camel_case_types)]
                    struct ListTeamsSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::ListTeamsRequest>
                    for ListTeamsSvc<T> {
                        type Response = super::ListTeamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTeamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::list_teams(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListTeamsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/UpdateTeam" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTeamSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::UpdateTeamRequest>
                    for UpdateTeamSvc<T> {
                        type Response = super::UpdateTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTeamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::update_team(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateTeamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/DeleteTeam" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTeamSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::DeleteTeamRequest>
                    for DeleteTeamSvc<T> {
                        type Response = super::DeleteTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTeamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::delete_team(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteTeamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/AddTeamMembers" => {
                    #[allow(non_camel_case_types)]
                    struct AddTeamMembersSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::AddTeamMembersRequest>
                    for AddTeamMembersSvc<T> {
                        type Response = super::AddTeamMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddTeamMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::add_team_members(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddTeamMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/RemoveTeamMembers" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTeamMembersSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::RemoveTeamMembersRequest>
                    for RemoveTeamMembersSvc<T> {
                        type Response = super::RemoveTeamMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveTeamMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::remove_team_members(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveTeamMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TeamService/ListTeamMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListTeamMembersSvc<T: TeamService>(pub Arc<T>);
                    impl<
                        T: TeamService,
                    > tonic::server::UnaryService<super::ListTeamMembersRequest>
                    for ListTeamMembersSvc<T> {
                        type Response = super::ListTeamMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTeamMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TeamService>::list_team_members(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListTeamMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for TeamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.TeamService";
    impl<T> tonic::server::NamedService for TeamServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod template_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manages versioned message templates used by campaigns.
 Templates are append-only — updates create new versions while preserving history.
*/
    #[derive(Debug, Clone)]
    pub struct TemplateServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TemplateServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TemplateServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TemplateServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            TemplateServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /** Create a new template with a body and variable definitions.
 Authorization: Requires MANAGER+ role.
*/
        pub async fn create_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTemplateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/CreateTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TemplateService", "CreateTemplate"));
            self.inner.unary(req, path, codec).await
        }
        /** Update an existing template, creating a new version.
 Authorization: Requires MANAGER+ role.
*/
        pub async fn update_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTemplateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/UpdateTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TemplateService", "UpdateTemplate"));
            self.inner.unary(req, path, codec).await
        }
        /** Retrieve a specific template by ID and optional version.
 Authorization: Authenticated user within the organization.
*/
        pub async fn get_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTemplateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/GetTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TemplateService", "GetTemplate"));
            self.inner.unary(req, path, codec).await
        }
        /** List all templates for the organization with pagination.
 Authorization: Authenticated user within the organization.
*/
        pub async fn list_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTemplatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTemplatesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/ListTemplates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pidgr.v1.TemplateService", "ListTemplates"));
            self.inner.unary(req, path, codec).await
        }
        /** Create a locale-specific translation of a template.
 Authorization: Requires PERMISSION_TEMPLATES_WRITE.
*/
        pub async fn create_template_translation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTemplateTranslationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTemplateTranslationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/CreateTemplateTranslation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.TemplateService",
                        "CreateTemplateTranslation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update an existing template translation.
 Authorization: Requires PERMISSION_TEMPLATES_WRITE.
*/
        pub async fn update_template_translation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTemplateTranslationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTemplateTranslationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/UpdateTemplateTranslation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.TemplateService",
                        "UpdateTemplateTranslation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List all translations for a template version.
 Authorization: Requires PERMISSION_TEMPLATES_READ.
*/
        pub async fn list_template_translations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTemplateTranslationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTemplateTranslationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/ListTemplateTranslations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.TemplateService",
                        "ListTemplateTranslations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Approve a template translation for use in campaigns.
 Authorization: Requires PERMISSION_TEMPLATES_REVIEW.
*/
        pub async fn approve_template_translation(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveTemplateTranslationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApproveTemplateTranslationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pidgr.v1.TemplateService/ApproveTemplateTranslation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pidgr.v1.TemplateService",
                        "ApproveTemplateTranslation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod template_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TemplateServiceServer.
    #[async_trait]
    pub trait TemplateService: std::marker::Send + std::marker::Sync + 'static {
        /** Create a new template with a body and variable definitions.
 Authorization: Requires MANAGER+ role.
*/
        async fn create_template(
            &self,
            request: tonic::Request<super::CreateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTemplateResponse>,
            tonic::Status,
        >;
        /** Update an existing template, creating a new version.
 Authorization: Requires MANAGER+ role.
*/
        async fn update_template(
            &self,
            request: tonic::Request<super::UpdateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTemplateResponse>,
            tonic::Status,
        >;
        /** Retrieve a specific template by ID and optional version.
 Authorization: Authenticated user within the organization.
*/
        async fn get_template(
            &self,
            request: tonic::Request<super::GetTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTemplateResponse>,
            tonic::Status,
        >;
        /** List all templates for the organization with pagination.
 Authorization: Authenticated user within the organization.
*/
        async fn list_templates(
            &self,
            request: tonic::Request<super::ListTemplatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTemplatesResponse>,
            tonic::Status,
        >;
        /** Create a locale-specific translation of a template.
 Authorization: Requires PERMISSION_TEMPLATES_WRITE.
*/
        async fn create_template_translation(
            &self,
            request: tonic::Request<super::CreateTemplateTranslationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTemplateTranslationResponse>,
            tonic::Status,
        >;
        /** Update an existing template translation.
 Authorization: Requires PERMISSION_TEMPLATES_WRITE.
*/
        async fn update_template_translation(
            &self,
            request: tonic::Request<super::UpdateTemplateTranslationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTemplateTranslationResponse>,
            tonic::Status,
        >;
        /** List all translations for a template version.
 Authorization: Requires PERMISSION_TEMPLATES_READ.
*/
        async fn list_template_translations(
            &self,
            request: tonic::Request<super::ListTemplateTranslationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTemplateTranslationsResponse>,
            tonic::Status,
        >;
        /** Approve a template translation for use in campaigns.
 Authorization: Requires PERMISSION_TEMPLATES_REVIEW.
*/
        async fn approve_template_translation(
            &self,
            request: tonic::Request<super::ApproveTemplateTranslationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApproveTemplateTranslationResponse>,
            tonic::Status,
        >;
    }
    /** Manages versioned message templates used by campaigns.
 Templates are append-only — updates create new versions while preserving history.
*/
    #[derive(Debug)]
    pub struct TemplateServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> TemplateServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TemplateServiceServer<T>
    where
        T: TemplateService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/pidgr.v1.TemplateService/CreateTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTemplateSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<super::CreateTemplateRequest>
                    for CreateTemplateSvc<T> {
                        type Response = super::CreateTemplateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTemplateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::create_template(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateTemplateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/UpdateTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTemplateSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<super::UpdateTemplateRequest>
                    for UpdateTemplateSvc<T> {
                        type Response = super::UpdateTemplateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTemplateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::update_template(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateTemplateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/GetTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct GetTemplateSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<super::GetTemplateRequest>
                    for GetTemplateSvc<T> {
                        type Response = super::GetTemplateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTemplateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::get_template(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetTemplateSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/ListTemplates" => {
                    #[allow(non_camel_case_types)]
                    struct ListTemplatesSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<super::ListTemplatesRequest>
                    for ListTemplatesSvc<T> {
                        type Response = super::ListTemplatesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTemplatesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::list_templates(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListTemplatesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/CreateTemplateTranslation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTemplateTranslationSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<
                        super::CreateTemplateTranslationRequest,
                    > for CreateTemplateTranslationSvc<T> {
                        type Response = super::CreateTemplateTranslationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateTemplateTranslationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::create_template_translation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateTemplateTranslationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/UpdateTemplateTranslation" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTemplateTranslationSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<
                        super::UpdateTemplateTranslationRequest,
                    > for UpdateTemplateTranslationSvc<T> {
                        type Response = super::UpdateTemplateTranslationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateTemplateTranslationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::update_template_translation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateTemplateTranslationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/ListTemplateTranslations" => {
                    #[allow(non_camel_case_types)]
                    struct ListTemplateTranslationsSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<super::ListTemplateTranslationsRequest>
                    for ListTemplateTranslationsSvc<T> {
                        type Response = super::ListTemplateTranslationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListTemplateTranslationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::list_template_translations(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListTemplateTranslationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pidgr.v1.TemplateService/ApproveTemplateTranslation" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveTemplateTranslationSvc<T: TemplateService>(pub Arc<T>);
                    impl<
                        T: TemplateService,
                    > tonic::server::UnaryService<
                        super::ApproveTemplateTranslationRequest,
                    > for ApproveTemplateTranslationSvc<T> {
                        type Response = super::ApproveTemplateTranslationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ApproveTemplateTranslationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TemplateService>::approve_template_translation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ApproveTemplateTranslationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for TemplateServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "pidgr.v1.TemplateService";
    impl<T> tonic::server::NamedService for TemplateServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}

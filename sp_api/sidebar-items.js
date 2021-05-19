initSidebarItems({"constant":[["MAX_EXTRINSIC_DEPTH","Maximum nesting level for extrinsics."]],"enum":[["InitializeBlock","Before calling any runtime api function, the runtime need to be initialized at the requested block. However, some functions like `execute_block` or `initialize_block` itself don’t require to have the runtime initialized at the requested block."]],"macro":[["decl_runtime_apis","Declares given traits as runtime apis."],["impl_runtime_apis","Tags given trait implementations as runtime apis."],["mock_impl_runtime_apis","Mocks given trait implementations as runtime apis."]],"struct":[["ApiError","An error describing which API call failed."],["ApiRef","Auxiliary wrapper that holds an api instance and binds it to the given lifetime."],["CallApiAtParams","Parameters for [`CallApiAt::call_api_at`]."],["OldRuntimeVersion",""]],"trait":[["ApiErrorExt","Extends the runtime api traits with an associated error type. This trait is given as super trait to every runtime api trait."],["ApiExt","Extends the runtime api implementation with some common functionality."],["CallApiAt","Something that can call into the an api at a given block."],["ConstructRuntimeApi","Something that can be constructed to a runtime api."],["Core","The `Core` runtime api that every Substrate runtime needs to implement."],["Metadata","The `Metadata` api trait that returns metadata for the runtime."],["ProvideRuntimeApi","Something that provides a runtime api."],["RuntimeApiInfo","Something that provides information about a runtime api."]],"type":[["ApiErrorFor","Extracts the `Api::Error` for a type that provides a runtime api."],["ProofRecorder","A type that records all accessed trie nodes and generates a proof out of it."],["StateBackendFor","Extract the state backend type for a type that implements `ProvideRuntimeApi`."],["StorageChanges",""],["StorageTransactionCache","A type that is used as cache for the storage transactions."],["TransactionFor","Extract the state backend transaction type for a type that implements `ProvideRuntimeApi`."]]});
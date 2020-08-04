use binding_macro::{cycles, genesis, service};
use protocol::traits::{ExecutorParams, ServiceResponse, ServiceSDK};
use protocol::types::{Metadata, ServiceContext, METADATA_KEY};

pub struct MetadataService<SDK> {
    sdk: SDK,
}

#[service]
impl<SDK: ServiceSDK> MetadataService<SDK> {
    pub fn new(sdk: SDK) -> Self {
        Self { sdk }
    }

    #[genesis]
    fn init_genesis(&mut self, metadata: Metadata) {
        self.sdk.set_value(METADATA_KEY.to_string(), metadata)
    }

    #[cycles(210_00)]
    #[read]
    fn get_metadata(&self, ctx: ServiceContext) -> ServiceResponse<Metadata> {
        let metadata: Metadata = self
            .sdk
            .get_value(&METADATA_KEY.to_owned())
            .expect("Metadata should always be in the genesis block");
        ServiceResponse::from_succeed(metadata)
    }
}

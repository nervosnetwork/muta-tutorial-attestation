mod types;
use binding_macro::{cycles, service};
use protocol::traits::{ExecutorParams, ServiceResponse, ServiceSDK, StoreMap};
use protocol::types::{Hash, ServiceContext};

use crate::types::{AttestInfoPayload, QueryAttestedInfoPayload, QueryAttestedInfoResponse};

const ATTESTED_INFO_KEY: &str = "attested_info";

pub trait AttestationInterface {
    fn inner_attest(
        &mut self,
        ctx: &ServiceContext,
        payload: AttestInfoPayload,
    ) -> ServiceResponse<Hash>;

    fn inner_query(
        &self,
        ctx: &ServiceContext,
        payload: QueryAttestedInfoPayload,
    ) -> ServiceResponse<QueryAttestedInfoResponse>;
}

pub struct AttestationService<SDK> {
    _sdk: SDK,
    attested_info: Box<dyn StoreMap<Hash, String>>,
}

impl<SDK: 'static + ServiceSDK> AttestationInterface for AttestationService<SDK> {
    fn inner_attest(
        &mut self,
        ctx: &ServiceContext,
        payload: AttestInfoPayload,
    ) -> ServiceResponse<Hash> {
        self.attest_info(ctx.clone(), payload)
    }

    fn inner_query(
        &self,
        ctx: &ServiceContext,
        payload: QueryAttestedInfoPayload,
    ) -> ServiceResponse<QueryAttestedInfoResponse> {
        self.query_attested_info(ctx.clone(), payload)
    }
}

#[service]
impl<SDK: 'static + ServiceSDK> AttestationService<SDK> {
    pub fn new(mut sdk: SDK) -> Self {
        let attested_info: Box<dyn StoreMap<Hash, String>> =
            sdk.alloc_or_recover_map(ATTESTED_INFO_KEY);

        Self {
            _sdk: sdk,
            attested_info,
        }
    }

    #[cycles(21_000)]
    #[write]
    fn attest_info(
        &mut self,
        ctx: ServiceContext,
        payload: AttestInfoPayload,
    ) -> ServiceResponse<Hash> {
        if let Some(hash) = ctx.get_tx_hash() {
            self.attested_info.insert(hash.clone(), payload.info);
            ServiceResponse::from_succeed(hash)
        } else {
            ServiceResponse::from_error(101, "Can not get tx hash".to_string())
        }
    }

    #[cycles(210_00)]
    #[read]
    fn query_attested_info(
        &self,
        ctx: ServiceContext,
        payload: QueryAttestedInfoPayload,
    ) -> ServiceResponse<QueryAttestedInfoResponse> {
        if let Some(info) = self.attested_info.get(&payload.hash) {
            ServiceResponse::from_succeed(QueryAttestedInfoResponse {
                attested_info: info,
            })
        } else {
            ServiceResponse::from_error(102, "Can not get attested info".to_string())
        }
    }
}

use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::support_card_protocol_validation_request::SupportCardProtocolValidationRequest;
use crate::game_protocol_validation::service::response::can_use_card_response::CanUseCardResponse;
use crate::game_protocol_validation::service::response::check_protocol_hacking_response::CheckProtocolHackingResponse;
use crate::game_protocol_validation::service::response::is_it_support_card_response::IsItSupportCardResponse;
use crate::game_protocol_validation::service::response::support_card_protocol_validation_response::SupportCardProtocolValidationResponse;

#[async_trait]
pub trait GameProtocolValidationService {
    async fn check_protocol_hacking(&mut self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> CheckProtocolHackingResponse;
    async fn can_use_card(&mut self, can_use_card_request: CanUseCardRequest) -> CanUseCardResponse;
    async fn is_it_support_card(&self, is_it_support_card_request: IsItSupportCardRequest) -> IsItSupportCardResponse;
}
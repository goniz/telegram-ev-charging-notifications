/* Auto-generated using: https://transform.tools/json-to-rust-serde */

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvedgeChargingStation {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "FriendlyName")]
    pub friendly_name: String,
    #[serde(rename = "FriendlyCode")]
    pub friendly_code: String,
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "Type")]
    pub type_field: Type,
    #[serde(rename = "AccessType")]
    pub access_type: AccessType,
    #[serde(rename = "Access")]
    pub access: Access,
    #[serde(rename = "PoiTypes")]
    pub poi_types: Vec<Value>,
    #[serde(rename = "WorkingTime")]
    pub working_time: Vec<Value>,
    #[serde(rename = "WorkingTime24_7")]
    pub working_time24_7: bool,
    #[serde(rename = "IsFavourite")]
    pub is_favourite: bool,
    #[serde(rename = "AssetStatus")]
    pub asset_status: AssetStatus,
    #[serde(rename = "ChargePoints")]
    pub charge_points: Vec<ChargePoint>,
    #[serde(rename = "TotalEvses")]
    pub total_evses: i64,
    #[serde(rename = "AvailableEvses")]
    pub available_evses: i64,
    #[serde(rename = "InstanceReservationEnabled")]
    pub instance_reservation_enabled: bool,
    #[serde(rename = "ChargePointOwnerId")]
    pub charge_point_owner_id: i64,
    #[serde(rename = "ChargePointOwnerCompanyName")]
    pub charge_point_owner_company_name: String,
    #[serde(rename = "InitialLocationPhoto")]
    pub initial_location_photo: String,
    #[serde(rename = "DepartureTimeBehaviourType")]
    pub departure_time_behaviour_type: DepartureTimeBehaviourType,
    #[serde(rename = "InitialLocationPhotoFileId")]
    pub initial_location_photo_file_id: i64,
    #[serde(rename = "ParkingBarrierManagable")]
    pub parking_barrier_managable: bool,
    #[serde(rename = "AuthenticationTypesAC")]
    pub authentication_types_ac: Vec<Value>,
    #[serde(rename = "AuthenticationTypesDC")]
    pub authentication_types_dc: Vec<AuthenticationTypesDc>,
    #[serde(rename = "AuthenticationTypesAny")]
    pub authentication_types_any: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "Country")]
    pub country: Country,
    #[serde(rename = "CityName")]
    pub city_name: String,
    #[serde(rename = "PostNumber")]
    pub post_number: String,
    #[serde(rename = "StreetName")]
    pub street_name: String,
    #[serde(rename = "HouseNumber")]
    pub house_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "ISO2Code")]
    pub iso2code: String,
    #[serde(rename = "ISO3Code")]
    pub iso3code: String,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessType {
    #[serde(rename = "IsPrivate")]
    pub is_private: bool,
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Access {
    #[serde(rename = "GPSLongitude")]
    pub gpslongitude: f64,
    #[serde(rename = "GPSLatitude")]
    pub gpslatitude: f64,
    #[serde(rename = "Instructions")]
    pub instructions: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetStatus {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargePoint {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "FriendlyCode")]
    pub friendly_code: String,
    #[serde(rename = "FloorLevel")]
    pub floor_level: i64,
    #[serde(rename = "ChargePointIdentificationRequired")]
    pub charge_point_identification_required: bool,
    #[serde(rename = "Evses")]
    pub evses: Vec<Evse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evse {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "FriendlyCode")]
    pub friendly_code: String,
    #[serde(rename = "NumPhases")]
    pub num_phases: i64,
    #[serde(rename = "LidLocking")]
    pub lid_locking: bool,
    #[serde(rename = "MaxPower")]
    pub max_power: f64,
    #[serde(rename = "MaxCurrent")]
    pub max_current: i64,
    #[serde(rename = "ReservationMgmEnabled")]
    pub reservation_mgm_enabled: bool,
    #[serde(rename = "RemoteStartEnabled")]
    pub remote_start_enabled: bool,
    #[serde(rename = "Connectors")]
    pub connectors: Vec<Connector>,
    #[serde(rename = "ParkingSensorInstalled")]
    pub parking_sensor_installed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connector {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "EvseCode")]
    pub evse_code: String,
    #[serde(rename = "LocationId")]
    pub location_id: i64,
    #[serde(rename = "ProtocolConnectorCode")]
    pub protocol_connector_code: String,
    #[serde(rename = "OutOfWorkingTime")]
    pub out_of_working_time: bool,
    #[serde(rename = "Type")]
    pub type_field: Type2,
    #[serde(rename = "Status")]
    pub status: Status,
    #[serde(rename = "NumberOfPhases")]
    pub number_of_phases: i64,
    #[serde(rename = "PaymentRequired")]
    pub payment_required: bool,
    #[serde(rename = "RatingIndex")]
    pub rating_index: i64,
    #[serde(rename = "PlannedDepartureTime")]
    pub planned_departure_time: Option<String>,
    #[serde(rename = "CurrentSessionId")]
    pub current_session_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type2 {
    #[serde(rename = "ChargingType")]
    pub charging_type: ChargingType,
    #[serde(rename = "Important")]
    pub important: bool,
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingType {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureTimeBehaviourType {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationTypesDc {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

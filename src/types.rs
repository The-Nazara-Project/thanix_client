#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASN {
	/// 16- or 32-bit autonomous system number
	pub asn: Option<u32>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub provider_count: Option<i64>,
	pub rir: Option<BriefRIR>,
	pub site_count: Option<i64>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASNRange {
	pub asn_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub end: Option<u32>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub rir: Option<BriefRIR>,
	pub slug: Option<String>,
	pub start: Option<u32>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASNRangeRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub end: u32,
	pub name: String,
	pub rir: serde_json::Value,
	pub slug: String,
	pub start: u32,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASNRequest {
	/// 16- or 32-bit autonomous system number
	pub asn: u32,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub rir: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Aggregate {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub date_added: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub prefix: Option<String>,
	pub rir: Option<BriefRIR>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AggregateRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub date_added: Option<String>,
	pub description: String,
	pub prefix: String,
	pub rir: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailableASN {
	pub asn: Option<i64>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailableIP {
	pub address: Option<String>,
	pub description: Option<String>,
	pub family: Option<i64>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailablePrefix {
	pub family: Option<i64>,
	pub prefix: Option<String>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailableVLAN {
	pub group: Option<BriefVLANGroup>,
	pub vid: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BackgroundTask {
	pub args: Option<Vec<serde_json::Value>>,
	pub created_at: Option<String>,
	pub description: Option<String>,
	pub ended_at: Option<String>,
	pub enqueued_at: Option<String>,
	pub func_name: Option<String>,
	pub id: String,
	pub is_canceled: Option<bool>,
	pub is_deferred: Option<bool>,
	pub is_failed: Option<bool>,
	pub is_finished: Option<bool>,
	pub is_queued: Option<bool>,
	pub is_scheduled: Option<bool>,
	pub is_started: Option<bool>,
	pub is_stopped: Option<bool>,
	pub kwargs: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub last_heartbeat: Option<String>,
	pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub origin: Option<String>,
	pub position: Option<i64>,
	pub result: Option<String>,
	pub result_ttl: Option<i64>,
	pub started_at: Option<String>,
	pub status: Option<String>,
	pub timeout: Option<i64>,
	pub url: Option<String>,
	pub worker_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BackgroundTaskRequest {
	pub created_at: String,
	pub description: String,
	pub ended_at: String,
	pub enqueued_at: String,
	pub func_name: String,
	pub id: String,
	pub is_canceled: bool,
	pub is_deferred: bool,
	pub is_failed: bool,
	pub is_finished: bool,
	pub is_queued: bool,
	pub is_scheduled: bool,
	pub is_started: bool,
	pub is_stopped: bool,
	pub last_heartbeat: String,
	pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub origin: String,
	pub result: String,
	pub result_ttl: i64,
	pub started_at: String,
	pub timeout: i64,
	pub worker_name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Bookmark {
	pub created: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub object: Option<serde_json::Value>,
	pub object_id: Option<u64>,
	pub object_type: Option<String>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BookmarkRequest {
	pub object_id: u64,
	pub object_type: String,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCable {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub label: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCableRequest {
	pub description: String,
	pub label: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuit {
	/// Unique circuit ID
	pub cid: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub provider: Option<BriefProvider>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitGroup {
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitGroupAssignmentSerializer_ {
	pub display: Option<String>,
	pub group: Option<BriefCircuitGroup>,
	pub id: i64,
	pub priority: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitGroupAssignmentSerializer_Request {
	pub group: serde_json::Value,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitGroupRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub description: String,
	pub provider: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitType {
	pub circuit_count: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitTypeRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCluster {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterGroup {
	pub cluster_count: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterGroupRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterType {
	pub cluster_count: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterTypeRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefConfigContextProfile {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefConfigContextProfileRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefConfigTemplate {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefConfigTemplateRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContact {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContactRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContactRole {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContactRoleRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCustomFieldChoiceSet {
	pub choices_count: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCustomFieldChoiceSetRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDataFile {
	pub display: Option<String>,
	pub id: i64,
	/// File path relative to the data source's root
	pub path: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDataSource {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDataSourceRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDevice {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceRequest {
	pub description: String,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceRole {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceRoleRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceType {
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub id: i64,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceTypeRequest {
	pub description: String,
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefFHRPGroup {
	pub description: Option<String>,
	pub display: Option<String>,
	pub group_id: Option<u16>,
	pub id: i64,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefFHRPGroupRequest {
	pub description: String,
	pub group_id: u16,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIKEPolicy {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIKEPolicyRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPAddress {
	pub address: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPAddressRequest {
	pub address: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecPolicy {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecPolicyRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecProfile {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecProfileRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInterface {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInterfaceRequest {
	pub description: String,
	pub device: serde_json::Value,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInventoryItemRole {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub inventoryitem_count: Option<i64>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInventoryItemRoleRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefJob {
	pub completed: Option<String>,
	pub created: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefJobRequest {
	pub completed: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPN {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub identifier: Option<i64>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPNRequest {
	pub description: String,
	pub identifier: Option<i64>,
	pub name: String,
	pub slug: String,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `evpn-vpws` - EVPN VPWS
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	/// * `spb` - SPB
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPNTermination {
	pub display: Option<String>,
	pub id: i64,
	pub l2vpn: Option<BriefL2VPN>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPNTerminationRequest {
	pub l2vpn: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefLocation {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub rack_count: Option<i64>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefLocationRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefMACAddress {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub mac_address: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefMACAddressRequest {
	pub description: String,
	pub mac_address: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefManufacturer {
	pub description: Option<String>,
	pub devicetype_count: Option<i64>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefManufacturerRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModule {
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub id: i64,
	pub module_bay: Option<NestedModuleBay>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleRequest {
	pub device: serde_json::Value,
	pub module_bay: NestedModuleBayRequest,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleType {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub profile: Option<BriefModuleTypeProfile>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleTypeProfile {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleTypeProfileRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleTypeRequest {
	pub description: String,
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub profile: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPlatform {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPlatformRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPanel {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub powerfeed_count: Option<i64>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPanelRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPort {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPortRequest {
	pub description: String,
	pub device: serde_json::Value,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPortTemplate {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPortTemplateRequest {
	pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProvider {
	pub circuit_count: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	/// Full name of the provider
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderAccount {
	pub account: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderAccountRequest {
	pub account: String,
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderNetwork {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderNetworkRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderRequest {
	pub description: String,
	/// Full name of the provider
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRIR {
	pub aggregate_count: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRIRRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRack {
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackRole {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub rack_count: Option<i64>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackRoleRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackType {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackTypeRequest {
	pub description: String,
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRearPortTemplate {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRearPortTemplateRequest {
	pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRegion {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub site_count: Option<i64>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRegionRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRole {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub prefix_count: Option<i64>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRoleRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSite {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	/// Full name of the site
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSiteGroup {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub site_count: Option<i64>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSiteGroupRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSiteRequest {
	pub description: String,
	/// Full name of the site
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTag {
	pub color: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenant {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenantGroup {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tenant_count: Option<i64>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenantGroupRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenantRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnel {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnelGroup {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tunnel_count: Option<i64>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnelGroupRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnelRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefUser {
	pub display: Option<String>,
	pub id: i64,
	pub url: Option<String>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLAN {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
	/// Numeric VLAN ID (1-4094)
	pub vid: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANGroup {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANGroupRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANRequest {
	pub description: String,
	pub name: String,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANTranslationPolicy {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANTranslationPolicyRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVRF {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub prefix_count: Option<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVRFRequest {
	pub description: String,
	pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualChassis {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub master: Option<NestedDevice>,
	pub member_count: Option<i64>,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualChassisRequest {
	pub description: String,
	pub master: Option<NestedDeviceRequest>,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuit {
	/// Unique circuit ID
	pub cid: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub provider_network: Option<BriefProviderNetwork>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub description: String,
	pub provider_network: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuitType {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub virtual_circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuitTypeRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualMachine {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualMachineRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefWirelessLANGroup {
	pub _depth: Option<i64>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub wirelesslan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefWirelessLANGroupRequest {
	pub description: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Cable {
	pub a_terminations: Option<Vec<GenericObject>>,
	pub b_terminations: Option<Vec<GenericObject>>,
	pub color: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub length: Option<f64>,
	pub length_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Single-mode Fiber
	/// * `smf-os1` - Single-mode Fiber (OS1)
	/// * `smf-os2` - Single-mode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `power` - Power
	/// * `usb` - USB
	pub r#type: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableRequest {
	pub a_terminations: Vec<GenericObjectRequest>,
	pub b_terminations: Vec<GenericObjectRequest>,
	pub color: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub label: String,
	pub length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	pub length_unit: Option<String>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Single-mode Fiber
	/// * `smf-os1` - Single-mode Fiber (OS1)
	/// * `smf-os2` - Single-mode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `power` - Power
	/// * `usb` - USB
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableTermination {
	pub cable: Option<i64>,
	/// * `A` - A
	/// * `B` - B
	pub cable_end: Option<String>,
	pub created: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub termination: Option<serde_json::Value>,
	pub termination_id: Option<u64>,
	pub termination_type: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableTerminationRequest {
	pub cable: i64,
	/// * `A` - A
	/// * `B` - B
	pub cable_end: String,
	pub termination_id: u64,
	pub termination_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Circuit {
	pub assignments: Option<Vec<BriefCircuitGroupAssignmentSerializer_>>,
	/// Unique circuit ID
	pub cid: Option<String>,
	pub comments: Option<String>,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub distance: Option<f64>,
	pub distance_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub install_date: Option<String>,
	pub last_updated: Option<String>,
	pub provider: Option<BriefProvider>,
	pub provider_account: Option<BriefProviderAccount>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub termination_a: Option<CircuitCircuitTermination>,
	pub termination_date: Option<String>,
	pub termination_z: Option<CircuitCircuitTermination>,
	pub r#type: Option<BriefCircuitType>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitCircuitTermination {
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	pub termination: Option<serde_json::Value>,
	pub termination_id: Option<i64>,
	pub termination_type: Option<String>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	pub url: Option<String>,
	/// ID of the local cross-connect
	pub xconnect_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitCircuitTerminationRequest {
	pub description: String,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	pub termination_id: Option<i64>,
	pub termination_type: Option<String>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroup {
	pub circuit_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroupAssignment {
	pub created: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub group: Option<BriefCircuitGroup>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub member: Option<serde_json::Value>,
	pub member_id: Option<u64>,
	pub member_type: Option<String>,
	pub priority: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub member_id: u64,
	pub member_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroupRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitRequest {
	pub assignments: Vec<BriefCircuitGroupAssignmentSerializer_Request>,
	/// Unique circuit ID
	pub cid: String,
	pub comments: String,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub install_date: Option<String>,
	pub provider: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub termination_date: Option<String>,
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitTermination {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub circuit: Option<BriefCircuit>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Patch panel ID and port number(s)
	pub pp_info: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	/// * `A` - A
	/// * `Z` - Z
	pub term_side: Option<String>,
	pub termination: Option<serde_json::Value>,
	pub termination_id: Option<i64>,
	pub termination_type: Option<String>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	pub url: Option<String>,
	/// ID of the local cross-connect
	pub xconnect_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitTerminationRequest {
	pub circuit: serde_json::Value,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Patch panel ID and port number(s)
	pub pp_info: String,
	pub tags: Vec<NestedTagRequest>,
	/// * `A` - A
	/// * `Z` - Z
	pub term_side: String,
	pub termination_id: Option<i64>,
	pub termination_type: Option<String>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitType {
	pub circuit_count: Option<i64>,
	pub color: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitTypeRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Cluster {
	pub allocated_disk: Option<i64>,
	pub allocated_memory: Option<i64>,
	pub allocated_vcpus: Option<f64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub group: Option<BriefClusterGroup>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub scope: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub r#type: Option<BriefClusterType>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterGroup {
	pub cluster_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterGroupRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub name: String,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterType {
	pub cluster_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterTypeRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigContext {
	pub cluster_groups: Option<Vec<ClusterGroup>>,
	pub cluster_types: Option<Vec<ClusterType>>,
	pub clusters: Option<Vec<Cluster>>,
	pub created: Option<String>,
	pub data: Option<serde_json::Value>,
	pub data_file: Option<BriefDataFile>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_source: Option<BriefDataSource>,
	pub data_synced: Option<String>,
	pub description: Option<String>,
	pub device_types: Option<Vec<DeviceType>>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub is_active: Option<bool>,
	pub last_updated: Option<String>,
	pub locations: Option<Vec<Location>>,
	pub name: Option<String>,
	pub platforms: Option<Vec<Platform>>,
	pub profile: Option<BriefConfigContextProfile>,
	pub regions: Option<Vec<Region>>,
	pub roles: Option<Vec<DeviceRole>>,
	pub site_groups: Option<Vec<SiteGroup>>,
	pub sites: Option<Vec<Site>>,
	pub tags: Option<Vec<String>>,
	pub tenant_groups: Option<Vec<TenantGroup>>,
	pub tenants: Option<Vec<Tenant>>,
	pub url: Option<String>,
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigContextProfile {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub data_file: Option<BriefDataFile>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_source: Option<BriefDataSource>,
	pub data_synced: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	/// A JSON schema specifying the structure of the context data for this profile
	pub schema: Option<serde_json::Value>,
	pub tags: Option<Vec<String>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigContextProfileRequest {
	pub comments: String,
	pub data_source: serde_json::Value,
	pub description: String,
	pub name: String,
	/// A JSON schema specifying the structure of the context data for this profile
	pub schema: Option<serde_json::Value>,
	pub tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigContextRequest {
	pub cluster_groups: Vec<i64>,
	pub cluster_types: Vec<i64>,
	pub clusters: Vec<i64>,
	pub data: serde_json::Value,
	pub data_source: serde_json::Value,
	pub description: String,
	pub device_types: Vec<i64>,
	pub is_active: bool,
	pub locations: Vec<i64>,
	pub name: String,
	pub platforms: Vec<i64>,
	pub profile: Option<serde_json::Value>,
	pub regions: Vec<i64>,
	pub roles: Vec<i64>,
	pub site_groups: Vec<i64>,
	pub sites: Vec<i64>,
	pub tags: Vec<String>,
	pub tenant_groups: Vec<i64>,
	pub tenants: Vec<i64>,
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigTemplate {
	/// Download file as attachment
	pub as_attachment: Option<bool>,
	pub created: Option<String>,
	pub data_file: Option<BriefDataFile>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_source: Option<BriefDataSource>,
	pub data_synced: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Extension to append to the rendered filename
	pub file_extension: Option<String>,
	/// Filename to give to the rendered export file
	pub file_name: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: Option<String>,
	pub name: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	/// Jinja template code.
	pub template_code: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigTemplateRequest {
	/// Download file as attachment
	pub as_attachment: bool,
	pub data_source: serde_json::Value,
	pub description: String,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Extension to append to the rendered filename
	pub file_extension: String,
	/// Filename to give to the rendered export file
	pub file_name: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: String,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
	/// Jinja template code.
	pub template_code: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePort {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_reachable: Option<bool>,
	pub connected_endpoints_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	pub speed: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePortRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	pub speed: Option<i64>,
	pub tags: Vec<NestedTagRequest>,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePortTemplate {
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePortTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPort {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_reachable: Option<bool>,
	pub connected_endpoints_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	pub speed: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPortRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	pub speed: Option<i64>,
	pub tags: Vec<NestedTagRequest>,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPortTemplate {
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPortTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Contact {
	pub address: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub email: Option<String>,
	pub groups: Option<Vec<ContactGroup>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub link: Option<String>,
	pub name: Option<String>,
	pub phone: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub title: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactAssignment {
	pub contact: Option<BriefContact>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub display: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub object: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub object_id: Option<u64>,
	pub object_type: Option<String>,
	pub priority: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<BriefContactRole>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactAssignmentRequest {
	pub contact: serde_json::Value,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub object_id: u64,
	pub object_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: String,
	pub role: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactGroup {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub contact_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedContactGroup>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<NestedContactGroupRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactRequest {
	pub address: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub email: String,
	pub groups: Vec<i64>,
	pub link: String,
	pub name: String,
	pub phone: String,
	pub tags: Vec<NestedTagRequest>,
	pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactRole {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactRoleRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomField {
	pub choice_set: Option<BriefCustomFieldChoiceSet>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub data_type: Option<String>,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub filter_logic: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Custom fields within the same group will be displayed together
	pub group_name: Option<String>,
	pub id: i64,
	/// Replicate this value when cloning objects
	pub is_cloneable: Option<bool>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: Option<String>,
	pub last_updated: Option<String>,
	/// Internal field name
	pub name: Option<String>,
	pub object_types: Option<Vec<String>>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	pub related_object_type: Option<String>,
	/// This field is required when creating new objects or editing an existing object.
	pub required: Option<bool>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: Option<u16>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ui_editable: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ui_visible: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// The value of this field must be unique for the assigned object
	pub unique: Option<bool>,
	pub url: Option<String>,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<f64>,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<f64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: Option<String>,
	/// Fields with higher weights appear lower in a form.
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomFieldChoiceSet {
	pub base_choices: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub choices_count: Option<i64>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub extra_choices: Option<Vec<Vec<serde_json::Value>>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: Option<bool>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomFieldChoiceSetRequest {
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	pub base_choices: String,
	pub description: String,
	pub extra_choices: Vec<Vec<serde_json::Value>>,
	pub name: String,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomFieldRequest {
	pub choice_set: Option<serde_json::Value>,
	pub comments: String,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	pub description: String,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	pub filter_logic: String,
	/// Custom fields within the same group will be displayed together
	pub group_name: String,
	/// Replicate this value when cloning objects
	pub is_cloneable: bool,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: String,
	/// Internal field name
	pub name: String,
	pub object_types: Vec<String>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	pub related_object_type: Option<String>,
	/// This field is required when creating new objects or editing an existing object.
	pub required: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: u16,
	/// * `text` - Text
	/// * `longtext` - Text (long)
	/// * `integer` - Integer
	/// * `decimal` - Decimal
	/// * `boolean` - Boolean (true/false)
	/// * `date` - Date
	/// * `datetime` - Date & time
	/// * `url` - URL
	/// * `json` - JSON
	/// * `select` - Selection
	/// * `multiselect` - Multiple selection
	/// * `object` - Object
	/// * `multiobject` - Multiple objects
	pub r#type: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	pub ui_editable: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	pub ui_visible: String,
	/// The value of this field must be unique for the assigned object
	pub unique: bool,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<f64>,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<f64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: String,
	/// Fields with higher weights appear lower in a form.
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomLink {
	/// The class of the first link in a group will be used for the dropdown button
	/// 
	/// * `default` - Default
	/// * `blue` - Blue
	/// * `indigo` - Indigo
	/// * `purple` - Purple
	/// * `pink` - Pink
	/// * `red` - Red
	/// * `orange` - Orange
	/// * `yellow` - Yellow
	/// * `green` - Green
	/// * `teal` - Teal
	/// * `cyan` - Cyan
	/// * `gray` - Gray
	/// * `black` - Black
	/// * `white` - White
	/// * `ghost-dark` - Link
	pub button_class: Option<String>,
	pub created: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	/// Links with the same group will appear as a dropdown menu
	pub group_name: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// Jinja2 template code for link text
	pub link_text: Option<String>,
	/// Jinja2 template code for link URL
	pub link_url: Option<String>,
	pub name: Option<String>,
	/// Force link to open in a new window
	pub new_window: Option<bool>,
	pub object_types: Option<Vec<String>>,
	pub url: Option<String>,
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomLinkRequest {
	/// The class of the first link in a group will be used for the dropdown button
	/// 
	/// * `default` - Default
	/// * `blue` - Blue
	/// * `indigo` - Indigo
	/// * `purple` - Purple
	/// * `pink` - Pink
	/// * `red` - Red
	/// * `orange` - Orange
	/// * `yellow` - Yellow
	/// * `green` - Green
	/// * `teal` - Teal
	/// * `cyan` - Cyan
	/// * `gray` - Gray
	/// * `black` - Black
	/// * `white` - White
	/// * `ghost-dark` - Link
	pub button_class: String,
	pub enabled: bool,
	/// Links with the same group will appear as a dropdown menu
	pub group_name: String,
	/// Jinja2 template code for link text
	pub link_text: String,
	/// Jinja2 template code for link URL
	pub link_url: String,
	pub name: String,
	/// Force link to open in a new window
	pub new_window: bool,
	pub object_types: Vec<String>,
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Dashboard {
	pub config: Option<serde_json::Value>,
	pub layout: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DashboardRequest {
	pub config: serde_json::Value,
	pub layout: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataFile {
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// SHA256 hash of the file data
	pub hash: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// File path relative to the data source's root
	pub path: Option<String>,
	pub size: Option<i64>,
	pub source: Option<BriefDataSource>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataSource {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	pub file_count: Option<i64>,
	pub id: i64,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: Option<String>,
	pub last_synced: Option<String>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parameters: Option<serde_json::Value>,
	pub source_url: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataSourceRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub enabled: bool,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: String,
	pub name: String,
	pub parameters: Option<serde_json::Value>,
	pub source_url: String,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	/// * `None` - ---------
	/// * `local` - Local
	/// * `git` - Git
	/// * `amazon-s3` - Amazon S3
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Device {
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub cluster: Option<BriefCluster>,
	pub comments: Option<String>,
	pub config_template: Option<BriefConfigTemplate>,
	pub console_port_count: Option<i64>,
	pub console_server_port_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_bay_count: Option<i64>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub face: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub front_port_count: Option<i64>,
	pub id: i64,
	pub interface_count: Option<i64>,
	pub inventory_item_count: Option<i64>,
	pub last_updated: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub location: Option<BriefLocation>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub module_bay_count: Option<i64>,
	pub name: Option<String>,
	pub oob_ip: Option<BriefIPAddress>,
	pub parent_device: Option<NestedDevice>,
	pub platform: Option<BriefPlatform>,
	pub position: Option<f64>,
	pub power_outlet_count: Option<i64>,
	pub power_port_count: Option<i64>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub rack: Option<BriefRack>,
	pub rear_port_count: Option<i64>,
	pub role: Option<BriefDeviceRole>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: Option<String>,
	pub site: Option<BriefSite>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub virtual_chassis: Option<BriefVirtualChassis>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBay {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub installed_device: Option<BriefDevice>,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBayRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	pub installed_device: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBayTemplate {
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBayTemplateRequest {
	pub description: String,
	pub device_type: serde_json::Value,
	/// Physical label
	pub label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceRole {
	pub _depth: Option<i64>,
	pub color: Option<String>,
	pub comments: Option<String>,
	pub config_template: Option<BriefConfigTemplate>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedDeviceRole>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
	/// Virtual machines may be assigned to this role
	pub vm_role: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceRoleRequest {
	pub color: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<NestedDeviceRoleRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
	/// Virtual machines may be assigned to this role
	pub vm_role: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceType {
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub console_port_template_count: Option<i64>,
	pub console_server_port_template_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub default_platform: Option<BriefPlatform>,
	pub description: Option<String>,
	pub device_bay_template_count: Option<i64>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: Option<bool>,
	pub front_image: Option<String>,
	pub front_port_template_count: Option<i64>,
	pub id: i64,
	pub interface_template_count: Option<i64>,
	pub inventory_item_template_count: Option<i64>,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: Option<bool>,
	pub last_updated: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub module_bay_template_count: Option<i64>,
	/// Discrete part number (optional)
	pub part_number: Option<String>,
	pub power_outlet_template_count: Option<i64>,
	pub power_port_template_count: Option<i64>,
	pub rear_image: Option<String>,
	pub rear_port_template_count: Option<i64>,
	pub slug: Option<String>,
	pub subdevice_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub u_height: Option<f64>,
	pub url: Option<String>,
	pub weight: Option<f64>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `rear-to-side` - Rear to side
	/// * `bottom-to-top` - Bottom to top
	/// * `top-to-bottom` - Top to bottom
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	pub airflow: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub default_platform: Option<serde_json::Value>,
	pub description: String,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: bool,
	pub front_image: Option<String>,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: bool,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub rear_image: Option<String>,
	pub slug: String,
	/// * `parent` - Parent
	/// * `child` - Child
	pub subdevice_role: Option<String>,
	pub tags: Vec<NestedTagRequest>,
	pub u_height: f64,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceWithConfigContext {
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub cluster: Option<BriefCluster>,
	pub comments: Option<String>,
	pub config_context: Option<serde_json::Value>,
	pub config_template: Option<BriefConfigTemplate>,
	pub console_port_count: Option<i64>,
	pub console_server_port_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_bay_count: Option<i64>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub face: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub front_port_count: Option<i64>,
	pub id: i64,
	pub interface_count: Option<i64>,
	pub inventory_item_count: Option<i64>,
	pub last_updated: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub location: Option<BriefLocation>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub module_bay_count: Option<i64>,
	pub name: Option<String>,
	pub oob_ip: Option<BriefIPAddress>,
	pub parent_device: Option<NestedDevice>,
	pub platform: Option<BriefPlatform>,
	pub position: Option<f64>,
	pub power_outlet_count: Option<i64>,
	pub power_port_count: Option<i64>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub rack: Option<BriefRack>,
	pub rear_port_count: Option<i64>,
	pub role: Option<BriefDeviceRole>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: Option<String>,
	pub site: Option<BriefSite>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub virtual_chassis: Option<BriefVirtualChassis>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceWithConfigContextRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `rear-to-side` - Rear to side
	/// * `bottom-to-top` - Bottom to top
	/// * `top-to-bottom` - Top to bottom
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	pub airflow: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub cluster: Option<serde_json::Value>,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device_type: serde_json::Value,
	/// * `front` - Front
	/// * `rear` - Rear
	pub face: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub location: Option<serde_json::Value>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub name: Option<String>,
	pub oob_ip: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	pub position: Option<f64>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub rack: Option<serde_json::Value>,
	pub role: serde_json::Value,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: String,
	pub site: serde_json::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub virtual_chassis: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EventRule {
	pub action_object: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub action_object_id: Option<u64>,
	pub action_object_type: Option<String>,
	pub action_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	/// The types of event which will trigger this rule.
	pub event_types: Option<Vec<String>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EventRuleRequest {
	pub action_object_id: Option<u64>,
	pub action_object_type: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	/// * `notification` - Notification
	pub action_type: String,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub enabled: bool,
	/// The types of event which will trigger this rule.
	pub event_types: Vec<String>,
	pub name: String,
	pub object_types: Vec<String>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExportTemplate {
	/// Download file as attachment
	pub as_attachment: Option<bool>,
	pub created: Option<String>,
	pub data_file: Option<BriefDataFile>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_source: Option<BriefDataSource>,
	pub data_synced: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Extension to append to the rendered filename
	pub file_extension: Option<String>,
	/// Filename to give to the rendered export file
	pub file_name: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: Option<String>,
	pub name: Option<String>,
	pub object_types: Option<Vec<String>>,
	/// Jinja template code.
	pub template_code: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExportTemplateRequest {
	/// Download file as attachment
	pub as_attachment: bool,
	pub data_source: serde_json::Value,
	pub description: String,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Extension to append to the rendered filename
	pub file_extension: String,
	/// Filename to give to the rendered export file
	pub file_name: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: String,
	pub name: String,
	pub object_types: Vec<String>,
	/// Jinja template code.
	pub template_code: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroup {
	pub auth_key: Option<String>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	pub auth_type: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub group_id: Option<u16>,
	pub id: i64,
	pub ip_addresses: Option<Vec<BriefIPAddress>>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroupAssignment {
	pub created: Option<String>,
	pub display: Option<String>,
	pub group: Option<BriefFHRPGroup>,
	pub id: i64,
	pub interface: Option<serde_json::Value>,
	pub interface_id: Option<u64>,
	pub interface_type: Option<String>,
	pub last_updated: Option<String>,
	pub priority: Option<u8>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub interface_id: u64,
	pub interface_type: String,
	pub priority: u8,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroupRequest {
	pub auth_key: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	pub auth_type: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group_id: u16,
	pub name: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPort {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub color: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	pub rear_port: Option<FrontPortRearPort>,
	/// Mapped position on corresponding rear port
	pub rear_port_position: Option<u16>,
	pub tags: Option<Vec<NestedTag>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortRearPort {
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortRearPortRequest {
	pub description: String,
	/// Physical label
	pub label: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub rear_port: FrontPortRearPortRequest,
	/// Mapped position on corresponding rear port
	pub rear_port_position: u16,
	pub tags: Vec<NestedTagRequest>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortTemplate {
	pub color: Option<String>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub rear_port: Option<BriefRearPortTemplate>,
	pub rear_port_position: Option<u16>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortTemplateRequest {
	pub color: String,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub rear_port: serde_json::Value,
	pub rear_port_position: u16,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenericObject {
	pub object: Option<serde_json::Value>,
	pub object_id: Option<i64>,
	pub object_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenericObjectRequest {
	pub object_id: i64,
	pub object_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Group {
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub permissions: Option<Vec<ObjectPermission>>,
	pub url: Option<String>,
	pub user_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GroupRequest {
	pub description: String,
	pub name: String,
	pub permissions: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEPolicy {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub name: Option<String>,
	pub preshared_key: Option<String>,
	pub proposals: Option<Vec<IKEProposal>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub version: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEPolicyRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	pub mode: String,
	pub name: String,
	pub preshared_key: String,
	pub proposals: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	pub version: i64,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEProposal {
	pub authentication_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub authentication_method: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub encryption_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub group: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	pub authentication_method: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: String,
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
	/// * `15` - Group 15
	/// * `16` - Group 16
	/// * `17` - Group 17
	/// * `18` - Group 18
	/// * `19` - Group 19
	/// * `20` - Group 20
	/// * `21` - Group 21
	/// * `22` - Group 22
	/// * `23` - Group 23
	/// * `24` - Group 24
	/// * `25` - Group 25
	/// * `26` - Group 26
	/// * `27` - Group 27
	/// * `28` - Group 28
	/// * `29` - Group 29
	/// * `30` - Group 30
	/// * `31` - Group 31
	/// * `32` - Group 32
	/// * `33` - Group 33
	/// * `34` - Group 34
	pub group: i64,
	pub name: String,
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPAddress {
	pub address: Option<String>,
	pub assigned_object: Option<serde_json::Value>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub nat_inside: Option<NestedIPAddress>,
	pub nat_outside: Option<Vec<NestedIPAddress>>,
	pub role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPAddressRequest {
	pub address: String,
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: String,
	pub nat_inside: Option<NestedIPAddressRequest>,
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	pub role: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPRange {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub end_address: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: Option<bool>,
	/// Report space as fully utilized
	pub mark_utilized: Option<bool>,
	pub role: Option<BriefRole>,
	pub size: Option<i64>,
	pub start_address: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPRangeRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub end_address: String,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: bool,
	/// Report space as fully utilized
	pub mark_utilized: bool,
	pub role: Option<serde_json::Value>,
	pub start_address: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecPolicy {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub pfs_group: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub proposals: Option<Vec<IPSecProposal>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecPolicyRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
	/// * `15` - Group 15
	/// * `16` - Group 16
	/// * `17` - Group 17
	/// * `18` - Group 18
	/// * `19` - Group 19
	/// * `20` - Group 20
	/// * `21` - Group 21
	/// * `22` - Group 22
	/// * `23` - Group 23
	/// * `24` - Group 24
	/// * `25` - Group 25
	/// * `26` - Group 26
	/// * `27` - Group 27
	/// * `28` - Group 28
	/// * `29` - Group 29
	/// * `30` - Group 30
	/// * `31` - Group 31
	/// * `32` - Group 32
	/// * `33` - Group 33
	/// * `34` - Group 34
	pub pfs_group: i64,
	pub proposals: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProfile {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub ike_policy: Option<BriefIKEPolicy>,
	pub ipsec_policy: Option<BriefIPSecPolicy>,
	pub last_updated: Option<String>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub name: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProfileRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub ike_policy: serde_json::Value,
	pub ipsec_policy: serde_json::Value,
	/// * `esp` - ESP
	/// * `ah` - AH
	pub mode: String,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProposal {
	pub authentication_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub encryption_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: String,
	pub name: String,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ImageAttachment {
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub image: Option<String>,
	pub image_height: Option<i64>,
	pub image_width: Option<i64>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub object_id: Option<u64>,
	pub object_type: Option<String>,
	pub parent: Option<serde_json::Value>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ImageAttachmentRequest {
	pub description: String,
	pub image: String,
	pub name: String,
	pub object_id: u64,
	pub object_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IntegerRange(pub i64);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IntegerRangeRequest(pub i64);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Interface {
	pub _occupied: Option<bool>,
	pub bridge: Option<NestedInterface>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_reachable: Option<bool>,
	pub connected_endpoints_type: Option<String>,
	pub count_fhrp_groups: Option<i64>,
	pub count_ipaddresses: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub duplex: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub enabled: Option<bool>,
	pub id: i64,
	pub l2vpn_termination: Option<BriefL2VPNTermination>,
	/// Physical label
	pub label: Option<String>,
	pub lag: Option<NestedInterface>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub mac_address: Option<String>,
	pub mac_addresses: Option<Vec<BriefMACAddress>>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	/// This interface is used only for out-of-band management
	pub mgmt_only: Option<bool>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub module: Option<BriefModule>,
	pub mtu: Option<u32>,
	pub name: Option<String>,
	pub parent: Option<NestedInterface>,
	pub poe_mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub poe_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub primary_mac_address: Option<BriefMACAddress>,
	pub qinq_svlan: Option<BriefVLAN>,
	pub rf_channel: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	pub rf_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub speed: Option<u32>,
	pub tagged_vlans: Option<Vec<VLAN>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tx_power: Option<i8>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub untagged_vlan: Option<BriefVLAN>,
	pub url: Option<String>,
	pub vdcs: Option<Vec<VirtualDeviceContext>>,
	pub vlan_translation_policy: Option<BriefVLANTranslationPolicy>,
	pub vrf: Option<BriefVRF>,
	pub wireless_lans: Option<Vec<WirelessLAN>>,
	pub wireless_link: Option<NestedWirelessLink>,
	pub wwn: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InterfaceRequest {
	pub bridge: Option<NestedInterfaceRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	pub duplex: Option<String>,
	pub enabled: bool,
	/// Physical label
	pub label: String,
	pub lag: Option<NestedInterfaceRequest>,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// This interface is used only for out-of-band management
	pub mgmt_only: bool,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: String,
	pub module: Option<serde_json::Value>,
	pub mtu: Option<u32>,
	pub name: String,
	pub parent: Option<NestedInterfaceRequest>,
	/// * `pd` - PD
	/// * `pse` - PSE
	pub poe_mode: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	pub poe_type: String,
	pub primary_mac_address: Option<serde_json::Value>,
	pub qinq_svlan: Option<serde_json::Value>,
	/// * `2.4g-1-2412-22` - 1 (2412 MHz)
	/// * `2.4g-2-2417-22` - 2 (2417 MHz)
	/// * `2.4g-3-2422-22` - 3 (2422 MHz)
	/// * `2.4g-4-2427-22` - 4 (2427 MHz)
	/// * `2.4g-5-2432-22` - 5 (2432 MHz)
	/// * `2.4g-6-2437-22` - 6 (2437 MHz)
	/// * `2.4g-7-2442-22` - 7 (2442 MHz)
	/// * `2.4g-8-2447-22` - 8 (2447 MHz)
	/// * `2.4g-9-2452-22` - 9 (2452 MHz)
	/// * `2.4g-10-2457-22` - 10 (2457 MHz)
	/// * `2.4g-11-2462-22` - 11 (2462 MHz)
	/// * `2.4g-12-2467-22` - 12 (2467 MHz)
	/// * `2.4g-13-2472-22` - 13 (2472 MHz)
	/// * `5g-32-5160-20` - 32 (5160/20 MHz)
	/// * `5g-34-5170-40` - 34 (5170/40 MHz)
	/// * `5g-36-5180-20` - 36 (5180/20 MHz)
	/// * `5g-38-5190-40` - 38 (5190/40 MHz)
	/// * `5g-40-5200-20` - 40 (5200/20 MHz)
	/// * `5g-42-5210-80` - 42 (5210/80 MHz)
	/// * `5g-44-5220-20` - 44 (5220/20 MHz)
	/// * `5g-46-5230-40` - 46 (5230/40 MHz)
	/// * `5g-48-5240-20` - 48 (5240/20 MHz)
	/// * `5g-50-5250-160` - 50 (5250/160 MHz)
	/// * `5g-52-5260-20` - 52 (5260/20 MHz)
	/// * `5g-54-5270-40` - 54 (5270/40 MHz)
	/// * `5g-56-5280-20` - 56 (5280/20 MHz)
	/// * `5g-58-5290-80` - 58 (5290/80 MHz)
	/// * `5g-60-5300-20` - 60 (5300/20 MHz)
	/// * `5g-62-5310-40` - 62 (5310/40 MHz)
	/// * `5g-64-5320-20` - 64 (5320/20 MHz)
	/// * `5g-100-5500-20` - 100 (5500/20 MHz)
	/// * `5g-102-5510-40` - 102 (5510/40 MHz)
	/// * `5g-104-5520-20` - 104 (5520/20 MHz)
	/// * `5g-106-5530-80` - 106 (5530/80 MHz)
	/// * `5g-108-5540-20` - 108 (5540/20 MHz)
	/// * `5g-110-5550-40` - 110 (5550/40 MHz)
	/// * `5g-112-5560-20` - 112 (5560/20 MHz)
	/// * `5g-114-5570-160` - 114 (5570/160 MHz)
	/// * `5g-116-5580-20` - 116 (5580/20 MHz)
	/// * `5g-118-5590-40` - 118 (5590/40 MHz)
	/// * `5g-120-5600-20` - 120 (5600/20 MHz)
	/// * `5g-122-5610-80` - 122 (5610/80 MHz)
	/// * `5g-124-5620-20` - 124 (5620/20 MHz)
	/// * `5g-126-5630-40` - 126 (5630/40 MHz)
	/// * `5g-128-5640-20` - 128 (5640/20 MHz)
	/// * `5g-132-5660-20` - 132 (5660/20 MHz)
	/// * `5g-134-5670-40` - 134 (5670/40 MHz)
	/// * `5g-136-5680-20` - 136 (5680/20 MHz)
	/// * `5g-138-5690-80` - 138 (5690/80 MHz)
	/// * `5g-140-5700-20` - 140 (5700/20 MHz)
	/// * `5g-142-5710-40` - 142 (5710/40 MHz)
	/// * `5g-144-5720-20` - 144 (5720/20 MHz)
	/// * `5g-149-5745-20` - 149 (5745/20 MHz)
	/// * `5g-151-5755-40` - 151 (5755/40 MHz)
	/// * `5g-153-5765-20` - 153 (5765/20 MHz)
	/// * `5g-155-5775-80` - 155 (5775/80 MHz)
	/// * `5g-157-5785-20` - 157 (5785/20 MHz)
	/// * `5g-159-5795-40` - 159 (5795/40 MHz)
	/// * `5g-161-5805-20` - 161 (5805/20 MHz)
	/// * `5g-163-5815-160` - 163 (5815/160 MHz)
	/// * `5g-165-5825-20` - 165 (5825/20 MHz)
	/// * `5g-167-5835-40` - 167 (5835/40 MHz)
	/// * `5g-169-5845-20` - 169 (5845/20 MHz)
	/// * `5g-171-5855-80` - 171 (5855/80 MHz)
	/// * `5g-173-5865-20` - 173 (5865/20 MHz)
	/// * `5g-175-5875-40` - 175 (5875/40 MHz)
	/// * `5g-177-5885-20` - 177 (5885/20 MHz)
	/// * `6g-1-5955-20` - 1 (5955/20 MHz)
	/// * `6g-3-5965-40` - 3 (5965/40 MHz)
	/// * `6g-5-5975-20` - 5 (5975/20 MHz)
	/// * `6g-7-5985-80` - 7 (5985/80 MHz)
	/// * `6g-9-5995-20` - 9 (5995/20 MHz)
	/// * `6g-11-6005-40` - 11 (6005/40 MHz)
	/// * `6g-13-6015-20` - 13 (6015/20 MHz)
	/// * `6g-15-6025-160` - 15 (6025/160 MHz)
	/// * `6g-17-6035-20` - 17 (6035/20 MHz)
	/// * `6g-19-6045-40` - 19 (6045/40 MHz)
	/// * `6g-21-6055-20` - 21 (6055/20 MHz)
	/// * `6g-23-6065-80` - 23 (6065/80 MHz)
	/// * `6g-25-6075-20` - 25 (6075/20 MHz)
	/// * `6g-27-6085-40` - 27 (6085/40 MHz)
	/// * `6g-29-6095-20` - 29 (6095/20 MHz)
	/// * `6g-31-6105-320` - 31 (6105/320 MHz)
	/// * `6g-33-6115-20` - 33 (6115/20 MHz)
	/// * `6g-35-6125-40` - 35 (6125/40 MHz)
	/// * `6g-37-6135-20` - 37 (6135/20 MHz)
	/// * `6g-39-6145-80` - 39 (6145/80 MHz)
	/// * `6g-41-6155-20` - 41 (6155/20 MHz)
	/// * `6g-43-6165-40` - 43 (6165/40 MHz)
	/// * `6g-45-6175-20` - 45 (6175/20 MHz)
	/// * `6g-47-6185-160` - 47 (6185/160 MHz)
	/// * `6g-49-6195-20` - 49 (6195/20 MHz)
	/// * `6g-51-6205-40` - 51 (6205/40 MHz)
	/// * `6g-53-6215-20` - 53 (6215/20 MHz)
	/// * `6g-55-6225-80` - 55 (6225/80 MHz)
	/// * `6g-57-6235-20` - 57 (6235/20 MHz)
	/// * `6g-59-6245-40` - 59 (6245/40 MHz)
	/// * `6g-61-6255-20` - 61 (6255/20 MHz)
	/// * `6g-65-6275-20` - 65 (6275/20 MHz)
	/// * `6g-67-6285-40` - 67 (6285/40 MHz)
	/// * `6g-69-6295-20` - 69 (6295/20 MHz)
	/// * `6g-71-6305-80` - 71 (6305/80 MHz)
	/// * `6g-73-6315-20` - 73 (6315/20 MHz)
	/// * `6g-75-6325-40` - 75 (6325/40 MHz)
	/// * `6g-77-6335-20` - 77 (6335/20 MHz)
	/// * `6g-79-6345-160` - 79 (6345/160 MHz)
	/// * `6g-81-6355-20` - 81 (6355/20 MHz)
	/// * `6g-83-6365-40` - 83 (6365/40 MHz)
	/// * `6g-85-6375-20` - 85 (6375/20 MHz)
	/// * `6g-87-6385-80` - 87 (6385/80 MHz)
	/// * `6g-89-6395-20` - 89 (6395/20 MHz)
	/// * `6g-91-6405-40` - 91 (6405/40 MHz)
	/// * `6g-93-6415-20` - 93 (6415/20 MHz)
	/// * `6g-95-6425-320` - 95 (6425/320 MHz)
	/// * `6g-97-6435-20` - 97 (6435/20 MHz)
	/// * `6g-99-6445-40` - 99 (6445/40 MHz)
	/// * `6g-101-6455-20` - 101 (6455/20 MHz)
	/// * `6g-103-6465-80` - 103 (6465/80 MHz)
	/// * `6g-105-6475-20` - 105 (6475/20 MHz)
	/// * `6g-107-6485-40` - 107 (6485/40 MHz)
	/// * `6g-109-6495-20` - 109 (6495/20 MHz)
	/// * `6g-111-6505-160` - 111 (6505/160 MHz)
	/// * `6g-113-6515-20` - 113 (6515/20 MHz)
	/// * `6g-115-6525-40` - 115 (6525/40 MHz)
	/// * `6g-117-6535-20` - 117 (6535/20 MHz)
	/// * `6g-119-6545-80` - 119 (6545/80 MHz)
	/// * `6g-121-6555-20` - 121 (6555/20 MHz)
	/// * `6g-123-6565-40` - 123 (6565/40 MHz)
	/// * `6g-125-6575-20` - 125 (6575/20 MHz)
	/// * `6g-129-6595-20` - 129 (6595/20 MHz)
	/// * `6g-131-6605-40` - 131 (6605/40 MHz)
	/// * `6g-133-6615-20` - 133 (6615/20 MHz)
	/// * `6g-135-6625-80` - 135 (6625/80 MHz)
	/// * `6g-137-6635-20` - 137 (6635/20 MHz)
	/// * `6g-139-6645-40` - 139 (6645/40 MHz)
	/// * `6g-141-6655-20` - 141 (6655/20 MHz)
	/// * `6g-143-6665-160` - 143 (6665/160 MHz)
	/// * `6g-145-6675-20` - 145 (6675/20 MHz)
	/// * `6g-147-6685-40` - 147 (6685/40 MHz)
	/// * `6g-149-6695-20` - 149 (6695/20 MHz)
	/// * `6g-151-6705-80` - 151 (6705/80 MHz)
	/// * `6g-153-6715-20` - 153 (6715/20 MHz)
	/// * `6g-155-6725-40` - 155 (6725/40 MHz)
	/// * `6g-157-6735-20` - 157 (6735/20 MHz)
	/// * `6g-159-6745-320` - 159 (6745/320 MHz)
	/// * `6g-161-6755-20` - 161 (6755/20 MHz)
	/// * `6g-163-6765-40` - 163 (6765/40 MHz)
	/// * `6g-165-6775-20` - 165 (6775/20 MHz)
	/// * `6g-167-6785-80` - 167 (6785/80 MHz)
	/// * `6g-169-6795-20` - 169 (6795/20 MHz)
	/// * `6g-171-6805-40` - 171 (6805/40 MHz)
	/// * `6g-173-6815-20` - 173 (6815/20 MHz)
	/// * `6g-175-6825-160` - 175 (6825/160 MHz)
	/// * `6g-177-6835-20` - 177 (6835/20 MHz)
	/// * `6g-179-6845-40` - 179 (6845/40 MHz)
	/// * `6g-181-6855-20` - 181 (6855/20 MHz)
	/// * `6g-183-6865-80` - 183 (6865/80 MHz)
	/// * `6g-185-6875-20` - 185 (6875/20 MHz)
	/// * `6g-187-6885-40` - 187 (6885/40 MHz)
	/// * `6g-189-6895-20` - 189 (6895/20 MHz)
	/// * `6g-193-6915-20` - 193 (6915/20 MHz)
	/// * `6g-195-6925-40` - 195 (6925/40 MHz)
	/// * `6g-197-6935-20` - 197 (6935/20 MHz)
	/// * `6g-199-6945-80` - 199 (6945/80 MHz)
	/// * `6g-201-6955-20` - 201 (6955/20 MHz)
	/// * `6g-203-6965-40` - 203 (6965/40 MHz)
	/// * `6g-205-6975-20` - 205 (6975/20 MHz)
	/// * `6g-207-6985-160` - 207 (6985/160 MHz)
	/// * `6g-209-6995-20` - 209 (6995/20 MHz)
	/// * `6g-211-7005-40` - 211 (7005/40 MHz)
	/// * `6g-213-7015-20` - 213 (7015/20 MHz)
	/// * `6g-215-7025-80` - 215 (7025/80 MHz)
	/// * `6g-217-7035-20` - 217 (7035/20 MHz)
	/// * `6g-219-7045-40` - 219 (7045/40 MHz)
	/// * `6g-221-7055-20` - 221 (7055/20 MHz)
	/// * `6g-225-7075-20` - 225 (7075/20 MHz)
	/// * `6g-227-7085-40` - 227 (7085/40 MHz)
	/// * `6g-229-7095-20` - 229 (7095/20 MHz)
	/// * `6g-233-7115-20` - 233 (7115/20 MHz)
	/// * `60g-1-58320-2160` - 1 (58.32/2.16 GHz)
	/// * `60g-2-60480-2160` - 2 (60.48/2.16 GHz)
	/// * `60g-3-62640-2160` - 3 (62.64/2.16 GHz)
	/// * `60g-4-64800-2160` - 4 (64.80/2.16 GHz)
	/// * `60g-5-66960-2160` - 5 (66.96/2.16 GHz)
	/// * `60g-6-69120-2160` - 6 (69.12/2.16 GHz)
	/// * `60g-9-59400-4320` - 9 (59.40/4.32 GHz)
	/// * `60g-10-61560-4320` - 10 (61.56/4.32 GHz)
	/// * `60g-11-63720-4320` - 11 (63.72/4.32 GHz)
	/// * `60g-12-65880-4320` - 12 (65.88/4.32 GHz)
	/// * `60g-13-68040-4320` - 13 (68.04/4.32 GHz)
	/// * `60g-17-60480-6480` - 17 (60.48/6.48 GHz)
	/// * `60g-18-62640-6480` - 18 (62.64/6.48 GHz)
	/// * `60g-19-64800-6480` - 19 (64.80/6.48 GHz)
	/// * `60g-20-66960-6480` - 20 (66.96/6.48 GHz)
	/// * `60g-25-61560-6480` - 25 (61.56/8.64 GHz)
	/// * `60g-26-63720-6480` - 26 (63.72/8.64 GHz)
	/// * `60g-27-65880-6480` - 27 (65.88/8.64 GHz)
	pub rf_channel: String,
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: String,
	pub speed: Option<u32>,
	pub tagged_vlans: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub tx_power: Option<i8>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME)
	/// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
	/// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
	/// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
	/// * `1000base-cx` - 1000BASE-CX (1GE DAC)
	/// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
	/// * `1000base-ex` - 1000BASE-EX (1GE)
	/// * `1000base-lsx` - 1000BASE-LSX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `1000base-zx` - 1000BASE-ZX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
	/// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
	/// * `10gbase-er` - 10GBASE-ER (10GE)
	/// * `10gbase-lr` - 10GBASE-LR (10GE)
	/// * `10gbase-lrm` - 10GBASE-LRM (10GE)
	/// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
	/// * `10gbase-sr` - 10GBASE-SR (10GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-zr` - 10GBASE-ZR (10GE)
	/// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
	/// * `25gbase-er` - 25GBASE-ER (25GE)
	/// * `25gbase-lr` - 25GBASE-LR (25GE)
	/// * `25gbase-sr` - 25GBASE-SR (25GE)
	/// * `25gbase-t` - 25GBASE-T (25GE)
	/// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
	/// * `40gbase-er4` - 40GBASE-ER4 (40GE)
	/// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
	/// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
	/// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
	/// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
	/// * `50gbase-er` - 50GBASE-ER (50GE)
	/// * `50gbase-fr` - 50GBASE-FR (50GE)
	/// * `50gbase-lr` - 50GBASE-LR (50GE)
	/// * `50gbase-sr` - 50GBASE-SR (50GE)
	/// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
	/// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
	/// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
	/// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
	/// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
	/// * `100gbase-dr` - 100GBASE-DR (100GE)
	/// * `100gbase-er4` - 100GBASE-ER4 (100GE)
	/// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
	/// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
	/// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
	/// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
	/// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
	/// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
	/// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
	/// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
	/// * `100gbase-zr` - 100GBASE-ZR (100GE)
	/// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
	/// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
	/// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
	/// * `200gbase-er4` - 200GBASE-ER4 (200GE)
	/// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
	/// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
	/// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
	/// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
	/// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
	/// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
	/// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
	/// * `400gbase-er8` - 400GBASE-ER8 (400GE)
	/// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
	/// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
	/// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
	/// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
	/// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
	/// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
	/// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
	/// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
	/// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
	/// * `400gbase-zr` - 400GBASE-ZR (400GE)
	/// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
	/// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
	/// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
	/// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `1000base-kx` - 1000BASE-KX (1GE)
	/// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
	/// * `5gbase-kr` - 5GBASE-KR (5GE)
	/// * `10gbase-kr` - 10GBASE-KR (10GE)
	/// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
	/// * `25gbase-kr` - 25GBASE-KR (25GE)
	/// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
	/// * `50gbase-kr` - 50GBASE-KR (50GE)
	/// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
	/// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
	/// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
	/// * `ieee802.11a` - IEEE 802.11a
	/// * `ieee802.11g` - IEEE 802.11b/g
	/// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
	/// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
	/// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
	/// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
	/// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
	/// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
	/// * `4g` - 4G
	/// * `5g` - 5G
	/// * `sonet-oc3` - OC-3/STM-1
	/// * `sonet-oc12` - OC-12/STM-4
	/// * `sonet-oc48` - OC-48/STM-16
	/// * `sonet-oc192` - OC-192/STM-64
	/// * `sonet-oc768` - OC-768/STM-256
	/// * `sonet-oc1920` - OC-1920/STM-640
	/// * `sonet-oc3840` - OC-3840/STM-1234
	/// * `1gfc-sfp` - SFP (1GFC)
	/// * `2gfc-sfp` - SFP (2GFC)
	/// * `4gfc-sfp` - SFP (4GFC)
	/// * `8gfc-sfpp` - SFP+ (8GFC)
	/// * `16gfc-sfpp` - SFP+ (16GFC)
	/// * `32gfc-sfp28` - SFP28 (32GFC)
	/// * `32gfc-sfpp` - SFP+ (32GFC)
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
	/// * `64gfc-sfpdd` - SFP-DD (64GFC)
	/// * `64gfc-sfpp` - SFP+ (64GFC)
	/// * `128gfc-qsfp28` - QSFP28 (128GFC)
	/// * `infiniband-sdr` - SDR (2 Gbps)
	/// * `infiniband-ddr` - DDR (4 Gbps)
	/// * `infiniband-qdr` - QDR (8 Gbps)
	/// * `infiniband-fdr10` - FDR10 (10 Gbps)
	/// * `infiniband-fdr` - FDR (13.5 Gbps)
	/// * `infiniband-edr` - EDR (25 Gbps)
	/// * `infiniband-hdr` - HDR (50 Gbps)
	/// * `infiniband-ndr` - NDR (100 Gbps)
	/// * `infiniband-xdr` - XDR (250 Gbps)
	/// * `t1` - T1 (1.544 Mbps)
	/// * `e1` - E1 (2.048 Mbps)
	/// * `t3` - T3 (45 Mbps)
	/// * `e3` - E3 (34 Mbps)
	/// * `xdsl` - xDSL
	/// * `docsis` - DOCSIS
	/// * `moca` - MoCA
	/// * `bpon` - BPON (622 Mbps / 155 Mbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `25g-pon` - 25G-PON (25 Gbps)
	/// * `50g-pon` - 50G-PON (50 Gbps)
	/// * `cisco-stackwise` - Cisco StackWise
	/// * `cisco-stackwise-plus` - Cisco StackWise Plus
	/// * `cisco-flexstack` - Cisco FlexStack
	/// * `cisco-flexstack-plus` - Cisco FlexStack Plus
	/// * `cisco-stackwise-80` - Cisco StackWise-80
	/// * `cisco-stackwise-160` - Cisco StackWise-160
	/// * `cisco-stackwise-320` - Cisco StackWise-320
	/// * `cisco-stackwise-480` - Cisco StackWise-480
	/// * `cisco-stackwise-1t` - Cisco StackWise-1T
	/// * `juniper-vcp` - Juniper VCP
	/// * `extreme-summitstack` - Extreme SummitStack
	/// * `extreme-summitstack-128` - Extreme SummitStack-128
	/// * `extreme-summitstack-256` - Extreme SummitStack-256
	/// * `extreme-summitstack-512` - Extreme SummitStack-512
	/// * `other` - Other
	pub r#type: String,
	pub untagged_vlan: Option<serde_json::Value>,
	pub vdcs: Vec<i64>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
	pub wireless_lans: Vec<i64>,
	pub wwn: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InterfaceTemplate {
	pub bridge: Option<NestedInterfaceTemplate>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub enabled: Option<bool>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub mgmt_only: Option<bool>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub poe_mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub poe_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub rf_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InterfaceTemplateRequest {
	pub bridge: Option<NestedInterfaceTemplateRequest>,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	pub enabled: bool,
	/// Physical label
	pub label: String,
	pub mgmt_only: bool,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	pub poe_mode: Option<String>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	pub poe_type: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: Option<String>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME)
	/// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
	/// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
	/// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
	/// * `1000base-cx` - 1000BASE-CX (1GE DAC)
	/// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
	/// * `1000base-ex` - 1000BASE-EX (1GE)
	/// * `1000base-lsx` - 1000BASE-LSX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `1000base-zx` - 1000BASE-ZX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
	/// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
	/// * `10gbase-er` - 10GBASE-ER (10GE)
	/// * `10gbase-lr` - 10GBASE-LR (10GE)
	/// * `10gbase-lrm` - 10GBASE-LRM (10GE)
	/// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
	/// * `10gbase-sr` - 10GBASE-SR (10GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-zr` - 10GBASE-ZR (10GE)
	/// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
	/// * `25gbase-er` - 25GBASE-ER (25GE)
	/// * `25gbase-lr` - 25GBASE-LR (25GE)
	/// * `25gbase-sr` - 25GBASE-SR (25GE)
	/// * `25gbase-t` - 25GBASE-T (25GE)
	/// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
	/// * `40gbase-er4` - 40GBASE-ER4 (40GE)
	/// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
	/// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
	/// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
	/// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
	/// * `50gbase-er` - 50GBASE-ER (50GE)
	/// * `50gbase-fr` - 50GBASE-FR (50GE)
	/// * `50gbase-lr` - 50GBASE-LR (50GE)
	/// * `50gbase-sr` - 50GBASE-SR (50GE)
	/// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
	/// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
	/// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
	/// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
	/// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
	/// * `100gbase-dr` - 100GBASE-DR (100GE)
	/// * `100gbase-er4` - 100GBASE-ER4 (100GE)
	/// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
	/// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
	/// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
	/// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
	/// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
	/// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
	/// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
	/// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
	/// * `100gbase-zr` - 100GBASE-ZR (100GE)
	/// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
	/// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
	/// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
	/// * `200gbase-er4` - 200GBASE-ER4 (200GE)
	/// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
	/// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
	/// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
	/// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
	/// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
	/// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
	/// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
	/// * `400gbase-er8` - 400GBASE-ER8 (400GE)
	/// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
	/// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
	/// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
	/// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
	/// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
	/// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
	/// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
	/// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
	/// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
	/// * `400gbase-zr` - 400GBASE-ZR (400GE)
	/// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
	/// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
	/// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
	/// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `1000base-kx` - 1000BASE-KX (1GE)
	/// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
	/// * `5gbase-kr` - 5GBASE-KR (5GE)
	/// * `10gbase-kr` - 10GBASE-KR (10GE)
	/// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
	/// * `25gbase-kr` - 25GBASE-KR (25GE)
	/// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
	/// * `50gbase-kr` - 50GBASE-KR (50GE)
	/// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
	/// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
	/// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
	/// * `ieee802.11a` - IEEE 802.11a
	/// * `ieee802.11g` - IEEE 802.11b/g
	/// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
	/// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
	/// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
	/// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
	/// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
	/// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
	/// * `4g` - 4G
	/// * `5g` - 5G
	/// * `sonet-oc3` - OC-3/STM-1
	/// * `sonet-oc12` - OC-12/STM-4
	/// * `sonet-oc48` - OC-48/STM-16
	/// * `sonet-oc192` - OC-192/STM-64
	/// * `sonet-oc768` - OC-768/STM-256
	/// * `sonet-oc1920` - OC-1920/STM-640
	/// * `sonet-oc3840` - OC-3840/STM-1234
	/// * `1gfc-sfp` - SFP (1GFC)
	/// * `2gfc-sfp` - SFP (2GFC)
	/// * `4gfc-sfp` - SFP (4GFC)
	/// * `8gfc-sfpp` - SFP+ (8GFC)
	/// * `16gfc-sfpp` - SFP+ (16GFC)
	/// * `32gfc-sfp28` - SFP28 (32GFC)
	/// * `32gfc-sfpp` - SFP+ (32GFC)
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
	/// * `64gfc-sfpdd` - SFP-DD (64GFC)
	/// * `64gfc-sfpp` - SFP+ (64GFC)
	/// * `128gfc-qsfp28` - QSFP28 (128GFC)
	/// * `infiniband-sdr` - SDR (2 Gbps)
	/// * `infiniband-ddr` - DDR (4 Gbps)
	/// * `infiniband-qdr` - QDR (8 Gbps)
	/// * `infiniband-fdr10` - FDR10 (10 Gbps)
	/// * `infiniband-fdr` - FDR (13.5 Gbps)
	/// * `infiniband-edr` - EDR (25 Gbps)
	/// * `infiniband-hdr` - HDR (50 Gbps)
	/// * `infiniband-ndr` - NDR (100 Gbps)
	/// * `infiniband-xdr` - XDR (250 Gbps)
	/// * `t1` - T1 (1.544 Mbps)
	/// * `e1` - E1 (2.048 Mbps)
	/// * `t3` - T3 (45 Mbps)
	/// * `e3` - E3 (34 Mbps)
	/// * `xdsl` - xDSL
	/// * `docsis` - DOCSIS
	/// * `moca` - MoCA
	/// * `bpon` - BPON (622 Mbps / 155 Mbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `25g-pon` - 25G-PON (25 Gbps)
	/// * `50g-pon` - 50G-PON (50 Gbps)
	/// * `cisco-stackwise` - Cisco StackWise
	/// * `cisco-stackwise-plus` - Cisco StackWise Plus
	/// * `cisco-flexstack` - Cisco FlexStack
	/// * `cisco-flexstack-plus` - Cisco FlexStack Plus
	/// * `cisco-stackwise-80` - Cisco StackWise-80
	/// * `cisco-stackwise-160` - Cisco StackWise-160
	/// * `cisco-stackwise-320` - Cisco StackWise-320
	/// * `cisco-stackwise-480` - Cisco StackWise-480
	/// * `cisco-stackwise-1t` - Cisco StackWise-1T
	/// * `juniper-vcp` - Juniper VCP
	/// * `extreme-summitstack` - Extreme SummitStack
	/// * `extreme-summitstack-128` - Extreme SummitStack-128
	/// * `extreme-summitstack-256` - Extreme SummitStack-256
	/// * `extreme-summitstack-512` - Extreme SummitStack-512
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItem {
	pub _depth: Option<i64>,
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	pub component: Option<serde_json::Value>,
	pub component_id: Option<u64>,
	pub component_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	/// This item was automatically discovered
	pub discovered: Option<bool>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub name: Option<String>,
	pub parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	pub part_id: Option<String>,
	pub role: Option<BriefInventoryItemRole>,
	pub serial: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemRequest {
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	pub component_id: Option<u64>,
	pub component_type: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// This item was automatically discovered
	pub discovered: bool,
	/// Physical label
	pub label: String,
	pub manufacturer: Option<serde_json::Value>,
	pub name: String,
	pub parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemRole {
	pub color: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub inventoryitem_count: Option<i64>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemRoleRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemTemplate {
	pub _depth: Option<i64>,
	pub component: Option<serde_json::Value>,
	pub component_id: Option<u64>,
	pub component_type: Option<String>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	pub part_id: Option<String>,
	pub role: Option<BriefInventoryItemRole>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemTemplateRequest {
	pub component_id: Option<u64>,
	pub component_type: Option<String>,
	pub description: String,
	pub device_type: serde_json::Value,
	/// Physical label
	pub label: String,
	pub manufacturer: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub role: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Job {
	pub completed: Option<String>,
	pub created: Option<String>,
	pub data: Option<serde_json::Value>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub error: Option<String>,
	pub id: i64,
	/// Recurrence interval (in minutes)
	pub interval: Option<u32>,
	pub job_id: Option<String>,
	pub log_entries: Option<Vec<serde_json::Value>>,
	pub name: Option<String>,
	pub object_id: Option<u64>,
	pub object_type: Option<String>,
	pub scheduled: Option<String>,
	pub started: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JournalEntry {
	pub assigned_object: Option<serde_json::Value>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub created_by: Option<i64>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub kind: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub last_updated: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JournalEntryRequest {
	pub assigned_object_id: u64,
	pub assigned_object_type: String,
	pub comments: String,
	pub created_by: Option<i64>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	pub kind: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPN {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub export_targets: Option<Vec<RouteTarget>>,
	pub id: i64,
	pub identifier: Option<i64>,
	pub import_targets: Option<Vec<RouteTarget>>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPNRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub export_targets: Vec<i64>,
	pub identifier: Option<i64>,
	pub import_targets: Vec<i64>,
	pub name: String,
	pub slug: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `evpn-vpws` - EVPN VPWS
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	/// * `spb` - SPB
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPNTermination {
	pub assigned_object: Option<serde_json::Value>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub l2vpn: Option<BriefL2VPN>,
	pub last_updated: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPNTerminationRequest {
	pub assigned_object_id: u64,
	pub assigned_object_type: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub l2vpn: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Location {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Local facility ID or description
	pub facility: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedLocation>,
	pub prefix_count: Option<i64>,
	pub rack_count: Option<i64>,
	pub site: Option<BriefSite>,
	pub slug: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocationRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Local facility ID or description
	pub facility: String,
	pub name: String,
	pub parent: Option<NestedLocationRequest>,
	pub site: serde_json::Value,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MACAddress {
	pub assigned_object: Option<serde_json::Value>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub mac_address: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MACAddressRequest {
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub mac_address: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Manufacturer {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub devicetype_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub inventoryitem_count: Option<i64>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub platform_count: Option<i64>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ManufacturerRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Module {
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub module_bay: Option<NestedModuleBay>,
	pub module_type: Option<BriefModuleType>,
	pub serial: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBay {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub installed_module: Option<BriefModule>,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Identifier to reference when renaming installed components
	pub position: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBayRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	pub installed_module: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Identifier to reference when renaming installed components
	pub position: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBayTemplate {
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Identifier to reference when renaming installed components
	pub position: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBayTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Identifier to reference when renaming installed components
	pub position: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleRequest {
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	pub module_bay: NestedModuleBayRequest,
	pub module_type: serde_json::Value,
	pub serial: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleType {
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub attributes: Option<serde_json::Value>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	/// Discrete part number (optional)
	pub part_number: Option<String>,
	pub profile: Option<BriefModuleTypeProfile>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub weight: Option<f64>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleTypeProfile {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub schema: Option<serde_json::Value>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleTypeProfileRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub schema: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	pub airflow: Option<String>,
	pub attributes: Option<serde_json::Value>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub profile: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedContactGroup {
	pub _depth: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedContactGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDevice {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDeviceRequest {
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDeviceRole {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDeviceRoleRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedGroup {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedIPAddress {
	pub address: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub family: Option<i64>,
	pub id: i64,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedIPAddressRequest {
	pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterface {
	pub _occupied: Option<bool>,
	pub cable: Option<i64>,
	pub device: Option<NestedDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterfaceRequest {
	pub cable: Option<i64>,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterfaceTemplate {
	pub display: Option<String>,
	pub id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedLocation {
	pub _depth: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedLocationRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedModuleBay {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedModuleBayRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedPlatform {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedPlatformRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedProviderAccount {
	pub account: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedRegion {
	pub _depth: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedRegionRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedSiteGroup {
	pub _depth: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedSiteGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTag {
	pub color: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTagRequest {
	pub color: String,
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTenantGroup {
	pub _depth: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTenantGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedUser {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub url: Option<String>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVLAN {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
	/// Numeric VLAN ID (1-4094)
	pub vid: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVLANRequest {
	pub description: String,
	pub name: String,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVMInterface {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
	pub virtual_machine: Option<NestedVirtualMachine>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVMInterfaceRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVirtualMachine {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVirtualMachineRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLANGroup {
	pub _depth: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLANGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLink {
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub ssid: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLinkRequest {
	pub ssid: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Notification {
	pub created: Option<String>,
	pub display: Option<String>,
	/// * `object_created` - Object created
	/// * `object_updated` - Object updated
	/// * `object_deleted` - Object deleted
	/// * `job_started` - Job started
	/// * `job_completed` - Job completed
	/// * `job_failed` - Job failed
	/// * `job_errored` - Job errored
	pub event_type: Option<String>,
	pub id: i64,
	pub object: Option<serde_json::Value>,
	pub object_id: Option<u64>,
	pub object_type: Option<String>,
	pub read: Option<String>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotificationGroup {
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub groups: Option<Vec<Group>>,
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
	pub users: Option<Vec<User>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotificationGroupRequest {
	pub description: String,
	pub groups: Vec<i64>,
	pub name: String,
	pub users: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotificationRequest {
	/// * `object_created` - Object created
	/// * `object_updated` - Object updated
	/// * `object_deleted` - Object deleted
	/// * `job_started` - Job started
	/// * `job_completed` - Job completed
	/// * `job_failed` - Job failed
	/// * `job_errored` - Job errored
	pub event_type: String,
	pub object_id: u64,
	pub object_type: String,
	pub read: Option<String>,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectChange {
	pub action: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub changed_object: Option<serde_json::Value>,
	pub changed_object_id: Option<u64>,
	pub changed_object_type: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub message: Option<String>,
	pub postchange_data: Option<serde_json::Value>,
	pub prechange_data: Option<serde_json::Value>,
	pub request_id: Option<String>,
	pub time: Option<String>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
	pub user_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectPermission {
	/// The list of actions granted by this permission
	pub actions: Option<Vec<String>>,
	/// Queryset filter matching the applicable objects of the selected type(s)
	pub constraints: Option<serde_json::Value>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	pub groups: Option<Vec<NestedGroup>>,
	pub id: i64,
	pub name: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub url: Option<String>,
	pub users: Option<Vec<NestedUser>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectPermissionRequest {
	/// The list of actions granted by this permission
	pub actions: Vec<String>,
	/// Queryset filter matching the applicable objects of the selected type(s)
	pub constraints: Option<serde_json::Value>,
	pub description: String,
	pub enabled: bool,
	pub groups: Vec<i64>,
	pub name: String,
	pub object_types: Vec<String>,
	pub users: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectType {
	pub app_label: Option<String>,
	pub app_name: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub features: Option<Vec<String>>,
	pub id: i64,
	pub is_plugin_model: Option<bool>,
	pub model: Option<String>,
	pub model_name: Option<String>,
	pub model_name_plural: Option<String>,
	pub public: Option<bool>,
	pub rest_api_endpoint: Option<String>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedASNList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ASN>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedASNRangeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ASNRange>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedAggregateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Aggregate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedBookmarkList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Bookmark>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCableList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Cable>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCableTerminationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CableTermination>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCircuitGroupAssignmentList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CircuitGroupAssignment>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCircuitGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CircuitGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCircuitList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Circuit>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCircuitTerminationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CircuitTermination>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCircuitTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CircuitType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedClusterGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ClusterGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedClusterList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Cluster>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedClusterTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ClusterType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConfigContextList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConfigContext>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConfigContextProfileList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConfigContextProfile>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConfigTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConfigTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConsolePortList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConsolePort>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConsolePortTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConsolePortTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConsoleServerPortList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConsoleServerPort>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedConsoleServerPortTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ConsoleServerPortTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedContactAssignmentList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ContactAssignment>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedContactGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ContactGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedContactList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Contact>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedContactRoleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ContactRole>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCustomFieldChoiceSetList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CustomFieldChoiceSet>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCustomFieldList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CustomField>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedCustomLinkList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<CustomLink>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDataFileList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DataFile>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDataSourceList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DataSource>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDeviceBayList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DeviceBay>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDeviceBayTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DeviceBayTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDeviceRoleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DeviceRole>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDeviceTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DeviceType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedDeviceWithConfigContextList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<DeviceWithConfigContext>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedEventRuleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<EventRule>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedExportTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ExportTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedFHRPGroupAssignmentList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<FHRPGroupAssignment>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedFHRPGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<FHRPGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedFrontPortList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<FrontPort>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedFrontPortTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<FrontPortTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Group>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIKEPolicyList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IKEPolicy>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIKEProposalList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IKEProposal>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIPAddressList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IPAddress>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIPRangeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IPRange>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIPSecPolicyList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IPSecPolicy>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIPSecProfileList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IPSecProfile>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedIPSecProposalList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<IPSecProposal>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedImageAttachmentList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ImageAttachment>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedInterfaceList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Interface>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedInterfaceTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<InterfaceTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedInventoryItemList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<InventoryItem>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedInventoryItemRoleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<InventoryItemRole>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedInventoryItemTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<InventoryItemTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedJobList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Job>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedJournalEntryList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<JournalEntry>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedL2VPNList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<L2VPN>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedL2VPNTerminationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<L2VPNTermination>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedLocationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Location>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedMACAddressList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<MACAddress>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedManufacturerList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Manufacturer>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedModuleBayList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ModuleBay>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedModuleBayTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ModuleBayTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedModuleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Module>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedModuleTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ModuleType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedModuleTypeProfileList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ModuleTypeProfile>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedNotificationGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<NotificationGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedNotificationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Notification>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedObjectChangeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ObjectChange>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedObjectPermissionList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ObjectPermission>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedObjectTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ObjectType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPlatformList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Platform>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPowerFeedList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<PowerFeed>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPowerOutletList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<PowerOutlet>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPowerOutletTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<PowerOutletTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPowerPanelList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<PowerPanel>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPowerPortList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<PowerPort>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPowerPortTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<PowerPortTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedPrefixList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Prefix>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedProviderAccountList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ProviderAccount>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedProviderList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Provider>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedProviderNetworkList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ProviderNetwork>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRIRList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RIR>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRackList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Rack>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRackReservationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RackReservation>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRackRoleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RackRole>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRackTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RackType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRackUnitList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RackUnit>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRearPortList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RearPort>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRearPortTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RearPortTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRegionList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Region>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRoleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Role>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedRouteTargetList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<RouteTarget>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedSavedFilterList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<SavedFilter>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedScriptList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Script>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedServiceList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Service>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedServiceTemplateList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<ServiceTemplate>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedSiteGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<SiteGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedSiteList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Site>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedSubscriptionList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Subscription>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTableConfigList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<TableConfig>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTagList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Tag>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTaggedItemList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<TaggedItem>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTenantGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<TenantGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTenantList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Tenant>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTokenList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Token>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTunnelGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<TunnelGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTunnelList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Tunnel>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedTunnelTerminationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<TunnelTermination>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedUserList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<User>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVLANGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VLANGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVLANList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VLAN>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVLANTranslationPolicyList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VLANTranslationPolicy>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVLANTranslationRuleList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VLANTranslationRule>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVMInterfaceList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VMInterface>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVRFList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VRF>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualChassisList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualChassis>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualCircuitList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualCircuit>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualCircuitTerminationList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualCircuitTermination>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualCircuitTypeList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualCircuitType>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualDeviceContextList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualDeviceContext>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualDiskList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualDisk>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedVirtualMachineWithConfigContextList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<VirtualMachineWithConfigContext>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedWebhookList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<Webhook>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedWirelessLANGroupList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<WirelessLANGroup>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedWirelessLANList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<WirelessLAN>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PaginatedWirelessLinkList {
	pub count: Option<i64>,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Option<Vec<WirelessLink>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedASNRangeRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rir: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedASNRequest {
	/// 16- or 32-bit autonomous system number
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asn: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rir: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedBookmarkRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCableTerminationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cable: Option<i64>,
	/// * `A` - A
	/// * `B` - B
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cable_end: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCircuitGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCircuitTerminationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub circuit: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	/// Physical circuit speed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub port_speed: Option<Option<u32>>,
	/// Patch panel ID and port number(s)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pp_info: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// * `A` - A
	/// * `Z` - Z
	#[serde(skip_serializing_if = "Option::is_none")]
	pub term_side: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_id: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_type: Option<Option<String>>,
	/// Upstream speed, if different from port speed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub upstream_speed: Option<Option<u32>>,
	/// ID of the local cross-connect
	#[serde(skip_serializing_if = "Option::is_none")]
	pub xconnect_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCircuitTypeRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedClusterGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedClusterTypeRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedConfigContextProfileRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// A JSON schema specifying the structure of the context data for this profile
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedConfigContextRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cluster_groups: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cluster_types: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub clusters: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_types: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_active: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locations: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platforms: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub regions: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site_groups: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sites: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant_groups: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenants: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedConfigTemplateRequest {
	/// Download file as attachment
	#[serde(skip_serializing_if = "Option::is_none")]
	pub as_attachment: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	#[serde(skip_serializing_if = "Option::is_none")]
	pub environment_params: Option<Option<serde_json::Value>>,
	/// Extension to append to the rendered filename
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_extension: Option<String>,
	/// Filename to give to the rendered export file
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_name: Option<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mime_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Jinja template code.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub template_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedContactRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub groups: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub link: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedContactRoleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCustomLinkRequest {
	/// The class of the first link in a group will be used for the dropdown button
	/// 
	/// * `default` - Default
	/// * `blue` - Blue
	/// * `indigo` - Indigo
	/// * `purple` - Purple
	/// * `pink` - Pink
	/// * `red` - Red
	/// * `orange` - Orange
	/// * `yellow` - Yellow
	/// * `green` - Green
	/// * `teal` - Teal
	/// * `cyan` - Cyan
	/// * `gray` - Gray
	/// * `black` - Black
	/// * `white` - White
	/// * `ghost-dark` - Link
	#[serde(skip_serializing_if = "Option::is_none")]
	pub button_class: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// Links with the same group will appear as a dropdown menu
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group_name: Option<String>,
	/// Jinja2 template code for link text
	#[serde(skip_serializing_if = "Option::is_none")]
	pub link_text: Option<String>,
	/// Jinja2 template code for link URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub link_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Force link to open in a new window
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_window: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedDashboardRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub layout: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedDeviceBayRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub installed_device: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedDeviceBayTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedExportTemplateRequest {
	/// Download file as attachment
	#[serde(skip_serializing_if = "Option::is_none")]
	pub as_attachment: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	#[serde(skip_serializing_if = "Option::is_none")]
	pub environment_params: Option<Option<serde_json::Value>>,
	/// Extension to append to the rendered filename
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_extension: Option<String>,
	/// Filename to give to the rendered export file
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_name: Option<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mime_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	/// Jinja template code.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub template_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedFHRPGroupAssignmentRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interface_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interface_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub priority: Option<u8>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedFHRPGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_key: Option<String>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group_id: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub protocol: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<Vec<i64>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedImageAttachmentRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedInventoryItemRoleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedInventoryItemTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub component_id: Option<Option<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub component_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	/// Manufacturer-assigned part identifier
	#[serde(skip_serializing_if = "Option::is_none")]
	pub part_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedL2VPNTerminationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub l2vpn: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedMACAddressRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_id: Option<Option<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mac_address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedManufacturerRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedModuleBayRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub installed_module: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Identifier to reference when renaming installed components
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedModuleBayTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Identifier to reference when renaming installed components
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedModuleTypeProfileRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedNotificationGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub groups: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub users: Option<Vec<i64>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedNotificationRequest {
	/// * `object_created` - Object created
	/// * `object_updated` - Object updated
	/// * `object_deleted` - Object deleted
	/// * `job_started` - Job started
	/// * `job_completed` - Job completed
	/// * `job_failed` - Job failed
	/// * `job_errored` - Job errored
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub read: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedObjectPermissionRequest {
	/// The list of actions granted by this permission
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actions: Option<Vec<String>>,
	/// Queryset filter matching the applicable objects of the selected type(s)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub constraints: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub groups: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub users: Option<Vec<i64>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedPowerPanelRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedProviderAccountRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub account: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedProviderNetworkRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedProviderRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accounts: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asns: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Full name of the provider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRIRRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// IP space managed by this RIR is considered private
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_private: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRackRoleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRoleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRouteTargetRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Route target value (formatted in accordance with RFC 4360)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedSavedFilterRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shared: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedScriptInputRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commit: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interval: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schedule_at: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedSubscriptionRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTableConfigRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub columns: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ordering: Option<Option<Vec<String>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shared: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub table: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTagRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTenantRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTokenRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_used: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<serde_json::Value>,
	/// Permit create/update/delete operations using this key
	#[serde(skip_serializing_if = "Option::is_none")]
	pub write_enabled: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTunnelGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedUserRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_joined: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub groups: Option<Vec<i64>>,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_active: Option<bool>,
	/// Designates whether the user can log into this admin site.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_staff: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_login: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub password: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<Vec<i64>>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVLANGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_id: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vid_ranges: Option<Vec<IntegerRangeRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVLANTranslationPolicyRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVLANTranslationRuleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Numeric VLAN ID (1-4094)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub local_vid: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub policy: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remote_vid: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVRFRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enforce_unique: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub export_targets: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub import_targets: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Unique route distinguisher (as defined in RFC 4364)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rd: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVirtualCircuitTypeRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVirtualDiskRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub virtual_machine: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWebhookRequest {
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_headers: Option<String>,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub body_template: Option<String>,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ca_file_path: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub http_content_type: Option<String>,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	#[serde(skip_serializing_if = "Option::is_none")]
	pub http_method: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub payload_url: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub secret: Option<String>,
	/// Enable SSL certificate verification. Disable with caution!
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ssl_verification: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableAggregateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_added: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rir: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCableRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub a_terminations: Option<Vec<GenericObjectRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub b_terminations: Option<Vec<GenericObjectRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub length: Option<Option<f64>>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	#[serde(skip_serializing_if = "Option::is_none")]
	pub length_unit: Option<Option<String>>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Single-mode Fiber
	/// * `smf-os1` - Single-mode Fiber (OS1)
	/// * `smf-os2` - Single-mode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `power` - Power
	/// * `usb` - USB
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCircuitGroupAssignmentRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_type: Option<String>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	#[serde(skip_serializing_if = "Option::is_none")]
	pub priority: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCircuitRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assignments: Option<Vec<BriefCircuitGroupAssignmentSerializer_Request>>,
	/// Unique circuit ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Committed rate
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commit_rate: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub distance: Option<Option<f64>>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	#[serde(skip_serializing_if = "Option::is_none")]
	pub distance_unit: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_date: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider_account: Option<Option<serde_json::Value>>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_date: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableClusterRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_id: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<Option<String>>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsolePortRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Port speed in bits per second
	/// 
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Physical port type
	/// 
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsolePortTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsoleServerPortRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Port speed in bits per second
	/// 
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Physical port type
	/// 
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableContactAssignmentRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub contact: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_type: Option<String>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	#[serde(skip_serializing_if = "Option::is_none")]
	pub priority: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableContactGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub base_choices: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub extra_choices: Option<Vec<Vec<serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Choices are automatically ordered alphabetically
	#[serde(skip_serializing_if = "Option::is_none")]
	pub order_alphabetically: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCustomFieldRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub choice_set: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_logic: Option<String>,
	/// Custom fields within the same group will be displayed together
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group_name: Option<String>,
	/// Replicate this value when cloning objects
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_cloneable: Option<bool>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Internal field name
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	#[serde(skip_serializing_if = "Option::is_none")]
	pub related_object_filter: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub related_object_type: Option<Option<String>>,
	/// This field is required when creating new objects or editing an existing object.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required: Option<bool>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_weight: Option<u16>,
	/// The type of data this custom field holds
	/// 
	/// * `text` - Text
	/// * `longtext` - Text (long)
	/// * `integer` - Integer
	/// * `decimal` - Decimal
	/// * `boolean` - Boolean (true/false)
	/// * `date` - Date
	/// * `datetime` - Date & time
	/// * `url` - URL
	/// * `json` - JSON
	/// * `select` - Selection
	/// * `multiselect` - Multiple selection
	/// * `object` - Object
	/// * `multiobject` - Multiple objects
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ui_editable: Option<String>,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ui_visible: Option<String>,
	/// The value of this field must be unique for the assigned object
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unique: Option<bool>,
	/// Maximum allowed value (for numeric fields)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_maximum: Option<Option<f64>>,
	/// Minimum allowed value (for numeric fields)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_minimum: Option<Option<f64>>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_regex: Option<String>,
	/// Fields with higher weights appear lower in a form.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDataSourceRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// Patterns (one per line) matching files to ignore when syncing
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ignore_rules: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source_url: Option<String>,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sync_interval: Option<Option<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDeviceRoleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config_template: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Virtual machines may be assigned to this role
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vm_role: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDeviceTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `rear-to-side` - Rear to side
	/// * `bottom-to-top` - Bottom to top
	/// * `top-to-bottom` - Top to bottom
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub airflow: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_platform: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Devices of this type are excluded when calculating rack utilization.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_from_utilization: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub front_image: Option<Option<String>>,
	/// Device consumes both front and rear rack faces.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_full_depth: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/// Discrete part number (optional)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub part_number: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rear_image: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subdevice_role: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub u_height: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<Option<f64>>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight_unit: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDeviceWithConfigContextRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `rear-to-side` - Rear to side
	/// * `bottom-to-top` - Bottom to top
	/// * `top-to-bottom` - Top to bottom
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub airflow: Option<Option<String>>,
	/// A unique tag used to identify this device
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_tag: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cluster: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config_template: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<serde_json::Value>,
	/// * `front` - Front
	/// * `rear` - Rear
	#[serde(skip_serializing_if = "Option::is_none")]
	pub face: Option<Option<String>>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub latitude: Option<Option<f64>>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	#[serde(skip_serializing_if = "Option::is_none")]
	pub local_context_data: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<Option<serde_json::Value>>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub longitude: Option<Option<f64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub oob_ip: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Option<f64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_ip4: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_ip6: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rack: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<serde_json::Value>,
	/// Chassis serial number, assigned by the manufacturer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site: Option<serde_json::Value>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vc_position: Option<Option<u8>>,
	/// Virtual chassis master election priority
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vc_priority: Option<Option<u8>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub virtual_chassis: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableEventRuleRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action_object_id: Option<Option<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action_object_type: Option<String>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	/// * `notification` - Notification
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action_type: Option<String>,
	/// A set of conditions which determine whether the event will be generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub conditions: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// The types of event which will trigger this rule.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_types: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object_types: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableFrontPortRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rear_port: Option<i64>,
	/// Mapped position on corresponding rear port
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rear_port_position: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableFrontPortTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rear_port: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rear_port_position: Option<u16>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIKEPolicyRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preshared_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proposals: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIKEProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	#[serde(skip_serializing_if = "Option::is_none")]
	pub authentication_algorithm: Option<Option<String>>,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	#[serde(skip_serializing_if = "Option::is_none")]
	pub authentication_method: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encryption_algorithm: Option<String>,
	/// Diffie-Hellman group ID
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
	/// * `15` - Group 15
	/// * `16` - Group 16
	/// * `17` - Group 17
	/// * `18` - Group 18
	/// * `19` - Group 19
	/// * `20` - Group 20
	/// * `21` - Group 21
	/// * `22` - Group 22
	/// * `23` - Group 23
	/// * `24` - Group 24
	/// * `25` - Group 25
	/// * `26` - Group 26
	/// * `27` - Group 27
	/// * `28` - Group 28
	/// * `29` - Group 29
	/// * `30` - Group 30
	/// * `31` - Group 31
	/// * `32` - Group 32
	/// * `33` - Group 33
	/// * `34` - Group 34
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Security association lifetime (in seconds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sa_lifetime: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPAddressRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_id: Option<Option<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Hostname or FQDN (not case-sensitive)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dns_name: Option<String>,
	/// The IP for which this address is the "outside" IP
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nat_inside: Option<Option<i64>>,
	/// The functional role of this IP
	/// 
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<String>>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vrf: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPRangeRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_address: Option<String>,
	/// Prevent the creation of IP addresses within this range
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_populated: Option<bool>,
	/// Report space as fully utilized
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_utilized: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_address: Option<String>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vrf: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPSecPolicyRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Diffie-Hellman group for Perfect Forward Secrecy
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
	/// * `15` - Group 15
	/// * `16` - Group 16
	/// * `17` - Group 17
	/// * `18` - Group 18
	/// * `19` - Group 19
	/// * `20` - Group 20
	/// * `21` - Group 21
	/// * `22` - Group 22
	/// * `23` - Group 23
	/// * `24` - Group 24
	/// * `25` - Group 25
	/// * `26` - Group 26
	/// * `27` - Group 27
	/// * `28` - Group 28
	/// * `29` - Group 29
	/// * `30` - Group 30
	/// * `31` - Group 31
	/// * `32` - Group 32
	/// * `33` - Group 33
	/// * `34` - Group 34
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pfs_group: Option<Option<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proposals: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPSecProfileRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ike_policy: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipsec_policy: Option<serde_json::Value>,
	/// * `esp` - ESP
	/// * `ah` - AH
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPSecProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	#[serde(skip_serializing_if = "Option::is_none")]
	pub authentication_algorithm: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encryption_algorithm: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Security association lifetime (in kilobytes)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sa_lifetime_data: Option<Option<u32>>,
	/// Security association lifetime (seconds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sa_lifetime_seconds: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableInterfaceRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bridge: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	#[serde(skip_serializing_if = "Option::is_none")]
	pub duplex: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lag: Option<Option<i64>>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	/// This interface is used only for out-of-band management
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mgmt_only: Option<bool>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mtu: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	/// * `pd` - PD
	/// * `pse` - PSE
	#[serde(skip_serializing_if = "Option::is_none")]
	pub poe_mode: Option<Option<String>>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub poe_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_mac_address: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub qinq_svlan: Option<Option<serde_json::Value>>,
	/// * `2.4g-1-2412-22` - 1 (2412 MHz)
	/// * `2.4g-2-2417-22` - 2 (2417 MHz)
	/// * `2.4g-3-2422-22` - 3 (2422 MHz)
	/// * `2.4g-4-2427-22` - 4 (2427 MHz)
	/// * `2.4g-5-2432-22` - 5 (2432 MHz)
	/// * `2.4g-6-2437-22` - 6 (2437 MHz)
	/// * `2.4g-7-2442-22` - 7 (2442 MHz)
	/// * `2.4g-8-2447-22` - 8 (2447 MHz)
	/// * `2.4g-9-2452-22` - 9 (2452 MHz)
	/// * `2.4g-10-2457-22` - 10 (2457 MHz)
	/// * `2.4g-11-2462-22` - 11 (2462 MHz)
	/// * `2.4g-12-2467-22` - 12 (2467 MHz)
	/// * `2.4g-13-2472-22` - 13 (2472 MHz)
	/// * `5g-32-5160-20` - 32 (5160/20 MHz)
	/// * `5g-34-5170-40` - 34 (5170/40 MHz)
	/// * `5g-36-5180-20` - 36 (5180/20 MHz)
	/// * `5g-38-5190-40` - 38 (5190/40 MHz)
	/// * `5g-40-5200-20` - 40 (5200/20 MHz)
	/// * `5g-42-5210-80` - 42 (5210/80 MHz)
	/// * `5g-44-5220-20` - 44 (5220/20 MHz)
	/// * `5g-46-5230-40` - 46 (5230/40 MHz)
	/// * `5g-48-5240-20` - 48 (5240/20 MHz)
	/// * `5g-50-5250-160` - 50 (5250/160 MHz)
	/// * `5g-52-5260-20` - 52 (5260/20 MHz)
	/// * `5g-54-5270-40` - 54 (5270/40 MHz)
	/// * `5g-56-5280-20` - 56 (5280/20 MHz)
	/// * `5g-58-5290-80` - 58 (5290/80 MHz)
	/// * `5g-60-5300-20` - 60 (5300/20 MHz)
	/// * `5g-62-5310-40` - 62 (5310/40 MHz)
	/// * `5g-64-5320-20` - 64 (5320/20 MHz)
	/// * `5g-100-5500-20` - 100 (5500/20 MHz)
	/// * `5g-102-5510-40` - 102 (5510/40 MHz)
	/// * `5g-104-5520-20` - 104 (5520/20 MHz)
	/// * `5g-106-5530-80` - 106 (5530/80 MHz)
	/// * `5g-108-5540-20` - 108 (5540/20 MHz)
	/// * `5g-110-5550-40` - 110 (5550/40 MHz)
	/// * `5g-112-5560-20` - 112 (5560/20 MHz)
	/// * `5g-114-5570-160` - 114 (5570/160 MHz)
	/// * `5g-116-5580-20` - 116 (5580/20 MHz)
	/// * `5g-118-5590-40` - 118 (5590/40 MHz)
	/// * `5g-120-5600-20` - 120 (5600/20 MHz)
	/// * `5g-122-5610-80` - 122 (5610/80 MHz)
	/// * `5g-124-5620-20` - 124 (5620/20 MHz)
	/// * `5g-126-5630-40` - 126 (5630/40 MHz)
	/// * `5g-128-5640-20` - 128 (5640/20 MHz)
	/// * `5g-132-5660-20` - 132 (5660/20 MHz)
	/// * `5g-134-5670-40` - 134 (5670/40 MHz)
	/// * `5g-136-5680-20` - 136 (5680/20 MHz)
	/// * `5g-138-5690-80` - 138 (5690/80 MHz)
	/// * `5g-140-5700-20` - 140 (5700/20 MHz)
	/// * `5g-142-5710-40` - 142 (5710/40 MHz)
	/// * `5g-144-5720-20` - 144 (5720/20 MHz)
	/// * `5g-149-5745-20` - 149 (5745/20 MHz)
	/// * `5g-151-5755-40` - 151 (5755/40 MHz)
	/// * `5g-153-5765-20` - 153 (5765/20 MHz)
	/// * `5g-155-5775-80` - 155 (5775/80 MHz)
	/// * `5g-157-5785-20` - 157 (5785/20 MHz)
	/// * `5g-159-5795-40` - 159 (5795/40 MHz)
	/// * `5g-161-5805-20` - 161 (5805/20 MHz)
	/// * `5g-163-5815-160` - 163 (5815/160 MHz)
	/// * `5g-165-5825-20` - 165 (5825/20 MHz)
	/// * `5g-167-5835-40` - 167 (5835/40 MHz)
	/// * `5g-169-5845-20` - 169 (5845/20 MHz)
	/// * `5g-171-5855-80` - 171 (5855/80 MHz)
	/// * `5g-173-5865-20` - 173 (5865/20 MHz)
	/// * `5g-175-5875-40` - 175 (5875/40 MHz)
	/// * `5g-177-5885-20` - 177 (5885/20 MHz)
	/// * `6g-1-5955-20` - 1 (5955/20 MHz)
	/// * `6g-3-5965-40` - 3 (5965/40 MHz)
	/// * `6g-5-5975-20` - 5 (5975/20 MHz)
	/// * `6g-7-5985-80` - 7 (5985/80 MHz)
	/// * `6g-9-5995-20` - 9 (5995/20 MHz)
	/// * `6g-11-6005-40` - 11 (6005/40 MHz)
	/// * `6g-13-6015-20` - 13 (6015/20 MHz)
	/// * `6g-15-6025-160` - 15 (6025/160 MHz)
	/// * `6g-17-6035-20` - 17 (6035/20 MHz)
	/// * `6g-19-6045-40` - 19 (6045/40 MHz)
	/// * `6g-21-6055-20` - 21 (6055/20 MHz)
	/// * `6g-23-6065-80` - 23 (6065/80 MHz)
	/// * `6g-25-6075-20` - 25 (6075/20 MHz)
	/// * `6g-27-6085-40` - 27 (6085/40 MHz)
	/// * `6g-29-6095-20` - 29 (6095/20 MHz)
	/// * `6g-31-6105-320` - 31 (6105/320 MHz)
	/// * `6g-33-6115-20` - 33 (6115/20 MHz)
	/// * `6g-35-6125-40` - 35 (6125/40 MHz)
	/// * `6g-37-6135-20` - 37 (6135/20 MHz)
	/// * `6g-39-6145-80` - 39 (6145/80 MHz)
	/// * `6g-41-6155-20` - 41 (6155/20 MHz)
	/// * `6g-43-6165-40` - 43 (6165/40 MHz)
	/// * `6g-45-6175-20` - 45 (6175/20 MHz)
	/// * `6g-47-6185-160` - 47 (6185/160 MHz)
	/// * `6g-49-6195-20` - 49 (6195/20 MHz)
	/// * `6g-51-6205-40` - 51 (6205/40 MHz)
	/// * `6g-53-6215-20` - 53 (6215/20 MHz)
	/// * `6g-55-6225-80` - 55 (6225/80 MHz)
	/// * `6g-57-6235-20` - 57 (6235/20 MHz)
	/// * `6g-59-6245-40` - 59 (6245/40 MHz)
	/// * `6g-61-6255-20` - 61 (6255/20 MHz)
	/// * `6g-65-6275-20` - 65 (6275/20 MHz)
	/// * `6g-67-6285-40` - 67 (6285/40 MHz)
	/// * `6g-69-6295-20` - 69 (6295/20 MHz)
	/// * `6g-71-6305-80` - 71 (6305/80 MHz)
	/// * `6g-73-6315-20` - 73 (6315/20 MHz)
	/// * `6g-75-6325-40` - 75 (6325/40 MHz)
	/// * `6g-77-6335-20` - 77 (6335/20 MHz)
	/// * `6g-79-6345-160` - 79 (6345/160 MHz)
	/// * `6g-81-6355-20` - 81 (6355/20 MHz)
	/// * `6g-83-6365-40` - 83 (6365/40 MHz)
	/// * `6g-85-6375-20` - 85 (6375/20 MHz)
	/// * `6g-87-6385-80` - 87 (6385/80 MHz)
	/// * `6g-89-6395-20` - 89 (6395/20 MHz)
	/// * `6g-91-6405-40` - 91 (6405/40 MHz)
	/// * `6g-93-6415-20` - 93 (6415/20 MHz)
	/// * `6g-95-6425-320` - 95 (6425/320 MHz)
	/// * `6g-97-6435-20` - 97 (6435/20 MHz)
	/// * `6g-99-6445-40` - 99 (6445/40 MHz)
	/// * `6g-101-6455-20` - 101 (6455/20 MHz)
	/// * `6g-103-6465-80` - 103 (6465/80 MHz)
	/// * `6g-105-6475-20` - 105 (6475/20 MHz)
	/// * `6g-107-6485-40` - 107 (6485/40 MHz)
	/// * `6g-109-6495-20` - 109 (6495/20 MHz)
	/// * `6g-111-6505-160` - 111 (6505/160 MHz)
	/// * `6g-113-6515-20` - 113 (6515/20 MHz)
	/// * `6g-115-6525-40` - 115 (6525/40 MHz)
	/// * `6g-117-6535-20` - 117 (6535/20 MHz)
	/// * `6g-119-6545-80` - 119 (6545/80 MHz)
	/// * `6g-121-6555-20` - 121 (6555/20 MHz)
	/// * `6g-123-6565-40` - 123 (6565/40 MHz)
	/// * `6g-125-6575-20` - 125 (6575/20 MHz)
	/// * `6g-129-6595-20` - 129 (6595/20 MHz)
	/// * `6g-131-6605-40` - 131 (6605/40 MHz)
	/// * `6g-133-6615-20` - 133 (6615/20 MHz)
	/// * `6g-135-6625-80` - 135 (6625/80 MHz)
	/// * `6g-137-6635-20` - 137 (6635/20 MHz)
	/// * `6g-139-6645-40` - 139 (6645/40 MHz)
	/// * `6g-141-6655-20` - 141 (6655/20 MHz)
	/// * `6g-143-6665-160` - 143 (6665/160 MHz)
	/// * `6g-145-6675-20` - 145 (6675/20 MHz)
	/// * `6g-147-6685-40` - 147 (6685/40 MHz)
	/// * `6g-149-6695-20` - 149 (6695/20 MHz)
	/// * `6g-151-6705-80` - 151 (6705/80 MHz)
	/// * `6g-153-6715-20` - 153 (6715/20 MHz)
	/// * `6g-155-6725-40` - 155 (6725/40 MHz)
	/// * `6g-157-6735-20` - 157 (6735/20 MHz)
	/// * `6g-159-6745-320` - 159 (6745/320 MHz)
	/// * `6g-161-6755-20` - 161 (6755/20 MHz)
	/// * `6g-163-6765-40` - 163 (6765/40 MHz)
	/// * `6g-165-6775-20` - 165 (6775/20 MHz)
	/// * `6g-167-6785-80` - 167 (6785/80 MHz)
	/// * `6g-169-6795-20` - 169 (6795/20 MHz)
	/// * `6g-171-6805-40` - 171 (6805/40 MHz)
	/// * `6g-173-6815-20` - 173 (6815/20 MHz)
	/// * `6g-175-6825-160` - 175 (6825/160 MHz)
	/// * `6g-177-6835-20` - 177 (6835/20 MHz)
	/// * `6g-179-6845-40` - 179 (6845/40 MHz)
	/// * `6g-181-6855-20` - 181 (6855/20 MHz)
	/// * `6g-183-6865-80` - 183 (6865/80 MHz)
	/// * `6g-185-6875-20` - 185 (6875/20 MHz)
	/// * `6g-187-6885-40` - 187 (6885/40 MHz)
	/// * `6g-189-6895-20` - 189 (6895/20 MHz)
	/// * `6g-193-6915-20` - 193 (6915/20 MHz)
	/// * `6g-195-6925-40` - 195 (6925/40 MHz)
	/// * `6g-197-6935-20` - 197 (6935/20 MHz)
	/// * `6g-199-6945-80` - 199 (6945/80 MHz)
	/// * `6g-201-6955-20` - 201 (6955/20 MHz)
	/// * `6g-203-6965-40` - 203 (6965/40 MHz)
	/// * `6g-205-6975-20` - 205 (6975/20 MHz)
	/// * `6g-207-6985-160` - 207 (6985/160 MHz)
	/// * `6g-209-6995-20` - 209 (6995/20 MHz)
	/// * `6g-211-7005-40` - 211 (7005/40 MHz)
	/// * `6g-213-7015-20` - 213 (7015/20 MHz)
	/// * `6g-215-7025-80` - 215 (7025/80 MHz)
	/// * `6g-217-7035-20` - 217 (7035/20 MHz)
	/// * `6g-219-7045-40` - 219 (7045/40 MHz)
	/// * `6g-221-7055-20` - 221 (7055/20 MHz)
	/// * `6g-225-7075-20` - 225 (7075/20 MHz)
	/// * `6g-227-7085-40` - 227 (7085/40 MHz)
	/// * `6g-229-7095-20` - 229 (7095/20 MHz)
	/// * `6g-233-7115-20` - 233 (7115/20 MHz)
	/// * `60g-1-58320-2160` - 1 (58.32/2.16 GHz)
	/// * `60g-2-60480-2160` - 2 (60.48/2.16 GHz)
	/// * `60g-3-62640-2160` - 3 (62.64/2.16 GHz)
	/// * `60g-4-64800-2160` - 4 (64.80/2.16 GHz)
	/// * `60g-5-66960-2160` - 5 (66.96/2.16 GHz)
	/// * `60g-6-69120-2160` - 6 (69.12/2.16 GHz)
	/// * `60g-9-59400-4320` - 9 (59.40/4.32 GHz)
	/// * `60g-10-61560-4320` - 10 (61.56/4.32 GHz)
	/// * `60g-11-63720-4320` - 11 (63.72/4.32 GHz)
	/// * `60g-12-65880-4320` - 12 (65.88/4.32 GHz)
	/// * `60g-13-68040-4320` - 13 (68.04/4.32 GHz)
	/// * `60g-17-60480-6480` - 17 (60.48/6.48 GHz)
	/// * `60g-18-62640-6480` - 18 (62.64/6.48 GHz)
	/// * `60g-19-64800-6480` - 19 (64.80/6.48 GHz)
	/// * `60g-20-66960-6480` - 20 (66.96/6.48 GHz)
	/// * `60g-25-61560-6480` - 25 (61.56/8.64 GHz)
	/// * `60g-26-63720-6480` - 26 (63.72/8.64 GHz)
	/// * `60g-27-65880-6480` - 27 (65.88/8.64 GHz)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rf_channel: Option<Option<String>>,
	/// Populated by selected channel (if set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rf_channel_frequency: Option<Option<f64>>,
	/// Populated by selected channel (if set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rf_channel_width: Option<Option<f64>>,
	/// * `ap` - Access point
	/// * `station` - Station
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rf_role: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tagged_vlans: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tx_power: Option<Option<i8>>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME)
	/// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
	/// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
	/// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
	/// * `1000base-cx` - 1000BASE-CX (1GE DAC)
	/// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
	/// * `1000base-ex` - 1000BASE-EX (1GE)
	/// * `1000base-lsx` - 1000BASE-LSX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `1000base-zx` - 1000BASE-ZX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
	/// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
	/// * `10gbase-er` - 10GBASE-ER (10GE)
	/// * `10gbase-lr` - 10GBASE-LR (10GE)
	/// * `10gbase-lrm` - 10GBASE-LRM (10GE)
	/// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
	/// * `10gbase-sr` - 10GBASE-SR (10GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-zr` - 10GBASE-ZR (10GE)
	/// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
	/// * `25gbase-er` - 25GBASE-ER (25GE)
	/// * `25gbase-lr` - 25GBASE-LR (25GE)
	/// * `25gbase-sr` - 25GBASE-SR (25GE)
	/// * `25gbase-t` - 25GBASE-T (25GE)
	/// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
	/// * `40gbase-er4` - 40GBASE-ER4 (40GE)
	/// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
	/// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
	/// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
	/// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
	/// * `50gbase-er` - 50GBASE-ER (50GE)
	/// * `50gbase-fr` - 50GBASE-FR (50GE)
	/// * `50gbase-lr` - 50GBASE-LR (50GE)
	/// * `50gbase-sr` - 50GBASE-SR (50GE)
	/// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
	/// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
	/// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
	/// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
	/// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
	/// * `100gbase-dr` - 100GBASE-DR (100GE)
	/// * `100gbase-er4` - 100GBASE-ER4 (100GE)
	/// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
	/// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
	/// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
	/// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
	/// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
	/// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
	/// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
	/// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
	/// * `100gbase-zr` - 100GBASE-ZR (100GE)
	/// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
	/// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
	/// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
	/// * `200gbase-er4` - 200GBASE-ER4 (200GE)
	/// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
	/// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
	/// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
	/// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
	/// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
	/// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
	/// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
	/// * `400gbase-er8` - 400GBASE-ER8 (400GE)
	/// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
	/// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
	/// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
	/// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
	/// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
	/// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
	/// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
	/// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
	/// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
	/// * `400gbase-zr` - 400GBASE-ZR (400GE)
	/// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
	/// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
	/// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
	/// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `1000base-kx` - 1000BASE-KX (1GE)
	/// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
	/// * `5gbase-kr` - 5GBASE-KR (5GE)
	/// * `10gbase-kr` - 10GBASE-KR (10GE)
	/// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
	/// * `25gbase-kr` - 25GBASE-KR (25GE)
	/// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
	/// * `50gbase-kr` - 50GBASE-KR (50GE)
	/// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
	/// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
	/// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
	/// * `ieee802.11a` - IEEE 802.11a
	/// * `ieee802.11g` - IEEE 802.11b/g
	/// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
	/// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
	/// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
	/// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
	/// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
	/// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
	/// * `4g` - 4G
	/// * `5g` - 5G
	/// * `sonet-oc3` - OC-3/STM-1
	/// * `sonet-oc12` - OC-12/STM-4
	/// * `sonet-oc48` - OC-48/STM-16
	/// * `sonet-oc192` - OC-192/STM-64
	/// * `sonet-oc768` - OC-768/STM-256
	/// * `sonet-oc1920` - OC-1920/STM-640
	/// * `sonet-oc3840` - OC-3840/STM-1234
	/// * `1gfc-sfp` - SFP (1GFC)
	/// * `2gfc-sfp` - SFP (2GFC)
	/// * `4gfc-sfp` - SFP (4GFC)
	/// * `8gfc-sfpp` - SFP+ (8GFC)
	/// * `16gfc-sfpp` - SFP+ (16GFC)
	/// * `32gfc-sfp28` - SFP28 (32GFC)
	/// * `32gfc-sfpp` - SFP+ (32GFC)
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
	/// * `64gfc-sfpdd` - SFP-DD (64GFC)
	/// * `64gfc-sfpp` - SFP+ (64GFC)
	/// * `128gfc-qsfp28` - QSFP28 (128GFC)
	/// * `infiniband-sdr` - SDR (2 Gbps)
	/// * `infiniband-ddr` - DDR (4 Gbps)
	/// * `infiniband-qdr` - QDR (8 Gbps)
	/// * `infiniband-fdr10` - FDR10 (10 Gbps)
	/// * `infiniband-fdr` - FDR (13.5 Gbps)
	/// * `infiniband-edr` - EDR (25 Gbps)
	/// * `infiniband-hdr` - HDR (50 Gbps)
	/// * `infiniband-ndr` - NDR (100 Gbps)
	/// * `infiniband-xdr` - XDR (250 Gbps)
	/// * `t1` - T1 (1.544 Mbps)
	/// * `e1` - E1 (2.048 Mbps)
	/// * `t3` - T3 (45 Mbps)
	/// * `e3` - E3 (34 Mbps)
	/// * `xdsl` - xDSL
	/// * `docsis` - DOCSIS
	/// * `moca` - MoCA
	/// * `bpon` - BPON (622 Mbps / 155 Mbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `25g-pon` - 25G-PON (25 Gbps)
	/// * `50g-pon` - 50G-PON (50 Gbps)
	/// * `cisco-stackwise` - Cisco StackWise
	/// * `cisco-stackwise-plus` - Cisco StackWise Plus
	/// * `cisco-flexstack` - Cisco FlexStack
	/// * `cisco-flexstack-plus` - Cisco FlexStack Plus
	/// * `cisco-stackwise-80` - Cisco StackWise-80
	/// * `cisco-stackwise-160` - Cisco StackWise-160
	/// * `cisco-stackwise-320` - Cisco StackWise-320
	/// * `cisco-stackwise-480` - Cisco StackWise-480
	/// * `cisco-stackwise-1t` - Cisco StackWise-1T
	/// * `juniper-vcp` - Juniper VCP
	/// * `extreme-summitstack` - Extreme SummitStack
	/// * `extreme-summitstack-128` - Extreme SummitStack-128
	/// * `extreme-summitstack-256` - Extreme SummitStack-256
	/// * `extreme-summitstack-512` - Extreme SummitStack-512
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub untagged_vlan: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vdcs: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vlan_translation_policy: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vrf: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wireless_lans: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wwn: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableInterfaceTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bridge: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mgmt_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `pd` - PD
	/// * `pse` - PSE
	#[serde(skip_serializing_if = "Option::is_none")]
	pub poe_mode: Option<Option<String>>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub poe_type: Option<Option<String>>,
	/// * `ap` - Access point
	/// * `station` - Station
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rf_role: Option<Option<String>>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME)
	/// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
	/// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
	/// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
	/// * `1000base-cx` - 1000BASE-CX (1GE DAC)
	/// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
	/// * `1000base-ex` - 1000BASE-EX (1GE)
	/// * `1000base-lsx` - 1000BASE-LSX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `1000base-zx` - 1000BASE-ZX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
	/// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
	/// * `10gbase-er` - 10GBASE-ER (10GE)
	/// * `10gbase-lr` - 10GBASE-LR (10GE)
	/// * `10gbase-lrm` - 10GBASE-LRM (10GE)
	/// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
	/// * `10gbase-sr` - 10GBASE-SR (10GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-zr` - 10GBASE-ZR (10GE)
	/// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
	/// * `25gbase-er` - 25GBASE-ER (25GE)
	/// * `25gbase-lr` - 25GBASE-LR (25GE)
	/// * `25gbase-sr` - 25GBASE-SR (25GE)
	/// * `25gbase-t` - 25GBASE-T (25GE)
	/// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
	/// * `40gbase-er4` - 40GBASE-ER4 (40GE)
	/// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
	/// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
	/// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
	/// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
	/// * `50gbase-er` - 50GBASE-ER (50GE)
	/// * `50gbase-fr` - 50GBASE-FR (50GE)
	/// * `50gbase-lr` - 50GBASE-LR (50GE)
	/// * `50gbase-sr` - 50GBASE-SR (50GE)
	/// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
	/// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
	/// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
	/// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
	/// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
	/// * `100gbase-dr` - 100GBASE-DR (100GE)
	/// * `100gbase-er4` - 100GBASE-ER4 (100GE)
	/// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
	/// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
	/// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
	/// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
	/// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
	/// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
	/// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
	/// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
	/// * `100gbase-zr` - 100GBASE-ZR (100GE)
	/// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
	/// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
	/// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
	/// * `200gbase-er4` - 200GBASE-ER4 (200GE)
	/// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
	/// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
	/// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
	/// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
	/// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
	/// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
	/// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
	/// * `400gbase-er8` - 400GBASE-ER8 (400GE)
	/// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
	/// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
	/// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
	/// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
	/// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
	/// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
	/// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
	/// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
	/// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
	/// * `400gbase-zr` - 400GBASE-ZR (400GE)
	/// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
	/// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
	/// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
	/// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `1000base-kx` - 1000BASE-KX (1GE)
	/// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
	/// * `5gbase-kr` - 5GBASE-KR (5GE)
	/// * `10gbase-kr` - 10GBASE-KR (10GE)
	/// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
	/// * `25gbase-kr` - 25GBASE-KR (25GE)
	/// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
	/// * `50gbase-kr` - 50GBASE-KR (50GE)
	/// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
	/// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
	/// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
	/// * `ieee802.11a` - IEEE 802.11a
	/// * `ieee802.11g` - IEEE 802.11b/g
	/// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
	/// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
	/// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
	/// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
	/// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
	/// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
	/// * `4g` - 4G
	/// * `5g` - 5G
	/// * `sonet-oc3` - OC-3/STM-1
	/// * `sonet-oc12` - OC-12/STM-4
	/// * `sonet-oc48` - OC-48/STM-16
	/// * `sonet-oc192` - OC-192/STM-64
	/// * `sonet-oc768` - OC-768/STM-256
	/// * `sonet-oc1920` - OC-1920/STM-640
	/// * `sonet-oc3840` - OC-3840/STM-1234
	/// * `1gfc-sfp` - SFP (1GFC)
	/// * `2gfc-sfp` - SFP (2GFC)
	/// * `4gfc-sfp` - SFP (4GFC)
	/// * `8gfc-sfpp` - SFP+ (8GFC)
	/// * `16gfc-sfpp` - SFP+ (16GFC)
	/// * `32gfc-sfp28` - SFP28 (32GFC)
	/// * `32gfc-sfpp` - SFP+ (32GFC)
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
	/// * `64gfc-sfpdd` - SFP-DD (64GFC)
	/// * `64gfc-sfpp` - SFP+ (64GFC)
	/// * `128gfc-qsfp28` - QSFP28 (128GFC)
	/// * `infiniband-sdr` - SDR (2 Gbps)
	/// * `infiniband-ddr` - DDR (4 Gbps)
	/// * `infiniband-qdr` - QDR (8 Gbps)
	/// * `infiniband-fdr10` - FDR10 (10 Gbps)
	/// * `infiniband-fdr` - FDR (13.5 Gbps)
	/// * `infiniband-edr` - EDR (25 Gbps)
	/// * `infiniband-hdr` - HDR (50 Gbps)
	/// * `infiniband-ndr` - NDR (100 Gbps)
	/// * `infiniband-xdr` - XDR (250 Gbps)
	/// * `t1` - T1 (1.544 Mbps)
	/// * `e1` - E1 (2.048 Mbps)
	/// * `t3` - T3 (45 Mbps)
	/// * `e3` - E3 (34 Mbps)
	/// * `xdsl` - xDSL
	/// * `docsis` - DOCSIS
	/// * `moca` - MoCA
	/// * `bpon` - BPON (622 Mbps / 155 Mbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `25g-pon` - 25G-PON (25 Gbps)
	/// * `50g-pon` - 50G-PON (50 Gbps)
	/// * `cisco-stackwise` - Cisco StackWise
	/// * `cisco-stackwise-plus` - Cisco StackWise Plus
	/// * `cisco-flexstack` - Cisco FlexStack
	/// * `cisco-flexstack-plus` - Cisco FlexStack Plus
	/// * `cisco-stackwise-80` - Cisco StackWise-80
	/// * `cisco-stackwise-160` - Cisco StackWise-160
	/// * `cisco-stackwise-320` - Cisco StackWise-320
	/// * `cisco-stackwise-480` - Cisco StackWise-480
	/// * `cisco-stackwise-1t` - Cisco StackWise-1T
	/// * `juniper-vcp` - Juniper VCP
	/// * `extreme-summitstack` - Extreme SummitStack
	/// * `extreme-summitstack-128` - Extreme SummitStack-128
	/// * `extreme-summitstack-256` - Extreme SummitStack-256
	/// * `extreme-summitstack-512` - Extreme SummitStack-512
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableInventoryItemRequest {
	/// A unique tag used to identify this item
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_tag: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub component_id: Option<Option<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub component_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// This item was automatically discovered
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discovered: Option<bool>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	/// Manufacturer-assigned part identifier
	#[serde(skip_serializing_if = "Option::is_none")]
	pub part_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableJournalEntryRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assigned_object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_by: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableL2VPNRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub export_targets: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub identifier: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub import_targets: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `evpn-vpws` - EVPN VPWS
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	/// * `spb` - SPB
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableLocationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Local facility ID or description
	#[serde(skip_serializing_if = "Option::is_none")]
	pub facility: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableModuleRequest {
	/// A unique tag used to identify this device
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_tag: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_bay: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableModuleTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	#[serde(skip_serializing_if = "Option::is_none")]
	pub airflow: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/// Discrete part number (optional)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub part_number: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<Option<f64>>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight_unit: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePlatformRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config_template: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerFeedRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub amperage: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	/// Maximum permissible draw (percentage)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_utilization: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phase: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_panel: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rack: Option<Option<serde_json::Value>>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	/// * `ac` - AC
	/// * `dc` - DC
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supply: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voltage: Option<i16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerOutletRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feed_leg: Option<Option<String>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_port: Option<Option<serde_json::Value>>,
	/// * `enabled` - Enabled
	/// * `disabled` - Disabled
	/// * `faulty` - Faulty
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Physical port type
	/// 
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
	/// * `iec-60320-c17` - C17
	/// * `iec-60320-c19` - C19
	/// * `iec-60320-c21` - C21
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15r` - NEMA 1-15R
	/// * `nema-5-15r` - NEMA 5-15R
	/// * `nema-5-20r` - NEMA 5-20R
	/// * `nema-5-30r` - NEMA 5-30R
	/// * `nema-5-50r` - NEMA 5-50R
	/// * `nema-6-15r` - NEMA 6-15R
	/// * `nema-6-20r` - NEMA 6-20R
	/// * `nema-6-30r` - NEMA 6-30R
	/// * `nema-6-50r` - NEMA 6-50R
	/// * `nema-10-30r` - NEMA 10-30R
	/// * `nema-10-50r` - NEMA 10-50R
	/// * `nema-14-20r` - NEMA 14-20R
	/// * `nema-14-30r` - NEMA 14-30R
	/// * `nema-14-50r` - NEMA 14-50R
	/// * `nema-14-60r` - NEMA 14-60R
	/// * `nema-15-15r` - NEMA 15-15R
	/// * `nema-15-20r` - NEMA 15-20R
	/// * `nema-15-30r` - NEMA 15-30R
	/// * `nema-15-50r` - NEMA 15-50R
	/// * `nema-15-60r` - NEMA 15-60R
	/// * `nema-l1-15r` - NEMA L1-15R
	/// * `nema-l5-15r` - NEMA L5-15R
	/// * `nema-l5-20r` - NEMA L5-20R
	/// * `nema-l5-30r` - NEMA L5-30R
	/// * `nema-l5-50r` - NEMA L5-50R
	/// * `nema-l6-15r` - NEMA L6-15R
	/// * `nema-l6-20r` - NEMA L6-20R
	/// * `nema-l6-30r` - NEMA L6-30R
	/// * `nema-l6-50r` - NEMA L6-50R
	/// * `nema-l10-30r` - NEMA L10-30R
	/// * `nema-l14-20r` - NEMA L14-20R
	/// * `nema-l14-30r` - NEMA L14-30R
	/// * `nema-l14-50r` - NEMA L14-50R
	/// * `nema-l14-60r` - NEMA L14-60R
	/// * `nema-l15-20r` - NEMA L15-20R
	/// * `nema-l15-30r` - NEMA L15-30R
	/// * `nema-l15-50r` - NEMA L15-50R
	/// * `nema-l15-60r` - NEMA L15-60R
	/// * `nema-l21-20r` - NEMA L21-20R
	/// * `nema-l21-30r` - NEMA L21-30R
	/// * `nema-l22-20r` - NEMA L22-20R
	/// * `nema-l22-30r` - NEMA L22-30R
	/// * `CS6360C` - CS6360C
	/// * `CS6364C` - CS6364C
	/// * `CS8164C` - CS8164C
	/// * `CS8264C` - CS8264C
	/// * `CS8364C` - CS8364C
	/// * `CS8464C` - CS8464C
	/// * `ita-e` - ITA Type E (CEE 7/5)
	/// * `ita-f` - ITA Type F (CEE 7/3)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `ita-multistandard` - ITA Multistandard
	/// * `usb-a` - USB Type A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-c` - USB Type C
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `eaton-c39` - Eaton C39
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerOutletTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feed_leg: Option<Option<String>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_port: Option<Option<serde_json::Value>>,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
	/// * `iec-60320-c17` - C17
	/// * `iec-60320-c19` - C19
	/// * `iec-60320-c21` - C21
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15r` - NEMA 1-15R
	/// * `nema-5-15r` - NEMA 5-15R
	/// * `nema-5-20r` - NEMA 5-20R
	/// * `nema-5-30r` - NEMA 5-30R
	/// * `nema-5-50r` - NEMA 5-50R
	/// * `nema-6-15r` - NEMA 6-15R
	/// * `nema-6-20r` - NEMA 6-20R
	/// * `nema-6-30r` - NEMA 6-30R
	/// * `nema-6-50r` - NEMA 6-50R
	/// * `nema-10-30r` - NEMA 10-30R
	/// * `nema-10-50r` - NEMA 10-50R
	/// * `nema-14-20r` - NEMA 14-20R
	/// * `nema-14-30r` - NEMA 14-30R
	/// * `nema-14-50r` - NEMA 14-50R
	/// * `nema-14-60r` - NEMA 14-60R
	/// * `nema-15-15r` - NEMA 15-15R
	/// * `nema-15-20r` - NEMA 15-20R
	/// * `nema-15-30r` - NEMA 15-30R
	/// * `nema-15-50r` - NEMA 15-50R
	/// * `nema-15-60r` - NEMA 15-60R
	/// * `nema-l1-15r` - NEMA L1-15R
	/// * `nema-l5-15r` - NEMA L5-15R
	/// * `nema-l5-20r` - NEMA L5-20R
	/// * `nema-l5-30r` - NEMA L5-30R
	/// * `nema-l5-50r` - NEMA L5-50R
	/// * `nema-l6-15r` - NEMA L6-15R
	/// * `nema-l6-20r` - NEMA L6-20R
	/// * `nema-l6-30r` - NEMA L6-30R
	/// * `nema-l6-50r` - NEMA L6-50R
	/// * `nema-l10-30r` - NEMA L10-30R
	/// * `nema-l14-20r` - NEMA L14-20R
	/// * `nema-l14-30r` - NEMA L14-30R
	/// * `nema-l14-50r` - NEMA L14-50R
	/// * `nema-l14-60r` - NEMA L14-60R
	/// * `nema-l15-20r` - NEMA L15-20R
	/// * `nema-l15-30r` - NEMA L15-30R
	/// * `nema-l15-50r` - NEMA L15-50R
	/// * `nema-l15-60r` - NEMA L15-60R
	/// * `nema-l21-20r` - NEMA L21-20R
	/// * `nema-l21-30r` - NEMA L21-30R
	/// * `nema-l22-20r` - NEMA L22-20R
	/// * `nema-l22-30r` - NEMA L22-30R
	/// * `CS6360C` - CS6360C
	/// * `CS6364C` - CS6364C
	/// * `CS8164C` - CS8164C
	/// * `CS8264C` - CS8264C
	/// * `CS8364C` - CS8364C
	/// * `CS8464C` - CS8464C
	/// * `ita-e` - ITA Type E (CEE 7/5)
	/// * `ita-f` - ITA Type F (CEE 7/3)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `ita-multistandard` - ITA Multistandard
	/// * `usb-a` - USB Type A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-c` - USB Type C
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `eaton-c39` - Eaton C39
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerPortRequest {
	/// Allocated power draw (watts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allocated_draw: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	/// Maximum power draw (watts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub maximum_draw: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Physical port type
	/// 
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
	/// * `iec-60320-c18` - C18
	/// * `iec-60320-c20` - C20
	/// * `iec-60320-c22` - C22
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15p` - NEMA 1-15P
	/// * `nema-5-15p` - NEMA 5-15P
	/// * `nema-5-20p` - NEMA 5-20P
	/// * `nema-5-30p` - NEMA 5-30P
	/// * `nema-5-50p` - NEMA 5-50P
	/// * `nema-6-15p` - NEMA 6-15P
	/// * `nema-6-20p` - NEMA 6-20P
	/// * `nema-6-30p` - NEMA 6-30P
	/// * `nema-6-50p` - NEMA 6-50P
	/// * `nema-10-30p` - NEMA 10-30P
	/// * `nema-10-50p` - NEMA 10-50P
	/// * `nema-14-20p` - NEMA 14-20P
	/// * `nema-14-30p` - NEMA 14-30P
	/// * `nema-14-50p` - NEMA 14-50P
	/// * `nema-14-60p` - NEMA 14-60P
	/// * `nema-15-15p` - NEMA 15-15P
	/// * `nema-15-20p` - NEMA 15-20P
	/// * `nema-15-30p` - NEMA 15-30P
	/// * `nema-15-50p` - NEMA 15-50P
	/// * `nema-15-60p` - NEMA 15-60P
	/// * `nema-l1-15p` - NEMA L1-15P
	/// * `nema-l5-15p` - NEMA L5-15P
	/// * `nema-l5-20p` - NEMA L5-20P
	/// * `nema-l5-30p` - NEMA L5-30P
	/// * `nema-l5-50p` - NEMA L5-50P
	/// * `nema-l6-15p` - NEMA L6-15P
	/// * `nema-l6-20p` - NEMA L6-20P
	/// * `nema-l6-30p` - NEMA L6-30P
	/// * `nema-l6-50p` - NEMA L6-50P
	/// * `nema-l10-30p` - NEMA L10-30P
	/// * `nema-l14-20p` - NEMA L14-20P
	/// * `nema-l14-30p` - NEMA L14-30P
	/// * `nema-l14-50p` - NEMA L14-50P
	/// * `nema-l14-60p` - NEMA L14-60P
	/// * `nema-l15-20p` - NEMA L15-20P
	/// * `nema-l15-30p` - NEMA L15-30P
	/// * `nema-l15-50p` - NEMA L15-50P
	/// * `nema-l15-60p` - NEMA L15-60P
	/// * `nema-l21-20p` - NEMA L21-20P
	/// * `nema-l21-30p` - NEMA L21-30P
	/// * `nema-l22-20p` - NEMA L22-20P
	/// * `nema-l22-30p` - NEMA L22-30P
	/// * `cs6361c` - CS6361C
	/// * `cs6365c` - CS6365C
	/// * `cs8165c` - CS8165C
	/// * `cs8265c` - CS8265C
	/// * `cs8365c` - CS8365C
	/// * `cs8465c` - CS8465C
	/// * `ita-c` - ITA Type C (CEE 7/16)
	/// * `ita-e` - ITA Type E (CEE 7/6)
	/// * `ita-f` - ITA Type F (CEE 7/4)
	/// * `ita-ef` - ITA Type E/F (CEE 7/7)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `usb-3-b` - USB 3.0 Type B
	/// * `usb-3-micro-b` - USB 3.0 Micro B
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerPortTemplateRequest {
	/// Allocated power draw (watts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allocated_draw: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Maximum power draw (watts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub maximum_draw: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
	/// * `iec-60320-c18` - C18
	/// * `iec-60320-c20` - C20
	/// * `iec-60320-c22` - C22
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15p` - NEMA 1-15P
	/// * `nema-5-15p` - NEMA 5-15P
	/// * `nema-5-20p` - NEMA 5-20P
	/// * `nema-5-30p` - NEMA 5-30P
	/// * `nema-5-50p` - NEMA 5-50P
	/// * `nema-6-15p` - NEMA 6-15P
	/// * `nema-6-20p` - NEMA 6-20P
	/// * `nema-6-30p` - NEMA 6-30P
	/// * `nema-6-50p` - NEMA 6-50P
	/// * `nema-10-30p` - NEMA 10-30P
	/// * `nema-10-50p` - NEMA 10-50P
	/// * `nema-14-20p` - NEMA 14-20P
	/// * `nema-14-30p` - NEMA 14-30P
	/// * `nema-14-50p` - NEMA 14-50P
	/// * `nema-14-60p` - NEMA 14-60P
	/// * `nema-15-15p` - NEMA 15-15P
	/// * `nema-15-20p` - NEMA 15-20P
	/// * `nema-15-30p` - NEMA 15-30P
	/// * `nema-15-50p` - NEMA 15-50P
	/// * `nema-15-60p` - NEMA 15-60P
	/// * `nema-l1-15p` - NEMA L1-15P
	/// * `nema-l5-15p` - NEMA L5-15P
	/// * `nema-l5-20p` - NEMA L5-20P
	/// * `nema-l5-30p` - NEMA L5-30P
	/// * `nema-l5-50p` - NEMA L5-50P
	/// * `nema-l6-15p` - NEMA L6-15P
	/// * `nema-l6-20p` - NEMA L6-20P
	/// * `nema-l6-30p` - NEMA L6-30P
	/// * `nema-l6-50p` - NEMA L6-50P
	/// * `nema-l10-30p` - NEMA L10-30P
	/// * `nema-l14-20p` - NEMA L14-20P
	/// * `nema-l14-30p` - NEMA L14-30P
	/// * `nema-l14-50p` - NEMA L14-50P
	/// * `nema-l14-60p` - NEMA L14-60P
	/// * `nema-l15-20p` - NEMA L15-20P
	/// * `nema-l15-30p` - NEMA L15-30P
	/// * `nema-l15-50p` - NEMA L15-50P
	/// * `nema-l15-60p` - NEMA L15-60P
	/// * `nema-l21-20p` - NEMA L21-20P
	/// * `nema-l21-30p` - NEMA L21-30P
	/// * `nema-l22-20p` - NEMA L22-20P
	/// * `nema-l22-30p` - NEMA L22-30P
	/// * `cs6361c` - CS6361C
	/// * `cs6365c` - CS6365C
	/// * `cs8165c` - CS8165C
	/// * `cs8265c` - CS8265C
	/// * `cs8365c` - CS8365C
	/// * `cs8465c` - CS8465C
	/// * `ita-c` - ITA Type C (CEE 7/16)
	/// * `ita-e` - ITA Type E (CEE 7/6)
	/// * `ita-f` - ITA Type F (CEE 7/4)
	/// * `ita-ef` - ITA Type E/F (CEE 7/7)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `usb-3-b` - USB 3.0 Type B
	/// * `usb-3-micro-b` - USB 3.0 Micro B
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePrefixRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// All IP addresses within this prefix are considered usable
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_pool: Option<bool>,
	/// Treat as fully utilized
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_utilized: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_id: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<Option<String>>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vlan: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vrf: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRackRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	#[serde(skip_serializing_if = "Option::is_none")]
	pub airflow: Option<Option<String>>,
	/// A unique tag used to identify this rack
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_tag: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	/// Units are numbered top-to-bottom
	#[serde(skip_serializing_if = "Option::is_none")]
	pub desc_units: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub facility_id: Option<Option<String>>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub form_factor: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<Option<serde_json::Value>>,
	/// Maximum load capacity for the rack
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_weight: Option<Option<u32>>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mounting_depth: Option<Option<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Outer dimension of rack (depth)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_depth: Option<Option<u16>>,
	/// Outer dimension of rack (height)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_height: Option<Option<u16>>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_unit: Option<Option<String>>,
	/// Outer dimension of rack (width)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_width: Option<Option<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rack_type: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site: Option<serde_json::Value>,
	/// Starting unit for rack
	#[serde(skip_serializing_if = "Option::is_none")]
	pub starting_unit: Option<u16>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	/// Height in rack units
	#[serde(skip_serializing_if = "Option::is_none")]
	pub u_height: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<Option<f64>>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight_unit: Option<Option<String>>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRackReservationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rack: Option<serde_json::Value>,
	/// * `pending` - Pending
	/// * `active` - Active
	/// * `stale` - Stale
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub units: Option<Vec<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRackTypeRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	/// Units are numbered top-to-bottom
	#[serde(skip_serializing_if = "Option::is_none")]
	pub desc_units: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub form_factor: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer: Option<serde_json::Value>,
	/// Maximum load capacity for the rack
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_weight: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mounting_depth: Option<Option<u16>>,
	/// Outer dimension of rack (depth)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_depth: Option<Option<u16>>,
	/// Outer dimension of rack (height)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_height: Option<Option<u16>>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_unit: Option<Option<String>>,
	/// Outer dimension of rack (width)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outer_width: Option<Option<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// Starting unit for rack
	#[serde(skip_serializing_if = "Option::is_none")]
	pub starting_unit: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// Height in rack units
	#[serde(skip_serializing_if = "Option::is_none")]
	pub u_height: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<Option<f64>>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight_unit: Option<Option<String>>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRearPortRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Treat as if a cable is connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mark_connected: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Number of front ports which may be mapped
	#[serde(skip_serializing_if = "Option::is_none")]
	pub positions: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRearPortTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<Option<serde_json::Value>>,
	/// Physical label
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module_type: Option<Option<serde_json::Value>>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub positions: Option<u16>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRegionRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableServiceRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipaddresses: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_object_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ports: Option<Vec<u16>>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	#[serde(skip_serializing_if = "Option::is_none")]
	pub protocol: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableServiceTemplateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ports: Option<Vec<u16>>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	#[serde(skip_serializing_if = "Option::is_none")]
	pub protocol: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableSiteGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableSiteRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asns: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Local facility ID or description
	#[serde(skip_serializing_if = "Option::is_none")]
	pub facility: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<Option<serde_json::Value>>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub latitude: Option<Option<f64>>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub longitude: Option<Option<f64>>,
	/// Full name of the site
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Physical location of the building
	#[serde(skip_serializing_if = "Option::is_none")]
	pub physical_address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<Option<serde_json::Value>>,
	/// If different from the physical address
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shipping_address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_zone: Option<Option<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableTenantGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableTunnelRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	/// * `wireguard` - WireGuard
	/// * `openvpn` - OpenVPN
	/// * `l2tp` - L2TP
	/// * `pptp` - PPTP
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encapsulation: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ipsec_profile: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tunnel_id: Option<Option<u64>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableTunnelTerminationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outside_ip: Option<Option<serde_json::Value>>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_id: Option<Option<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub termination_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tunnel: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVLANRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)
	/// 
	/// * `svlan` - Service
	/// * `cvlan` - Customer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub qinq_role: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub qinq_svlan: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site: Option<Option<serde_json::Value>>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	/// Numeric VLAN ID (1-4094)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vid: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVMInterfaceRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bridge: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mtu: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_mac_address: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub qinq_svlan: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tagged_vlans: Option<Vec<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub untagged_vlan: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub virtual_machine: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vlan_translation_policy: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vrf: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualChassisRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub domain: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub master: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualCircuitRequest {
	/// Unique circuit ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider_account: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider_network: Option<serde_json::Value>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualCircuitTerminationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interface: Option<serde_json::Value>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub virtual_circuit: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualDeviceContextRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub identifier: Option<Option<u16>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_ip4: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_ip6: Option<Option<serde_json::Value>>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cluster: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config_template: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disk: Option<Option<u32>>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	#[serde(skip_serializing_if = "Option::is_none")]
	pub local_context_data: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub memory: Option<Option<u32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_ip4: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_ip6: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub site: Option<Option<serde_json::Value>>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	/// * `paused` - Paused
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vcpus: Option<Option<f64>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableWirelessLANGroupRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableWirelessLANRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_cipher: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_psk: Option<String>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_id: Option<Option<i64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ssid: Option<String>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vlan: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableWirelessLinkRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_cipher: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_psk: Option<String>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_type: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_fields: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub distance: Option<Option<f64>>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	#[serde(skip_serializing_if = "Option::is_none")]
	pub distance_unit: Option<Option<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interface_a: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interface_b: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ssid: Option<String>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<NestedTagRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant: Option<Option<serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Platform {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub config_template: Option<BriefConfigTemplate>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub name: Option<String>,
	pub parent: Option<NestedPlatform>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlatformRequest {
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub manufacturer: Option<serde_json::Value>,
	pub name: String,
	pub parent: Option<NestedPlatformRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerFeed {
	pub _occupied: Option<bool>,
	pub amperage: Option<u16>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub comments: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_reachable: Option<bool>,
	pub connected_endpoints_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	/// Maximum permissible draw (percentage)
	pub max_utilization: Option<u8>,
	pub name: Option<String>,
	pub phase: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub power_panel: Option<BriefPowerPanel>,
	pub rack: Option<BriefRack>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub supply: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
	pub voltage: Option<i16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerFeedRequest {
	pub amperage: u16,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// Maximum permissible draw (percentage)
	pub max_utilization: u8,
	pub name: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	pub phase: String,
	pub power_panel: serde_json::Value,
	pub rack: Option<serde_json::Value>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	pub status: String,
	/// * `ac` - AC
	/// * `dc` - DC
	pub supply: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	pub r#type: String,
	pub voltage: i16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutlet {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub color: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_reachable: Option<bool>,
	pub connected_endpoints_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub feed_leg: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	pub power_port: Option<BriefPowerPort>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutletRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub power_port: Option<serde_json::Value>,
	/// * `enabled` - Enabled
	/// * `disabled` - Disabled
	/// * `faulty` - Faulty
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
	/// * `iec-60320-c17` - C17
	/// * `iec-60320-c19` - C19
	/// * `iec-60320-c21` - C21
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15r` - NEMA 1-15R
	/// * `nema-5-15r` - NEMA 5-15R
	/// * `nema-5-20r` - NEMA 5-20R
	/// * `nema-5-30r` - NEMA 5-30R
	/// * `nema-5-50r` - NEMA 5-50R
	/// * `nema-6-15r` - NEMA 6-15R
	/// * `nema-6-20r` - NEMA 6-20R
	/// * `nema-6-30r` - NEMA 6-30R
	/// * `nema-6-50r` - NEMA 6-50R
	/// * `nema-10-30r` - NEMA 10-30R
	/// * `nema-10-50r` - NEMA 10-50R
	/// * `nema-14-20r` - NEMA 14-20R
	/// * `nema-14-30r` - NEMA 14-30R
	/// * `nema-14-50r` - NEMA 14-50R
	/// * `nema-14-60r` - NEMA 14-60R
	/// * `nema-15-15r` - NEMA 15-15R
	/// * `nema-15-20r` - NEMA 15-20R
	/// * `nema-15-30r` - NEMA 15-30R
	/// * `nema-15-50r` - NEMA 15-50R
	/// * `nema-15-60r` - NEMA 15-60R
	/// * `nema-l1-15r` - NEMA L1-15R
	/// * `nema-l5-15r` - NEMA L5-15R
	/// * `nema-l5-20r` - NEMA L5-20R
	/// * `nema-l5-30r` - NEMA L5-30R
	/// * `nema-l5-50r` - NEMA L5-50R
	/// * `nema-l6-15r` - NEMA L6-15R
	/// * `nema-l6-20r` - NEMA L6-20R
	/// * `nema-l6-30r` - NEMA L6-30R
	/// * `nema-l6-50r` - NEMA L6-50R
	/// * `nema-l10-30r` - NEMA L10-30R
	/// * `nema-l14-20r` - NEMA L14-20R
	/// * `nema-l14-30r` - NEMA L14-30R
	/// * `nema-l14-50r` - NEMA L14-50R
	/// * `nema-l14-60r` - NEMA L14-60R
	/// * `nema-l15-20r` - NEMA L15-20R
	/// * `nema-l15-30r` - NEMA L15-30R
	/// * `nema-l15-50r` - NEMA L15-50R
	/// * `nema-l15-60r` - NEMA L15-60R
	/// * `nema-l21-20r` - NEMA L21-20R
	/// * `nema-l21-30r` - NEMA L21-30R
	/// * `nema-l22-20r` - NEMA L22-20R
	/// * `nema-l22-30r` - NEMA L22-30R
	/// * `CS6360C` - CS6360C
	/// * `CS6364C` - CS6364C
	/// * `CS8164C` - CS8164C
	/// * `CS8264C` - CS8264C
	/// * `CS8364C` - CS8364C
	/// * `CS8464C` - CS8464C
	/// * `ita-e` - ITA Type E (CEE 7/5)
	/// * `ita-f` - ITA Type F (CEE 7/3)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `ita-multistandard` - ITA Multistandard
	/// * `usb-a` - USB Type A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-c` - USB Type C
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `eaton-c39` - Eaton C39
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutletTemplate {
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub feed_leg: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub power_port: Option<BriefPowerPortTemplate>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutletTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub power_port: Option<serde_json::Value>,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
	/// * `iec-60320-c17` - C17
	/// * `iec-60320-c19` - C19
	/// * `iec-60320-c21` - C21
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15r` - NEMA 1-15R
	/// * `nema-5-15r` - NEMA 5-15R
	/// * `nema-5-20r` - NEMA 5-20R
	/// * `nema-5-30r` - NEMA 5-30R
	/// * `nema-5-50r` - NEMA 5-50R
	/// * `nema-6-15r` - NEMA 6-15R
	/// * `nema-6-20r` - NEMA 6-20R
	/// * `nema-6-30r` - NEMA 6-30R
	/// * `nema-6-50r` - NEMA 6-50R
	/// * `nema-10-30r` - NEMA 10-30R
	/// * `nema-10-50r` - NEMA 10-50R
	/// * `nema-14-20r` - NEMA 14-20R
	/// * `nema-14-30r` - NEMA 14-30R
	/// * `nema-14-50r` - NEMA 14-50R
	/// * `nema-14-60r` - NEMA 14-60R
	/// * `nema-15-15r` - NEMA 15-15R
	/// * `nema-15-20r` - NEMA 15-20R
	/// * `nema-15-30r` - NEMA 15-30R
	/// * `nema-15-50r` - NEMA 15-50R
	/// * `nema-15-60r` - NEMA 15-60R
	/// * `nema-l1-15r` - NEMA L1-15R
	/// * `nema-l5-15r` - NEMA L5-15R
	/// * `nema-l5-20r` - NEMA L5-20R
	/// * `nema-l5-30r` - NEMA L5-30R
	/// * `nema-l5-50r` - NEMA L5-50R
	/// * `nema-l6-15r` - NEMA L6-15R
	/// * `nema-l6-20r` - NEMA L6-20R
	/// * `nema-l6-30r` - NEMA L6-30R
	/// * `nema-l6-50r` - NEMA L6-50R
	/// * `nema-l10-30r` - NEMA L10-30R
	/// * `nema-l14-20r` - NEMA L14-20R
	/// * `nema-l14-30r` - NEMA L14-30R
	/// * `nema-l14-50r` - NEMA L14-50R
	/// * `nema-l14-60r` - NEMA L14-60R
	/// * `nema-l15-20r` - NEMA L15-20R
	/// * `nema-l15-30r` - NEMA L15-30R
	/// * `nema-l15-50r` - NEMA L15-50R
	/// * `nema-l15-60r` - NEMA L15-60R
	/// * `nema-l21-20r` - NEMA L21-20R
	/// * `nema-l21-30r` - NEMA L21-30R
	/// * `nema-l22-20r` - NEMA L22-20R
	/// * `nema-l22-30r` - NEMA L22-30R
	/// * `CS6360C` - CS6360C
	/// * `CS6364C` - CS6364C
	/// * `CS8164C` - CS8164C
	/// * `CS8264C` - CS8264C
	/// * `CS8364C` - CS8364C
	/// * `CS8464C` - CS8464C
	/// * `ita-e` - ITA Type E (CEE 7/5)
	/// * `ita-f` - ITA Type F (CEE 7/3)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `ita-multistandard` - ITA Multistandard
	/// * `usb-a` - USB Type A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-c` - USB Type C
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `eaton-c39` - Eaton C39
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPanel {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub location: Option<BriefLocation>,
	pub name: Option<String>,
	pub powerfeed_count: Option<i64>,
	pub site: Option<BriefSite>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPanelRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub location: Option<serde_json::Value>,
	pub name: String,
	pub site: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPort {
	pub _occupied: Option<bool>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_reachable: Option<bool>,
	pub connected_endpoints_type: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPortRequest {
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
	/// * `iec-60320-c18` - C18
	/// * `iec-60320-c20` - C20
	/// * `iec-60320-c22` - C22
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15p` - NEMA 1-15P
	/// * `nema-5-15p` - NEMA 5-15P
	/// * `nema-5-20p` - NEMA 5-20P
	/// * `nema-5-30p` - NEMA 5-30P
	/// * `nema-5-50p` - NEMA 5-50P
	/// * `nema-6-15p` - NEMA 6-15P
	/// * `nema-6-20p` - NEMA 6-20P
	/// * `nema-6-30p` - NEMA 6-30P
	/// * `nema-6-50p` - NEMA 6-50P
	/// * `nema-10-30p` - NEMA 10-30P
	/// * `nema-10-50p` - NEMA 10-50P
	/// * `nema-14-20p` - NEMA 14-20P
	/// * `nema-14-30p` - NEMA 14-30P
	/// * `nema-14-50p` - NEMA 14-50P
	/// * `nema-14-60p` - NEMA 14-60P
	/// * `nema-15-15p` - NEMA 15-15P
	/// * `nema-15-20p` - NEMA 15-20P
	/// * `nema-15-30p` - NEMA 15-30P
	/// * `nema-15-50p` - NEMA 15-50P
	/// * `nema-15-60p` - NEMA 15-60P
	/// * `nema-l1-15p` - NEMA L1-15P
	/// * `nema-l5-15p` - NEMA L5-15P
	/// * `nema-l5-20p` - NEMA L5-20P
	/// * `nema-l5-30p` - NEMA L5-30P
	/// * `nema-l5-50p` - NEMA L5-50P
	/// * `nema-l6-15p` - NEMA L6-15P
	/// * `nema-l6-20p` - NEMA L6-20P
	/// * `nema-l6-30p` - NEMA L6-30P
	/// * `nema-l6-50p` - NEMA L6-50P
	/// * `nema-l10-30p` - NEMA L10-30P
	/// * `nema-l14-20p` - NEMA L14-20P
	/// * `nema-l14-30p` - NEMA L14-30P
	/// * `nema-l14-50p` - NEMA L14-50P
	/// * `nema-l14-60p` - NEMA L14-60P
	/// * `nema-l15-20p` - NEMA L15-20P
	/// * `nema-l15-30p` - NEMA L15-30P
	/// * `nema-l15-50p` - NEMA L15-50P
	/// * `nema-l15-60p` - NEMA L15-60P
	/// * `nema-l21-20p` - NEMA L21-20P
	/// * `nema-l21-30p` - NEMA L21-30P
	/// * `nema-l22-20p` - NEMA L22-20P
	/// * `nema-l22-30p` - NEMA L22-30P
	/// * `cs6361c` - CS6361C
	/// * `cs6365c` - CS6365C
	/// * `cs8165c` - CS8165C
	/// * `cs8265c` - CS8265C
	/// * `cs8365c` - CS8365C
	/// * `cs8465c` - CS8465C
	/// * `ita-c` - ITA Type C (CEE 7/16)
	/// * `ita-e` - ITA Type E (CEE 7/6)
	/// * `ita-f` - ITA Type F (CEE 7/4)
	/// * `ita-ef` - ITA Type E/F (CEE 7/7)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `usb-3-b` - USB 3.0 Type B
	/// * `usb-3-micro-b` - USB 3.0 Micro B
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPortTemplate {
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPortTemplateRequest {
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
	/// * `iec-60320-c18` - C18
	/// * `iec-60320-c20` - C20
	/// * `iec-60320-c22` - C22
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15p` - NEMA 1-15P
	/// * `nema-5-15p` - NEMA 5-15P
	/// * `nema-5-20p` - NEMA 5-20P
	/// * `nema-5-30p` - NEMA 5-30P
	/// * `nema-5-50p` - NEMA 5-50P
	/// * `nema-6-15p` - NEMA 6-15P
	/// * `nema-6-20p` - NEMA 6-20P
	/// * `nema-6-30p` - NEMA 6-30P
	/// * `nema-6-50p` - NEMA 6-50P
	/// * `nema-10-30p` - NEMA 10-30P
	/// * `nema-10-50p` - NEMA 10-50P
	/// * `nema-14-20p` - NEMA 14-20P
	/// * `nema-14-30p` - NEMA 14-30P
	/// * `nema-14-50p` - NEMA 14-50P
	/// * `nema-14-60p` - NEMA 14-60P
	/// * `nema-15-15p` - NEMA 15-15P
	/// * `nema-15-20p` - NEMA 15-20P
	/// * `nema-15-30p` - NEMA 15-30P
	/// * `nema-15-50p` - NEMA 15-50P
	/// * `nema-15-60p` - NEMA 15-60P
	/// * `nema-l1-15p` - NEMA L1-15P
	/// * `nema-l5-15p` - NEMA L5-15P
	/// * `nema-l5-20p` - NEMA L5-20P
	/// * `nema-l5-30p` - NEMA L5-30P
	/// * `nema-l5-50p` - NEMA L5-50P
	/// * `nema-l6-15p` - NEMA L6-15P
	/// * `nema-l6-20p` - NEMA L6-20P
	/// * `nema-l6-30p` - NEMA L6-30P
	/// * `nema-l6-50p` - NEMA L6-50P
	/// * `nema-l10-30p` - NEMA L10-30P
	/// * `nema-l14-20p` - NEMA L14-20P
	/// * `nema-l14-30p` - NEMA L14-30P
	/// * `nema-l14-50p` - NEMA L14-50P
	/// * `nema-l14-60p` - NEMA L14-60P
	/// * `nema-l15-20p` - NEMA L15-20P
	/// * `nema-l15-30p` - NEMA L15-30P
	/// * `nema-l15-50p` - NEMA L15-50P
	/// * `nema-l15-60p` - NEMA L15-60P
	/// * `nema-l21-20p` - NEMA L21-20P
	/// * `nema-l21-30p` - NEMA L21-30P
	/// * `nema-l22-20p` - NEMA L22-20P
	/// * `nema-l22-30p` - NEMA L22-30P
	/// * `cs6361c` - CS6361C
	/// * `cs6365c` - CS6365C
	/// * `cs8165c` - CS8165C
	/// * `cs8265c` - CS8265C
	/// * `cs8365c` - CS8365C
	/// * `cs8465c` - CS8465C
	/// * `ita-c` - ITA Type C (CEE 7/16)
	/// * `ita-e` - ITA Type E (CEE 7/6)
	/// * `ita-f` - ITA Type F (CEE 7/4)
	/// * `ita-ef` - ITA Type E/F (CEE 7/7)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `usb-3-b` - USB 3.0 Type B
	/// * `usb-3-micro-b` - USB 3.0 Micro B
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Prefix {
	pub _depth: Option<i64>,
	pub children: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: Option<bool>,
	pub last_updated: Option<String>,
	/// Treat as fully utilized
	pub mark_utilized: Option<bool>,
	pub prefix: Option<String>,
	pub role: Option<BriefRole>,
	pub scope: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vlan: Option<BriefVLAN>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PrefixRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: bool,
	/// Treat as fully utilized
	pub mark_utilized: bool,
	pub prefix: String,
	pub role: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Provider {
	pub accounts: Option<Vec<NestedProviderAccount>>,
	pub asns: Option<Vec<ASN>>,
	pub circuit_count: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// Full name of the provider
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderAccount {
	pub account: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub provider: Option<BriefProvider>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderAccountRequest {
	pub account: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub provider: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderNetwork {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub provider: Option<BriefProvider>,
	pub service_id: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderNetworkRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub provider: serde_json::Value,
	pub service_id: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderRequest {
	pub accounts: Vec<i64>,
	pub asns: Vec<i64>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Full name of the provider
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RIR {
	pub aggregate_count: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// IP space managed by this RIR is considered private
	pub is_private: Option<bool>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RIRRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// IP space managed by this RIR is considered private
	pub is_private: bool,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Rack {
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: Option<bool>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub facility_id: Option<String>,
	pub form_factor: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub location: Option<BriefLocation>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub name: Option<String>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	pub outer_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	pub powerfeed_count: Option<i64>,
	pub rack_type: Option<BriefRackType>,
	pub role: Option<BriefRackRole>,
	pub serial: Option<String>,
	pub site: Option<BriefSite>,
	/// Starting unit for rack
	pub starting_unit: Option<u16>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	/// Height in rack units
	pub u_height: Option<u8>,
	pub url: Option<String>,
	pub weight: Option<f64>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub width: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	pub airflow: String,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	pub description: String,
	pub facility_id: Option<String>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	pub location: Option<serde_json::Value>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub name: String,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	pub rack_type: Option<serde_json::Value>,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	pub site: serde_json::Value,
	/// Starting unit for rack
	pub starting_unit: u16,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// Height in rack units
	pub u_height: u8,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: i64,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackReservation {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub rack: Option<BriefRack>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub units: Option<Vec<u16>>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackReservationRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub rack: serde_json::Value,
	/// * `pending` - Pending
	/// * `active` - Active
	/// * `stale` - Stale
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub units: Vec<u16>,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackRole {
	pub color: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub rack_count: Option<i64>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackRoleRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackType {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: Option<bool>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub form_factor: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	pub model: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	pub outer_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	pub slug: Option<String>,
	/// Starting unit for rack
	pub starting_unit: Option<u16>,
	pub tags: Option<Vec<NestedTag>>,
	/// Height in rack units
	pub u_height: Option<u8>,
	pub url: Option<String>,
	pub weight: Option<f64>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub width: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackTypeRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	pub description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	pub manufacturer: serde_json::Value,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	pub model: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	pub slug: String,
	/// Starting unit for rack
	pub starting_unit: u16,
	pub tags: Vec<NestedTagRequest>,
	/// Height in rack units
	pub u_height: u8,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: i64,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackUnit {
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub face: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: f64,
	pub name: Option<String>,
	pub occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPort {
	pub _occupied: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub color: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Number of front ports which may be mapped
	pub positions: Option<u16>,
	pub tags: Option<Vec<NestedTag>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPortRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Number of front ports which may be mapped
	pub positions: u16,
	pub tags: Vec<NestedTagRequest>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPortTemplate {
	pub color: Option<String>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub display: Option<String>,
	pub id: i64,
	/// Physical label
	pub label: Option<String>,
	pub last_updated: Option<String>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub positions: Option<u16>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPortTemplateRequest {
	pub color: String,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub positions: u16,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Region {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedRegion>,
	pub prefix_count: Option<i64>,
	pub site_count: Option<i64>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RegionRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<NestedRegionRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Role {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub prefix_count: Option<i64>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub vlan_count: Option<i64>,
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RoleRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RouteTarget {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// Route target value (formatted in accordance with RFC 4360)
	pub name: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RouteTargetRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Route target value (formatted in accordance with RFC 4360)
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SavedFilter {
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub parameters: Option<serde_json::Value>,
	pub shared: Option<bool>,
	pub slug: Option<String>,
	pub url: Option<String>,
	pub user: Option<i64>,
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SavedFilterRequest {
	pub description: String,
	pub enabled: bool,
	pub name: String,
	pub object_types: Vec<String>,
	pub parameters: serde_json::Value,
	pub shared: bool,
	pub slug: String,
	pub user: Option<i64>,
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Script {
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub is_executable: Option<bool>,
	pub module: Option<i64>,
	pub name: Option<String>,
	pub result: Option<BriefJob>,
	pub url: Option<String>,
	pub vars: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ScriptInputRequest {
	pub commit: bool,
	pub data: serde_json::Value,
	pub interval: Option<i64>,
	pub schedule_at: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Service {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub ipaddresses: Option<Vec<IPAddress>>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<serde_json::Value>,
	pub parent_object_id: Option<u64>,
	pub parent_object_type: Option<String>,
	pub ports: Option<Vec<u16>>,
	pub protocol: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub ipaddresses: Vec<i64>,
	pub name: String,
	pub parent_object_id: u64,
	pub parent_object_type: String,
	pub ports: Vec<u16>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceTemplate {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub ports: Option<Vec<u16>>,
	pub protocol: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceTemplateRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub ports: Vec<u16>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Site {
	pub asns: Option<Vec<ASN>>,
	pub circuit_count: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Local facility ID or description
	pub facility: Option<String>,
	pub group: Option<BriefSiteGroup>,
	pub id: i64,
	pub last_updated: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	/// Full name of the site
	pub name: Option<String>,
	/// Physical location of the building
	pub physical_address: Option<String>,
	pub prefix_count: Option<i64>,
	pub rack_count: Option<i64>,
	pub region: Option<BriefRegion>,
	/// If different from the physical address
	pub shipping_address: Option<String>,
	pub slug: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub time_zone: Option<String>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteGroup {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedSiteGroup>,
	pub prefix_count: Option<i64>,
	pub site_count: Option<i64>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<NestedSiteGroupRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteRequest {
	pub asns: Vec<i64>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Local facility ID or description
	pub facility: String,
	pub group: Option<serde_json::Value>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	/// Full name of the site
	pub name: String,
	/// Physical location of the building
	pub physical_address: String,
	pub region: Option<serde_json::Value>,
	/// If different from the physical address
	pub shipping_address: String,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub time_zone: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Subscription {
	pub created: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub object: Option<serde_json::Value>,
	pub object_id: Option<u64>,
	pub object_type: Option<String>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubscriptionRequest {
	pub object_id: u64,
	pub object_type: String,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TableConfig {
	pub columns: Option<Vec<String>>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub object_type: Option<String>,
	pub ordering: Option<Vec<String>>,
	pub shared: Option<bool>,
	pub table: Option<String>,
	pub url: Option<String>,
	pub user: Option<i64>,
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TableConfigRequest {
	pub columns: Vec<String>,
	pub description: String,
	pub enabled: bool,
	pub name: String,
	pub object_type: String,
	pub ordering: Option<Vec<String>>,
	pub shared: bool,
	pub table: String,
	pub user: Option<i64>,
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tag {
	pub color: Option<String>,
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub slug: Option<String>,
	pub tagged_items: Option<i64>,
	pub url: Option<String>,
	pub weight: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TagRequest {
	pub color: String,
	pub description: String,
	pub name: String,
	pub object_types: Vec<String>,
	pub slug: String,
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaggedItem {
	pub display: Option<String>,
	pub id: i64,
	pub object: Option<serde_json::Value>,
	pub object_id: Option<i32>,
	pub object_type: Option<String>,
	pub tag: Option<BriefTag>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tenant {
	pub circuit_count: Option<i64>,
	pub cluster_count: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub group: Option<BriefTenantGroup>,
	pub id: i64,
	pub ipaddress_count: Option<i64>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub prefix_count: Option<i64>,
	pub rack_count: Option<i64>,
	pub site_count: Option<i64>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub virtualmachine_count: Option<i64>,
	pub vlan_count: Option<i64>,
	pub vrf_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenantGroup {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedTenantGroup>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant_count: Option<i64>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenantGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<NestedTenantGroupRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenantRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Token {
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub expires: Option<String>,
	pub id: i64,
	pub last_used: Option<String>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
	/// Permit create/update/delete operations using this key
	pub write_enabled: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenProvision {
	pub created: Option<String>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub expires: Option<String>,
	pub id: i64,
	pub key: Option<String>,
	pub last_used: Option<String>,
	pub url: Option<String>,
	pub user: Option<BriefUser>,
	/// Permit create/update/delete operations using this key
	pub write_enabled: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenProvisionRequest {
	pub description: String,
	pub expires: Option<String>,
	pub password: String,
	pub username: String,
	/// Permit create/update/delete operations using this key
	pub write_enabled: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenRequest {
	pub description: String,
	pub expires: Option<String>,
	pub key: String,
	pub last_used: Option<String>,
	pub user: serde_json::Value,
	/// Permit create/update/delete operations using this key
	pub write_enabled: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tunnel {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub encapsulation: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub group: Option<BriefTunnelGroup>,
	pub id: i64,
	pub ipsec_profile: Option<BriefIPSecProfile>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub terminations_count: Option<i64>,
	pub tunnel_id: Option<u64>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelGroup {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub tunnel_count: Option<i64>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelGroupRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	/// * `wireguard` - WireGuard
	/// * `openvpn` - OpenVPN
	/// * `l2tp` - L2TP
	/// * `pptp` - PPTP
	pub encapsulation: String,
	pub group: Option<serde_json::Value>,
	pub ipsec_profile: Option<serde_json::Value>,
	pub name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub tunnel_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelTermination {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub outside_ip: Option<BriefIPAddress>,
	pub role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub termination: Option<serde_json::Value>,
	pub termination_id: Option<u64>,
	pub termination_type: Option<String>,
	pub tunnel: Option<BriefTunnel>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelTerminationRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub outside_ip: Option<serde_json::Value>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub tags: Vec<NestedTagRequest>,
	pub termination_id: Option<u64>,
	pub termination_type: String,
	pub tunnel: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
	pub date_joined: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub email: Option<String>,
	pub first_name: Option<String>,
	pub groups: Option<Vec<Group>>,
	pub id: i64,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	pub is_active: Option<bool>,
	/// Designates whether the user can log into this admin site.
	pub is_staff: Option<bool>,
	pub last_login: Option<String>,
	pub last_name: Option<String>,
	pub permissions: Option<Vec<ObjectPermission>>,
	pub url: Option<String>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserRequest {
	pub date_joined: String,
	pub email: String,
	pub first_name: String,
	pub groups: Vec<i64>,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	pub is_active: bool,
	/// Designates whether the user can log into this admin site.
	pub is_staff: bool,
	pub last_login: Option<String>,
	pub last_name: String,
	pub password: String,
	pub permissions: Vec<i64>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLAN {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub group: Option<BriefVLANGroup>,
	pub id: i64,
	pub l2vpn_termination: Option<BriefL2VPNTermination>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub prefix_count: Option<i64>,
	pub qinq_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub qinq_svlan: Option<NestedVLAN>,
	pub role: Option<BriefRole>,
	pub site: Option<BriefSite>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	/// Numeric VLAN ID (1-4094)
	pub vid: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANGroup {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub scope: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub utilization: Option<String>,
	pub vid_ranges: Option<Vec<IntegerRange>>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANGroupRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vid_ranges: Vec<IntegerRangeRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub name: String,
	/// * `svlan` - Service
	/// * `cvlan` - Customer
	pub qinq_role: Option<String>,
	pub qinq_svlan: Option<NestedVLANRequest>,
	pub role: Option<serde_json::Value>,
	pub site: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationPolicy {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	pub name: Option<String>,
	pub rules: Option<Vec<VLANTranslationRule>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationPolicyRequest {
	pub description: String,
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationRule {
	pub description: Option<String>,
	pub display: Option<String>,
	pub id: i64,
	/// Numeric VLAN ID (1-4094)
	pub local_vid: Option<u16>,
	pub policy: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	pub remote_vid: Option<u16>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationRuleRequest {
	pub description: String,
	/// Numeric VLAN ID (1-4094)
	pub local_vid: u16,
	pub policy: i64,
	/// Numeric VLAN ID (1-4094)
	pub remote_vid: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VMInterface {
	pub bridge: Option<NestedVMInterface>,
	pub count_fhrp_groups: Option<i64>,
	pub count_ipaddresses: Option<i64>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub enabled: Option<bool>,
	pub id: i64,
	pub l2vpn_termination: Option<BriefL2VPNTermination>,
	pub last_updated: Option<String>,
	pub mac_address: Option<String>,
	pub mac_addresses: Option<Vec<BriefMACAddress>>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub mtu: Option<u32>,
	pub name: Option<String>,
	pub parent: Option<NestedVMInterface>,
	pub primary_mac_address: Option<BriefMACAddress>,
	pub qinq_svlan: Option<BriefVLAN>,
	pub tagged_vlans: Option<Vec<VLAN>>,
	pub tags: Option<Vec<NestedTag>>,
	pub untagged_vlan: Option<BriefVLAN>,
	pub url: Option<String>,
	pub virtual_machine: Option<BriefVirtualMachine>,
	pub vlan_translation_policy: Option<BriefVLANTranslationPolicy>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VMInterfaceRequest {
	pub bridge: Option<NestedVMInterfaceRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub enabled: bool,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: String,
	pub mtu: Option<u32>,
	pub name: String,
	pub parent: Option<NestedVMInterfaceRequest>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub virtual_machine: serde_json::Value,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VRF {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	pub enforce_unique: Option<bool>,
	pub export_targets: Option<Vec<RouteTarget>>,
	pub id: i64,
	pub import_targets: Option<Vec<RouteTarget>>,
	pub ipaddress_count: Option<i64>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub prefix_count: Option<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VRFRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	pub enforce_unique: bool,
	pub export_targets: Vec<i64>,
	pub import_targets: Vec<i64>,
	pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualChassis {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub domain: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub master: Option<NestedDevice>,
	pub member_count: Option<i64>,
	pub members: Option<Vec<NestedDevice>>,
	pub name: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualChassisRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub domain: String,
	pub master: Option<NestedDeviceRequest>,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuit {
	/// Unique circuit ID
	pub cid: Option<String>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub provider_account: Option<BriefProviderAccount>,
	pub provider_network: Option<BriefProviderNetwork>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub r#type: Option<BriefVirtualCircuitType>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub provider_account: Option<serde_json::Value>,
	pub provider_network: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitTermination {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub interface: Option<BriefInterface>,
	pub last_updated: Option<String>,
	pub role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub virtual_circuit: Option<BriefVirtualCircuit>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitTerminationRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub interface: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub tags: Vec<NestedTagRequest>,
	pub virtual_circuit: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitType {
	pub color: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub virtual_circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitTypeRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDeviceContext {
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub identifier: Option<u16>,
	pub interface_count: Option<i64>,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDeviceContextRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	pub identifier: Option<u16>,
	pub name: String,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDisk {
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub size: Option<u32>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub virtual_machine: Option<BriefVirtualMachine>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDiskRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub size: u32,
	pub tags: Vec<NestedTagRequest>,
	pub virtual_machine: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualMachineWithConfigContext {
	pub cluster: Option<BriefCluster>,
	pub comments: Option<String>,
	pub config_context: Option<serde_json::Value>,
	pub config_template: Option<BriefConfigTemplate>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub device: Option<BriefDevice>,
	pub disk: Option<u32>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub interface_count: Option<i64>,
	pub last_updated: Option<String>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub memory: Option<u32>,
	pub name: Option<String>,
	pub platform: Option<BriefPlatform>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub role: Option<BriefDeviceRole>,
	pub serial: Option<String>,
	pub site: Option<BriefSite>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vcpus: Option<f64>,
	pub virtual_disk_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualMachineWithConfigContextRequest {
	pub cluster: Option<serde_json::Value>,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: Option<serde_json::Value>,
	pub disk: Option<u32>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub memory: Option<u32>,
	pub name: String,
	pub platform: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	pub site: Option<serde_json::Value>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	/// * `paused` - Paused
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vcpus: Option<f64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Webhook {
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	pub additional_headers: Option<String>,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	pub body_template: Option<String>,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	pub ca_file_path: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	pub http_content_type: Option<String>,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	pub http_method: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	pub payload_url: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	pub secret: Option<String>,
	/// Enable SSL certificate verification. Disable with caution!
	pub ssl_verification: Option<bool>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WebhookRequest {
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	pub additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	pub body_template: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	pub ca_file_path: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	pub http_content_type: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	pub http_method: String,
	pub name: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	pub payload_url: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	pub secret: String,
	/// Enable SSL certificate verification. Disable with caution!
	pub ssl_verification: bool,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLAN {
	pub auth_cipher: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub auth_psk: Option<String>,
	pub auth_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub group: Option<BriefWirelessLANGroup>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub scope: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub ssid: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
	pub vlan: Option<BriefVLAN>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLANGroup {
	pub _depth: Option<i64>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub id: i64,
	pub last_updated: Option<String>,
	pub name: Option<String>,
	pub parent: Option<NestedWirelessLANGroup>,
	pub slug: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub url: Option<String>,
	pub wirelesslan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLANGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<NestedWirelessLANGroupRequest>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLANRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: String,
	pub auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub ssid: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLink {
	pub auth_cipher: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub auth_psk: Option<String>,
	pub auth_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub distance: Option<f64>,
	pub distance_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub id: i64,
	pub interface_a: Option<BriefInterface>,
	pub interface_b: Option<BriefInterface>,
	pub last_updated: Option<String>,
	pub ssid: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub tenant: Option<BriefTenant>,
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLinkRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: String,
	pub auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub interface_a: serde_json::Value,
	pub interface_b: serde_json::Value,
	pub ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableAggregateRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub date_added: Option<String>,
	pub description: String,
	pub prefix: String,
	pub rir: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCableRequest {
	pub a_terminations: Vec<GenericObjectRequest>,
	pub b_terminations: Vec<GenericObjectRequest>,
	pub color: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub label: String,
	pub length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	pub length_unit: Option<String>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Single-mode Fiber
	/// * `smf-os1` - Single-mode Fiber (OS1)
	/// * `smf-os2` - Single-mode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `power` - Power
	/// * `usb` - USB
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCircuitGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub member_id: u64,
	pub member_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: Option<String>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCircuitRequest {
	pub assignments: Vec<BriefCircuitGroupAssignmentSerializer_Request>,
	/// Unique circuit ID
	pub cid: String,
	pub comments: String,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub install_date: Option<String>,
	pub provider: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub termination_date: Option<String>,
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableClusterRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub name: String,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsolePortRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Port speed in bits per second
	/// 
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	pub speed: Option<u32>,
	pub tags: Vec<NestedTagRequest>,
	/// Physical port type
	/// 
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsolePortTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsoleServerPortRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Port speed in bits per second
	/// 
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	pub speed: Option<u32>,
	pub tags: Vec<NestedTagRequest>,
	/// Physical port type
	/// 
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsoleServerPortTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `de-9` - DE-9
	/// * `db-25` - DB-25
	/// * `rj-11` - RJ-11
	/// * `rj-12` - RJ-12
	/// * `rj-45` - RJ-45
	/// * `mini-din-8` - Mini-DIN 8
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableContactAssignmentRequest {
	pub contact: serde_json::Value,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub object_id: u64,
	pub object_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: Option<String>,
	pub role: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableContactGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCustomFieldChoiceSetRequest {
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	pub base_choices: Option<String>,
	pub description: String,
	pub extra_choices: Vec<Vec<serde_json::Value>>,
	pub name: String,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCustomFieldRequest {
	pub choice_set: Option<serde_json::Value>,
	pub comments: String,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	pub description: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	pub filter_logic: String,
	/// Custom fields within the same group will be displayed together
	pub group_name: String,
	/// Replicate this value when cloning objects
	pub is_cloneable: bool,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: String,
	/// Internal field name
	pub name: String,
	pub object_types: Vec<String>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	pub related_object_type: Option<String>,
	/// This field is required when creating new objects or editing an existing object.
	pub required: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: u16,
	/// The type of data this custom field holds
	/// 
	/// * `text` - Text
	/// * `longtext` - Text (long)
	/// * `integer` - Integer
	/// * `decimal` - Decimal
	/// * `boolean` - Boolean (true/false)
	/// * `date` - Date
	/// * `datetime` - Date & time
	/// * `url` - URL
	/// * `json` - JSON
	/// * `select` - Selection
	/// * `multiselect` - Multiple selection
	/// * `object` - Object
	/// * `multiobject` - Multiple objects
	pub r#type: String,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	pub ui_editable: String,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	pub ui_visible: String,
	/// The value of this field must be unique for the assigned object
	pub unique: bool,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<f64>,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<f64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: String,
	/// Fields with higher weights appear lower in a form.
	pub weight: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDataSourceRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub enabled: bool,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: String,
	pub name: String,
	pub parameters: Option<serde_json::Value>,
	pub source_url: String,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDeviceRoleRequest {
	pub color: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
	/// Virtual machines may be assigned to this role
	pub vm_role: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDeviceTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `rear-to-side` - Rear to side
	/// * `bottom-to-top` - Bottom to top
	/// * `top-to-bottom` - Top to bottom
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	pub airflow: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub default_platform: Option<serde_json::Value>,
	pub description: String,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: bool,
	pub front_image: Option<String>,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: bool,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub rear_image: Option<String>,
	pub slug: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	pub subdevice_role: Option<String>,
	pub tags: Vec<NestedTagRequest>,
	pub u_height: f64,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDeviceWithConfigContextRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `rear-to-side` - Rear to side
	/// * `bottom-to-top` - Bottom to top
	/// * `top-to-bottom` - Top to bottom
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	pub airflow: Option<String>,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub cluster: Option<serde_json::Value>,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device_type: serde_json::Value,
	/// * `front` - Front
	/// * `rear` - Rear
	pub face: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub location: Option<serde_json::Value>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub name: Option<String>,
	pub oob_ip: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	pub position: Option<f64>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub rack: Option<serde_json::Value>,
	pub role: serde_json::Value,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: String,
	pub site: serde_json::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub virtual_chassis: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableEventRuleRequest {
	pub action_object_id: Option<u64>,
	pub action_object_type: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	/// * `notification` - Notification
	pub action_type: String,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub enabled: bool,
	/// The types of event which will trigger this rule.
	pub event_types: Vec<String>,
	pub name: String,
	pub object_types: Vec<String>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableFrontPortRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub rear_port: i64,
	/// Mapped position on corresponding rear port
	pub rear_port_position: u16,
	pub tags: Vec<NestedTagRequest>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableFrontPortTemplateRequest {
	pub color: String,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub rear_port: serde_json::Value,
	pub rear_port_position: u16,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIKEPolicyRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	pub mode: Option<String>,
	pub name: String,
	pub preshared_key: String,
	pub proposals: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	pub version: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIKEProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: Option<String>,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	pub authentication_method: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: String,
	/// Diffie-Hellman group ID
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
	/// * `15` - Group 15
	/// * `16` - Group 16
	/// * `17` - Group 17
	/// * `18` - Group 18
	/// * `19` - Group 19
	/// * `20` - Group 20
	/// * `21` - Group 21
	/// * `22` - Group 22
	/// * `23` - Group 23
	/// * `24` - Group 24
	/// * `25` - Group 25
	/// * `26` - Group 26
	/// * `27` - Group 27
	/// * `28` - Group 28
	/// * `29` - Group 29
	/// * `30` - Group 30
	/// * `31` - Group 31
	/// * `32` - Group 32
	/// * `33` - Group 33
	/// * `34` - Group 34
	pub group: u16,
	pub name: String,
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPAddressRequest {
	pub address: String,
	pub assigned_object_id: Option<u64>,
	pub assigned_object_type: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: String,
	/// The IP for which this address is the "outside" IP
	pub nat_inside: Option<i64>,
	/// The functional role of this IP
	/// 
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	pub role: Option<String>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPRangeRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub end_address: String,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: bool,
	/// Report space as fully utilized
	pub mark_utilized: bool,
	pub role: Option<serde_json::Value>,
	pub start_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPSecPolicyRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	/// Diffie-Hellman group for Perfect Forward Secrecy
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
	/// * `15` - Group 15
	/// * `16` - Group 16
	/// * `17` - Group 17
	/// * `18` - Group 18
	/// * `19` - Group 19
	/// * `20` - Group 20
	/// * `21` - Group 21
	/// * `22` - Group 22
	/// * `23` - Group 23
	/// * `24` - Group 24
	/// * `25` - Group 25
	/// * `26` - Group 26
	/// * `27` - Group 27
	/// * `28` - Group 28
	/// * `29` - Group 29
	/// * `30` - Group 30
	/// * `31` - Group 31
	/// * `32` - Group 32
	/// * `33` - Group 33
	/// * `34` - Group 34
	pub pfs_group: Option<u16>,
	pub proposals: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPSecProfileRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub ike_policy: serde_json::Value,
	pub ipsec_policy: serde_json::Value,
	/// * `esp` - ESP
	/// * `ah` - AH
	pub mode: String,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPSecProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: Option<String>,
	pub name: String,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableInterfaceRequest {
	pub bridge: Option<i64>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	pub duplex: Option<String>,
	pub enabled: bool,
	/// Physical label
	pub label: String,
	pub lag: Option<i64>,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// This interface is used only for out-of-band management
	pub mgmt_only: bool,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: Option<String>,
	pub module: Option<serde_json::Value>,
	pub mtu: Option<u32>,
	pub name: String,
	pub parent: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	pub poe_mode: Option<String>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	pub poe_type: Option<String>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub qinq_svlan: Option<serde_json::Value>,
	/// * `2.4g-1-2412-22` - 1 (2412 MHz)
	/// * `2.4g-2-2417-22` - 2 (2417 MHz)
	/// * `2.4g-3-2422-22` - 3 (2422 MHz)
	/// * `2.4g-4-2427-22` - 4 (2427 MHz)
	/// * `2.4g-5-2432-22` - 5 (2432 MHz)
	/// * `2.4g-6-2437-22` - 6 (2437 MHz)
	/// * `2.4g-7-2442-22` - 7 (2442 MHz)
	/// * `2.4g-8-2447-22` - 8 (2447 MHz)
	/// * `2.4g-9-2452-22` - 9 (2452 MHz)
	/// * `2.4g-10-2457-22` - 10 (2457 MHz)
	/// * `2.4g-11-2462-22` - 11 (2462 MHz)
	/// * `2.4g-12-2467-22` - 12 (2467 MHz)
	/// * `2.4g-13-2472-22` - 13 (2472 MHz)
	/// * `5g-32-5160-20` - 32 (5160/20 MHz)
	/// * `5g-34-5170-40` - 34 (5170/40 MHz)
	/// * `5g-36-5180-20` - 36 (5180/20 MHz)
	/// * `5g-38-5190-40` - 38 (5190/40 MHz)
	/// * `5g-40-5200-20` - 40 (5200/20 MHz)
	/// * `5g-42-5210-80` - 42 (5210/80 MHz)
	/// * `5g-44-5220-20` - 44 (5220/20 MHz)
	/// * `5g-46-5230-40` - 46 (5230/40 MHz)
	/// * `5g-48-5240-20` - 48 (5240/20 MHz)
	/// * `5g-50-5250-160` - 50 (5250/160 MHz)
	/// * `5g-52-5260-20` - 52 (5260/20 MHz)
	/// * `5g-54-5270-40` - 54 (5270/40 MHz)
	/// * `5g-56-5280-20` - 56 (5280/20 MHz)
	/// * `5g-58-5290-80` - 58 (5290/80 MHz)
	/// * `5g-60-5300-20` - 60 (5300/20 MHz)
	/// * `5g-62-5310-40` - 62 (5310/40 MHz)
	/// * `5g-64-5320-20` - 64 (5320/20 MHz)
	/// * `5g-100-5500-20` - 100 (5500/20 MHz)
	/// * `5g-102-5510-40` - 102 (5510/40 MHz)
	/// * `5g-104-5520-20` - 104 (5520/20 MHz)
	/// * `5g-106-5530-80` - 106 (5530/80 MHz)
	/// * `5g-108-5540-20` - 108 (5540/20 MHz)
	/// * `5g-110-5550-40` - 110 (5550/40 MHz)
	/// * `5g-112-5560-20` - 112 (5560/20 MHz)
	/// * `5g-114-5570-160` - 114 (5570/160 MHz)
	/// * `5g-116-5580-20` - 116 (5580/20 MHz)
	/// * `5g-118-5590-40` - 118 (5590/40 MHz)
	/// * `5g-120-5600-20` - 120 (5600/20 MHz)
	/// * `5g-122-5610-80` - 122 (5610/80 MHz)
	/// * `5g-124-5620-20` - 124 (5620/20 MHz)
	/// * `5g-126-5630-40` - 126 (5630/40 MHz)
	/// * `5g-128-5640-20` - 128 (5640/20 MHz)
	/// * `5g-132-5660-20` - 132 (5660/20 MHz)
	/// * `5g-134-5670-40` - 134 (5670/40 MHz)
	/// * `5g-136-5680-20` - 136 (5680/20 MHz)
	/// * `5g-138-5690-80` - 138 (5690/80 MHz)
	/// * `5g-140-5700-20` - 140 (5700/20 MHz)
	/// * `5g-142-5710-40` - 142 (5710/40 MHz)
	/// * `5g-144-5720-20` - 144 (5720/20 MHz)
	/// * `5g-149-5745-20` - 149 (5745/20 MHz)
	/// * `5g-151-5755-40` - 151 (5755/40 MHz)
	/// * `5g-153-5765-20` - 153 (5765/20 MHz)
	/// * `5g-155-5775-80` - 155 (5775/80 MHz)
	/// * `5g-157-5785-20` - 157 (5785/20 MHz)
	/// * `5g-159-5795-40` - 159 (5795/40 MHz)
	/// * `5g-161-5805-20` - 161 (5805/20 MHz)
	/// * `5g-163-5815-160` - 163 (5815/160 MHz)
	/// * `5g-165-5825-20` - 165 (5825/20 MHz)
	/// * `5g-167-5835-40` - 167 (5835/40 MHz)
	/// * `5g-169-5845-20` - 169 (5845/20 MHz)
	/// * `5g-171-5855-80` - 171 (5855/80 MHz)
	/// * `5g-173-5865-20` - 173 (5865/20 MHz)
	/// * `5g-175-5875-40` - 175 (5875/40 MHz)
	/// * `5g-177-5885-20` - 177 (5885/20 MHz)
	/// * `6g-1-5955-20` - 1 (5955/20 MHz)
	/// * `6g-3-5965-40` - 3 (5965/40 MHz)
	/// * `6g-5-5975-20` - 5 (5975/20 MHz)
	/// * `6g-7-5985-80` - 7 (5985/80 MHz)
	/// * `6g-9-5995-20` - 9 (5995/20 MHz)
	/// * `6g-11-6005-40` - 11 (6005/40 MHz)
	/// * `6g-13-6015-20` - 13 (6015/20 MHz)
	/// * `6g-15-6025-160` - 15 (6025/160 MHz)
	/// * `6g-17-6035-20` - 17 (6035/20 MHz)
	/// * `6g-19-6045-40` - 19 (6045/40 MHz)
	/// * `6g-21-6055-20` - 21 (6055/20 MHz)
	/// * `6g-23-6065-80` - 23 (6065/80 MHz)
	/// * `6g-25-6075-20` - 25 (6075/20 MHz)
	/// * `6g-27-6085-40` - 27 (6085/40 MHz)
	/// * `6g-29-6095-20` - 29 (6095/20 MHz)
	/// * `6g-31-6105-320` - 31 (6105/320 MHz)
	/// * `6g-33-6115-20` - 33 (6115/20 MHz)
	/// * `6g-35-6125-40` - 35 (6125/40 MHz)
	/// * `6g-37-6135-20` - 37 (6135/20 MHz)
	/// * `6g-39-6145-80` - 39 (6145/80 MHz)
	/// * `6g-41-6155-20` - 41 (6155/20 MHz)
	/// * `6g-43-6165-40` - 43 (6165/40 MHz)
	/// * `6g-45-6175-20` - 45 (6175/20 MHz)
	/// * `6g-47-6185-160` - 47 (6185/160 MHz)
	/// * `6g-49-6195-20` - 49 (6195/20 MHz)
	/// * `6g-51-6205-40` - 51 (6205/40 MHz)
	/// * `6g-53-6215-20` - 53 (6215/20 MHz)
	/// * `6g-55-6225-80` - 55 (6225/80 MHz)
	/// * `6g-57-6235-20` - 57 (6235/20 MHz)
	/// * `6g-59-6245-40` - 59 (6245/40 MHz)
	/// * `6g-61-6255-20` - 61 (6255/20 MHz)
	/// * `6g-65-6275-20` - 65 (6275/20 MHz)
	/// * `6g-67-6285-40` - 67 (6285/40 MHz)
	/// * `6g-69-6295-20` - 69 (6295/20 MHz)
	/// * `6g-71-6305-80` - 71 (6305/80 MHz)
	/// * `6g-73-6315-20` - 73 (6315/20 MHz)
	/// * `6g-75-6325-40` - 75 (6325/40 MHz)
	/// * `6g-77-6335-20` - 77 (6335/20 MHz)
	/// * `6g-79-6345-160` - 79 (6345/160 MHz)
	/// * `6g-81-6355-20` - 81 (6355/20 MHz)
	/// * `6g-83-6365-40` - 83 (6365/40 MHz)
	/// * `6g-85-6375-20` - 85 (6375/20 MHz)
	/// * `6g-87-6385-80` - 87 (6385/80 MHz)
	/// * `6g-89-6395-20` - 89 (6395/20 MHz)
	/// * `6g-91-6405-40` - 91 (6405/40 MHz)
	/// * `6g-93-6415-20` - 93 (6415/20 MHz)
	/// * `6g-95-6425-320` - 95 (6425/320 MHz)
	/// * `6g-97-6435-20` - 97 (6435/20 MHz)
	/// * `6g-99-6445-40` - 99 (6445/40 MHz)
	/// * `6g-101-6455-20` - 101 (6455/20 MHz)
	/// * `6g-103-6465-80` - 103 (6465/80 MHz)
	/// * `6g-105-6475-20` - 105 (6475/20 MHz)
	/// * `6g-107-6485-40` - 107 (6485/40 MHz)
	/// * `6g-109-6495-20` - 109 (6495/20 MHz)
	/// * `6g-111-6505-160` - 111 (6505/160 MHz)
	/// * `6g-113-6515-20` - 113 (6515/20 MHz)
	/// * `6g-115-6525-40` - 115 (6525/40 MHz)
	/// * `6g-117-6535-20` - 117 (6535/20 MHz)
	/// * `6g-119-6545-80` - 119 (6545/80 MHz)
	/// * `6g-121-6555-20` - 121 (6555/20 MHz)
	/// * `6g-123-6565-40` - 123 (6565/40 MHz)
	/// * `6g-125-6575-20` - 125 (6575/20 MHz)
	/// * `6g-129-6595-20` - 129 (6595/20 MHz)
	/// * `6g-131-6605-40` - 131 (6605/40 MHz)
	/// * `6g-133-6615-20` - 133 (6615/20 MHz)
	/// * `6g-135-6625-80` - 135 (6625/80 MHz)
	/// * `6g-137-6635-20` - 137 (6635/20 MHz)
	/// * `6g-139-6645-40` - 139 (6645/40 MHz)
	/// * `6g-141-6655-20` - 141 (6655/20 MHz)
	/// * `6g-143-6665-160` - 143 (6665/160 MHz)
	/// * `6g-145-6675-20` - 145 (6675/20 MHz)
	/// * `6g-147-6685-40` - 147 (6685/40 MHz)
	/// * `6g-149-6695-20` - 149 (6695/20 MHz)
	/// * `6g-151-6705-80` - 151 (6705/80 MHz)
	/// * `6g-153-6715-20` - 153 (6715/20 MHz)
	/// * `6g-155-6725-40` - 155 (6725/40 MHz)
	/// * `6g-157-6735-20` - 157 (6735/20 MHz)
	/// * `6g-159-6745-320` - 159 (6745/320 MHz)
	/// * `6g-161-6755-20` - 161 (6755/20 MHz)
	/// * `6g-163-6765-40` - 163 (6765/40 MHz)
	/// * `6g-165-6775-20` - 165 (6775/20 MHz)
	/// * `6g-167-6785-80` - 167 (6785/80 MHz)
	/// * `6g-169-6795-20` - 169 (6795/20 MHz)
	/// * `6g-171-6805-40` - 171 (6805/40 MHz)
	/// * `6g-173-6815-20` - 173 (6815/20 MHz)
	/// * `6g-175-6825-160` - 175 (6825/160 MHz)
	/// * `6g-177-6835-20` - 177 (6835/20 MHz)
	/// * `6g-179-6845-40` - 179 (6845/40 MHz)
	/// * `6g-181-6855-20` - 181 (6855/20 MHz)
	/// * `6g-183-6865-80` - 183 (6865/80 MHz)
	/// * `6g-185-6875-20` - 185 (6875/20 MHz)
	/// * `6g-187-6885-40` - 187 (6885/40 MHz)
	/// * `6g-189-6895-20` - 189 (6895/20 MHz)
	/// * `6g-193-6915-20` - 193 (6915/20 MHz)
	/// * `6g-195-6925-40` - 195 (6925/40 MHz)
	/// * `6g-197-6935-20` - 197 (6935/20 MHz)
	/// * `6g-199-6945-80` - 199 (6945/80 MHz)
	/// * `6g-201-6955-20` - 201 (6955/20 MHz)
	/// * `6g-203-6965-40` - 203 (6965/40 MHz)
	/// * `6g-205-6975-20` - 205 (6975/20 MHz)
	/// * `6g-207-6985-160` - 207 (6985/160 MHz)
	/// * `6g-209-6995-20` - 209 (6995/20 MHz)
	/// * `6g-211-7005-40` - 211 (7005/40 MHz)
	/// * `6g-213-7015-20` - 213 (7015/20 MHz)
	/// * `6g-215-7025-80` - 215 (7025/80 MHz)
	/// * `6g-217-7035-20` - 217 (7035/20 MHz)
	/// * `6g-219-7045-40` - 219 (7045/40 MHz)
	/// * `6g-221-7055-20` - 221 (7055/20 MHz)
	/// * `6g-225-7075-20` - 225 (7075/20 MHz)
	/// * `6g-227-7085-40` - 227 (7085/40 MHz)
	/// * `6g-229-7095-20` - 229 (7095/20 MHz)
	/// * `6g-233-7115-20` - 233 (7115/20 MHz)
	/// * `60g-1-58320-2160` - 1 (58.32/2.16 GHz)
	/// * `60g-2-60480-2160` - 2 (60.48/2.16 GHz)
	/// * `60g-3-62640-2160` - 3 (62.64/2.16 GHz)
	/// * `60g-4-64800-2160` - 4 (64.80/2.16 GHz)
	/// * `60g-5-66960-2160` - 5 (66.96/2.16 GHz)
	/// * `60g-6-69120-2160` - 6 (69.12/2.16 GHz)
	/// * `60g-9-59400-4320` - 9 (59.40/4.32 GHz)
	/// * `60g-10-61560-4320` - 10 (61.56/4.32 GHz)
	/// * `60g-11-63720-4320` - 11 (63.72/4.32 GHz)
	/// * `60g-12-65880-4320` - 12 (65.88/4.32 GHz)
	/// * `60g-13-68040-4320` - 13 (68.04/4.32 GHz)
	/// * `60g-17-60480-6480` - 17 (60.48/6.48 GHz)
	/// * `60g-18-62640-6480` - 18 (62.64/6.48 GHz)
	/// * `60g-19-64800-6480` - 19 (64.80/6.48 GHz)
	/// * `60g-20-66960-6480` - 20 (66.96/6.48 GHz)
	/// * `60g-25-61560-6480` - 25 (61.56/8.64 GHz)
	/// * `60g-26-63720-6480` - 26 (63.72/8.64 GHz)
	/// * `60g-27-65880-6480` - 27 (65.88/8.64 GHz)
	pub rf_channel: Option<String>,
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: Option<String>,
	pub speed: Option<u32>,
	pub tagged_vlans: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub tx_power: Option<i8>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME)
	/// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
	/// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
	/// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
	/// * `1000base-cx` - 1000BASE-CX (1GE DAC)
	/// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
	/// * `1000base-ex` - 1000BASE-EX (1GE)
	/// * `1000base-lsx` - 1000BASE-LSX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `1000base-zx` - 1000BASE-ZX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
	/// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
	/// * `10gbase-er` - 10GBASE-ER (10GE)
	/// * `10gbase-lr` - 10GBASE-LR (10GE)
	/// * `10gbase-lrm` - 10GBASE-LRM (10GE)
	/// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
	/// * `10gbase-sr` - 10GBASE-SR (10GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-zr` - 10GBASE-ZR (10GE)
	/// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
	/// * `25gbase-er` - 25GBASE-ER (25GE)
	/// * `25gbase-lr` - 25GBASE-LR (25GE)
	/// * `25gbase-sr` - 25GBASE-SR (25GE)
	/// * `25gbase-t` - 25GBASE-T (25GE)
	/// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
	/// * `40gbase-er4` - 40GBASE-ER4 (40GE)
	/// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
	/// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
	/// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
	/// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
	/// * `50gbase-er` - 50GBASE-ER (50GE)
	/// * `50gbase-fr` - 50GBASE-FR (50GE)
	/// * `50gbase-lr` - 50GBASE-LR (50GE)
	/// * `50gbase-sr` - 50GBASE-SR (50GE)
	/// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
	/// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
	/// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
	/// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
	/// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
	/// * `100gbase-dr` - 100GBASE-DR (100GE)
	/// * `100gbase-er4` - 100GBASE-ER4 (100GE)
	/// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
	/// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
	/// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
	/// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
	/// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
	/// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
	/// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
	/// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
	/// * `100gbase-zr` - 100GBASE-ZR (100GE)
	/// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
	/// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
	/// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
	/// * `200gbase-er4` - 200GBASE-ER4 (200GE)
	/// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
	/// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
	/// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
	/// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
	/// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
	/// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
	/// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
	/// * `400gbase-er8` - 400GBASE-ER8 (400GE)
	/// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
	/// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
	/// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
	/// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
	/// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
	/// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
	/// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
	/// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
	/// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
	/// * `400gbase-zr` - 400GBASE-ZR (400GE)
	/// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
	/// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
	/// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
	/// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `1000base-kx` - 1000BASE-KX (1GE)
	/// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
	/// * `5gbase-kr` - 5GBASE-KR (5GE)
	/// * `10gbase-kr` - 10GBASE-KR (10GE)
	/// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
	/// * `25gbase-kr` - 25GBASE-KR (25GE)
	/// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
	/// * `50gbase-kr` - 50GBASE-KR (50GE)
	/// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
	/// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
	/// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
	/// * `ieee802.11a` - IEEE 802.11a
	/// * `ieee802.11g` - IEEE 802.11b/g
	/// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
	/// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
	/// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
	/// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
	/// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
	/// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
	/// * `4g` - 4G
	/// * `5g` - 5G
	/// * `sonet-oc3` - OC-3/STM-1
	/// * `sonet-oc12` - OC-12/STM-4
	/// * `sonet-oc48` - OC-48/STM-16
	/// * `sonet-oc192` - OC-192/STM-64
	/// * `sonet-oc768` - OC-768/STM-256
	/// * `sonet-oc1920` - OC-1920/STM-640
	/// * `sonet-oc3840` - OC-3840/STM-1234
	/// * `1gfc-sfp` - SFP (1GFC)
	/// * `2gfc-sfp` - SFP (2GFC)
	/// * `4gfc-sfp` - SFP (4GFC)
	/// * `8gfc-sfpp` - SFP+ (8GFC)
	/// * `16gfc-sfpp` - SFP+ (16GFC)
	/// * `32gfc-sfp28` - SFP28 (32GFC)
	/// * `32gfc-sfpp` - SFP+ (32GFC)
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
	/// * `64gfc-sfpdd` - SFP-DD (64GFC)
	/// * `64gfc-sfpp` - SFP+ (64GFC)
	/// * `128gfc-qsfp28` - QSFP28 (128GFC)
	/// * `infiniband-sdr` - SDR (2 Gbps)
	/// * `infiniband-ddr` - DDR (4 Gbps)
	/// * `infiniband-qdr` - QDR (8 Gbps)
	/// * `infiniband-fdr10` - FDR10 (10 Gbps)
	/// * `infiniband-fdr` - FDR (13.5 Gbps)
	/// * `infiniband-edr` - EDR (25 Gbps)
	/// * `infiniband-hdr` - HDR (50 Gbps)
	/// * `infiniband-ndr` - NDR (100 Gbps)
	/// * `infiniband-xdr` - XDR (250 Gbps)
	/// * `t1` - T1 (1.544 Mbps)
	/// * `e1` - E1 (2.048 Mbps)
	/// * `t3` - T3 (45 Mbps)
	/// * `e3` - E3 (34 Mbps)
	/// * `xdsl` - xDSL
	/// * `docsis` - DOCSIS
	/// * `moca` - MoCA
	/// * `bpon` - BPON (622 Mbps / 155 Mbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `25g-pon` - 25G-PON (25 Gbps)
	/// * `50g-pon` - 50G-PON (50 Gbps)
	/// * `cisco-stackwise` - Cisco StackWise
	/// * `cisco-stackwise-plus` - Cisco StackWise Plus
	/// * `cisco-flexstack` - Cisco FlexStack
	/// * `cisco-flexstack-plus` - Cisco FlexStack Plus
	/// * `cisco-stackwise-80` - Cisco StackWise-80
	/// * `cisco-stackwise-160` - Cisco StackWise-160
	/// * `cisco-stackwise-320` - Cisco StackWise-320
	/// * `cisco-stackwise-480` - Cisco StackWise-480
	/// * `cisco-stackwise-1t` - Cisco StackWise-1T
	/// * `juniper-vcp` - Juniper VCP
	/// * `extreme-summitstack` - Extreme SummitStack
	/// * `extreme-summitstack-128` - Extreme SummitStack-128
	/// * `extreme-summitstack-256` - Extreme SummitStack-256
	/// * `extreme-summitstack-512` - Extreme SummitStack-512
	/// * `other` - Other
	pub r#type: String,
	pub untagged_vlan: Option<serde_json::Value>,
	pub vdcs: Vec<i64>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
	pub wireless_lans: Vec<i64>,
	pub wwn: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableInterfaceTemplateRequest {
	pub bridge: Option<i64>,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	pub enabled: bool,
	/// Physical label
	pub label: String,
	pub mgmt_only: bool,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	pub poe_mode: Option<String>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	pub poe_type: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: Option<String>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME)
	/// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
	/// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
	/// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
	/// * `1000base-cx` - 1000BASE-CX (1GE DAC)
	/// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
	/// * `1000base-ex` - 1000BASE-EX (1GE)
	/// * `1000base-lsx` - 1000BASE-LSX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `1000base-zx` - 1000BASE-ZX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
	/// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
	/// * `10gbase-er` - 10GBASE-ER (10GE)
	/// * `10gbase-lr` - 10GBASE-LR (10GE)
	/// * `10gbase-lrm` - 10GBASE-LRM (10GE)
	/// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
	/// * `10gbase-sr` - 10GBASE-SR (10GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-zr` - 10GBASE-ZR (10GE)
	/// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
	/// * `25gbase-er` - 25GBASE-ER (25GE)
	/// * `25gbase-lr` - 25GBASE-LR (25GE)
	/// * `25gbase-sr` - 25GBASE-SR (25GE)
	/// * `25gbase-t` - 25GBASE-T (25GE)
	/// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
	/// * `40gbase-er4` - 40GBASE-ER4 (40GE)
	/// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
	/// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
	/// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
	/// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
	/// * `50gbase-er` - 50GBASE-ER (50GE)
	/// * `50gbase-fr` - 50GBASE-FR (50GE)
	/// * `50gbase-lr` - 50GBASE-LR (50GE)
	/// * `50gbase-sr` - 50GBASE-SR (50GE)
	/// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
	/// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
	/// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
	/// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
	/// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
	/// * `100gbase-dr` - 100GBASE-DR (100GE)
	/// * `100gbase-er4` - 100GBASE-ER4 (100GE)
	/// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
	/// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
	/// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
	/// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
	/// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
	/// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
	/// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
	/// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
	/// * `100gbase-zr` - 100GBASE-ZR (100GE)
	/// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
	/// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
	/// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
	/// * `200gbase-er4` - 200GBASE-ER4 (200GE)
	/// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
	/// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
	/// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
	/// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
	/// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
	/// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
	/// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
	/// * `400gbase-er8` - 400GBASE-ER8 (400GE)
	/// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
	/// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
	/// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
	/// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
	/// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
	/// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
	/// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
	/// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
	/// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
	/// * `400gbase-zr` - 400GBASE-ZR (400GE)
	/// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
	/// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
	/// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
	/// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `1000base-kx` - 1000BASE-KX (1GE)
	/// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
	/// * `5gbase-kr` - 5GBASE-KR (5GE)
	/// * `10gbase-kr` - 10GBASE-KR (10GE)
	/// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
	/// * `25gbase-kr` - 25GBASE-KR (25GE)
	/// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
	/// * `50gbase-kr` - 50GBASE-KR (50GE)
	/// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
	/// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
	/// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
	/// * `ieee802.11a` - IEEE 802.11a
	/// * `ieee802.11g` - IEEE 802.11b/g
	/// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
	/// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
	/// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
	/// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
	/// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
	/// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
	/// * `4g` - 4G
	/// * `5g` - 5G
	/// * `sonet-oc3` - OC-3/STM-1
	/// * `sonet-oc12` - OC-12/STM-4
	/// * `sonet-oc48` - OC-48/STM-16
	/// * `sonet-oc192` - OC-192/STM-64
	/// * `sonet-oc768` - OC-768/STM-256
	/// * `sonet-oc1920` - OC-1920/STM-640
	/// * `sonet-oc3840` - OC-3840/STM-1234
	/// * `1gfc-sfp` - SFP (1GFC)
	/// * `2gfc-sfp` - SFP (2GFC)
	/// * `4gfc-sfp` - SFP (4GFC)
	/// * `8gfc-sfpp` - SFP+ (8GFC)
	/// * `16gfc-sfpp` - SFP+ (16GFC)
	/// * `32gfc-sfp28` - SFP28 (32GFC)
	/// * `32gfc-sfpp` - SFP+ (32GFC)
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
	/// * `64gfc-sfpdd` - SFP-DD (64GFC)
	/// * `64gfc-sfpp` - SFP+ (64GFC)
	/// * `128gfc-qsfp28` - QSFP28 (128GFC)
	/// * `infiniband-sdr` - SDR (2 Gbps)
	/// * `infiniband-ddr` - DDR (4 Gbps)
	/// * `infiniband-qdr` - QDR (8 Gbps)
	/// * `infiniband-fdr10` - FDR10 (10 Gbps)
	/// * `infiniband-fdr` - FDR (13.5 Gbps)
	/// * `infiniband-edr` - EDR (25 Gbps)
	/// * `infiniband-hdr` - HDR (50 Gbps)
	/// * `infiniband-ndr` - NDR (100 Gbps)
	/// * `infiniband-xdr` - XDR (250 Gbps)
	/// * `t1` - T1 (1.544 Mbps)
	/// * `e1` - E1 (2.048 Mbps)
	/// * `t3` - T3 (45 Mbps)
	/// * `e3` - E3 (34 Mbps)
	/// * `xdsl` - xDSL
	/// * `docsis` - DOCSIS
	/// * `moca` - MoCA
	/// * `bpon` - BPON (622 Mbps / 155 Mbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `25g-pon` - 25G-PON (25 Gbps)
	/// * `50g-pon` - 50G-PON (50 Gbps)
	/// * `cisco-stackwise` - Cisco StackWise
	/// * `cisco-stackwise-plus` - Cisco StackWise Plus
	/// * `cisco-flexstack` - Cisco FlexStack
	/// * `cisco-flexstack-plus` - Cisco FlexStack Plus
	/// * `cisco-stackwise-80` - Cisco StackWise-80
	/// * `cisco-stackwise-160` - Cisco StackWise-160
	/// * `cisco-stackwise-320` - Cisco StackWise-320
	/// * `cisco-stackwise-480` - Cisco StackWise-480
	/// * `cisco-stackwise-1t` - Cisco StackWise-1T
	/// * `juniper-vcp` - Juniper VCP
	/// * `extreme-summitstack` - Extreme SummitStack
	/// * `extreme-summitstack-128` - Extreme SummitStack-128
	/// * `extreme-summitstack-256` - Extreme SummitStack-256
	/// * `extreme-summitstack-512` - Extreme SummitStack-512
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableInventoryItemRequest {
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	pub component_id: Option<u64>,
	pub component_type: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// This item was automatically discovered
	pub discovered: bool,
	/// Physical label
	pub label: String,
	pub manufacturer: Option<serde_json::Value>,
	pub name: String,
	pub parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableJournalEntryRequest {
	pub assigned_object_id: u64,
	pub assigned_object_type: String,
	pub comments: String,
	pub created_by: Option<i64>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	pub kind: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableL2VPNRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub export_targets: Vec<i64>,
	pub identifier: Option<i64>,
	pub import_targets: Vec<i64>,
	pub name: String,
	pub slug: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `evpn-vpws` - EVPN VPWS
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	/// * `spb` - SPB
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableLocationRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Local facility ID or description
	pub facility: String,
	pub name: String,
	pub parent: Option<i64>,
	pub site: serde_json::Value,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableModuleRequest {
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	pub module_bay: i64,
	pub module_type: serde_json::Value,
	pub serial: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableModuleTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	pub airflow: Option<String>,
	pub attributes: Option<serde_json::Value>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub profile: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePlatformRequest {
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub manufacturer: Option<serde_json::Value>,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerFeedRequest {
	pub amperage: u16,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// Maximum permissible draw (percentage)
	pub max_utilization: u8,
	pub name: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	pub phase: String,
	pub power_panel: serde_json::Value,
	pub rack: Option<serde_json::Value>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	pub status: String,
	/// * `ac` - AC
	/// * `dc` - DC
	pub supply: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	pub r#type: String,
	pub voltage: i16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerOutletRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub power_port: Option<serde_json::Value>,
	/// * `enabled` - Enabled
	/// * `disabled` - Disabled
	/// * `faulty` - Faulty
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	/// Physical port type
	/// 
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
	/// * `iec-60320-c17` - C17
	/// * `iec-60320-c19` - C19
	/// * `iec-60320-c21` - C21
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15r` - NEMA 1-15R
	/// * `nema-5-15r` - NEMA 5-15R
	/// * `nema-5-20r` - NEMA 5-20R
	/// * `nema-5-30r` - NEMA 5-30R
	/// * `nema-5-50r` - NEMA 5-50R
	/// * `nema-6-15r` - NEMA 6-15R
	/// * `nema-6-20r` - NEMA 6-20R
	/// * `nema-6-30r` - NEMA 6-30R
	/// * `nema-6-50r` - NEMA 6-50R
	/// * `nema-10-30r` - NEMA 10-30R
	/// * `nema-10-50r` - NEMA 10-50R
	/// * `nema-14-20r` - NEMA 14-20R
	/// * `nema-14-30r` - NEMA 14-30R
	/// * `nema-14-50r` - NEMA 14-50R
	/// * `nema-14-60r` - NEMA 14-60R
	/// * `nema-15-15r` - NEMA 15-15R
	/// * `nema-15-20r` - NEMA 15-20R
	/// * `nema-15-30r` - NEMA 15-30R
	/// * `nema-15-50r` - NEMA 15-50R
	/// * `nema-15-60r` - NEMA 15-60R
	/// * `nema-l1-15r` - NEMA L1-15R
	/// * `nema-l5-15r` - NEMA L5-15R
	/// * `nema-l5-20r` - NEMA L5-20R
	/// * `nema-l5-30r` - NEMA L5-30R
	/// * `nema-l5-50r` - NEMA L5-50R
	/// * `nema-l6-15r` - NEMA L6-15R
	/// * `nema-l6-20r` - NEMA L6-20R
	/// * `nema-l6-30r` - NEMA L6-30R
	/// * `nema-l6-50r` - NEMA L6-50R
	/// * `nema-l10-30r` - NEMA L10-30R
	/// * `nema-l14-20r` - NEMA L14-20R
	/// * `nema-l14-30r` - NEMA L14-30R
	/// * `nema-l14-50r` - NEMA L14-50R
	/// * `nema-l14-60r` - NEMA L14-60R
	/// * `nema-l15-20r` - NEMA L15-20R
	/// * `nema-l15-30r` - NEMA L15-30R
	/// * `nema-l15-50r` - NEMA L15-50R
	/// * `nema-l15-60r` - NEMA L15-60R
	/// * `nema-l21-20r` - NEMA L21-20R
	/// * `nema-l21-30r` - NEMA L21-30R
	/// * `nema-l22-20r` - NEMA L22-20R
	/// * `nema-l22-30r` - NEMA L22-30R
	/// * `CS6360C` - CS6360C
	/// * `CS6364C` - CS6364C
	/// * `CS8164C` - CS8164C
	/// * `CS8264C` - CS8264C
	/// * `CS8364C` - CS8364C
	/// * `CS8464C` - CS8464C
	/// * `ita-e` - ITA Type E (CEE 7/5)
	/// * `ita-f` - ITA Type F (CEE 7/3)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `ita-multistandard` - ITA Multistandard
	/// * `usb-a` - USB Type A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-c` - USB Type C
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `eaton-c39` - Eaton C39
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerOutletTemplateRequest {
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub power_port: Option<serde_json::Value>,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
	/// * `iec-60320-c17` - C17
	/// * `iec-60320-c19` - C19
	/// * `iec-60320-c21` - C21
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15r` - NEMA 1-15R
	/// * `nema-5-15r` - NEMA 5-15R
	/// * `nema-5-20r` - NEMA 5-20R
	/// * `nema-5-30r` - NEMA 5-30R
	/// * `nema-5-50r` - NEMA 5-50R
	/// * `nema-6-15r` - NEMA 6-15R
	/// * `nema-6-20r` - NEMA 6-20R
	/// * `nema-6-30r` - NEMA 6-30R
	/// * `nema-6-50r` - NEMA 6-50R
	/// * `nema-10-30r` - NEMA 10-30R
	/// * `nema-10-50r` - NEMA 10-50R
	/// * `nema-14-20r` - NEMA 14-20R
	/// * `nema-14-30r` - NEMA 14-30R
	/// * `nema-14-50r` - NEMA 14-50R
	/// * `nema-14-60r` - NEMA 14-60R
	/// * `nema-15-15r` - NEMA 15-15R
	/// * `nema-15-20r` - NEMA 15-20R
	/// * `nema-15-30r` - NEMA 15-30R
	/// * `nema-15-50r` - NEMA 15-50R
	/// * `nema-15-60r` - NEMA 15-60R
	/// * `nema-l1-15r` - NEMA L1-15R
	/// * `nema-l5-15r` - NEMA L5-15R
	/// * `nema-l5-20r` - NEMA L5-20R
	/// * `nema-l5-30r` - NEMA L5-30R
	/// * `nema-l5-50r` - NEMA L5-50R
	/// * `nema-l6-15r` - NEMA L6-15R
	/// * `nema-l6-20r` - NEMA L6-20R
	/// * `nema-l6-30r` - NEMA L6-30R
	/// * `nema-l6-50r` - NEMA L6-50R
	/// * `nema-l10-30r` - NEMA L10-30R
	/// * `nema-l14-20r` - NEMA L14-20R
	/// * `nema-l14-30r` - NEMA L14-30R
	/// * `nema-l14-50r` - NEMA L14-50R
	/// * `nema-l14-60r` - NEMA L14-60R
	/// * `nema-l15-20r` - NEMA L15-20R
	/// * `nema-l15-30r` - NEMA L15-30R
	/// * `nema-l15-50r` - NEMA L15-50R
	/// * `nema-l15-60r` - NEMA L15-60R
	/// * `nema-l21-20r` - NEMA L21-20R
	/// * `nema-l21-30r` - NEMA L21-30R
	/// * `nema-l22-20r` - NEMA L22-20R
	/// * `nema-l22-30r` - NEMA L22-30R
	/// * `CS6360C` - CS6360C
	/// * `CS6364C` - CS6364C
	/// * `CS8164C` - CS8164C
	/// * `CS8264C` - CS8264C
	/// * `CS8364C` - CS8364C
	/// * `CS8464C` - CS8464C
	/// * `ita-e` - ITA Type E (CEE 7/5)
	/// * `ita-f` - ITA Type F (CEE 7/3)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `ita-multistandard` - ITA Multistandard
	/// * `usb-a` - USB Type A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-c` - USB Type C
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `eaton-c39` - Eaton C39
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerPortRequest {
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
	/// Physical port type
	/// 
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
	/// * `iec-60320-c18` - C18
	/// * `iec-60320-c20` - C20
	/// * `iec-60320-c22` - C22
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15p` - NEMA 1-15P
	/// * `nema-5-15p` - NEMA 5-15P
	/// * `nema-5-20p` - NEMA 5-20P
	/// * `nema-5-30p` - NEMA 5-30P
	/// * `nema-5-50p` - NEMA 5-50P
	/// * `nema-6-15p` - NEMA 6-15P
	/// * `nema-6-20p` - NEMA 6-20P
	/// * `nema-6-30p` - NEMA 6-30P
	/// * `nema-6-50p` - NEMA 6-50P
	/// * `nema-10-30p` - NEMA 10-30P
	/// * `nema-10-50p` - NEMA 10-50P
	/// * `nema-14-20p` - NEMA 14-20P
	/// * `nema-14-30p` - NEMA 14-30P
	/// * `nema-14-50p` - NEMA 14-50P
	/// * `nema-14-60p` - NEMA 14-60P
	/// * `nema-15-15p` - NEMA 15-15P
	/// * `nema-15-20p` - NEMA 15-20P
	/// * `nema-15-30p` - NEMA 15-30P
	/// * `nema-15-50p` - NEMA 15-50P
	/// * `nema-15-60p` - NEMA 15-60P
	/// * `nema-l1-15p` - NEMA L1-15P
	/// * `nema-l5-15p` - NEMA L5-15P
	/// * `nema-l5-20p` - NEMA L5-20P
	/// * `nema-l5-30p` - NEMA L5-30P
	/// * `nema-l5-50p` - NEMA L5-50P
	/// * `nema-l6-15p` - NEMA L6-15P
	/// * `nema-l6-20p` - NEMA L6-20P
	/// * `nema-l6-30p` - NEMA L6-30P
	/// * `nema-l6-50p` - NEMA L6-50P
	/// * `nema-l10-30p` - NEMA L10-30P
	/// * `nema-l14-20p` - NEMA L14-20P
	/// * `nema-l14-30p` - NEMA L14-30P
	/// * `nema-l14-50p` - NEMA L14-50P
	/// * `nema-l14-60p` - NEMA L14-60P
	/// * `nema-l15-20p` - NEMA L15-20P
	/// * `nema-l15-30p` - NEMA L15-30P
	/// * `nema-l15-50p` - NEMA L15-50P
	/// * `nema-l15-60p` - NEMA L15-60P
	/// * `nema-l21-20p` - NEMA L21-20P
	/// * `nema-l21-30p` - NEMA L21-30P
	/// * `nema-l22-20p` - NEMA L22-20P
	/// * `nema-l22-30p` - NEMA L22-30P
	/// * `cs6361c` - CS6361C
	/// * `cs6365c` - CS6365C
	/// * `cs8165c` - CS8165C
	/// * `cs8265c` - CS8265C
	/// * `cs8365c` - CS8365C
	/// * `cs8465c` - CS8465C
	/// * `ita-c` - ITA Type C (CEE 7/16)
	/// * `ita-e` - ITA Type E (CEE 7/6)
	/// * `ita-f` - ITA Type F (CEE 7/4)
	/// * `ita-ef` - ITA Type E/F (CEE 7/7)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `usb-3-b` - USB 3.0 Type B
	/// * `usb-3-micro-b` - USB 3.0 Micro B
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerPortTemplateRequest {
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
	/// * `iec-60320-c18` - C18
	/// * `iec-60320-c20` - C20
	/// * `iec-60320-c22` - C22
	/// * `iec-60309-p-n-e-4h` - P+N+E 4H
	/// * `iec-60309-p-n-e-6h` - P+N+E 6H
	/// * `iec-60309-p-n-e-9h` - P+N+E 9H
	/// * `iec-60309-2p-e-4h` - 2P+E 4H
	/// * `iec-60309-2p-e-6h` - 2P+E 6H
	/// * `iec-60309-2p-e-9h` - 2P+E 9H
	/// * `iec-60309-3p-e-4h` - 3P+E 4H
	/// * `iec-60309-3p-e-6h` - 3P+E 6H
	/// * `iec-60309-3p-e-9h` - 3P+E 9H
	/// * `iec-60309-3p-n-e-4h` - 3P+N+E 4H
	/// * `iec-60309-3p-n-e-6h` - 3P+N+E 6H
	/// * `iec-60309-3p-n-e-9h` - 3P+N+E 9H
	/// * `iec-60906-1` - IEC 60906-1
	/// * `nbr-14136-10a` - 2P+T 10A (NBR 14136)
	/// * `nbr-14136-20a` - 2P+T 20A (NBR 14136)
	/// * `nema-1-15p` - NEMA 1-15P
	/// * `nema-5-15p` - NEMA 5-15P
	/// * `nema-5-20p` - NEMA 5-20P
	/// * `nema-5-30p` - NEMA 5-30P
	/// * `nema-5-50p` - NEMA 5-50P
	/// * `nema-6-15p` - NEMA 6-15P
	/// * `nema-6-20p` - NEMA 6-20P
	/// * `nema-6-30p` - NEMA 6-30P
	/// * `nema-6-50p` - NEMA 6-50P
	/// * `nema-10-30p` - NEMA 10-30P
	/// * `nema-10-50p` - NEMA 10-50P
	/// * `nema-14-20p` - NEMA 14-20P
	/// * `nema-14-30p` - NEMA 14-30P
	/// * `nema-14-50p` - NEMA 14-50P
	/// * `nema-14-60p` - NEMA 14-60P
	/// * `nema-15-15p` - NEMA 15-15P
	/// * `nema-15-20p` - NEMA 15-20P
	/// * `nema-15-30p` - NEMA 15-30P
	/// * `nema-15-50p` - NEMA 15-50P
	/// * `nema-15-60p` - NEMA 15-60P
	/// * `nema-l1-15p` - NEMA L1-15P
	/// * `nema-l5-15p` - NEMA L5-15P
	/// * `nema-l5-20p` - NEMA L5-20P
	/// * `nema-l5-30p` - NEMA L5-30P
	/// * `nema-l5-50p` - NEMA L5-50P
	/// * `nema-l6-15p` - NEMA L6-15P
	/// * `nema-l6-20p` - NEMA L6-20P
	/// * `nema-l6-30p` - NEMA L6-30P
	/// * `nema-l6-50p` - NEMA L6-50P
	/// * `nema-l10-30p` - NEMA L10-30P
	/// * `nema-l14-20p` - NEMA L14-20P
	/// * `nema-l14-30p` - NEMA L14-30P
	/// * `nema-l14-50p` - NEMA L14-50P
	/// * `nema-l14-60p` - NEMA L14-60P
	/// * `nema-l15-20p` - NEMA L15-20P
	/// * `nema-l15-30p` - NEMA L15-30P
	/// * `nema-l15-50p` - NEMA L15-50P
	/// * `nema-l15-60p` - NEMA L15-60P
	/// * `nema-l21-20p` - NEMA L21-20P
	/// * `nema-l21-30p` - NEMA L21-30P
	/// * `nema-l22-20p` - NEMA L22-20P
	/// * `nema-l22-30p` - NEMA L22-30P
	/// * `cs6361c` - CS6361C
	/// * `cs6365c` - CS6365C
	/// * `cs8165c` - CS8165C
	/// * `cs8265c` - CS8265C
	/// * `cs8365c` - CS8365C
	/// * `cs8465c` - CS8465C
	/// * `ita-c` - ITA Type C (CEE 7/16)
	/// * `ita-e` - ITA Type E (CEE 7/6)
	/// * `ita-f` - ITA Type F (CEE 7/4)
	/// * `ita-ef` - ITA Type E/F (CEE 7/7)
	/// * `ita-g` - ITA Type G (BS 1363)
	/// * `ita-h` - ITA Type H
	/// * `ita-i` - ITA Type I
	/// * `ita-j` - ITA Type J
	/// * `ita-k` - ITA Type K
	/// * `ita-l` - ITA Type L (CEI 23-50)
	/// * `ita-m` - ITA Type M (BS 546)
	/// * `ita-n` - ITA Type N
	/// * `ita-o` - ITA Type O
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `usb-3-b` - USB 3.0 Type B
	/// * `usb-3-micro-b` - USB 3.0 Micro B
	/// * `molex-micro-fit-1x2` - Molex Micro-Fit 1x2
	/// * `molex-micro-fit-2x2` - Molex Micro-Fit 2x2
	/// * `molex-micro-fit-2x4` - Molex Micro-Fit 2x4
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePrefixRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: bool,
	/// Treat as fully utilized
	pub mark_utilized: bool,
	pub prefix: String,
	pub role: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRackRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	pub airflow: Option<String>,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	pub description: String,
	pub facility_id: Option<String>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	pub location: Option<serde_json::Value>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub name: String,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	pub rack_type: Option<serde_json::Value>,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	pub site: serde_json::Value,
	/// Starting unit for rack
	pub starting_unit: u16,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// Height in rack units
	pub u_height: u8,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRackReservationRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub rack: serde_json::Value,
	/// * `pending` - Pending
	/// * `active` - Active
	/// * `stale` - Stale
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub units: Vec<u16>,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRackTypeRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	pub description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: String,
	pub manufacturer: serde_json::Value,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	pub model: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	pub slug: String,
	/// Starting unit for rack
	pub starting_unit: u16,
	pub tags: Vec<NestedTagRequest>,
	/// Height in rack units
	pub u_height: u8,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRearPortRequest {
	pub color: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	/// Physical label
	pub label: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Number of front ports which may be mapped
	pub positions: u16,
	pub tags: Vec<NestedTagRequest>,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRearPortTemplateRequest {
	pub color: String,
	pub description: String,
	pub device_type: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub positions: u16,
	/// * `8p8c` - 8P8C
	/// * `8p6c` - 8P6C
	/// * `8p4c` - 8P4C
	/// * `8p2c` - 8P2C
	/// * `6p6c` - 6P6C
	/// * `6p4c` - 6P4C
	/// * `6p2c` - 6P2C
	/// * `4p4c` - 4P4C
	/// * `4p2c` - 4P2C
	/// * `gg45` - GG45
	/// * `tera-4p` - TERA 4P
	/// * `tera-2p` - TERA 2P
	/// * `tera-1p` - TERA 1P
	/// * `110-punch` - 110 Punch
	/// * `bnc` - BNC
	/// * `f` - F Connector
	/// * `n` - N Connector
	/// * `mrj21` - MRJ21
	/// * `fc` - FC
	/// * `fc-pc` - FC/PC
	/// * `fc-upc` - FC/UPC
	/// * `fc-apc` - FC/APC
	/// * `lc` - LC
	/// * `lc-pc` - LC/PC
	/// * `lc-upc` - LC/UPC
	/// * `lc-apc` - LC/APC
	/// * `lsh` - LSH
	/// * `lsh-pc` - LSH/PC
	/// * `lsh-upc` - LSH/UPC
	/// * `lsh-apc` - LSH/APC
	/// * `lx5` - LX.5
	/// * `lx5-pc` - LX.5/PC
	/// * `lx5-upc` - LX.5/UPC
	/// * `lx5-apc` - LX.5/APC
	/// * `mpo` - MPO
	/// * `mtrj` - MTRJ
	/// * `sc` - SC
	/// * `sc-pc` - SC/PC
	/// * `sc-upc` - SC/UPC
	/// * `sc-apc` - SC/APC
	/// * `st` - ST
	/// * `cs` - CS
	/// * `sn` - SN
	/// * `sma-905` - SMA 905
	/// * `sma-906` - SMA 906
	/// * `urm-p2` - URM-P2
	/// * `urm-p4` - URM-P4
	/// * `urm-p8` - URM-P8
	/// * `splice` - Splice
	/// * `usb-a` - USB Type A
	/// * `usb-b` - USB Type B
	/// * `usb-c` - USB Type C
	/// * `usb-mini-a` - USB Mini A
	/// * `usb-mini-b` - USB Mini B
	/// * `usb-micro-a` - USB Micro A
	/// * `usb-micro-b` - USB Micro B
	/// * `usb-micro-ab` - USB Micro AB
	/// * `other` - Other
	pub r#type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRegionRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableServiceRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub ipaddresses: Vec<i64>,
	pub name: String,
	pub parent_object_id: u64,
	pub parent_object_type: String,
	pub ports: Vec<u16>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableServiceTemplateRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub ports: Vec<u16>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableSiteGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableSiteRequest {
	pub asns: Vec<i64>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// Local facility ID or description
	pub facility: String,
	pub group: Option<serde_json::Value>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	/// Full name of the site
	pub name: String,
	/// Physical location of the building
	pub physical_address: String,
	pub region: Option<serde_json::Value>,
	/// If different from the physical address
	pub shipping_address: String,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub time_zone: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableTenantGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableTunnelRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	/// * `wireguard` - WireGuard
	/// * `openvpn` - OpenVPN
	/// * `l2tp` - L2TP
	/// * `pptp` - PPTP
	pub encapsulation: String,
	pub group: Option<serde_json::Value>,
	pub ipsec_profile: Option<serde_json::Value>,
	pub name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub tunnel_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableTunnelTerminationRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub outside_ip: Option<serde_json::Value>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub tags: Vec<NestedTagRequest>,
	pub termination_id: Option<u64>,
	pub termination_type: String,
	pub tunnel: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVLANRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub name: String,
	/// Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)
	/// 
	/// * `svlan` - Service
	/// * `cvlan` - Customer
	pub qinq_role: Option<String>,
	pub qinq_svlan: Option<i64>,
	pub role: Option<serde_json::Value>,
	pub site: Option<serde_json::Value>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVMInterfaceRequest {
	pub bridge: Option<i64>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub enabled: bool,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: Option<String>,
	pub mtu: Option<u32>,
	pub name: String,
	pub parent: Option<i64>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub virtual_machine: serde_json::Value,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualChassisRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub domain: String,
	pub master: Option<i64>,
	pub name: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub provider_account: Option<serde_json::Value>,
	pub provider_network: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualCircuitTerminationRequest {
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub interface: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub tags: Vec<NestedTagRequest>,
	pub virtual_circuit: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualDeviceContextRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: serde_json::Value,
	pub identifier: Option<u16>,
	pub name: String,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualMachineWithConfigContextRequest {
	pub cluster: Option<serde_json::Value>,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub device: Option<serde_json::Value>,
	pub disk: Option<u32>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub memory: Option<u32>,
	pub name: String,
	pub platform: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	pub site: Option<serde_json::Value>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	/// * `paused` - Paused
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vcpus: Option<f64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableWirelessLANGroupRequest {
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub name: String,
	pub parent: Option<i64>,
	pub slug: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableWirelessLANRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: Option<String>,
	pub auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub group: Option<serde_json::Value>,
	pub scope_id: Option<i64>,
	pub scope_type: Option<String>,
	pub ssid: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableWirelessLinkRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: Option<String>,
	pub auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: Option<String>,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub interface_a: serde_json::Value,
	pub interface_b: serde_json::Value,
	pub ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tags: Vec<NestedTagRequest>,
	pub tenant: Option<serde_json::Value>,
}

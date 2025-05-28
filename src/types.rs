#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASN {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// 16- or 32-bit autonomous system number
	pub asn: Option<u32>,
	pub rir: Option<BriefRIR>,
	pub tenant: Option<BriefTenant>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub site_count: Option<i64>,
	pub provider_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASNRange {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub rir: Option<BriefRIR>,
	pub start: Option<u32>,
	pub end: Option<u32>,
	pub tenant: Option<BriefTenant>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub asn_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASNRangeRequest {
	pub name: String,
	pub slug: String,
	pub rir: serde_json::Value,
	pub start: u32,
	pub end: u32,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ASNRequest {
	/// 16- or 32-bit autonomous system number
	pub asn: u32,
	pub rir: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Aggregate {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub prefix: Option<String>,
	pub rir: Option<BriefRIR>,
	pub tenant: Option<BriefTenant>,
	pub date_added: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AggregateRequest {
	pub prefix: String,
	pub rir: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub date_added: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailableASN {
	pub asn: Option<i64>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailableIP {
	pub family: Option<i64>,
	pub address: Option<String>,
	pub vrf: Option<BriefVRF>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailablePrefix {
	pub family: Option<i64>,
	pub prefix: Option<String>,
	pub vrf: Option<BriefVRF>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AvailableVLAN {
	pub vid: Option<i64>,
	pub group: Option<BriefVLANGroup>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Bookmark {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<u64>,
	pub object: Option<serde_json::Value>,
	pub user: Option<BriefUser>,
	pub created: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BookmarkRequest {
	pub object_type: String,
	pub object_id: u64,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCable {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub label: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCableRequest {
	pub label: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuit {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// Unique circuit ID
	pub cid: Option<String>,
	pub provider: Option<BriefProvider>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitGroupAssignmentSerializer_ {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub group: Option<BriefCircuitGroup>,
	pub priority: Option<std::collections::HashMap<String, serde_json::Value>>,
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
	pub provider: serde_json::Value,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCircuitTypeRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCluster {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub cluster_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub cluster_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefClusterTypeRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefConfigTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefConfigTemplateRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContact {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContactRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContactRole {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefContactRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCustomFieldChoiceSet {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub choices_count: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefCustomFieldChoiceSetRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDataFile {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// File path relative to the data source's root
	pub path: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDataSource {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDataSourceRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDevice {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceRequest {
	pub name: Option<String>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceRole {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefDeviceTypeRequest {
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefFHRPGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: Option<String>,
	pub group_id: Option<u16>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefFHRPGroupRequest {
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: String,
	pub group_id: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIKEPolicy {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIKEPolicyRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPAddress {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub address: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPAddressRequest {
	pub address: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecPolicy {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecPolicyRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecProfile {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefIPSecProfileRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInterface {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub cable: Option<BriefCable>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInterfaceRequest {
	pub device: serde_json::Value,
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInventoryItemRole {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub inventoryitem_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefInventoryItemRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefJob {
	pub url: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub completed: Option<String>,
	pub user: Option<BriefUser>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefJobRequest {
	pub completed: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPN {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub identifier: Option<i64>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPNRequest {
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPNTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub l2vpn: Option<BriefL2VPN>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefL2VPNTerminationRequest {
	pub l2vpn: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefLocation {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub rack_count: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefLocationRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefMACAddress {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub mac_address: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefMACAddressRequest {
	pub mac_address: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefManufacturer {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub devicetype_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefManufacturerRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModule {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module_bay: Option<NestedModuleBay>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleRequest {
	pub device: serde_json::Value,
	pub module_bay: NestedModuleBayRequest,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub profile: Option<BriefModuleTypeProfile>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleTypeProfile {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleTypeProfileRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefModuleTypeRequest {
	pub profile: Option<serde_json::Value>,
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPlatform {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPlatformRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPanel {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub powerfeed_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPanelRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPort {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub cable: Option<BriefCable>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPortRequest {
	pub device: serde_json::Value,
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefPowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProvider {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// Full name of the provider
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderAccount {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub account: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderAccountRequest {
	pub name: String,
	pub account: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderNetwork {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderNetworkRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefProviderRequest {
	/// Full name of the provider
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRIR {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub aggregate_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRIRRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRack {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub device_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackRole {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub rack_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRackTypeRequest {
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRearPortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRegion {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub site_count: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRegionRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRole {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub prefix_count: Option<i64>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSite {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// Full name of the site
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSiteGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub site_count: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSiteGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefSiteRequest {
	/// Full name of the site
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTag {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenant {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenantGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tenant_count: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenantGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTenantRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnel {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnelGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tunnel_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnelGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefTunnelRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefUser {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
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
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// Numeric VLAN ID (1-4094)
	pub vid: Option<u16>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANRequest {
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANTranslationPolicy {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVLANTranslationPolicyRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVRF {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub description: Option<String>,
	pub prefix_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVRFRequest {
	pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualChassis {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub master: Option<NestedDevice>,
	pub description: Option<String>,
	pub member_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualChassisRequest {
	pub name: String,
	pub master: Option<NestedDeviceRequest>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuit {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// Unique circuit ID
	pub cid: Option<String>,
	pub provider_network: Option<BriefProviderNetwork>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider_network: serde_json::Value,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuitType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub virtual_circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualCircuitTypeRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualMachine {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefVirtualMachineRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefWirelessLANGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub wirelesslan_count: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BriefWirelessLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Cable {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Singlemode Fiber
	/// * `smf-os1` - Singlemode Fiber (OS1)
	/// * `smf-os2` - Singlemode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `usb` - USB
	/// * `power` - Power
	pub r#type: Option<String>,
	pub a_terminations: Option<Vec<GenericObject>>,
	pub b_terminations: Option<Vec<GenericObject>>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tenant: Option<BriefTenant>,
	pub label: Option<String>,
	pub color: Option<String>,
	pub length: Option<f64>,
	pub length_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableRequest {
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Singlemode Fiber
	/// * `smf-os1` - Singlemode Fiber (OS1)
	/// * `smf-os2` - Singlemode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `usb` - USB
	/// * `power` - Power
	pub r#type: Option<String>,
	pub a_terminations: Vec<GenericObjectRequest>,
	pub b_terminations: Vec<GenericObjectRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub label: String,
	pub color: String,
	pub length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	pub length_unit: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub cable: Option<i64>,
	/// * `A` - A
	/// * `B` - B
	pub cable_end: Option<String>,
	pub termination_type: Option<String>,
	pub termination_id: Option<u64>,
	pub termination: Option<serde_json::Value>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableTerminationRequest {
	pub cable: i64,
	/// * `A` - A
	/// * `B` - B
	pub cable_end: String,
	pub termination_type: String,
	pub termination_id: u64,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Circuit {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Unique circuit ID
	pub cid: Option<String>,
	pub provider: Option<BriefProvider>,
	pub provider_account: Option<BriefProviderAccount>,
	pub r#type: Option<BriefCircuitType>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tenant: Option<BriefTenant>,
	pub install_date: Option<String>,
	pub termination_date: Option<String>,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub description: Option<String>,
	pub distance: Option<f64>,
	pub distance_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub termination_a: Option<CircuitCircuitTermination>,
	pub termination_z: Option<CircuitCircuitTermination>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub assignments: Option<Vec<BriefCircuitGroupAssignmentSerializer_>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitCircuitTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub termination_type: Option<String>,
	pub termination_id: Option<i64>,
	pub termination: Option<serde_json::Value>,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitCircuitTerminationRequest {
	pub termination_type: Option<String>,
	pub termination_id: Option<i64>,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tenant: Option<BriefTenant>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroupAssignment {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub group: Option<BriefCircuitGroup>,
	pub member_type: Option<String>,
	pub member_id: Option<u64>,
	pub member: Option<serde_json::Value>,
	pub priority: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub member_type: String,
	pub member_id: u64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: String,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tenant: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub install_date: Option<String>,
	pub termination_date: Option<String>,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub assignments: Vec<BriefCircuitGroupAssignmentSerializer_Request>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub circuit: Option<BriefCircuit>,
	/// * `A` - A
	/// * `Z` - Z
	pub term_side: Option<String>,
	pub termination_type: Option<String>,
	pub termination_id: Option<i64>,
	pub termination: Option<serde_json::Value>,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: Option<String>,
	/// Patch panel ID and port number(s)
	pub pp_info: Option<String>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitTerminationRequest {
	pub circuit: serde_json::Value,
	/// * `A` - A
	/// * `Z` - Z
	pub term_side: String,
	pub termination_type: Option<String>,
	pub termination_id: Option<i64>,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: String,
	/// Patch panel ID and port number(s)
	pub pp_info: String,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitType {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitTypeRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Cluster {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub r#type: Option<BriefClusterType>,
	pub group: Option<BriefClusterGroup>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tenant: Option<BriefTenant>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub scope: Option<serde_json::Value>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub device_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
	pub allocated_vcpus: Option<f64>,
	pub allocated_memory: Option<i64>,
	pub allocated_disk: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub cluster_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterRequest {
	pub name: String,
	pub r#type: serde_json::Value,
	pub group: Option<serde_json::Value>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterType {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub cluster_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterTypeRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigContext {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub weight: Option<u16>,
	pub description: Option<String>,
	pub is_active: Option<bool>,
	pub regions: Option<Vec<Region>>,
	pub site_groups: Option<Vec<SiteGroup>>,
	pub sites: Option<Vec<Site>>,
	pub locations: Option<Vec<Location>>,
	pub device_types: Option<Vec<DeviceType>>,
	pub roles: Option<Vec<DeviceRole>>,
	pub platforms: Option<Vec<Platform>>,
	pub cluster_types: Option<Vec<ClusterType>>,
	pub cluster_groups: Option<Vec<ClusterGroup>>,
	pub clusters: Option<Vec<Cluster>>,
	pub tenant_groups: Option<Vec<TenantGroup>>,
	pub tenants: Option<Vec<Tenant>>,
	pub tags: Option<Vec<String>>,
	pub data_source: Option<BriefDataSource>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_file: Option<BriefDataFile>,
	pub data_synced: Option<String>,
	pub data: Option<serde_json::Value>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigContextRequest {
	pub name: String,
	pub weight: u16,
	pub description: String,
	pub is_active: bool,
	pub regions: Vec<i64>,
	pub site_groups: Vec<i64>,
	pub sites: Vec<i64>,
	pub locations: Vec<i64>,
	pub device_types: Vec<i64>,
	pub roles: Vec<i64>,
	pub platforms: Vec<i64>,
	pub cluster_types: Vec<i64>,
	pub cluster_groups: Vec<i64>,
	pub clusters: Vec<i64>,
	pub tenant_groups: Vec<i64>,
	pub tenants: Vec<i64>,
	pub tags: Vec<String>,
	pub data_source: serde_json::Value,
	pub data: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Jinja template code.
	pub template_code: Option<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: Option<String>,
	/// Filename to give to the rendered export file
	pub file_name: Option<String>,
	/// Extension to append to the rendered filename
	pub file_extension: Option<String>,
	/// Download file as attachment
	pub as_attachment: Option<bool>,
	pub data_source: Option<BriefDataSource>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_file: Option<BriefDataFile>,
	pub data_synced: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigTemplateRequest {
	pub name: String,
	pub description: String,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Jinja template code.
	pub template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: String,
	/// Filename to give to the rendered export file
	pub file_name: String,
	/// Extension to append to the rendered filename
	pub file_extension: String,
	/// Download file as attachment
	pub as_attachment: bool,
	pub data_source: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePort {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub speed: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_type: Option<String>,
	pub connected_endpoints_reachable: Option<bool>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	pub speed: Option<i64>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsolePortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPort {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub speed: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_type: Option<String>,
	pub connected_endpoints_reachable: Option<bool>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	pub speed: Option<i64>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleServerPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Contact {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub groups: Option<Vec<ContactGroup>>,
	pub name: Option<String>,
	pub title: Option<String>,
	pub phone: Option<String>,
	pub email: Option<String>,
	pub address: Option<String>,
	pub link: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactAssignment {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<u64>,
	pub object: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub contact: Option<BriefContact>,
	pub role: Option<BriefContactRole>,
	pub priority: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactAssignmentRequest {
	pub object_type: String,
	pub object_id: u64,
	pub contact: serde_json::Value,
	pub role: Option<serde_json::Value>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub parent: Option<NestedContactGroup>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub contact_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<NestedContactGroupRequest>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactRequest {
	pub groups: Vec<i64>,
	pub name: String,
	pub title: String,
	pub phone: String,
	pub email: String,
	pub address: String,
	pub link: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactRole {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ContactRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomField {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub related_object_type: Option<String>,
	pub data_type: Option<String>,
	/// Internal field name
	pub name: Option<String>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: Option<String>,
	/// Custom fields within the same group will be displayed together
	pub group_name: Option<String>,
	pub description: Option<String>,
	/// This field is required when creating new objects or editing an existing object.
	pub required: Option<bool>,
	/// The value of this field must be unique for the assigned object
	pub unique: Option<bool>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: Option<u16>,
	pub filter_logic: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ui_visible: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ui_editable: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Replicate this value when cloning objects
	pub is_cloneable: Option<bool>,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	/// Fields with higher weights appear lower in a form.
	pub weight: Option<u16>,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<i64>,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: Option<String>,
	pub choice_set: Option<BriefCustomFieldChoiceSet>,
	pub comments: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomFieldChoiceSet {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub base_choices: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub extra_choices: Option<Vec<Vec<serde_json::Value>>>,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: Option<bool>,
	pub choices_count: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomFieldChoiceSetRequest {
	pub name: String,
	pub description: String,
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	pub base_choices: String,
	pub extra_choices: Vec<Vec<serde_json::Value>>,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomFieldRequest {
	pub object_types: Vec<String>,
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
	pub related_object_type: Option<String>,
	/// Internal field name
	pub name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: String,
	/// Custom fields within the same group will be displayed together
	pub group_name: String,
	pub description: String,
	/// This field is required when creating new objects or editing an existing object.
	pub required: bool,
	/// The value of this field must be unique for the assigned object
	pub unique: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: u16,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	pub filter_logic: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	pub ui_visible: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	pub ui_editable: String,
	/// Replicate this value when cloning objects
	pub is_cloneable: bool,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	/// Fields with higher weights appear lower in a form.
	pub weight: u16,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<i64>,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: String,
	pub choice_set: Option<serde_json::Value>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomLink {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub name: Option<String>,
	pub enabled: Option<bool>,
	/// Jinja2 template code for link text
	pub link_text: Option<String>,
	/// Jinja2 template code for link URL
	pub link_url: Option<String>,
	pub weight: Option<u16>,
	/// Links with the same group will appear as a dropdown menu
	pub group_name: Option<String>,
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
	/// Force link to open in a new window
	pub new_window: Option<bool>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomLinkRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub enabled: bool,
	/// Jinja2 template code for link text
	pub link_text: String,
	/// Jinja2 template code for link URL
	pub link_url: String,
	pub weight: u16,
	/// Links with the same group will appear as a dropdown menu
	pub group_name: String,
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
	/// Force link to open in a new window
	pub new_window: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Dashboard {
	pub layout: Option<serde_json::Value>,
	pub config: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DashboardRequest {
	pub layout: serde_json::Value,
	pub config: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataFile {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub source: Option<BriefDataSource>,
	/// File path relative to the data source's root
	pub path: Option<String>,
	pub last_updated: Option<String>,
	pub size: Option<i64>,
	/// SHA256 hash of the file data
	pub hash: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataSource {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub source_url: Option<String>,
	pub enabled: Option<bool>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	pub parameters: Option<serde_json::Value>,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: Option<String>,
	pub comments: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub last_synced: Option<String>,
	pub file_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataSourceRequest {
	pub name: String,
	/// * `None` - ---------
	/// * `local` - Local
	/// * `git` - Git
	/// * `amazon-s3` - Amazon S3
	pub r#type: serde_json::Value,
	pub source_url: String,
	pub enabled: bool,
	pub description: String,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	pub parameters: Option<serde_json::Value>,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Device {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub role: Option<BriefDeviceRole>,
	pub tenant: Option<BriefTenant>,
	pub platform: Option<BriefPlatform>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: Option<String>,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub site: Option<BriefSite>,
	pub location: Option<BriefLocation>,
	pub rack: Option<BriefRack>,
	pub position: Option<f64>,
	pub face: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub parent_device: Option<NestedDevice>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub oob_ip: Option<BriefIPAddress>,
	pub cluster: Option<BriefCluster>,
	pub virtual_chassis: Option<BriefVirtualChassis>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub config_template: Option<BriefConfigTemplate>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub console_port_count: Option<i64>,
	pub console_server_port_count: Option<i64>,
	pub power_port_count: Option<i64>,
	pub power_outlet_count: Option<i64>,
	pub interface_count: Option<i64>,
	pub front_port_count: Option<i64>,
	pub rear_port_count: Option<i64>,
	pub device_bay_count: Option<i64>,
	pub module_bay_count: Option<i64>,
	pub inventory_item_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBay {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub description: Option<String>,
	pub installed_device: Option<BriefDevice>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBayRequest {
	pub device: serde_json::Value,
	pub name: String,
	/// Physical label
	pub label: String,
	pub description: String,
	pub installed_device: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBayTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceBayTemplateRequest {
	pub device_type: serde_json::Value,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceRole {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	/// Virtual machines may be assigned to this role
	pub vm_role: Option<bool>,
	pub config_template: Option<BriefConfigTemplate>,
	pub parent: Option<NestedDeviceRole>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub device_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	/// Virtual machines may be assigned to this role
	pub vm_role: bool,
	pub config_template: Option<serde_json::Value>,
	pub parent: Option<NestedDeviceRoleRequest>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceType {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub default_platform: Option<BriefPlatform>,
	pub model: Option<String>,
	pub slug: Option<String>,
	/// Discrete part number (optional)
	pub part_number: Option<String>,
	pub u_height: Option<f64>,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: Option<bool>,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: Option<bool>,
	pub subdevice_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub weight: Option<f64>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub front_image: Option<String>,
	pub rear_image: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub device_count: Option<i64>,
	pub console_port_template_count: Option<i64>,
	pub console_server_port_template_count: Option<i64>,
	pub power_port_template_count: Option<i64>,
	pub power_outlet_template_count: Option<i64>,
	pub interface_template_count: Option<i64>,
	pub front_port_template_count: Option<i64>,
	pub rear_port_template_count: Option<i64>,
	pub device_bay_template_count: Option<i64>,
	pub module_bay_template_count: Option<i64>,
	pub inventory_item_template_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceTypeRequest {
	pub manufacturer: serde_json::Value,
	pub default_platform: Option<serde_json::Value>,
	pub model: String,
	pub slug: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub u_height: f64,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: bool,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: bool,
	/// * `parent` - Parent
	/// * `child` - Child
	pub subdevice_role: Option<String>,
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
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	pub front_image: Option<String>,
	pub rear_image: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceWithConfigContext {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub role: Option<BriefDeviceRole>,
	pub tenant: Option<BriefTenant>,
	pub platform: Option<BriefPlatform>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: Option<String>,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub site: Option<BriefSite>,
	pub location: Option<BriefLocation>,
	pub rack: Option<BriefRack>,
	pub position: Option<f64>,
	pub face: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub parent_device: Option<NestedDevice>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub oob_ip: Option<BriefIPAddress>,
	pub cluster: Option<BriefCluster>,
	pub virtual_chassis: Option<BriefVirtualChassis>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub config_template: Option<BriefConfigTemplate>,
	pub config_context: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub console_port_count: Option<i64>,
	pub console_server_port_count: Option<i64>,
	pub power_port_count: Option<i64>,
	pub power_outlet_count: Option<i64>,
	pub interface_count: Option<i64>,
	pub front_port_count: Option<i64>,
	pub rear_port_count: Option<i64>,
	pub device_bay_count: Option<i64>,
	pub module_bay_count: Option<i64>,
	pub inventory_item_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceWithConfigContextRequest {
	pub name: Option<String>,
	pub device_type: serde_json::Value,
	pub role: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub rack: Option<serde_json::Value>,
	pub position: Option<f64>,
	/// * `front` - Front
	/// * `rear` - Rear
	pub face: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	pub status: String,
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
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub oob_ip: Option<serde_json::Value>,
	pub cluster: Option<serde_json::Value>,
	pub virtual_chassis: Option<serde_json::Value>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub description: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EventRule {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub name: Option<String>,
	pub enabled: Option<bool>,
	/// The types of event which will trigger this rule.
	pub event_types: Option<Vec<String>>,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	pub action_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub action_object_type: Option<String>,
	pub action_object_id: Option<u64>,
	pub action_object: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EventRuleRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub enabled: bool,
	/// The types of event which will trigger this rule.
	pub event_types: Vec<String>,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	/// * `notification` - Notification
	pub action_type: String,
	pub action_object_type: String,
	pub action_object_id: Option<u64>,
	pub description: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExportTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub name: Option<String>,
	pub description: Option<String>,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Jinja template code.
	pub template_code: Option<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: Option<String>,
	/// Filename to give to the rendered export file
	pub file_name: Option<String>,
	/// Extension to append to the rendered filename
	pub file_extension: Option<String>,
	/// Download file as attachment
	pub as_attachment: Option<bool>,
	pub data_source: Option<BriefDataSource>,
	/// Path to remote file (relative to data source root)
	pub data_path: Option<String>,
	pub data_file: Option<BriefDataFile>,
	pub data_synced: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExportTemplateRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub description: String,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Jinja template code.
	pub template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: String,
	/// Filename to give to the rendered export file
	pub file_name: String,
	/// Extension to append to the rendered filename
	pub file_extension: String,
	/// Download file as attachment
	pub as_attachment: bool,
	pub data_source: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroup {
	pub id: i64,
	pub name: Option<String>,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: Option<String>,
	pub group_id: Option<u16>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	pub auth_type: Option<String>,
	pub auth_key: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub ip_addresses: Option<Vec<BriefIPAddress>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroupAssignment {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub group: Option<BriefFHRPGroup>,
	pub interface_type: Option<String>,
	pub interface_id: Option<u64>,
	pub interface: Option<serde_json::Value>,
	pub priority: Option<u8>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub interface_type: String,
	pub interface_id: u64,
	pub priority: u8,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FHRPGroupRequest {
	pub name: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: String,
	pub group_id: u16,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	pub auth_type: Option<String>,
	pub auth_key: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPort {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub color: Option<String>,
	pub rear_port: Option<FrontPortRearPort>,
	/// Mapped position on corresponding rear port
	pub rear_port_position: Option<u16>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortRearPort {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortRearPortRequest {
	pub name: String,
	/// Physical label
	pub label: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub rear_port: FrontPortRearPortRequest,
	/// Mapped position on corresponding rear port
	pub rear_port_position: u16,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub color: Option<String>,
	pub rear_port: Option<BriefRearPortTemplate>,
	pub rear_port_position: Option<u16>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FrontPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub rear_port: serde_json::Value,
	pub rear_port_position: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenericObject {
	pub object_type: Option<String>,
	pub object_id: Option<i64>,
	pub object: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenericObjectRequest {
	pub object_type: String,
	pub object_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Group {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub permissions: Option<Vec<ObjectPermission>>,
	pub user_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GroupRequest {
	pub name: String,
	pub description: String,
	pub permissions: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEPolicy {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub version: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub proposals: Option<Vec<IKEProposal>>,
	pub preshared_key: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEPolicyRequest {
	pub name: String,
	pub description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	pub version: i64,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	pub mode: String,
	pub proposals: Vec<i64>,
	pub preshared_key: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEProposal {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub authentication_method: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub encryption_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub authentication_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub group: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IKEProposalRequest {
	pub name: String,
	pub description: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	pub authentication_method: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: String,
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
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPAddress {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub address: Option<String>,
	pub vrf: Option<BriefVRF>,
	pub tenant: Option<BriefTenant>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object: Option<serde_json::Value>,
	pub nat_inside: Option<NestedIPAddress>,
	pub nat_outside: Option<Vec<NestedIPAddress>>,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPAddressRequest {
	pub address: String,
	pub vrf: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	pub status: String,
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	pub role: String,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub nat_inside: Option<NestedIPAddressRequest>,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPRange {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub start_address: Option<String>,
	pub end_address: Option<String>,
	pub size: Option<i64>,
	pub vrf: Option<BriefVRF>,
	pub tenant: Option<BriefTenant>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<BriefRole>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: Option<bool>,
	/// Report space as 100% utilized
	pub mark_utilized: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPRangeRequest {
	pub start_address: String,
	pub end_address: String,
	pub vrf: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: bool,
	/// Report space as 100% utilized
	pub mark_utilized: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecPolicy {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub proposals: Option<Vec<IPSecProposal>>,
	pub pfs_group: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecPolicyRequest {
	pub name: String,
	pub description: String,
	pub proposals: Vec<i64>,
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
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProfile {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ike_policy: Option<BriefIKEPolicy>,
	pub ipsec_policy: Option<BriefIPSecPolicy>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProfileRequest {
	pub name: String,
	pub description: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	pub mode: String,
	pub ike_policy: serde_json::Value,
	pub ipsec_policy: serde_json::Value,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProposal {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub encryption_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub authentication_algorithm: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IPSecProposalRequest {
	pub name: String,
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
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: String,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ImageAttachment {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<u64>,
	pub parent: Option<serde_json::Value>,
	pub name: Option<String>,
	pub image: Option<String>,
	pub image_height: Option<i64>,
	pub image_width: Option<i64>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ImageAttachmentRequest {
	pub object_type: String,
	pub object_id: u64,
	pub name: String,
	pub image: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IntegerRange(pub Vec<i64>);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IntegerRangeRequest(pub Vec<i64>);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Interface {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub vdcs: Option<Vec<VirtualDeviceContext>>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub enabled: Option<bool>,
	pub parent: Option<NestedInterface>,
	pub bridge: Option<NestedInterface>,
	pub lag: Option<NestedInterface>,
	pub mtu: Option<u32>,
	pub mac_address: Option<String>,
	pub primary_mac_address: Option<BriefMACAddress>,
	pub mac_addresses: Option<Vec<BriefMACAddress>>,
	pub speed: Option<u32>,
	pub duplex: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub wwn: Option<String>,
	/// This interface is used only for out-of-band management
	pub mgmt_only: Option<bool>,
	pub description: Option<String>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub rf_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub rf_channel: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub poe_mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub poe_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	pub tx_power: Option<u8>,
	pub untagged_vlan: Option<BriefVLAN>,
	pub tagged_vlans: Option<Vec<VLAN>>,
	pub qinq_svlan: Option<BriefVLAN>,
	pub vlan_translation_policy: Option<BriefVLANTranslationPolicy>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub wireless_link: Option<NestedWirelessLink>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub wireless_lans: Option<Vec<WirelessLAN>>,
	pub vrf: Option<BriefVRF>,
	pub l2vpn_termination: Option<BriefL2VPNTermination>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_type: Option<String>,
	pub connected_endpoints_reachable: Option<bool>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub count_ipaddresses: Option<i64>,
	pub count_fhrp_groups: Option<i64>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InterfaceRequest {
	pub device: serde_json::Value,
	pub vdcs: Vec<i64>,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
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
	/// * `ieee802.11n` - IEEE 802.11n
	/// * `ieee802.11ac` - IEEE 802.11ac
	/// * `ieee802.11ad` - IEEE 802.11ad
	/// * `ieee802.11ax` - IEEE 802.11ax
	/// * `ieee802.11ay` - IEEE 802.11ay
	/// * `ieee802.11be` - IEEE 802.11be
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
	pub enabled: bool,
	pub parent: Option<NestedInterfaceRequest>,
	pub bridge: Option<NestedInterfaceRequest>,
	pub lag: Option<NestedInterfaceRequest>,
	pub mtu: Option<u32>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub speed: Option<u32>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	pub duplex: Option<String>,
	pub wwn: Option<String>,
	/// This interface is used only for out-of-band management
	pub mgmt_only: bool,
	pub description: String,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: String,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: String,
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
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	pub tx_power: Option<u8>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub wireless_lans: Vec<i64>,
	pub vrf: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InterfaceTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub enabled: Option<bool>,
	pub mgmt_only: Option<bool>,
	pub description: Option<String>,
	pub bridge: Option<NestedInterfaceTemplate>,
	pub poe_mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub poe_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub rf_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InterfaceTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
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
	/// * `ieee802.11n` - IEEE 802.11n
	/// * `ieee802.11ac` - IEEE 802.11ac
	/// * `ieee802.11ad` - IEEE 802.11ad
	/// * `ieee802.11ax` - IEEE 802.11ax
	/// * `ieee802.11ay` - IEEE 802.11ay
	/// * `ieee802.11be` - IEEE 802.11be
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
	pub enabled: bool,
	pub mgmt_only: bool,
	pub description: String,
	pub bridge: Option<NestedInterfaceTemplateRequest>,
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
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItem {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub parent: Option<i64>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<BriefInventoryItemRole>,
	pub manufacturer: Option<BriefManufacturer>,
	/// Manufacturer-assigned part identifier
	pub part_id: Option<String>,
	pub serial: Option<String>,
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	/// This item was automatically discovered
	pub discovered: Option<bool>,
	pub description: Option<String>,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
	pub component: Option<serde_json::Value>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemRequest {
	pub device: serde_json::Value,
	pub parent: Option<i64>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub manufacturer: Option<serde_json::Value>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub serial: String,
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	/// This item was automatically discovered
	pub discovered: bool,
	pub description: String,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemRole {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub inventoryitem_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub role: Option<BriefInventoryItemRole>,
	pub manufacturer: Option<BriefManufacturer>,
	/// Manufacturer-assigned part identifier
	pub part_id: Option<String>,
	pub description: Option<String>,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
	pub component: Option<serde_json::Value>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InventoryItemTemplateRequest {
	pub device_type: serde_json::Value,
	pub parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	pub role: Option<serde_json::Value>,
	pub manufacturer: Option<serde_json::Value>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub description: String,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Job {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<u64>,
	pub name: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub scheduled: Option<String>,
	/// Recurrence interval (in minutes)
	pub interval: Option<u32>,
	pub started: Option<String>,
	pub completed: Option<String>,
	pub user: Option<BriefUser>,
	pub data: Option<serde_json::Value>,
	pub error: Option<String>,
	pub job_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JournalEntry {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object: Option<serde_json::Value>,
	pub created: Option<String>,
	pub created_by: Option<i64>,
	pub kind: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JournalEntryRequest {
	pub assigned_object_type: String,
	pub assigned_object_id: u64,
	pub created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	pub kind: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPN {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub identifier: Option<i64>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub import_targets: Option<Vec<RouteTarget>>,
	pub export_targets: Option<Vec<RouteTarget>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tenant: Option<BriefTenant>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPNRequest {
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
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub import_targets: Vec<i64>,
	pub export_targets: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub tenant: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPNTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub l2vpn: Option<BriefL2VPN>,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object: Option<serde_json::Value>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct L2VPNTerminationRequest {
	pub l2vpn: serde_json::Value,
	pub assigned_object_type: String,
	pub assigned_object_id: u64,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Location {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub site: Option<BriefSite>,
	pub parent: Option<NestedLocation>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tenant: Option<BriefTenant>,
	/// Local facility ID or description
	pub facility: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub rack_count: Option<i64>,
	pub device_count: Option<i64>,
	pub prefix_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocationRequest {
	pub name: String,
	pub slug: String,
	pub site: serde_json::Value,
	pub parent: Option<NestedLocationRequest>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	/// Local facility ID or description
	pub facility: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MACAddress {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub mac_address: Option<String>,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub assigned_object: Option<serde_json::Value>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MACAddressRequest {
	pub mac_address: String,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Manufacturer {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub devicetype_count: Option<i64>,
	pub inventoryitem_count: Option<i64>,
	pub platform_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ManufacturerRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Module {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module_bay: Option<NestedModuleBay>,
	pub module_type: Option<BriefModuleType>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub serial: Option<String>,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBay {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	pub installed_module: Option<BriefModule>,
	/// Physical label
	pub label: Option<String>,
	/// Identifier to reference when renaming installed components
	pub position: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBayRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub installed_module: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	/// Identifier to reference when renaming installed components
	pub position: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBayTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	/// Identifier to reference when renaming installed components
	pub position: Option<String>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleBayTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// Identifier to reference when renaming installed components
	pub position: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleRequest {
	pub device: serde_json::Value,
	pub module_bay: NestedModuleBayRequest,
	pub module_type: serde_json::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub serial: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleType {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub profile: Option<BriefModuleTypeProfile>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	/// Discrete part number (optional)
	pub part_number: Option<String>,
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub weight: Option<f64>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub attributes: Option<serde_json::Value>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleTypeProfile {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub schema: Option<serde_json::Value>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleTypeProfileRequest {
	pub name: String,
	pub description: String,
	pub schema: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModuleTypeRequest {
	pub profile: Option<serde_json::Value>,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	pub airflow: Option<String>,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	pub description: String,
	pub attributes: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedContactGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedContactGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDevice {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDeviceRequest {
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDeviceRole {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedDeviceRoleRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedIPAddress {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub family: Option<i64>,
	pub address: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedIPAddressRequest {
	pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterface {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<NestedDevice>,
	pub name: Option<String>,
	pub cable: Option<i64>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterfaceRequest {
	pub name: String,
	pub cable: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterfaceTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedLocation {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedLocationRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedModuleBay {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedModuleBayRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedProviderAccount {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub account: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedRegion {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedRegionRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedSiteGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedSiteGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTag {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTagRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTenantGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedTenantGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedUser {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVLAN {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	/// Numeric VLAN ID (1-4094)
	pub vid: Option<u16>,
	pub name: Option<String>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVLANRequest {
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVMInterface {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub virtual_machine: Option<NestedVirtualMachine>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVMInterfaceRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVirtualMachine {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedVirtualMachineRequest {
	pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLANGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLANGroupRequest {
	pub name: String,
	pub slug: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLink {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub ssid: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NestedWirelessLinkRequest {
	pub ssid: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Notification {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<u64>,
	pub object: Option<serde_json::Value>,
	pub user: Option<BriefUser>,
	pub created: Option<String>,
	pub read: Option<String>,
	/// * `object_created` - Object created
	/// * `object_updated` - Object updated
	/// * `object_deleted` - Object deleted
	/// * `job_started` - Job started
	/// * `job_completed` - Job completed
	/// * `job_failed` - Job failed
	/// * `job_errored` - Job errored
	pub event_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotificationGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub display_url: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub groups: Option<Vec<Group>>,
	pub users: Option<Vec<User>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotificationGroupRequest {
	pub name: String,
	pub description: String,
	pub groups: Vec<i64>,
	pub users: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotificationRequest {
	pub object_type: String,
	pub object_id: u64,
	pub user: serde_json::Value,
	pub read: Option<String>,
	/// * `object_created` - Object created
	/// * `object_updated` - Object updated
	/// * `object_deleted` - Object deleted
	/// * `job_started` - Job started
	/// * `job_completed` - Job completed
	/// * `job_failed` - Job failed
	/// * `job_errored` - Job errored
	pub event_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectChange {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub time: Option<String>,
	pub user: Option<BriefUser>,
	pub user_name: Option<String>,
	pub request_id: Option<String>,
	pub action: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub changed_object_type: Option<String>,
	pub changed_object_id: Option<u64>,
	pub changed_object: Option<serde_json::Value>,
	pub prechange_data: Option<serde_json::Value>,
	pub postchange_data: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectPermission {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub enabled: Option<bool>,
	pub object_types: Option<Vec<String>>,
	/// The list of actions granted by this permission
	pub actions: Option<Vec<String>>,
	/// Queryset filter matching the applicable objects of the selected type(s)
	pub constraints: Option<serde_json::Value>,
	pub groups: Option<Vec<NestedGroup>>,
	pub users: Option<Vec<NestedUser>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectPermissionRequest {
	pub name: String,
	pub description: String,
	pub enabled: bool,
	pub object_types: Vec<String>,
	/// The list of actions granted by this permission
	pub actions: Vec<String>,
	/// Queryset filter matching the applicable objects of the selected type(s)
	pub constraints: Option<serde_json::Value>,
	pub groups: Vec<i64>,
	pub users: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ObjectType {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub app_label: Option<String>,
	pub model: Option<String>,
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
	pub name: String,
	pub slug: String,
	pub rir: serde_json::Value,
	pub start: u32,
	pub end: u32,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedASNRequest {
	/// 16- or 32-bit autonomous system number
	pub asn: u32,
	pub rir: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedBookmarkRequest {
	pub object_type: String,
	pub object_id: u64,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCableTerminationRequest {
	pub cable: i64,
	/// * `A` - A
	/// * `B` - B
	pub cable_end: String,
	pub termination_type: String,
	pub termination_id: u64,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCircuitGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tenant: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCircuitTerminationRequest {
	pub circuit: serde_json::Value,
	/// * `A` - A
	/// * `Z` - Z
	pub term_side: String,
	pub termination_type: Option<String>,
	pub termination_id: Option<i64>,
	/// Physical circuit speed
	pub port_speed: Option<u32>,
	/// Upstream speed, if different from port speed
	pub upstream_speed: Option<u32>,
	/// ID of the local cross-connect
	pub xconnect_id: String,
	/// Patch panel ID and port number(s)
	pub pp_info: String,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCircuitTypeRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedClusterGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedClusterTypeRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedConfigContextRequest {
	pub name: String,
	pub weight: u16,
	pub description: String,
	pub is_active: bool,
	pub regions: Vec<i64>,
	pub site_groups: Vec<i64>,
	pub sites: Vec<i64>,
	pub locations: Vec<i64>,
	pub device_types: Vec<i64>,
	pub roles: Vec<i64>,
	pub platforms: Vec<i64>,
	pub cluster_types: Vec<i64>,
	pub cluster_groups: Vec<i64>,
	pub clusters: Vec<i64>,
	pub tenant_groups: Vec<i64>,
	pub tenants: Vec<i64>,
	pub tags: Vec<String>,
	pub data_source: serde_json::Value,
	pub data: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedConfigTemplateRequest {
	pub name: String,
	pub description: String,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Jinja template code.
	pub template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: String,
	/// Filename to give to the rendered export file
	pub file_name: String,
	/// Extension to append to the rendered filename
	pub file_extension: String,
	/// Download file as attachment
	pub as_attachment: bool,
	pub data_source: serde_json::Value,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedContactRequest {
	pub groups: Vec<i64>,
	pub name: String,
	pub title: String,
	pub phone: String,
	pub email: String,
	pub address: String,
	pub link: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedContactRoleRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedCustomLinkRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub enabled: bool,
	/// Jinja2 template code for link text
	pub link_text: String,
	/// Jinja2 template code for link URL
	pub link_url: String,
	pub weight: u16,
	/// Links with the same group will appear as a dropdown menu
	pub group_name: String,
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
	/// Force link to open in a new window
	pub new_window: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedDashboardRequest {
	pub layout: serde_json::Value,
	pub config: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedDeviceBayRequest {
	pub device: serde_json::Value,
	pub name: String,
	/// Physical label
	pub label: String,
	pub description: String,
	pub installed_device: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedDeviceBayTemplateRequest {
	pub device_type: serde_json::Value,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedExportTemplateRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub description: String,
	/// Any <a href="https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment">additional parameters</a> to pass when constructing the Jinja environment
	pub environment_params: Option<serde_json::Value>,
	/// Jinja template code.
	pub template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	pub mime_type: String,
	/// Filename to give to the rendered export file
	pub file_name: String,
	/// Extension to append to the rendered filename
	pub file_extension: String,
	/// Download file as attachment
	pub as_attachment: bool,
	pub data_source: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedFHRPGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub interface_type: String,
	pub interface_id: u64,
	pub priority: u8,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedFHRPGroupRequest {
	pub name: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	pub protocol: String,
	pub group_id: u16,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	pub auth_type: Option<String>,
	pub auth_key: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedGroupRequest {
	pub name: String,
	pub description: String,
	pub permissions: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedImageAttachmentRequest {
	pub object_type: String,
	pub object_id: u64,
	pub name: String,
	pub image: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedInventoryItemRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedInventoryItemTemplateRequest {
	pub device_type: serde_json::Value,
	pub parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	pub role: Option<serde_json::Value>,
	pub manufacturer: Option<serde_json::Value>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub description: String,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedL2VPNTerminationRequest {
	pub l2vpn: serde_json::Value,
	pub assigned_object_type: String,
	pub assigned_object_id: u64,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedMACAddressRequest {
	pub mac_address: String,
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedManufacturerRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedModuleBayRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	pub installed_module: Option<serde_json::Value>,
	/// Physical label
	pub label: String,
	/// Identifier to reference when renaming installed components
	pub position: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedModuleBayTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// Identifier to reference when renaming installed components
	pub position: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedModuleTypeProfileRequest {
	pub name: String,
	pub description: String,
	pub schema: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedNotificationGroupRequest {
	pub name: String,
	pub description: String,
	pub groups: Vec<i64>,
	pub users: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedNotificationRequest {
	pub object_type: String,
	pub object_id: u64,
	pub user: serde_json::Value,
	pub read: Option<String>,
	/// * `object_created` - Object created
	/// * `object_updated` - Object updated
	/// * `object_deleted` - Object deleted
	/// * `job_started` - Job started
	/// * `job_completed` - Job completed
	/// * `job_failed` - Job failed
	/// * `job_errored` - Job errored
	pub event_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedObjectPermissionRequest {
	pub name: String,
	pub description: String,
	pub enabled: bool,
	pub object_types: Vec<String>,
	/// The list of actions granted by this permission
	pub actions: Vec<String>,
	/// Queryset filter matching the applicable objects of the selected type(s)
	pub constraints: Option<serde_json::Value>,
	pub groups: Vec<i64>,
	pub users: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedPlatformRequest {
	pub name: String,
	pub slug: String,
	pub manufacturer: Option<serde_json::Value>,
	pub config_template: Option<serde_json::Value>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedPowerPanelRequest {
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub name: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedProviderAccountRequest {
	pub provider: serde_json::Value,
	pub name: String,
	pub account: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedProviderNetworkRequest {
	pub provider: serde_json::Value,
	pub name: String,
	pub service_id: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedProviderRequest {
	/// Full name of the provider
	pub name: String,
	pub slug: String,
	pub accounts: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub asns: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRIRRequest {
	pub name: String,
	pub slug: String,
	/// IP space managed by this RIR is considered private
	pub is_private: bool,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRackReservationRequest {
	pub rack: serde_json::Value,
	pub units: Vec<u16>,
	pub user: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRackRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRoleRequest {
	pub name: String,
	pub slug: String,
	pub weight: u16,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedRouteTargetRequest {
	/// Route target value (formatted in accordance with RFC 4360)
	pub name: String,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedSavedFilterRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub slug: String,
	pub description: String,
	pub user: Option<i64>,
	pub weight: u16,
	pub enabled: bool,
	pub shared: bool,
	pub parameters: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedScriptInputRequest {
	pub data: serde_json::Value,
	pub commit: bool,
	pub schedule_at: Option<String>,
	pub interval: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedSubscriptionRequest {
	pub object_type: String,
	pub object_id: u64,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTableConfigRequest {
	pub object_type: String,
	pub table: String,
	pub name: String,
	pub description: String,
	pub user: Option<i64>,
	pub weight: u16,
	pub enabled: bool,
	pub shared: bool,
	pub columns: Vec<String>,
	pub ordering: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTagRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub weight: u16,
	pub object_types: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTenantRequest {
	pub name: String,
	pub slug: String,
	pub group: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTokenRequest {
	pub user: serde_json::Value,
	pub expires: Option<String>,
	pub last_used: Option<String>,
	pub key: String,
	/// Permit create/update/delete operations using this key
	pub write_enabled: bool,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedTunnelGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: String,
	pub password: String,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
	/// Designates whether the user can log into this admin site.
	pub is_staff: bool,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	pub is_active: bool,
	pub date_joined: String,
	pub last_login: Option<String>,
	pub groups: Vec<i64>,
	pub permissions: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub vid_ranges: Vec<IntegerRangeRequest>,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVLANTranslationPolicyRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVLANTranslationRuleRequest {
	pub policy: i64,
	/// Numeric VLAN ID (1-4094)
	pub local_vid: u16,
	/// Numeric VLAN ID (1-4094)
	pub remote_vid: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVRFRequest {
	pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub tenant: Option<serde_json::Value>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	pub enforce_unique: bool,
	pub description: String,
	pub comments: String,
	pub import_targets: Vec<i64>,
	pub export_targets: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVirtualCircuitTypeRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedVirtualDiskRequest {
	pub virtual_machine: serde_json::Value,
	pub name: String,
	pub description: String,
	pub size: u32,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWebhookRequest {
	pub name: String,
	pub description: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	pub payload_url: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	pub http_method: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	pub http_content_type: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	pub additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	pub body_template: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	pub secret: String,
	/// Enable SSL certificate verification. Disable with caution!
	pub ssl_verification: bool,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	pub ca_file_path: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableAggregateRequest {
	pub prefix: String,
	pub rir: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub date_added: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCableRequest {
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Singlemode Fiber
	/// * `smf-os1` - Singlemode Fiber (OS1)
	/// * `smf-os2` - Singlemode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `usb` - USB
	/// * `power` - Power
	pub r#type: Option<String>,
	pub a_terminations: Vec<GenericObjectRequest>,
	pub b_terminations: Vec<GenericObjectRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub label: String,
	pub color: String,
	pub length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	pub length_unit: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCircuitGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub member_type: String,
	pub member_id: u64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: Option<String>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub install_date: Option<String>,
	pub termination_date: Option<String>,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub assignments: Vec<BriefCircuitGroupAssignmentSerializer_Request>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableClusterRequest {
	pub name: String,
	pub r#type: serde_json::Value,
	pub group: Option<serde_json::Value>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsolePortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsolePortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsoleServerPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableContactAssignmentRequest {
	pub object_type: String,
	pub object_id: u64,
	pub contact: serde_json::Value,
	pub role: Option<serde_json::Value>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: Option<String>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableContactGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	pub name: String,
	pub description: String,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	pub base_choices: Option<String>,
	pub extra_choices: Vec<Vec<serde_json::Value>>,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableCustomFieldRequest {
	pub object_types: Vec<String>,
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
	pub related_object_type: Option<String>,
	/// Internal field name
	pub name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: String,
	/// Custom fields within the same group will be displayed together
	pub group_name: String,
	pub description: String,
	/// This field is required when creating new objects or editing an existing object.
	pub required: bool,
	/// The value of this field must be unique for the assigned object
	pub unique: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: u16,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	pub filter_logic: String,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	pub ui_visible: String,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	pub ui_editable: String,
	/// Replicate this value when cloning objects
	pub is_cloneable: bool,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	/// Fields with higher weights appear lower in a form.
	pub weight: u16,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<i64>,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: String,
	pub choice_set: Option<serde_json::Value>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDataSourceRequest {
	pub name: String,
	pub r#type: String,
	pub source_url: String,
	pub enabled: bool,
	pub description: String,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	pub parameters: Option<serde_json::Value>,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDeviceRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	/// Virtual machines may be assigned to this role
	pub vm_role: bool,
	pub config_template: Option<serde_json::Value>,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDeviceTypeRequest {
	pub manufacturer: serde_json::Value,
	pub default_platform: Option<serde_json::Value>,
	pub model: String,
	pub slug: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub u_height: f64,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: bool,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: bool,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	pub subdevice_role: Option<String>,
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
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	pub front_image: Option<String>,
	pub rear_image: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableDeviceWithConfigContextRequest {
	pub name: Option<String>,
	pub device_type: serde_json::Value,
	pub role: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub rack: Option<serde_json::Value>,
	pub position: Option<f64>,
	/// * `front` - Front
	/// * `rear` - Rear
	pub face: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	pub status: String,
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
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub oob_ip: Option<serde_json::Value>,
	pub cluster: Option<serde_json::Value>,
	pub virtual_chassis: Option<serde_json::Value>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub description: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableEventRuleRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub enabled: bool,
	/// The types of event which will trigger this rule.
	pub event_types: Vec<String>,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	/// * `notification` - Notification
	pub action_type: String,
	pub action_object_type: String,
	pub action_object_id: Option<u64>,
	pub description: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableFrontPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub rear_port: i64,
	/// Mapped position on corresponding rear port
	pub rear_port_position: u16,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableFrontPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub rear_port: serde_json::Value,
	pub rear_port_position: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIKEPolicyRequest {
	pub name: String,
	pub description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	pub version: u16,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	pub mode: Option<String>,
	pub proposals: Vec<i64>,
	pub preshared_key: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIKEProposalRequest {
	pub name: String,
	pub description: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	pub authentication_method: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: Option<String>,
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
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPAddressRequest {
	pub address: String,
	pub vrf: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	pub status: String,
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
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	/// The IP for which this address is the "outside" IP
	pub nat_inside: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPRangeRequest {
	pub start_address: String,
	pub end_address: String,
	pub vrf: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: bool,
	/// Report space as 100% utilized
	pub mark_utilized: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPSecPolicyRequest {
	pub name: String,
	pub description: String,
	pub proposals: Vec<i64>,
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
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPSecProfileRequest {
	pub name: String,
	pub description: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	pub mode: String,
	pub ike_policy: serde_json::Value,
	pub ipsec_policy: serde_json::Value,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableIPSecProposalRequest {
	pub name: String,
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
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: Option<String>,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableInterfaceRequest {
	pub device: serde_json::Value,
	pub vdcs: Vec<i64>,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
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
	/// * `ieee802.11n` - IEEE 802.11n
	/// * `ieee802.11ac` - IEEE 802.11ac
	/// * `ieee802.11ad` - IEEE 802.11ad
	/// * `ieee802.11ax` - IEEE 802.11ax
	/// * `ieee802.11ay` - IEEE 802.11ay
	/// * `ieee802.11be` - IEEE 802.11be
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
	pub enabled: bool,
	pub parent: Option<i64>,
	pub bridge: Option<i64>,
	pub lag: Option<i64>,
	pub mtu: Option<u32>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub speed: Option<u32>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	pub duplex: Option<String>,
	pub wwn: Option<String>,
	/// This interface is used only for out-of-band management
	pub mgmt_only: bool,
	pub description: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: Option<String>,
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
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	pub tx_power: Option<u8>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub wireless_lans: Vec<i64>,
	pub vrf: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableInterfaceTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
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
	/// * `ieee802.11n` - IEEE 802.11n
	/// * `ieee802.11ac` - IEEE 802.11ac
	/// * `ieee802.11ad` - IEEE 802.11ad
	/// * `ieee802.11ax` - IEEE 802.11ax
	/// * `ieee802.11ay` - IEEE 802.11ay
	/// * `ieee802.11be` - IEEE 802.11be
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
	pub enabled: bool,
	pub mgmt_only: bool,
	pub description: String,
	pub bridge: Option<i64>,
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
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableInventoryItemRequest {
	pub device: serde_json::Value,
	pub parent: Option<i64>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub manufacturer: Option<serde_json::Value>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub serial: String,
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	/// This item was automatically discovered
	pub discovered: bool,
	pub description: String,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableJournalEntryRequest {
	pub assigned_object_type: String,
	pub assigned_object_id: u64,
	pub created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	pub kind: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableL2VPNRequest {
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
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub import_targets: Vec<i64>,
	pub export_targets: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub tenant: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableLocationRequest {
	pub name: String,
	pub slug: String,
	pub site: serde_json::Value,
	pub parent: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	/// Local facility ID or description
	pub facility: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableModuleRequest {
	pub device: serde_json::Value,
	pub module_bay: i64,
	pub module_type: serde_json::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub serial: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableModuleTypeRequest {
	pub profile: Option<serde_json::Value>,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	pub airflow: Option<String>,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	pub description: String,
	pub attributes: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerFeedRequest {
	pub power_panel: serde_json::Value,
	pub rack: Option<serde_json::Value>,
	pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	pub status: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	pub r#type: String,
	/// * `ac` - AC
	/// * `dc` - DC
	pub supply: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	pub phase: String,
	pub voltage: i16,
	pub amperage: u16,
	/// Maximum permissible draw (percentage)
	pub max_utilization: u8,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub description: String,
	pub tenant: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerOutletRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// Physical port type
	/// 
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
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
	/// * `enabled` - Enabled
	/// * `disabled` - Disabled
	/// * `faulty` - Faulty
	pub status: String,
	pub color: String,
	pub power_port: Option<serde_json::Value>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerOutletTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
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
	pub power_port: Option<serde_json::Value>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// Physical port type
	/// 
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
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
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePowerPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
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
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritablePrefixRequest {
	pub prefix: String,
	pub vrf: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: bool,
	/// Treat as fully utilized
	pub mark_utilized: bool,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRackRequest {
	pub name: String,
	pub facility_id: Option<String>,
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub rack_type: Option<serde_json::Value>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: u16,
	/// Height in rack units
	pub u_height: u8,
	/// Starting unit for rack
	pub starting_unit: u16,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	pub airflow: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRackTypeRequest {
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
	pub description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: String,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: u16,
	/// Height in rack units
	pub u_height: u8,
	/// Starting unit for rack
	pub starting_unit: u16,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRearPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	/// Number of front ports which may be mapped
	pub positions: u16,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRearPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub positions: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableRegionRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableServiceRequest {
	pub parent_object_type: String,
	pub parent_object_id: u64,
	pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub ports: Vec<u16>,
	pub ipaddresses: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableServiceTemplateRequest {
	pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub ports: Vec<u16>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableSiteGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableSiteRequest {
	/// Full name of the site
	pub name: String,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub region: Option<serde_json::Value>,
	pub group: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// Local facility ID or description
	pub facility: String,
	pub time_zone: Option<String>,
	pub description: String,
	/// Physical location of the building
	pub physical_address: String,
	/// If different from the physical address
	pub shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub comments: String,
	pub asns: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableTenantGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableTunnelRequest {
	pub name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	pub status: String,
	pub group: Option<serde_json::Value>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	/// * `wireguard` - WireGuard
	/// * `openvpn` - OpenVPN
	/// * `l2tp` - L2TP
	/// * `pptp` - PPTP
	pub encapsulation: String,
	pub ipsec_profile: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub tunnel_id: Option<u64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableTunnelTerminationRequest {
	pub tunnel: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub termination_type: String,
	pub termination_id: Option<u64>,
	pub outside_ip: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVLANRequest {
	pub site: Option<serde_json::Value>,
	pub group: Option<serde_json::Value>,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
	pub name: String,
	pub tenant: Option<serde_json::Value>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub description: String,
	/// Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)
	/// 
	/// * `svlan` - Service
	/// * `cvlan` - Customer
	pub qinq_role: Option<String>,
	pub qinq_svlan: Option<i64>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVMInterfaceRequest {
	pub virtual_machine: serde_json::Value,
	pub name: String,
	pub enabled: bool,
	pub parent: Option<i64>,
	pub bridge: Option<i64>,
	pub mtu: Option<u32>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub description: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: Option<String>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualChassisRequest {
	pub name: String,
	pub domain: String,
	pub master: Option<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider_network: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualCircuitTerminationRequest {
	pub virtual_circuit: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub interface: serde_json::Value,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualDeviceContextRequest {
	pub name: String,
	pub device: serde_json::Value,
	pub identifier: Option<u16>,
	pub tenant: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	pub status: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	/// * `paused` - Paused
	pub status: String,
	pub site: Option<serde_json::Value>,
	pub cluster: Option<serde_json::Value>,
	pub device: Option<serde_json::Value>,
	pub serial: String,
	pub role: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub vcpus: Option<f64>,
	pub memory: Option<u32>,
	pub disk: Option<u32>,
	pub description: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableWirelessLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableWirelessLANRequest {
	pub ssid: String,
	pub description: String,
	pub group: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	pub status: String,
	pub vlan: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub tenant: Option<serde_json::Value>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: Option<String>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: Option<String>,
	pub auth_psk: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PatchedWritableWirelessLinkRequest {
	pub interface_a: serde_json::Value,
	pub interface_b: serde_json::Value,
	pub ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: Option<String>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: Option<String>,
	pub auth_psk: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Platform {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub config_template: Option<BriefConfigTemplate>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub device_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlatformRequest {
	pub name: String,
	pub slug: String,
	pub manufacturer: Option<serde_json::Value>,
	pub config_template: Option<serde_json::Value>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerFeed {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub power_panel: Option<BriefPowerPanel>,
	pub rack: Option<BriefRack>,
	pub name: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub supply: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub phase: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub voltage: Option<i16>,
	pub amperage: Option<u16>,
	/// Maximum permissible draw (percentage)
	pub max_utilization: Option<u8>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_type: Option<String>,
	pub connected_endpoints_reachable: Option<bool>,
	pub description: Option<String>,
	pub tenant: Option<BriefTenant>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerFeedRequest {
	pub power_panel: serde_json::Value,
	pub rack: Option<serde_json::Value>,
	pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	pub status: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	pub r#type: String,
	/// * `ac` - AC
	/// * `dc` - DC
	pub supply: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	pub phase: String,
	pub voltage: i16,
	pub amperage: u16,
	/// Maximum permissible draw (percentage)
	pub max_utilization: u8,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub description: String,
	pub tenant: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutlet {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub color: Option<String>,
	pub power_port: Option<BriefPowerPort>,
	pub feed_leg: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_type: Option<String>,
	pub connected_endpoints_reachable: Option<bool>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutletRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
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
	/// * `enabled` - Enabled
	/// * `disabled` - Disabled
	/// * `faulty` - Faulty
	pub status: String,
	pub color: String,
	pub power_port: Option<serde_json::Value>,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutletTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub power_port: Option<BriefPowerPortTemplate>,
	pub feed_leg: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerOutletTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
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
	pub power_port: Option<serde_json::Value>,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPanel {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub site: Option<BriefSite>,
	pub location: Option<BriefLocation>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub powerfeed_count: Option<i64>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPanelRequest {
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub name: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPort {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub connected_endpoints: Option<Vec<serde_json::Value>>,
	pub connected_endpoints_type: Option<String>,
	pub connected_endpoints_reachable: Option<bool>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
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
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
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
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Prefix {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub family: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub prefix: Option<String>,
	pub vrf: Option<BriefVRF>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub scope: Option<serde_json::Value>,
	pub tenant: Option<BriefTenant>,
	pub vlan: Option<BriefVLAN>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<BriefRole>,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: Option<bool>,
	/// Treat as fully utilized
	pub mark_utilized: Option<bool>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub children: Option<i64>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PrefixRequest {
	pub prefix: String,
	pub vrf: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: bool,
	/// Treat as fully utilized
	pub mark_utilized: bool,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Provider {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Full name of the provider
	pub name: Option<String>,
	pub slug: Option<String>,
	pub accounts: Option<Vec<NestedProviderAccount>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub asns: Option<Vec<ASN>>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderAccount {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub provider: Option<BriefProvider>,
	pub name: Option<String>,
	pub account: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderAccountRequest {
	pub provider: serde_json::Value,
	pub name: String,
	pub account: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderNetwork {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub provider: Option<BriefProvider>,
	pub name: Option<String>,
	pub service_id: Option<String>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderNetworkRequest {
	pub provider: serde_json::Value,
	pub name: String,
	pub service_id: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderRequest {
	/// Full name of the provider
	pub name: String,
	pub slug: String,
	pub accounts: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub asns: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RIR {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	/// IP space managed by this RIR is considered private
	pub is_private: Option<bool>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub aggregate_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RIRRequest {
	pub name: String,
	pub slug: String,
	/// IP space managed by this RIR is considered private
	pub is_private: bool,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Rack {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub facility_id: Option<String>,
	pub site: Option<BriefSite>,
	pub location: Option<BriefLocation>,
	pub tenant: Option<BriefTenant>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<BriefRackRole>,
	pub serial: Option<String>,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub rack_type: Option<BriefRackType>,
	pub form_factor: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub width: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Height in rack units
	pub u_height: Option<u8>,
	/// Starting unit for rack
	pub starting_unit: Option<u16>,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Units are numbered top-to-bottom
	pub desc_units: Option<bool>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	pub outer_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub airflow: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub device_count: Option<i64>,
	pub powerfeed_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackRequest {
	pub name: String,
	pub facility_id: Option<String>,
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub rack_type: Option<serde_json::Value>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: i64,
	/// Height in rack units
	pub u_height: u8,
	/// Starting unit for rack
	pub starting_unit: u16,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	pub airflow: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackReservation {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub rack: Option<BriefRack>,
	pub units: Option<Vec<u16>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub user: Option<BriefUser>,
	pub tenant: Option<BriefTenant>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackReservationRequest {
	pub rack: serde_json::Value,
	pub units: Vec<u16>,
	pub user: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackRole {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub rack_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackType {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub manufacturer: Option<BriefManufacturer>,
	pub model: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub form_factor: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub width: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Height in rack units
	pub u_height: Option<u8>,
	/// Starting unit for rack
	pub starting_unit: Option<u16>,
	/// Units are numbered top-to-bottom
	pub desc_units: Option<bool>,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	pub outer_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	pub weight_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackTypeRequest {
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
	pub description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: i64,
	/// Height in rack units
	pub u_height: u8,
	/// Starting unit for rack
	pub starting_unit: u16,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RackUnit {
	pub id: f64,
	pub name: Option<String>,
	pub face: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub device: Option<BriefDevice>,
	pub occupied: Option<bool>,
	pub display: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPort {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub device: Option<BriefDevice>,
	pub module: Option<BriefModule>,
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub color: Option<String>,
	/// Number of front ports which may be mapped
	pub positions: Option<u16>,
	pub description: Option<String>,
	/// Treat as if a cable is connected
	pub mark_connected: Option<bool>,
	pub cable: Option<BriefCable>,
	pub cable_end: Option<String>,
	pub link_peers: Option<Vec<serde_json::Value>>,
	/// Return the type of the peer link terminations, or None.
	pub link_peers_type: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub _occupied: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	/// Number of front ports which may be mapped
	pub positions: u16,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPortTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub device_type: Option<BriefDeviceType>,
	pub module_type: Option<BriefModuleType>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: Option<String>,
	/// Physical label
	pub label: Option<String>,
	pub r#type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub color: Option<String>,
	pub positions: Option<u16>,
	pub description: Option<String>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RearPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub positions: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Region {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub parent: Option<NestedRegion>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub site_count: Option<i64>,
	pub prefix_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RegionRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<NestedRegionRequest>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Role {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub weight: Option<u16>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub prefix_count: Option<i64>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RoleRequest {
	pub name: String,
	pub slug: String,
	pub weight: u16,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RouteTarget {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Route target value (formatted in accordance with RFC 4360)
	pub name: Option<String>,
	pub tenant: Option<BriefTenant>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RouteTargetRequest {
	/// Route target value (formatted in accordance with RFC 4360)
	pub name: String,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SavedFilter {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_types: Option<Vec<String>>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub user: Option<i64>,
	pub weight: Option<u16>,
	pub enabled: Option<bool>,
	pub shared: Option<bool>,
	pub parameters: Option<serde_json::Value>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SavedFilterRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub slug: String,
	pub description: String,
	pub user: Option<i64>,
	pub weight: u16,
	pub enabled: bool,
	pub shared: bool,
	pub parameters: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Script {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub module: Option<i64>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub vars: Option<serde_json::Value>,
	pub result: Option<BriefJob>,
	pub display: Option<String>,
	pub is_executable: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ScriptInputRequest {
	pub data: serde_json::Value,
	pub commit: bool,
	pub schedule_at: Option<String>,
	pub interval: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Service {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub parent_object_type: Option<String>,
	pub parent_object_id: Option<u64>,
	pub parent: Option<serde_json::Value>,
	pub name: Option<String>,
	pub protocol: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ports: Option<Vec<u16>>,
	pub ipaddresses: Option<Vec<IPAddress>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceRequest {
	pub parent_object_type: String,
	pub parent_object_id: u64,
	pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub ports: Vec<u16>,
	pub ipaddresses: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceTemplate {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub protocol: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ports: Option<Vec<u16>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceTemplateRequest {
	pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub ports: Vec<u16>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Site {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Full name of the site
	pub name: Option<String>,
	pub slug: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub region: Option<BriefRegion>,
	pub group: Option<BriefSiteGroup>,
	pub tenant: Option<BriefTenant>,
	/// Local facility ID or description
	pub facility: Option<String>,
	pub time_zone: Option<String>,
	pub description: Option<String>,
	/// Physical location of the building
	pub physical_address: Option<String>,
	/// If different from the physical address
	pub shipping_address: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub comments: Option<String>,
	pub asns: Option<Vec<ASN>>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub circuit_count: Option<i64>,
	pub device_count: Option<i64>,
	pub prefix_count: Option<i64>,
	pub rack_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
	pub vlan_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub parent: Option<NestedSiteGroup>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub site_count: Option<i64>,
	pub prefix_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<NestedSiteGroupRequest>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteRequest {
	/// Full name of the site
	pub name: String,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub region: Option<serde_json::Value>,
	pub group: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// Local facility ID or description
	pub facility: String,
	pub time_zone: Option<String>,
	pub description: String,
	/// Physical location of the building
	pub physical_address: String,
	/// If different from the physical address
	pub shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub comments: String,
	pub asns: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Subscription {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<u64>,
	pub object: Option<serde_json::Value>,
	pub user: Option<BriefUser>,
	pub created: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubscriptionRequest {
	pub object_type: String,
	pub object_id: u64,
	pub user: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TableConfig {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub table: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub user: Option<i64>,
	pub weight: Option<u16>,
	pub enabled: Option<bool>,
	pub shared: Option<bool>,
	pub columns: Option<Vec<String>>,
	pub ordering: Option<Vec<String>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TableConfigRequest {
	pub object_type: String,
	pub table: String,
	pub name: String,
	pub description: String,
	pub user: Option<i64>,
	pub weight: u16,
	pub enabled: bool,
	pub shared: bool,
	pub columns: Vec<String>,
	pub ordering: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tag {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	pub description: Option<String>,
	pub weight: Option<u16>,
	pub object_types: Option<Vec<String>>,
	pub tagged_items: Option<i64>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TagRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub weight: u16,
	pub object_types: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaggedItem {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub object_type: Option<String>,
	pub object_id: Option<i32>,
	pub object: Option<serde_json::Value>,
	pub tag: Option<BriefTag>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tenant {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub group: Option<BriefTenantGroup>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub circuit_count: Option<i64>,
	pub device_count: Option<i64>,
	pub ipaddress_count: Option<i64>,
	pub prefix_count: Option<i64>,
	pub rack_count: Option<i64>,
	pub site_count: Option<i64>,
	pub virtualmachine_count: Option<i64>,
	pub vlan_count: Option<i64>,
	pub vrf_count: Option<i64>,
	pub cluster_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenantGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub parent: Option<NestedTenantGroup>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub tenant_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenantGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<NestedTenantGroupRequest>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenantRequest {
	pub name: String,
	pub slug: String,
	pub group: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Token {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub user: Option<BriefUser>,
	pub created: Option<String>,
	pub expires: Option<String>,
	pub last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	pub write_enabled: Option<bool>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenProvision {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub user: Option<BriefUser>,
	pub created: Option<String>,
	pub expires: Option<String>,
	pub last_used: Option<String>,
	pub key: Option<String>,
	/// Permit create/update/delete operations using this key
	pub write_enabled: Option<bool>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenProvisionRequest {
	pub expires: Option<String>,
	/// Permit create/update/delete operations using this key
	pub write_enabled: bool,
	pub description: String,
	pub username: String,
	pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenRequest {
	pub user: serde_json::Value,
	pub expires: Option<String>,
	pub last_used: Option<String>,
	pub key: String,
	/// Permit create/update/delete operations using this key
	pub write_enabled: bool,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tunnel {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub group: Option<BriefTunnelGroup>,
	pub encapsulation: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub ipsec_profile: Option<BriefIPSecProfile>,
	pub tenant: Option<BriefTenant>,
	pub tunnel_id: Option<u64>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub terminations_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub tunnel_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelGroupRequest {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelRequest {
	pub name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	pub status: String,
	pub group: Option<serde_json::Value>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	/// * `wireguard` - WireGuard
	/// * `openvpn` - OpenVPN
	/// * `l2tp` - L2TP
	/// * `pptp` - PPTP
	pub encapsulation: String,
	pub ipsec_profile: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub tunnel_id: Option<u64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub tunnel: Option<BriefTunnel>,
	pub role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub termination_type: Option<String>,
	pub termination_id: Option<u64>,
	pub termination: Option<serde_json::Value>,
	pub outside_ip: Option<BriefIPAddress>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TunnelTerminationRequest {
	pub tunnel: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub termination_type: String,
	pub termination_id: Option<u64>,
	pub outside_ip: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: Option<String>,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub email: Option<String>,
	/// Designates whether the user can log into this admin site.
	pub is_staff: Option<bool>,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	pub is_active: Option<bool>,
	pub date_joined: Option<String>,
	pub last_login: Option<String>,
	pub groups: Option<Vec<Group>>,
	pub permissions: Option<Vec<ObjectPermission>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	pub username: String,
	pub password: String,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
	/// Designates whether the user can log into this admin site.
	pub is_staff: bool,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	pub is_active: bool,
	pub date_joined: String,
	pub last_login: Option<String>,
	pub groups: Vec<i64>,
	pub permissions: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLAN {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub site: Option<BriefSite>,
	pub group: Option<BriefVLANGroup>,
	/// Numeric VLAN ID (1-4094)
	pub vid: Option<u16>,
	pub name: Option<String>,
	pub tenant: Option<BriefTenant>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub role: Option<BriefRole>,
	pub description: Option<String>,
	pub qinq_role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub qinq_svlan: Option<NestedVLAN>,
	pub comments: Option<String>,
	pub l2vpn_termination: Option<BriefL2VPNTermination>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub prefix_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub scope: Option<serde_json::Value>,
	pub vid_ranges: Option<Vec<IntegerRange>>,
	pub tenant: Option<BriefTenant>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub vlan_count: Option<i64>,
	pub utilization: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub vid_ranges: Vec<IntegerRangeRequest>,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANRequest {
	pub site: Option<serde_json::Value>,
	pub group: Option<serde_json::Value>,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
	pub name: String,
	pub tenant: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub description: String,
	/// * `svlan` - Service
	/// * `cvlan` - Customer
	pub qinq_role: Option<String>,
	pub qinq_svlan: Option<NestedVLANRequest>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationPolicy {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub rules: Option<Vec<VLANTranslationRule>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationPolicyRequest {
	pub name: String,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationRule {
	pub id: i64,
	pub url: Option<String>,
	pub display: Option<String>,
	pub policy: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	pub local_vid: Option<u16>,
	/// Numeric VLAN ID (1-4094)
	pub remote_vid: Option<u16>,
	pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VLANTranslationRuleRequest {
	pub policy: i64,
	/// Numeric VLAN ID (1-4094)
	pub local_vid: u16,
	/// Numeric VLAN ID (1-4094)
	pub remote_vid: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VMInterface {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub virtual_machine: Option<BriefVirtualMachine>,
	pub name: Option<String>,
	pub enabled: Option<bool>,
	pub parent: Option<NestedVMInterface>,
	pub bridge: Option<NestedVMInterface>,
	pub mtu: Option<u32>,
	pub mac_address: Option<String>,
	pub primary_mac_address: Option<BriefMACAddress>,
	pub mac_addresses: Option<Vec<BriefMACAddress>>,
	pub description: Option<String>,
	pub mode: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub untagged_vlan: Option<BriefVLAN>,
	pub tagged_vlans: Option<Vec<VLAN>>,
	pub qinq_svlan: Option<BriefVLAN>,
	pub vlan_translation_policy: Option<BriefVLANTranslationPolicy>,
	pub vrf: Option<BriefVRF>,
	pub l2vpn_termination: Option<BriefL2VPNTermination>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub count_ipaddresses: Option<i64>,
	pub count_fhrp_groups: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VMInterfaceRequest {
	pub virtual_machine: serde_json::Value,
	pub name: String,
	pub enabled: bool,
	pub parent: Option<NestedVMInterfaceRequest>,
	pub bridge: Option<NestedVMInterfaceRequest>,
	pub mtu: Option<u32>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub description: String,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: String,
	pub untagged_vlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VRF {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub tenant: Option<BriefTenant>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	pub enforce_unique: Option<bool>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub import_targets: Option<Vec<RouteTarget>>,
	pub export_targets: Option<Vec<RouteTarget>>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub ipaddress_count: Option<i64>,
	pub prefix_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VRFRequest {
	pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	pub rd: Option<String>,
	pub tenant: Option<serde_json::Value>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	pub enforce_unique: bool,
	pub description: String,
	pub comments: String,
	pub import_targets: Vec<i64>,
	pub export_targets: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualChassis {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub domain: Option<String>,
	pub master: Option<NestedDevice>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub member_count: Option<i64>,
	pub members: Option<Vec<NestedDevice>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualChassisRequest {
	pub name: String,
	pub domain: String,
	pub master: Option<NestedDeviceRequest>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuit {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	/// Unique circuit ID
	pub cid: Option<String>,
	pub provider_network: Option<BriefProviderNetwork>,
	pub provider_account: Option<BriefProviderAccount>,
	pub r#type: Option<BriefVirtualCircuitType>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tenant: Option<BriefTenant>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider_network: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitTermination {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub virtual_circuit: Option<BriefVirtualCircuit>,
	pub role: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub interface: Option<BriefInterface>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitTerminationRequest {
	pub virtual_circuit: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub interface: serde_json::Value,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitType {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub color: Option<String>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub virtual_circuit_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualCircuitTypeRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDeviceContext {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub device: Option<BriefDevice>,
	pub identifier: Option<u16>,
	pub tenant: Option<BriefTenant>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub interface_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDeviceContextRequest {
	pub name: String,
	pub device: serde_json::Value,
	pub identifier: Option<u16>,
	pub tenant: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	pub status: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDisk {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub virtual_machine: Option<BriefVirtualMachine>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub size: Option<u32>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualDiskRequest {
	pub virtual_machine: serde_json::Value,
	pub name: String,
	pub description: String,
	pub size: u32,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualMachineWithConfigContext {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub site: Option<BriefSite>,
	pub cluster: Option<BriefCluster>,
	pub device: Option<BriefDevice>,
	pub serial: Option<String>,
	pub role: Option<BriefDeviceRole>,
	pub tenant: Option<BriefTenant>,
	pub platform: Option<BriefPlatform>,
	pub primary_ip: Option<BriefIPAddress>,
	pub primary_ip4: Option<BriefIPAddress>,
	pub primary_ip6: Option<BriefIPAddress>,
	pub vcpus: Option<f64>,
	pub memory: Option<u32>,
	pub disk: Option<u32>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub config_template: Option<BriefConfigTemplate>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub config_context: Option<serde_json::Value>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub interface_count: Option<i64>,
	pub virtual_disk_count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualMachineWithConfigContextRequest {
	pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	/// * `paused` - Paused
	pub status: String,
	pub site: Option<serde_json::Value>,
	pub cluster: Option<serde_json::Value>,
	pub device: Option<serde_json::Value>,
	pub serial: String,
	pub role: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub vcpus: Option<f64>,
	pub memory: Option<u32>,
	pub disk: Option<u32>,
	pub description: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Webhook {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	pub payload_url: Option<String>,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	pub http_method: Option<String>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	pub http_content_type: Option<String>,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	pub additional_headers: Option<String>,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	pub body_template: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	pub secret: Option<String>,
	/// Enable SSL certificate verification. Disable with caution!
	pub ssl_verification: Option<bool>,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	pub ca_file_path: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Option<Vec<NestedTag>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WebhookRequest {
	pub name: String,
	pub description: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	pub payload_url: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	pub http_method: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	pub http_content_type: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	pub additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	pub body_template: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	pub secret: String,
	/// Enable SSL certificate verification. Disable with caution!
	pub ssl_verification: bool,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	pub ca_file_path: Option<String>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLAN {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub ssid: Option<String>,
	pub description: Option<String>,
	pub group: Option<BriefWirelessLANGroup>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub vlan: Option<BriefVLAN>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub scope: Option<serde_json::Value>,
	pub tenant: Option<BriefTenant>,
	pub auth_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub auth_cipher: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub auth_psk: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLANGroup {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub parent: Option<NestedWirelessLANGroup>,
	pub description: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
	pub wirelesslan_count: Option<i64>,
	pub comments: Option<String>,
	pub _depth: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<NestedWirelessLANGroupRequest>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLANRequest {
	pub ssid: String,
	pub description: String,
	pub group: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	pub status: String,
	pub vlan: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub tenant: Option<serde_json::Value>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: String,
	pub auth_psk: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLink {
	pub id: i64,
	pub url: Option<String>,
	pub display_url: Option<String>,
	pub display: Option<String>,
	pub interface_a: Option<BriefInterface>,
	pub interface_b: Option<BriefInterface>,
	pub ssid: Option<String>,
	pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tenant: Option<BriefTenant>,
	pub auth_type: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub auth_cipher: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub auth_psk: Option<String>,
	pub distance: Option<f64>,
	pub distance_unit: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub description: Option<String>,
	pub comments: Option<String>,
	pub tags: Option<Vec<NestedTag>>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub created: Option<String>,
	pub last_updated: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessLinkRequest {
	pub interface_a: serde_json::Value,
	pub interface_b: serde_json::Value,
	pub ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: String,
	pub auth_psk: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableAggregateRequest {
	pub prefix: String,
	pub rir: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub date_added: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCableRequest {
	/// * `cat3` - CAT3
	/// * `cat5` - CAT5
	/// * `cat5e` - CAT5e
	/// * `cat6` - CAT6
	/// * `cat6a` - CAT6a
	/// * `cat7` - CAT7
	/// * `cat7a` - CAT7a
	/// * `cat8` - CAT8
	/// * `dac-active` - Direct Attach Copper (Active)
	/// * `dac-passive` - Direct Attach Copper (Passive)
	/// * `mrj21-trunk` - MRJ21 Trunk
	/// * `coaxial` - Coaxial
	/// * `mmf` - Multimode Fiber
	/// * `mmf-om1` - Multimode Fiber (OM1)
	/// * `mmf-om2` - Multimode Fiber (OM2)
	/// * `mmf-om3` - Multimode Fiber (OM3)
	/// * `mmf-om4` - Multimode Fiber (OM4)
	/// * `mmf-om5` - Multimode Fiber (OM5)
	/// * `smf` - Singlemode Fiber
	/// * `smf-os1` - Singlemode Fiber (OS1)
	/// * `smf-os2` - Singlemode Fiber (OS2)
	/// * `aoc` - Active Optical Cabling (AOC)
	/// * `usb` - USB
	/// * `power` - Power
	pub r#type: Option<String>,
	pub a_terminations: Vec<GenericObjectRequest>,
	pub b_terminations: Vec<GenericObjectRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub label: String,
	pub color: String,
	pub length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	pub length_unit: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCircuitGroupAssignmentRequest {
	pub group: serde_json::Value,
	pub member_type: String,
	pub member_id: u64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: Option<String>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub install_date: Option<String>,
	pub termination_date: Option<String>,
	/// Committed rate
	pub commit_rate: Option<u32>,
	pub description: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub assignments: Vec<BriefCircuitGroupAssignmentSerializer_Request>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableClusterRequest {
	pub name: String,
	pub r#type: serde_json::Value,
	pub group: Option<serde_json::Value>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsolePortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsolePortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsoleServerPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableConsoleServerPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableContactAssignmentRequest {
	pub object_type: String,
	pub object_id: u64,
	pub contact: serde_json::Value,
	pub role: Option<serde_json::Value>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	pub priority: Option<String>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableContactGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCustomFieldChoiceSetRequest {
	pub name: String,
	pub description: String,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	pub base_choices: Option<String>,
	pub extra_choices: Vec<Vec<serde_json::Value>>,
	/// Choices are automatically ordered alphabetically
	pub order_alphabetically: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableCustomFieldRequest {
	pub object_types: Vec<String>,
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
	pub related_object_type: Option<String>,
	/// Internal field name
	pub name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	pub label: String,
	/// Custom fields within the same group will be displayed together
	pub group_name: String,
	pub description: String,
	/// This field is required when creating new objects or editing an existing object.
	pub required: bool,
	/// The value of this field must be unique for the assigned object
	pub unique: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	pub search_weight: u16,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	pub filter_logic: String,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	pub ui_visible: String,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	pub ui_editable: String,
	/// Replicate this value when cloning objects
	pub is_cloneable: bool,
	/// Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. "Foo").
	pub default: Option<serde_json::Value>,
	/// Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. "Foo").
	pub related_object_filter: Option<serde_json::Value>,
	/// Fields with higher weights appear lower in a form.
	pub weight: u16,
	/// Minimum allowed value (for numeric fields)
	pub validation_minimum: Option<i64>,
	/// Maximum allowed value (for numeric fields)
	pub validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	pub validation_regex: String,
	pub choice_set: Option<serde_json::Value>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDataSourceRequest {
	pub name: String,
	pub r#type: String,
	pub source_url: String,
	pub enabled: bool,
	pub description: String,
	/// * `1` - Minutely
	/// * `60` - Hourly
	/// * `720` - 12 hours
	/// * `1440` - Daily
	/// * `10080` - Weekly
	/// * `43200` - 30 days
	pub sync_interval: Option<u16>,
	pub parameters: Option<serde_json::Value>,
	/// Patterns (one per line) matching files to ignore when syncing
	pub ignore_rules: String,
	pub comments: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDeviceRoleRequest {
	pub name: String,
	pub slug: String,
	pub color: String,
	/// Virtual machines may be assigned to this role
	pub vm_role: bool,
	pub config_template: Option<serde_json::Value>,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDeviceTypeRequest {
	pub manufacturer: serde_json::Value,
	pub default_platform: Option<serde_json::Value>,
	pub model: String,
	pub slug: String,
	/// Discrete part number (optional)
	pub part_number: String,
	pub u_height: f64,
	/// Devices of this type are excluded when calculating rack utilization.
	pub exclude_from_utilization: bool,
	/// Device consumes both front and rear rack faces.
	pub is_full_depth: bool,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	pub subdevice_role: Option<String>,
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
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	pub front_image: Option<String>,
	pub rear_image: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableDeviceWithConfigContextRequest {
	pub name: Option<String>,
	pub device_type: serde_json::Value,
	pub role: serde_json::Value,
	pub tenant: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	/// Chassis serial number, assigned by the manufacturer
	pub serial: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub rack: Option<serde_json::Value>,
	pub position: Option<f64>,
	/// * `front` - Front
	/// * `rear` - Rear
	pub face: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	pub status: String,
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
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub oob_ip: Option<serde_json::Value>,
	pub cluster: Option<serde_json::Value>,
	pub virtual_chassis: Option<serde_json::Value>,
	pub vc_position: Option<u8>,
	/// Virtual chassis master election priority
	pub vc_priority: Option<u8>,
	pub description: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableEventRuleRequest {
	pub object_types: Vec<String>,
	pub name: String,
	pub enabled: bool,
	/// The types of event which will trigger this rule.
	pub event_types: Vec<String>,
	/// A set of conditions which determine whether the event will be generated.
	pub conditions: Option<serde_json::Value>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	/// * `notification` - Notification
	pub action_type: String,
	pub action_object_type: String,
	pub action_object_id: Option<u64>,
	pub description: String,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub tags: Vec<NestedTagRequest>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableFrontPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub rear_port: i64,
	/// Mapped position on corresponding rear port
	pub rear_port_position: u16,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableFrontPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub rear_port: serde_json::Value,
	pub rear_port_position: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIKEPolicyRequest {
	pub name: String,
	pub description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	pub version: u16,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	pub mode: Option<String>,
	pub proposals: Vec<i64>,
	pub preshared_key: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIKEProposalRequest {
	pub name: String,
	pub description: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	pub authentication_method: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - 3DES
	/// * `des-cbc` - DES
	pub encryption_algorithm: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: Option<String>,
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
	/// Security association lifetime (in seconds)
	pub sa_lifetime: Option<u32>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPAddressRequest {
	pub address: String,
	pub vrf: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	pub status: String,
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
	pub assigned_object_type: Option<String>,
	pub assigned_object_id: Option<u64>,
	/// The IP for which this address is the "outside" IP
	pub nat_inside: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	pub dns_name: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPRangeRequest {
	pub start_address: String,
	pub end_address: String,
	pub vrf: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Prevent the creation of IP addresses within this range
	pub mark_populated: bool,
	/// Report space as 100% utilized
	pub mark_utilized: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPSecPolicyRequest {
	pub name: String,
	pub description: String,
	pub proposals: Vec<i64>,
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
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPSecProfileRequest {
	pub name: String,
	pub description: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	pub mode: String,
	pub ike_policy: serde_json::Value,
	pub ipsec_policy: serde_json::Value,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableIPSecProposalRequest {
	pub name: String,
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
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	pub authentication_algorithm: Option<String>,
	/// Security association lifetime (seconds)
	pub sa_lifetime_seconds: Option<u32>,
	/// Security association lifetime (in kilobytes)
	pub sa_lifetime_data: Option<u32>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableInterfaceRequest {
	pub device: serde_json::Value,
	pub vdcs: Vec<i64>,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
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
	/// * `ieee802.11n` - IEEE 802.11n
	/// * `ieee802.11ac` - IEEE 802.11ac
	/// * `ieee802.11ad` - IEEE 802.11ad
	/// * `ieee802.11ax` - IEEE 802.11ax
	/// * `ieee802.11ay` - IEEE 802.11ay
	/// * `ieee802.11be` - IEEE 802.11be
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
	pub enabled: bool,
	pub parent: Option<i64>,
	pub bridge: Option<i64>,
	pub lag: Option<i64>,
	pub mtu: Option<u32>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub speed: Option<u32>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	pub duplex: Option<String>,
	pub wwn: Option<String>,
	/// This interface is used only for out-of-band management
	pub mgmt_only: bool,
	pub description: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	pub rf_role: Option<String>,
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
	/// Populated by selected channel (if set)
	pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	pub rf_channel_width: Option<f64>,
	pub tx_power: Option<u8>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub wireless_lans: Vec<i64>,
	pub vrf: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableInterfaceTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `1000base-sx` - 1000BASE-SX (1GE)
	/// * `1000base-lx` - 1000BASE-LX (1GE)
	/// * `1000base-tx` - 1000BASE-TX (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
	/// * `100base-x-sfp` - SFP (100ME)
	/// * `1000base-x-gbic` - GBIC (1GE)
	/// * `1000base-x-sfp` - SFP (1GE)
	/// * `10gbase-x-sfpp` - SFP+ (10GE)
	/// * `10gbase-x-xfp` - XFP (10GE)
	/// * `10gbase-x-xenpak` - XENPAK (10GE)
	/// * `10gbase-x-x2` - X2 (10GE)
	/// * `25gbase-x-sfp28` - SFP28 (25GE)
	/// * `50gbase-x-sfp56` - SFP56 (50GE)
	/// * `40gbase-x-qsfpp` - QSFP+ (40GE)
	/// * `50gbase-x-sfp28` - QSFP28 (50GE)
	/// * `100gbase-x-cfp` - CFP (100GE)
	/// * `100gbase-x-cfp2` - CFP2 (100GE)
	/// * `200gbase-x-cfp2` - CFP2 (200GE)
	/// * `400gbase-x-cfp2` - CFP2 (400GE)
	/// * `100gbase-x-cfp4` - CFP4 (100GE)
	/// * `100gbase-x-cxp` - CXP (100GE)
	/// * `100gbase-x-cpak` - Cisco CPAK (100GE)
	/// * `100gbase-x-dsfp` - DSFP (100GE)
	/// * `100gbase-x-sfpdd` - SFP-DD (100GE)
	/// * `100gbase-x-qsfp28` - QSFP28 (100GE)
	/// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
	/// * `200gbase-x-qsfp56` - QSFP56 (200GE)
	/// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
	/// * `400gbase-x-qsfp112` - QSFP112 (400GE)
	/// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
	/// * `400gbase-x-osfp` - OSFP (400GE)
	/// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
	/// * `400gbase-x-cdfp` - CDFP (400GE)
	/// * `400gbase-x-cfp8` - CPF8 (400GE)
	/// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
	/// * `800gbase-x-osfp` - OSFP (800GE)
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
	/// * `ieee802.11n` - IEEE 802.11n
	/// * `ieee802.11ac` - IEEE 802.11ac
	/// * `ieee802.11ad` - IEEE 802.11ad
	/// * `ieee802.11ax` - IEEE 802.11ax
	/// * `ieee802.11ay` - IEEE 802.11ay
	/// * `ieee802.11be` - IEEE 802.11be
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
	pub enabled: bool,
	pub mgmt_only: bool,
	pub description: String,
	pub bridge: Option<i64>,
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
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableInventoryItemRequest {
	pub device: serde_json::Value,
	pub parent: Option<i64>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub manufacturer: Option<serde_json::Value>,
	/// Manufacturer-assigned part identifier
	pub part_id: String,
	pub serial: String,
	/// A unique tag used to identify this item
	pub asset_tag: Option<String>,
	/// This item was automatically discovered
	pub discovered: bool,
	pub description: String,
	pub component_type: Option<String>,
	pub component_id: Option<u64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableJournalEntryRequest {
	pub assigned_object_type: String,
	pub assigned_object_id: u64,
	pub created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	pub kind: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableL2VPNRequest {
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
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub import_targets: Vec<i64>,
	pub export_targets: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub tenant: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableLocationRequest {
	pub name: String,
	pub slug: String,
	pub site: serde_json::Value,
	pub parent: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	/// Local facility ID or description
	pub facility: String,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableModuleRequest {
	pub device: serde_json::Value,
	pub module_bay: i64,
	pub module_type: serde_json::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub serial: String,
	/// A unique tag used to identify this device
	pub asset_tag: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableModuleTypeRequest {
	pub profile: Option<serde_json::Value>,
	pub manufacturer: serde_json::Value,
	pub model: String,
	/// Discrete part number (optional)
	pub part_number: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	pub airflow: Option<String>,
	pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	pub description: String,
	pub attributes: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerFeedRequest {
	pub power_panel: serde_json::Value,
	pub rack: Option<serde_json::Value>,
	pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	pub status: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	pub r#type: String,
	/// * `ac` - AC
	/// * `dc` - DC
	pub supply: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	pub phase: String,
	pub voltage: i16,
	pub amperage: u16,
	/// Maximum permissible draw (percentage)
	pub max_utilization: u8,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub description: String,
	pub tenant: Option<serde_json::Value>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerOutletRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// Physical port type
	/// 
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
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
	/// * `enabled` - Enabled
	/// * `disabled` - Disabled
	/// * `faulty` - Faulty
	pub status: String,
	pub color: String,
	pub power_port: Option<serde_json::Value>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerOutletTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c5` - C5
	/// * `iec-60320-c7` - C7
	/// * `iec-60320-c13` - C13
	/// * `iec-60320-c15` - C15
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
	pub power_port: Option<serde_json::Value>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	pub feed_leg: Option<String>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
	/// Physical port type
	/// 
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
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
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePowerPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
	/// * `iec-60320-c6` - C6
	/// * `iec-60320-c8` - C8
	/// * `iec-60320-c14` - C14
	/// * `iec-60320-c16` - C16
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
	/// Maximum power draw (watts)
	pub maximum_draw: Option<u32>,
	/// Allocated power draw (watts)
	pub allocated_draw: Option<u32>,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritablePrefixRequest {
	pub prefix: String,
	pub vrf: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub tenant: Option<serde_json::Value>,
	pub vlan: Option<serde_json::Value>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	/// All IP addresses within this prefix are considered usable
	pub is_pool: bool,
	/// Treat as fully utilized
	pub mark_utilized: bool,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRackRequest {
	pub name: String,
	pub facility_id: Option<String>,
	pub site: serde_json::Value,
	pub location: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub serial: String,
	/// A unique tag used to identify this rack
	pub asset_tag: Option<String>,
	pub rack_type: Option<serde_json::Value>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: u16,
	/// Height in rack units
	pub u_height: u8,
	/// Starting unit for rack
	pub starting_unit: u16,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	pub airflow: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRackTypeRequest {
	pub manufacturer: serde_json::Value,
	pub model: String,
	pub slug: String,
	pub description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	pub form_factor: String,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	pub width: u16,
	/// Height in rack units
	pub u_height: u8,
	/// Starting unit for rack
	pub starting_unit: u16,
	/// Units are numbered top-to-bottom
	pub desc_units: bool,
	/// Outer dimension of rack (width)
	pub outer_width: Option<u16>,
	/// Outer dimension of rack (height)
	pub outer_height: Option<u16>,
	/// Outer dimension of rack (depth)
	pub outer_depth: Option<u16>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	pub outer_unit: Option<String>,
	pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	pub max_weight: Option<u32>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	pub weight_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	pub mounting_depth: Option<u16>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRearPortRequest {
	pub device: serde_json::Value,
	pub module: Option<serde_json::Value>,
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	/// Number of front ports which may be mapped
	pub positions: u16,
	pub description: String,
	/// Treat as if a cable is connected
	pub mark_connected: bool,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRearPortTemplateRequest {
	pub device_type: Option<serde_json::Value>,
	pub module_type: Option<serde_json::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	pub name: String,
	/// Physical label
	pub label: String,
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
	pub color: String,
	pub positions: u16,
	pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableRegionRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableServiceRequest {
	pub parent_object_type: String,
	pub parent_object_id: u64,
	pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub ports: Vec<u16>,
	pub ipaddresses: Vec<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableServiceTemplateRequest {
	pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	pub protocol: String,
	pub ports: Vec<u16>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableSiteGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableSiteRequest {
	/// Full name of the site
	pub name: String,
	pub slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	pub status: String,
	pub region: Option<serde_json::Value>,
	pub group: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	/// Local facility ID or description
	pub facility: String,
	pub time_zone: Option<String>,
	pub description: String,
	/// Physical location of the building
	pub physical_address: String,
	/// If different from the physical address
	pub shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	pub longitude: Option<f64>,
	pub comments: String,
	pub asns: Vec<i64>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableTenantGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableTunnelRequest {
	pub name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	pub status: String,
	pub group: Option<serde_json::Value>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	/// * `wireguard` - WireGuard
	/// * `openvpn` - OpenVPN
	/// * `l2tp` - L2TP
	/// * `pptp` - PPTP
	pub encapsulation: String,
	pub ipsec_profile: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub tunnel_id: Option<u64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableTunnelTerminationRequest {
	pub tunnel: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub termination_type: String,
	pub termination_id: Option<u64>,
	pub outside_ip: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVLANRequest {
	pub site: Option<serde_json::Value>,
	pub group: Option<serde_json::Value>,
	/// Numeric VLAN ID (1-4094)
	pub vid: u16,
	pub name: String,
	pub tenant: Option<serde_json::Value>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	pub status: String,
	pub role: Option<serde_json::Value>,
	pub description: String,
	/// Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)
	/// 
	/// * `svlan` - Service
	/// * `cvlan` - Customer
	pub qinq_role: Option<String>,
	pub qinq_svlan: Option<i64>,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVMInterfaceRequest {
	pub virtual_machine: serde_json::Value,
	pub name: String,
	pub enabled: bool,
	pub parent: Option<i64>,
	pub bridge: Option<i64>,
	pub mtu: Option<u32>,
	pub primary_mac_address: Option<serde_json::Value>,
	pub description: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	/// * `q-in-q` - Q-in-Q (802.1ad)
	pub mode: Option<String>,
	pub untagged_vlan: Option<serde_json::Value>,
	pub tagged_vlans: Vec<i64>,
	pub qinq_svlan: Option<serde_json::Value>,
	pub vlan_translation_policy: Option<serde_json::Value>,
	pub vrf: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualChassisRequest {
	pub name: String,
	pub domain: String,
	pub master: Option<i64>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualCircuitRequest {
	/// Unique circuit ID
	pub cid: String,
	pub provider_network: serde_json::Value,
	pub provider_account: Option<serde_json::Value>,
	pub r#type: serde_json::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualCircuitTerminationRequest {
	pub virtual_circuit: serde_json::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	pub role: String,
	pub interface: serde_json::Value,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualDeviceContextRequest {
	pub name: String,
	pub device: serde_json::Value,
	pub identifier: Option<u16>,
	pub tenant: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	pub status: String,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableVirtualMachineWithConfigContextRequest {
	pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	/// * `paused` - Paused
	pub status: String,
	pub site: Option<serde_json::Value>,
	pub cluster: Option<serde_json::Value>,
	pub device: Option<serde_json::Value>,
	pub serial: String,
	pub role: Option<serde_json::Value>,
	pub tenant: Option<serde_json::Value>,
	pub platform: Option<serde_json::Value>,
	pub primary_ip4: Option<serde_json::Value>,
	pub primary_ip6: Option<serde_json::Value>,
	pub vcpus: Option<f64>,
	pub memory: Option<u32>,
	pub disk: Option<u32>,
	pub description: String,
	pub comments: String,
	pub config_template: Option<serde_json::Value>,
	/// Local config context data takes precedence over source contexts in the final rendered config context
	pub local_context_data: Option<serde_json::Value>,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableWirelessLANGroupRequest {
	pub name: String,
	pub slug: String,
	pub parent: Option<i64>,
	pub description: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
	pub comments: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableWirelessLANRequest {
	pub ssid: String,
	pub description: String,
	pub group: Option<serde_json::Value>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	pub status: String,
	pub vlan: Option<serde_json::Value>,
	pub scope_type: Option<String>,
	pub scope_id: Option<i64>,
	pub tenant: Option<serde_json::Value>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: Option<String>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: Option<String>,
	pub auth_psk: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WritableWirelessLinkRequest {
	pub interface_a: serde_json::Value,
	pub interface_b: serde_json::Value,
	pub ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	pub status: String,
	pub tenant: Option<serde_json::Value>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	pub auth_type: Option<String>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	pub auth_cipher: Option<String>,
	pub auth_psk: String,
	pub distance: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `mi` - Miles
	/// * `ft` - Feet
	pub distance_unit: Option<String>,
	pub description: String,
	pub comments: String,
	pub tags: Vec<NestedTagRequest>,
	pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

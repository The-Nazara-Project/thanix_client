use serde_qs;
use reqwest::Url;
use crate::util::ThanixClient;
/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantGroupRequest {
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPanelRequest {
	site: i64,
	description: String,
	comments: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	location: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsolePortTemplateRequest {
	description: String,
	module_type: Option<i64>,
	/// Physical label
	label: String,
	device_type: Option<i64>,
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
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Representation of a VLAN which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableVLAN {
	vid: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPRangeList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<IPRange>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactAssignmentRequest {
	content_type: String,
	role: i64,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	contact: i64,
	object_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCableRequest {
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	b_terminations: Vec<GenericObjectRequest>,
	tags: Vec<NestedTagRequest>,
	color: String,
	a_terminations: Vec<GenericObjectRequest>,
	length: Option<f64>,
	tenant: Option<i64>,
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
	/// * `power` - Power
	r#type: String,
	custom_fields: String,
	label: String,
	description: String,
	comments: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTunnelTerminationRequest {
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	outside_ip: Option<i64>,
	tags: Vec<NestedTagRequest>,
	tunnel: i64,
	termination_id: Option<i64>,
	termination_type: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceTemplateRequest {
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	comments: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderAccount {
	name: String,
	display: String,
	id: i64,
	url: String,
	account: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANRequest {
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	name: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableBookmarkRequest {
	object_type: String,
	user: i64,
	object_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItem {
	custom_fields: String,
	name: String,
	created: Option<String>,
	parent: Option<i64>,
	serial: String,
	description: String,
	component_id: Option<i64>,
	last_updated: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	display: String,
	/// Physical label
	label: String,
	url: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	tags: Vec<NestedTag>,
	component_type: Option<String>,
	_depth: i64,
	id: i64,
	/// This item was automatically discovered
	discovered: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLAN {
	id: i64,
	auth_type: String,
	auth_psk: String,
	auth_cipher: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	description: String,
	ssid: String,
	status: String,
	last_updated: Option<String>,
	created: Option<String>,
	comments: String,
	url: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedDashboardRequest {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTenantRequest {
	name: String,
	slug: String,
	group: Option<i64>,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitTerminationList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<CircuitTermination>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleTypeList {
	next: Option<String>,
	previous: Option<String>,
	results: Vec<ModuleType>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRoleRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRequest {
	name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecPolicyList {
	next: Option<String>,
	results: Vec<IPSecPolicy>,
	previous: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRearPortTemplate {
	url: String,
	id: i64,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRange {
	url: String,
	end: i64,
	last_updated: Option<String>,
	display: String,
	tags: Vec<NestedTag>,
	description: String,
	custom_fields: String,
	id: i64,
	created: Option<String>,
	slug: String,
	asn_count: i64,
	start: i64,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualDeviceContextList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<VirtualDeviceContext>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRouteTargetRequest {
	comments: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayTemplate {
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	id: i64,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceTypeRequest {
	description: String,
	/// Discrete part number (optional)
	part_number: String,
	u_height: f64,
	front_image: String,
	model: String,
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: Option<String>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	slug: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	weight: Option<f64>,
	custom_fields: String,
	rear_image: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: Option<String>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Group {
	url: String,
	id: i64,
	user_count: i64,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrefixRequest {
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	prefix: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	comments: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLANGroupList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<WirelessLANGroup>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigContext {
	cluster_types: Vec<i64>,
	roles: Vec<i64>,
	is_active: bool,
	locations: Vec<i64>,
	cluster_groups: Vec<i64>,
	tenant_groups: Vec<i64>,
	created: Option<String>,
	description: String,
	id: i64,
	display: String,
	url: String,
	weight: i64,
	sites: Vec<i64>,
	name: String,
	site_groups: Vec<i64>,
	clusters: Vec<i64>,
	tags: Vec<String>,
	platforms: Vec<i64>,
	tenants: Vec<i64>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	data_synced: Option<String>,
	last_updated: Option<String>,
	device_types: Vec<i64>,
	regions: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPAddress {
	id: i64,
	status: String,
	assigned_object_type: Option<String>,
	role: String,
	url: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	description: String,
	display: String,
	comments: String,
	assigned_object_id: Option<i64>,
	nat_outside: Vec<NestedIPAddress>,
	address: String,
	tags: Vec<NestedTag>,
	family: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteGroup {
	display: String,
	custom_fields: String,
	last_updated: Option<String>,
	id: i64,
	_depth: i64,
	name: String,
	url: String,
	tags: Vec<NestedTag>,
	slug: String,
	description: String,
	created: Option<String>,
	site_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackReservation {
	custom_fields: String,
	last_updated: Option<String>,
	id: i64,
	units: Vec<i64>,
	created: Option<String>,
	description: String,
	display: String,
	comments: String,
	url: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataFile {
	url: String,
	last_updated: String,
	size: i64,
	id: i64,
	display: String,
	/// File path relative to the data source's root
	path: String,
	/// SHA256 hash of the file data
	hash: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitCircuitTermination {
	description: String,
	id: i64,
	display: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	url: String,
	/// ID of the local cross-connect
	xconnect_id: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFrontPortTemplateRequest {
	color: String,
	/// Physical label
	label: String,
	rear_port_position: i64,
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
	/// * `other` - Other
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	rear_port: i64,
	module_type: Option<i64>,
	device_type: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInventoryItemRole {
	slug: String,
	display: String,
	id: i64,
	url: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerFeedRequest {
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	amperage: i64,
	tags: Vec<NestedTagRequest>,
	comments: String,
	power_panel: i64,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	rack: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	voltage: i64,
	name: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	custom_fields: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	tenant: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedL2VPNList {
	results: Vec<L2VPN>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleBayRequest {
	installed_module: i64,
	custom_fields: String,
	name: String,
	device: i64,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	description: String,
	/// Identifier to reference when renaming installed components
	position: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLANRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	group: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	vlan: Option<i64>,
	ssid: String,
	tenant: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	auth_psk: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceRoleRequest {
	color: String,
	config_template: Option<i64>,
	slug: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVMInterfaceRequest {
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	untagged_vlan: Option<i64>,
	mtu: Option<i64>,
	tags: Vec<NestedTagRequest>,
	vrf: Option<i64>,
	custom_fields: String,
	bridge: Option<i64>,
	mac_address: Option<String>,
	description: String,
	tagged_vlans: Vec<i64>,
	enabled: bool,
	parent: Option<i64>,
	virtual_machine: i64,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Platform {
	virtualmachine_count: i64,
	display: String,
	id: i64,
	last_updated: Option<String>,
	device_count: i64,
	created: Option<String>,
	slug: String,
	url: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manufacturer {
	platform_count: i64,
	description: String,
	id: i64,
	slug: String,
	url: String,
	last_updated: Option<String>,
	display: String,
	created: Option<String>,
	devicetype_count: i64,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTag>,
	inventoryitem_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelGroup {
	created: Option<String>,
	id: i64,
	custom_fields: String,
	tunnel_count: i64,
	display: String,
	last_updated: Option<String>,
	url: String,
	tags: Vec<NestedTag>,
	name: String,
	slug: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInventoryItemTemplateRequest {
	manufacturer: Option<i64>,
	device_type: i64,
	component_type: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	component_id: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	role: Option<i64>,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCableRequest {
	a_terminations: Vec<GenericObjectRequest>,
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
	/// * `power` - Power
	r#type: String,
	label: String,
	b_terminations: Vec<GenericObjectRequest>,
	length: Option<f64>,
	color: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	custom_fields: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTerminationRequest {
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	description: String,
	/// ID of the local cross-connect
	xconnect_id: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRoleRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebhookRequest {
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	tags: Vec<NestedTagRequest>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	name: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	custom_fields: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecProfileRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatformRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableASNRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	tenant: Option<i64>,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPAddressRequest {
	custom_fields: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	role: String,
	description: String,
	assigned_object_id: Option<i64>,
	address: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	comments: String,
	assigned_object_type: Option<String>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLinkRequest {
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	interface_b: i64,
	ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	interface_a: i64,
	auth_psk: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletRequest {
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
	/// * `dc-terminal` - DC Terminal
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: Option<String>,
	description: String,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Physical label
	label: String,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRegionRequest {
	description: String,
	parent: Option<i64>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderAccountRequest {
	name: String,
	account: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VRFRequest {
	comments: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	export_targets: Vec<i64>,
	name: String,
	description: String,
	import_targets: Vec<i64>,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPanelList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<PowerPanel>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCluster {
	id: i64,
	display: String,
	url: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleType {
	model: String,
	id: i64,
	url: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactGroupList {
	results: Vec<ContactGroup>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANGroup {
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	scope_id: Option<i64>,
	utilization: String,
	last_updated: Option<String>,
	name: String,
	custom_fields: String,
	created: Option<String>,
	vlan_count: i64,
	id: i64,
	display: String,
	slug: String,
	scope_type: Option<String>,
	tags: Vec<NestedTag>,
	url: String,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualDiskList {
	results: Vec<VirtualDisk>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteRequest {
	slug: String,
	/// Full name of the site
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortTemplateRequest {
	/// Physical label
	label: String,
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
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: Option<String>,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomLinkList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<CustomLink>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomFieldList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<CustomField>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCustomFieldRequest {
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	object_type: String,
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
	r#type: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Internal field name
	name: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	content_types: Vec<String>,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	choice_set: Option<i64>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderNetworkRequest {
	description: String,
	comments: String,
	provider: i64,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	service_id: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedVLANGroupRequest {
	scope_type: Option<String>,
	slug: String,
	scope_id: Option<i64>,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	custom_fields: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantGroupRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLANGroup {
	_depth: i64,
	slug: String,
	name: String,
	url: String,
	id: i64,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTagList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<Tag>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterType {
	url: String,
	name: String,
	display: String,
	slug: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedFHRPGroupRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	auth_key: String,
	description: String,
	group_id: i64,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	name: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualChassisRequest {
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	domain: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFHRPGroupAssignmentRequest {
	priority: i64,
	group: i64,
	interface_id: i64,
	interface_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePlatformRequest {
	slug: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	config_template: Option<i64>,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupRequest {
	group_id: i64,
	auth_key: String,
	description: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RIRRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	name: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecProfileRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	description: String,
	ike_policy: i64,
	custom_fields: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	ipsec_policy: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecPolicyRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLink {
	url: String,
	ssid: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRequest {
	comments: String,
	custom_fields: String,
	phone: String,
	email: String,
	address: String,
	link: String,
	description: String,
	title: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLAN {
	id: i64,
	url: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceRoleList {
	next: Option<String>,
	results: Vec<DeviceRole>,
	previous: Option<String>,
	count: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableLocationRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	parent: Option<i64>,
	description: String,
	name: String,
	slug: String,
	site: i64,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Contact {
	id: i64,
	link: String,
	display: String,
	description: String,
	address: String,
	tags: Vec<NestedTag>,
	email: String,
	comments: String,
	created: Option<String>,
	phone: String,
	url: String,
	name: String,
	last_updated: Option<String>,
	title: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRole {
	display: String,
	tags: Vec<NestedTag>,
	url: String,
	id: i64,
	name: String,
	description: String,
	custom_fields: String,
	created: Option<String>,
	slug: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventRuleRequest {
	content_types: Vec<String>,
	name: String,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	enabled: bool,
	action_object_id: Option<i64>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	custom_fields: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	description: String,
	tags: Vec<NestedTagRequest>,
	action_object_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRole {
	description: String,
	color: String,
	custom_fields: String,
	id: i64,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	display: String,
	created: Option<String>,
	inventoryitem_count: i64,
	name: String,
	slug: String,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANGroupRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFrontPortList {
	previous: Option<String>,
	results: Vec<FrontPort>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerFeedRequest {
	comments: String,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	amperage: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	description: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	rack: Option<i64>,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	voltage: i64,
	power_panel: i64,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPortRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	description: String,
	module: Option<i64>,
	/// Physical label
	label: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRIRRequest {
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactAssignmentRequest {
	contact: i64,
	role: i64,
	tags: Vec<NestedTagRequest>,
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	content_type: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedConfigTemplateRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantRequest {
	slug: String,
	description: String,
	comments: String,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProposal {
	name: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	url: String,
	encryption_algorithm: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	custom_fields: String,
	description: String,
	authentication_algorithm: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tag {
	id: i64,
	display: String,
	color: String,
	object_types: Vec<String>,
	description: String,
	slug: String,
	tagged_items: i64,
	created: Option<String>,
	last_updated: Option<String>,
	url: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableObjectPermissionRequest {
	users: Vec<i64>,
	description: String,
	enabled: bool,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	groups: Vec<i64>,
	object_types: Vec<String>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLinkRequest {
	ssid: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedEventRuleList {
	count: i64,
	next: Option<String>,
	results: Vec<EventRule>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitTypeRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsoleServerPortList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<ConsoleServerPort>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIKEPolicy {
	id: i64,
	url: String,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRoleRequest {
	weight: i64,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableASNRequest {
	tags: Vec<NestedTagRequest>,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	description: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	comments: String,
	tenant: Option<i64>,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderNetworkList {
	results: Vec<ProviderNetwork>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedManufacturerRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInventoryItemRoleRequest {
	name: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedASNRangeList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<ASNRange>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableL2VPNRequest {
	comments: String,
	identifier: Option<i64>,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	r#type: String,
	tenant: Option<i64>,
	description: String,
	slug: String,
	import_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	export_targets: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	device_type: Option<i64>,
	module_type: Option<i64>,
	description: String,
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
	r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelList {
	next: Option<String>,
	results: Vec<Tunnel>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableJournalEntryRequest {
	created_by: Option<i64>,
	assigned_object_type: String,
	custom_fields: String,
	assigned_object_id: i64,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactGroupRequest {
	custom_fields: String,
	description: String,
	parent: Option<i64>,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceWithConfigContextRequest {
	comments: String,
	custom_fields: String,
	position: Option<f64>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	vc_position: Option<i64>,
	name: Option<String>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableAggregateRequest {
	date_added: Option<String>,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	prefix: String,
	description: String,
	comments: String,
	custom_fields: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableExportTemplateRequest {
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	description: String,
	name: String,
	content_types: Vec<String>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Download file as attachment
	as_attachment: bool,
	/// Remote data source
	data_source: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFrontPortTemplateRequest {
	rear_port_position: i64,
	/// Physical label
	label: String,
	device_type: Option<i64>,
	rear_port: i64,
	description: String,
	color: String,
	module_type: Option<i64>,
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
	/// * `other` - Other
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitCircuitTerminationRequest {
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuit {
	id: i64,
	display: String,
	url: String,
	/// Unique circuit ID
	cid: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPanel {
	url: String,
	id: i64,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMInterface {
	custom_fields: String,
	last_updated: Option<String>,
	enabled: bool,
	tags: Vec<NestedTag>,
	tagged_vlans: Vec<i64>,
	mtu: Option<i64>,
	mode: String,
	name: String,
	display: String,
	count_ipaddresses: i64,
	mac_address: Option<String>,
	id: i64,
	description: String,
	created: Option<String>,
	url: String,
	count_fhrp_groups: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLink {
	created: Option<String>,
	last_updated: Option<String>,
	auth_psk: String,
	description: String,
	id: i64,
	comments: String,
	auth_cipher: String,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	ssid: String,
	url: String,
	status: String,
	auth_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNTermination {
	display: String,
	tags: Vec<NestedTag>,
	url: String,
	assigned_object_id: i64,
	custom_fields: String,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	assigned_object_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	group: Option<i64>,
	address: String,
	phone: String,
	title: String,
	email: String,
	comments: String,
	link: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLANGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	parent: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceRequest {
	wireless_lans: Vec<i64>,
	vdcs: Vec<i64>,
	/// Physical label
	label: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	tagged_vlans: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	wwn: Option<String>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
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
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
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
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
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
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
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
	r#type: String,
	tx_power: Option<i64>,
	custom_fields: String,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	name: String,
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
	rf_channel: String,
	mac_address: Option<String>,
	mtu: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	enabled: bool,
	description: String,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	speed: Option<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteGroupRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsolePortList {
	previous: Option<String>,
	results: Vec<ConsolePort>,
	next: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactAssignmentList {
	previous: Option<String>,
	results: Vec<ContactAssignment>,
	next: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderAccountList {
	next: Option<String>,
	count: i64,
	results: Vec<ProviderAccount>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedTunnelGroupRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLinkRequest {
	comments: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	auth_psk: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	description: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	ssid: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayTemplateRequest {
	/// Physical label
	label: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVMInterface {
	url: String,
	name: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Prefix {
	display: String,
	created: Option<String>,
	_depth: i64,
	comments: String,
	last_updated: Option<String>,
	description: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	children: i64,
	status: String,
	url: String,
	family: String,
	tags: Vec<NestedTag>,
	id: i64,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	prefix: String,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedServiceTemplateList {
	previous: Option<String>,
	count: i64,
	results: Vec<ServiceTemplate>,
	next: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationRequest {
	slug: String,
	custom_fields: String,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Device {
	description: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	power_port_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	console_port_count: i64,
	power_outlet_count: i64,
	device_bay_count: i64,
	inventory_item_count: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	url: String,
	status: String,
	created: Option<String>,
	front_port_count: i64,
	comments: String,
	console_server_port_count: i64,
	rear_port_count: i64,
	airflow: String,
	name: Option<String>,
	last_updated: Option<String>,
	module_bay_count: i64,
	face: String,
	position: Option<f64>,
	interface_count: i64,
	display: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	vc_position: Option<i64>,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableBookmarkRequest {
	object_id: i64,
	user: i64,
	object_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceRole {
	url: String,
	description: String,
	display: String,
	color: String,
	name: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	id: i64,
	tags: Vec<NestedTag>,
	slug: String,
	virtualmachine_count: i64,
	device_count: i64,
	created: Option<String>,
	custom_fields: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRegionRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedBookmarkList {
	results: Vec<Bookmark>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableEventRuleRequest {
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	action_object_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	enabled: bool,
	action_object_id: Option<i64>,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	description: String,
	content_types: Vec<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANGroupRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleRequest {
	device: i64,
	module_type: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	module_bay: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	serial: String,
	description: String,
	comments: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualChassisRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPN {
	r#type: String,
	identifier: Option<i64>,
	name: String,
	tags: Vec<NestedTag>,
	comments: String,
	last_updated: Option<String>,
	description: String,
	custom_fields: String,
	created: Option<String>,
	slug: String,
	id: i64,
	import_targets: Vec<i64>,
	display: String,
	export_targets: Vec<i64>,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataSource {
	description: String,
	source_url: String,
	id: i64,
	file_count: i64,
	r#type: String,
	name: String,
	url: String,
	enabled: bool,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	display: String,
	status: String,
	created: Option<String>,
	comments: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIKEPolicyRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterGroupRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPortList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<PowerPort>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortTemplateRequest {
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	r#type: String,
	/// Physical label
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsoleServerPortTemplateRequest {
	module_type: Option<i64>,
	description: String,
	/// Physical label
	label: String,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedObjectPermissionList {
	results: Vec<ObjectPermission>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Representation of a prefix which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailablePrefix {
	family: i64,
	prefix: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContact {
	name: String,
	url: String,
	id: i64,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterGroupList {
	results: Vec<ClusterGroup>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceWithConfigContextList {
	next: Option<String>,
	results: Vec<DeviceWithConfigContext>,
	count: i64,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCableList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<Cable>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEProposalRequest {
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
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
	group: i64,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	tags: Vec<NestedTagRequest>,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	comments: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	name: String,
	description: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConfigContextRequest {
	locations: Vec<i64>,
	regions: Vec<i64>,
	roles: Vec<i64>,
	site_groups: Vec<i64>,
	sites: Vec<i64>,
	device_types: Vec<i64>,
	cluster_groups: Vec<i64>,
	weight: i64,
	platforms: Vec<i64>,
	is_active: bool,
	cluster_types: Vec<i64>,
	clusters: Vec<i64>,
	tenants: Vec<i64>,
	name: String,
	tenant_groups: Vec<i64>,
	tags: Vec<String>,
	description: String,
	/// Remote data source
	data_source: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteTargetRequest {
	description: String,
	comments: String,
	custom_fields: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableASNRangeRequest {
	custom_fields: String,
	description: String,
	end: i64,
	rir: i64,
	tenant: Option<i64>,
	name: String,
	start: i64,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceType {
	model: String,
	url: String,
	id: i64,
	slug: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRackRoleRequest {
	slug: String,
	description: String,
	name: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInterfaceRequest {
	mac_address: Option<String>,
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
	rf_channel: String,
	untagged_vlan: Option<i64>,
	wireless_lans: Vec<i64>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
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
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
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
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
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
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
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
	r#type: String,
	custom_fields: String,
	parent: Option<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	wwn: Option<String>,
	bridge: Option<i64>,
	tx_power: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	description: String,
	name: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// Physical label
	label: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	device: i64,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	vrf: Option<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	vdcs: Vec<i64>,
	mtu: Option<i64>,
	tags: Vec<NestedTagRequest>,
	lag: Option<i64>,
	module: Option<i64>,
	speed: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	enabled: bool,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	tagged_vlans: Vec<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecPolicy {
	url: String,
	tags: Vec<NestedTag>,
	description: String,
	name: String,
	proposals: Vec<i64>,
	custom_fields: String,
	pfs_group: String,
	created: Option<String>,
	display: String,
	id: i64,
	last_updated: Option<String>,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecPolicy {
	url: String,
	display: String,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Role {
	slug: String,
	tags: Vec<NestedTag>,
	url: String,
	display: String,
	created: Option<String>,
	name: String,
	last_updated: Option<String>,
	id: i64,
	weight: i64,
	prefix_count: i64,
	vlan_count: i64,
	custom_fields: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTemplate {
	/// Path to remote file (relative to data source root)
	data_path: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	content_types: Vec<String>,
	url: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	id: i64,
	name: String,
	description: String,
	display: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	data_synced: Option<String>,
	last_updated: Option<String>,
	created: Option<String>,
	/// Download file as attachment
	as_attachment: bool,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
	url: String,
	device_count: i64,
	id: i64,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	display: String,
	_depth: i64,
	name: String,
	status: String,
	custom_fields: String,
	rack_count: i64,
	description: String,
	created: Option<String>,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserRequest {
	groups: Vec<i64>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	password: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	first_name: String,
	email: String,
	date_joined: String,
	last_name: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableLocationRequest {
	slug: String,
	tenant: Option<i64>,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	custom_fields: String,
	parent: Option<i64>,
	description: String,
	site: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedClusterTypeRequest {
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	custom_fields: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRack {
	display: String,
	id: i64,
	name: String,
	url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLinkList {
	previous: Option<String>,
	results: Vec<WirelessLink>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCircuitTerminationRequest {
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
	site: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	provider_network: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	circuit: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Aggregate {
	comments: String,
	display: String,
	description: String,
	date_added: Option<String>,
	tags: Vec<NestedTag>,
	family: String,
	url: String,
	custom_fields: String,
	prefix: String,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	custom_fields: String,
	primary_ip4: Option<i64>,
	role: Option<i64>,
	memory: Option<i64>,
	device: Option<i64>,
	tenant: Option<i64>,
	vcpus: Option<f64>,
	disk: Option<i64>,
	comments: String,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	site: Option<i64>,
	cluster: Option<i64>,
	platform: Option<i64>,
	name: String,
	primary_ip6: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedUser {
	id: i64,
	url: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualMachineWithConfigContext {
	tags: Vec<NestedTag>,
	memory: Option<i64>,
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	interface_count: i64,
	virtual_disk_count: i64,
	description: String,
	custom_fields: String,
	disk: Option<i64>,
	vcpus: Option<f64>,
	status: String,
	url: String,
	comments: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedAggregateList {
	results: Vec<Aggregate>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInterfaceTemplateRequest {
	enabled: bool,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	description: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	device_type: Option<i64>,
	mgmt_only: bool,
	module_type: Option<i64>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	bridge: Option<i64>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
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
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
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
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
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
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
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
	r#type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerFeedRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	comments: String,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	name: String,
	custom_fields: String,
	voltage: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	tags: Vec<NestedTagRequest>,
	amperage: i64,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPlatform {
	url: String,
	display: String,
	slug: String,
	id: i64,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortTemplate {
	created: Option<String>,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
	display: String,
	url: String,
	/// Physical label
	label: String,
	description: String,
	r#type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayTemplate {
	/// Physical label
	label: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	display: String,
	last_updated: Option<String>,
	url: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceRoleRequest {
	slug: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	name: String,
	description: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataSource {
	name: String,
	url: String,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceRoleRequest {
	name: String,
	description: String,
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	color: String,
	custom_fields: String,
	slug: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLAN {
	url: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	id: i64,
	comments: String,
	tags: Vec<NestedTag>,
	status: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	prefix_count: i64,
	name: String,
	description: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactRequest {
	custom_fields: String,
	name: String,
	group: Option<i64>,
	email: String,
	phone: String,
	title: String,
	link: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	address: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortRequest {
	description: String,
	/// Number of front ports which may be mapped
	positions: i64,
	custom_fields: String,
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
	/// * `other` - Other
	r#type: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRegionList {
	results: Vec<Region>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTenantGroupList {
	previous: Option<String>,
	results: Vec<TenantGroup>,
	count: i64,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemRoleList {
	next: Option<String>,
	count: i64,
	results: Vec<InventoryItemRole>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLANGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPRangeRequest {
	end_address: String,
	description: String,
	start_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	vrf: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	comments: String,
	tenant: Option<i64>,
	/// The primary function of this range
	role: Option<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RIR {
	name: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	aggregate_count: i64,
	display: String,
	url: String,
	slug: String,
	description: String,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPAddressRequest {
	address: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupAssignment {
	display: String,
	id: i64,
	interface_id: i64,
	interface_type: String,
	priority: i64,
	created: Option<String>,
	last_updated: Option<String>,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNTerminationRequest {
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRoleRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPlatformList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<Platform>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVMInterfaceList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<VMInterface>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDataFileList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<DataFile>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRole {
	display: String,
	slug: String,
	url: String,
	name: String,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsolePortTemplateRequest {
	device_type: Option<i64>,
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
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
	r#type: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedFilterRequest {
	slug: String,
	description: String,
	weight: i64,
	content_types: Vec<String>,
	user: Option<i64>,
	name: String,
	enabled: bool,
	shared: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	name: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	extra_choices: Option<Vec<Vec<String>>>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPRangeRequest {
	start_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	vrf: Option<i64>,
	/// The primary function of this range
	role: Option<i64>,
	description: String,
	comments: String,
	tenant: Option<i64>,
	end_address: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedContactRoleRequest {
	description: String,
	name: String,
	custom_fields: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Token {
	description: String,
	created: String,
	display: String,
	key: String,
	url: String,
	id: i64,
	last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	expires: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVRFRequest {
	description: String,
	name: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
	tenant: Option<i64>,
	comments: String,
	custom_fields: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tenant {
	description: String,
	device_count: i64,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	slug: String,
	created: Option<String>,
	last_updated: Option<String>,
	site_count: i64,
	vlan_count: i64,
	ipaddress_count: i64,
	name: String,
	id: i64,
	url: String,
	rack_count: i64,
	vrf_count: i64,
	display: String,
	cluster_count: i64,
	circuit_count: i64,
	prefix_count: i64,
	virtualmachine_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleBayTemplateList {
	previous: Option<String>,
	results: Vec<ModuleBayTemplate>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWebhookRequest {
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	custom_fields: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	tags: Vec<NestedTagRequest>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	name: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Circuit {
	last_updated: Option<String>,
	display: String,
	id: i64,
	custom_fields: String,
	created: Option<String>,
	comments: String,
	url: String,
	status: String,
	/// Committed rate
	commit_rate: Option<i64>,
	install_date: Option<String>,
	termination_date: Option<String>,
	tags: Vec<NestedTag>,
	/// Unique circuit ID
	cid: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortTemplate {
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	id: i64,
	url: String,
	r#type: String,
	created: Option<String>,
	description: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerOutletRequest {
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	power_port: Option<i64>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	device: i64,
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	tags: Vec<NestedTagRequest>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	custom_fields: String,
	name: String,
	description: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSite {
	display: String,
	url: String,
	/// Full name of the site
	name: String,
	slug: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTenantList {
	results: Vec<Tenant>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderNetwork {
	display: String,
	id: i64,
	name: String,
	url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelGroupList {
	results: Vec<TunnelGroup>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedJobList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<Job>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortTemplate {
	description: String,
	created: Option<String>,
	url: String,
	positions: i64,
	last_updated: Option<String>,
	id: i64,
	/// Physical label
	label: String,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	r#type: String,
	color: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEPolicyRequest {
	proposals: Vec<i64>,
	description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	name: String,
	preshared_key: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRoleRequest {
	name: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	color: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenRequest {
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	expires: Option<String>,
	last_used: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterGroup {
	url: String,
	slug: String,
	id: i64,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceWithConfigContextRequest {
	name: Option<String>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	tags: Vec<NestedTagRequest>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	comments: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	tenant: Option<i64>,
	cluster: Option<i64>,
	device_type: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	primary_ip4: Option<i64>,
	custom_fields: String,
	description: String,
	/// The function this device serves
	role: i64,
	location: Option<i64>,
	platform: Option<i64>,
	rack: Option<i64>,
	position: Option<f64>,
	virtual_chassis: Option<i64>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	config_template: Option<i64>,
	vc_position: Option<i64>,
	oob_ip: Option<i64>,
	primary_ip6: Option<i64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	site: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFrontPortRequest {
	module: Option<i64>,
	description: String,
	custom_fields: String,
	name: String,
	rear_port: i64,
	device: i64,
	/// Physical label
	label: String,
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
	/// * `other` - Other
	r#type: String,
	color: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFHRPGroupAssignmentList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<FHRPGroupAssignment>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceTemplate {
	rf_role: Option<String>,
	description: String,
	poe_mode: Option<String>,
	display: String,
	url: String,
	mgmt_only: bool,
	last_updated: Option<String>,
	r#type: String,
	created: Option<String>,
	id: i64,
	enabled: bool,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	poe_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedManufacturerList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<Manufacturer>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProfile {
	description: String,
	display: String,
	mode: String,
	comments: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: String,
	last_updated: Option<String>,
	id: i64,
	name: String,
	url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleBayList {
	count: i64,
	next: Option<String>,
	results: Vec<ModuleBay>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedTagRequest {
	name: String,
	color: String,
	slug: String,
	description: String,
	object_types: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	comments: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIKEProposalRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	custom_fields: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	/// Diffie-Hellman group ID
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
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
	group: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactGroup {
	created: Option<String>,
	url: String,
	description: String,
	display: String,
	name: String,
	contact_count: i64,
	last_updated: Option<String>,
	_depth: i64,
	slug: String,
	tags: Vec<NestedTag>,
	id: i64,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePlatformRequest {
	config_template: Option<i64>,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderNetworkRequest {
	service_id: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
	provider: i64,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerFeed {
	id: i64,
	comments: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTag>,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	display: String,
	r#type: String,
	connected_endpoints: Vec<String>,
	voltage: i64,
	description: String,
	amperage: i64,
	connected_endpoints_reachable: bool,
	connected_endpoints_type: String,
	supply: String,
	name: String,
	created: Option<String>,
	_occupied: bool,
	custom_fields: String,
	phase: String,
	status: String,
	url: String,
	cable_end: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	link_peers: Vec<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRouteTargetRequest {
	description: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableAggregateRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	custom_fields: String,
	comments: String,
	date_added: Option<String>,
	prefix: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTenantGroupRequest {
	description: String,
	parent: Option<i64>,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleTypeRequest {
	weight: Option<f64>,
	custom_fields: String,
	/// Discrete part number (optional)
	part_number: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	manufacturer: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	model: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackReservationList {
	next: Option<String>,
	results: Vec<RackReservation>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventRule {
	name: String,
	action_object_type: String,
	last_updated: Option<String>,
	action_object_id: Option<i64>,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	url: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	created: Option<String>,
	action_type: String,
	content_types: Vec<String>,
	enabled: bool,
	description: String,
	custom_fields: String,
	display: String,
	id: i64,
	action_object: String,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	tags: Vec<NestedTag>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactList {
	previous: Option<String>,
	count: i64,
	results: Vec<Contact>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceTypeList {
	next: Option<String>,
	results: Vec<DeviceType>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableClusterRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
	site: Option<i64>,
	tenant: Option<i64>,
	name: String,
	r#type: i64,
	group: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDataSourceRequest {
	r#type: String,
	comments: String,
	description: String,
	source_url: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	name: String,
	enabled: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderAccountRequest {
	comments: String,
	account: String,
	name: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecProposalRequest {
	description: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bookmark {
	id: i64,
	display: String,
	object_id: i64,
	url: String,
	object_type: String,
	created: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitList {
	next: Option<String>,
	previous: Option<String>,
	results: Vec<Circuit>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIKEPolicyList {
	results: Vec<IKEPolicy>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleNestedModuleBayRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSiteList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<Site>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecProfileRequest {
	ike_policy: i64,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	ipsec_policy: i64,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegionRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceTypeRequest {
	/// Discrete part number (optional)
	part_number: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	manufacturer: i64,
	model: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	description: String,
	slug: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	weight: Option<f64>,
	comments: String,
	default_platform: Option<i64>,
	rear_image: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	u_height: f64,
	front_image: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletTemplate {
	r#type: Option<String>,
	feed_leg: Option<String>,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
	display: String,
	/// Physical label
	label: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderNetwork {
	description: String,
	last_updated: Option<String>,
	comments: String,
	id: i64,
	display: String,
	tags: Vec<NestedTag>,
	name: String,
	url: String,
	custom_fields: String,
	service_id: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Webhook {
	tags: Vec<NestedTag>,
	name: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	description: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	last_updated: Option<String>,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	created: Option<String>,
	url: String,
	id: i64,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	custom_fields: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTagRequest {
	name: String,
	color: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortTemplateRequest {
	color: String,
	rear_port_position: i64,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// * `other` - Other
	r#type: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableServiceRequest {
	device: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	description: String,
	custom_fields: String,
	comments: String,
	virtual_machine: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	interface_type: String,
	group: i64,
	priority: i64,
	interface_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderNetworkRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInterfaceRequest {
	name: String,
	/// Physical label
	label: String,
	lag: Option<i64>,
	enabled: bool,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	tagged_vlans: Vec<i64>,
	mac_address: Option<String>,
	vdcs: Vec<i64>,
	wireless_lans: Vec<i64>,
	tx_power: Option<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	rf_channel: String,
	untagged_vlan: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	speed: Option<i64>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
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
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
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
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
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
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
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
	r#type: String,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	vrf: Option<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	module: Option<i64>,
	tags: Vec<NestedTagRequest>,
	device: i64,
	parent: Option<i64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	bridge: Option<i64>,
	wwn: Option<String>,
	mtu: Option<i64>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerOutletTemplateRequest {
	device_type: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	module_type: Option<i64>,
	description: String,
	/// Physical label
	label: String,
	power_port: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// * `dc-terminal` - DC Terminal
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPort {
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	id: i64,
	connected_endpoints_reachable: bool,
	custom_fields: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	r#type: Option<String>,
	name: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	cable_end: String,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	display: String,
	_occupied: bool,
	link_peers: Vec<String>,
	created: Option<String>,
	url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Job {
	created: String,
	/// Recurrence interval (in minutes)
	interval: Option<i64>,
	name: String,
	url: String,
	object_type: String,
	id: i64,
	status: String,
	display: String,
	started: Option<String>,
	scheduled: Option<String>,
	job_id: String,
	object_id: Option<i64>,
	completed: Option<String>,
	error: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCircuitTypeRequest {
	slug: String,
	name: String,
	color: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteRequest {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	asns: Vec<i64>,
	/// Physical location of the building
	physical_address: String,
	time_zone: Option<String>,
	/// If different from the physical address
	shipping_address: String,
	comments: String,
	slug: String,
	custom_fields: String,
	/// Full name of the site
	name: String,
	/// Local facility ID or description
	facility: String,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleBay {
	display: String,
	id: i64,
	url: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTokenRequest {
	key: String,
	last_used: Option<String>,
	expires: Option<String>,
	user: i64,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProfileRequest {
	name: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLANRequest {
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	vlan: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	auth_psk: String,
	group: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	comments: String,
	tenant: Option<i64>,
	custom_fields: String,
	ssid: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomLink {
	id: i64,
	enabled: bool,
	content_types: Vec<String>,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	weight: i64,
	/// Force link to open in a new window
	new_window: bool,
	url: String,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	name: String,
	/// The class of the first link in a group will be used for the dropdown button
	/// 
	/// * `outline-dark` - Default
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
	button_class: String,
	/// Jinja2 template code for link URL
	link_url: String,
	/// Jinja2 template code for link text
	link_text: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderRequest {
	slug: String,
	/// Full name of the provider
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualDeviceContextRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	name: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	device: Option<i64>,
	primary_ip4: Option<i64>,
	primary_ip6: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPanelRequest {
	description: String,
	comments: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderAccount {
	id: i64,
	name: String,
	created: Option<String>,
	description: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	account: String,
	url: String,
	display: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelTerminationRequest {
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_id: Option<i64>,
	termination_type: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<Module>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedASNList {
	next: Option<String>,
	results: Vec<ASN>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleType {
	id: i64,
	display: String,
	description: String,
	last_updated: Option<String>,
	url: String,
	created: Option<String>,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	weight_unit: Option<String>,
	comments: String,
	model: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ManufacturerRequest {
	custom_fields: String,
	name: String,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomLinkRequest {
	name: String,
	enabled: bool,
	/// Jinja2 template code for link URL
	link_url: String,
	/// Jinja2 template code for link text
	link_text: String,
	/// Force link to open in a new window
	new_window: bool,
	weight: i64,
	content_types: Vec<String>,
	/// The class of the first link in a group will be used for the dropdown button
	/// 
	/// * `outline-dark` - Default
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
	button_class: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCableRequest {
	label: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomFieldChoiceSetList {
	results: Vec<CustomFieldChoiceSet>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableUserRequest {
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	email: String,
	password: String,
	date_joined: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	first_name: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSavedFilterList {
	previous: Option<String>,
	next: Option<String>,
	results: Vec<SavedFilter>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDisk {
	created: Option<String>,
	tags: Vec<NestedTag>,
	name: String,
	id: i64,
	description: String,
	custom_fields: String,
	last_updated: Option<String>,
	size: i64,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIKEPolicyRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	preshared_key: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	description: String,
	proposals: Vec<i64>,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPrefixList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<Prefix>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceBayTemplateRequest {
	device_type: i64,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVLANRequest {
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// VLAN group (optional)
	group: Option<i64>,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	/// The primary function of this VLAN
	role: Option<i64>,
	custom_fields: String,
	description: String,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCustomFieldChoiceSet {
	name: String,
	choices_count: String,
	id: i64,
	url: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DashboardRequest {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelGroupRequest {
	slug: String,
	name: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRole {
	display: String,
	name: String,
	slug: String,
	description: String,
	last_updated: Option<String>,
	id: i64,
	color: String,
	tags: Vec<NestedTag>,
	rack_count: i64,
	custom_fields: String,
	created: Option<String>,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterTypeRequest {
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
}

/// Used by device component serializers.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentNestedModuleRequest {
	device: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePrefixRequest {
	vlan: Option<i64>,
	comments: String,
	/// The primary function of this prefix
	role: Option<i64>,
	prefix: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	vrf: Option<i64>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	site: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	tenant: Option<i64>,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIKEPolicyRequest {
	proposals: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	description: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	preshared_key: String,
	name: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCircuitTerminationRequest {
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
	custom_fields: String,
	circuit: i64,
	site: Option<i64>,
	tags: Vec<NestedTagRequest>,
	provider_network: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRIR {
	name: String,
	slug: String,
	display: String,
	id: i64,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenant {
	slug: String,
	id: i64,
	name: String,
	url: String,
	display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPortTemplateRequest {
	module_type: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	device_type: Option<i64>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	description: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRearPortRequest {
	device: i64,
	module: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Number of front ports which may be mapped
	positions: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
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
	/// * `other` - Other
	r#type: String,
	/// Physical label
	label: String,
	color: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceWithConfigContext {
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	url: String,
	custom_fields: String,
	last_updated: Option<String>,
	module_bay_count: i64,
	name: Option<String>,
	tags: Vec<NestedTag>,
	inventory_item_count: i64,
	power_outlet_count: i64,
	front_port_count: i64,
	rear_port_count: i64,
	console_port_count: i64,
	device_bay_count: i64,
	id: i64,
	created: Option<String>,
	interface_count: i64,
	airflow: String,
	display: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	status: String,
	face: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	comments: String,
	console_server_port_count: i64,
	power_port_count: i64,
	position: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	description: String,
	vc_position: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableL2VPNTerminationRequest {
	assigned_object_id: i64,
	l2vpn: i64,
	custom_fields: String,
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedSavedFilterRequest {
	name: String,
	description: String,
	user: Option<i64>,
	content_types: Vec<String>,
	weight: i64,
	slug: String,
	enabled: bool,
	shared: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceBayList {
	results: Vec<DeviceBay>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	url: String,
	date_joined: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	email: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	groups: Vec<i64>,
	last_name: String,
	first_name: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualMachineWithConfigContextRequest {
	name: String,
	vcpus: Option<f64>,
	disk: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	memory: Option<i64>,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataFile {
	url: String,
	/// File path relative to the data source's root
	path: String,
	display: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedObjectChangeList {
	results: Vec<ObjectChange>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableSiteRequest {
	/// Local facility ID or description
	facility: String,
	group: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	custom_fields: String,
	asns: Vec<i64>,
	/// If different from the physical address
	shipping_address: String,
	slug: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// Full name of the site
	name: String,
	comments: String,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	region: Option<i64>,
	tenant: Option<i64>,
	time_zone: Option<String>,
	tags: Vec<NestedTagRequest>,
	/// Physical location of the building
	physical_address: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVMInterfaceRequest {
	name: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRearPort {
	id: i64,
	url: String,
	/// Physical label
	label: String,
	description: String,
	display: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContentType {
	id: i64,
	display: String,
	app_label: String,
	model: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterGroupRequest {
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRoleRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRequest {
	serial: String,
	/// Physical label
	label: String,
	/// This item was automatically discovered
	discovered: bool,
	component_id: Option<i64>,
	custom_fields: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactGroupRequest {
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProvider {
	slug: String,
	display: String,
	id: i64,
	url: String,
	/// Full name of the provider
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactGroup {
	id: i64,
	display: String,
	url: String,
	name: String,
	slug: String,
	_depth: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerOutletRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	power_port: Option<i64>,
	device: i64,
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
	/// * `dc-terminal` - DC Terminal
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	/// Physical label
	label: String,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRearPortRequest {
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleBayRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualMachineWithConfigContextRequest {
	disk: Option<i64>,
	device: Option<i64>,
	tags: Vec<NestedTagRequest>,
	primary_ip4: Option<i64>,
	tenant: Option<i64>,
	cluster: Option<i64>,
	role: Option<i64>,
	site: Option<i64>,
	name: String,
	description: String,
	platform: Option<i64>,
	comments: String,
	custom_fields: String,
	primary_ip6: Option<i64>,
	vcpus: Option<f64>,
	memory: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePort {
	connected_endpoints_reachable: bool,
	name: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	speed: Option<String>,
	url: String,
	connected_endpoints_type: String,
	description: String,
	custom_fields: String,
	connected_endpoints: Vec<String>,
	id: i64,
	created: Option<String>,
	_occupied: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	cable_end: String,
	display: String,
	r#type: String,
	link_peers: Vec<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualMachine {
	id: i64,
	url: String,
	name: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactRoleList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<ContactRole>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortRequest {
	/// Physical label
	label: String,
	custom_fields: String,
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	speed: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
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
	r#type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataSourceRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCustomFieldChoiceSetRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutlet {
	/// Physical label
	label: String,
	connected_endpoints: Vec<String>,
	tags: Vec<NestedTag>,
	description: String,
	created: Option<String>,
	name: String,
	connected_endpoints_reachable: bool,
	cable_end: String,
	link_peers: Vec<String>,
	feed_leg: Option<String>,
	connected_endpoints_type: String,
	custom_fields: String,
	url: String,
	display: String,
	_occupied: bool,
	last_updated: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	id: i64,
	r#type: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDeviceContext {
	tags: Vec<NestedTag>,
	display: String,
	status: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	interface_count: i64,
	created: Option<String>,
	custom_fields: String,
	last_updated: Option<String>,
	comments: String,
	url: String,
	description: String,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRackRequest {
	facility_id: Option<String>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	tenant: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// Functional role
	role: Option<i64>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	site: i64,
	serial: String,
	location: Option<i64>,
	/// Height in rack units
	u_height: i64,
	weight: Option<f64>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	comments: String,
	/// Starting unit for rack
	starting_unit: i64,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRearPortRequest {
	color: String,
	module: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
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
	/// * `other` - Other
	r#type: String,
	/// Number of front ports which may be mapped
	positions: i64,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	device: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceType {
	inventory_item_template_count: i64,
	console_server_port_template_count: i64,
	last_updated: Option<String>,
	model: String,
	id: i64,
	created: Option<String>,
	slug: String,
	u_height: f64,
	/// Discrete part number (optional)
	part_number: String,
	url: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	device_count: i64,
	power_outlet_template_count: i64,
	weight_unit: Option<String>,
	custom_fields: String,
	subdevice_role: Option<String>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	interface_template_count: i64,
	weight: Option<f64>,
	power_port_template_count: i64,
	front_image: String,
	rear_image: String,
	comments: String,
	console_port_template_count: i64,
	description: String,
	airflow: Option<String>,
	tags: Vec<NestedTag>,
	device_bay_template_count: i64,
	rear_port_template_count: i64,
	module_bay_template_count: i64,
	front_port_template_count: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelGroupRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPN {
	identifier: Option<i64>,
	url: String,
	display: String,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	r#type: String,
	id: i64,
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRole {
	url: String,
	id: i64,
	display: String,
	name: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRearPortTemplateList {
	previous: Option<String>,
	results: Vec<RearPortTemplate>,
	count: i64,
	next: Option<String>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConfigTemplateRequest {
	description: String,
	name: String,
	/// Jinja2 template code.
	template_code: String,
	/// Remote data source
	data_source: Option<i64>,
	data_file: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPanelRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletTemplateRequest {
	/// Physical label
	label: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleBayRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	installed_module: i64,
	device: i64,
	description: String,
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsolePortTemplateList {
	results: Vec<ConsolePortTemplate>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualDeviceContextRequest {
	name: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	tags: Vec<NestedTagRequest>,
	primary_ip4: Option<i64>,
	description: String,
	custom_fields: String,
	comments: String,
	primary_ip6: Option<i64>,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	device: Option<i64>,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANGroupRequest {
	scope_type: Option<String>,
	scope_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	description: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTemplateRequest {
	description: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	name: String,
	content_types: Vec<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Download file as attachment
	as_attachment: bool,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPlatformRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactAssignment {
	tags: Vec<NestedTag>,
	url: String,
	created: Option<String>,
	last_updated: Option<String>,
	custom_fields: String,
	content_type: String,
	display: String,
	id: i64,
	object_id: i64,
	object: String,
	priority: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPortTemplateRequest {
	/// Physical label
	label: String,
	description: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	device_type: Option<i64>,
	module_type: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableSiteRequest {
	/// If different from the physical address
	shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	slug: String,
	/// Physical location of the building
	physical_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	tags: Vec<NestedTagRequest>,
	time_zone: Option<String>,
	asns: Vec<i64>,
	description: String,
	/// Local facility ID or description
	facility: String,
	/// Full name of the site
	name: String,
	tenant: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	region: Option<i64>,
	group: Option<i64>,
	custom_fields: String,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLANGroupRequest {
	custom_fields: String,
	slug: String,
	description: String,
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCircuitRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	provider: i64,
	install_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	/// Unique circuit ID
	cid: String,
	description: String,
	r#type: i64,
	provider_account: Option<i64>,
	termination_date: Option<String>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCableTerminationRequest {
	termination_id: i64,
	cable: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	termination_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRearPortTemplateRequest {
	module_type: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	color: String,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// * `other` - Other
	r#type: String,
	positions: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTunnelRequest {
	group: Option<i64>,
	tunnel_id: Option<i64>,
	name: String,
	tenant: Option<i64>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	comments: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	ipsec_profile: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableUserRequest {
	email: String,
	date_joined: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	first_name: String,
	last_name: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	password: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPAddressRequest {
	assigned_object_type: Option<String>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	comments: String,
	custom_fields: String,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	description: String,
	tenant: Option<i64>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	vrf: Option<i64>,
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
	role: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_id: Option<i64>,
	address: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTokenList {
	count: i64,
	previous: Option<String>,
	results: Vec<Token>,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AggregateRequest {
	date_added: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	prefix: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedLocation {
	url: String,
	display: String,
	name: String,
	id: i64,
	slug: String,
	_depth: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNRequest {
	name: String,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	r#type: String,
	identifier: Option<i64>,
	slug: String,
}

/// Used by device component serializers.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentNestedModule {
	device: i64,
	display: String,
	id: i64,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANRequest {
	name: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConfigContextList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<ConfigContext>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceWithConfigContextRequest {
	platform: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	oob_ip: Option<i64>,
	site: i64,
	position: Option<f64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	primary_ip6: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	vc_position: Option<i64>,
	comments: String,
	location: Option<i64>,
	rack: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	device_type: i64,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	tenant: Option<i64>,
	cluster: Option<i64>,
	custom_fields: String,
	name: Option<String>,
	/// The function this device serves
	role: i64,
	primary_ip4: Option<i64>,
	config_template: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	virtual_chassis: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualDiskRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
	virtual_machine: i64,
	size: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Region {
	display: String,
	tags: Vec<NestedTag>,
	description: String,
	created: Option<String>,
	site_count: i64,
	last_updated: Option<String>,
	name: String,
	custom_fields: String,
	slug: String,
	id: i64,
	url: String,
	_depth: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemTemplate {
	/// Physical label
	label: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	parent: Option<i64>,
	url: String,
	component_id: Option<i64>,
	_depth: i64,
	/// Manufacturer-assigned part identifier
	part_id: String,
	display: String,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	component_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRouteTargetList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<RouteTarget>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tunnel {
	description: String,
	status: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	tunnel_id: Option<i64>,
	last_updated: Option<String>,
	display: String,
	comments: String,
	id: i64,
	url: String,
	encapsulation: String,
	name: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactAssignmentRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	content_type: String,
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualMachineRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSiteGroupList {
	count: i64,
	next: Option<String>,
	results: Vec<SiteGroup>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPort {
	custom_fields: String,
	created: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	speed: Option<String>,
	cable_end: String,
	link_peers: Vec<String>,
	connected_endpoints_type: String,
	description: String,
	url: String,
	_occupied: bool,
	id: i64,
	display: String,
	/// Physical label
	label: String,
	r#type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints: Vec<String>,
	connected_endpoints_reachable: bool,
	name: String,
	tags: Vec<NestedTag>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigTemplate {
	last_updated: Option<String>,
	name: String,
	url: String,
	/// Jinja2 template code.
	template_code: String,
	tags: Vec<NestedTag>,
	description: String,
	/// Path to remote file (relative to data source root)
	data_path: String,
	created: Option<String>,
	display: String,
	id: i64,
	data_synced: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedGroupRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableObjectPermissionRequest {
	object_types: Vec<String>,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	enabled: bool,
	name: String,
	users: Vec<i64>,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFrontPortTemplateList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<FrontPortTemplate>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPanelRequest {
	site: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	location: Option<i64>,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRackReservationRequest {
	user: i64,
	description: String,
	custom_fields: String,
	tenant: Option<i64>,
	units: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	rack: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortTemplate {
	/// Physical label
	label: String,
	id: i64,
	url: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	r#type: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	comments: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	ipaddresses: Vec<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedFHRPGroup {
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	group_id: i64,
	url: String,
	display: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInterfaceTemplateList {
	results: Vec<InterfaceTemplate>,
	next: Option<String>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsolePortRequest {
	device: i64,
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
	r#type: String,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	module: Option<i64>,
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
	speed: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderRequest {
	/// Full name of the provider
	name: String,
	asns: Vec<i64>,
	accounts: Vec<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableEventRuleRequest {
	content_types: Vec<String>,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	enabled: bool,
	action_object_type: String,
	action_object_id: Option<i64>,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	description: String,
	custom_fields: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	name: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rack {
	facility_id: Option<String>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	description: String,
	width: String,
	id: i64,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	created: Option<String>,
	r#type: Option<String>,
	url: String,
	/// Height in rack units
	u_height: i64,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	last_updated: Option<String>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	/// Starting unit for rack
	starting_unit: i64,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	status: String,
	weight: Option<f64>,
	comments: String,
	custom_fields: String,
	weight_unit: Option<String>,
	name: String,
	outer_unit: Option<String>,
	serial: String,
	powerfeed_count: i64,
	display: String,
	device_count: i64,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableL2VPNTerminationRequest {
	custom_fields: String,
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	l2vpn: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleTypeRequest {
	model: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVLANRequest {
	/// The primary function of this VLAN
	role: Option<i64>,
	name: String,
	tenant: Option<i64>,
	description: String,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// VLAN group (optional)
	group: Option<i64>,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceBayRequest {
	name: String,
	/// Physical label
	label: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	installed_device: Option<i64>,
	custom_fields: String,
	device: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayNestedModule {
	serial: String,
	url: String,
	id: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDevice {
	id: i64,
	name: Option<String>,
	url: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayRequest {
	name: String,
	description: String,
	/// Identifier to reference when renaming installed components
	position: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModule {
	id: i64,
	display: String,
	url: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericObjectRequest {
	object_type: String,
	object_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedUserList {
	results: Vec<User>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCircuitRequest {
	/// Committed rate
	commit_rate: Option<i64>,
	provider: i64,
	r#type: i64,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	description: String,
	custom_fields: String,
	termination_date: Option<String>,
	/// Unique circuit ID
	cid: String,
	tenant: Option<i64>,
	provider_account: Option<i64>,
	install_date: Option<String>,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContentTypeList {
	count: i64,
	next: Option<String>,
	results: Vec<ContentType>,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInterfaceList {
	count: i64,
	results: Vec<Interface>,
	previous: Option<String>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenProvision {
	url: String,
	key: String,
	last_used: String,
	description: String,
	created: String,
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	display: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceTypeRequest {
	model: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRole {
	name: String,
	slug: String,
	id: i64,
	url: String,
	display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectPermission {
	groups: Vec<i64>,
	id: i64,
	enabled: bool,
	users: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	name: String,
	url: String,
	object_types: Vec<String>,
	display: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLANList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<WirelessLAN>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsoleServerPortRequest {
	tags: Vec<NestedTagRequest>,
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
	r#type: String,
	device: i64,
	custom_fields: String,
	/// Physical label
	label: String,
	module: Option<i64>,
	name: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	speed: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInventoryItemRequest {
	custom_fields: String,
	manufacturer: Option<i64>,
	serial: String,
	component_id: Option<i64>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// Physical label
	label: String,
	component_type: Option<String>,
	name: String,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	role: Option<i64>,
	device: i64,
	description: String,
	/// This item was automatically discovered
	discovered: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemTemplateList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<InventoryItemTemplate>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayRequest {
	name: String,
	/// Physical label
	label: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRequest {
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cable {
	display: String,
	tags: Vec<NestedTag>,
	a_terminations: Vec<GenericObject>,
	last_updated: Option<String>,
	custom_fields: String,
	url: String,
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
	/// * `power` - Power
	r#type: String,
	length_unit: Option<String>,
	id: i64,
	status: String,
	length: Option<f64>,
	comments: String,
	color: String,
	description: String,
	b_terminations: Vec<GenericObject>,
	created: Option<String>,
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderNetworkRequest {
	comments: String,
	custom_fields: String,
	service_id: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteTarget {
	description: String,
	url: String,
	custom_fields: String,
	created: Option<String>,
	comments: String,
	last_updated: Option<String>,
	id: i64,
	display: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePrefixRequest {
	tags: Vec<NestedTagRequest>,
	/// The primary function of this prefix
	role: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	vlan: Option<i64>,
	site: Option<i64>,
	comments: String,
	vrf: Option<i64>,
	description: String,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	custom_fields: String,
	prefix: String,
	tenant: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderAccountRequest {
	name: String,
	account: String,
	provider: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableTerminationRequest {
	cable: i64,
	termination_id: i64,
	termination_type: String,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsolePortRequest {
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
	speed: Option<i64>,
	module: Option<i64>,
	description: String,
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
	r#type: String,
	device: i64,
	name: String,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedInventoryItemRoleRequest {
	custom_fields: String,
	description: String,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteGroup {
	name: String,
	url: String,
	id: i64,
	display: String,
	slug: String,
	_depth: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackRoleList {
	count: i64,
	results: Vec<RackRole>,
	previous: Option<String>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleBayTemplateRequest {
	/// Identifier to reference when renaming installed components
	position: String,
	device_type: i64,
	description: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRackRequest {
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	location: Option<i64>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	site: i64,
	tags: Vec<NestedTagRequest>,
	/// Functional role
	role: Option<i64>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	/// Starting unit for rack
	starting_unit: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	weight: Option<f64>,
	tenant: Option<i64>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Height in rack units
	u_height: i64,
	serial: String,
	comments: String,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	facility_id: Option<String>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	description: String,
	custom_fields: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	name: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cluster {
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: String,
	device_count: i64,
	last_updated: Option<String>,
	status: String,
	name: String,
	display: String,
	comments: String,
	description: String,
	url: String,
	virtualmachine_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBay {
	url: String,
	display: String,
	/// Identifier to reference when renaming installed components
	position: String,
	custom_fields: String,
	created: Option<String>,
	description: String,
	last_updated: Option<String>,
	id: i64,
	name: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRangeRequest {
	slug: String,
	end: i64,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	start: i64,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTag {
	display: String,
	color: String,
	slug: String,
	url: String,
	name: String,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldChoiceSet {
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	display: String,
	choices_count: String,
	id: i64,
	created: Option<String>,
	name: String,
	url: String,
	base_choices: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceTemplate {
	created: Option<String>,
	tags: Vec<NestedTag>,
	ports: Vec<i64>,
	custom_fields: String,
	id: i64,
	comments: String,
	last_updated: Option<String>,
	protocol: String,
	description: String,
	name: String,
	display: String,
	url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigContextRequest {
	weight: i64,
	description: String,
	name: String,
	site_groups: Vec<i64>,
	platforms: Vec<i64>,
	tenants: Vec<i64>,
	cluster_groups: Vec<i64>,
	sites: Vec<i64>,
	regions: Vec<i64>,
	clusters: Vec<i64>,
	device_types: Vec<i64>,
	locations: Vec<i64>,
	is_active: bool,
	roles: Vec<i64>,
	cluster_types: Vec<i64>,
	tenant_groups: Vec<i64>,
	tags: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVRFRequest {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	custom_fields: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	tenant: Option<i64>,
	export_targets: Vec<i64>,
	name: String,
	import_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableServiceTemplateRequest {
	name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	ports: Vec<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMInterfaceRequest {
	custom_fields: String,
	mac_address: Option<String>,
	tagged_vlans: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	enabled: bool,
	name: String,
	mtu: Option<i64>,
}

/// Representation of an ASN which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableASN {
	asn: i64,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Interface {
	mac_address: Option<String>,
	description: String,
	mtu: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	rf_channel: String,
	name: String,
	r#type: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints_reachable: bool,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	url: String,
	enabled: bool,
	wwn: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	_occupied: bool,
	connected_endpoints: Vec<String>,
	poe_mode: String,
	wireless_lans: Vec<i64>,
	connected_endpoints_type: String,
	count_ipaddresses: i64,
	tx_power: Option<i64>,
	tagged_vlans: Vec<i64>,
	id: i64,
	cable_end: String,
	display: String,
	count_fhrp_groups: i64,
	last_updated: Option<String>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	speed: Option<i64>,
	rf_role: String,
	duplex: Option<String>,
	mode: String,
	vdcs: Vec<i64>,
	link_peers: Vec<String>,
	poe_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayNestedModuleRequest {
	serial: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsoleServerPortTemplateList {
	previous: Option<String>,
	count: i64,
	results: Vec<ConsoleServerPortTemplate>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIKEProposalList {
	count: i64,
	results: Vec<IKEProposal>,
	next: Option<String>,
	previous: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteGroupRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelTermination {
	termination_type: String,
	last_updated: Option<String>,
	id: i64,
	role: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	url: String,
	termination_id: Option<i64>,
	display: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualDiskRequest {
	size: i64,
	name: String,
	description: String,
	virtual_machine: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceBayTemplateList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<DeviceBayTemplate>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JournalEntryRequest {
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
	created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	comments: String,
	custom_fields: String,
	assigned_object_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableExportTemplateRequest {
	/// Download file as attachment
	as_attachment: bool,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	name: String,
	description: String,
	/// Remote data source
	data_source: Option<i64>,
	content_types: Vec<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecPolicyRequest {
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	proposals: Vec<i64>,
	custom_fields: String,
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
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
	pfs_group: i64,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitRequest {
	/// Unique circuit ID
	cid: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	termination_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	install_date: Option<String>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	custom_fields: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCustomFieldRequest {
	/// Internal field name
	name: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
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
	r#type: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	choice_set: Option<i64>,
	content_types: Vec<String>,
	object_type: String,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	description: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceBayTemplateRequest {
	description: String,
	device_type: i64,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	bridge: Option<i64>,
	/// Physical label
	label: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	module_type: Option<i64>,
	mgmt_only: bool,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	device_type: Option<i64>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
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
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
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
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
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
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
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
	r#type: String,
	enabled: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceBayRequest {
	/// Physical label
	label: String,
	description: String,
	installed_device: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	device: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVRFRequest {
	name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleRequest {
	device: i64,
	description: String,
	serial: String,
	custom_fields: String,
	module_bay: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	module_type: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedClusterGroupRequest {
	custom_fields: String,
	description: String,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterGroup {
	custom_fields: String,
	tags: Vec<NestedTag>,
	display: String,
	id: i64,
	name: String,
	slug: String,
	description: String,
	url: String,
	created: Option<String>,
	last_updated: Option<String>,
	cluster_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleNestedModuleBay {
	url: String,
	id: i64,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedManufacturerRequest {
	slug: String,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRequest {
	description: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	color: String,
	custom_fields: String,
	/// Physical label
	label: String,
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
	/// * `other` - Other
	r#type: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableL2VPNRequest {
	import_targets: Vec<i64>,
	tenant: Option<i64>,
	name: String,
	custom_fields: String,
	identifier: Option<i64>,
	comments: String,
	slug: String,
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	r#type: String,
	export_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNTermination {
	display: String,
	id: i64,
	url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldRequest {
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	object_type: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
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
	r#type: String,
	description: String,
	/// Internal field name
	name: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	content_types: Vec<String>,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleTypeRequest {
	comments: String,
	/// Discrete part number (optional)
	part_number: String,
	custom_fields: String,
	weight: Option<f64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	model: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantGroup {
	slug: String,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	id: i64,
	tenant_count: i64,
	url: String,
	_depth: i64,
	name: String,
	description: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBay {
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	display: String,
	custom_fields: String,
	id: i64,
	created: Option<String>,
	name: String,
	description: String,
	/// Physical label
	label: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VRF {
	created: Option<String>,
	display: String,
	tags: Vec<NestedTag>,
	import_targets: Vec<i64>,
	comments: String,
	export_targets: Vec<i64>,
	url: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
	custom_fields: String,
	description: String,
	last_updated: Option<String>,
	ipaddress_count: i64,
	prefix_count: i64,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableClusterRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	r#type: i64,
	group: Option<i64>,
	tenant: Option<i64>,
	site: Option<i64>,
	comments: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupAssignmentRequest {
	priority: i64,
	interface_type: String,
	interface_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRIRRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterTypeRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNRequest {
	/// * `vpws` - VPWS
	/// * `vpls` - VPLS
	/// * `vxlan` - VXLAN
	/// * `vxlan-evpn` - VXLAN-EVPN
	/// * `mpls-evpn` - MPLS EVPN
	/// * `pbb-evpn` - PBB EVPN
	/// * `epl` - EPL
	/// * `evpl` - EVPL
	/// * `ep-lan` - Ethernet Private LAN
	/// * `evp-lan` - Ethernet Virtual Private LAN
	/// * `ep-tree` - Ethernet Private Tree
	/// * `evp-tree` - Ethernet Virtual Private Tree
	r#type: String,
	export_targets: Vec<i64>,
	name: String,
	comments: String,
	custom_fields: String,
	slug: String,
	import_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	identifier: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCustomFieldChoiceSetRequest {
	description: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	name: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecProfile {
	id: i64,
	url: String,
	name: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectChange {
	action: String,
	request_id: String,
	display: String,
	id: i64,
	time: String,
	user_name: String,
	changed_object_type: String,
	url: String,
	changed_object_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIKEProposalRequest {
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	/// Diffie-Hellman group ID
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
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
	group: i64,
	custom_fields: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableServiceRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ports: Vec<i64>,
	comments: String,
	virtual_machine: Option<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	device: Option<i64>,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableTermination {
	termination_id: i64,
	cable: i64,
	id: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	last_updated: Option<String>,
	created: Option<String>,
	url: String,
	termination_type: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPortTemplateList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<PowerPortTemplate>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Module {
	serial: String,
	status: String,
	created: Option<String>,
	url: String,
	custom_fields: String,
	description: String,
	comments: String,
	last_updated: Option<String>,
	display: String,
	tags: Vec<NestedTag>,
	id: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPRange {
	comments: String,
	size: i64,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	start_address: String,
	custom_fields: String,
	end_address: String,
	url: String,
	display: String,
	status: String,
	description: String,
	family: String,
	last_updated: Option<String>,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedL2VPNTerminationList {
	count: i64,
	previous: Option<String>,
	results: Vec<L2VPNTermination>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedImageAttachmentRequest {
	object_id: i64,
	name: String,
	image: String,
	image_width: i64,
	image_height: i64,
	content_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerOutletTemplateRequest {
	device_type: Option<i64>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `hdot-cx` - HDOT Cx
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20a` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32a` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	/// Physical label
	label: String,
	power_port: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedFilter {
	url: String,
	created: Option<String>,
	id: i64,
	description: String,
	display: String,
	name: String,
	user: Option<i64>,
	enabled: bool,
	shared: bool,
	content_types: Vec<String>,
	last_updated: Option<String>,
	weight: i64,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceTemplateRequest {
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: Option<String>,
	description: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: Option<String>,
	/// * `virtual` - Virtual
	/// * `bridge` - Bridge
	/// * `lag` - Link Aggregation Group (LAG)
	/// * `100base-fx` - 100BASE-FX (10/100ME FIBER)
	/// * `100base-lfx` - 100BASE-LFX (10/100ME FIBER)
	/// * `100base-tx` - 100BASE-TX (10/100ME)
	/// * `100base-t1` - 100BASE-T1 (10/100ME Single Pair)
	/// * `1000base-t` - 1000BASE-T (1GE)
	/// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
	/// * `5gbase-t` - 5GBASE-T (5GE)
	/// * `10gbase-t` - 10GBASE-T (10GE)
	/// * `10gbase-cx4` - 10GBASE-CX4 (10GE)
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
	/// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
	/// * `other-wireless` - Other (Wireless)
	/// * `gsm` - GSM
	/// * `cdma` - CDMA
	/// * `lte` - LTE
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
	/// * `64gfc-qsfpp` - QSFP+ (64GFC)
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
	/// * `gpon` - GPON (2.5 Gbps / 1.25 Gps)
	/// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
	/// * `xgs-pon` - XGS-PON (10 Gbps)
	/// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
	/// * `epon` - EPON (1 Gbps)
	/// * `10g-epon` - 10G-EPON (10 Gbps)
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
	r#type: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: Option<String>,
	enabled: bool,
	mgmt_only: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRole {
	slug: String,
	display: String,
	id: i64,
	url: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GroupRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterTypeList {
	results: Vec<ClusterType>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleRequest {
	tags: Vec<NestedTagRequest>,
	weight: i64,
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelRequest {
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	custom_fields: String,
	comments: String,
	tunnel_id: Option<i64>,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterType {
	custom_fields: String,
	created: Option<String>,
	url: String,
	slug: String,
	last_updated: Option<String>,
	cluster_count: i64,
	description: String,
	id: i64,
	tags: Vec<NestedTag>,
	display: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConfigContextRequest {
	site_groups: Vec<i64>,
	name: String,
	sites: Vec<i64>,
	device_types: Vec<i64>,
	description: String,
	cluster_types: Vec<i64>,
	cluster_groups: Vec<i64>,
	tenant_groups: Vec<i64>,
	locations: Vec<i64>,
	roles: Vec<i64>,
	platforms: Vec<i64>,
	is_active: bool,
	weight: i64,
	tenants: Vec<i64>,
	clusters: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	regions: Vec<i64>,
	tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVLANList {
	previous: Option<String>,
	count: i64,
	results: Vec<VLAN>,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPAddressRequest {
	tags: Vec<NestedTagRequest>,
	assigned_object_id: Option<i64>,
	assigned_object_type: Option<String>,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	custom_fields: String,
	address: String,
	tenant: Option<i64>,
	comments: String,
	vrf: Option<i64>,
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
	role: String,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedExportTemplateList {
	results: Vec<ExportTemplate>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecProposalList {
	next: Option<String>,
	previous: Option<String>,
	results: Vec<IPSecProposal>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceTypeRequest {
	weight: Option<f64>,
	tags: Vec<NestedTagRequest>,
	model: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	front_image: String,
	/// Discrete part number (optional)
	part_number: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	default_platform: Option<i64>,
	manufacturer: i64,
	slug: String,
	u_height: f64,
	description: String,
	comments: String,
	custom_fields: String,
	rear_image: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldChoiceSetRequest {
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	name: String,
	extra_choices: Option<Vec<Vec<String>>>,
	description: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRoleRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedServiceList {
	previous: Option<String>,
	next: Option<String>,
	results: Vec<Service>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitType {
	id: i64,
	name: String,
	slug: String,
	url: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPAddressList {
	count: i64,
	results: Vec<IPAddress>,
	next: Option<String>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRearPortTemplateRequest {
	module_type: Option<i64>,
	device_type: Option<i64>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// * `other` - Other
	r#type: String,
	positions: i64,
	description: String,
	color: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTermination {
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	link_peers: Vec<String>,
	custom_fields: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	created: Option<String>,
	url: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	description: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	id: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	_occupied: bool,
	display: String,
	cable_end: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderRequest {
	/// Full name of the provider
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	asns: Vec<i64>,
	accounts: Vec<i64>,
	comments: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericObject {
	object_id: i64,
	object_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASN {
	comments: String,
	tags: Vec<NestedTag>,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	id: i64,
	display: String,
	custom_fields: String,
	url: String,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	site_count: i64,
	provider_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderRequest {
	asns: Vec<i64>,
	/// Full name of the provider
	name: String,
	accounts: Vec<i64>,
	slug: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemList {
	previous: Option<String>,
	results: Vec<InventoryItem>,
	next: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JournalEntry {
	created: Option<String>,
	custom_fields: String,
	assigned_object_type: String,
	tags: Vec<NestedTag>,
	kind: String,
	url: String,
	assigned_object_id: i64,
	display: String,
	created_by: Option<i64>,
	last_updated: Option<String>,
	comments: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceTemplate {
	display: String,
	id: i64,
	url: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedImageAttachmentList {
	next: Option<String>,
	results: Vec<ImageAttachment>,
	count: i64,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageAttachment {
	created: Option<String>,
	id: i64,
	display: String,
	image_width: i64,
	object_id: i64,
	url: String,
	name: String,
	content_type: String,
	image_height: i64,
	last_updated: Option<String>,
	image: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableSiteGroupRequest {
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	parent: Option<i64>,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterList {
	count: i64,
	results: Vec<Cluster>,
	previous: Option<String>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRearPortList {
	results: Vec<RearPort>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomField {
	display: String,
	description: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	data_type: String,
	ui_visible: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Internal field name
	name: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	ui_editable: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	r#type: String,
	content_types: Vec<String>,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	created: Option<String>,
	last_updated: Option<String>,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	id: i64,
	url: String,
	filter_logic: String,
	object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleBayTemplateRequest {
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	device_type: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortTemplateRequest {
	/// Physical label
	label: String,
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
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTypeRequest {
	custom_fields: String,
	color: String,
	description: String,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTenantRequest {
	name: String,
	group: Option<i64>,
	slug: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNTerminationRequest {
	assigned_object_id: i64,
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableServiceTemplateRequest {
	description: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableRequest {
	b_terminations: Vec<GenericObjectRequest>,
	comments: String,
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
	/// * `power` - Power
	r#type: String,
	color: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: Option<String>,
	custom_fields: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	label: String,
	length: Option<f64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	a_terminations: Vec<GenericObjectRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitType {
	description: String,
	name: String,
	id: i64,
	slug: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	last_updated: Option<String>,
	circuit_count: i64,
	display: String,
	url: String,
	color: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortRequest {
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	speed: Option<i64>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	r#type: String,
	custom_fields: String,
	/// Physical label
	label: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemTemplateRequest {
	description: String,
	component_type: Option<String>,
	component_id: Option<i64>,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualChassis {
	member_count: i64,
	created: Option<String>,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	id: i64,
	name: String,
	url: String,
	domain: String,
	display: String,
	description: String,
	custom_fields: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPAddress {
	display: String,
	id: i64,
	address: String,
	url: String,
	family: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFrontPortRequest {
	module: Option<i64>,
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
	/// * `other` - Other
	r#type: String,
	/// Physical label
	label: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	custom_fields: String,
	device: i64,
	name: String,
	rear_port: i64,
	tags: Vec<NestedTagRequest>,
	color: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVLANGroupList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<VLANGroup>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Site {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	asns: Vec<i64>,
	last_updated: Option<String>,
	/// Full name of the site
	name: String,
	device_count: i64,
	virtualmachine_count: i64,
	vlan_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	prefix_count: i64,
	created: Option<String>,
	rack_count: i64,
	comments: String,
	circuit_count: i64,
	url: String,
	status: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	time_zone: Option<String>,
	description: String,
	slug: String,
	display: String,
	/// Local facility ID or description
	facility: String,
	/// Physical location of the building
	physical_address: String,
	id: i64,
	/// If different from the physical address
	shipping_address: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedJournalEntryList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<JournalEntry>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCustomLinkRequest {
	name: String,
	content_types: Vec<String>,
	/// Jinja2 template code for link text
	link_text: String,
	enabled: bool,
	/// Jinja2 template code for link URL
	link_url: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	/// Force link to open in a new window
	new_window: bool,
	weight: i64,
	/// The class of the first link in a group will be used for the dropdown button
	/// 
	/// * `outline-dark` - Default
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
	button_class: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	description: String,
	auth_psk: String,
	comments: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	custom_fields: String,
	ssid: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsoleServerPortRequest {
	description: String,
	name: String,
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
	speed: Option<i64>,
	custom_fields: String,
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
	r#type: String,
	module: Option<i64>,
	device: i64,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTunnelTerminationRequest {
	custom_fields: String,
	termination_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	tunnel: i64,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_type: String,
	outside_ip: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedLocationRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroup {
	url: String,
	ip_addresses: Vec<NestedIPAddress>,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	description: String,
	custom_fields: String,
	group_id: i64,
	comments: String,
	name: String,
	auth_key: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<Rack>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecPolicyRequest {
	description: String,
	/// Diffie-Hellman group for Perfect Forward Secrecy
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
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
	pfs_group: Option<i64>,
	proposals: Vec<i64>,
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleTypeRequest {
	description: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	/// Discrete part number (optional)
	part_number: String,
	model: String,
	manufacturer: i64,
	weight: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualMachineWithConfigContextList {
	results: Vec<VirtualMachineWithConfigContext>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Provider {
	created: Option<String>,
	id: i64,
	display: String,
	url: String,
	slug: String,
	accounts: Vec<i64>,
	asns: Vec<i64>,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	circuit_count: i64,
	description: String,
	comments: String,
	/// Full name of the provider
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackReservationRequest {
	description: String,
	custom_fields: String,
	comments: String,
	units: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDeviceContextRequest {
	description: String,
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	custom_fields: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecProfileList {
	results: Vec<IPSecProfile>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagRequest {
	object_types: Vec<String>,
	slug: String,
	description: String,
	color: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerOutletTemplateList {
	results: Vec<PowerOutletTemplate>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortRequest {
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: Option<String>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Physical label
	label: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConfigTemplateList {
	next: Option<String>,
	results: Vec<ConfigTemplate>,
	previous: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANGroup {
	display: String,
	name: String,
	id: i64,
	slug: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEProposal {
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	authentication_method: String,
	group: String,
	encryption_algorithm: String,
	custom_fields: String,
	description: String,
	comments: String,
	authentication_algorithm: String,
	url: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLinkRequest {
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	ssid: String,
	custom_fields: String,
	interface_a: i64,
	auth_psk: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	interface_b: i64,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRoleRequest {
	slug: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectPermissionRequest {
	enabled: bool,
	description: String,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	object_types: Vec<String>,
	name: String,
	users: Vec<i64>,
	groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
	name: String,
	last_updated: Option<String>,
	comments: String,
	protocol: String,
	ipaddresses: Vec<i64>,
	display: String,
	url: String,
	tags: Vec<NestedTag>,
	description: String,
	custom_fields: String,
	created: Option<String>,
	id: i64,
	ports: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dashboard {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataSourceRequest {
	source_url: String,
	comments: String,
	enabled: bool,
	description: String,
	name: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnel {
	name: String,
	url: String,
	display: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVRFList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<VRF>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEPolicy {
	proposals: Vec<i64>,
	display: String,
	mode: String,
	custom_fields: String,
	description: String,
	last_updated: Option<String>,
	name: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	preshared_key: String,
	version: String,
	comments: String,
	id: i64,
	url: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTenantGroupRequest {
	name: String,
	custom_fields: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPRangeRequest {
	start_address: String,
	end_address: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPortRequest {
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
	name: String,
	tags: Vec<NestedTagRequest>,
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
	/// * `dc-terminal` - DC Terminal
	/// * `saf-d-grid` - Saf-D-Grid
	/// * `neutrik-powercon-20` - Neutrik powerCON (20A)
	/// * `neutrik-powercon-32` - Neutrik powerCON (32A)
	/// * `neutrik-powercon-true1` - Neutrik powerCON TRUE1
	/// * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP
	/// * `ubiquiti-smartpower` - Ubiquiti SmartPower
	/// * `hardwired` - Hardwired
	/// * `other` - Other
	r#type: String,
	module: Option<i64>,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInventoryItemRequest {
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	component_type: Option<String>,
	component_id: Option<i64>,
	custom_fields: String,
	name: String,
	/// Physical label
	label: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	parent: Option<i64>,
	serial: String,
	role: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// This item was automatically discovered
	discovered: bool,
	device: i64,
	manufacturer: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDataSourceList {
	next: Option<String>,
	results: Vec<DataSource>,
	previous: Option<String>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortTemplate {
	/// Physical label
	label: String,
	color: String,
	display: String,
	description: String,
	url: String,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	rear_port_position: i64,
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Representation of an IP address which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableIP {
	address: String,
	family: i64,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPanel {
	last_updated: Option<String>,
	id: i64,
	name: String,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	display: String,
	url: String,
	powerfeed_count: i64,
	custom_fields: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortTemplateRequest {
	color: String,
	positions: i64,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// * `other` - Other
	r#type: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRegionRequest {
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	parent: Option<i64>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigTemplateRequest {
	/// Jinja2 template code.
	template_code: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRIRList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<RIR>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenProvisionRequest {
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	password: String,
	username: String,
	description: String,
	expires: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitTypeList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<CircuitType>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleRequest {
	custom_fields: String,
	serial: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	cid: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactGroupRequest {
	parent: Option<i64>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedManufacturer {
	url: String,
	slug: String,
	display: String,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecProposalRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	name: String,
	description: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelTerminationList {
	count: i64,
	results: Vec<TunnelTermination>,
	next: Option<String>,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRoleList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWebhookList {
	previous: Option<String>,
	results: Vec<Webhook>,
	count: i64,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTokenRequest {
	key: String,
	last_used: Option<String>,
	user: i64,
	expires: Option<String>,
	description: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<Provider>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConfigTemplateRequest {
	name: String,
	/// Jinja2 template code.
	template_code: String,
	description: String,
	/// Remote data source
	data_source: Option<i64>,
	data_file: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BookmarkRequest {
	object_type: String,
	object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVRF {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	display: String,
	url: String,
	name: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantGroup {
	id: i64,
	name: String,
	display: String,
	url: String,
	_depth: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRequest {
	custom_fields: String,
	facility_id: Option<String>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	weight: Option<f64>,
	comments: String,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Starting unit for rack
	starting_unit: i64,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	name: String,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	serial: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Height in rack units
	u_height: i64,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: Option<String>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedFHRPGroupRequest {
	group_id: i64,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFHRPGroupList {
	next: Option<String>,
	count: i64,
	results: Vec<FHRPGroup>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableASNRangeRequest {
	start: i64,
	custom_fields: String,
	description: String,
	tenant: Option<i64>,
	name: String,
	rir: i64,
	tags: Vec<NestedTagRequest>,
	end: i64,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageAttachmentRequest {
	image_width: i64,
	object_id: i64,
	content_type: String,
	name: String,
	image_height: i64,
	image: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortTemplate {
	id: i64,
	url: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelGroup {
	name: String,
	url: String,
	display: String,
	slug: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecPolicyRequest {
	description: String,
	comments: String,
	custom_fields: String,
	/// Diffie-Hellman group for Perfect Forward Secrecy
	/// 
	/// * `1` - Group 1
	/// * `2` - Group 2
	/// * `5` - Group 5
	/// * `14` - Group 14
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
	pfs_group: Option<i64>,
	tags: Vec<NestedTagRequest>,
	proposals: Vec<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPort {
	color: String,
	name: String,
	id: i64,
	r#type: String,
	display: String,
	url: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	cable_end: String,
	description: String,
	link_peers: Vec<String>,
	_occupied: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedConfigTemplate {
	url: String,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderAccountRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	provider: i64,
	name: String,
	account: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPort {
	cable: Option<i64>,
	_occupied: bool,
	display: String,
	name: String,
	url: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedLocationList {
	results: Vec<Location>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableJournalEntryRequest {
	assigned_object_id: i64,
	created_by: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	comments: String,
	assigned_object_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualChassisRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	name: String,
	master: Option<i64>,
	domain: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedGroupList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<Group>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterface {
	name: String,
	display: String,
	cable: Option<i64>,
	_occupied: bool,
	id: i64,
	url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerFeedList {
	results: Vec<PowerFeed>,
	next: Option<String>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRackReservationRequest {
	rack: i64,
	custom_fields: String,
	comments: String,
	user: i64,
	tenant: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	units: Vec<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableSiteGroupRequest {
	custom_fields: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPort {
	r#type: String,
	description: String,
	link_peers: Vec<String>,
	custom_fields: String,
	/// Physical label
	label: String,
	name: String,
	tags: Vec<NestedTag>,
	id: i64,
	_occupied: bool,
	/// Number of front ports which may be mapped
	positions: i64,
	last_updated: Option<String>,
	color: String,
	created: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	cable_end: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	display: String,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortRequest {
	name: String,
	cable: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInventoryItemTemplateRequest {
	parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: i64,
	component_id: Option<i64>,
	/// Physical label
	label: String,
	role: Option<i64>,
	manufacturer: Option<i64>,
	component_type: Option<String>,
	description: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVMInterfaceRequest {
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	mac_address: Option<String>,
	vrf: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	mtu: Option<i64>,
	bridge: Option<i64>,
	virtual_machine: i64,
	parent: Option<i64>,
	untagged_vlan: Option<i64>,
	tagged_vlans: Vec<i64>,
	enabled: bool,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualChassisRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
	domain: String,
	master: Option<i64>,
	comments: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCableTerminationList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<CableTermination>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDiskRequest {
	name: String,
	size: i64,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDataSourceRequest {
	comments: String,
	r#type: String,
	source_url: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	enabled: bool,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceRequest {
	name: String,
	cable: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCable {
	label: String,
	id: i64,
	url: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTunnelRequest {
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	tenant: Option<i64>,
	description: String,
	group: Option<i64>,
	comments: String,
	name: String,
	ipsec_profile: Option<i64>,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	tunnel_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualChassisList {
	results: Vec<VirtualChassis>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRegion {
	_depth: i64,
	url: String,
	slug: String,
	id: i64,
	display: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualChassis {
	id: i64,
	display: String,
	name: String,
	url: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANGroup {
	id: i64,
	wirelesslan_count: i64,
	_depth: i64,
	url: String,
	created: Option<String>,
	slug: String,
	description: String,
	last_updated: Option<String>,
	display: String,
	name: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerOutletList {
	count: i64,
	results: Vec<PowerOutlet>,
	previous: Option<String>,
	next: Option<String>,
}


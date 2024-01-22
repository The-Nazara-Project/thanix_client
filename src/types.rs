use serde_qs;
use serde_json;
use reqwest::Url;
use crate::util::ThanixClient;
/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLANRequest {
	description: String,
	group: Option<i64>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	tenant: Option<i64>,
	vlan: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	ssid: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	auth_psk: String,
	custom_fields: serde_json::value::Value,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
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
	/// * `power` - Power
	r#type: String,
	tags: Vec<NestedTagRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: Option<String>,
	custom_fields: serde_json::value::Value,
	comments: String,
	a_terminations: Vec<GenericObjectRequest>,
	b_terminations: Vec<GenericObjectRequest>,
	label: String,
	color: String,
	length: Option<f64>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleTypeRequest {
	/// Discrete part number (optional)
	part_number: String,
	model: String,
	description: String,
	comments: String,
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	manufacturer: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRearPort {
	url: String,
	display: String,
	name: String,
	/// Physical label
	label: String,
	description: String,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Token {
	url: String,
	description: String,
	created: String,
	expires: Option<String>,
	id: i64,
	display: String,
	last_used: Option<String>,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserRequest {
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	last_name: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	groups: Vec<i64>,
	email: String,
	first_name: String,
	date_joined: String,
	password: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLinkList {
	results: Vec<WirelessLink>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTenantGroupList {
	results: Vec<TenantGroup>,
	next: Option<String>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPAddressRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	assigned_object_id: Option<i64>,
	address: String,
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	role: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	custom_fields: serde_json::value::Value,
	assigned_object_type: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterList {
	next: Option<String>,
	results: Vec<Cluster>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	name: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLANGroupRequest {
	parent: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: serde_json::value::Value,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitCircuitTerminationRequest {
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ManufacturerRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	custom_fields: serde_json::value::Value,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPRangeRequest {
	/// The primary function of this range
	role: Option<i64>,
	description: String,
	end_address: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	vrf: Option<i64>,
	start_address: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedServiceList {
	results: Vec<Service>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackReservation {
	display: String,
	comments: String,
	created: Option<String>,
	id: i64,
	last_updated: Option<String>,
	units: Vec<i64>,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	description: String,
	url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTokenRequest {
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	key: String,
	last_used: Option<String>,
	user: i64,
	expires: Option<String>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEPolicy {
	created: Option<String>,
	proposals: Vec<i64>,
	url: String,
	version: serde_json::value::Value,
	comments: String,
	name: String,
	description: String,
	preshared_key: String,
	display: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	id: i64,
	mode: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebhookRequest {
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	tags: Vec<NestedTagRequest>,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	name: String,
	custom_fields: serde_json::value::Value,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	description: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRoleRequest {
	custom_fields: serde_json::value::Value,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
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
	name: String,
	/// Physical label
	label: String,
	custom_fields: serde_json::value::Value,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceBayRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	name: String,
	/// Physical label
	label: String,
	device: i64,
	description: String,
	installed_device: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderRequest {
	/// Full name of the provider
	name: String,
	slug: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Region {
	last_updated: Option<String>,
	description: String,
	display: String,
	custom_fields: serde_json::value::Value,
	site_count: i64,
	id: i64,
	tags: Vec<NestedTag>,
	_depth: i64,
	name: String,
	slug: String,
	created: Option<String>,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVLANRequest {
	tenant: Option<i64>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// VLAN group (optional)
	group: Option<i64>,
	name: String,
	comments: String,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	custom_fields: serde_json::value::Value,
	description: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	/// The primary function of this VLAN
	role: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTemplate {
	data_synced: Option<String>,
	name: String,
	/// Download file as attachment
	as_attachment: bool,
	description: String,
	content_types: Vec<String>,
	display: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	id: i64,
	url: String,
	created: Option<String>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Path to remote file (relative to data source root)
	data_path: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPAddressRequest {
	custom_fields: serde_json::value::Value,
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
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	vrf: Option<i64>,
	address: String,
	description: String,
	assigned_object_type: Option<String>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigTemplateRequest {
	/// Jinja2 template code.
	template_code: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
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
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualMachineRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderRequest {
	description: String,
	slug: String,
	accounts: Vec<i64>,
	comments: String,
	custom_fields: serde_json::value::Value,
	asns: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	/// Full name of the provider
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenProvision {
	key: String,
	display: String,
	last_used: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	id: i64,
	description: String,
	url: String,
	created: String,
	expires: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceRole {
	color: String,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	device_count: i64,
	description: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	custom_fields: serde_json::value::Value,
	slug: String,
	display: String,
	url: String,
	virtualmachine_count: i64,
	id: i64,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleType {
	created: Option<String>,
	last_updated: Option<String>,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	tags: Vec<NestedTag>,
	weight_unit: Option<serde_json::value::Value>,
	model: String,
	url: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	description: String,
	id: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRIR {
	name: String,
	display: String,
	id: i64,
	url: String,
	slug: String,
}

/// Used by device component serializers.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentNestedModuleRequest {
	device: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFHRPGroupList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<FHRPGroup>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectPermissionRequest {
	name: String,
	users: Vec<i64>,
	description: String,
	enabled: bool,
	object_types: Vec<String>,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
	display: String,
	url: String,
	last_updated: Option<String>,
	description: String,
	rack_count: i64,
	id: i64,
	slug: String,
	device_count: i64,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTag>,
	created: Option<String>,
	_depth: i64,
	status: serde_json::value::Value,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContentType {
	display: String,
	app_label: String,
	url: String,
	id: i64,
	model: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Device {
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	rear_port_count: i64,
	vc_position: Option<i64>,
	console_server_port_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	name: Option<String>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	tags: Vec<NestedTag>,
	display: String,
	face: serde_json::value::Value,
	custom_fields: serde_json::value::Value,
	interface_count: i64,
	comments: String,
	status: serde_json::value::Value,
	console_port_count: i64,
	power_outlet_count: i64,
	device_bay_count: i64,
	url: String,
	power_port_count: i64,
	front_port_count: i64,
	airflow: serde_json::value::Value,
	inventory_item_count: i64,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	position: Option<f64>,
	last_updated: Option<String>,
	id: i64,
	module_bay_count: i64,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLink {
	url: String,
	id: i64,
	display: String,
	ssid: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCircuitTerminationRequest {
	/// ID of the local cross-connect
	xconnect_id: String,
	provider_network: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: serde_json::value::Value,
	description: String,
	tags: Vec<NestedTagRequest>,
	circuit: i64,
	site: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableJournalEntryRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	comments: String,
	created_by: Option<i64>,
	assigned_object_id: i64,
	assigned_object_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerFeedRequest {
	custom_fields: serde_json::value::Value,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	comments: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	tags: Vec<NestedTagRequest>,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	voltage: i64,
	description: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	name: String,
	amperage: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceTemplate {
	r#type: serde_json::value::Value,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
	/// Physical label
	label: String,
	url: String,
	display: String,
	rf_role: Option<serde_json::value::Value>,
	poe_type: Option<serde_json::value::Value>,
	created: Option<String>,
	poe_mode: Option<serde_json::value::Value>,
	mgmt_only: bool,
	last_updated: Option<String>,
	enabled: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceTemplate {
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	comments: String,
	ports: Vec<i64>,
	id: i64,
	custom_fields: serde_json::value::Value,
	display: String,
	protocol: serde_json::value::Value,
	url: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tunnel {
	description: String,
	encapsulation: serde_json::value::Value,
	last_updated: Option<String>,
	comments: String,
	id: i64,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTag>,
	name: String,
	created: Option<String>,
	url: String,
	tunnel_id: Option<i64>,
	display: String,
	status: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsoleServerPortTemplateRequest {
	/// Physical label
	label: String,
	module_type: Option<i64>,
	description: String,
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

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactRequest {
	email: String,
	description: String,
	name: String,
	comments: String,
	title: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	address: String,
	phone: String,
	group: Option<i64>,
	link: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroup {
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	display: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	id: i64,
	url: String,
	comments: String,
	name: String,
	created: Option<String>,
	auth_key: String,
	group_id: i64,
	tags: Vec<NestedTag>,
	ip_addresses: Vec<NestedIPAddress>,
	description: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortRequest {
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
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleBayRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedL2VPNList {
	count: i64,
	next: Option<String>,
	results: Vec<L2VPN>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIKEPolicyRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	preshared_key: String,
	name: String,
	comments: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	proposals: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	serial: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	description: String,
	comments: String,
	module_type: i64,
	tags: Vec<NestedTagRequest>,
	module_bay: i64,
	custom_fields: serde_json::value::Value,
	device: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	extra_choices: Option<Vec<Vec<String>>>,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	name: String,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsoleServerPortRequest {
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
	custom_fields: serde_json::value::Value,
	name: String,
	device: i64,
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
	module: Option<i64>,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualMachineWithConfigContextRequest {
	comments: String,
	vcpus: Option<f64>,
	tags: Vec<NestedTagRequest>,
	primary_ip4: Option<i64>,
	primary_ip6: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	site: Option<i64>,
	role: Option<i64>,
	platform: Option<i64>,
	memory: Option<i64>,
	tenant: Option<i64>,
	device: Option<i64>,
	disk: Option<i64>,
	name: String,
	custom_fields: serde_json::value::Value,
	description: String,
	cluster: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortTemplate {
	url: String,
	/// Physical label
	label: String,
	display: String,
	r#type: serde_json::value::Value,
	color: String,
	rear_port_position: i64,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Representation of an IP address which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableIP {
	description: String,
	family: i64,
	address: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSite {
	/// Full name of the site
	name: String,
	slug: String,
	display: String,
	url: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceType {
	url: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	rear_image: String,
	tags: Vec<NestedTag>,
	front_port_template_count: i64,
	interface_template_count: i64,
	device_count: i64,
	airflow: Option<serde_json::value::Value>,
	power_port_template_count: i64,
	display: String,
	id: i64,
	weight_unit: Option<serde_json::value::Value>,
	inventory_item_template_count: i64,
	/// Discrete part number (optional)
	part_number: String,
	u_height: f64,
	slug: String,
	weight: Option<f64>,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	front_image: String,
	description: String,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
	console_port_template_count: i64,
	console_server_port_template_count: i64,
	power_outlet_template_count: i64,
	device_bay_template_count: i64,
	module_bay_template_count: i64,
	comments: String,
	model: String,
	rear_port_template_count: i64,
	last_updated: Option<String>,
	subdevice_role: Option<serde_json::value::Value>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactGroupRequest {
	slug: String,
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNTerminationRequest {
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	assigned_object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleBayTemplateRequest {
	description: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleBay {
	url: String,
	display: String,
	name: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCustomFieldChoiceSetRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceBayTemplateList {
	results: Vec<DeviceBayTemplate>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANGroup {
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	display: String,
	scope_type: Option<String>,
	created: Option<String>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	custom_fields: serde_json::value::Value,
	scope_id: Option<i64>,
	url: String,
	tags: Vec<NestedTag>,
	vlan_count: i64,
	utilization: String,
	name: String,
	slug: String,
	id: i64,
	last_updated: Option<String>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPort {
	/// Physical label
	label: String,
	_occupied: bool,
	/// Treat as if a cable is connected
	mark_connected: bool,
	link_peers: Vec<String>,
	connected_endpoints_type: String,
	tags: Vec<NestedTag>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	connected_endpoints: Vec<String>,
	display: String,
	url: String,
	name: String,
	r#type: Option<serde_json::value::Value>,
	cable_end: String,
	custom_fields: serde_json::value::Value,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	connected_endpoints_reachable: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortTemplateRequest {
	positions: i64,
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
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	color: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageAttachment {
	name: String,
	display: String,
	id: i64,
	content_type: String,
	created: Option<String>,
	last_updated: Option<String>,
	object_id: i64,
	url: String,
	image: String,
	image_height: i64,
	image_width: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEProposal {
	display: String,
	name: String,
	authentication_algorithm: serde_json::value::Value,
	group: serde_json::value::Value,
	url: String,
	description: String,
	encryption_algorithm: serde_json::value::Value,
	tags: Vec<NestedTag>,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	authentication_method: serde_json::value::Value,
	last_updated: Option<String>,
	comments: String,
	id: i64,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
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
pub struct NestedTunnel {
	url: String,
	display: String,
	id: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedFHRPGroupRequest {
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	group_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEPolicyRequest {
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	description: String,
	preshared_key: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: serde_json::value::Value,
	proposals: Vec<i64>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceWithConfigContextRequest {
	vc_position: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	name: Option<String>,
	position: Option<f64>,
	description: String,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	custom_fields: serde_json::value::Value,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	comments: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<InventoryItem>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigTemplate {
	/// Jinja2 template code.
	template_code: String,
	id: i64,
	data_synced: Option<String>,
	created: Option<String>,
	url: String,
	description: String,
	/// Path to remote file (relative to data source root)
	data_path: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	display: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConfigTemplateList {
	next: Option<String>,
	results: Vec<ConfigTemplate>,
	previous: Option<String>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceTemplateRequest {
	/// Physical label
	label: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: Option<String>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: Option<String>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: Option<String>,
	mgmt_only: bool,
	enabled: bool,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedObjectChangeList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<ObjectChange>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFrontPortRequest {
	color: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: serde_json::value::Value,
	name: String,
	/// Mapped position on corresponding rear port
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
	device: i64,
	module: Option<i64>,
	rear_port: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletTemplateRequest {
	/// Physical label
	label: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RIR {
	created: Option<String>,
	display: String,
	id: i64,
	url: String,
	tags: Vec<NestedTag>,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	description: String,
	aggregate_count: i64,
	name: String,
	last_updated: Option<String>,
	custom_fields: serde_json::value::Value,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerOutletRequest {
	custom_fields: serde_json::value::Value,
	description: String,
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
	device: i64,
	module: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	power_port: Option<i64>,
	name: String,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRearPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Number of front ports which may be mapped
	positions: i64,
	tags: Vec<NestedTagRequest>,
	color: String,
	custom_fields: serde_json::value::Value,
	device: i64,
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
	name: String,
	module: Option<i64>,
	description: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRoleRequest {
	slug: String,
	description: String,
	custom_fields: serde_json::value::Value,
	name: String,
	tags: Vec<NestedTagRequest>,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortRequest {
	name: String,
	custom_fields: serde_json::value::Value,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	speed: Option<i64>,
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
	/// Physical label
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortTemplate {
	/// Physical label
	label: String,
	r#type: serde_json::value::Value,
	url: String,
	id: i64,
	description: String,
	created: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRange {
	last_updated: Option<String>,
	end: i64,
	asn_count: i64,
	tags: Vec<NestedTag>,
	description: String,
	name: String,
	url: String,
	custom_fields: serde_json::value::Value,
	slug: String,
	start: i64,
	display: String,
	created: Option<String>,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContentTypeList {
	results: Vec<ContentType>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecPolicyRequest {
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
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	proposals: Vec<i64>,
	custom_fields: serde_json::value::Value,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableSiteRequest {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Physical location of the building
	physical_address: String,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// If different from the physical address
	shipping_address: String,
	tenant: Option<i64>,
	region: Option<i64>,
	comments: String,
	/// Local facility ID or description
	facility: String,
	/// Full name of the site
	name: String,
	time_zone: Option<String>,
	custom_fields: serde_json::value::Value,
	asns: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	slug: String,
	group: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackReservationRequest {
	description: String,
	custom_fields: serde_json::value::Value,
	comments: String,
	units: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLAN {
	created: Option<String>,
	status: serde_json::value::Value,
	custom_fields: serde_json::value::Value,
	prefix_count: i64,
	comments: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	url: String,
	last_updated: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	description: String,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMInterface {
	mode: serde_json::value::Value,
	tags: Vec<NestedTag>,
	count_fhrp_groups: i64,
	created: Option<String>,
	display: String,
	mtu: Option<i64>,
	count_ipaddresses: i64,
	last_updated: Option<String>,
	description: String,
	enabled: bool,
	id: i64,
	url: String,
	name: String,
	custom_fields: serde_json::value::Value,
	tagged_vlans: Vec<i64>,
	mac_address: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLAN {
	url: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRequest {
	parent: Option<i64>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	component_type: Option<String>,
	/// Physical label
	label: String,
	component_id: Option<i64>,
	custom_fields: serde_json::value::Value,
	serial: String,
	/// This item was automatically discovered
	discovered: bool,
	name: String,
	description: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	rear_port_position: i64,
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
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecProposalList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<IPSecProposal>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VRF {
	id: i64,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	import_targets: Vec<i64>,
	description: String,
	display: String,
	prefix_count: i64,
	comments: String,
	ipaddress_count: i64,
	name: String,
	export_targets: Vec<i64>,
	created: Option<String>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecProfileRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedManufacturerRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableServiceTemplateRequest {
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	ports: Vec<i64>,
	description: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRIRList {
	previous: Option<String>,
	results: Vec<RIR>,
	next: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedAggregateList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<Aggregate>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitTerminationList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<CircuitTermination>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomLinkList {
	results: Vec<CustomLink>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayRequest {
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	/// Physical label
	label: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterGroup {
	url: String,
	id: i64,
	display: String,
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterTypeRequest {
	name: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemTemplateList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<InventoryItemTemplate>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelTerminationList {
	count: i64,
	next: Option<String>,
	results: Vec<TunnelTermination>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANRequest {
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleBayRequest {
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Identifier to reference when renaming installed components
	position: String,
	device: i64,
	installed_module: i64,
	custom_fields: serde_json::value::Value,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutlet {
	cable_end: String,
	display: String,
	url: String,
	connected_endpoints_reachable: bool,
	feed_leg: Option<serde_json::value::Value>,
	last_updated: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	custom_fields: serde_json::value::Value,
	id: i64,
	description: String,
	connected_endpoints: Vec<String>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	connected_endpoints_type: String,
	/// Physical label
	label: String,
	r#type: Option<serde_json::value::Value>,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	_occupied: bool,
	link_peers: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTunnelTerminationRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	tunnel: i64,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_id: Option<i64>,
	termination_type: String,
	outside_ip: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInventoryItemRequest {
	/// Physical label
	label: String,
	custom_fields: serde_json::value::Value,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	device: i64,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	role: Option<i64>,
	component_id: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	serial: String,
	manufacturer: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
	description: String,
	parent: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRequest {
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteGroup {
	created: Option<String>,
	url: String,
	name: String,
	last_updated: Option<String>,
	slug: String,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	description: String,
	id: i64,
	site_count: i64,
	_depth: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCableRequest {
	b_terminations: Vec<GenericObjectRequest>,
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
	tenant: Option<i64>,
	description: String,
	length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	a_terminations: Vec<GenericObjectRequest>,
	color: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleType {
	model: String,
	url: String,
	display: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANRequest {
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIKEProposalRequest {
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
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
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
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsoleServerPortRequest {
	device: i64,
	custom_fields: serde_json::value::Value,
	/// Physical label
	label: String,
	description: String,
	module: Option<i64>,
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
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFHRPGroupAssignmentList {
	count: i64,
	previous: Option<String>,
	results: Vec<FHRPGroupAssignment>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceWithConfigContextList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<DeviceWithConfigContext>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackList {
	next: Option<String>,
	results: Vec<Rack>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteTargetRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	custom_fields: serde_json::value::Value,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceRoleRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	name: String,
	description: String,
	custom_fields: serde_json::value::Value,
	color: String,
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
pub struct NestedProviderNetworkRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNRequest {
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
	name: String,
	slug: String,
	identifier: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRegionRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASN {
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	created: Option<String>,
	site_count: i64,
	description: String,
	url: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	display: String,
	id: i64,
	comments: String,
	tags: Vec<NestedTag>,
	provider_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitRequest {
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	install_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	termination_date: Option<String>,
	description: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	/// Unique circuit ID
	cid: String,
	/// Committed rate
	commit_rate: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomLink {
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	/// Force link to open in a new window
	new_window: bool,
	display: String,
	weight: i64,
	url: String,
	content_types: Vec<String>,
	last_updated: Option<String>,
	id: i64,
	name: String,
	/// Jinja2 template code for link text
	link_text: String,
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
	created: Option<String>,
	enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFrontPortList {
	count: i64,
	next: Option<String>,
	results: Vec<FrontPort>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayNestedModuleRequest {
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConfigContextRequest {
	tags: Vec<String>,
	clusters: Vec<i64>,
	cluster_groups: Vec<i64>,
	tenants: Vec<i64>,
	name: String,
	device_types: Vec<i64>,
	weight: i64,
	cluster_types: Vec<i64>,
	site_groups: Vec<i64>,
	locations: Vec<i64>,
	roles: Vec<i64>,
	platforms: Vec<i64>,
	description: String,
	sites: Vec<i64>,
	regions: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	is_active: bool,
	tenant_groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	priority: i64,
	interface_id: i64,
	interface_type: String,
	group: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedUserList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<User>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTag {
	id: i64,
	display: String,
	url: String,
	name: String,
	slug: String,
	color: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSiteList {
	count: i64,
	previous: Option<String>,
	results: Vec<Site>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedGroupRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterGroupList {
	next: Option<String>,
	previous: Option<String>,
	results: Vec<ClusterGroup>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedManufacturer {
	name: String,
	id: i64,
	url: String,
	slug: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedTunnelGroupRequest {
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceTemplateRequest {
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	name: String,
	ports: Vec<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRackReservationRequest {
	units: Vec<i64>,
	rack: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	tenant: Option<i64>,
	user: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecPolicyList {
	next: Option<String>,
	results: Vec<IPSecPolicy>,
	previous: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedObjectPermissionList {
	previous: Option<String>,
	next: Option<String>,
	results: Vec<ObjectPermission>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVMInterfaceRequest {
	name: String,
	bridge: Option<i64>,
	parent: Option<i64>,
	untagged_vlan: Option<i64>,
	vrf: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	virtual_machine: i64,
	tagged_vlans: Vec<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	enabled: bool,
	mtu: Option<i64>,
	mac_address: Option<String>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableExportTemplateRequest {
	name: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	content_types: Vec<String>,
	description: String,
	/// Download file as attachment
	as_attachment: bool,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Remote data source
	data_source: Option<i64>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerOutletTemplateRequest {
	description: String,
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
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	module_type: Option<i64>,
	device_type: Option<i64>,
	/// Physical label
	label: String,
	power_port: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactRoleList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<ContactRole>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: serde_json::value::Value,
	description: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedEventRuleList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<EventRule>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRearPortList {
	count: i64,
	results: Vec<RearPort>,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableTerminationRequest {
	cable: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	termination_type: String,
	termination_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupAssignmentRequest {
	priority: i64,
	interface_type: String,
	interface_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItem {
	component_id: Option<i64>,
	created: Option<String>,
	serial: String,
	name: String,
	_depth: i64,
	/// This item was automatically discovered
	discovered: bool,
	/// Manufacturer-assigned part identifier
	part_id: String,
	parent: Option<i64>,
	tags: Vec<NestedTag>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	custom_fields: serde_json::value::Value,
	description: String,
	last_updated: Option<String>,
	id: i64,
	display: String,
	component_type: Option<String>,
	url: String,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualChassisRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModule {
	display: String,
	id: i64,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANGroupRequest {
	name: String,
	slug: String,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	description: String,
	scope_id: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	tags: Vec<NestedTagRequest>,
	scope_type: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRearPortTemplateRequest {
	positions: i64,
	description: String,
	color: String,
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: Option<i64>,
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
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<Contact>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	custom_fields: serde_json::value::Value,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	serial: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRoleRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	custom_fields: serde_json::value::Value,
	color: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPrefixList {
	next: Option<String>,
	previous: Option<String>,
	results: Vec<Prefix>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedSavedFilterRequest {
	slug: String,
	user: Option<i64>,
	enabled: bool,
	content_types: Vec<String>,
	name: String,
	shared: bool,
	weight: i64,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualDeviceContextRequest {
	device: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	primary_ip4: Option<i64>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	comments: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	tenant: Option<i64>,
	custom_fields: serde_json::value::Value,
	primary_ip6: Option<i64>,
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

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableL2VPNRequest {
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
	slug: String,
	import_targets: Vec<i64>,
	custom_fields: serde_json::value::Value,
	description: String,
	name: String,
	export_targets: Vec<i64>,
	comments: String,
	identifier: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterType {
	id: i64,
	slug: String,
	display: String,
	url: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterface {
	name: String,
	url: String,
	cable: Option<i64>,
	_occupied: bool,
	id: i64,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelList {
	count: i64,
	results: Vec<Tunnel>,
	next: Option<String>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceRequest {
	cable: Option<i64>,
	name: String,
}

/// Used by device component serializers.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentNestedModule {
	id: i64,
	url: String,
	display: String,
	device: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
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
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactAssignment {
	url: String,
	custom_fields: serde_json::value::Value,
	display: String,
	content_type: String,
	object: serde_json::value::Value,
	priority: serde_json::value::Value,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	object_id: i64,
	id: i64,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableL2VPNTerminationRequest {
	l2vpn: i64,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	assigned_object_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTermination {
	tags: Vec<NestedTag>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	display: String,
	url: String,
	link_peers: Vec<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	_occupied: bool,
	cable_end: String,
	last_updated: Option<String>,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
	id: i64,
	/// Physical circuit speed
	port_speed: Option<i64>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableObjectPermissionRequest {
	object_types: Vec<String>,
	groups: Vec<i64>,
	users: Vec<i64>,
	name: String,
	description: String,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	enabled: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableEventRuleRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	enabled: bool,
	content_types: Vec<String>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	action_object_id: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_type: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsoleServerPortList {
	results: Vec<ConsoleServerPort>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPRangeRequest {
	start_address: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	end_address: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPort {
	name: String,
	url: String,
	cable: Option<i64>,
	id: i64,
	display: String,
	_occupied: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitTypeList {
	next: Option<String>,
	results: Vec<CircuitType>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVLANRequest {
	tenant: Option<i64>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	/// The primary function of this VLAN
	role: Option<i64>,
	name: String,
	/// VLAN group (optional)
	group: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConfigContextRequest {
	is_active: bool,
	device_types: Vec<i64>,
	locations: Vec<i64>,
	tenant_groups: Vec<i64>,
	tenants: Vec<i64>,
	tags: Vec<String>,
	cluster_groups: Vec<i64>,
	weight: i64,
	site_groups: Vec<i64>,
	description: String,
	name: String,
	cluster_types: Vec<i64>,
	clusters: Vec<i64>,
	roles: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	sites: Vec<i64>,
	regions: Vec<i64>,
	platforms: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLANGroupRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRegionList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<Region>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRIRRequest {
	name: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIKEPolicyList {
	count: i64,
	previous: Option<String>,
	results: Vec<IKEPolicy>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPlatformList {
	count: i64,
	next: Option<String>,
	results: Vec<Platform>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	display: String,
	last_name: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	date_joined: String,
	url: String,
	id: i64,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	groups: Vec<i64>,
	first_name: String,
	email: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecProposalRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	description: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	comments: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderNetworkRequest {
	comments: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
	provider: i64,
	service_id: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPanel {
	display: String,
	created: Option<String>,
	id: i64,
	name: String,
	url: String,
	comments: String,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	powerfeed_count: i64,
	last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceRoleList {
	next: Option<String>,
	count: i64,
	results: Vec<DeviceRole>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTunnelTerminationRequest {
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_type: String,
	termination_id: Option<i64>,
	custom_fields: serde_json::value::Value,
	outside_ip: Option<i64>,
	tags: Vec<NestedTagRequest>,
	tunnel: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterTypeList {
	next: Option<String>,
	results: Vec<ClusterType>,
	previous: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFrontPortTemplateList {
	next: Option<String>,
	results: Vec<FrontPortTemplate>,
	previous: Option<String>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableExportTemplateRequest {
	/// Extension to append to the rendered filename
	file_extension: String,
	name: String,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
	content_types: Vec<String>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Download file as attachment
	as_attachment: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayNestedModule {
	id: i64,
	url: String,
	display: String,
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	module_type: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
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
	/// Physical label
	label: String,
	device_type: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomLinkRequest {
	/// Force link to open in a new window
	new_window: bool,
	weight: i64,
	name: String,
	enabled: bool,
	/// Jinja2 template code for link URL
	link_url: String,
	/// Jinja2 template code for link text
	link_text: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
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
	content_types: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleTypeRequest {
	custom_fields: serde_json::value::Value,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	description: String,
	/// Discrete part number (optional)
	part_number: String,
	comments: String,
	model: String,
	tags: Vec<NestedTagRequest>,
	weight: Option<f64>,
	manufacturer: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortTemplate {
	last_updated: Option<String>,
	description: String,
	created: Option<String>,
	id: i64,
	url: String,
	display: String,
	/// Physical label
	label: String,
	r#type: serde_json::value::Value,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualChassisRequest {
	custom_fields: serde_json::value::Value,
	description: String,
	tags: Vec<NestedTagRequest>,
	master: Option<i64>,
	comments: String,
	name: String,
	domain: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitTypeRequest {
	name: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsolePortTemplateList {
	results: Vec<ConsolePortTemplate>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCircuitRequest {
	/// Unique circuit ID
	cid: String,
	provider: i64,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	/// Committed rate
	commit_rate: Option<i64>,
	install_date: Option<String>,
	tenant: Option<i64>,
	r#type: i64,
	comments: String,
	custom_fields: serde_json::value::Value,
	provider_account: Option<i64>,
	termination_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableUserRequest {
	password: String,
	email: String,
	first_name: String,
	last_name: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	date_joined: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedTagRequest {
	object_types: Vec<String>,
	name: String,
	color: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JournalEntryRequest {
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	assigned_object_id: i64,
	assigned_object_type: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	created_by: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRole {
	custom_fields: serde_json::value::Value,
	description: String,
	created: Option<String>,
	id: i64,
	inventoryitem_count: i64,
	color: String,
	last_updated: Option<String>,
	display: String,
	slug: String,
	tags: Vec<NestedTag>,
	name: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceWithConfigContextRequest {
	platform: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	primary_ip4: Option<i64>,
	description: String,
	location: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// The function this device serves
	role: i64,
	position: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	virtual_chassis: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	config_template: Option<i64>,
	tenant: Option<i64>,
	primary_ip6: Option<i64>,
	oob_ip: Option<i64>,
	comments: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	device_type: i64,
	site: i64,
	name: Option<String>,
	cluster: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	rack: Option<i64>,
	vc_position: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RIRRequest {
	custom_fields: serde_json::value::Value,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortRequest {
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: serde_json::value::Value,
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
	color: String,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedInventoryItemRoleRequest {
	custom_fields: serde_json::value::Value,
	slug: String,
	color: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPRange {
	created: Option<String>,
	start_address: String,
	end_address: String,
	description: String,
	custom_fields: serde_json::value::Value,
	/// Treat as 100% utilized
	mark_utilized: bool,
	display: String,
	status: serde_json::value::Value,
	size: i64,
	url: String,
	comments: String,
	family: serde_json::value::Value,
	tags: Vec<NestedTag>,
	id: i64,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
	weight: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableAggregateRequest {
	prefix: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	date_added: Option<String>,
	tenant: Option<i64>,
	description: String,
	custom_fields: serde_json::value::Value,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRIRRequest {
	custom_fields: serde_json::value::Value,
	slug: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedVLANGroupRequest {
	scope_type: Option<String>,
	scope_id: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRouteTargetRequest {
	comments: String,
	custom_fields: serde_json::value::Value,
	tenant: Option<i64>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomFieldList {
	next: Option<String>,
	count: i64,
	results: Vec<CustomField>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPanelRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactAssignmentList {
	count: i64,
	next: Option<String>,
	results: Vec<ContactAssignment>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRequest {
	description: String,
	name: String,
	custom_fields: serde_json::value::Value,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleBayList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<ModuleBay>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableAggregateRequest {
	prefix: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
	date_added: Option<String>,
	comments: String,
	custom_fields: serde_json::value::Value,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePrefixRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	tenant: Option<i64>,
	vlan: Option<i64>,
	prefix: String,
	site: Option<i64>,
	comments: String,
	custom_fields: serde_json::value::Value,
	/// The primary function of this prefix
	role: Option<i64>,
	vrf: Option<i64>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTenantRequest {
	name: String,
	comments: String,
	slug: String,
	group: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
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
	name: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Provider {
	last_updated: Option<String>,
	/// Full name of the provider
	name: String,
	circuit_count: i64,
	url: String,
	comments: String,
	asns: Vec<i64>,
	slug: String,
	accounts: Vec<i64>,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	id: i64,
	display: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMInterfaceRequest {
	description: String,
	tagged_vlans: Vec<i64>,
	custom_fields: serde_json::value::Value,
	mac_address: Option<String>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	enabled: bool,
	mtu: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRackRoleRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
	slug: String,
	color: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VRFRequest {
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	comments: String,
	custom_fields: serde_json::value::Value,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	name: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPRangeList {
	next: Option<String>,
	results: Vec<IPRange>,
	count: i64,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDisk {
	size: i64,
	id: i64,
	url: String,
	custom_fields: serde_json::value::Value,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	name: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPortRequest {
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: serde_json::value::Value,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	device: i64,
	name: String,
	/// Physical label
	label: String,
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
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecPolicy {
	url: String,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLAN {
	ssid: String,
	auth_type: serde_json::value::Value,
	custom_fields: serde_json::value::Value,
	display: String,
	id: i64,
	description: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	url: String,
	last_updated: Option<String>,
	status: serde_json::value::Value,
	auth_cipher: serde_json::value::Value,
	auth_psk: String,
	comments: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedASNList {
	count: i64,
	next: Option<String>,
	results: Vec<ASN>,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTenantList {
	results: Vec<Tenant>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableServiceRequest {
	tags: Vec<NestedTagRequest>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	ports: Vec<i64>,
	virtual_machine: Option<i64>,
	device: Option<i64>,
	name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	description: String,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFrontPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: serde_json::value::Value,
	device: i64,
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	name: String,
	rear_port: i64,
	color: String,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteRequest {
	/// Full name of the site
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPanelRequest {
	name: String,
	description: String,
	location: Option<i64>,
	tags: Vec<NestedTagRequest>,
	site: i64,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageAttachmentRequest {
	image_width: i64,
	image_height: i64,
	content_type: String,
	object_id: i64,
	name: String,
	image: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelGroupRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCustomFieldChoiceSet {
	display: String,
	id: i64,
	choices_count: String,
	name: String,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRole {
	name: String,
	url: String,
	slug: String,
	display: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactGroup {
	slug: String,
	id: i64,
	display: String,
	_depth: i64,
	url: String,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTenantGroupRequest {
	custom_fields: serde_json::value::Value,
	description: String,
	slug: String,
	name: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSiteGroupList {
	results: Vec<SiteGroup>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRole {
	name: String,
	slug: String,
	display: String,
	url: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterType {
	created: Option<String>,
	last_updated: Option<String>,
	url: String,
	tags: Vec<NestedTag>,
	display: String,
	description: String,
	cluster_count: i64,
	slug: String,
	id: i64,
	name: String,
	custom_fields: serde_json::value::Value,
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
pub struct PatchedWritableASNRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	custom_fields: serde_json::value::Value,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProfileRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceType {
	model: String,
	display: String,
	url: String,
	id: i64,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTagList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<Tag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteTarget {
	display: String,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	id: i64,
	url: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortRequest {
	cable: Option<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRoleRequest {
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	weight: i64,
	description: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// Full name of the site
	name: String,
	time_zone: Option<String>,
	/// Physical location of the building
	physical_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	slug: String,
	comments: String,
	/// If different from the physical address
	shipping_address: String,
	/// Local facility ID or description
	facility: String,
	asns: Vec<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedContactRoleRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBay {
	url: String,
	created: Option<String>,
	description: String,
	id: i64,
	/// Physical label
	label: String,
	display: String,
	name: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderNetwork {
	service_id: String,
	comments: String,
	id: i64,
	display: String,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	url: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	description: String,
	name: String,
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
pub struct WritableCircuitTerminationRequest {
	provider_network: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	site: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	custom_fields: serde_json::value::Value,
	circuit: i64,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPAddress {
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	family: serde_json::value::Value,
	assigned_object_type: Option<String>,
	assigned_object_id: Option<i64>,
	address: String,
	nat_outside: Vec<NestedIPAddress>,
	id: i64,
	status: serde_json::value::Value,
	description: String,
	comments: String,
	role: serde_json::value::Value,
	created: Option<String>,
	tags: Vec<NestedTag>,
	display: String,
	url: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecPolicyRequest {
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	proposals: Vec<i64>,
	custom_fields: serde_json::value::Value,
	name: String,
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
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatformRequest {
	description: String,
	custom_fields: serde_json::value::Value,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedServiceTemplateList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<ServiceTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	module_type: Option<i64>,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	/// Physical label
	label: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNTermination {
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	assigned_object_id: i64,
	url: String,
	id: i64,
	assigned_object_type: String,
	tags: Vec<NestedTag>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRouteTargetList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<RouteTarget>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cluster {
	id: i64,
	comments: String,
	display: String,
	name: String,
	virtualmachine_count: i64,
	url: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	device_count: i64,
	status: serde_json::value::Value,
	description: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCableRequest {
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderRequest {
	description: String,
	comments: String,
	slug: String,
	asns: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	accounts: Vec<i64>,
	custom_fields: serde_json::value::Value,
	/// Full name of the provider
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderAccount {
	url: String,
	name: String,
	id: i64,
	comments: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	account: String,
	description: String,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataSource {
	display: String,
	name: String,
	id: i64,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePort {
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	r#type: serde_json::value::Value,
	speed: Option<serde_json::value::Value>,
	connected_endpoints_type: String,
	connected_endpoints_reachable: bool,
	last_updated: Option<String>,
	connected_endpoints: Vec<String>,
	cable_end: String,
	_occupied: bool,
	display: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	url: String,
	description: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	name: String,
	link_peers: Vec<String>,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInventoryItemRoleRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualDeviceContextRequest {
	description: String,
	custom_fields: serde_json::value::Value,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	primary_ip4: Option<i64>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	comments: String,
	primary_ip6: Option<i64>,
	device: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	tenant: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANGroup {
	id: i64,
	url: String,
	display: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Aggregate {
	display: String,
	date_added: Option<String>,
	comments: String,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	description: String,
	url: String,
	family: serde_json::value::Value,
	prefix: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCustomFieldChoiceSetRequest {
	name: String,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	description: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	extra_choices: Option<Vec<Vec<String>>>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVRFRequest {
	export_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	import_targets: Vec<i64>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	name: String,
	comments: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableTermination {
	display: String,
	last_updated: Option<String>,
	url: String,
	termination_id: i64,
	cable: i64,
	created: Option<String>,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	id: i64,
	termination_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRole {
	slug: String,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	url: String,
	id: i64,
	name: String,
	display: String,
	created: Option<String>,
	description: String,
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
pub struct PatchedWritableIKEProposalRequest {
	description: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
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
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPortList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<PowerPort>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitType {
	id: i64,
	display: String,
	name: String,
	url: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DashboardRequest {
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInventoryItemTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	description: String,
	component_id: Option<i64>,
	component_type: Option<String>,
	parent: Option<i64>,
	/// Physical label
	label: String,
	device_type: i64,
	role: Option<i64>,
	manufacturer: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPortRequest {
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
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
	description: String,
	/// Physical label
	label: String,
	device: i64,
	name: String,
	custom_fields: serde_json::value::Value,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLinkRequest {
	ssid: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	tags: Vec<NestedTagRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	auth_psk: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRegionRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	custom_fields: serde_json::value::Value,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIKEPolicyRequest {
	tags: Vec<NestedTagRequest>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	comments: String,
	description: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	name: String,
	proposals: Vec<i64>,
	preshared_key: String,
	custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortTemplate {
	id: i64,
	url: String,
	description: String,
	color: String,
	/// Physical label
	label: String,
	r#type: serde_json::value::Value,
	display: String,
	positions: i64,
	created: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTunnelRequest {
	description: String,
	tunnel_id: Option<i64>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	ipsec_profile: Option<i64>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	group: Option<i64>,
	custom_fields: serde_json::value::Value,
	comments: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPanelRequest {
	location: Option<i64>,
	name: String,
	description: String,
	site: i64,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePlatformRequest {
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	name: String,
	description: String,
	slug: String,
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRoleList {
	results: Vec<Role>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableASNRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	tenant: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPAddressRequest {
	address: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCustomFieldRequest {
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	content_types: Vec<String>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
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
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	description: String,
	/// Internal field name
	name: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	choice_set: Option<i64>,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tag {
	slug: String,
	tagged_items: i64,
	name: String,
	description: String,
	display: String,
	object_types: Vec<String>,
	created: Option<String>,
	last_updated: Option<String>,
	color: String,
	id: i64,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLANRequest {
	tenant: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	auth_psk: String,
	vlan: Option<i64>,
	ssid: String,
	comments: String,
	group: Option<i64>,
	description: String,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemTemplateRequest {
	/// Physical label
	label: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	description: String,
	parent: Option<i64>,
	component_type: Option<String>,
	component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRangeRequest {
	end: i64,
	description: String,
	start: i64,
	custom_fields: serde_json::value::Value,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleRequest {
	serial: String,
	device: i64,
	comments: String,
	custom_fields: serde_json::value::Value,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	module_bay: i64,
	description: String,
	module_type: i64,
	tags: Vec<NestedTagRequest>,
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
pub struct RackRequest {
	description: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	facility_id: Option<String>,
	name: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: Option<String>,
	/// Height in rack units
	u_height: i64,
	/// Starting unit for rack
	starting_unit: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: Option<String>,
	weight: Option<f64>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	comments: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	custom_fields: serde_json::value::Value,
	serial: String,
	tags: Vec<NestedTagRequest>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPN {
	comments: String,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	slug: String,
	name: String,
	import_targets: Vec<i64>,
	id: i64,
	url: String,
	identifier: Option<i64>,
	tags: Vec<NestedTag>,
	display: String,
	last_updated: Option<String>,
	description: String,
	r#type: serde_json::value::Value,
	export_targets: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bookmark {
	url: String,
	display: String,
	id: i64,
	object_id: i64,
	object_type: String,
	created: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRequest {
	name: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceRequest {
	custom_fields: serde_json::value::Value,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	description: String,
	comments: String,
	name: String,
	ipaddresses: Vec<i64>,
	ports: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactRequest {
	group: Option<i64>,
	phone: String,
	address: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	title: String,
	name: String,
	description: String,
	email: String,
	link: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFHRPGroupAssignmentRequest {
	interface_type: String,
	priority: i64,
	group: i64,
	interface_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomField {
	content_types: Vec<String>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	object_type: String,
	filter_logic: serde_json::value::Value,
	description: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	data_type: String,
	id: i64,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	ui_visible: serde_json::value::Value,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	display: String,
	created: Option<String>,
	/// Internal field name
	name: String,
	r#type: serde_json::value::Value,
	last_updated: Option<String>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	ui_editable: serde_json::value::Value,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	url: String,
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
pub struct WritableDeviceTypeRequest {
	custom_fields: serde_json::value::Value,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	rear_image: String,
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	model: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	manufacturer: i64,
	/// Discrete part number (optional)
	part_number: String,
	u_height: f64,
	front_image: String,
	default_platform: Option<i64>,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	description: String,
	slug: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInventoryItemTemplateRequest {
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	manufacturer: Option<i64>,
	parent: Option<i64>,
	role: Option<i64>,
	device_type: i64,
	component_type: Option<String>,
	description: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderAccountRequest {
	description: String,
	provider: i64,
	name: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	account: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	cid: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Module {
	status: serde_json::value::Value,
	id: i64,
	tags: Vec<NestedTag>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	serial: String,
	description: String,
	custom_fields: serde_json::value::Value,
	display: String,
	comments: String,
	created: Option<String>,
	last_updated: Option<String>,
	url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerOutletTemplateList {
	results: Vec<PowerOutletTemplate>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVMInterfaceList {
	previous: Option<String>,
	count: i64,
	results: Vec<VMInterface>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigContextRequest {
	cluster_groups: Vec<i64>,
	regions: Vec<i64>,
	locations: Vec<i64>,
	tenant_groups: Vec<i64>,
	platforms: Vec<i64>,
	sites: Vec<i64>,
	weight: i64,
	is_active: bool,
	description: String,
	clusters: Vec<i64>,
	roles: Vec<i64>,
	tags: Vec<String>,
	site_groups: Vec<i64>,
	name: String,
	tenants: Vec<i64>,
	device_types: Vec<i64>,
	cluster_types: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsolePortTemplateRequest {
	module_type: Option<i64>,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceBayTemplateRequest {
	/// Physical label
	label: String,
	description: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderAccountRequest {
	name: String,
	account: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemTemplate {
	display: String,
	id: i64,
	component_type: Option<String>,
	/// Physical label
	label: String,
	_depth: i64,
	created: Option<String>,
	component_id: Option<i64>,
	description: String,
	url: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	last_updated: Option<String>,
	parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
	name: String,
	display: String,
	last_updated: Option<String>,
	custom_fields: serde_json::value::Value,
	ports: Vec<i64>,
	comments: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	id: i64,
	description: String,
	ipaddresses: Vec<i64>,
	url: String,
	protocol: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLANGroupList {
	count: i64,
	next: Option<String>,
	results: Vec<WirelessLANGroup>,
	previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenRequest {
	last_used: Option<String>,
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	key: String,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemRoleList {
	next: Option<String>,
	results: Vec<InventoryItemRole>,
	previous: Option<String>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPlatform {
	id: i64,
	slug: String,
	display: String,
	url: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedJournalEntryList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<JournalEntry>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrefixRequest {
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	custom_fields: serde_json::value::Value,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Treat as 100% utilized
	mark_utilized: bool,
	prefix: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dashboard {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRackReservationRequest {
	description: String,
	rack: i64,
	comments: String,
	user: i64,
	tenant: Option<i64>,
	custom_fields: serde_json::value::Value,
	units: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AggregateRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	prefix: String,
	date_added: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantGroupRequest {
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRearPortTemplateRequest {
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
	device_type: Option<i64>,
	description: String,
	color: String,
	module_type: Option<i64>,
	positions: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceTypeRequest {
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	rear_image: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	description: String,
	manufacturer: i64,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	comments: String,
	default_platform: Option<i64>,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	model: String,
	u_height: f64,
	slug: String,
	front_image: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRearPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	module: Option<i64>,
	device: i64,
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
	custom_fields: serde_json::value::Value,
	/// Number of front ports which may be mapped
	positions: i64,
	color: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Physical label
	label: String,
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
pub struct ASNRequest {
	custom_fields: serde_json::value::Value,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletTemplate {
	feed_leg: Option<serde_json::value::Value>,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	id: i64,
	description: String,
	last_updated: Option<String>,
	url: String,
	r#type: Option<serde_json::value::Value>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableClusterRequest {
	r#type: i64,
	tenant: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	site: Option<i64>,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	group: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortTemplate {
	/// Physical label
	label: String,
	last_updated: Option<String>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	created: Option<String>,
	r#type: Option<serde_json::value::Value>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: String,
	display: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	id: i64,
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
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	choice_set: Option<i64>,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
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
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	object_type: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	description: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Internal field name
	name: String,
	content_types: Vec<String>,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPRangeRequest {
	start_address: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	end_address: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	vrf: Option<i64>,
	tenant: Option<i64>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// The primary function of this range
	role: Option<i64>,
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
pub struct PatchedWritableIPSecProposalRequest {
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	description: String,
	name: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderAccountRequest {
	account: String,
	tags: Vec<NestedTagRequest>,
	provider: i64,
	description: String,
	name: String,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleNestedModuleBayRequest {
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactGroup {
	_depth: i64,
	id: i64,
	slug: String,
	url: String,
	created: Option<String>,
	description: String,
	custom_fields: serde_json::value::Value,
	display: String,
	tags: Vec<NestedTag>,
	name: String,
	last_updated: Option<String>,
	contact_count: i64,
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
pub struct NestedRearPortTemplate {
	id: i64,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTenantGroupRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
	description: String,
	name: String,
	parent: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRole {
	id: i64,
	url: String,
	slug: String,
	display: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPortTemplateList {
	previous: Option<String>,
	results: Vec<PowerPortTemplate>,
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

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerFeedRequest {
	custom_fields: serde_json::value::Value,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	tenant: Option<i64>,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	name: String,
	description: String,
	power_panel: i64,
	tags: Vec<NestedTagRequest>,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	voltage: i64,
	rack: Option<i64>,
	amperage: i64,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEProposalRequest {
	name: String,
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
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConfigTemplateRequest {
	data_file: Option<i64>,
	/// Jinja2 template code.
	template_code: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Remote data source
	data_source: Option<i64>,
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

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Platform {
	device_count: i64,
	description: String,
	virtualmachine_count: i64,
	id: i64,
	display: String,
	created: Option<String>,
	slug: String,
	name: String,
	url: String,
	last_updated: Option<String>,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTag>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANGroupRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
	description: String,
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
pub struct PaginatedContactGroupList {
	next: Option<String>,
	count: i64,
	results: Vec<ContactGroup>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Role {
	name: String,
	url: String,
	description: String,
	display: String,
	custom_fields: serde_json::value::Value,
	slug: String,
	tags: Vec<NestedTag>,
	prefix_count: i64,
	vlan_count: i64,
	created: Option<String>,
	weight: i64,
	id: i64,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedUser {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	url: String,
	id: i64,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackRoleList {
	results: Vec<RackRole>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePlatformRequest {
	slug: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	name: String,
	config_template: Option<i64>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTemplateRequest {
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	content_types: Vec<String>,
	name: String,
	description: String,
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
pub struct NestedIPSecProfile {
	id: i64,
	display: String,
	name: String,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPN {
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
	url: String,
	id: i64,
	name: String,
	display: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecProfileRequest {
	tags: Vec<NestedTagRequest>,
	ike_policy: i64,
	name: String,
	description: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	ipsec_policy: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedManufacturerList {
	count: i64,
	results: Vec<Manufacturer>,
	previous: Option<String>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedFilterRequest {
	slug: String,
	weight: i64,
	name: String,
	shared: bool,
	content_types: Vec<String>,
	description: String,
	user: Option<i64>,
	enabled: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortTemplateRequest {
	/// Physical label
	label: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
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
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantRequest {
	custom_fields: serde_json::value::Value,
	name: String,
	slug: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelGroupList {
	previous: Option<String>,
	count: i64,
	results: Vec<TunnelGroup>,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Site {
	description: String,
	/// Local facility ID or description
	facility: String,
	asns: Vec<i64>,
	rack_count: i64,
	url: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	last_updated: Option<String>,
	id: i64,
	display: String,
	/// Physical location of the building
	physical_address: String,
	tags: Vec<NestedTag>,
	circuit_count: i64,
	device_count: i64,
	/// Full name of the site
	name: String,
	time_zone: Option<String>,
	/// If different from the physical address
	shipping_address: String,
	slug: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	status: serde_json::value::Value,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	created: Option<String>,
	prefix_count: i64,
	virtualmachine_count: i64,
	vlan_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDataSourceRequest {
	description: String,
	enabled: bool,
	comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	r#type: String,
	source_url: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceRoleRequest {
	tags: Vec<NestedTagRequest>,
	color: String,
	name: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	config_template: Option<i64>,
	description: String,
	custom_fields: serde_json::value::Value,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableServiceRequest {
	virtual_machine: Option<i64>,
	ports: Vec<i64>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	name: String,
	custom_fields: serde_json::value::Value,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	description: String,
	comments: String,
	device: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRequest {
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	color: String,
	description: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: serde_json::value::Value,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldChoiceSetRequest {
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	name: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactGroupRequest {
	name: String,
	slug: String,
	custom_fields: serde_json::value::Value,
	parent: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GroupRequest {
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventRuleRequest {
	content_types: Vec<String>,
	custom_fields: serde_json::value::Value,
	/// Triggers when a matching object is updated.
	type_update: bool,
	name: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	enabled: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	action_object_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_id: Option<i64>,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVMInterface {
	name: String,
	url: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleTypeRequest {
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	model: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	/// Discrete part number (optional)
	part_number: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedExportTemplateList {
	results: Vec<ExportTemplate>,
	next: Option<String>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRouteTargetRequest {
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	comments: String,
	description: String,
	custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCableList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<Cable>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualDiskRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
	virtual_machine: i64,
	name: String,
	size: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsolePortRequest {
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
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	device: i64,
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
	name: String,
	custom_fields: serde_json::value::Value,
	module: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecPolicyRequest {
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
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomFieldChoiceSetList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<CustomFieldChoiceSet>,
}

/// Representation of a VLAN which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableVLAN {
	vid: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDataFileList {
	next: Option<String>,
	count: i64,
	previous: Option<String>,
	results: Vec<DataFile>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableLocationRequest {
	tags: Vec<NestedTagRequest>,
	site: i64,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	name: String,
	slug: String,
	parent: Option<i64>,
	custom_fields: serde_json::value::Value,
	tenant: Option<i64>,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedBookmarkList {
	next: Option<String>,
	results: Vec<Bookmark>,
	previous: Option<String>,
	count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPanelList {
	previous: Option<String>,
	results: Vec<PowerPanel>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceRequest {
	name: String,
	mtu: Option<i64>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	tagged_vlans: Vec<i64>,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	description: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
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
	wwn: Option<String>,
	wireless_lans: Vec<i64>,
	tx_power: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	enabled: bool,
	tags: Vec<NestedTagRequest>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	vdcs: Vec<i64>,
	/// Physical label
	label: String,
	speed: Option<i64>,
	mac_address: Option<String>,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactAssignmentRequest {
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	custom_fields: serde_json::value::Value,
	content_type: String,
	object_id: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Circuit {
	comments: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	install_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	last_updated: Option<String>,
	status: serde_json::value::Value,
	termination_date: Option<String>,
	/// Unique circuit ID
	cid: String,
	description: String,
	created: Option<String>,
	url: String,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCableTerminationRequest {
	cable: i64,
	termination_id: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	termination_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDeviceContext {
	url: String,
	interface_count: i64,
	description: String,
	status: serde_json::value::Value,
	display: String,
	comments: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	last_updated: Option<String>,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
	id: i64,
	name: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPAddressRequest {
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
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
	tenant: Option<i64>,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	address: String,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	custom_fields: serde_json::value::Value,
	vrf: Option<i64>,
	assigned_object_type: Option<String>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsolePortTemplateRequest {
	module_type: Option<i64>,
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
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceRoleRequest {
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	custom_fields: serde_json::value::Value,
	name: String,
	config_template: Option<i64>,
	color: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupRequest {
	description: String,
	group_id: i64,
	comments: String,
	auth_key: String,
	name: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceTemplate {
	display: String,
	url: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedLocationRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Interface {
	/// Physical label
	label: String,
	mode: serde_json::value::Value,
	connected_endpoints_type: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints: Vec<String>,
	poe_type: serde_json::value::Value,
	custom_fields: serde_json::value::Value,
	count_fhrp_groups: i64,
	id: i64,
	mtu: Option<i64>,
	enabled: bool,
	cable_end: String,
	created: Option<String>,
	rf_channel: serde_json::value::Value,
	count_ipaddresses: i64,
	duplex: Option<serde_json::value::Value>,
	_occupied: bool,
	url: String,
	mac_address: Option<String>,
	last_updated: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	tags: Vec<NestedTag>,
	tagged_vlans: Vec<i64>,
	wireless_lans: Vec<i64>,
	wwn: Option<String>,
	rf_role: serde_json::value::Value,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	display: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	name: String,
	vdcs: Vec<i64>,
	poe_mode: serde_json::value::Value,
	speed: Option<i64>,
	connected_endpoints_reachable: bool,
	r#type: serde_json::value::Value,
	tx_power: Option<i64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	link_peers: Vec<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRegion {
	name: String,
	id: i64,
	_depth: i64,
	display: String,
	slug: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLinkRequest {
	auth_psk: String,
	comments: String,
	description: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	ssid: String,
	interface_a: i64,
	custom_fields: serde_json::value::Value,
	interface_b: i64,
	tenant: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelTermination {
	url: String,
	role: serde_json::value::Value,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	id: i64,
	termination_type: String,
	created: Option<String>,
	termination_id: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLink {
	id: i64,
	status: serde_json::value::Value,
	ssid: String,
	display: String,
	tags: Vec<NestedTag>,
	auth_psk: String,
	auth_type: serde_json::value::Value,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
	comments: String,
	last_updated: Option<String>,
	auth_cipher: serde_json::value::Value,
	description: String,
	url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFrontPortTemplateRequest {
	/// Physical label
	label: String,
	device_type: Option<i64>,
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
	module_type: Option<i64>,
	rear_port: i64,
	rear_port_position: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderNetworkRequest {
	name: String,
	service_id: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableL2VPNRequest {
	slug: String,
	name: String,
	description: String,
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
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	comments: String,
	tenant: Option<i64>,
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableUserRequest {
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	email: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	first_name: String,
	last_name: String,
	password: String,
	date_joined: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	vcpus: Option<f64>,
	device: Option<i64>,
	description: String,
	memory: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	tenant: Option<i64>,
	role: Option<i64>,
	cluster: Option<i64>,
	custom_fields: serde_json::value::Value,
	name: String,
	disk: Option<i64>,
	comments: String,
	platform: Option<i64>,
	primary_ip4: Option<i64>,
	site: Option<i64>,
	primary_ip6: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLANGroup {
	display: String,
	url: String,
	name: String,
	slug: String,
	_depth: i64,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLANList {
	next: Option<String>,
	count: i64,
	results: Vec<WirelessLAN>,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDataSourceList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<DataSource>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedFHRPGroup {
	display: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	group_id: i64,
	id: i64,
	url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFrontPortTemplateRequest {
	module_type: Option<i64>,
	color: String,
	rear_port_position: i64,
	device_type: Option<i64>,
	rear_port: i64,
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
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataSourceRequest {
	source_url: String,
	description: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	enabled: bool,
	name: String,
	comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPanelRequest {
	name: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableLocationRequest {
	slug: String,
	tenant: Option<i64>,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	site: i64,
	parent: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
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

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteGroup {
	id: i64,
	display: String,
	url: String,
	name: String,
	slug: String,
	_depth: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceWithConfigContextRequest {
	location: Option<i64>,
	tenant: Option<i64>,
	name: Option<String>,
	platform: Option<i64>,
	oob_ip: Option<i64>,
	virtual_chassis: Option<i64>,
	vc_position: Option<i64>,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	/// The function this device serves
	role: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	config_template: Option<i64>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	position: Option<f64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	device_type: i64,
	cluster: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	comments: String,
	primary_ip6: Option<i64>,
	site: i64,
	primary_ip4: Option<i64>,
	rack: Option<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLANGroupRequest {
	slug: String,
	description: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTerminationRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
	/// Patch panel ID and port number(s)
	pp_info: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableObjectPermissionRequest {
	users: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	groups: Vec<i64>,
	object_types: Vec<String>,
	name: String,
	description: String,
	enabled: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInterfaceRequest {
	name: String,
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
	mtu: Option<i64>,
	speed: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	lag: Option<i64>,
	tagged_vlans: Vec<i64>,
	tx_power: Option<i64>,
	wireless_lans: Vec<i64>,
	module: Option<i64>,
	description: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	enabled: bool,
	wwn: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	/// Physical label
	label: String,
	device: i64,
	untagged_vlan: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	mac_address: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
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
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	vrf: Option<i64>,
	vdcs: Vec<i64>,
	parent: Option<i64>,
	bridge: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRequest {
	address: String,
	tags: Vec<NestedTagRequest>,
	title: String,
	name: String,
	custom_fields: serde_json::value::Value,
	comments: String,
	link: String,
	phone: String,
	email: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactAssignmentRequest {
	content_type: String,
	custom_fields: serde_json::value::Value,
	role: i64,
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	contact: i64,
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

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderAccountRequest {
	account: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPAddressList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<IPAddress>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANGroup {
	id: i64,
	tags: Vec<NestedTag>,
	wirelesslan_count: i64,
	_depth: i64,
	slug: String,
	custom_fields: serde_json::value::Value,
	description: String,
	created: Option<String>,
	url: String,
	last_updated: Option<String>,
	display: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedASNRangeList {
	count: i64,
	results: Vec<ASNRange>,
	previous: Option<String>,
	next: Option<String>,
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
pub struct RearPort {
	last_updated: Option<String>,
	link_peers: Vec<String>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	cable_end: String,
	name: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	r#type: serde_json::value::Value,
	id: i64,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
	url: String,
	/// Physical label
	label: String,
	color: String,
	/// Number of front ports which may be mapped
	positions: i64,
	display: String,
	tags: Vec<NestedTag>,
	_occupied: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRoleRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecProfileList {
	previous: Option<String>,
	results: Vec<IPSecProfile>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleBayRequest {
	installed_module: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	device: i64,
	name: String,
	custom_fields: serde_json::value::Value,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterGroup {
	display: String,
	url: String,
	slug: String,
	description: String,
	tags: Vec<NestedTag>,
	id: i64,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	name: String,
	cluster_count: i64,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPort {
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	name: String,
	link_peers: Vec<String>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	display: String,
	color: String,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	_occupied: bool,
	id: i64,
	cable_end: String,
	url: String,
	r#type: serde_json::value::Value,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNRequest {
	name: String,
	custom_fields: serde_json::value::Value,
	description: String,
	slug: String,
	export_targets: Vec<i64>,
	import_targets: Vec<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVRF {
	name: String,
	url: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	id: i64,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedJobList {
	count: i64,
	next: Option<String>,
	results: Vec<Job>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualDiskRequest {
	virtual_machine: i64,
	name: String,
	size: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderRequest {
	description: String,
	accounts: Vec<i64>,
	comments: String,
	/// Full name of the provider
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	asns: Vec<i64>,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePrefixRequest {
	site: Option<i64>,
	vrf: Option<i64>,
	description: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	custom_fields: serde_json::value::Value,
	prefix: String,
	tenant: Option<i64>,
	vlan: Option<i64>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// The primary function of this prefix
	role: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTunnelRequest {
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	name: String,
	custom_fields: serde_json::value::Value,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	group: Option<i64>,
	ipsec_profile: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	tenant: Option<i64>,
	tunnel_id: Option<i64>,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInterfaceList {
	previous: Option<String>,
	count: i64,
	results: Vec<Interface>,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCircuitRequest {
	tenant: Option<i64>,
	provider_account: Option<i64>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	install_date: Option<String>,
	termination_date: Option<String>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	description: String,
	/// Unique circuit ID
	cid: String,
	provider: i64,
	r#type: i64,
	/// Committed rate
	commit_rate: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenProvisionRequest {
	description: String,
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	username: String,
	password: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedConfigTemplate {
	name: String,
	id: i64,
	url: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedClusterGroupRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactGroupRequest {
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableEventRuleRequest {
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	tags: Vec<NestedTagRequest>,
	content_types: Vec<String>,
	name: String,
	action_object_type: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	custom_fields: serde_json::value::Value,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_id: Option<i64>,
	description: String,
	enabled: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLinkRequest {
	ssid: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProfile {
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	comments: String,
	display: String,
	url: String,
	custom_fields: serde_json::value::Value,
	mode: serde_json::value::Value,
	name: String,
	description: String,
	created: Option<String>,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProvider {
	id: i64,
	/// Full name of the provider
	name: String,
	slug: String,
	url: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantGroup {
	slug: String,
	_depth: i64,
	id: i64,
	url: String,
	name: String,
	display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldRequest {
	/// Internal field name
	name: String,
	content_types: Vec<String>,
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
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	object_type: String,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualMachine {
	id: i64,
	display: String,
	url: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRackRequest {
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	location: Option<i64>,
	/// Functional role
	role: Option<i64>,
	site: i64,
	tenant: Option<i64>,
	/// Height in rack units
	u_height: i64,
	facility_id: Option<String>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	tags: Vec<NestedTagRequest>,
	weight: Option<f64>,
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
	/// Starting unit for rack
	starting_unit: i64,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	serial: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	description: String,
	comments: String,
	name: String,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVRFList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<VRF>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVLANGroupList {
	results: Vec<VLANGroup>,
	count: i64,
	previous: Option<String>,
	next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedConfigTemplateRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedL2VPNTerminationList {
	previous: Option<String>,
	results: Vec<L2VPNTermination>,
	count: i64,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInterfaceTemplateRequest {
	module_type: Option<i64>,
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
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	device_type: Option<i64>,
	description: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	mgmt_only: bool,
	bridge: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNTerminationRequest {
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderNetworkList {
	results: Vec<ProviderNetwork>,
	count: i64,
	next: Option<String>,
	previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRoleRequest {
	name: String,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualMachineWithConfigContextList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<VirtualMachineWithConfigContext>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualMachineWithConfigContextRequest {
	comments: String,
	custom_fields: serde_json::value::Value,
	memory: Option<i64>,
	disk: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	vcpus: Option<f64>,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVRFRequest {
	tags: Vec<NestedTagRequest>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	comments: String,
	name: String,
	custom_fields: serde_json::value::Value,
	tenant: Option<i64>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	description: String,
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerOutletTemplateRequest {
	description: String,
	module_type: Option<i64>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	power_port: Option<i64>,
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
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsoleServerPortTemplateRequest {
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
	device_type: Option<i64>,
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactAssignmentRequest {
	object_id: i64,
	contact: i64,
	content_type: String,
	role: i64,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortTemplateRequest {
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
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualMachineWithConfigContext {
	interface_count: i64,
	name: String,
	url: String,
	comments: String,
	created: Option<String>,
	status: serde_json::value::Value,
	display: String,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	virtual_disk_count: i64,
	description: String,
	id: i64,
	disk: Option<i64>,
	memory: Option<i64>,
	tags: Vec<NestedTag>,
	vcpus: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Job {
	job_id: String,
	id: i64,
	status: serde_json::value::Value,
	error: String,
	display: String,
	url: String,
	created: String,
	/// Recurrence interval (in minutes)
	interval: Option<i64>,
	scheduled: Option<String>,
	started: Option<String>,
	completed: Option<String>,
	object_id: Option<i64>,
	name: String,
	object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInterfaceTemplateRequest {
	bridge: Option<i64>,
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
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	enabled: bool,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	device_type: Option<i64>,
	description: String,
	module_type: Option<i64>,
	mgmt_only: bool,
	/// Physical label
	label: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteGroupRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: serde_json::value::Value,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Contact {
	email: String,
	display: String,
	comments: String,
	last_updated: Option<String>,
	address: String,
	id: i64,
	custom_fields: serde_json::value::Value,
	description: String,
	title: String,
	url: String,
	phone: String,
	link: String,
	tags: Vec<NestedTag>,
	name: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDeviceContextRequest {
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	tags: Vec<NestedTagRequest>,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	comments: String,
	name: String,
	custom_fields: serde_json::value::Value,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayTemplateRequest {
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataFile {
	/// SHA256 hash of the file data
	hash: String,
	/// File path relative to the data source's root
	path: String,
	display: String,
	last_updated: String,
	url: String,
	id: i64,
	size: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIKEProposalList {
	results: Vec<IKEProposal>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manufacturer {
	id: i64,
	devicetype_count: i64,
	inventoryitem_count: i64,
	platform_count: i64,
	description: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	url: String,
	display: String,
	last_updated: Option<String>,
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPAddress {
	display: String,
	id: i64,
	url: String,
	family: i64,
	address: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConfigTemplateRequest {
	data_file: Option<i64>,
	/// Remote data source
	data_source: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	/// Jinja2 template code.
	template_code: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRegionRequest {
	slug: String,
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecPolicy {
	proposals: Vec<i64>,
	created: Option<String>,
	name: String,
	pfs_group: serde_json::value::Value,
	description: String,
	id: i64,
	display: String,
	url: String,
	comments: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<Provider>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelTerminationRequest {
	custom_fields: serde_json::value::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_type: String,
	tags: Vec<NestedTagRequest>,
	termination_id: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Webhook {
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	custom_fields: serde_json::value::Value,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	name: String,
	id: i64,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	url: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	last_updated: Option<String>,
	description: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	display: String,
	tags: Vec<NestedTag>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceTypeRequest {
	comments: String,
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
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	u_height: f64,
	front_image: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	rear_image: String,
	slug: String,
	model: String,
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: Option<String>,
	description: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerOutletRequest {
	custom_fields: serde_json::value::Value,
	power_port: Option<i64>,
	name: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	module: Option<i64>,
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
	description: String,
	tags: Vec<NestedTagRequest>,
	device: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagRequest {
	color: String,
	name: String,
	description: String,
	object_types: Vec<String>,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelRequest {
	tunnel_id: Option<i64>,
	custom_fields: serde_json::value::Value,
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableSiteGroupRequest {
	custom_fields: serde_json::value::Value,
	description: String,
	name: String,
	slug: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableL2VPNTerminationRequest {
	l2vpn: i64,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	assigned_object_id: i64,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupAssignment {
	created: Option<String>,
	last_updated: Option<String>,
	interface_id: i64,
	id: i64,
	url: String,
	interface_type: String,
	display: String,
	priority: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BookmarkRequest {
	object_id: i64,
	object_type: String,
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
pub struct NestedCircuit {
	id: i64,
	display: String,
	url: String,
	/// Unique circuit ID
	cid: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRearPortTemplateList {
	count: i64,
	previous: Option<String>,
	next: Option<String>,
	results: Vec<RearPortTemplate>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBay {
	/// Physical label
	label: String,
	name: String,
	created: Option<String>,
	display: String,
	description: String,
	id: i64,
	url: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	/// Identifier to reference when renaming installed components
	position: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConfigContextList {
	count: i64,
	results: Vec<ConfigContext>,
	previous: Option<String>,
	next: Option<String>,
}

/// Minimal representation of some generic object identified by ContentType and PK.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericObject {
	object_type: String,
	object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRack {
	display: String,
	name: String,
	id: i64,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataFile {
	id: i64,
	/// File path relative to the data source's root
	path: String,
	url: String,
	display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitType {
	description: String,
	display: String,
	name: String,
	id: i64,
	url: String,
	slug: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	created: Option<String>,
	color: String,
	circuit_count: i64,
}

/// Minimal representation of some generic object identified by ContentType and PK.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericObjectRequest {
	object_type: String,
	object_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedManufacturerRequest {
	description: String,
	custom_fields: serde_json::value::Value,
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualDeviceContextList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<VirtualDeviceContext>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cable {
	id: i64,
	b_terminations: Vec<GenericObject>,
	last_updated: Option<String>,
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
	status: serde_json::value::Value,
	color: String,
	display: String,
	url: String,
	length: Option<f64>,
	a_terminations: Vec<GenericObject>,
	custom_fields: serde_json::value::Value,
	comments: String,
	description: String,
	length_unit: Option<serde_json::value::Value>,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleNestedModuleBay {
	id: i64,
	name: String,
	display: String,
	url: String,
}

/// Representation of an ASN which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableASN {
	asn: i64,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInterfaceTemplateList {
	count: i64,
	results: Vec<InterfaceTemplate>,
	next: Option<String>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANRequest {
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	comments: String,
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
	auth_psk: String,
	description: String,
	ssid: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDiskRequest {
	custom_fields: serde_json::value::Value,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	size: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceWithConfigContext {
	rear_port_count: i64,
	console_server_port_count: i64,
	comments: String,
	last_updated: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	custom_fields: serde_json::value::Value,
	name: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	console_port_count: i64,
	power_outlet_count: i64,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	status: serde_json::value::Value,
	interface_count: i64,
	front_port_count: i64,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	created: Option<String>,
	device_bay_count: i64,
	inventory_item_count: i64,
	display: String,
	tags: Vec<NestedTag>,
	power_port_count: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	face: serde_json::value::Value,
	vc_position: Option<i64>,
	description: String,
	id: i64,
	position: Option<f64>,
	module_bay_count: i64,
	url: String,
	airflow: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleList {
	count: i64,
	next: Option<String>,
	previous: Option<String>,
	results: Vec<Module>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInventoryItemRole {
	display: String,
	id: i64,
	url: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWebhookRequest {
	description: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	custom_fields: serde_json::value::Value,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedDashboardRequest {
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCustomLinkRequest {
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
	name: String,
	weight: i64,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	enabled: bool,
	content_types: Vec<String>,
	/// Force link to open in a new window
	new_window: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleTypeList {
	next: Option<String>,
	count: i64,
	results: Vec<ModuleType>,
	previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderNetworkRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	comments: String,
	provider: i64,
	service_id: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableSiteRequest {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	tenant: Option<i64>,
	/// Local facility ID or description
	facility: String,
	time_zone: Option<String>,
	region: Option<i64>,
	/// Full name of the site
	name: String,
	slug: String,
	description: String,
	/// If different from the physical address
	shipping_address: String,
	/// Physical location of the building
	physical_address: String,
	tags: Vec<NestedTagRequest>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	group: Option<i64>,
	asns: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTenantRequest {
	slug: String,
	group: Option<i64>,
	comments: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderAccountList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<ProviderAccount>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerFeed {
	/// Treat as if a cable is connected
	mark_connected: bool,
	display: String,
	connected_endpoints_reachable: bool,
	custom_fields: serde_json::value::Value,
	url: String,
	voltage: i64,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	_occupied: bool,
	link_peers: Vec<String>,
	phase: serde_json::value::Value,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	id: i64,
	connected_endpoints_type: String,
	cable_end: String,
	tags: Vec<NestedTag>,
	status: serde_json::value::Value,
	last_updated: Option<String>,
	comments: String,
	r#type: serde_json::value::Value,
	amperage: i64,
	supply: serde_json::value::Value,
	connected_endpoints: Vec<String>,
	created: Option<String>,
	description: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableASNRangeRequest {
	tenant: Option<i64>,
	custom_fields: serde_json::value::Value,
	rir: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	name: String,
	end: i64,
	start: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedGroupList {
	count: i64,
	results: Vec<Group>,
	previous: Option<String>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualDiskList {
	count: i64,
	previous: Option<String>,
	results: Vec<VirtualDisk>,
	next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWebhookList {
	count: i64,
	previous: Option<String>,
	results: Vec<Webhook>,
	next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedFilter {
	shared: bool,
	last_updated: Option<String>,
	enabled: bool,
	slug: String,
	display: String,
	id: i64,
	url: String,
	name: String,
	user: Option<i64>,
	created: Option<String>,
	description: String,
	content_types: Vec<String>,
	weight: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterTypeRequest {
	custom_fields: serde_json::value::Value,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecPolicyRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDevice {
	id: i64,
	name: Option<String>,
	display: String,
	url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRole {
	id: i64,
	slug: String,
	display: String,
	name: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsolePortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	/// Physical label
	label: String,
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
	description: String,
	module: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	device: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegionRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	description: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableServiceTemplateRequest {
	name: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	/// Physical label
	label: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortTemplate {
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenant {
	id: i64,
	display: String,
	url: String,
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectPermission {
	name: String,
	display: String,
	description: String,
	id: i64,
	object_types: Vec<String>,
	url: String,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	users: Vec<i64>,
	enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSavedFilterList {
	results: Vec<SavedFilter>,
	next: Option<String>,
	count: i64,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedImageAttachmentList {
	next: Option<String>,
	results: Vec<ImageAttachment>,
	count: i64,
	previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedLocationList {
	previous: Option<String>,
	next: Option<String>,
	results: Vec<Location>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCableRequest {
	custom_fields: serde_json::value::Value,
	color: String,
	description: String,
	label: String,
	b_terminations: Vec<GenericObjectRequest>,
	tags: Vec<NestedTagRequest>,
	a_terminations: Vec<GenericObjectRequest>,
	tenant: Option<i64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	comments: String,
	length: Option<f64>,
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
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRole {
	url: String,
	color: String,
	slug: String,
	description: String,
	display: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: serde_json::value::Value,
	id: i64,
	rack_count: i64,
	last_updated: Option<String>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIKEPolicyRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigContext {
	url: String,
	description: String,
	cluster_types: Vec<i64>,
	data_synced: Option<String>,
	id: i64,
	sites: Vec<i64>,
	last_updated: Option<String>,
	tenant_groups: Vec<i64>,
	created: Option<String>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	display: String,
	clusters: Vec<i64>,
	weight: i64,
	is_active: bool,
	site_groups: Vec<i64>,
	regions: Vec<i64>,
	platforms: Vec<i64>,
	cluster_groups: Vec<i64>,
	tenants: Vec<i64>,
	name: String,
	locations: Vec<i64>,
	device_types: Vec<i64>,
	roles: Vec<i64>,
	tags: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataSource {
	enabled: bool,
	description: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	created: Option<String>,
	file_count: i64,
	comments: String,
	id: i64,
	name: String,
	source_url: String,
	r#type: serde_json::value::Value,
	status: serde_json::value::Value,
	url: String,
	display: String,
	last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: i64,
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLinkRequest {
	interface_b: i64,
	tenant: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	ssid: String,
	description: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	auth_psk: String,
	interface_a: i64,
	tags: Vec<NestedTagRequest>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableClusterRequest {
	site: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	tenant: Option<i64>,
	description: String,
	r#type: i64,
	group: Option<i64>,
	name: String,
	custom_fields: serde_json::value::Value,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInventoryItemRequest {
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	custom_fields: serde_json::value::Value,
	description: String,
	component_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	device: i64,
	serial: String,
	manufacturer: Option<i64>,
	name: String,
	parent: Option<i64>,
	role: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualChassisRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	domain: String,
	comments: String,
	name: String,
	custom_fields: serde_json::value::Value,
	master: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedImageAttachmentRequest {
	object_id: i64,
	image_height: i64,
	image: String,
	content_type: String,
	name: String,
	image_width: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRackRequest {
	comments: String,
	/// Functional role
	role: Option<i64>,
	name: String,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	location: Option<i64>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	serial: String,
	weight: Option<f64>,
	facility_id: Option<String>,
	tenant: Option<i64>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Height in rack units
	u_height: i64,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	site: i64,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	custom_fields: serde_json::value::Value,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Starting unit for rack
	starting_unit: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayTemplate {
	url: String,
	id: i64,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceBayList {
	count: i64,
	previous: Option<String>,
	results: Vec<DeviceBay>,
	next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPlatformRequest {
	slug: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerOutletList {
	previous: Option<String>,
	results: Vec<PowerOutlet>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterGroupRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rack {
	last_updated: Option<String>,
	url: String,
	tags: Vec<NestedTag>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Height in rack units
	u_height: i64,
	serial: String,
	weight: Option<f64>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	status: serde_json::value::Value,
	description: String,
	facility_id: Option<String>,
	display: String,
	custom_fields: serde_json::value::Value,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	device_count: i64,
	weight_unit: Option<serde_json::value::Value>,
	name: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	created: Option<String>,
	id: i64,
	width: serde_json::value::Value,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Starting unit for rack
	starting_unit: i64,
	outer_unit: Option<serde_json::value::Value>,
	powerfeed_count: i64,
	comments: String,
	r#type: Option<serde_json::value::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleBayTemplateList {
	next: Option<String>,
	results: Vec<ModuleBayTemplate>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerFeedRequest {
	comments: String,
	rack: Option<i64>,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	power_panel: i64,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	name: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	amperage: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	custom_fields: serde_json::value::Value,
	voltage: i64,
	description: String,
	tenant: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectChange {
	id: i64,
	request_id: String,
	url: String,
	display: String,
	changed_object_type: String,
	changed_object_id: i64,
	action: serde_json::value::Value,
	time: String,
	user_name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelGroup {
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	description: String,
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
	tunnel_count: i64,
	display: String,
	id: i64,
	url: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Group {
	name: String,
	user_count: i64,
	url: String,
	id: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNTermination {
	id: i64,
	url: String,
	display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTokenList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerFeedList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<PowerFeed>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsolePortList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<ConsolePort>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitCircuitTermination {
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	description: String,
	display: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	id: i64,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedClusterTypeRequest {
	custom_fields: serde_json::value::Value,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleBayTemplateRequest {
	device_type: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsoleServerPortTemplateList {
	previous: Option<String>,
	count: i64,
	results: Vec<ConsoleServerPortTemplate>,
	next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCluster {
	display: String,
	url: String,
	name: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVLANList {
	results: Vec<VLAN>,
	previous: Option<String>,
	next: Option<String>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTokenRequest {
	last_used: Option<String>,
	user: i64,
	expires: Option<String>,
	key: String,
	description: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIKEPolicy {
	url: String,
	name: String,
	id: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVMInterfaceRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitList {
	next: Option<String>,
	results: Vec<Circuit>,
	previous: Option<String>,
	count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInterfaceRequest {
	wwn: Option<String>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	untagged_vlan: Option<i64>,
	speed: Option<i64>,
	module: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	tagged_vlans: Vec<i64>,
	vdcs: Vec<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
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
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	tx_power: Option<i64>,
	parent: Option<i64>,
	mtu: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	mac_address: Option<String>,
	custom_fields: serde_json::value::Value,
	enabled: bool,
	name: String,
	bridge: Option<i64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	lag: Option<i64>,
	wireless_lans: Vec<i64>,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	vrf: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	device: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tenant {
	vlan_count: i64,
	circuit_count: i64,
	vrf_count: i64,
	rack_count: i64,
	slug: String,
	name: String,
	description: String,
	id: i64,
	display: String,
	comments: String,
	created: Option<String>,
	device_count: i64,
	ipaddress_count: i64,
	tags: Vec<NestedTag>,
	prefix_count: i64,
	url: String,
	custom_fields: serde_json::value::Value,
	site_count: i64,
	virtualmachine_count: i64,
	cluster_count: i64,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDataSourceRequest {
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	comments: String,
	r#type: String,
	name: String,
	description: String,
	source_url: String,
	enabled: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualChassis {
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	name: String,
	description: String,
	member_count: i64,
	comments: String,
	domain: String,
	tags: Vec<NestedTag>,
	url: String,
	id: i64,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualChassisRequest {
	comments: String,
	name: String,
	domain: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceTypeList {
	next: Option<String>,
	previous: Option<String>,
	count: i64,
	results: Vec<DeviceType>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackReservationList {
	results: Vec<RackReservation>,
	previous: Option<String>,
	count: i64,
	next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPort {
	_occupied: bool,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints_reachable: bool,
	cable_end: String,
	link_peers: Vec<String>,
	/// Physical label
	label: String,
	id: i64,
	display: String,
	description: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	url: String,
	r#type: serde_json::value::Value,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	speed: Option<serde_json::value::Value>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedFHRPGroupRequest {
	comments: String,
	auth_key: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	group_id: i64,
	custom_fields: serde_json::value::Value,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableJournalEntryRequest {
	assigned_object_type: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	comments: String,
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	created_by: Option<i64>,
	assigned_object_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTypeRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	slug: String,
	description: String,
	color: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	/// Identifier to reference when renaming installed components
	position: String,
	url: String,
	/// Physical label
	label: String,
	id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecProfileRequest {
	ike_policy: i64,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: serde_json::value::Value,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	comments: String,
	ipsec_policy: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPanel {
	id: i64,
	display: String,
	name: String,
	url: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableSiteGroupRequest {
	name: String,
	slug: String,
	custom_fields: serde_json::value::Value,
	parent: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContact {
	url: String,
	display: String,
	name: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderAccount {
	name: String,
	display: String,
	url: String,
	account: String,
	id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualChassisList {
	previous: Option<String>,
	next: Option<String>,
	count: i64,
	results: Vec<VirtualChassis>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableBookmarkRequest {
	user: i64,
	object_id: i64,
	object_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedLocation {
	slug: String,
	_depth: i64,
	display: String,
	id: i64,
	url: String,
	name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVMInterfaceRequest {
	vrf: Option<i64>,
	mac_address: Option<String>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	bridge: Option<i64>,
	parent: Option<i64>,
	virtual_machine: i64,
	mtu: Option<i64>,
	description: String,
	enabled: bool,
	untagged_vlan: Option<i64>,
	tagged_vlans: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Prefix {
	tags: Vec<NestedTag>,
	custom_fields: serde_json::value::Value,
	status: serde_json::value::Value,
	created: Option<String>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	description: String,
	children: i64,
	_depth: i64,
	last_updated: Option<String>,
	display: String,
	id: i64,
	comments: String,
	prefix: String,
	url: String,
	family: serde_json::value::Value,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantGroup {
	name: String,
	_depth: i64,
	slug: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	display: String,
	url: String,
	custom_fields: serde_json::value::Value,
	description: String,
	last_updated: Option<String>,
	id: i64,
	tenant_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProposal {
	created: Option<String>,
	display: String,
	authentication_algorithm: serde_json::value::Value,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	url: String,
	id: i64,
	encryption_algorithm: serde_json::value::Value,
	description: String,
	name: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JournalEntry {
	created_by: Option<i64>,
	url: String,
	comments: String,
	id: i64,
	created: Option<String>,
	tags: Vec<NestedTag>,
	assigned_object_type: String,
	display: String,
	custom_fields: serde_json::value::Value,
	assigned_object_id: i64,
	last_updated: Option<String>,
	kind: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelGroup {
	display: String,
	id: i64,
	name: String,
	slug: String,
	url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCircuitTypeRequest {
	custom_fields: serde_json::value::Value,
	tags: Vec<NestedTagRequest>,
	color: String,
	slug: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceBayRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Physical label
	label: String,
	device: i64,
	custom_fields: serde_json::value::Value,
	description: String,
	installed_device: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCableTerminationList {
	previous: Option<String>,
	count: i64,
	next: Option<String>,
	results: Vec<CableTermination>,
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
	comments: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	custom_fields: serde_json::value::Value,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableASNRangeRequest {
	slug: String,
	rir: i64,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	start: i64,
	description: String,
	end: i64,
	name: String,
	custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRoleRequest {
	slug: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldChoiceSet {
	display: String,
	name: String,
	choices_count: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	last_updated: Option<String>,
	id: i64,
	url: String,
	extra_choices: Option<Vec<Vec<String>>>,
	description: String,
	base_choices: serde_json::value::Value,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventRule {
	name: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	custom_fields: serde_json::value::Value,
	last_updated: Option<String>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	id: i64,
	content_types: Vec<String>,
	action_type: serde_json::value::Value,
	display: String,
	description: String,
	tags: Vec<NestedTag>,
	enabled: bool,
	action_object_type: String,
	action_object: serde_json::value::Value,
	/// Triggers when a matching object is updated.
	type_update: bool,
	created: Option<String>,
	action_object_id: Option<i64>,
	/// Triggers when a matching object is created.
	type_create: bool,
	url: String,
}


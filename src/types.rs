use serde_qs;
use reqwest::Url;
use crate::util::ThanixClient;
/// Adds support for custom fields and tags.
pub struct PatchedRoleRequest {
	name: String,
	custom_fields: String,
	weight: i64,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturer {
	name: String,
	slug: String,
	url: Url,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedVLANGroupRequest {
	name: String,
	scope_type: Option<String>,
	scope_id: Option<i64>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	description: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

pub struct DashboardRequest {
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnel {
	url: Url,
	name: String,
	id: i64,
	display: String,
}

pub struct PaginatedConsoleServerPortList {
	results: Vec<ConsoleServerPort>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedAggregateList {
	next: Option<Url>,
	results: Vec<Aggregate>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct JournalEntryRequest {
	tags: Vec<NestedTagRequest>,
	assigned_object_id: i64,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	custom_fields: String,
	assigned_object_type: String,
	created_by: Option<i64>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct VMInterfaceRequest {
	name: String,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	enabled: bool,
	tagged_vlans: Vec<i64>,
	mtu: Option<i64>,
	mac_address: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelTerminationRequest {
	tags: Vec<NestedTagRequest>,
	tunnel: i64,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_id: Option<i64>,
	custom_fields: String,
	termination_type: String,
	outside_ip: Option<i64>,
}

/// Representation of a prefix which does not exist in the database.
pub struct AvailablePrefix {
	family: i64,
	prefix: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsoleServerPortTemplateRequest {
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
	module_type: Option<i64>,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatform {
	display: String,
	url: Url,
	slug: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct DataSource {
	description: String,
	source_url: String,
	enabled: bool,
	file_count: i64,
	url: Url,
	id: i64,
	comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	display: String,
	last_updated: Option<String>,
	status: String,
	created: Option<String>,
	r#type: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct EventRule {
	tags: Vec<NestedTag>,
	action_object_type: String,
	display: String,
	created: Option<String>,
	custom_fields: String,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	action_object_id: Option<i64>,
	id: i64,
	description: String,
	last_updated: Option<String>,
	enabled: bool,
	content_types: Vec<String>,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	name: String,
	url: Url,
	action_type: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	action_object: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRequest {
	name: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCluster {
	id: i64,
	name: String,
	url: Url,
	display: String,
}

pub struct PaginatedObjectPermissionList {
	results: Vec<ObjectPermission>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRole {
	slug: String,
	id: i64,
	name: String,
	display: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedCircuitTypeRequest {
	custom_fields: String,
	name: String,
	slug: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProfileRequest {
	ipsec_policy: i64,
	tags: Vec<NestedTagRequest>,
	ike_policy: i64,
	custom_fields: String,
	name: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	description: String,
	comments: String,
}

/// Representation of an ASN which does not exist in the database.
pub struct AvailableASN {
	description: String,
	asn: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplate {
	last_updated: Option<String>,
	/// Physical label
	label: String,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	display: String,
	description: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderNetworkRequest {
	provider: i64,
	name: String,
	description: String,
	comments: String,
	service_id: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFrontPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	description: String,
	color: String,
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
	rear_port: i64,
	module: Option<i64>,
	custom_fields: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct BookmarkRequest {
	object_type: String,
	object_id: i64,
}

/// Adds support for custom fields and tags.
pub struct Cable {
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
	last_updated: Option<String>,
	created: Option<String>,
	tags: Vec<NestedTag>,
	b_terminations: Vec<GenericObject>,
	url: Url,
	a_terminations: Vec<GenericObject>,
	id: i64,
	description: String,
	comments: String,
	status: String,
	custom_fields: String,
	color: String,
	length_unit: Option<String>,
	display: String,
	label: String,
}

pub struct PaginatedIPRangeList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<IPRange>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRole {
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	inventoryitem_count: i64,
	description: String,
	custom_fields: String,
	id: i64,
	created: Option<String>,
	color: String,
	name: String,
	display: String,
	slug: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerOutletTemplateRequest {
	power_port: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
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
	/// Physical label
	label: String,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProvider {
	url: Url,
	/// Full name of the provider
	name: String,
	slug: String,
	id: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModule {
	id: i64,
	serial: String,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleType {
	description: String,
	comments: String,
	last_updated: Option<String>,
	display: String,
	url: Url,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: String,
	id: i64,
	model: String,
	weight_unit: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedFHRPGroup {
	group_id: i64,
	id: i64,
	url: Url,
	display: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
}

pub struct PaginatedSiteGroupList {
	previous: Option<Url>,
	results: Vec<SiteGroup>,
	next: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInventoryItemTemplateRequest {
	device_type: i64,
	component_type: Option<String>,
	role: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_id: Option<i64>,
	manufacturer: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRIRRequest {
	slug: String,
	name: String,
}

pub struct PaginatedModuleBayTemplateList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ModuleBayTemplate>,
}

pub struct PaginatedVLANList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<VLAN>,
}

/// Adds support for custom fields and tags.
pub struct WebhookRequest {
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	name: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	tags: Vec<NestedTagRequest>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	custom_fields: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	description: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSite {
	/// Full name of the site
	name: String,
	slug: String,
	url: Url,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PrefixRequest {
	custom_fields: String,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	prefix: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct UserRequest {
	date_joined: String,
	groups: Vec<i64>,
	password: String,
	last_name: String,
	email: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	first_name: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
}

/// Adds support for custom fields and tags.
pub struct IPSecProposalRequest {
	custom_fields: String,
	name: String,
	description: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	comments: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProfileRequest {
	name: String,
	ike_policy: i64,
	custom_fields: String,
	description: String,
	ipsec_policy: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
}

/// Adds support for custom fields and tags.
pub struct PowerPort {
	description: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_reachable: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	_occupied: bool,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	last_updated: Option<String>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	display: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	created: Option<String>,
	connected_endpoints_type: String,
	id: i64,
	tags: Vec<NestedTag>,
	r#type: Option<String>,
	name: String,
	url: Url,
	/// Physical label
	label: String,
	cable_end: String,
	link_peers: Vec<String>,
}

pub struct PaginatedModuleBayList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ModuleBay>,
}

/// Adds support for custom fields and tags.
pub struct ASN {
	provider_count: i64,
	display: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	custom_fields: String,
	comments: String,
	created: Option<String>,
	url: Url,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	description: String,
	id: i64,
	site_count: i64,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRoleRequest {
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePlatformRequest {
	name: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: String,
	config_template: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplate {
	url: Url,
	display: String,
	/// Physical label
	label: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	r#type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
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

pub struct PaginatedWirelessLinkList {
	count: i64,
	next: Option<Url>,
	results: Vec<WirelessLink>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitTerminationRequest {
	custom_fields: String,
	provider_network: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	circuit: i64,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// ID of the local cross-connect
	xconnect_id: String,
	site: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedRegionList {
	results: Vec<Region>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNTerminationRequest {
	assigned_object_type: String,
	custom_fields: String,
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
	l2vpn: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsolePortTemplateRequest {
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
	/// Physical label
	label: String,
	module_type: Option<i64>,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContextRequest {
	comments: String,
	custom_fields: String,
	description: String,
	name: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	tags: Vec<NestedTagRequest>,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRequest {
	custom_fields: String,
	tenant: Option<i64>,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPortRequest {
	device: i64,
	description: String,
	module: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
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
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableBookmarkRequest {
	object_type: String,
	object_id: i64,
	user: i64,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetworkRequest {
	description: String,
	comments: String,
	custom_fields: String,
	name: String,
	service_id: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachineRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenRequest {
	description: String,
	last_used: Option<String>,
	expires: Option<String>,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRequest {
	component_type: Option<String>,
	serial: String,
	custom_fields: String,
	name: String,
	component_id: Option<i64>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	parent: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// Physical label
	label: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceTypeRequest {
	model: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLink {
	id: i64,
	display: String,
	ssid: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicy {
	custom_fields: String,
	id: i64,
	display: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	name: String,
	last_updated: Option<String>,
	proposals: Vec<i64>,
	pfs_group: String,
	comments: String,
	description: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvision {
	url: Url,
	id: i64,
	last_used: String,
	created: String,
	expires: Option<String>,
	display: String,
	description: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	key: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterface {
	url: Url,
	display: String,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Full name of the provider
	name: String,
	asns: Vec<i64>,
	comments: String,
	accounts: Vec<i64>,
	custom_fields: String,
}

pub struct PaginatedDeviceBayTemplateList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceBayTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermission {
	id: i64,
	url: Url,
	enabled: bool,
	users: Vec<i64>,
	name: String,
	description: String,
	display: String,
	groups: Vec<i64>,
	object_types: Vec<String>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
}

pub struct PaginatedRIRList {
	count: i64,
	next: Option<Url>,
	results: Vec<RIR>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicyRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	proposals: Vec<i64>,
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
	comments: String,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicy {
	id: i64,
	version: String,
	created: Option<String>,
	url: Url,
	proposals: Vec<i64>,
	tags: Vec<NestedTag>,
	name: String,
	mode: String,
	display: String,
	preshared_key: String,
	description: String,
	comments: String,
	custom_fields: String,
	last_updated: Option<String>,
}

pub struct PaginatedTunnelGroupList {
	results: Vec<TunnelGroup>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	device: Option<i64>,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	role: Option<i64>,
	disk: Option<i64>,
	custom_fields: String,
	primary_ip4: Option<i64>,
	description: String,
	cluster: Option<i64>,
	primary_ip6: Option<i64>,
	name: String,
	site: Option<i64>,
	memory: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	vcpus: Option<f64>,
	platform: Option<i64>,
}

pub struct PaginatedConfigContextList {
	results: Vec<ConfigContext>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

pub struct PaginatedSavedFilterList {
	count: i64,
	previous: Option<Url>,
	results: Vec<SavedFilter>,
	next: Option<Url>,
}

pub struct PaginatedProviderAccountList {
	results: Vec<ProviderAccount>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PowerPanelRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	comments: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEPolicyRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	description: String,
	custom_fields: String,
	preshared_key: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	proposals: Vec<i64>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
}

pub struct Dashboard {
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetworkRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Contact {
	id: i64,
	last_updated: Option<String>,
	phone: String,
	address: String,
	description: String,
	url: Url,
	created: Option<String>,
	email: String,
	title: String,
	tags: Vec<NestedTag>,
	name: String,
	comments: String,
	display: String,
	link: Url,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANRequest {
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
}

pub struct PaginatedCustomLinkList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<CustomLink>,
}

/// Adds support for custom fields and tags.
pub struct WritableTenantRequest {
	name: String,
	group: Option<i64>,
	description: String,
	comments: String,
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleBayRequest {
	name: String,
	device: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Physical label
	label: String,
	installed_module: i64,
	/// Identifier to reference when renaming installed components
	position: String,
}

/// Adds support for custom fields and tags.
pub struct RearPort {
	url: Url,
	display: String,
	name: String,
	tags: Vec<NestedTag>,
	cable_end: String,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	_occupied: bool,
	color: String,
	created: Option<String>,
	description: String,
	r#type: String,
	/// Number of front ports which may be mapped
	positions: i64,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	custom_fields: String,
	id: i64,
	link_peers: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct ManufacturerRequest {
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedCircuitTypeList {
	results: Vec<CircuitType>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplate {
	url: Url,
	description: String,
	/// Path to remote file (relative to data source root)
	data_path: String,
	id: i64,
	/// Jinja2 template code.
	template_code: String,
	display: String,
	data_synced: Option<String>,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDiskRequest {
	size: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	virtual_machine: i64,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleTypeRequest {
	description: String,
	comments: String,
	custom_fields: String,
	model: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	manufacturer: i64,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableRouteTargetRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	comments: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct CableTermination {
	display: String,
	url: Url,
	termination_id: i64,
	last_updated: Option<String>,
	cable: i64,
	termination_type: String,
	created: Option<String>,
	id: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
}

/// Adds support for custom fields and tags.
pub struct WritableConsolePortRequest {
	name: String,
	/// Physical label
	label: String,
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
	description: String,
	module: Option<i64>,
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilterRequest {
	content_types: Vec<String>,
	enabled: bool,
	slug: String,
	shared: bool,
	weight: i64,
	description: String,
	name: String,
	user: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableDeviceBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: i64,
	/// Physical label
	label: String,
	description: String,
}

pub struct PaginatedFrontPortTemplateList {
	results: Vec<FrontPortTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEPolicyRequest {
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	description: String,
	proposals: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	preshared_key: String,
	name: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	custom_fields: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPAddressRequest {
	vrf: Option<i64>,
	assigned_object_type: Option<String>,
	description: String,
	comments: String,
	assigned_object_id: Option<i64>,
	address: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRequest {
	name: String,
}

/// Representation of an IP address which does not exist in the database.
pub struct AvailableIP {
	description: String,
	family: i64,
	address: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProposal {
	tags: Vec<NestedTag>,
	id: i64,
	name: String,
	authentication_algorithm: String,
	url: Url,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	created: Option<String>,
	comments: String,
	description: String,
	last_updated: Option<String>,
	encryption_algorithm: String,
	display: String,
	custom_fields: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLAN {
	id: i64,
	url: Url,
	display: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterType {
	slug: String,
	id: i64,
	display: String,
	url: Url,
	name: String,
}

pub struct PaginatedIPSecProposalList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<IPSecProposal>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroupRequest {
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct GroupRequest {
	name: String,
}

pub struct PaginatedL2VPNTerminationList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<L2VPNTermination>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTenantRequest {
	comments: String,
	group: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct Webhook {
	tags: Vec<NestedTag>,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	custom_fields: String,
	description: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	id: i64,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	last_updated: Option<String>,
	created: Option<String>,
	url: Url,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	name: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	display: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedRIRRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	description: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleRequest {
	serial: String,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	module_type: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	module_bay: i64,
	device: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroup {
	name: String,
	id: i64,
	slug: String,
	display: String,
	url: Url,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableWirelessLANGroupRequest {
	description: String,
	custom_fields: String,
	name: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegionRequest {
	name: String,
	slug: String,
}

pub struct PaginatedModuleTypeList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ModuleType>,
}

/// Adds support for custom fields and tags.
pub struct L2VPNRequest {
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
	export_targets: Vec<i64>,
	slug: String,
	description: String,
	import_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	identifier: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocationRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableFrontPortRequest {
	module: Option<i64>,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	tags: Vec<NestedTagRequest>,
	color: String,
	rear_port: i64,
	description: String,
	device: i64,
	custom_fields: String,
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
}

/// Adds support for custom fields and tags.
pub struct VRF {
	name: String,
	export_targets: Vec<i64>,
	url: Url,
	ipaddress_count: i64,
	import_targets: Vec<i64>,
	created: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	comments: String,
	display: String,
	custom_fields: String,
	id: i64,
	prefix_count: i64,
	last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableUserRequest {
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	last_name: String,
	first_name: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	date_joined: String,
	email: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	password: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct User {
	email: String,
	id: i64,
	display: String,
	url: Url,
	last_name: String,
	first_name: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	date_joined: String,
	groups: Vec<i64>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

pub struct PaginatedContactGroupList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ContactGroup>,
}

/// Adds support for custom fields and tags.
pub struct CircuitRequest {
	tags: Vec<NestedTagRequest>,
	/// Unique circuit ID
	cid: String,
	/// Committed rate
	commit_rate: Option<i64>,
	description: String,
	custom_fields: String,
	install_date: Option<String>,
	termination_date: Option<String>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableAggregateRequest {
	date_added: Option<String>,
	description: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	custom_fields: String,
	prefix: String,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicyRequest {
	proposals: Vec<i64>,
	name: String,
	description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	preshared_key: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct CircuitTypeRequest {
	color: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceRoleRequest {
	name: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	description: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	config_template: Option<i64>,
	custom_fields: String,
}

pub struct PaginatedTenantGroupList {
	count: i64,
	results: Vec<TenantGroup>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct Provider {
	id: i64,
	custom_fields: String,
	/// Full name of the provider
	name: String,
	accounts: Vec<i64>,
	description: String,
	asns: Vec<i64>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	url: Url,
	display: String,
	last_updated: Option<String>,
	circuit_count: i64,
	comments: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceBayRequest {
	device: i64,
	name: String,
	/// Physical label
	label: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	installed_device: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfileRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	name: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterTypeRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterRequest {
	name: String,
}

pub struct PaginatedProviderList {
	count: i64,
	next: Option<Url>,
	results: Vec<Provider>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedRackRoleRequest {
	name: String,
	color: String,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPanelRequest {
	name: String,
	custom_fields: String,
	location: Option<i64>,
	site: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfile {
	id: i64,
	url: Url,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct VLANGroup {
	tags: Vec<NestedTag>,
	display: String,
	custom_fields: String,
	description: String,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	scope_type: Option<String>,
	slug: String,
	vlan_count: i64,
	name: String,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	scope_id: Option<i64>,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	utilization: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContext {
	custom_fields: String,
	memory: Option<i64>,
	disk: Option<i64>,
	last_updated: Option<String>,
	display: String,
	created: Option<String>,
	interface_count: i64,
	virtual_disk_count: i64,
	id: i64,
	comments: String,
	status: String,
	tags: Vec<NestedTag>,
	url: Url,
	vcpus: Option<f64>,
	description: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterGroupRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	description: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualMachineWithConfigContextRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	platform: Option<i64>,
	description: String,
	cluster: Option<i64>,
	vcpus: Option<f64>,
	site: Option<i64>,
	memory: Option<i64>,
	primary_ip4: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	primary_ip6: Option<i64>,
	disk: Option<i64>,
	tenant: Option<i64>,
	comments: String,
	name: String,
	device: Option<i64>,
	role: Option<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableContactGroupRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	name: String,
	parent: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
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
pub struct PatchedWritableRearPortRequest {
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	color: String,
	/// Physical label
	label: String,
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
	tags: Vec<NestedTagRequest>,
	/// Number of front ports which may be mapped
	positions: i64,
	device: i64,
	name: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortRequest {
	name: String,
	cable: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContext {
	display: String,
	last_updated: Option<String>,
	weight: i64,
	roles: Vec<i64>,
	locations: Vec<i64>,
	url: Url,
	name: String,
	platforms: Vec<i64>,
	is_active: bool,
	regions: Vec<i64>,
	device_types: Vec<i64>,
	cluster_groups: Vec<i64>,
	description: String,
	tenants: Vec<i64>,
	sites: Vec<i64>,
	clusters: Vec<i64>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	tenant_groups: Vec<i64>,
	created: Option<String>,
	id: i64,
	site_groups: Vec<i64>,
	cluster_types: Vec<i64>,
	tags: Vec<String>,
	data_synced: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRoleRequest {
	name: String,
	slug: String,
}

pub struct PaginatedVirtualChassisList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<VirtualChassis>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderAccountRequest {
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	account: String,
	name: String,
	provider: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerOutletRequest {
	description: String,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	comments: String,
	custom_fields: String,
	tunnel_id: Option<i64>,
}

pub struct PaginatedTunnelList {
	count: i64,
	next: Option<Url>,
	results: Vec<Tunnel>,
	previous: Option<Url>,
}

pub struct PaginatedRearPortTemplateList {
	count: i64,
	results: Vec<RearPortTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ClusterType {
	cluster_count: i64,
	tags: Vec<NestedTag>,
	name: String,
	id: i64,
	display: String,
	slug: String,
	url: Url,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPN {
	custom_fields: String,
	description: String,
	import_targets: Vec<i64>,
	identifier: Option<i64>,
	id: i64,
	name: String,
	display: String,
	slug: String,
	tags: Vec<NestedTag>,
	comments: String,
	last_updated: Option<String>,
	export_targets: Vec<i64>,
	created: Option<String>,
	r#type: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct ContactRoleRequest {
	description: String,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

pub struct PaginatedPowerFeedList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<PowerFeed>,
	count: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableLocationRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	name: String,
	tenant: Option<i64>,
	slug: String,
	custom_fields: String,
	parent: Option<i64>,
	description: String,
	site: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct IPAddress {
	status: String,
	role: String,
	description: String,
	comments: String,
	created: Option<String>,
	nat_outside: Vec<NestedIPAddress>,
	custom_fields: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	assigned_object_type: Option<String>,
	id: i64,
	url: Url,
	address: String,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	family: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVMInterfaceRequest {
	name: String,
	parent: Option<i64>,
	mtu: Option<i64>,
	vrf: Option<i64>,
	mac_address: Option<String>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	bridge: Option<i64>,
	untagged_vlan: Option<i64>,
	virtual_machine: i64,
	description: String,
	enabled: bool,
	tagged_vlans: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLinkRequest {
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	interface_b: i64,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	custom_fields: String,
	interface_a: i64,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	ssid: String,
	auth_psk: String,
	comments: String,
	description: String,
}

pub struct PaginatedVLANGroupList {
	results: Vec<VLANGroup>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRF {
	url: Url,
	id: i64,
	name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	display: String,
}

pub struct PaginatedL2VPNList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<L2VPN>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplate {
	description: String,
	color: String,
	created: Option<String>,
	last_updated: Option<String>,
	r#type: String,
	display: String,
	id: i64,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	positions: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableExportTemplateRequest {
	/// Remote data source
	data_source: Option<i64>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	content_types: Vec<String>,
	/// Download file as attachment
	as_attachment: bool,
	/// Extension to append to the rendered filename
	file_extension: String,
	name: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachmentRequest {
	image_height: i64,
	object_id: i64,
	image: String,
	image_width: i64,
	content_type: String,
	name: String,
}

pub struct PaginatedVirtualDiskList {
	results: Vec<VirtualDisk>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLANRequest {
	group: Option<i64>,
	description: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	vlan: Option<i64>,
	ssid: String,
	tenant: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	auth_psk: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ipaddresses: Vec<i64>,
	comments: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
}

/// Adds support for custom fields and tags.
pub struct Module {
	created: Option<String>,
	description: String,
	id: i64,
	serial: String,
	tags: Vec<NestedTag>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	status: String,
	custom_fields: String,
	url: Url,
	comments: String,
	display: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ConsolePort {
	tags: Vec<NestedTag>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	speed: Option<String>,
	name: String,
	/// Physical label
	label: String,
	url: Url,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	custom_fields: String,
	description: String,
	_occupied: bool,
	cable_end: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	created: Option<String>,
	connected_endpoints_reachable: bool,
	display: String,
	r#type: String,
	id: i64,
	link_peers: Vec<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactAssignmentRequest {
	content_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	object_id: i64,
	role: i64,
	contact: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroup {
	id: i64,
	slug: String,
	url: Url,
	name: String,
	_depth: i64,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelGroup {
	id: i64,
	url: Url,
	slug: String,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceRoleRequest {
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
	color: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDevice {
	id: i64,
	display: String,
	name: Option<String>,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct CircuitTerminationRequest {
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItem {
	tags: Vec<NestedTag>,
	component_id: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	parent: Option<i64>,
	custom_fields: String,
	_depth: i64,
	description: String,
	created: Option<String>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	last_updated: Option<String>,
	id: i64,
	component_type: Option<String>,
	url: Url,
	/// Physical label
	label: String,
	serial: String,
	name: String,
	/// This item was automatically discovered
	discovered: bool,
	display: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableLocationRequest {
	tenant: Option<i64>,
	description: String,
	site: i64,
	slug: String,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceBayRequest {
	name: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	installed_device: Option<i64>,
	device: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableSiteGroupRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	parent: Option<i64>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContext {
	status: String,
	module_bay_count: i64,
	face: String,
	power_outlet_count: i64,
	description: String,
	custom_fields: String,
	device_bay_count: i64,
	last_updated: Option<String>,
	comments: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	position: Option<f64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	name: Option<String>,
	display: String,
	console_server_port_count: i64,
	rear_port_count: i64,
	inventory_item_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	tags: Vec<NestedTag>,
	power_port_count: i64,
	id: i64,
	url: Url,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	airflow: String,
	created: Option<String>,
	vc_position: Option<i64>,
	console_port_count: i64,
	interface_count: i64,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	front_port_count: i64,
}

pub struct PaginatedVMInterfaceList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<VMInterface>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterface {
	cable: Option<i64>,
	_occupied: bool,
	display: String,
	id: i64,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceRequest {
	virtual_machine: Option<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	name: String,
	comments: String,
	device: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableClusterRequest {
	group: Option<i64>,
	r#type: i64,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	site: Option<i64>,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	name: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ClusterRequest {
	description: String,
	comments: String,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRoleRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedFHRPGroupRequest {
	name: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	auth_key: String,
	description: String,
	group_id: i64,
	comments: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
}

pub struct PaginatedIPSecProfileList {
	count: i64,
	next: Option<Url>,
	results: Vec<IPSecProfile>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRole {
	name: String,
	url: Url,
	id: i64,
	display: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldChoiceSetRequest {
	extra_choices: Option<Vec<Vec<String>>>,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	description: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccount {
	id: i64,
	display: String,
	name: String,
	url: Url,
	account: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassisRequest {
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Location {
	description: String,
	name: String,
	device_count: i64,
	url: Url,
	status: String,
	custom_fields: String,
	last_updated: Option<String>,
	display: String,
	_depth: i64,
	created: Option<String>,
	rack_count: i64,
	slug: String,
	tags: Vec<NestedTag>,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct ModuleBay {
	description: String,
	id: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	display: String,
	url: Url,
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ProviderRequest {
	slug: String,
	accounts: Vec<i64>,
	description: String,
	asns: Vec<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Full name of the provider
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleBayRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTermination {
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	url: Url,
	/// ID of the local cross-connect
	xconnect_id: String,
	id: i64,
	/// Physical circuit speed
	port_speed: Option<i64>,
	display: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableEventRuleRequest {
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	tags: Vec<NestedTagRequest>,
	/// Triggers when a matching object is created.
	type_create: bool,
	content_types: Vec<String>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	description: String,
	enabled: bool,
	action_object_id: Option<i64>,
	name: String,
	action_object_type: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleBayRequest {
	custom_fields: String,
	device: i64,
	name: String,
	installed_module: i64,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTag {
	display: String,
	id: i64,
	name: String,
	slug: String,
	color: String,
	url: Url,
}

pub struct Job {
	scheduled: Option<String>,
	job_id: String,
	id: i64,
	created: String,
	display: String,
	name: String,
	object_type: String,
	url: Url,
	completed: Option<String>,
	status: String,
	/// Recurrence interval (in minutes)
	interval: Option<i64>,
	started: Option<String>,
	error: String,
	object_id: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroupRequest {
	name: String,
	slug: String,
}

pub struct PaginatedTenantList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Tenant>,
}

pub struct PaginatedVirtualMachineWithConfigContextList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<VirtualMachineWithConfigContext>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecPolicyRequest {
	tags: Vec<NestedTagRequest>,
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
	custom_fields: String,
	comments: String,
	name: String,
	description: String,
	proposals: Vec<i64>,
}

pub struct PaginatedUserList {
	next: Option<Url>,
	results: Vec<User>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct IKEProposal {
	url: Url,
	authentication_algorithm: String,
	comments: String,
	id: i64,
	group: String,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	tags: Vec<NestedTag>,
	description: String,
	custom_fields: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	display: String,
	encryption_algorithm: String,
	authentication_method: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRearPortRequest {
	device: i64,
	/// Physical label
	label: String,
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
	color: String,
	/// Number of front ports which may be mapped
	positions: i64,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

pub struct PaginatedBookmarkList {
	count: i64,
	results: Vec<Bookmark>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePrefixRequest {
	comments: String,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	prefix: String,
	/// The primary function of this prefix
	role: Option<i64>,
	tenant: Option<i64>,
	vlan: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	custom_fields: String,
	site: Option<i64>,
	vrf: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
}

/// Adds support for custom fields and tags.
pub struct PowerFeed {
	url: Url,
	phase: String,
	connected_endpoints: Vec<String>,
	r#type: String,
	_occupied: bool,
	voltage: i64,
	connected_endpoints_type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	display: String,
	supply: String,
	link_peers: Vec<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	id: i64,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	name: String,
	cable_end: String,
	comments: String,
	custom_fields: String,
	last_updated: Option<String>,
	status: String,
	connected_endpoints_reachable: bool,
	tags: Vec<NestedTag>,
	description: String,
	amperage: i64,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDeviceContextRequest {
	custom_fields: String,
	name: String,
	description: String,
	primary_ip4: Option<i64>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	primary_ip6: Option<i64>,
	tags: Vec<NestedTagRequest>,
	device: Option<i64>,
	tenant: Option<i64>,
	comments: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccount {
	custom_fields: String,
	display: String,
	account: String,
	description: String,
	id: i64,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	comments: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ConsolePortRequest {
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
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
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableClusterRequest {
	r#type: i64,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	name: String,
	tenant: Option<i64>,
	site: Option<i64>,
	tags: Vec<NestedTagRequest>,
	group: Option<i64>,
	comments: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableBookmarkRequest {
	user: i64,
	object_id: i64,
	object_type: String,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceTemplateRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	description: String,
	ports: Vec<i64>,
	comments: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplate {
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	id: i64,
	feed_leg: Option<String>,
	r#type: Option<String>,
	/// Physical label
	label: String,
	display: String,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

pub struct PaginatedProviderNetworkList {
	results: Vec<ProviderNetwork>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocation {
	name: String,
	slug: String,
	display: String,
	url: Url,
	_depth: i64,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplateRequest {
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
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
	r#type: Option<String>,
	description: String,
	/// Physical label
	label: String,
}

pub struct PaginatedTagList {
	next: Option<Url>,
	results: Vec<Tag>,
	previous: Option<Url>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetwork {
	display: String,
	url: Url,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPRangeRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	end_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	start_address: String,
	/// The primary function of this range
	role: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	vrf: Option<i64>,
	comments: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct TenantRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLink {
	comments: String,
	url: Url,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	ssid: String,
	display: String,
	id: i64,
	auth_type: String,
	auth_cipher: String,
	auth_psk: String,
	status: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableRegionRequest {
	description: String,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsolePortTemplateRequest {
	device_type: Option<i64>,
	module_type: Option<i64>,
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
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct WritableInterfaceRequest {
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	wwn: Option<String>,
	module: Option<i64>,
	mac_address: Option<String>,
	name: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	bridge: Option<i64>,
	custom_fields: String,
	untagged_vlan: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	description: String,
	speed: Option<i64>,
	mtu: Option<i64>,
	device: i64,
	tagged_vlans: Vec<i64>,
	vdcs: Vec<i64>,
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
	tx_power: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	parent: Option<i64>,
	lag: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	wireless_lans: Vec<i64>,
	/// Physical label
	label: String,
	vrf: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PowerOutlet {
	cable_end: String,
	r#type: Option<String>,
	feed_leg: Option<String>,
	display: String,
	link_peers: Vec<String>,
	custom_fields: String,
	created: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	name: String,
	/// Physical label
	label: String,
	description: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	id: i64,
	connected_endpoints_reachable: bool,
	url: Url,
	tags: Vec<NestedTag>,
	_occupied: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableExportTemplateRequest {
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Download file as attachment
	as_attachment: bool,
	name: String,
	content_types: Vec<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDeviceContextRequest {
	primary_ip6: Option<i64>,
	custom_fields: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	primary_ip4: Option<i64>,
	device: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	comments: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	description: String,
}

pub struct PaginatedContactAssignmentList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ContactAssignment>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WirelessLANRequest {
	auth_psk: String,
	description: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	custom_fields: String,
	ssid: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelRequest {
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	group: Option<i64>,
	tenant: Option<i64>,
	custom_fields: String,
	description: String,
	name: String,
	ipsec_profile: Option<i64>,
	tags: Vec<NestedTagRequest>,
	tunnel_id: Option<i64>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct RearPortRequest {
	color: String,
	custom_fields: String,
	description: String,
	name: String,
	/// Number of front ports which may be mapped
	positions: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
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
pub struct VMInterface {
	enabled: bool,
	custom_fields: String,
	url: Url,
	last_updated: Option<String>,
	mac_address: Option<String>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	count_ipaddresses: i64,
	display: String,
	count_fhrp_groups: i64,
	id: i64,
	mtu: Option<i64>,
	mode: String,
	tagged_vlans: Vec<i64>,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccountRequest {
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	account: String,
	description: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPortRequest {
	description: String,
	name: String,
	/// Physical label
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplateRequest {
	description: String,
	/// Physical label
	label: String,
	positions: i64,
	color: String,
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
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProposalRequest {
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
	custom_fields: String,
	name: String,
	description: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTerminationRequest {
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderNetworkRequest {
	provider: i64,
	name: String,
	description: String,
	comments: String,
	service_id: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableTenantGroupRequest {
	custom_fields: String,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
}

pub struct PaginatedCableList {
	results: Vec<Cable>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplate {
	id: i64,
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	created: Option<String>,
	url: Url,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableCableRequest {
	a_terminations: Vec<GenericObjectRequest>,
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
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	description: String,
	label: String,
	color: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	length: Option<f64>,
	custom_fields: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritablePrefixRequest {
	vlan: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	tenant: Option<i64>,
	vrf: Option<i64>,
	comments: String,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	site: Option<i64>,
	prefix: String,
	/// The primary function of this prefix
	role: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackReservationRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	user: i64,
	tenant: Option<i64>,
	rack: i64,
	comments: String,
	units: Vec<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct RackRequest {
	/// Height in rack units
	u_height: i64,
	/// Starting unit for rack
	starting_unit: i64,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	serial: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	custom_fields: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	comments: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: Option<String>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	name: String,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	tags: Vec<NestedTagRequest>,
	facility_id: Option<String>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: Option<String>,
	weight: Option<f64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct Circuit {
	install_date: Option<String>,
	id: i64,
	created: Option<String>,
	tags: Vec<NestedTag>,
	/// Unique circuit ID
	cid: String,
	description: String,
	url: Url,
	comments: String,
	display: String,
	status: String,
	last_updated: Option<String>,
	termination_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroup {
	last_updated: Option<String>,
	_depth: i64,
	display: String,
	name: String,
	custom_fields: String,
	description: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	wirelesslan_count: i64,
	url: Url,
	slug: String,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableDeviceBayTemplateRequest {
	description: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

pub struct PaginatedConsolePortTemplateList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<ConsolePortTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplateRequest {
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
	rear_port_position: i64,
	color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatformRequest {
	slug: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedGroupRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	description: String,
	tenant: Option<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableAggregateRequest {
	comments: String,
	tenant: Option<i64>,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	description: String,
	prefix: String,
	date_added: Option<String>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct RoleRequest {
	tags: Vec<NestedTagRequest>,
	weight: i64,
	slug: String,
	description: String,
	name: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterTypeRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterfaceRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct AggregateRequest {
	date_added: Option<String>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	prefix: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecPolicyRequest {
	name: String,
	comments: String,
	proposals: Vec<i64>,
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
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

pub struct PaginatedInterfaceList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Interface>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInventoryItemTemplateRequest {
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	role: Option<i64>,
	parent: Option<i64>,
	/// Physical label
	label: String,
	component_type: Option<String>,
	description: String,
	manufacturer: Option<i64>,
	device_type: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroupRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleTypeRequest {
	model: String,
}

pub struct PaginatedRouteTargetList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<RouteTarget>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSetRequest {
	description: String,
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplate {
	tags: Vec<NestedTag>,
	display: String,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	protocol: String,
	id: i64,
	custom_fields: String,
	comments: String,
	url: Url,
	name: String,
	ports: Vec<i64>,
}

pub struct PaginatedImageAttachmentList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<ImageAttachment>,
}

pub struct PaginatedVirtualDeviceContextList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<VirtualDeviceContext>,
}

/// Adds support for custom fields and tags.
pub struct SiteRequest {
	asns: Vec<i64>,
	slug: String,
	comments: String,
	time_zone: Option<String>,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// Full name of the site
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// If different from the physical address
	shipping_address: String,
	/// Local facility ID or description
	facility: String,
	/// Physical location of the building
	physical_address: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableFrontPortTemplateRequest {
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	color: String,
	rear_port_position: i64,
	description: String,
	module_type: Option<i64>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct RouteTarget {
	tags: Vec<NestedTag>,
	display: String,
	description: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	comments: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct WirelessLinkRequest {
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	auth_psk: String,
	description: String,
	comments: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableModuleBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	device_type: i64,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVLANRequest {
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// VLAN group (optional)
	group: Option<i64>,
	comments: String,
	description: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	tags: Vec<NestedTagRequest>,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	custom_fields: String,
	tenant: Option<i64>,
	name: String,
	/// The primary function of this VLAN
	role: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTermination {
	url: Url,
	id: i64,
	display: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableRegionRequest {
	description: String,
	custom_fields: String,
	slug: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContextRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	name: String,
	description: String,
	memory: Option<i64>,
	custom_fields: String,
	vcpus: Option<f64>,
	disk: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignmentRequest {
	interface_id: i64,
	priority: i64,
	interface_type: String,
}

pub struct PaginatedConsoleServerPortTemplateList {
	count: i64,
	next: Option<Url>,
	results: Vec<ConsoleServerPortTemplate>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct CableTerminationRequest {
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	cable: i64,
	termination_type: String,
	termination_id: i64,
}

pub struct PaginatedIKEProposalList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<IKEProposal>,
}

pub struct PaginatedWebhookList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Webhook>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRIR {
	id: i64,
	name: String,
	slug: String,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDataSourceRequest {
	description: String,
	comments: String,
	name: String,
	r#type: String,
	enabled: bool,
	source_url: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

pub struct PaginatedFHRPGroupList {
	results: Vec<FHRPGroup>,
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct L2VPNTerminationRequest {
	assigned_object_type: String,
	assigned_object_id: i64,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedServiceList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Service>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteRequest {
	/// Full name of the site
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableUserRequest {
	last_name: String,
	first_name: String,
	password: String,
	email: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	date_joined: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLANRequest {
	group: Option<i64>,
	tenant: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	auth_psk: String,
	tags: Vec<NestedTagRequest>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	custom_fields: String,
	ssid: String,
	description: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	vlan: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct CableRequest {
	a_terminations: Vec<GenericObjectRequest>,
	length: Option<f64>,
	b_terminations: Vec<GenericObjectRequest>,
	custom_fields: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	color: String,
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
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: Option<String>,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct DataFile {
	url: Url,
	size: i64,
	last_updated: String,
	id: i64,
	display: String,
	/// File path relative to the data source's root
	path: String,
	/// SHA256 hash of the file data
	hash: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConfigContextRequest {
	tenant_groups: Vec<i64>,
	cluster_types: Vec<i64>,
	regions: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
	device_types: Vec<i64>,
	name: String,
	locations: Vec<i64>,
	site_groups: Vec<i64>,
	platforms: Vec<i64>,
	cluster_groups: Vec<i64>,
	tags: Vec<String>,
	clusters: Vec<i64>,
	weight: i64,
	is_active: bool,
	roles: Vec<i64>,
	tenants: Vec<i64>,
	sites: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfileRequest {
	name: String,
}

pub struct PaginatedVRFList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<VRF>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRoleRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRole {
	slug: String,
	url: Url,
	id: i64,
	display: String,
	name: String,
}

pub struct PaginatedDataFileList {
	previous: Option<Url>,
	results: Vec<DataFile>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedCableTerminationRequest {
	cable: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	termination_type: String,
	termination_id: i64,
}

pub struct PaginatedManufacturerList {
	previous: Option<Url>,
	results: Vec<Manufacturer>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableSiteRequest {
	/// Local facility ID or description
	facility: String,
	custom_fields: String,
	tenant: Option<i64>,
	region: Option<i64>,
	tags: Vec<NestedTagRequest>,
	group: Option<i64>,
	comments: String,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	asns: Vec<i64>,
	slug: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Full name of the site
	name: String,
	/// If different from the physical address
	shipping_address: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	time_zone: Option<String>,
	/// Physical location of the building
	physical_address: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualChassisRequest {
	description: String,
	name: String,
	domain: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	master: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PowerPanel {
	comments: String,
	id: i64,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	display: String,
	custom_fields: String,
	powerfeed_count: i64,
	last_updated: Option<String>,
	description: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct Site {
	virtualmachine_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	display: String,
	/// Full name of the site
	name: String,
	status: String,
	/// Local facility ID or description
	facility: String,
	vlan_count: i64,
	/// If different from the physical address
	shipping_address: String,
	time_zone: Option<String>,
	/// Physical location of the building
	physical_address: String,
	created: Option<String>,
	id: i64,
	circuit_count: i64,
	slug: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	last_updated: Option<String>,
	custom_fields: String,
	description: String,
	url: Url,
	comments: String,
	tags: Vec<NestedTag>,
	device_count: i64,
	prefix_count: i64,
	asns: Vec<i64>,
	rack_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPort {
	_occupied: bool,
	name: String,
	id: i64,
	display: String,
	cable: Option<i64>,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Tag {
	display: String,
	url: Url,
	id: i64,
	description: String,
	object_types: Vec<String>,
	name: String,
	slug: String,
	color: String,
	tagged_items: i64,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassisRequest {
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	domain: String,
	description: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleRequest {
	comments: String,
	device: i64,
	module_bay: i64,
	serial: String,
	module_type: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPanelRequest {
	name: String,
	site: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	location: Option<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContextRequest {
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	vc_position: Option<i64>,
	position: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
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
	description: String,
	comments: String,
	name: Option<String>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
}

pub struct PaginatedConsolePortList {
	results: Vec<ConsolePort>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicy {
	url: Url,
	display: String,
	id: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroup {
	url: Url,
	display: String,
	name: String,
	id: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVRFRequest {
	export_targets: Vec<i64>,
	import_targets: Vec<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	tenant: Option<i64>,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
}

/// Adds support for custom fields and tags.
pub struct Tunnel {
	encapsulation: String,
	status: String,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	custom_fields: String,
	description: String,
	id: i64,
	tags: Vec<NestedTag>,
	name: String,
	tunnel_id: Option<i64>,
	url: Url,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPRangeRequest {
	vrf: Option<i64>,
	description: String,
	end_address: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	start_address: String,
	/// The primary function of this range
	role: Option<i64>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataFile {
	url: Url,
	id: i64,
	/// File path relative to the data source's root
	path: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct Device {
	rear_port_count: i64,
	inventory_item_count: i64,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	module_bay_count: i64,
	id: i64,
	status: String,
	name: Option<String>,
	created: Option<String>,
	comments: String,
	tags: Vec<NestedTag>,
	display: String,
	last_updated: Option<String>,
	console_server_port_count: i64,
	position: Option<f64>,
	device_bay_count: i64,
	airflow: String,
	power_outlet_count: i64,
	front_port_count: i64,
	vc_position: Option<i64>,
	custom_fields: String,
	url: Url,
	description: String,
	interface_count: i64,
	power_port_count: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	console_port_count: i64,
	face: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantRequest {
	slug: String,
	name: String,
}

pub struct PaginatedPowerPanelList {
	next: Option<Url>,
	count: i64,
	results: Vec<PowerPanel>,
	previous: Option<Url>,
}

pub struct PatchedDashboardRequest {
}

/// Adds support for custom fields and tags.
pub struct TunnelTermination {
	custom_fields: String,
	url: Url,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	role: String,
	display: String,
	termination_id: Option<i64>,
	tags: Vec<NestedTag>,
	termination_type: String,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelRequest {
	comments: String,
	ipsec_profile: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	group: Option<i64>,
	tunnel_id: Option<i64>,
	name: String,
	tenant: Option<i64>,
	custom_fields: String,
	description: String,
}

pub struct PaginatedJournalEntryList {
	results: Vec<JournalEntry>,
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedManufacturerRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	custom_fields: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEProposalRequest {
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
	comments: String,
	custom_fields: String,
	name: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct RIR {
	url: Url,
	name: String,
	description: String,
	aggregate_count: i64,
	slug: String,
	id: i64,
	tags: Vec<NestedTag>,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
}

pub struct PaginatedDataSourceList {
	previous: Option<Url>,
	count: i64,
	results: Vec<DataSource>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplate {
	rear_port_position: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	display: String,
	id: i64,
	r#type: String,
	color: String,
	/// Physical label
	label: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegion {
	slug: String,
	display: String,
	name: String,
	id: i64,
	url: Url,
	_depth: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecPolicy {
	name: String,
	display: String,
	url: Url,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDiskRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
	virtual_machine: i64,
	size: i64,
}

pub struct PaginatedRackReservationList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<RackReservation>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplate {
	/// Physical label
	label: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	poe_type: Option<String>,
	display: String,
	mgmt_only: bool,
	last_updated: Option<String>,
	enabled: bool,
	rf_role: Option<String>,
	r#type: String,
	created: Option<String>,
	poe_mode: Option<String>,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitRequest {
	custom_fields: String,
	provider: i64,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	r#type: i64,
	install_date: Option<String>,
	provider_account: Option<i64>,
	termination_date: Option<String>,
	/// Unique circuit ID
	cid: String,
	/// Committed rate
	commit_rate: Option<i64>,
	tenant: Option<i64>,
	description: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceRequest {
	cable: Option<i64>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUser {
	url: Url,
	id: i64,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCable {
	label: String,
	display: String,
	url: Url,
	id: i64,
}

pub struct PaginatedPlatformList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<Platform>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSourceRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelRequest {
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroup {
	url: Url,
	display: String,
	description: String,
	tags: Vec<NestedTag>,
	_depth: i64,
	id: i64,
	name: String,
	last_updated: Option<String>,
	slug: String,
	custom_fields: String,
	created: Option<String>,
	site_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTerminationRequest {
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedContactRoleRequest {
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
}

pub struct PaginatedRearPortList {
	previous: Option<Url>,
	results: Vec<RearPort>,
	count: i64,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerOutletTemplateRequest {
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
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
	power_port: Option<i64>,
	module_type: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct Service {
	id: i64,
	display: String,
	url: Url,
	comments: String,
	last_updated: Option<String>,
	ports: Vec<i64>,
	ipaddresses: Vec<i64>,
	description: String,
	custom_fields: String,
	name: String,
	created: Option<String>,
	protocol: String,
	tags: Vec<NestedTag>,
}

pub struct PaginatedInventoryItemRoleList {
	count: i64,
	results: Vec<InventoryItemRole>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Token {
	created: String,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	last_used: Option<String>,
	id: i64,
	expires: Option<String>,
	url: Url,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturerRequest {
	slug: String,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroupRequest {
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceTypeRequest {
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: Option<String>,
	/// Discrete part number (optional)
	part_number: String,
	u_height: f64,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: Option<String>,
	description: String,
	front_image: String,
	tags: Vec<NestedTagRequest>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	comments: String,
	rear_image: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	slug: String,
	model: String,
	weight: Option<f64>,
	custom_fields: String,
}

pub struct PaginatedFrontPortList {
	results: Vec<FrontPort>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedPowerOutletTemplateList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<PowerOutletTemplate>,
}

/// Adds support for custom fields and tags.
pub struct ASNRange {
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	name: String,
	created: Option<String>,
	end: i64,
	start: i64,
	asn_count: i64,
	last_updated: Option<String>,
	url: Url,
	display: String,
	id: i64,
	slug: String,
}

pub struct PaginatedCircuitTerminationList {
	results: Vec<CircuitTermination>,
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableConsoleServerPortRequest {
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	device: i64,
	/// Physical label
	label: String,
	name: String,
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
}

/// Adds support for custom fields and tags.
pub struct WritablePlatformRequest {
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
	slug: String,
	config_template: Option<i64>,
}

pub struct PaginatedRackRoleList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<RackRole>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedTagRequest {
	slug: String,
	color: String,
	description: String,
	name: String,
	object_types: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceWithConfigContextRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	comments: String,
	location: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	virtual_chassis: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	primary_ip6: Option<i64>,
	rack: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	tenant: Option<i64>,
	name: Option<String>,
	oob_ip: Option<i64>,
	primary_ip4: Option<i64>,
	platform: Option<i64>,
	cluster: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	device_type: i64,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	config_template: Option<i64>,
	custom_fields: String,
	position: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// The function this device serves
	role: i64,
	vc_position: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	description: String,
	site: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableSiteGroupRequest {
	slug: String,
	parent: Option<i64>,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldRequest {
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	description: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	object_type: String,
	/// Internal field name
	name: String,
	choice_set: Option<i64>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
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
	content_types: Vec<String>,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSource {
	name: String,
	id: i64,
	url: Url,
	display: String,
}

pub struct ContentType {
	id: i64,
	url: Url,
	display: String,
	app_label: String,
	model: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplateRequest {
	name: String,
}

pub struct PaginatedDeviceTypeList {
	next: Option<Url>,
	results: Vec<DeviceType>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct VirtualDisk {
	created: Option<String>,
	name: String,
	description: String,
	last_updated: Option<String>,
	id: i64,
	url: Url,
	size: i64,
	custom_fields: String,
	tags: Vec<NestedTag>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct WritableConfigTemplateRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Jinja2 template code.
	template_code: String,
	name: String,
	/// Remote data source
	data_source: Option<i64>,
	data_file: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedCustomLinkRequest {
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	weight: i64,
	/// Jinja2 template code for link URL
	link_url: String,
	content_types: Vec<String>,
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
	enabled: bool,
	/// Force link to open in a new window
	new_window: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitType {
	url: Url,
	name: String,
	display: String,
	id: i64,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRole {
	id: i64,
	url: Url,
	name: String,
	slug: String,
	display: String,
}

pub struct PaginatedIPSecPolicyList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<IPSecPolicy>,
}

pub struct PaginatedObjectChangeList {
	next: Option<Url>,
	results: Vec<ObjectChange>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct DeviceType {
	rear_image: Url,
	power_port_template_count: i64,
	front_port_template_count: i64,
	last_updated: Option<String>,
	url: Url,
	airflow: Option<String>,
	slug: String,
	console_server_port_template_count: i64,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	device_count: i64,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	id: i64,
	device_bay_template_count: i64,
	module_bay_template_count: i64,
	weight: Option<f64>,
	power_outlet_template_count: i64,
	interface_template_count: i64,
	custom_fields: String,
	comments: String,
	inventory_item_template_count: i64,
	model: String,
	console_port_template_count: i64,
	/// Discrete part number (optional)
	part_number: String,
	weight_unit: Option<String>,
	rear_port_template_count: i64,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	subdevice_role: Option<String>,
	u_height: f64,
	front_image: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetwork {
	name: String,
	custom_fields: String,
	display: String,
	description: String,
	url: Url,
	id: i64,
	created: Option<String>,
	service_id: String,
	last_updated: Option<String>,
	comments: String,
	tags: Vec<NestedTag>,
}

pub struct PaginatedFHRPGroupAssignmentList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<FHRPGroupAssignment>,
	count: i64,
}

pub struct PaginatedPowerOutletList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<PowerOutlet>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRouteTargetRequest {
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
	custom_fields: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PowerFeedRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	amperage: i64,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	comments: String,
	voltage: i64,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableFrontPortTemplateRequest {
	/// Physical label
	label: String,
	description: String,
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
	device_type: Option<i64>,
	module_type: Option<i64>,
	color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	rear_port_position: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplate {
	/// Physical label
	label: String,
	url: Url,
	display: String,
	r#type: String,
	created: Option<String>,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPAddressRequest {
	custom_fields: String,
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
	assigned_object_id: Option<i64>,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	tenant: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	vrf: Option<i64>,
	assigned_object_type: Option<String>,
	description: String,
	comments: String,
	address: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLinkRequest {
	auth_psk: String,
	ssid: String,
	interface_b: i64,
	interface_a: i64,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
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
	tenant: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroup {
	_depth: i64,
	id: i64,
	name: String,
	display: String,
	url: Url,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct IPRangeRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	end_address: String,
	description: String,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	start_address: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TagRequest {
	object_types: Vec<String>,
	slug: String,
	color: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct IKEProposalRequest {
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
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
	name: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	cid: String,
}

/// Adds support for custom fields and tags.
pub struct WritableContactAssignmentRequest {
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	role: i64,
	contact: i64,
	content_type: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContext {
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	interface_count: i64,
	url: Url,
	comments: String,
	created: Option<String>,
	status: String,
	display: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	id: i64,
	name: String,
	custom_fields: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRack {
	url: Url,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	group: i64,
	interface_id: i64,
	interface_type: String,
	priority: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableRackRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	/// Functional role
	role: Option<i64>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Height in rack units
	u_height: i64,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	weight: Option<f64>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	site: i64,
	description: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	serial: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	facility_id: Option<String>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
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
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	name: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	location: Option<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroup {
	name: String,
	_depth: i64,
	last_updated: Option<String>,
	slug: String,
	tags: Vec<NestedTag>,
	display: String,
	description: String,
	tenant_count: i64,
	url: Url,
	created: Option<String>,
	id: i64,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterTypeRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
	slug: String,
}

pub struct PaginatedInventoryItemTemplateList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<InventoryItemTemplate>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableContactGroupRequest {
	name: String,
	parent: Option<i64>,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModuleRequest {
	serial: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderRequest {
	slug: String,
	/// Full name of the provider
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerOutletRequest {
	description: String,
	power_port: Option<i64>,
	device: i64,
	/// Physical label
	label: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	name: String,
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldRequest {
	content_types: Vec<String>,
	/// Internal field name
	name: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	object_type: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
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
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	description: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Group {
	name: String,
	display: String,
	id: i64,
	user_count: i64,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddressRequest {
	address: String,
}

pub struct PaginatedDeviceBayList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<DeviceBay>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplate {
	description: String,
	id: i64,
	parent: Option<i64>,
	url: Url,
	component_id: Option<i64>,
	display: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	last_updated: Option<String>,
	component_type: Option<String>,
	_depth: i64,
	created: Option<String>,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSet {
	display: String,
	name: String,
	url: Url,
	id: i64,
	choices_count: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableTenantGroupRequest {
	custom_fields: String,
	slug: String,
	parent: Option<i64>,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedDeviceRoleList {
	count: i64,
	previous: Option<Url>,
	results: Vec<DeviceRole>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLink {
	url: Url,
	weight: i64,
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
	/// Force link to open in a new window
	new_window: bool,
	content_types: Vec<String>,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	/// Jinja2 template code for link text
	link_text: String,
	/// Jinja2 template code for link URL
	link_url: String,
	enabled: bool,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct IPRange {
	created: Option<String>,
	status: String,
	size: i64,
	url: Url,
	custom_fields: String,
	family: String,
	tags: Vec<NestedTag>,
	id: i64,
	display: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	comments: String,
	last_updated: Option<String>,
	start_address: String,
	end_address: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
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
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

pub struct PaginatedEventRuleList {
	count: i64,
	previous: Option<Url>,
	results: Vec<EventRule>,
	next: Option<Url>,
}

pub struct PaginatedInterfaceTemplateList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<InterfaceTemplate>,
}

/// Representation of a VLAN which does not exist in the database.
pub struct AvailableVLAN {
	vid: i64,
}

pub struct PaginatedRoleList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Role>,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerFeedRequest {
	power_panel: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	amperage: i64,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	voltage: i64,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	tenant: Option<i64>,
	name: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	rack: Option<i64>,
	comments: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	description: String,
}

pub struct ObjectChange {
	display: String,
	changed_object_type: String,
	action: String,
	url: Url,
	request_id: String,
	id: i64,
	changed_object_id: i64,
	user_name: String,
	time: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceTypeRequest {
	default_platform: Option<i64>,
	tags: Vec<NestedTagRequest>,
	front_image: String,
	u_height: f64,
	model: String,
	weight: Option<f64>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	/// Discrete part number (optional)
	part_number: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	slug: String,
	rear_image: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	description: String,
	comments: String,
	custom_fields: String,
	manufacturer: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
}

/// Adds support for custom fields and tags.
pub struct VLAN {
	id: i64,
	custom_fields: String,
	url: Url,
	created: Option<String>,
	prefix_count: i64,
	tags: Vec<NestedTag>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	status: String,
	name: String,
	comments: String,
	description: String,
	last_updated: Option<String>,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct RIRRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanelRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVLANRequest {
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	tenant: Option<i64>,
	/// The primary function of this VLAN
	role: Option<i64>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// VLAN group (optional)
	group: Option<i64>,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBay {
	url: Url,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceTypeRequest {
	model: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	/// Discrete part number (optional)
	part_number: String,
	description: String,
	u_height: f64,
	slug: String,
	comments: String,
	weight: Option<f64>,
	manufacturer: i64,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	rear_image: String,
	default_platform: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	front_image: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceType {
	url: Url,
	display: String,
	model: String,
	slug: String,
	id: i64,
}

pub struct PaginatedDeviceWithConfigContextList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<DeviceWithConfigContext>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderRequest {
	comments: String,
	accounts: Vec<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Full name of the provider
	name: String,
	asns: Vec<i64>,
	custom_fields: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	description: String,
	module_type: Option<i64>,
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

pub struct PaginatedJobList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<Job>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLinkRequest {
	content_types: Vec<String>,
	/// Jinja2 template code for link text
	link_text: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	/// Jinja2 template code for link URL
	link_url: String,
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
	enabled: bool,
	name: String,
	/// Force link to open in a new window
	new_window: bool,
	weight: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableModuleBayTemplateRequest {
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleBayRequest {
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroup {
	name: String,
	display: String,
	slug: String,
	description: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	id: i64,
	custom_fields: String,
	last_updated: Option<String>,
	url: Url,
	tunnel_count: i64,
}

/// Adds support for custom fields and tags.
pub struct Cluster {
	status: String,
	id: i64,
	created: Option<String>,
	display: String,
	device_count: i64,
	description: String,
	tags: Vec<NestedTag>,
	comments: String,
	url: Url,
	virtualmachine_count: i64,
	custom_fields: String,
	last_updated: Option<String>,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachment {
	image_width: i64,
	id: i64,
	object_id: i64,
	last_updated: Option<String>,
	created: Option<String>,
	content_type: String,
	image_height: i64,
	url: Url,
	image: Url,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCableRequest {
	b_terminations: Vec<GenericObjectRequest>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	color: String,
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
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	length: Option<f64>,
	a_terminations: Vec<GenericObjectRequest>,
	description: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableConsoleServerPortRequest {
	custom_fields: String,
	description: String,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	device: i64,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableTokenRequest {
	user: i64,
	last_used: Option<String>,
	expires: Option<String>,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroup {
	id: i64,
	display: String,
	url: Url,
	name: String,
	slug: String,
	_depth: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct RegionRequest {
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignmentRequest {
	content_type: String,
	tags: Vec<NestedTagRequest>,
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	custom_fields: String,
}

pub struct PaginatedIPAddressList {
	previous: Option<Url>,
	results: Vec<IPAddress>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct L2VPNTermination {
	created: Option<String>,
	tags: Vec<NestedTag>,
	assigned_object_type: String,
	last_updated: Option<String>,
	url: Url,
	assigned_object_id: i64,
	display: String,
	custom_fields: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDataSourceRequest {
	name: String,
	enabled: bool,
	description: String,
	r#type: String,
	comments: String,
	source_url: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPortRequest {
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	custom_fields: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	module: Option<i64>,
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
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNRequest {
	slug: String,
	export_targets: Vec<i64>,
	description: String,
	comments: String,
	name: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	import_targets: Vec<i64>,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassis {
	id: i64,
	url: Url,
	name: String,
	display: String,
}

pub struct PaginatedPowerPortList {
	next: Option<Url>,
	count: i64,
	results: Vec<PowerPort>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct InterfaceRequest {
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	mtu: Option<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	mac_address: Option<String>,
	tx_power: Option<i64>,
	wireless_lans: Vec<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	tagged_vlans: Vec<i64>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
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
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	speed: Option<i64>,
	description: String,
	wwn: Option<String>,
	vdcs: Vec<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	enabled: bool,
	custom_fields: String,
}

/// Used by device component serializers.
pub struct ComponentNestedModule {
	id: i64,
	device: i64,
	url: Url,
	display: String,
}

pub struct PaginatedRackList {
	results: Vec<Rack>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualChassisRequest {
	name: String,
	master: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	domain: String,
	comments: String,
	custom_fields: String,
}

pub struct PaginatedTokenList {
	results: Vec<Token>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNRequest {
	slug: String,
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
}

/// Adds support for custom fields and tags.
pub struct ClusterGroup {
	last_updated: Option<String>,
	display: String,
	custom_fields: String,
	name: String,
	cluster_count: i64,
	id: i64,
	tags: Vec<NestedTag>,
	url: Url,
	description: String,
	slug: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct Aggregate {
	display: String,
	description: String,
	custom_fields: String,
	url: Url,
	date_added: Option<String>,
	prefix: String,
	comments: String,
	tags: Vec<NestedTag>,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	family: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenant {
	url: Url,
	display: String,
	name: String,
	id: i64,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLinkRequest {
	ssid: String,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRangeRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	rir: i64,
	end: i64,
	start: i64,
	tenant: Option<i64>,
	custom_fields: String,
	slug: String,
	description: String,
}

pub struct PaginatedCircuitList {
	count: i64,
	results: Vec<Circuit>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RackReservationRequest {
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	units: Vec<i64>,
	comments: String,
}

pub struct PaginatedGroupList {
	next: Option<Url>,
	count: i64,
	results: Vec<Group>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct CircuitTermination {
	cable_end: String,
	tags: Vec<NestedTag>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	link_peers: Vec<String>,
	display: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	created: Option<String>,
	/// ID of the local cross-connect
	xconnect_id: String,
	last_updated: Option<String>,
	_occupied: bool,
	/// Physical circuit speed
	port_speed: Option<i64>,
	id: i64,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	url: Url,
	description: String,
}

pub struct PaginatedTunnelTerminationList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<TunnelTermination>,
}

/// Adds support for custom fields and tags.
pub struct Rack {
	id: i64,
	url: Url,
	description: String,
	width: String,
	weight: Option<f64>,
	powerfeed_count: i64,
	weight_unit: Option<String>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	comments: String,
	r#type: Option<String>,
	name: String,
	/// Height in rack units
	u_height: i64,
	display: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	tags: Vec<NestedTag>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	created: Option<String>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	outer_unit: Option<String>,
	/// Starting unit for rack
	starting_unit: i64,
	facility_id: Option<String>,
	custom_fields: String,
	serial: String,
	device_count: i64,
	status: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ModuleTypeRequest {
	comments: String,
	custom_fields: String,
	weight: Option<f64>,
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuit {
	url: Url,
	/// Unique circuit ID
	cid: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableJournalEntryRequest {
	custom_fields: String,
	created_by: Option<i64>,
	assigned_object_id: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignment {
	created: Option<String>,
	id: i64,
	url: Url,
	display: String,
	interface_id: i64,
	interface_type: String,
	priority: i64,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRFRequest {
	name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupRequest {
	tags: Vec<NestedTagRequest>,
	group_id: i64,
	custom_fields: String,
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
	auth_key: String,
	comments: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct FrontPortRequest {
	description: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	color: String,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct VirtualDiskRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	size: i64,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct LocationRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNRequest {
	identifier: Option<i64>,
	custom_fields: String,
	name: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	export_targets: Vec<i64>,
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
	import_targets: Vec<i64>,
}

pub struct PaginatedASNRangeList {
	previous: Option<Url>,
	count: i64,
	results: Vec<ASNRange>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilter {
	shared: bool,
	display: String,
	url: Url,
	content_types: Vec<String>,
	enabled: bool,
	description: String,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	slug: String,
	weight: i64,
	user: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct DeviceBayRequest {
	custom_fields: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableTokenRequest {
	last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	expires: Option<String>,
	key: String,
	user: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplateRequest {
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ContactRole {
	slug: String,
	name: String,
	display: String,
	description: String,
	id: i64,
	last_updated: Option<String>,
	url: Url,
	created: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

pub struct PaginatedContactRoleList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ContactRole>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplate {
	id: i64,
	url: Url,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct EventRuleRequest {
	description: String,
	custom_fields: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_id: Option<i64>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	content_types: Vec<String>,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	action_object_type: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	enabled: bool,
	tags: Vec<NestedTagRequest>,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Prefix {
	display: String,
	status: String,
	comments: String,
	custom_fields: String,
	children: i64,
	last_updated: Option<String>,
	prefix: String,
	url: Url,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	description: String,
	family: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	_depth: i64,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

/// Adds support for custom fields and tags.
pub struct DeviceRole {
	display: String,
	slug: String,
	url: Url,
	virtualmachine_count: i64,
	name: String,
	id: i64,
	last_updated: Option<String>,
	color: String,
	tags: Vec<NestedTag>,
	device_count: i64,
	created: Option<String>,
	custom_fields: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTagRequest {
	slug: String,
	name: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplateRequest {
	ports: Vec<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct Interface {
	wwn: Option<String>,
	tags: Vec<NestedTag>,
	duplex: Option<String>,
	cable_end: String,
	last_updated: Option<String>,
	_occupied: bool,
	connected_endpoints_type: String,
	vdcs: Vec<i64>,
	mac_address: Option<String>,
	description: String,
	poe_mode: String,
	tagged_vlans: Vec<i64>,
	wireless_lans: Vec<i64>,
	r#type: String,
	rf_role: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	link_peers: Vec<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints: Vec<String>,
	count_fhrp_groups: i64,
	mtu: Option<i64>,
	enabled: bool,
	speed: Option<i64>,
	/// Physical label
	label: String,
	display: String,
	url: Url,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	mode: String,
	rf_channel: String,
	tx_power: Option<i64>,
	id: i64,
	created: Option<String>,
	count_ipaddresses: i64,
	connected_endpoints_reachable: bool,
	name: String,
	custom_fields: String,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	poe_type: String,
}

pub struct PaginatedConfigTemplateList {
	next: Option<Url>,
	results: Vec<ConfigTemplate>,
	previous: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermissionRequest {
	name: String,
	description: String,
	users: Vec<i64>,
	enabled: bool,
	object_types: Vec<String>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct Tenant {
	created: Option<String>,
	slug: String,
	prefix_count: i64,
	virtualmachine_count: i64,
	name: String,
	ipaddress_count: i64,
	url: Url,
	rack_count: i64,
	custom_fields: String,
	cluster_count: i64,
	description: String,
	tags: Vec<NestedTag>,
	device_count: i64,
	site_count: i64,
	id: i64,
	circuit_count: i64,
	vlan_count: i64,
	vrf_count: i64,
	comments: String,
	last_updated: Option<String>,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct JournalEntry {
	custom_fields: String,
	assigned_object_type: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	url: Url,
	display: String,
	kind: String,
	comments: String,
	id: i64,
	created: Option<String>,
	assigned_object_id: i64,
	created_by: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNTerminationRequest {
	custom_fields: String,
	assigned_object_type: String,
	l2vpn: i64,
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSet {
	choices_count: String,
	name: String,
	description: String,
	url: Url,
	created: Option<String>,
	id: i64,
	base_choices: String,
	last_updated: Option<String>,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	display: String,
	extra_choices: Option<Vec<Vec<String>>>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedSavedFilterRequest {
	slug: String,
	description: String,
	content_types: Vec<String>,
	user: Option<i64>,
	enabled: bool,
	shared: bool,
	weight: i64,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroupRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	slug: String,
}

pub struct PaginatedExportTemplateList {
	count: i64,
	results: Vec<ExportTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWebhookRequest {
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	description: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	tags: Vec<NestedTagRequest>,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	name: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachine {
	id: i64,
	url: Url,
	display: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelGroupRequest {
	name: String,
	slug: String,
}

pub struct PaginatedClusterGroupList {
	results: Vec<ClusterGroup>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerFeedRequest {
	tags: Vec<NestedTagRequest>,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	description: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	voltage: i64,
	amperage: i64,
	comments: String,
	name: String,
	rack: Option<i64>,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	power_panel: i64,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	tenant: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Bookmark {
	display: String,
	id: i64,
	created: String,
	url: Url,
	object_id: i64,
	object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInterfaceTemplateRequest {
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	description: String,
	bridge: Option<i64>,
	device_type: Option<i64>,
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
	module_type: Option<i64>,
	enabled: bool,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	mgmt_only: bool,
}

/// Adds support for custom fields and tags.
pub struct VLANGroupRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
	scope_type: Option<String>,
	custom_fields: String,
	scope_id: Option<i64>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerPortTemplateRequest {
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
	device_type: Option<i64>,
	module_type: Option<i64>,
	description: String,
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
}

pub struct PaginatedClusterTypeList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ClusterType>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableRearPortTemplateRequest {
	device_type: Option<i64>,
	positions: i64,
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
	/// Physical label
	label: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct WritableSiteRequest {
	/// Full name of the site
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	group: Option<i64>,
	description: String,
	/// If different from the physical address
	shipping_address: String,
	/// Physical location of the building
	physical_address: String,
	asns: Vec<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// Local facility ID or description
	facility: String,
	custom_fields: String,
	comments: String,
	tenant: Option<i64>,
	region: Option<i64>,
	slug: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	time_zone: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitTerminationRequest {
	custom_fields: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	site: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	provider_network: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	circuit: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInventoryItemRequest {
	manufacturer: Option<i64>,
	component_type: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_id: Option<i64>,
	parent: Option<i64>,
	device: i64,
	/// Physical label
	label: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	role: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
	serial: String,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroupRequest {
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroup {
	_depth: i64,
	slug: String,
	id: i64,
	name: String,
	display: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplate {
	description: String,
	id: i64,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Path to remote file (relative to data source root)
	data_path: String,
	created: Option<String>,
	content_types: Vec<String>,
	name: String,
	/// Download file as attachment
	as_attachment: bool,
	last_updated: Option<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	display: String,
	data_synced: Option<String>,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct Manufacturer {
	created: Option<String>,
	url: Url,
	last_updated: Option<String>,
	inventoryitem_count: i64,
	platform_count: i64,
	devicetype_count: i64,
	display: String,
	name: String,
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	description: String,
	id: i64,
}

pub struct PaginatedLocationList {
	previous: Option<Url>,
	count: i64,
	results: Vec<Location>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceBay {
	id: i64,
	custom_fields: String,
	display: String,
	created: Option<String>,
	description: String,
	name: String,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	url: Url,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct PatchedWritableConfigTemplateRequest {
	/// Remote data source
	data_source: Option<i64>,
	description: String,
	data_file: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Jinja2 template code.
	template_code: String,
	name: String,
}

pub struct PaginatedContentTypeList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ContentType>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct VLANRequest {
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	description: String,
	comments: String,
	name: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddress {
	address: String,
	display: String,
	url: Url,
	id: i64,
	family: i64,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignment {
	display: String,
	content_type: String,
	priority: String,
	custom_fields: String,
	id: i64,
	url: Url,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	created: Option<String>,
	object: String,
	object_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldRequest {
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	object_type: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
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
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	choice_set: Option<i64>,
	/// Internal field name
	name: String,
	content_types: Vec<String>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	description: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerOutletRequest {
	description: String,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	custom_fields: String,
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
	device: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactRequest {
	email: String,
	phone: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	link: Url,
	comments: String,
	address: String,
	group: Option<i64>,
	custom_fields: String,
	name: String,
	title: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPort {
	name: String,
	id: i64,
	url: Url,
	description: String,
	/// Physical label
	label: String,
	display: String,
}

pub struct PaginatedWirelessLANList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<WirelessLAN>,
}

pub struct PaginatedIKEPolicyList {
	next: Option<Url>,
	count: i64,
	results: Vec<IKEPolicy>,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerPortTemplateRequest {
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
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
	r#type: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	module_type: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleType {
	id: i64,
	url: Url,
	display: String,
	model: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRackReservationRequest {
	custom_fields: String,
	user: i64,
	rack: i64,
	units: Vec<i64>,
	description: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroup {
	display: String,
	url: Url,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	auth_key: String,
	last_updated: Option<String>,
	ip_addresses: Vec<NestedIPAddress>,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	group_id: i64,
	created: Option<String>,
	name: String,
	id: i64,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableInventoryItemRequest {
	manufacturer: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// Physical label
	label: String,
	role: Option<i64>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	name: String,
	device: i64,
	custom_fields: String,
	serial: String,
	description: String,
	component_type: Option<String>,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	component_id: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomField {
	id: i64,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	display: String,
	/// Internal field name
	name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	last_updated: Option<String>,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	r#type: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	filter_logic: String,
	created: Option<String>,
	url: Url,
	data_type: String,
	ui_visible: String,
	description: String,
	ui_editable: String,
	object_type: String,
	content_types: Vec<String>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRearPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct ASNRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PlatformRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableConsolePortRequest {
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
	/// Physical label
	label: String,
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
	description: String,
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	module: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	component_type: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	description: String,
	parent: Option<i64>,
	component_id: Option<i64>,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModule {
	url: Url,
	id: i64,
	display: String,
}

pub struct PaginatedCableTerminationList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<CableTermination>,
}

/// Adds support for custom fields and tags.
pub struct DataSourceRequest {
	enabled: bool,
	source_url: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	comments: String,
	description: String,
	name: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObjectRequest {
	object_id: i64,
	object_type: String,
}

pub struct PaginatedASNList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<ASN>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRangeRequest {
	description: String,
	end: i64,
	name: String,
	rir: i64,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	start: i64,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfile {
	comments: String,
	name: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	mode: String,
	display: String,
	description: String,
	custom_fields: String,
	created: Option<String>,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct FrontPort {
	/// Physical label
	label: String,
	cable_end: String,
	description: String,
	display: String,
	tags: Vec<NestedTag>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	color: String,
	id: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	link_peers: Vec<String>,
	last_updated: Option<String>,
	created: Option<String>,
	name: String,
	url: Url,
	_occupied: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	custom_fields: String,
	r#type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceWithConfigContextRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	site: i64,
	/// The function this device serves
	role: i64,
	position: Option<f64>,
	tags: Vec<NestedTagRequest>,
	location: Option<i64>,
	rack: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	platform: Option<i64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	description: String,
	config_template: Option<i64>,
	cluster: Option<i64>,
	vc_position: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	tenant: Option<i64>,
	name: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	primary_ip4: Option<i64>,
	oob_ip: Option<i64>,
	virtual_chassis: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	comments: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	primary_ip6: Option<i64>,
	custom_fields: String,
	device_type: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerPortRequest {
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
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
	r#type: Option<String>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct RouteTargetRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct IPAddressRequest {
	assigned_object_type: Option<String>,
	assigned_object_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	description: String,
	address: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	role: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: Option<String>,
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
	description: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: Option<String>,
	/// Physical label
	label: String,
	mgmt_only: bool,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObject {
	object_type: String,
	object_id: i64,
}

pub struct PaginatedSiteList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Site>,
}

pub struct PaginatedClusterList {
	next: Option<Url>,
	count: i64,
	results: Vec<Cluster>,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplateRequest {
	/// Download file as attachment
	as_attachment: bool,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	description: String,
	name: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	content_types: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableContactRequest {
	group: Option<i64>,
	name: String,
	comments: String,
	custom_fields: String,
	description: String,
	link: Url,
	tags: Vec<NestedTagRequest>,
	phone: String,
	title: String,
	address: String,
	email: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplateRequest {
	/// Jinja2 template code.
	template_code: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableWirelessLANGroupRequest {
	custom_fields: String,
	slug: String,
	description: String,
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct CircuitType {
	color: String,
	created: Option<String>,
	circuit_count: i64,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	display: String,
	custom_fields: String,
	last_updated: Option<String>,
	slug: String,
	id: i64,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPN {
	name: String,
	url: Url,
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
	display: String,
	identifier: Option<i64>,
	slug: String,
}

pub struct PaginatedContactList {
	next: Option<Url>,
	count: i64,
	results: Vec<Contact>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSetRequest {
	name: String,
}

pub struct PaginatedPowerPortTemplateList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<PowerPortTemplate>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplate {
	display: String,
	description: String,
	r#type: Option<String>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBayRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConfigContextRequest {
	regions: Vec<i64>,
	cluster_groups: Vec<i64>,
	clusters: Vec<i64>,
	is_active: bool,
	/// Remote data source
	data_source: Option<i64>,
	cluster_types: Vec<i64>,
	weight: i64,
	device_types: Vec<i64>,
	platforms: Vec<i64>,
	roles: Vec<i64>,
	tags: Vec<String>,
	locations: Vec<i64>,
	name: String,
	site_groups: Vec<i64>,
	tenant_groups: Vec<i64>,
	tenants: Vec<i64>,
	description: String,
	sites: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableVMInterfaceRequest {
	vrf: Option<i64>,
	bridge: Option<i64>,
	mtu: Option<i64>,
	parent: Option<i64>,
	tagged_vlans: Vec<i64>,
	enabled: bool,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	untagged_vlan: Option<i64>,
	custom_fields: String,
	mac_address: Option<String>,
	virtual_machine: i64,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanel {
	url: Url,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct ClusterGroupRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRoleRequest {
	slug: String,
	name: String,
}

pub struct PaginatedCustomFieldList {
	next: Option<Url>,
	results: Vec<CustomField>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct RackReservation {
	custom_fields: String,
	description: String,
	url: Url,
	created: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	display: String,
	last_updated: Option<String>,
	comments: String,
	units: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedTunnelGroupRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct RackRole {
	created: Option<String>,
	display: String,
	url: Url,
	color: String,
	last_updated: Option<String>,
	rack_count: i64,
	name: String,
	id: i64,
	tags: Vec<NestedTag>,
	slug: String,
	custom_fields: String,
	description: String,
}

pub struct PaginatedModuleList {
	next: Option<Url>,
	results: Vec<Module>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderAccountRequest {
	name: String,
	provider: i64,
	account: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableFHRPGroupAssignmentRequest {
	group: i64,
	interface_type: String,
	interface_id: i64,
	priority: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplateRequest {
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

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCableRequest {
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableObjectPermissionRequest {
	groups: Vec<i64>,
	users: Vec<i64>,
	description: String,
	name: String,
	enabled: bool,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	object_types: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceRoleRequest {
	custom_fields: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	description: String,
	color: String,
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableObjectPermissionRequest {
	users: Vec<i64>,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	description: String,
	object_types: Vec<String>,
	name: String,
	enabled: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecPolicyRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccountRequest {
	account: String,
	name: String,
}

/// Used by device component serializers.
pub struct ComponentNestedModuleRequest {
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct ContactRequest {
	phone: String,
	address: String,
	tags: Vec<NestedTagRequest>,
	email: String,
	title: String,
	description: String,
	link: Url,
	comments: String,
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleRequest {
	custom_fields: String,
	serial: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
}

/// Adds support for custom fields and tags.
pub struct Platform {
	name: String,
	last_updated: Option<String>,
	slug: String,
	created: Option<String>,
	display: String,
	description: String,
	device_count: i64,
	virtualmachine_count: i64,
	id: i64,
	url: Url,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	enabled: bool,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	device_type: Option<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	module_type: Option<i64>,
	/// Physical label
	label: String,
	mgmt_only: bool,
	description: String,
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
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedImageAttachmentRequest {
	name: String,
	content_type: String,
	image: String,
	image_height: i64,
	object_id: i64,
	image_width: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelTerminationRequest {
	custom_fields: String,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	tunnel: i64,
	termination_type: String,
	outside_ip: Option<i64>,
	termination_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleBay {
	name: String,
	url: Url,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct RackRoleRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	color: String,
	custom_fields: String,
	description: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInterfaceRequest {
	tagged_vlans: Vec<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	module: Option<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
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
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	enabled: bool,
	wwn: Option<String>,
	description: String,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	tx_power: Option<i64>,
	bridge: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	untagged_vlan: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	speed: Option<i64>,
	vrf: Option<i64>,
	parent: Option<i64>,
	mtu: Option<i64>,
	mac_address: Option<String>,
	lag: Option<i64>,
	tags: Vec<NestedTagRequest>,
	device: i64,
	vdcs: Vec<i64>,
	wireless_lans: Vec<i64>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	name: String,
	custom_fields: String,
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
	/// Physical label
	label: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableEventRuleRequest {
	action_object_type: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	enabled: bool,
	custom_fields: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	content_types: Vec<String>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	description: String,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	action_object_id: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPortRequest {
	name: String,
	/// Physical label
	label: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
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

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplate {
	display: String,
	url: Url,
	id: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContact {
	name: String,
	display: String,
	url: Url,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRole {
	name: String,
	id: i64,
	display: String,
	slug: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicyRequest {
	name: String,
}

pub struct PaginatedPrefixList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Prefix>,
}

/// Adds support for custom fields and tags.
pub struct Role {
	slug: String,
	display: String,
	name: String,
	last_updated: Option<String>,
	weight: i64,
	url: Url,
	id: i64,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	prefix_count: i64,
	vlan_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRoleRequest {
	slug: String,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Region {
	created: Option<String>,
	name: String,
	custom_fields: String,
	id: i64,
	slug: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	site_count: i64,
	_depth: i64,
	description: String,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPort {
	custom_fields: String,
	_occupied: bool,
	last_updated: Option<String>,
	speed: Option<String>,
	description: String,
	r#type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints_reachable: bool,
	connected_endpoints_type: String,
	name: String,
	url: Url,
	tags: Vec<NestedTag>,
	created: Option<String>,
	id: i64,
	display: String,
	link_peers: Vec<String>,
	connected_endpoints: Vec<String>,
	/// Physical label
	label: String,
	cable_end: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackRequest {
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	serial: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	tags: Vec<NestedTagRequest>,
	weight: Option<f64>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// Functional role
	role: Option<i64>,
	site: i64,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	location: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	description: String,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Starting unit for rack
	starting_unit: i64,
	name: String,
	facility_id: Option<String>,
	tenant: Option<i64>,
	comments: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Height in rack units
	u_height: i64,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContextRequest {
	site_groups: Vec<i64>,
	weight: i64,
	cluster_groups: Vec<i64>,
	locations: Vec<i64>,
	roles: Vec<i64>,
	clusters: Vec<i64>,
	tenants: Vec<i64>,
	cluster_types: Vec<i64>,
	tags: Vec<String>,
	sites: Vec<i64>,
	description: String,
	tenant_groups: Vec<i64>,
	device_types: Vec<i64>,
	platforms: Vec<i64>,
	is_active: bool,
	regions: Vec<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEProposalRequest {
	comments: String,
	custom_fields: String,
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
	tags: Vec<NestedTagRequest>,
	description: String,
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
}

pub struct PaginatedInventoryItemList {
	previous: Option<Url>,
	results: Vec<InventoryItem>,
	next: Option<Url>,
	count: i64,
}

pub struct PaginatedWirelessLANGroupList {
	count: i64,
	next: Option<Url>,
	results: Vec<WirelessLANGroup>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitRequest {
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	r#type: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	provider_account: Option<i64>,
	/// Unique circuit ID
	cid: String,
	tenant: Option<i64>,
	install_date: Option<String>,
	termination_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	description: String,
	provider: i64,
}

pub struct PaginatedCustomFieldChoiceSetList {
	count: i64,
	results: Vec<CustomFieldChoiceSet>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleTypeRequest {
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	description: String,
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	custom_fields: String,
	manufacturer: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroup {
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	contact_count: i64,
	_depth: i64,
	id: i64,
	slug: String,
	display: String,
	name: String,
	description: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVRFRequest {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	comments: String,
	custom_fields: String,
	name: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	tags: Vec<NestedTagRequest>,
	export_targets: Vec<i64>,
	description: String,
	import_targets: Vec<i64>,
	tenant: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvisionRequest {
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	username: String,
	password: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassis {
	description: String,
	name: String,
	url: Url,
	comments: String,
	id: i64,
	domain: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	member_count: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct ASNRangeRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	start: i64,
	name: String,
	custom_fields: String,
	end: i64,
}

/// Adds support for custom fields and tags.
pub struct WirelessLAN {
	tags: Vec<NestedTag>,
	url: Url,
	description: String,
	auth_cipher: String,
	ssid: String,
	auth_psk: String,
	status: String,
	id: i64,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	comments: String,
	auth_type: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableJournalEntryRequest {
	comments: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	assigned_object_type: String,
	created_by: Option<i64>,
	assigned_object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitTypeRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedInventoryItemRoleRequest {
	name: String,
	slug: String,
	color: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct VRFRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	import_targets: Vec<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	export_targets: Vec<i64>,
	name: String,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroupRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	custom_fields: String,
}

pub struct PaginatedServiceTemplateList {
	count: i64,
	results: Vec<ServiceTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceTemplateRequest {
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	comments: String,
	ports: Vec<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelTerminationRequest {
	termination_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_type: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProposalRequest {
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	name: String,
	comments: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableRearPortTemplateRequest {
	/// Physical label
	label: String,
	device_type: Option<i64>,
	positions: i64,
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
	color: String,
	module_type: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceRequest {
	comments: String,
	custom_fields: String,
	device: Option<i64>,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	virtual_machine: Option<i64>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	description: String,
}


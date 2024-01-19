use serde_qs;
use reqwest::Url;
use crate::util::THANIX_CLIENT;
/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccountRequest {
	account: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceRequest {
	description: String,
	ipaddresses: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	name: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldChoiceSetRequest {
	name: String,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSetRequest {
	name: String,
}

pub struct PaginatedContentTypeList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ContentType>,
}

pub struct PaginatedTunnelTerminationList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<TunnelTermination>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderRequest {
	custom_fields: String,
	comments: String,
	slug: String,
	/// Full name of the provider
	name: String,
	description: String,
	accounts: Vec<i64>,
	asns: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ClusterRequest {
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRouteTargetRequest {
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProposalRequest {
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
	tags: Vec<NestedTagRequest>,
	comments: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContextRequest {
	tags: Vec<NestedTagRequest>,
	vcpus: Option<f64>,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	name: String,
	comments: String,
	memory: Option<i64>,
	disk: Option<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	cid: String,
}

pub struct PaginatedManufacturerList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<Manufacturer>,
}

pub struct PaginatedUserList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<User>,
}

/// Adds support for custom fields and tags.
pub struct PatchedInventoryItemRoleRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	description: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct ASNRangeRequest {
	name: String,
	custom_fields: String,
	start: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	end: i64,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInventoryItemTemplateRequest {
	description: String,
	component_id: Option<i64>,
	device_type: i64,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// Physical label
	label: String,
	manufacturer: Option<i64>,
	role: Option<i64>,
	component_type: Option<String>,
	parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDataSourceRequest {
	source_url: String,
	description: String,
	enabled: bool,
	name: String,
	r#type: String,
	comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilterRequest {
	weight: i64,
	slug: String,
	shared: bool,
	user: Option<i64>,
	description: String,
	content_types: Vec<String>,
	name: String,
	enabled: bool,
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
pub struct NestedClusterGroup {
	url: Url,
	display: String,
	slug: String,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitTermination {
	/// Patch panel ID and port number(s)
	pp_info: String,
	last_updated: Option<String>,
	display: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	link_peers: Vec<String>,
	url: Url,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	created: Option<String>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	_occupied: bool,
	id: i64,
	custom_fields: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	description: String,
	cable_end: String,
	tags: Vec<NestedTag>,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRoleRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPN {
	r#type: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	display: String,
	slug: String,
	id: i64,
	description: String,
	last_updated: Option<String>,
	import_targets: Vec<i64>,
	name: String,
	comments: String,
	url: Url,
	created: Option<String>,
	export_targets: Vec<i64>,
	identifier: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableExportTemplateRequest {
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Download file as attachment
	as_attachment: bool,
	name: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	content_types: Vec<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderNetworkRequest {
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	service_id: String,
	provider: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerOutletRequest {
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
	description: String,
	custom_fields: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
}

/// Adds support for custom fields and tags.
pub struct WritableVRFRequest {
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	comments: String,
	export_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	tenant: Option<i64>,
	description: String,
	import_targets: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceRequest {
	ports: Vec<i64>,
	description: String,
	virtual_machine: Option<i64>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	device: Option<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct IPRange {
	tags: Vec<NestedTag>,
	custom_fields: String,
	last_updated: Option<String>,
	family: String,
	start_address: String,
	status: String,
	id: i64,
	description: String,
	end_address: String,
	created: Option<String>,
	size: i64,
	url: Url,
	/// Treat as 100% utilized
	mark_utilized: bool,
	display: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableFrontPortTemplateRequest {
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
	color: String,
	rear_port: i64,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	rear_port_position: i64,
	description: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct VRFRequest {
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	comments: String,
	name: String,
	custom_fields: String,
	import_targets: Vec<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	description: String,
	export_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPAddressRequest {
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
	address: String,
	tenant: Option<i64>,
	assigned_object_type: Option<String>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	description: String,
	assigned_object_id: Option<i64>,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	vrf: Option<i64>,
	custom_fields: String,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRoleRequest {
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceTemplateRequest {
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ports: Vec<i64>,
	description: String,
	comments: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerOutletTemplateRequest {
	/// Physical label
	label: String,
	power_port: Option<i64>,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	module_type: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterType {
	display: String,
	url: Url,
	id: i64,
	name: String,
	slug: String,
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
pub struct NestedTunnelGroupRequest {
	name: String,
	slug: String,
}

pub struct PaginatedVLANList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<VLAN>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct Site {
	rack_count: i64,
	virtualmachine_count: i64,
	status: String,
	/// Physical location of the building
	physical_address: String,
	prefix_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	custom_fields: String,
	vlan_count: i64,
	time_zone: Option<String>,
	description: String,
	/// If different from the physical address
	shipping_address: String,
	display: String,
	/// Local facility ID or description
	facility: String,
	url: Url,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	asns: Vec<i64>,
	/// Full name of the site
	name: String,
	last_updated: Option<String>,
	created: Option<String>,
	circuit_count: i64,
	device_count: i64,
	slug: String,
	id: i64,
	comments: String,
	tags: Vec<NestedTag>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Location {
	name: String,
	tags: Vec<NestedTag>,
	id: i64,
	_depth: i64,
	description: String,
	created: Option<String>,
	rack_count: i64,
	url: Url,
	status: String,
	custom_fields: String,
	last_updated: Option<String>,
	slug: String,
	device_count: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct Circuit {
	/// Unique circuit ID
	cid: String,
	status: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	comments: String,
	termination_date: Option<String>,
	created: Option<String>,
	custom_fields: String,
	display: String,
	description: String,
	/// Committed rate
	commit_rate: Option<i64>,
	url: Url,
	install_date: Option<String>,
	id: i64,
}

pub struct PaginatedIPSecProposalList {
	next: Option<Url>,
	results: Vec<IPSecProposal>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableModuleBayTemplateRequest {
	/// Physical label
	label: String,
	description: String,
	/// Identifier to reference when renaming installed components
	position: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct EventRule {
	action_object: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	custom_fields: String,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	created: Option<String>,
	enabled: bool,
	action_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	action_object_type: String,
	last_updated: Option<String>,
	display: String,
	action_object_id: Option<i64>,
	description: String,
	name: String,
	id: i64,
	tags: Vec<NestedTag>,
	content_types: Vec<String>,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableBookmarkRequest {
	object_id: i64,
	user: i64,
	object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TagRequest {
	slug: String,
	name: String,
	color: String,
	object_types: Vec<String>,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableContactGroupRequest {
	parent: Option<i64>,
	slug: String,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

pub struct PaginatedServiceList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Service>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerFeedRequest {
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	tags: Vec<NestedTagRequest>,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	voltage: i64,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	comments: String,
	description: String,
	amperage: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	tenant: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	rack: Option<i64>,
	custom_fields: String,
	power_panel: i64,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplateRequest {
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
	description: String,
}

/// Adds support for custom fields and tags.
pub struct JournalEntryRequest {
	assigned_object_id: i64,
	created_by: Option<i64>,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	assigned_object_type: String,
}

pub struct PaginatedGroupList {
	next: Option<Url>,
	results: Vec<Group>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableEventRuleRequest {
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	enabled: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	custom_fields: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	name: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_id: Option<i64>,
	content_types: Vec<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	action_object_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceType {
	url: Url,
	model: String,
	slug: String,
	id: i64,
	display: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObjectRequest {
	object_type: String,
	object_id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInventoryItemRequest {
	parent: Option<i64>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	manufacturer: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	custom_fields: String,
	role: Option<i64>,
	/// Physical label
	label: String,
	serial: String,
	/// This item was automatically discovered
	discovered: bool,
	device: i64,
	component_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	component_type: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PowerPanelRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	name: String,
	comments: String,
}

pub struct PaginatedClusterList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Cluster>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplateRequest {
	parent: Option<i64>,
	component_id: Option<i64>,
	/// Physical label
	label: String,
	component_type: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplateRequest {
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCable {
	id: i64,
	url: Url,
	display: String,
	label: String,
}

/// Adds support for custom fields and tags.
pub struct WritableClusterRequest {
	tenant: Option<i64>,
	r#type: i64,
	name: String,
	comments: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	site: Option<i64>,
	group: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPN {
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
	display: String,
	url: Url,
	slug: String,
	identifier: Option<i64>,
	id: i64,
}

pub struct PaginatedPowerOutletTemplateList {
	results: Vec<PowerOutletTemplate>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactAssignmentRequest {
	role: i64,
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	contact: i64,
	content_type: String,
}

/// Adds support for custom fields and tags.
pub struct IPAddressRequest {
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	assigned_object_type: Option<String>,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	custom_fields: String,
	comments: String,
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
	address: String,
}

pub struct PaginatedModuleBayTemplateList {
	results: Vec<ModuleBayTemplate>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroupRequest {
	name: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleBayRequest {
	device: i64,
	name: String,
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	installed_module: i64,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplateRequest {
	description: String,
	/// Identifier to reference when renaming installed components
	position: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct Tenant {
	name: String,
	circuit_count: i64,
	site_count: i64,
	created: Option<String>,
	virtualmachine_count: i64,
	vrf_count: i64,
	cluster_count: i64,
	id: i64,
	custom_fields: String,
	prefix_count: i64,
	last_updated: Option<String>,
	rack_count: i64,
	url: Url,
	slug: String,
	tags: Vec<NestedTag>,
	ipaddress_count: i64,
	description: String,
	device_count: i64,
	display: String,
	comments: String,
	vlan_count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableVMInterfaceRequest {
	name: String,
	enabled: bool,
	untagged_vlan: Option<i64>,
	virtual_machine: i64,
	parent: Option<i64>,
	vrf: Option<i64>,
	tagged_vlans: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	mtu: Option<i64>,
	custom_fields: String,
	bridge: Option<i64>,
	mac_address: Option<String>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	comments: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRouteTargetRequest {
	custom_fields: String,
	description: String,
	comments: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEProposalRequest {
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	comments: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	tags: Vec<NestedTagRequest>,
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
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelRequest {
	tenant: Option<i64>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	group: Option<i64>,
	description: String,
	name: String,
	ipsec_profile: Option<i64>,
	custom_fields: String,
	tunnel_id: Option<i64>,
}

pub struct PaginatedIKEProposalList {
	previous: Option<Url>,
	results: Vec<IKEProposal>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocationRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroup {
	display: String,
	url: Url,
	name: String,
	id: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleType {
	display: String,
	description: String,
	last_updated: Option<String>,
	weight_unit: Option<String>,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTag>,
	url: Url,
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	created: Option<String>,
	id: i64,
}

pub struct PaginatedEventRuleList {
	count: i64,
	previous: Option<Url>,
	results: Vec<EventRule>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedCustomLinkRequest {
	weight: i64,
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
	content_types: Vec<String>,
	enabled: bool,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	/// Force link to open in a new window
	new_window: bool,
	/// Jinja2 template code for link URL
	link_url: String,
	/// Jinja2 template code for link text
	link_text: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderRequest {
	/// Full name of the provider
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLinkRequest {
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
	content_types: Vec<String>,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	/// Jinja2 template code for link text
	link_text: String,
	enabled: bool,
	/// Jinja2 template code for link URL
	link_url: String,
	weight: i64,
	/// Force link to open in a new window
	new_window: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableAggregateRequest {
	prefix: String,
	custom_fields: String,
	description: String,
	tenant: Option<i64>,
	date_added: Option<String>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNTerminationRequest {
	l2vpn: i64,
	custom_fields: String,
	assigned_object_id: i64,
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVLANRequest {
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	description: String,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	/// The primary function of this VLAN
	role: Option<i64>,
	/// VLAN group (optional)
	group: Option<i64>,
	comments: String,
	custom_fields: String,
	name: String,
	tenant: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRole {
	name: String,
	url: Url,
	display: String,
	id: i64,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldRequest {
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	object_type: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Internal field name
	name: String,
	description: String,
	choice_set: Option<i64>,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
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
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	content_types: Vec<String>,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
}

/// Adds support for custom fields and tags.
pub struct CableRequest {
	label: String,
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
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: Option<String>,
	length: Option<f64>,
	custom_fields: String,
	color: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	b_terminations: Vec<GenericObjectRequest>,
	a_terminations: Vec<GenericObjectRequest>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroupRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRackReservationRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	units: Vec<i64>,
	custom_fields: String,
	rack: i64,
	user: i64,
	tenant: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccount {
	id: i64,
	url: Url,
	display: String,
	name: String,
	account: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDisk {
	name: String,
	url: Url,
	custom_fields: String,
	created: Option<String>,
	description: String,
	size: i64,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Tag {
	last_updated: Option<String>,
	display: String,
	id: i64,
	object_types: Vec<String>,
	slug: String,
	created: Option<String>,
	color: String,
	tagged_items: i64,
	name: String,
	description: String,
	url: Url,
}

pub struct PaginatedPowerPanelList {
	next: Option<Url>,
	results: Vec<PowerPanel>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedImageAttachmentRequest {
	image_height: i64,
	image_width: i64,
	content_type: String,
	object_id: i64,
	image: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLANRequest {
	custom_fields: String,
	comments: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
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
	description: String,
	tags: Vec<NestedTagRequest>,
	ssid: String,
	auth_psk: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceRoleRequest {
	slug: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	name: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePlatformRequest {
	name: String,
	config_template: Option<i64>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWebhookRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	name: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	custom_fields: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitRequest {
	custom_fields: String,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	install_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Committed rate
	commit_rate: Option<i64>,
	comments: String,
	termination_date: Option<String>,
	/// Unique circuit ID
	cid: String,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPort {
	/// Physical label
	label: String,
	connected_endpoints_type: String,
	id: i64,
	connected_endpoints_reachable: bool,
	custom_fields: String,
	speed: Option<String>,
	description: String,
	link_peers: Vec<String>,
	_occupied: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	cable_end: String,
	tags: Vec<NestedTag>,
	url: Url,
	/// Treat as if a cable is connected
	mark_connected: bool,
	last_updated: Option<String>,
	display: String,
	name: String,
	created: Option<String>,
	connected_endpoints: Vec<String>,
	r#type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProvider {
	url: Url,
	/// Full name of the provider
	name: String,
	display: String,
	id: i64,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Manufacturer {
	last_updated: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	inventoryitem_count: i64,
	slug: String,
	created: Option<String>,
	custom_fields: String,
	display: String,
	url: Url,
	id: i64,
	name: String,
	devicetype_count: i64,
	platform_count: i64,
}

pub struct PaginatedIPSecPolicyList {
	count: i64,
	results: Vec<IPSecPolicy>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RackRoleRequest {
	name: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRearPortRequest {
	/// Physical label
	label: String,
	color: String,
	custom_fields: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	module: Option<i64>,
	/// Number of front ports which may be mapped
	positions: i64,
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
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataFile {
	display: String,
	url: Url,
	/// File path relative to the data source's root
	path: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct CircuitTerminationRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	custom_fields: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	description: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
}

pub struct PaginatedWebhookList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<Webhook>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPRangeRequest {
	end_address: String,
	/// The primary function of this range
	role: Option<i64>,
	tenant: Option<i64>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	start_address: String,
	tags: Vec<NestedTagRequest>,
	vrf: Option<i64>,
	comments: String,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

/// Adds support for custom fields and tags.
pub struct ConsolePort {
	description: String,
	connected_endpoints_type: String,
	last_updated: Option<String>,
	url: Url,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	tags: Vec<NestedTag>,
	connected_endpoints: Vec<String>,
	id: i64,
	display: String,
	_occupied: bool,
	created: Option<String>,
	name: String,
	link_peers: Vec<String>,
	connected_endpoints_reachable: bool,
	cable_end: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	speed: Option<String>,
	/// Physical label
	label: String,
	r#type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTermination {
	display: String,
	url: Url,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBayRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelRequest {
	name: String,
}

pub struct PaginatedContactGroupList {
	results: Vec<ContactGroup>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicyRequest {
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	description: String,
	proposals: Vec<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	preshared_key: String,
	name: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
}

/// Representation of an IP address which does not exist in the database.
pub struct AvailableIP {
	family: i64,
	description: String,
	address: String,
}

pub struct PaginatedRearPortTemplateList {
	next: Option<Url>,
	results: Vec<RearPortTemplate>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPAddressRequest {
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
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
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	custom_fields: String,
	tenant: Option<i64>,
	description: String,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
	assigned_object_type: Option<String>,
	assigned_object_id: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	address: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLinkRequest {
	custom_fields: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	ssid: String,
	description: String,
	auth_psk: String,
	tags: Vec<NestedTagRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	interface_a: i64,
	interface_b: i64,
	tenant: Option<i64>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRangeRequest {
	slug: String,
	tenant: Option<i64>,
	custom_fields: String,
	end: i64,
	start: i64,
	name: String,
	rir: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePrefixRequest {
	tenant: Option<i64>,
	/// The primary function of this prefix
	role: Option<i64>,
	vrf: Option<i64>,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	prefix: String,
	custom_fields: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	vlan: Option<i64>,
	site: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachine {
	id: i64,
	display: String,
	name: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct CableTerminationRequest {
	termination_type: String,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	cable: i64,
	termination_id: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerFeed {
	status: String,
	connected_endpoints_type: String,
	connected_endpoints: Vec<String>,
	custom_fields: String,
	created: Option<String>,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	display: String,
	cable_end: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	tags: Vec<NestedTag>,
	connected_endpoints_reachable: bool,
	phase: String,
	link_peers: Vec<String>,
	url: Url,
	last_updated: Option<String>,
	supply: String,
	r#type: String,
	id: i64,
	comments: String,
	_occupied: bool,
	amperage: i64,
	voltage: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitTypeRequest {
	custom_fields: String,
	color: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedIKEPolicyList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<IKEPolicy>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermission {
	groups: Vec<i64>,
	users: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	object_types: Vec<String>,
	enabled: bool,
	description: String,
	url: Url,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplateRequest {
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	custom_fields: String,
	name: String,
	ports: Vec<i64>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct WritableConfigTemplateRequest {
	/// Remote data source
	data_source: Option<i64>,
	data_file: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Jinja2 template code.
	template_code: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	comments: String,
	weight: Option<f64>,
	rear_image: String,
	custom_fields: String,
	manufacturer: i64,
	slug: String,
	u_height: f64,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	front_image: String,
	/// Discrete part number (optional)
	part_number: String,
	default_platform: Option<i64>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	model: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatformRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleBayRequest {
	custom_fields: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	device: i64,
	name: String,
	/// Physical label
	label: String,
	installed_module: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEPolicyRequest {
	description: String,
	proposals: Vec<i64>,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	comments: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	preshared_key: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleTypeRequest {
	model: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelGroup {
	display: String,
	name: String,
	slug: String,
	url: Url,
	id: i64,
}

pub struct PaginatedCustomFieldChoiceSetList {
	count: i64,
	next: Option<Url>,
	results: Vec<CustomFieldChoiceSet>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct CircuitType {
	display: String,
	color: String,
	circuit_count: i64,
	name: String,
	last_updated: Option<String>,
	created: Option<String>,
	tags: Vec<NestedTag>,
	id: i64,
	url: Url,
	description: String,
	custom_fields: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceType {
	id: i64,
	power_outlet_template_count: i64,
	u_height: f64,
	device_bay_template_count: i64,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	created: Option<String>,
	console_server_port_template_count: i64,
	slug: String,
	comments: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	weight_unit: Option<String>,
	rear_port_template_count: i64,
	airflow: Option<String>,
	subdevice_role: Option<String>,
	device_count: i64,
	display: String,
	front_port_template_count: i64,
	url: Url,
	console_port_template_count: i64,
	front_image: Url,
	description: String,
	weight: Option<f64>,
	/// Discrete part number (optional)
	part_number: String,
	custom_fields: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	module_bay_template_count: i64,
	interface_template_count: i64,
	model: String,
	inventory_item_template_count: i64,
	rear_image: Url,
	power_port_template_count: i64,
}

pub struct PaginatedSiteGroupList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<SiteGroup>,
}

pub struct PaginatedAggregateList {
	results: Vec<Aggregate>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RackReservation {
	last_updated: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	comments: String,
	display: String,
	units: Vec<i64>,
	created: Option<String>,
	url: Url,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct TunnelRequest {
	custom_fields: String,
	name: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	tunnel_id: Option<i64>,
	comments: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
}

/// Adds support for custom fields and tags.
pub struct WritableJournalEntryRequest {
	created_by: Option<i64>,
	comments: String,
	assigned_object_type: String,
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitRequest {
	description: String,
	/// Committed rate
	commit_rate: Option<i64>,
	tenant: Option<i64>,
	termination_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	provider: i64,
	r#type: i64,
	provider_account: Option<i64>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	/// Unique circuit ID
	cid: String,
	install_date: Option<String>,
	custom_fields: String,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroup {
	id: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	description: String,
	slug: String,
	display: String,
	name: String,
	url: Url,
	created: Option<String>,
	last_updated: Option<String>,
	wirelesslan_count: i64,
	_depth: i64,
}

/// Adds support for custom fields and tags.
pub struct WirelessLink {
	url: Url,
	display: String,
	comments: String,
	custom_fields: String,
	status: String,
	id: i64,
	auth_type: String,
	last_updated: Option<String>,
	description: String,
	auth_cipher: String,
	created: Option<String>,
	auth_psk: String,
	ssid: String,
	tags: Vec<NestedTag>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroupRequest {
	description: String,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSet {
	choices_count: String,
	display: String,
	created: Option<String>,
	url: Url,
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
	name: String,
	base_choices: String,
	last_updated: Option<String>,
	id: i64,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
}

/// Adds support for custom fields and tags.
pub struct DeviceRole {
	last_updated: Option<String>,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	tags: Vec<NestedTag>,
	description: String,
	name: String,
	device_count: i64,
	created: Option<String>,
	display: String,
	id: i64,
	url: Url,
	color: String,
	slug: String,
	custom_fields: String,
	virtualmachine_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRequest {
	name: String,
}

pub struct PaginatedVMInterfaceList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<VMInterface>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConfigContextRequest {
	description: String,
	locations: Vec<i64>,
	platforms: Vec<i64>,
	cluster_groups: Vec<i64>,
	device_types: Vec<i64>,
	weight: i64,
	/// Remote data source
	data_source: Option<i64>,
	tenant_groups: Vec<i64>,
	cluster_types: Vec<i64>,
	name: String,
	regions: Vec<i64>,
	site_groups: Vec<i64>,
	sites: Vec<i64>,
	roles: Vec<i64>,
	clusters: Vec<i64>,
	tenants: Vec<i64>,
	tags: Vec<String>,
	is_active: bool,
}

pub struct PaginatedTunnelGroupList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<TunnelGroup>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackRequest {
	/// Functional role
	role: Option<i64>,
	tags: Vec<NestedTagRequest>,
	serial: String,
	location: Option<i64>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// Height in rack units
	u_height: i64,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	name: String,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	facility_id: Option<String>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	site: i64,
	/// Starting unit for rack
	starting_unit: i64,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	weight: Option<f64>,
	comments: String,
	custom_fields: String,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	tenant: Option<i64>,
	description: String,
}

pub struct PaginatedCustomFieldList {
	results: Vec<CustomField>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPortRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	device: i64,
	name: String,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRole {
	slug: String,
	name: String,
	url: Url,
	display: String,
	id: i64,
}

pub struct PaginatedProviderList {
	count: i64,
	results: Vec<Provider>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualChassisRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	master: Option<i64>,
	custom_fields: String,
	comments: String,
	domain: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualMachineWithConfigContextRequest {
	tenant: Option<i64>,
	primary_ip4: Option<i64>,
	primary_ip6: Option<i64>,
	description: String,
	comments: String,
	platform: Option<i64>,
	custom_fields: String,
	device: Option<i64>,
	cluster: Option<i64>,
	site: Option<i64>,
	name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	role: Option<i64>,
	disk: Option<i64>,
	vcpus: Option<f64>,
	memory: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	/// Physical label
	label: String,
}

pub struct PaginatedVLANGroupList {
	results: Vec<VLANGroup>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableJournalEntryRequest {
	created_by: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	custom_fields: String,
	assigned_object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRoleRequest {
	name: String,
	slug: String,
}

pub struct PaginatedRegionList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<Region>,
}

/// Adds support for custom fields and tags.
pub struct SiteRequest {
	time_zone: Option<String>,
	/// Local facility ID or description
	facility: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	asns: Vec<i64>,
	description: String,
	/// Full name of the site
	name: String,
	/// If different from the physical address
	shipping_address: String,
	slug: String,
	/// Physical location of the building
	physical_address: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableFrontPortTemplateRequest {
	rear_port: i64,
	rear_port_position: i64,
	module_type: Option<i64>,
	description: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableInterfaceRequest {
	vdcs: Vec<i64>,
	/// Physical label
	label: String,
	wireless_lans: Vec<i64>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	untagged_vlan: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	module: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	mtu: Option<i64>,
	enabled: bool,
	speed: Option<i64>,
	parent: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
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
	mac_address: Option<String>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	tx_power: Option<i64>,
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
	bridge: Option<i64>,
	tagged_vlans: Vec<i64>,
	lag: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	vrf: Option<i64>,
	custom_fields: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	device: i64,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	wwn: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableUserRequest {
	password: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	date_joined: String,
	last_name: String,
	email: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	first_name: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccount {
	url: Url,
	id: i64,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	name: String,
	comments: String,
	last_updated: Option<String>,
	account: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplate {
	created: Option<String>,
	last_updated: Option<String>,
	/// Identifier to reference when renaming installed components
	position: String,
	display: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	/// Physical label
	label: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddress {
	address: String,
	family: i64,
	display: String,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicy {
	name: String,
	display: String,
	proposals: Vec<i64>,
	comments: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	pfs_group: String,
	url: Url,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContextRequest {
	clusters: Vec<i64>,
	locations: Vec<i64>,
	sites: Vec<i64>,
	cluster_types: Vec<i64>,
	roles: Vec<i64>,
	tags: Vec<String>,
	tenants: Vec<i64>,
	is_active: bool,
	regions: Vec<i64>,
	weight: i64,
	description: String,
	platforms: Vec<i64>,
	name: String,
	site_groups: Vec<i64>,
	device_types: Vec<i64>,
	tenant_groups: Vec<i64>,
	cluster_groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct DataFile {
	/// File path relative to the data source's root
	path: String,
	url: Url,
	display: String,
	size: i64,
	id: i64,
	last_updated: String,
	/// SHA256 hash of the file data
	hash: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLAN {
	custom_fields: String,
	created: Option<String>,
	url: Url,
	ssid: String,
	id: i64,
	tags: Vec<NestedTag>,
	comments: String,
	display: String,
	description: String,
	auth_cipher: String,
	last_updated: Option<String>,
	auth_psk: String,
	auth_type: String,
	status: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerPortTemplateRequest {
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	device_type: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	module_type: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConfigContextRequest {
	site_groups: Vec<i64>,
	cluster_types: Vec<i64>,
	platforms: Vec<i64>,
	is_active: bool,
	clusters: Vec<i64>,
	name: String,
	device_types: Vec<i64>,
	locations: Vec<i64>,
	weight: i64,
	roles: Vec<i64>,
	sites: Vec<i64>,
	regions: Vec<i64>,
	tenant_groups: Vec<i64>,
	tenants: Vec<i64>,
	cluster_groups: Vec<i64>,
	tags: Vec<String>,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct Cable {
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
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	status: String,
	length: Option<f64>,
	last_updated: Option<String>,
	id: i64,
	comments: String,
	b_terminations: Vec<GenericObject>,
	label: String,
	color: String,
	a_terminations: Vec<GenericObject>,
	custom_fields: String,
	display: String,
	url: Url,
	length_unit: Option<String>,
}

pub struct PaginatedSavedFilterList {
	results: Vec<SavedFilter>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfile {
	display: String,
	id: i64,
	name: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRole {
	url: Url,
	id: i64,
	display: String,
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterfaceRequest {
	name: String,
}

pub struct PaginatedRoleList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Role>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerOutletRequest {
	device: i64,
	module: Option<i64>,
	description: String,
	custom_fields: String,
	name: String,
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
	tags: Vec<NestedTagRequest>,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
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

/// Adds support for custom fields and tags.
pub struct IPSecProfileRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	custom_fields: String,
	description: String,
	comments: String,
}

pub struct PaginatedRackReservationList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<RackReservation>,
}

/// Adds support for custom fields and tags.
pub struct Tunnel {
	comments: String,
	last_updated: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	status: String,
	description: String,
	created: Option<String>,
	display: String,
	tunnel_id: Option<i64>,
	encapsulation: String,
	name: String,
	custom_fields: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct Webhook {
	display: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	custom_fields: String,
	created: Option<String>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	last_updated: Option<String>,
	id: i64,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	description: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	url: Url,
	tags: Vec<NestedTag>,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInterfaceTemplateRequest {
	module_type: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
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
	/// Physical label
	label: String,
	enabled: bool,
	mgmt_only: bool,
	description: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	bridge: Option<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNRequest {
	comments: String,
	custom_fields: String,
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
	identifier: Option<i64>,
	tenant: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	import_targets: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct RackRequest {
	/// Height in rack units
	u_height: i64,
	/// Starting unit for rack
	starting_unit: i64,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: Option<String>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	weight: Option<f64>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: Option<String>,
	custom_fields: String,
	comments: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	name: String,
	serial: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	facility_id: Option<String>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
}

pub struct PaginatedTagList {
	count: i64,
	previous: Option<Url>,
	results: Vec<Tag>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableCableRequest {
	label: String,
	tenant: Option<i64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
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
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	length: Option<f64>,
	description: String,
	comments: String,
	a_terminations: Vec<GenericObjectRequest>,
	b_terminations: Vec<GenericObjectRequest>,
}

pub struct PaginatedJournalEntryList {
	count: i64,
	results: Vec<JournalEntry>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableSiteGroupRequest {
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	parent: Option<i64>,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroup {
	last_updated: Option<String>,
	url: Url,
	tags: Vec<NestedTag>,
	_depth: i64,
	slug: String,
	created: Option<String>,
	id: i64,
	name: String,
	display: String,
	contact_count: i64,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroup {
	display: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	slug: String,
	tunnel_count: i64,
	description: String,
	last_updated: Option<String>,
	custom_fields: String,
	url: Url,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct RackRole {
	last_updated: Option<String>,
	custom_fields: String,
	rack_count: i64,
	name: String,
	description: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	slug: String,
	id: i64,
	url: Url,
	display: String,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfileRequest {
	name: String,
}

pub struct PaginatedDeviceBayTemplateList {
	results: Vec<DeviceBayTemplate>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedFrontPortTemplateList {
	previous: Option<Url>,
	results: Vec<FrontPortTemplate>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedTunnelGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	name: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct PatchedWritableConfigTemplateRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// Remote data source
	data_source: Option<i64>,
	/// Jinja2 template code.
	template_code: String,
	data_file: Option<i64>,
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
	virtual_chassis: Option<i64>,
	primary_ip4: Option<i64>,
	custom_fields: String,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	tenant: Option<i64>,
	site: i64,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	config_template: Option<i64>,
	location: Option<i64>,
	oob_ip: Option<i64>,
	cluster: Option<i64>,
	position: Option<f64>,
	tags: Vec<NestedTagRequest>,
	primary_ip6: Option<i64>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	device_type: i64,
	rack: Option<i64>,
	platform: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	comments: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// The function this device serves
	role: i64,
	name: Option<String>,
	vc_position: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicyRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecPolicyRequest {
	tags: Vec<NestedTagRequest>,
	proposals: Vec<i64>,
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
	description: String,
	name: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplate {
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	id: i64,
	url: Url,
	/// Physical label
	label: String,
	r#type: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Platform {
	url: Url,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	description: String,
	name: String,
	slug: String,
	created: Option<String>,
	device_count: i64,
	id: i64,
	last_updated: Option<String>,
	virtualmachine_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRIRRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
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

/// Adds support for custom fields and tags.
pub struct L2VPNTermination {
	url: Url,
	display: String,
	id: i64,
	custom_fields: String,
	assigned_object_id: i64,
	created: Option<String>,
	assigned_object_type: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
}

pub struct PaginatedCircuitTerminationList {
	results: Vec<CircuitTermination>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPortRequest {
	device: i64,
	module: Option<i64>,
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
	name: String,
	/// Physical label
	label: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplateRequest {
	description: String,
	content_types: Vec<String>,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	name: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Download file as attachment
	as_attachment: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroup {
	id: i64,
	name: String,
	url: Url,
	display: String,
	slug: String,
	_depth: i64,
}

/// Adds support for custom fields and tags.
pub struct ModuleBay {
	url: Url,
	id: i64,
	description: String,
	tags: Vec<NestedTag>,
	name: String,
	created: Option<String>,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
	custom_fields: String,
	last_updated: Option<String>,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroup {
	display: String,
	slug: String,
	id: i64,
	_depth: i64,
	url: Url,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroupRequest {
	name: String,
	slug: String,
}

pub struct PaginatedFHRPGroupAssignmentList {
	count: i64,
	previous: Option<Url>,
	results: Vec<FHRPGroupAssignment>,
	next: Option<Url>,
}

pub struct PaginatedServiceTemplateList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ServiceTemplate>,
}

pub struct PaginatedVirtualDiskList {
	previous: Option<Url>,
	results: Vec<VirtualDisk>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct ModuleTypeRequest {
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	description: String,
	custom_fields: String,
	model: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
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
pub struct NestedRIR {
	url: Url,
	slug: String,
	display: String,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VMInterface {
	name: String,
	enabled: bool,
	id: i64,
	custom_fields: String,
	url: Url,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	count_ipaddresses: i64,
	tagged_vlans: Vec<i64>,
	created: Option<String>,
	count_fhrp_groups: i64,
	description: String,
	mac_address: Option<String>,
	mtu: Option<i64>,
	display: String,
	mode: String,
}

/// Adds support for custom fields and tags.
pub struct JournalEntry {
	last_updated: Option<String>,
	id: i64,
	created: Option<String>,
	url: Url,
	custom_fields: String,
	assigned_object_id: i64,
	tags: Vec<NestedTag>,
	comments: String,
	created_by: Option<i64>,
	display: String,
	assigned_object_type: String,
	kind: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsolePortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	module_type: Option<i64>,
	description: String,
	device_type: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ASNRange {
	url: Url,
	last_updated: Option<String>,
	name: String,
	start: i64,
	slug: String,
	display: String,
	description: String,
	id: i64,
	end: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	asn_count: i64,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTerminationRequest {
	description: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
}

/// Representation of a prefix which does not exist in the database.
pub struct AvailablePrefix {
	family: i64,
	prefix: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRole {
	url: Url,
	created: Option<String>,
	slug: String,
	name: String,
	last_updated: Option<String>,
	inventoryitem_count: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	id: i64,
	description: String,
	color: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterTypeRequest {
	name: String,
	custom_fields: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePlatformRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	config_template: Option<i64>,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleBay {
	display: String,
	id: i64,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVRFRequest {
	description: String,
	name: String,
	custom_fields: String,
	export_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	tenant: Option<i64>,
	import_targets: Vec<i64>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct Device {
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	console_port_count: i64,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	interface_count: i64,
	comments: String,
	device_bay_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	power_port_count: i64,
	url: Url,
	last_updated: Option<String>,
	face: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	status: String,
	console_server_port_count: i64,
	airflow: String,
	description: String,
	display: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	position: Option<f64>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	front_port_count: i64,
	vc_position: Option<i64>,
	module_bay_count: i64,
	name: Option<String>,
	inventory_item_count: i64,
	rear_port_count: i64,
	power_outlet_count: i64,
	id: i64,
	custom_fields: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPortRequest {
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableInventoryItemRequest {
	/// This item was automatically discovered
	discovered: bool,
	serial: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	parent: Option<i64>,
	role: Option<i64>,
	/// Physical label
	label: String,
	component_id: Option<i64>,
	manufacturer: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct IKEProposal {
	tags: Vec<NestedTag>,
	authentication_method: String,
	created: Option<String>,
	description: String,
	authentication_algorithm: String,
	encryption_algorithm: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	last_updated: Option<String>,
	display: String,
	id: i64,
	url: Url,
	name: String,
	comments: String,
	group: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContextRequest {
	vc_position: Option<i64>,
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
	position: Option<f64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	name: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	description: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
}

/// Adds support for custom fields and tags.
pub struct PrefixRequest {
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	custom_fields: String,
	prefix: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCluster {
	url: Url,
	id: i64,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ASN {
	display: String,
	last_updated: Option<String>,
	site_count: i64,
	provider_count: i64,
	url: Url,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	id: i64,
	created: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	comments: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachmentRequest {
	content_type: String,
	name: String,
	object_id: i64,
	image_height: i64,
	image: String,
	image_width: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRoleRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModule {
	url: Url,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct Interface {
	description: String,
	tagged_vlans: Vec<i64>,
	connected_endpoints_reachable: bool,
	display: String,
	wwn: Option<String>,
	poe_type: String,
	_occupied: bool,
	mode: String,
	tags: Vec<NestedTag>,
	name: String,
	r#type: String,
	url: Url,
	speed: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	link_peers: Vec<String>,
	wireless_lans: Vec<i64>,
	mac_address: Option<String>,
	connected_endpoints: Vec<String>,
	poe_mode: String,
	duplex: Option<String>,
	/// Physical label
	label: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	tx_power: Option<i64>,
	created: Option<String>,
	rf_channel: String,
	enabled: bool,
	mtu: Option<i64>,
	last_updated: Option<String>,
	count_fhrp_groups: i64,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	rf_role: String,
	count_ipaddresses: i64,
	vdcs: Vec<i64>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints_type: String,
	id: i64,
	cable_end: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDiskRequest {
	description: String,
	custom_fields: String,
	size: i64,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableFrontPortRequest {
	name: String,
	device: i64,
	color: String,
	rear_port: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Physical label
	label: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
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
}

/// Adds support for custom fields and tags.
pub struct AggregateRequest {
	prefix: String,
	description: String,
	custom_fields: String,
	date_added: Option<String>,
	tags: Vec<NestedTagRequest>,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocation {
	url: Url,
	id: i64,
	name: String,
	slug: String,
	_depth: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProfileRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	comments: String,
	ipsec_policy: i64,
	ike_policy: i64,
	custom_fields: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableLocationRequest {
	parent: Option<i64>,
	slug: String,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	tenant: Option<i64>,
	custom_fields: String,
	name: String,
	site: i64,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantRequest {
	slug: String,
	name: String,
}

pub struct PaginatedTenantList {
	previous: Option<Url>,
	results: Vec<Tenant>,
	count: i64,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermissionRequest {
	object_types: Vec<String>,
	users: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	name: String,
	enabled: bool,
	description: String,
	groups: Vec<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableContactGroupRequest {
	custom_fields: String,
	description: String,
	slug: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Provider {
	accounts: Vec<i64>,
	description: String,
	last_updated: Option<String>,
	circuit_count: i64,
	url: Url,
	created: Option<String>,
	/// Full name of the provider
	name: String,
	custom_fields: String,
	display: String,
	slug: String,
	comments: String,
	asns: Vec<i64>,
	tags: Vec<NestedTag>,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomField {
	object_type: String,
	url: Url,
	description: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	filter_logic: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	ui_visible: String,
	ui_editable: String,
	content_types: Vec<String>,
	data_type: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	created: Option<String>,
	/// Internal field name
	name: String,
	last_updated: Option<String>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	r#type: String,
	id: i64,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	display: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturerRequest {
	name: String,
	slug: String,
}

pub struct PaginatedTokenList {
	next: Option<Url>,
	results: Vec<Token>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RearPort {
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	display: String,
	cable_end: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	created: Option<String>,
	name: String,
	description: String,
	url: Url,
	/// Number of front ports which may be mapped
	positions: i64,
	link_peers: Vec<String>,
	id: i64,
	tags: Vec<NestedTag>,
	color: String,
	_occupied: bool,
	/// Physical label
	label: String,
	r#type: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRangeRequest {
	slug: String,
	start: i64,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	rir: i64,
	custom_fields: String,
	tenant: Option<i64>,
	end: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct GroupRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANRequest {
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
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
	termination_date: Option<String>,
	description: String,
	comments: String,
	custom_fields: String,
	install_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	/// Unique circuit ID
	cid: String,
	provider_account: Option<i64>,
	r#type: i64,
	provider: i64,
	tenant: Option<i64>,
	/// Committed rate
	commit_rate: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct VMInterfaceRequest {
	enabled: bool,
	description: String,
	tagged_vlans: Vec<i64>,
	custom_fields: String,
	mac_address: Option<String>,
	mtu: Option<i64>,
	name: String,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnel {
	display: String,
	id: i64,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleTypeRequest {
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	weight: Option<f64>,
	custom_fields: String,
	/// Discrete part number (optional)
	part_number: String,
	manufacturer: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
	model: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplate {
	content_types: Vec<String>,
	last_updated: Option<String>,
	description: String,
	created: Option<String>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	data_synced: Option<String>,
	display: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	name: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Download file as attachment
	as_attachment: bool,
	url: Url,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEPolicyRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	proposals: Vec<i64>,
	preshared_key: String,
	description: String,
	comments: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRequest {
	description: String,
	custom_fields: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	/// This item was automatically discovered
	discovered: bool,
	component_type: Option<String>,
	component_id: Option<i64>,
	parent: Option<i64>,
	/// Physical label
	label: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	serial: String,
}

pub struct PaginatedPowerPortTemplateList {
	results: Vec<PowerPortTemplate>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedTagRequest {
	name: String,
	color: String,
	object_types: Vec<String>,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelTerminationRequest {
	tunnel: i64,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	custom_fields: String,
	termination_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	termination_type: String,
	outside_ip: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableTokenRequest {
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	last_used: Option<String>,
	description: String,
	key: String,
	user: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableSiteGroupRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	slug: String,
	parent: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplate {
	display: String,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
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

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRoleRequest {
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct UserRequest {
	date_joined: String,
	email: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	password: String,
	groups: Vec<i64>,
	first_name: String,
	last_name: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceBayRequest {
	installed_device: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	device: i64,
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableRegionRequest {
	custom_fields: String,
	parent: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDeviceContextRequest {
	primary_ip4: Option<i64>,
	device: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	primary_ip6: Option<i64>,
	name: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	description: String,
	custom_fields: String,
	tenant: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleType {
	id: i64,
	display: String,
	url: Url,
	model: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRack {
	id: i64,
	url: Url,
	display: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplate {
	positions: i64,
	r#type: String,
	display: String,
	color: String,
	created: Option<String>,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
	description: String,
	url: Url,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignment {
	created: Option<String>,
	display: String,
	url: Url,
	priority: i64,
	id: i64,
	interface_id: i64,
	last_updated: Option<String>,
	interface_type: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedCableTerminationRequest {
	termination_id: i64,
	cable: i64,
	termination_type: String,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
}

/// Adds support for custom fields and tags.
pub struct RIRRequest {
	slug: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProfileRequest {
	ike_policy: i64,
	custom_fields: String,
	comments: String,
	ipsec_policy: i64,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
}

pub struct PaginatedVirtualChassisList {
	count: i64,
	next: Option<Url>,
	results: Vec<VirtualChassis>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedRIRRequest {
	custom_fields: String,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// IP space managed by this RIR is considered private
	is_private: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterface {
	id: i64,
	_occupied: bool,
	url: Url,
	display: String,
	name: String,
	cable: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableConsoleServerPortRequest {
	name: String,
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
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
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
	/// Physical label
	label: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroup {
	description: String,
	tags: Vec<NestedTag>,
	id: i64,
	last_updated: Option<String>,
	display: String,
	name: String,
	tenant_count: i64,
	url: Url,
	slug: String,
	_depth: i64,
	created: Option<String>,
	custom_fields: String,
}

pub struct PaginatedFrontPortList {
	count: i64,
	results: Vec<FrontPort>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct DataSourceRequest {
	enabled: bool,
	name: String,
	source_url: String,
	description: String,
	comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecPolicyRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
	name: String,
	description: String,
	proposals: Vec<i64>,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableRearPortTemplateRequest {
	module_type: Option<i64>,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	color: String,
	positions: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLinkRequest {
	ssid: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tenant: Option<i64>,
	interface_a: i64,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	auth_psk: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	interface_b: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuit {
	display: String,
	url: Url,
	/// Unique circuit ID
	cid: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPanelRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	site: i64,
	location: Option<i64>,
	description: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanelRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct EventRuleRequest {
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	enabled: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	description: String,
	name: String,
	content_types: Vec<String>,
	custom_fields: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	tags: Vec<NestedTagRequest>,
	action_object_type: String,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	action_object_id: Option<i64>,
}

pub struct PaginatedConsolePortTemplateList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<ConsolePortTemplate>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCableRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	a_terminations: Vec<GenericObjectRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	b_terminations: Vec<GenericObjectRequest>,
	label: String,
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
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	length: Option<f64>,
	description: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddressRequest {
	address: String,
}

pub struct PaginatedModuleBayList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ModuleBay>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatform {
	name: String,
	slug: String,
	display: String,
	id: i64,
	url: Url,
}

pub struct PaginatedInventoryItemRoleList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<InventoryItemRole>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableWirelessLANGroupRequest {
	parent: Option<i64>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceWithConfigContextRequest {
	primary_ip6: Option<i64>,
	tenant: Option<i64>,
	virtual_chassis: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	name: Option<String>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	custom_fields: String,
	position: Option<f64>,
	tags: Vec<NestedTagRequest>,
	platform: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	primary_ip4: Option<i64>,
	vc_position: Option<i64>,
	device_type: i64,
	cluster: Option<i64>,
	/// The function this device serves
	role: i64,
	description: String,
	config_template: Option<i64>,
	location: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	comments: String,
	site: i64,
	rack: Option<i64>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	oob_ip: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableConsolePortRequest {
	description: String,
	custom_fields: String,
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	module: Option<i64>,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRole {
	slug: String,
	name: String,
	id: i64,
	display: String,
	url: Url,
}

pub struct PaginatedProviderAccountList {
	previous: Option<Url>,
	count: i64,
	results: Vec<ProviderAccount>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceBayRequest {
	name: String,
	/// Physical label
	label: String,
	device: i64,
	installed_device: Option<i64>,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct RoleRequest {
	description: String,
	weight: i64,
	name: String,
	custom_fields: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct VLAN {
	comments: String,
	url: Url,
	id: i64,
	status: String,
	custom_fields: String,
	last_updated: Option<String>,
	display: String,
	description: String,
	tags: Vec<NestedTag>,
	prefix_count: i64,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	created: Option<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualChassisRequest {
	master: Option<i64>,
	domain: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VRF {
	ipaddress_count: i64,
	id: i64,
	export_targets: Vec<i64>,
	import_targets: Vec<i64>,
	created: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	url: Url,
	name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	prefix_count: i64,
	last_updated: Option<String>,
	comments: String,
	custom_fields: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct VLANRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	custom_fields: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerPortTemplateRequest {
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
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

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContext {
	device_bay_count: i64,
	created: Option<String>,
	vc_position: Option<i64>,
	console_port_count: i64,
	name: Option<String>,
	console_server_port_count: i64,
	module_bay_count: i64,
	inventory_item_count: i64,
	status: String,
	comments: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	tags: Vec<NestedTag>,
	position: Option<f64>,
	power_port_count: i64,
	id: i64,
	description: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	url: Url,
	last_updated: Option<String>,
	custom_fields: String,
	display: String,
	face: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	airflow: String,
	power_outlet_count: i64,
	interface_count: i64,
	front_port_count: i64,
	rear_port_count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableObjectPermissionRequest {
	object_types: Vec<String>,
	name: String,
	description: String,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	users: Vec<i64>,
	enabled: bool,
	groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct ASNRequest {
	description: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// 16- or 32-bit autonomous system number
	asn: i64,
}

pub struct PatchedDashboardRequest {
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplate {
	last_updated: Option<String>,
	url: Url,
	id: i64,
	color: String,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
	rear_port_position: i64,
	r#type: String,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTag {
	id: i64,
	slug: String,
	color: String,
	display: String,
	name: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRFRequest {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
}

pub struct PaginatedASNList {
	count: i64,
	results: Vec<ASN>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RackReservationRequest {
	comments: String,
	description: String,
	units: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct FrontPortRequest {
	description: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	/// Physical label
	label: String,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	name: String,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterTypeRequest {
	name: String,
	slug: String,
}

/// Used by device component serializers.
pub struct ComponentNestedModule {
	id: i64,
	display: String,
	url: Url,
	device: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroupRequest {
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvision {
	expires: Option<String>,
	display: String,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	id: i64,
	created: String,
	url: Url,
	last_used: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenRequest {
	expires: Option<String>,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	last_used: Option<String>,
}

pub struct PaginatedRackList {
	previous: Option<Url>,
	results: Vec<Rack>,
	next: Option<Url>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroup {
	display: String,
	name: String,
	url: Url,
	id: i64,
	_depth: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct RouteTargetRequest {
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WebhookRequest {
	custom_fields: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	name: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	description: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	tags: Vec<NestedTagRequest>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvisionRequest {
	password: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	username: String,
	expires: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccountRequest {
	tags: Vec<NestedTagRequest>,
	account: String,
	custom_fields: String,
	description: String,
	comments: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
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
	url: Url,
}

/// Representation of a VLAN which does not exist in the database.
pub struct AvailableVLAN {
	vid: i64,
}

pub struct PaginatedObjectPermissionList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ObjectPermission>,
}

/// Representation of an ASN which does not exist in the database.
pub struct AvailableASN {
	asn: i64,
	description: String,
}

pub struct PaginatedVirtualDeviceContextList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<VirtualDeviceContext>,
}

pub struct PaginatedTunnelList {
	results: Vec<Tunnel>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInterfaceTemplateRequest {
	mgmt_only: bool,
	description: String,
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
	module_type: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	bridge: Option<i64>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableSiteRequest {
	tenant: Option<i64>,
	comments: String,
	/// If different from the physical address
	shipping_address: String,
	custom_fields: String,
	time_zone: Option<String>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Local facility ID or description
	facility: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	region: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	group: Option<i64>,
	/// Full name of the site
	name: String,
	asns: Vec<i64>,
	slug: String,
	/// Physical location of the building
	physical_address: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
}

pub struct PaginatedRackRoleList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<RackRole>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableExportTemplateRequest {
	name: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	description: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	content_types: Vec<String>,
	/// Download file as attachment
	as_attachment: bool,
	/// Remote data source
	data_source: Option<i64>,
}

pub struct PaginatedDataSourceList {
	count: i64,
	previous: Option<Url>,
	results: Vec<DataSource>,
	next: Option<Url>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableWirelessLANGroupRequest {
	name: String,
	custom_fields: String,
	parent: Option<i64>,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
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
pub struct ModuleBayNestedModule {
	id: i64,
	url: Url,
	display: String,
	serial: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroupRequest {
	name: String,
	slug: String,
}

pub struct PaginatedPowerPortList {
	previous: Option<Url>,
	results: Vec<PowerPort>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct VLANGroup {
	display: String,
	utilization: String,
	description: String,
	scope_type: Option<String>,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	tags: Vec<NestedTag>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	id: i64,
	name: String,
	last_updated: Option<String>,
	slug: String,
	vlan_count: i64,
	scope_id: Option<i64>,
	custom_fields: String,
	url: Url,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSet {
	name: String,
	display: String,
	id: i64,
	url: Url,
	choices_count: String,
}

/// Adds support for custom fields and tags.
pub struct IPRangeRequest {
	comments: String,
	description: String,
	start_address: String,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	tags: Vec<NestedTagRequest>,
	end_address: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInterfaceRequest {
	tx_power: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	device: i64,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	mac_address: Option<String>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
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
	speed: Option<i64>,
	wireless_lans: Vec<i64>,
	name: String,
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
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	parent: Option<i64>,
	untagged_vlan: Option<i64>,
	description: String,
	wwn: Option<String>,
	vrf: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
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
	vdcs: Vec<i64>,
	enabled: bool,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	lag: Option<i64>,
	bridge: Option<i64>,
	/// Physical label
	label: String,
	custom_fields: String,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	mtu: Option<i64>,
	tagged_vlans: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelTerminationRequest {
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	tags: Vec<NestedTagRequest>,
	termination_id: Option<i64>,
	tunnel: i64,
	outside_ip: Option<i64>,
	custom_fields: String,
	termination_type: String,
}

/// Used by device component serializers.
pub struct ComponentNestedModuleRequest {
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDeviceContextRequest {
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	primary_ip6: Option<i64>,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	primary_ip4: Option<i64>,
	device: Option<i64>,
	custom_fields: String,
	name: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableRackRequest {
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	facility_id: Option<String>,
	site: i64,
	location: Option<i64>,
	tenant: Option<i64>,
	/// Starting unit for rack
	starting_unit: i64,
	weight: Option<f64>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	name: String,
	/// Functional role
	role: Option<i64>,
	serial: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	description: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	/// Height in rack units
	u_height: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInventoryItemTemplateRequest {
	description: String,
	manufacturer: Option<i64>,
	parent: Option<i64>,
	/// Physical label
	label: String,
	device_type: i64,
	role: Option<i64>,
	component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	component_type: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
}

/// Adds support for custom fields and tags.
pub struct PowerOutlet {
	link_peers: Vec<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	display: String,
	id: i64,
	connected_endpoints_reachable: bool,
	r#type: Option<String>,
	cable_end: String,
	/// Physical label
	label: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	_occupied: bool,
	feed_leg: Option<String>,
	name: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLAN {
	id: i64,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	display: String,
	name: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplateRequest {
	/// Physical label
	label: String,
	color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
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
pub struct PatchedWritableConsolePortRequest {
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	device: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	module: Option<i64>,
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
}

/// Adds support for custom fields and tags.
pub struct Aggregate {
	prefix: String,
	family: String,
	description: String,
	created: Option<String>,
	display: String,
	url: Url,
	comments: String,
	date_added: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	last_updated: Option<String>,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	name: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetwork {
	custom_fields: String,
	created: Option<String>,
	description: String,
	name: String,
	display: String,
	comments: String,
	last_updated: Option<String>,
	service_id: String,
	tags: Vec<NestedTag>,
	id: i64,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRoleRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortRequest {
	cable: Option<i64>,
	name: String,
}

pub struct PaginatedDeviceWithConfigContextList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<DeviceWithConfigContext>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTerminationRequest {
}

/// Adds support for custom fields and tags.
pub struct RearPortRequest {
	color: String,
	description: String,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// Physical label
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableTokenRequest {
	description: String,
	expires: Option<String>,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	last_used: Option<String>,
	user: i64,
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
pub struct NestedInterfaceTemplate {
	id: i64,
	display: String,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturer {
	url: Url,
	id: i64,
	name: String,
	slug: String,
	display: String,
}

pub struct PaginatedASNRangeList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ASNRange>,
}

pub struct PaginatedL2VPNTerminationList {
	count: i64,
	next: Option<Url>,
	results: Vec<L2VPNTermination>,
	previous: Option<Url>,
}

pub struct PaginatedContactAssignmentList {
	count: i64,
	previous: Option<Url>,
	results: Vec<ContactAssignment>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ConsolePortRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
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
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitType {
	id: i64,
	display: String,
	name: String,
	url: Url,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

/// Adds support for custom fields and tags.
pub struct PlatformRequest {
	custom_fields: String,
	slug: String,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct TunnelTerminationRequest {
	termination_id: Option<i64>,
	custom_fields: String,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_type: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct Role {
	tags: Vec<NestedTag>,
	url: Url,
	name: String,
	display: String,
	slug: String,
	custom_fields: String,
	created: Option<String>,
	prefix_count: i64,
	vlan_count: i64,
	id: i64,
	description: String,
	last_updated: Option<String>,
	weight: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldRequest {
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Internal field name
	name: String,
	description: String,
	choice_set: Option<i64>,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
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
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	content_types: Vec<String>,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	object_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLink {
	id: i64,
	url: Url,
	display: String,
	ssid: String,
}

/// Adds support for custom fields and tags.
pub struct Prefix {
	url: Url,
	id: i64,
	children: i64,
	custom_fields: String,
	prefix: String,
	display: String,
	family: String,
	description: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Treat as 100% utilized
	mark_utilized: bool,
	status: String,
	tags: Vec<NestedTag>,
	comments: String,
	last_updated: Option<String>,
	_depth: i64,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLANRequest {
	custom_fields: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	ssid: String,
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
	tenant: Option<i64>,
	auth_psk: String,
	tags: Vec<NestedTagRequest>,
	group: Option<i64>,
	comments: String,
	description: String,
	vlan: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	comments: String,
	tenant: Option<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterface {
	display: String,
	name: String,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct DeviceTypeRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: Option<String>,
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: Option<String>,
	model: String,
	slug: String,
	/// Discrete part number (optional)
	part_number: String,
	u_height: f64,
	comments: String,
	rear_image: String,
	front_image: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetworkRequest {
	description: String,
	comments: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	service_id: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsoleServerPortTemplateRequest {
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
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
}

pub struct PaginatedPlatformList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Platform>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	link: Url,
	name: String,
	group: Option<i64>,
	email: String,
	address: String,
	title: String,
	description: String,
	custom_fields: String,
	phone: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableClusterRequest {
	r#type: i64,
	name: String,
	comments: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	group: Option<i64>,
	tenant: Option<i64>,
	site: Option<i64>,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ModuleBayRequest {
	/// Physical label
	label: String,
	name: String,
	custom_fields: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsolePortTemplateRequest {
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
	module_type: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplate {
	id: i64,
	r#type: String,
	poe_mode: Option<String>,
	/// Physical label
	label: String,
	url: Url,
	description: String,
	mgmt_only: bool,
	poe_type: Option<String>,
	display: String,
	created: Option<String>,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	rf_role: Option<String>,
	enabled: bool,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct LocationRequest {
	custom_fields: String,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTenantRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	comments: String,
	slug: String,
	group: Option<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachineRequest {
	name: String,
}

pub struct PaginatedPowerFeedList {
	results: Vec<PowerFeed>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

pub struct PaginatedWirelessLANList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<WirelessLAN>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
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

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContextRequest {
	custom_fields: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableTenantRequest {
	comments: String,
	name: String,
	group: Option<i64>,
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: String,
	description: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplateRequest {
	/// Jinja2 template code.
	template_code: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldRequest {
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
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
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	object_type: String,
	description: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Internal field name
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Module {
	description: String,
	custom_fields: String,
	status: String,
	last_updated: Option<String>,
	display: String,
	id: i64,
	tags: Vec<NestedTag>,
	url: Url,
	created: Option<String>,
	serial: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVMInterfaceRequest {
	untagged_vlan: Option<i64>,
	tagged_vlans: Vec<i64>,
	name: String,
	parent: Option<i64>,
	bridge: Option<i64>,
	vrf: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	virtual_machine: i64,
	enabled: bool,
	mac_address: Option<String>,
	mtu: Option<i64>,
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
pub struct NestedClusterRequest {
	name: String,
}

pub struct PaginatedVirtualMachineWithConfigContextList {
	results: Vec<VirtualMachineWithConfigContext>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PowerFeedRequest {
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	comments: String,
	voltage: i64,
	amperage: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTagRequest {
	color: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplate {
	last_updated: Option<String>,
	id: i64,
	url: Url,
	ports: Vec<i64>,
	protocol: String,
	custom_fields: String,
	created: Option<String>,
	comments: String,
	display: String,
	name: String,
	tags: Vec<NestedTag>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteRequest {
	/// Full name of the site
	name: String,
	slug: String,
}

pub struct PaginatedExportTemplateList {
	count: i64,
	results: Vec<ExportTemplate>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFrontPortRequest {
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
	device: i64,
	color: String,
	description: String,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	rear_port: i64,
	module: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Token {
	key: String,
	url: Url,
	expires: Option<String>,
	id: i64,
	created: String,
	display: String,
	last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPort {
	name: String,
	url: Url,
	display: String,
	_occupied: bool,
	id: i64,
	cable: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct VLANGroupRequest {
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	scope_type: Option<String>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	custom_fields: String,
	scope_id: Option<i64>,
	name: String,
}

pub struct PaginatedClusterGroupList {
	results: Vec<ClusterGroup>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ProviderRequest {
	/// Full name of the provider
	name: String,
	accounts: Vec<i64>,
	comments: String,
	slug: String,
	custom_fields: String,
	description: String,
	asns: Vec<i64>,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedIPSecProfileList {
	count: i64,
	next: Option<Url>,
	results: Vec<IPSecProfile>,
	previous: Option<Url>,
}

pub struct PaginatedRouteTargetList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<RouteTarget>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProposalRequest {
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	name: String,
	comments: String,
	custom_fields: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableDeviceBayTemplateRequest {
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedContactRoleRequest {
	description: String,
	slug: String,
	name: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignment {
	custom_fields: String,
	priority: String,
	id: i64,
	content_type: String,
	created: Option<String>,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	url: Url,
	object: String,
	display: String,
	object_id: i64,
}

pub struct PaginatedPowerOutletList {
	results: Vec<PowerOutlet>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedRIRList {
	previous: Option<Url>,
	count: i64,
	results: Vec<RIR>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignmentRequest {
	priority: i64,
	interface_id: i64,
	interface_type: String,
}

/// Adds support for custom fields and tags.
pub struct WritableContactAssignmentRequest {
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	custom_fields: String,
	content_type: String,
	tags: Vec<NestedTagRequest>,
	contact: i64,
	role: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerPanel {
	name: String,
	id: i64,
	custom_fields: String,
	display: String,
	url: Url,
	description: String,
	comments: String,
	tags: Vec<NestedTag>,
	powerfeed_count: i64,
	created: Option<String>,
	last_updated: Option<String>,
}

pub struct Job {
	name: String,
	object_id: Option<i64>,
	url: Url,
	completed: Option<String>,
	status: String,
	object_type: String,
	error: String,
	job_id: String,
	started: Option<String>,
	created: String,
	id: i64,
	/// Recurrence interval (in minutes)
	interval: Option<i64>,
	scheduled: Option<String>,
	display: String,
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
pub struct NestedRearPortTemplate {
	display: String,
	id: i64,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDevice {
	id: i64,
	url: Url,
	display: String,
	name: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PowerPort {
	created: Option<String>,
	r#type: Option<String>,
	connected_endpoints: Vec<String>,
	display: String,
	/// Physical label
	label: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	cable_end: String,
	last_updated: Option<String>,
	connected_endpoints_reachable: bool,
	_occupied: bool,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	custom_fields: String,
	id: i64,
	name: String,
	tags: Vec<NestedTag>,
	connected_endpoints_type: String,
	url: Url,
	description: String,
	link_peers: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceRoleRequest {
	color: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	config_template: Option<i64>,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePrefixRequest {
	tenant: Option<i64>,
	site: Option<i64>,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	prefix: String,
	/// The primary function of this prefix
	role: Option<i64>,
	vrf: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Treat as 100% utilized
	mark_utilized: bool,
	vlan: Option<i64>,
}

pub struct PaginatedClusterTypeList {
	previous: Option<Url>,
	results: Vec<ClusterType>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct CableTermination {
	created: Option<String>,
	termination_id: i64,
	termination_type: String,
	id: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	last_updated: Option<String>,
	cable: i64,
	display: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct DataSource {
	id: i64,
	status: String,
	url: Url,
	file_count: i64,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	name: String,
	last_updated: Option<String>,
	display: String,
	comments: String,
	r#type: String,
	enabled: bool,
	created: Option<String>,
	source_url: String,
	description: String,
}

pub struct PaginatedModuleTypeList {
	previous: Option<Url>,
	results: Vec<ModuleType>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ManufacturerRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetworkRequest {
	name: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplate {
	display: String,
	/// Jinja2 template code.
	template_code: String,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	url: Url,
	/// Path to remote file (relative to data source root)
	data_path: String,
	name: String,
	data_synced: Option<String>,
	tags: Vec<NestedTag>,
	description: String,
}

pub struct PaginatedCableList {
	previous: Option<Url>,
	results: Vec<Cable>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedCircuitTypeRequest {
	color: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	interface_type: String,
	group: i64,
	priority: i64,
	interface_id: i64,
}

pub struct PaginatedWirelessLinkList {
	count: i64,
	previous: Option<Url>,
	results: Vec<WirelessLink>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct TunnelTermination {
	termination_type: String,
	termination_id: Option<i64>,
	url: Url,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	id: i64,
	role: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLinkRequest {
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	auth_psk: String,
	comments: String,
	ssid: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableFHRPGroupAssignmentRequest {
	priority: i64,
	interface_type: String,
	group: i64,
	interface_id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEProposalRequest {
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	name: String,
	custom_fields: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
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
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
}

/// Adds support for custom fields and tags.
pub struct ContactRoleRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	slug: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLink {
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	display: String,
	/// Jinja2 template code for link URL
	link_url: String,
	id: i64,
	weight: i64,
	url: Url,
	enabled: bool,
	/// Jinja2 template code for link text
	link_text: String,
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
	/// Force link to open in a new window
	new_window: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegionRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedVLANGroupRequest {
	scope_id: Option<i64>,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	custom_fields: String,
	description: String,
	scope_type: Option<String>,
	tags: Vec<NestedTagRequest>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	name: String,
	slug: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableTenantGroupRequest {
	parent: Option<i64>,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Region {
	created: Option<String>,
	site_count: i64,
	display: String,
	_depth: i64,
	description: String,
	url: Url,
	id: i64,
	tags: Vec<NestedTag>,
	name: String,
	slug: String,
	custom_fields: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassisRequest {
	domain: String,
	custom_fields: String,
	name: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProposalRequest {
	name: String,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackReservationRequest {
	units: Vec<i64>,
	user: i64,
	custom_fields: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	rack: i64,
	tenant: Option<i64>,
}

pub struct PaginatedCircuitTypeList {
	results: Vec<CircuitType>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderAccountRequest {
	description: String,
	provider: i64,
	comments: String,
	account: String,
	name: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetwork {
	id: i64,
	display: String,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct TenantRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
	comments: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedGroupRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassis {
	url: Url,
	id: i64,
	name: String,
	display: String,
}

pub struct PaginatedVRFList {
	count: i64,
	results: Vec<VRF>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceRequest {
	name: String,
	ports: Vec<i64>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	device: Option<i64>,
	description: String,
	custom_fields: String,
	comments: String,
	virtual_machine: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ClusterTypeRequest {
	name: String,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterGroupRequest {
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableEventRuleRequest {
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	description: String,
	action_object_id: Option<i64>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	content_types: Vec<String>,
	/// Triggers when a matching object is updated.
	type_update: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	action_object_type: String,
	name: String,
	enabled: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
}

/// Adds support for custom fields and tags.
pub struct Cluster {
	custom_fields: String,
	description: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	status: String,
	comments: String,
	url: Url,
	name: String,
	device_count: i64,
	id: i64,
	display: String,
	virtualmachine_count: i64,
	created: Option<String>,
}

pub struct PaginatedConfigTemplateList {
	results: Vec<ConfigTemplate>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedProviderNetworkList {
	results: Vec<ProviderNetwork>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDiskRequest {
	virtual_machine: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	size: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRearPortRequest {
	/// Physical label
	label: String,
	module: Option<i64>,
	color: String,
	/// Number of front ports which may be mapped
	positions: i64,
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
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachment {
	id: i64,
	content_type: String,
	url: Url,
	object_id: i64,
	created: Option<String>,
	image_height: i64,
	image_width: i64,
	name: String,
	display: String,
	last_updated: Option<String>,
	image: Url,
}

pub struct PaginatedIPRangeList {
	count: i64,
	results: Vec<IPRange>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModuleRequest {
	serial: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLANRequest {
	custom_fields: String,
	auth_psk: String,
	description: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tenant: Option<i64>,
	vlan: Option<i64>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	group: Option<i64>,
	ssid: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
}

pub struct PaginatedWirelessLANGroupList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<WirelessLANGroup>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableDataSourceRequest {
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	r#type: String,
	enabled: bool,
	comments: String,
	name: String,
	source_url: String,
	description: String,
}

pub struct PaginatedDeviceBayList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceBay>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecPolicy {
	url: Url,
	id: i64,
	display: String,
	name: String,
}

pub struct PaginatedCableTerminationList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<CableTermination>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegion {
	name: String,
	display: String,
	_depth: i64,
	slug: String,
	url: Url,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedSavedFilterRequest {
	content_types: Vec<String>,
	weight: i64,
	slug: String,
	shared: bool,
	description: String,
	name: String,
	user: Option<i64>,
	enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitTerminationRequest {
	provider_network: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
	site: Option<i64>,
	custom_fields: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	tags: Vec<NestedTagRequest>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	circuit: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicy {
	url: Url,
	id: i64,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceRoleRequest {
	tags: Vec<NestedTagRequest>,
	config_template: Option<i64>,
	name: String,
	description: String,
	color: String,
	slug: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicy {
	version: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	description: String,
	url: Url,
	created: Option<String>,
	preshared_key: String,
	mode: String,
	name: String,
	last_updated: Option<String>,
	proposals: Vec<i64>,
	comments: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceBay {
	created: Option<String>,
	display: String,
	url: Url,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	id: i64,
	custom_fields: String,
	description: String,
	/// Physical label
	label: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroup {
	last_updated: Option<String>,
	auth_key: String,
	custom_fields: String,
	group_id: i64,
	display: String,
	name: String,
	description: String,
	created: Option<String>,
	comments: String,
	tags: Vec<NestedTag>,
	ip_addresses: Vec<NestedIPAddress>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	url: Url,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNTerminationRequest {
	custom_fields: String,
	l2vpn: i64,
	assigned_object_id: i64,
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ContactRole {
	id: i64,
	url: Url,
	name: String,
	slug: String,
	last_updated: Option<String>,
	display: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	description: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct InterfaceRequest {
	custom_fields: String,
	wireless_lans: Vec<i64>,
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
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	name: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// Physical label
	label: String,
	vdcs: Vec<i64>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	speed: Option<i64>,
	mtu: Option<i64>,
	tags: Vec<NestedTagRequest>,
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
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	mac_address: Option<String>,
	enabled: bool,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	tx_power: Option<i64>,
	tagged_vlans: Vec<i64>,
	wwn: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderAccountRequest {
	name: String,
	account: String,
	provider: i64,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplateRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCableRequest {
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRole {
	name: String,
	display: String,
	id: i64,
	slug: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplate {
	id: i64,
	url: Url,
	created: Option<String>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Physical label
	label: String,
	last_updated: Option<String>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	r#type: Option<String>,
	description: String,
	display: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroup {
	url: Url,
	site_count: i64,
	id: i64,
	last_updated: Option<String>,
	name: String,
	slug: String,
	description: String,
	display: String,
	custom_fields: String,
	_depth: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

pub struct PaginatedIPAddressList {
	count: i64,
	results: Vec<IPAddress>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassis {
	name: String,
	display: String,
	description: String,
	comments: String,
	created: Option<String>,
	last_updated: Option<String>,
	custom_fields: String,
	member_count: i64,
	id: i64,
	url: Url,
	tags: Vec<NestedTag>,
	domain: String,
}

pub struct PaginatedContactList {
	count: i64,
	results: Vec<Contact>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct User {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	display: String,
	first_name: String,
	id: i64,
	last_name: String,
	email: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	date_joined: String,
	groups: Vec<i64>,
	url: Url,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContext {
	last_updated: Option<String>,
	name: String,
	disk: Option<i64>,
	created: Option<String>,
	interface_count: i64,
	description: String,
	url: Url,
	id: i64,
	status: String,
	comments: String,
	tags: Vec<NestedTag>,
	virtual_disk_count: i64,
	memory: Option<i64>,
	vcpus: Option<f64>,
	display: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableContactRequest {
	email: String,
	tags: Vec<NestedTagRequest>,
	address: String,
	title: String,
	comments: String,
	custom_fields: String,
	link: Url,
	phone: String,
	group: Option<i64>,
	description: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct BookmarkRequest {
	object_id: i64,
	object_type: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterType {
	slug: String,
	url: Url,
	display: String,
	name: String,
	description: String,
	created: Option<String>,
	cluster_count: i64,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct IKEProposalRequest {
	tags: Vec<NestedTagRequest>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	name: String,
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
	group: i64,
	comments: String,
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
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupRequest {
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	group_id: i64,
	auth_key: String,
	custom_fields: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBay {
	id: i64,
	url: Url,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItem {
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	_depth: i64,
	url: Url,
	display: String,
	component_type: Option<String>,
	component_id: Option<i64>,
	created: Option<String>,
	id: i64,
	last_updated: Option<String>,
	custom_fields: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	serial: String,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// This item was automatically discovered
	discovered: bool,
	name: String,
	description: String,
}

pub struct ContentType {
	url: Url,
	id: i64,
	display: String,
	app_label: String,
	model: String,
}

/// Adds support for custom fields and tags.
pub struct Contact {
	display: String,
	name: String,
	address: String,
	tags: Vec<NestedTag>,
	phone: String,
	url: Url,
	last_updated: Option<String>,
	email: String,
	created: Option<String>,
	title: String,
	description: String,
	custom_fields: String,
	comments: String,
	link: Url,
	id: i64,
}

pub struct PaginatedContactRoleList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ContactRole>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPortRequest {
	custom_fields: String,
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
	name: String,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	/// Full name of the provider
	name: String,
	accounts: Vec<i64>,
	description: String,
	asns: Vec<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableTenantGroupRequest {
	parent: Option<i64>,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

pub struct PaginatedJobList {
	count: i64,
	previous: Option<Url>,
	results: Vec<Job>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

pub struct Dashboard {
}

pub struct DashboardRequest {
}

pub struct PaginatedRearPortList {
	count: i64,
	next: Option<Url>,
	results: Vec<RearPort>,
	previous: Option<Url>,
}

pub struct PaginatedFHRPGroupList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<FHRPGroup>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroupRequest {
	slug: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

pub struct PaginatedDataFileList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DataFile>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableRearPortTemplateRequest {
	positions: i64,
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
	/// Physical label
	label: String,
	device_type: Option<i64>,
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	tenant: Option<i64>,
	vcpus: Option<f64>,
	tags: Vec<NestedTagRequest>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	name: String,
	comments: String,
	cluster: Option<i64>,
	custom_fields: String,
	device: Option<i64>,
	site: Option<i64>,
	memory: Option<i64>,
	primary_ip4: Option<i64>,
	role: Option<i64>,
	platform: Option<i64>,
	primary_ip6: Option<i64>,
	disk: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
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
pub struct WritableCircuitTerminationRequest {
	site: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	provider_network: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	custom_fields: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	circuit: i64,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IPAddress {
	created: Option<String>,
	url: Url,
	display: String,
	last_updated: Option<String>,
	nat_outside: Vec<NestedIPAddress>,
	family: String,
	role: String,
	comments: String,
	description: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	assigned_object_type: Option<String>,
	assigned_object_id: Option<i64>,
	tags: Vec<NestedTag>,
	id: i64,
	custom_fields: String,
	status: String,
	address: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroupRequest {
	slug: String,
	name: String,
}

pub struct PaginatedDeviceRoleList {
	previous: Option<Url>,
	results: Vec<DeviceRole>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableConsoleServerPortRequest {
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
	tags: Vec<NestedTagRequest>,
	module: Option<i64>,
	device: i64,
	name: String,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

pub struct PaginatedDeviceTypeList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceType>,
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
	device: i64,
	module_bay: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	module_type: i64,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContext {
	interface_count: i64,
	comments: String,
	name: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	status: String,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	display: String,
	url: Url,
	id: i64,
	last_updated: Option<String>,
	created: Option<String>,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPort {
	id: i64,
	name: String,
	description: String,
	url: Url,
	/// Physical label
	label: String,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLinkRequest {
	ssid: String,
}

pub struct PaginatedPrefixList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Prefix>,
}

/// Adds support for custom fields and tags.
pub struct L2VPNRequest {
	export_targets: Vec<i64>,
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
	import_targets: Vec<i64>,
	description: String,
	custom_fields: String,
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerOutletTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
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
	module_type: Option<i64>,
	/// Physical label
	label: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct RegionRequest {
	custom_fields: String,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContext {
	tenant_groups: Vec<i64>,
	cluster_groups: Vec<i64>,
	tags: Vec<String>,
	url: Url,
	/// Path to remote file (relative to data source root)
	data_path: String,
	platforms: Vec<i64>,
	description: String,
	weight: i64,
	device_types: Vec<i64>,
	display: String,
	cluster_types: Vec<i64>,
	created: Option<String>,
	data_synced: Option<String>,
	last_updated: Option<String>,
	tenants: Vec<i64>,
	regions: Vec<i64>,
	locations: Vec<i64>,
	id: i64,
	name: String,
	site_groups: Vec<i64>,
	clusters: Vec<i64>,
	roles: Vec<i64>,
	is_active: bool,
	sites: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSite {
	url: Url,
	display: String,
	id: i64,
	/// Full name of the site
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct FrontPort {
	custom_fields: String,
	display: String,
	id: i64,
	r#type: String,
	description: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	name: String,
	tags: Vec<NestedTag>,
	cable_end: String,
	link_peers: Vec<String>,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	_occupied: bool,
	created: Option<String>,
	url: Url,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitTypeRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignmentRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	content_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVLANRequest {
	name: String,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// The primary function of this VLAN
	role: Option<i64>,
	/// VLAN group (optional)
	group: Option<i64>,
	description: String,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	comments: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplate {
	/// Physical label
	label: String,
	r#type: Option<String>,
	url: Url,
	id: i64,
	feed_leg: Option<String>,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanel {
	name: String,
	display: String,
	url: Url,
	id: i64,
}

pub struct PaginatedTenantGroupList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<TenantGroup>,
	count: i64,
}

pub struct PaginatedConsoleServerPortList {
	count: i64,
	previous: Option<Url>,
	results: Vec<ConsoleServerPort>,
	next: Option<Url>,
}

pub struct PaginatedL2VPNList {
	results: Vec<L2VPN>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Group {
	display: String,
	user_count: i64,
	id: i64,
	name: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct ModuleRequest {
	custom_fields: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	comments: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	serial: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenant {
	slug: String,
	id: i64,
	name: String,
	display: String,
	url: Url,
}

pub struct PaginatedConfigContextList {
	count: i64,
	previous: Option<Url>,
	results: Vec<ConfigContext>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroup {
	url: Url,
	id: i64,
	name: String,
	display: String,
	slug: String,
	_depth: i64,
}

pub struct PaginatedInterfaceTemplateList {
	previous: Option<Url>,
	results: Vec<InterfaceTemplate>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUser {
	display: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	url: Url,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceTypeRequest {
	u_height: f64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	weight: Option<f64>,
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	description: String,
	manufacturer: i64,
	comments: String,
	rear_image: String,
	tags: Vec<NestedTagRequest>,
	default_platform: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	custom_fields: String,
	slug: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	front_image: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedFHRPGroupRequest {
	group_id: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	description: String,
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
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplate {
	display: String,
	id: i64,
	name: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableModuleBayTemplateRequest {
	device_type: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleTypeRequest {
	model: String,
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	comments: String,
	manufacturer: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Discrete part number (optional)
	part_number: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterGroupRequest {
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: String,
	description: String,
	name: String,
}

pub struct PaginatedCircuitList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Circuit>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedRackRoleRequest {
	color: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	description: String,
	custom_fields: String,
}

pub struct PaginatedImageAttachmentList {
	count: i64,
	previous: Option<Url>,
	results: Vec<ImageAttachment>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderNetworkRequest {
	custom_fields: String,
	service_id: String,
	provider: i64,
	name: String,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedObjectChangeList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ObjectChange>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPanelRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	location: Option<i64>,
	description: String,
	site: i64,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPRangeRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	/// The primary function of this range
	role: Option<i64>,
	tenant: Option<i64>,
	vrf: Option<i64>,
	comments: String,
	description: String,
	start_address: String,
	end_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
}

/// Adds support for custom fields and tags.
pub struct Rack {
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Height in rack units
	u_height: i64,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	custom_fields: String,
	facility_id: Option<String>,
	weight_unit: Option<String>,
	outer_unit: Option<String>,
	description: String,
	r#type: Option<String>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	comments: String,
	weight: Option<f64>,
	tags: Vec<NestedTag>,
	status: String,
	name: String,
	width: String,
	device_count: i64,
	powerfeed_count: i64,
	id: i64,
	url: Url,
	serial: String,
	last_updated: Option<String>,
	/// Starting unit for rack
	starting_unit: i64,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	created: Option<String>,
	display: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableUserRequest {
	last_name: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	first_name: String,
	password: String,
	date_joined: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	email: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelRequest {
	custom_fields: String,
	ipsec_profile: Option<i64>,
	tunnel_id: Option<i64>,
	description: String,
	comments: String,
	group: Option<i64>,
	tenant: Option<i64>,
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
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedInterfaceList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<Interface>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContact {
	id: i64,
	display: String,
	url: Url,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplate {
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	url: Url,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	description: String,
	display: String,
}

pub struct PaginatedLocationList {
	results: Vec<Location>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RIR {
	description: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	aggregate_count: i64,
	custom_fields: String,
	created: Option<String>,
	url: Url,
	display: String,
	slug: String,
	id: i64,
	name: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerFeedRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	voltage: i64,
	comments: String,
	amperage: i64,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	rack: Option<i64>,
	custom_fields: String,
	power_panel: i64,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableAggregateRequest {
	comments: String,
	tenant: Option<i64>,
	description: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	custom_fields: String,
	prefix: String,
	date_added: Option<String>,
	tags: Vec<NestedTagRequest>,
}

pub struct ObjectChange {
	url: Url,
	request_id: String,
	action: String,
	id: i64,
	user_name: String,
	changed_object_id: i64,
	time: String,
	display: String,
	changed_object_type: String,
}

pub struct PaginatedConsolePortList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ConsolePort>,
}

/// Adds support for custom fields and tags.
pub struct Service {
	ports: Vec<i64>,
	ipaddresses: Vec<i64>,
	protocol: String,
	name: String,
	custom_fields: String,
	last_updated: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	id: i64,
	display: String,
	created: Option<String>,
	url: Url,
	comments: String,
}

pub struct PaginatedModuleList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Module>,
}

/// Adds support for custom fields and tags.
pub struct IPSecProposal {
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	url: Url,
	description: String,
	name: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	encryption_algorithm: String,
	id: i64,
	created: Option<String>,
	last_updated: Option<String>,
	authentication_algorithm: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerOutletRequest {
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	power_port: Option<i64>,
	custom_fields: String,
	name: String,
	description: String,
	device: i64,
	/// Physical label
	label: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
}

pub struct PaginatedBookmarkList {
	previous: Option<Url>,
	results: Vec<Bookmark>,
	count: i64,
	next: Option<Url>,
}

pub struct PaginatedInventoryItemList {
	count: i64,
	next: Option<Url>,
	results: Vec<InventoryItem>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRF {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	url: Url,
	display: String,
	id: i64,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableRegionRequest {
	description: String,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	parent: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ClusterGroup {
	url: Url,
	name: String,
	display: String,
	last_updated: Option<String>,
	cluster_count: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	slug: String,
	created: Option<String>,
	description: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct RouteTarget {
	id: i64,
	created: Option<String>,
	comments: String,
	url: Url,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	custom_fields: String,
	display: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicyRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilter {
	display: String,
	url: Url,
	name: String,
	user: Option<i64>,
	description: String,
	created: Option<String>,
	slug: String,
	content_types: Vec<String>,
	shared: bool,
	weight: i64,
	enabled: bool,
	id: i64,
	last_updated: Option<String>,
}

pub struct PaginatedConsoleServerPortTemplateList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ConsoleServerPortTemplate>,
}

/// Adds support for custom fields and tags.
pub struct PatchedManufacturerRequest {
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableBookmarkRequest {
	user: i64,
	object_type: String,
	object_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Bookmark {
	url: Url,
	object_type: String,
	id: i64,
	display: String,
	object_id: i64,
	created: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedRoleRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	slug: String,
	description: String,
	weight: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableLocationRequest {
	name: String,
	site: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	slug: String,
	description: String,
	parent: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleRequest {
	comments: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	custom_fields: String,
	serial: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	module_bay: i64,
	module_type: i64,
	device: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableObjectPermissionRequest {
	groups: Vec<i64>,
	name: String,
	object_types: Vec<String>,
	description: String,
	enabled: bool,
	users: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplate {
	description: String,
	component_type: Option<String>,
	component_id: Option<i64>,
	id: i64,
	last_updated: Option<String>,
	created: Option<String>,
	display: String,
	_depth: i64,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	parent: Option<i64>,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PowerPortRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	custom_fields: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
}

pub struct PaginatedInventoryItemTemplateList {
	count: i64,
	previous: Option<Url>,
	results: Vec<InventoryItemTemplate>,
	next: Option<Url>,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObject {
	object_id: i64,
	object_type: String,
}

/// Adds support for custom fields and tags.
pub struct ContactRequest {
	description: String,
	title: String,
	phone: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	email: String,
	comments: String,
	name: String,
	link: Url,
	address: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfile {
	custom_fields: String,
	last_updated: Option<String>,
	comments: String,
	id: i64,
	url: Url,
	description: String,
	tags: Vec<NestedTag>,
	display: String,
	name: String,
	mode: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct DeviceBayRequest {
	custom_fields: String,
	description: String,
	name: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	description: String,
	r#type: String,
	created: Option<String>,
	id: i64,
	url: Url,
	last_updated: Option<String>,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSource {
	id: i64,
	url: Url,
	display: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassisRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTermination {
	url: Url,
	/// Physical circuit speed
	port_speed: Option<i64>,
	display: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	id: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDiskRequest {
	virtual_machine: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
	size: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceTemplateRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ports: Vec<i64>,
	name: String,
	description: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNRequest {
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
	identifier: Option<i64>,
	name: String,
	comments: String,
	custom_fields: String,
	description: String,
	import_targets: Vec<i64>,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableSiteRequest {
	group: Option<i64>,
	/// If different from the physical address
	shipping_address: String,
	custom_fields: String,
	tenant: Option<i64>,
	region: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Full name of the site
	name: String,
	/// Local facility ID or description
	facility: String,
	asns: Vec<i64>,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// Physical location of the building
	physical_address: String,
	slug: String,
	time_zone: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableDeviceBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	device_type: i64,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPNTerminationRequest {
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	custom_fields: String,
}

pub struct PaginatedSiteList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Site>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplateRequest {
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: Option<String>,
	description: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

pub struct PaginatedCustomLinkList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<CustomLink>,
}


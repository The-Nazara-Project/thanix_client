use serde_qs;
use serde_json;
use reqwest::Url;
use crate::util::ThanixClient;
/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleType {
	 pub custom_fields: serde_json::value::Value,
	 pub id: i64,
	 pub url: String,
	 pub model: String,
	 pub comments: String,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	/// Discrete part number (optional)
	 pub part_number: String,
	 pub weight: Option<f64>,
	 pub weight_unit: Option<serde_json::value::Value>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCircuitTypeRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub description: String,
	 pub name: String,
	 pub color: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedL2VPNTerminationList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<L2VPNTermination>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASN {
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub site_count: i64,
	 pub id: i64,
	 pub display: String,
	/// 16- or 32-bit autonomous system number
	 pub asn: i64,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub provider_count: i64,
	 pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPAddressList {
	 pub results: Vec<IPAddress>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderAccount {
	 pub url: String,
	 pub id: i64,
	 pub display: String,
	 pub name: String,
	 pub account: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDeviceContext {
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub created: Option<String>,
	 pub url: String,
	 pub comments: String,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub id: i64,
	/// Numeric identifier unique to the parent device
	 pub identifier: Option<i64>,
	 pub status: serde_json::value::Value,
	 pub name: String,
	 pub interface_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRearPortRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub module: Option<i64>,
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
	 pub r#type: String,
	/// Physical label
	 pub label: String,
	 pub color: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Number of front ports which may be mapped
	 pub positions: i64,
	 pub device: i64,
	 pub description: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceWithConfigContext {
	 pub url: String,
	 pub comments: String,
	 pub inventory_item_count: i64,
	/// Chassis serial number, assigned by the manufacturer
	 pub serial: String,
	 pub vc_position: Option<i64>,
	 pub device_bay_count: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub front_port_count: i64,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub id: i64,
	 pub rear_port_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub status: serde_json::value::Value,
	 pub last_updated: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	 pub display: String,
	 pub name: Option<String>,
	 pub module_bay_count: i64,
	 pub power_port_count: i64,
	 pub created: Option<String>,
	 pub description: String,
	 pub console_port_count: i64,
	 pub airflow: serde_json::value::Value,
	 pub power_outlet_count: i64,
	 pub interface_count: i64,
	 pub console_server_port_count: i64,
	 pub position: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	/// Virtual chassis master election priority
	 pub vc_priority: Option<i64>,
	 pub face: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProfileRequest {
	 pub description: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// * `esp` - ESP
	/// * `ah` - AH
	 pub mode: String,
	 pub comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleTypeRequest {
	 pub model: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleBayRequest {
	 pub installed_module: i64,
	 pub tags: Vec<NestedTagRequest>,
	/// Identifier to reference when renaming installed components
	 pub position: String,
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
	 pub device: i64,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVLANRequest {
	/// VLAN group (optional)
	 pub group: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	/// Numeric VLAN ID (1-4094)
	 pub vid: i64,
	 pub comments: String,
	/// The specific site to which this VLAN is assigned (if any)
	 pub site: Option<i64>,
	 pub tenant: Option<i64>,
	/// The primary function of this VLAN
	 pub role: Option<i64>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterTypeRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub name: String,
	 pub slug: String,
}

/// Representation of a prefix which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailablePrefix {
	 pub family: i64,
	 pub prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPortList {
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub results: Vec<PowerPort>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Provider {
	 pub accounts: Vec<i64>,
	 pub slug: String,
	 pub url: String,
	/// Full name of the provider
	 pub name: String,
	 pub comments: String,
	 pub display: String,
	 pub asns: Vec<i64>,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub circuit_count: i64,
	 pub description: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePlatformRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub config_template: Option<i64>,
	 pub slug: String,
	 pub name: String,
	 pub description: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	 pub manufacturer: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleType {
	 pub url: String,
	 pub model: String,
	 pub id: i64,
	 pub display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderNetworkRequest {
	 pub provider: i64,
	 pub description: String,
	 pub service_id: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceType {
	 pub id: i64,
	 pub url: String,
	 pub slug: String,
	 pub model: String,
	 pub display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInterfaceTemplateRequest {
	/// Physical label
	 pub label: String,
	 pub bridge: Option<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	 pub rf_role: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	 pub poe_type: String,
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
	 pub r#type: String,
	 pub enabled: bool,
	 pub description: String,
	 pub device_type: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	 pub poe_mode: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub mgmt_only: bool,
	 pub module_type: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRearPortRequest {
	 pub module: Option<i64>,
	/// Physical label
	 pub label: String,
	 pub color: String,
	 pub device: i64,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub description: String,
	/// Number of front ports which may be mapped
	 pub positions: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
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
	 pub r#type: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceTypeRequest {
	/// Discrete part number (optional)
	 pub part_number: String,
	/// Devices of this type are excluded when calculating rack utilization.
	 pub exclude_from_utilization: bool,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	 pub subdevice_role: String,
	 pub slug: String,
	 pub custom_fields: serde_json::value::Value,
	 pub manufacturer: i64,
	 pub front_image: String,
	 pub default_platform: Option<i64>,
	/// Device consumes both front and rear rack faces.
	 pub is_full_depth: bool,
	 pub model: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	 pub airflow: String,
	 pub description: String,
	 pub weight: Option<f64>,
	 pub comments: String,
	 pub rear_image: String,
	 pub u_height: f64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedImageAttachmentRequest {
	 pub object_id: i64,
	 pub name: String,
	 pub content_type: String,
	 pub image_height: i64,
	 pub image_width: i64,
	 pub image: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AggregateRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub date_added: Option<String>,
	 pub prefix: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRIRRequest {
	 pub name: String,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderNetworkRequest {
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCustomFieldRequest {
	/// Maximum allowed value (for numeric fields)
	 pub validation_maximum: Option<i64>,
	/// Fields with higher weights appear lower in a form.
	 pub weight: i64,
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
	/// Internal field name
	 pub name: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	 pub required: bool,
	 pub choice_set: Option<i64>,
	/// Replicate this value when cloning objects
	 pub is_cloneable: bool,
	 pub description: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	 pub search_weight: i64,
	/// Minimum allowed value (for numeric fields)
	 pub validation_minimum: Option<i64>,
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
	 pub content_types: Vec<String>,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	 pub filter_logic: String,
	 pub object_type: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	 pub validation_regex: String,
	/// Custom fields within the same group will be displayed together
	 pub group_name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	 pub label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIKEProposalRequest {
	 pub name: String,
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
	 pub group: i64,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	 pub encryption_algorithm: String,
	 pub custom_fields: serde_json::value::Value,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	 pub authentication_method: String,
	/// Security association lifetime (in seconds)
	 pub sa_lifetime: Option<i64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	 pub authentication_algorithm: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Webhook {
	 pub tags: Vec<NestedTag>,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	 pub ca_file_path: Option<String>,
	 pub display: String,
	 pub description: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	 pub http_method: String,
	 pub last_updated: Option<String>,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	 pub additional_headers: String,
	 pub id: i64,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	 pub http_content_type: String,
	 pub created: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	 pub secret: String,
	 pub url: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	 pub payload_url: String,
	/// Enable SSL certificate verification. Disable with caution!
	 pub ssl_verification: bool,
	 pub custom_fields: serde_json::value::Value,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	 pub body_template: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsoleServerPortTemplateList {
	 pub results: Vec<ConsoleServerPortTemplate>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuit {
	 pub url: String,
	 pub display: String,
	/// Unique circuit ID
	 pub cid: String,
	 pub id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnel {
	 pub id: i64,
	 pub display: String,
	 pub url: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderNetworkList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<ProviderNetwork>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortTemplate {
	 pub color: String,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	 pub display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub url: String,
	 pub rear_port_position: i64,
	 pub r#type: serde_json::value::Value,
	 pub id: i64,
	/// Physical label
	 pub label: String,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedUserList {
	 pub previous: Option<String>,
	 pub results: Vec<User>,
	 pub next: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterType {
	 pub slug: String,
	 pub url: String,
	 pub display: String,
	 pub cluster_count: i64,
	 pub last_updated: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
	 pub id: i64,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePlatformRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	 pub manufacturer: Option<i64>,
	 pub config_template: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayRequest {
	/// Physical label
	 pub label: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatformRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub description: String,
	 pub slug: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactAssignmentRequest {
	 pub role: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub content_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	 pub priority: String,
	 pub contact: i64,
	 pub object_id: i64,
	 pub tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
	 pub url: String,
	 pub first_name: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	 pub is_active: bool,
	 pub date_joined: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	 pub username: String,
	 pub email: String,
	 pub display: String,
	/// Designates whether the user can log into this admin site.
	 pub is_staff: bool,
	 pub id: i64,
	 pub last_name: String,
	 pub groups: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceBayRequest {
	 pub description: String,
	 pub name: String,
	 pub device: i64,
	/// Physical label
	 pub label: String,
	 pub installed_device: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProfile {
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub id: i64,
	 pub url: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub name: String,
	 pub display: String,
	 pub mode: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableUserRequest {
	 pub date_joined: String,
	 pub password: String,
	 pub first_name: String,
	 pub last_name: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	 pub is_active: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	 pub username: String,
	 pub email: String,
	/// Designates whether the user can log into this admin site.
	 pub is_staff: bool,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	 pub groups: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactAssignmentList {
	 pub previous: Option<String>,
	 pub results: Vec<ContactAssignment>,
	 pub next: Option<String>,
	 pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<Circuit>,
	 pub previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactRoleList {
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<ContactRole>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedObjectChangeList {
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<ObjectChange>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitTypeRequest {
	 pub name: String,
	 pub slug: String,
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
	 pub r#type: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRearPortList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<RearPort>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableJournalEntryRequest {
	 pub assigned_object_type: String,
	 pub assigned_object_id: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	 pub kind: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerOutletTemplateList {
	 pub count: i64,
	 pub results: Vec<PowerOutletTemplate>,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPN {
	 pub name: String,
	 pub slug: String,
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
	 pub r#type: String,
	 pub identifier: Option<i64>,
	 pub url: String,
	 pub display: String,
	 pub id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletTemplateRequest {
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
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
	 pub r#type: Option<String>,
	/// Physical label
	 pub label: String,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	 pub feed_leg: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tag {
	 pub display: String,
	 pub id: i64,
	 pub created: Option<String>,
	 pub slug: String,
	 pub name: String,
	 pub description: String,
	 pub object_types: Vec<String>,
	 pub url: String,
	 pub color: String,
	 pub tagged_items: i64,
	 pub last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecProposalRequest {
	 pub name: String,
	 pub comments: String,
	/// Security association lifetime (seconds)
	 pub sa_lifetime_seconds: Option<i64>,
	/// Security association lifetime (in kilobytes)
	 pub sa_lifetime_data: Option<i64>,
	 pub description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	 pub encryption_algorithm: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	 pub authentication_algorithm: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerOutletTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
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
	 pub r#type: String,
	/// Physical label
	 pub label: String,
	 pub device_type: Option<i64>,
	 pub description: String,
	 pub power_port: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	 pub feed_leg: String,
	 pub module_type: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNRequest {
	 pub identifier: Option<i64>,
	 pub slug: String,
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
	 pub r#type: String,
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceTemplateRequest {
	/// Physical label
	 pub label: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	 pub poe_type: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub mgmt_only: bool,
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
	 pub r#type: String,
	 pub description: String,
	 pub enabled: bool,
	/// * `pd` - PD
	/// * `pse` - PSE
	 pub poe_mode: Option<String>,
	/// * `ap` - Access point
	/// * `station` - Station
	 pub rf_role: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
	/// Identifier to reference when renaming installed components
	 pub position: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantGroup {
	 pub slug: String,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub description: String,
	 pub tenant_count: i64,
	 pub _depth: i64,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub display: String,
	 pub name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRegionRequest {
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub slug: String,
	 pub parent: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPortTemplate {
	 pub url: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub description: String,
	/// Physical label
	 pub label: String,
	 pub created: Option<String>,
	 pub id: i64,
	 pub display: String,
	 pub last_updated: Option<String>,
	 pub r#type: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVLANGroupList {
	 pub count: i64,
	 pub previous: Option<String>,
	 pub results: Vec<VLANGroup>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Role {
	 pub prefix_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub vlan_count: i64,
	 pub created: Option<String>,
	 pub slug: String,
	 pub weight: i64,
	 pub description: String,
	 pub url: String,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub id: i64,
	 pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterTypeList {
	 pub results: Vec<ClusterType>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSiteGroupList {
	 pub results: Vec<SiteGroup>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedFHRPGroupRequest {
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	 pub protocol: String,
	 pub name: String,
	 pub auth_key: String,
	 pub group_id: i64,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	 pub auth_type: String,
	 pub comments: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterRequest {
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRearPortTemplateRequest {
	/// Physical label
	 pub label: String,
	 pub device_type: Option<i64>,
	 pub description: String,
	 pub color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
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
	 pub r#type: String,
	 pub module_type: Option<i64>,
	 pub positions: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bookmark {
	 pub object_id: i64,
	 pub display: String,
	 pub id: i64,
	 pub created: String,
	 pub object_type: String,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecPolicyRequest {
	 pub name: String,
	 pub proposals: Vec<i64>,
	 pub description: String,
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
	 pub pfs_group: Option<i64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackReservationRequest {
	 pub units: Vec<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayNestedModule {
	 pub url: String,
	 pub display: String,
	 pub id: i64,
	 pub serial: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualChassis {
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub description: String,
	 pub domain: String,
	 pub tags: Vec<NestedTag>,
	 pub member_count: i64,
	 pub last_updated: Option<String>,
	 pub comments: String,
	 pub url: String,
	 pub id: i64,
	 pub name: String,
	 pub display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerFeedRequest {
	 pub custom_fields: serde_json::value::Value,
	/// * `ac` - AC
	/// * `dc` - DC
	 pub supply: String,
	 pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	 pub status: String,
	 pub description: String,
	 pub comments: String,
	/// Maximum permissible draw (percentage)
	 pub max_utilization: i64,
	 pub voltage: i64,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	 pub phase: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	 pub r#type: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub amperage: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFHRPGroupList {
	 pub results: Vec<FHRPGroup>,
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableBookmarkRequest {
	 pub object_id: i64,
	 pub object_type: String,
	 pub user: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceRequest {
	 pub cable: Option<i64>,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecPolicy {
	 pub url: String,
	 pub pfs_group: serde_json::value::Value,
	 pub display: String,
	 pub proposals: Vec<i64>,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	 pub id: i64,
	 pub name: String,
	 pub created: Option<String>,
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldRequest {
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	 pub filter_logic: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	 pub label: String,
	/// Internal field name
	 pub name: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	 pub ui_editable: String,
	/// Fields with higher weights appear lower in a form.
	 pub weight: i64,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	 pub validation_regex: String,
	 pub content_types: Vec<String>,
	 pub object_type: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	 pub ui_visible: String,
	/// Replicate this value when cloning objects
	 pub is_cloneable: bool,
	/// Custom fields within the same group will be displayed together
	 pub group_name: String,
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
	/// If true, this field is required when creating new objects or editing an existing object.
	 pub required: bool,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	 pub search_weight: i64,
	/// Minimum allowed value (for numeric fields)
	 pub validation_minimum: Option<i64>,
	 pub description: String,
	/// Maximum allowed value (for numeric fields)
	 pub validation_maximum: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLAN {
	 pub url: String,
	 pub id: i64,
	 pub display: String,
	 pub name: String,
	/// Numeric VLAN ID (1-4094)
	 pub vid: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsolePortList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<ConsolePort>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rack {
	 pub description: String,
	 pub powerfeed_count: i64,
	/// Outer dimension of rack (width)
	 pub outer_width: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	 pub mounting_depth: Option<i64>,
	 pub weight: Option<f64>,
	 pub name: String,
	 pub outer_unit: Option<serde_json::value::Value>,
	 pub tags: Vec<NestedTag>,
	 pub display: String,
	/// Maximum load capacity for the rack
	 pub max_weight: Option<i64>,
	 pub r#type: Option<serde_json::value::Value>,
	 pub id: i64,
	 pub comments: String,
	 pub facility_id: Option<String>,
	/// Outer dimension of rack (depth)
	 pub outer_depth: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	/// Starting unit for rack
	 pub starting_unit: i64,
	 pub device_count: i64,
	 pub url: String,
	 pub status: serde_json::value::Value,
	/// A unique tag used to identify this rack
	 pub asset_tag: Option<String>,
	 pub serial: String,
	 pub width: serde_json::value::Value,
	/// Height in rack units
	 pub u_height: i64,
	 pub weight_unit: Option<serde_json::value::Value>,
	/// Units are numbered top-to-bottom
	 pub desc_units: bool,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleBayTemplateRequest {
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub device_type: i64,
	/// Physical label
	 pub label: String,
	/// Identifier to reference when renaming installed components
	 pub position: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Contact {
	 pub display: String,
	 pub link: String,
	 pub phone: String,
	 pub name: String,
	 pub email: String,
	 pub tags: Vec<NestedTag>,
	 pub address: String,
	 pub description: String,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub title: String,
	 pub comments: String,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRearPortTemplateRequest {
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
	 pub r#type: String,
	 pub positions: i64,
	 pub description: String,
	/// Physical label
	 pub label: String,
	 pub color: String,
	 pub module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub device_type: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFHRPGroupAssignmentRequest {
	 pub group: i64,
	 pub interface_type: String,
	 pub interface_id: i64,
	 pub priority: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleNestedModuleBayRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePrefixRequest {
	 pub tenant: Option<i64>,
	 pub prefix: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub vlan: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	 pub is_pool: bool,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	 pub site: Option<i64>,
	/// The primary function of this prefix
	 pub role: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub vrf: Option<i64>,
	 pub description: String,
	 pub comments: String,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecProfileList {
	 pub results: Vec<IPSecProfile>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPAddressRequest {
	 pub assigned_object_type: Option<String>,
	 pub address: String,
	 pub custom_fields: serde_json::value::Value,
	 pub vrf: Option<i64>,
	 pub tenant: Option<i64>,
	 pub comments: String,
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
	 pub role: String,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	 pub status: String,
	/// Hostname or FQDN (not case-sensitive)
	 pub dns_name: String,
	 pub assigned_object_id: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Representation of a VLAN which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableVLAN {
	 pub vid: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortTemplateRequest {
	 pub rear_port_position: i64,
	 pub color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub description: String,
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
	 pub r#type: String,
	/// Physical label
	 pub label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEPolicy {
	 pub proposals: Vec<i64>,
	 pub created: Option<String>,
	 pub url: String,
	 pub id: i64,
	 pub name: String,
	 pub version: serde_json::value::Value,
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub mode: serde_json::value::Value,
	 pub display: String,
	 pub preshared_key: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsolePortRequest {
	/// Physical label
	 pub label: String,
	 pub module: Option<i64>,
	 pub description: String,
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
	 pub speed: Option<i64>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub device: i64,
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
	 pub r#type: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPRangeRequest {
	 pub start_address: String,
	 pub comments: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub end_address: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceWithConfigContextRequest {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub oob_ip: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	 pub serial: String,
	 pub vc_position: Option<i64>,
	 pub comments: String,
	/// The function this device serves
	 pub role: i64,
	 pub primary_ip4: Option<i64>,
	 pub position: Option<f64>,
	 pub rack: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	 pub airflow: String,
	 pub primary_ip6: Option<i64>,
	 pub cluster: Option<i64>,
	/// * `front` - Front
	/// * `rear` - Rear
	 pub face: String,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub config_template: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub virtual_chassis: Option<i64>,
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
	 pub platform: Option<i64>,
	 pub device_type: i64,
	 pub site: i64,
	 pub name: Option<String>,
	 pub tenant: Option<i64>,
	/// Virtual chassis master election priority
	 pub vc_priority: Option<i64>,
	 pub location: Option<i64>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VRFRequest {
	/// Prevent duplicate prefixes/IP addresses within this VRF
	 pub enforce_unique: bool,
	 pub export_targets: Vec<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	 pub rd: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub comments: String,
	 pub import_targets: Vec<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableObjectPermissionRequest {
	 pub enabled: bool,
	 pub name: String,
	 pub groups: Vec<i64>,
	 pub description: String,
	/// The list of actions granted by this permission
	 pub actions: Vec<String>,
	 pub users: Vec<i64>,
	 pub object_types: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTunnelRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	 pub status: String,
	 pub ipsec_profile: Option<i64>,
	 pub tunnel_id: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub group: Option<i64>,
	 pub tenant: Option<i64>,
	 pub name: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	 pub encapsulation: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableL2VPNRequest {
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub import_targets: Vec<i64>,
	 pub description: String,
	 pub name: String,
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
	 pub r#type: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub export_targets: Vec<i64>,
	 pub tenant: Option<i64>,
	 pub identifier: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableUserRequest {
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	 pub groups: Vec<i64>,
	 pub email: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	 pub username: String,
	 pub password: String,
	/// Designates whether the user can log into this admin site.
	 pub is_staff: bool,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	 pub is_active: bool,
	 pub first_name: String,
	 pub date_joined: String,
	 pub last_name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePort {
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub speed: Option<serde_json::value::Value>,
	 pub url: String,
	 pub cable_end: String,
	 pub link_peers: Vec<String>,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
	 pub connected_endpoints: Vec<String>,
	 pub custom_fields: serde_json::value::Value,
	/// Physical label
	 pub label: String,
	 pub r#type: serde_json::value::Value,
	 pub description: String,
	 pub _occupied: bool,
	 pub connected_endpoints_reachable: bool,
	 pub display: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub connected_endpoints_type: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedEventRuleList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<EventRule>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerFeed {
	 pub voltage: i64,
	 pub name: String,
	 pub tags: Vec<NestedTag>,
	 pub url: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub id: i64,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub phase: serde_json::value::Value,
	 pub _occupied: bool,
	 pub last_updated: Option<String>,
	/// Maximum permissible draw (percentage)
	 pub max_utilization: i64,
	 pub connected_endpoints_type: String,
	 pub created: Option<String>,
	 pub comments: String,
	 pub r#type: serde_json::value::Value,
	 pub custom_fields: serde_json::value::Value,
	 pub link_peers: Vec<String>,
	 pub status: serde_json::value::Value,
	 pub connected_endpoints_reachable: bool,
	 pub display: String,
	 pub supply: serde_json::value::Value,
	 pub connected_endpoints: Vec<String>,
	 pub description: String,
	 pub cable_end: String,
	 pub amperage: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tunnel {
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub id: i64,
	 pub status: serde_json::value::Value,
	 pub tunnel_id: Option<i64>,
	 pub url: String,
	 pub display: String,
	 pub last_updated: Option<String>,
	 pub comments: String,
	 pub created: Option<String>,
	 pub encapsulation: serde_json::value::Value,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderAccountRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub provider: i64,
	 pub account: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleBayRequest {
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTagRequest {
	 pub color: String,
	 pub slug: String,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelRequest {
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectPermissionRequest {
	 pub name: String,
	 pub users: Vec<i64>,
	/// The list of actions granted by this permission
	 pub actions: Vec<String>,
	 pub description: String,
	 pub enabled: bool,
	 pub groups: Vec<i64>,
	 pub object_types: Vec<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GroupRequest {
	 pub name: String,
}

/// Used by device component serializers.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentNestedModule {
	 pub id: i64,
	 pub device: i64,
	 pub display: String,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderAccount {
	 pub id: i64,
	 pub description: String,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub created: Option<String>,
	 pub account: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableServiceTemplateRequest {
	 pub name: String,
	 pub description: String,
	 pub ports: Vec<i64>,
	 pub comments: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	 pub protocol: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRequest {
	 pub comments: String,
	 pub email: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub name: String,
	 pub link: String,
	 pub phone: String,
	 pub title: String,
	 pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemRoleList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<InventoryItemRole>,
	 pub previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRouteTargetList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<RouteTarget>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWebhookRequest {
	 pub name: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	 pub secret: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	 pub body_template: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	 pub ca_file_path: Option<String>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	 pub http_content_type: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	 pub http_method: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	 pub additional_headers: String,
	/// Enable SSL certificate verification. Disable with caution!
	 pub ssl_verification: bool,
	 pub description: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	 pub payload_url: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRoleList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<Role>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebhookRequest {
	 pub name: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	 pub payload_url: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	 pub body_template: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	 pub http_method: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	 pub secret: String,
	 pub tags: Vec<NestedTagRequest>,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	 pub additional_headers: String,
	/// Enable SSL certificate verification. Disable with caution!
	 pub ssl_verification: bool,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	 pub http_content_type: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	 pub ca_file_path: Option<String>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableASNRequest {
	/// Regional Internet Registry responsible for this AS number space
	 pub rir: i64,
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// 16- or 32-bit autonomous system number
	 pub asn: i64,
	 pub comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConfigContextRequest {
	 pub sites: Vec<i64>,
	 pub name: String,
	 pub cluster_groups: Vec<i64>,
	 pub locations: Vec<i64>,
	 pub is_active: bool,
	/// Remote data source
	 pub data_source: Option<i64>,
	 pub roles: Vec<i64>,
	 pub regions: Vec<i64>,
	 pub platforms: Vec<i64>,
	 pub cluster_types: Vec<i64>,
	 pub clusters: Vec<i64>,
	 pub device_types: Vec<i64>,
	 pub weight: i64,
	 pub description: String,
	 pub site_groups: Vec<i64>,
	 pub tenant_groups: Vec<i64>,
	 pub tags: Vec<String>,
	 pub tenants: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNTermination {
	 pub assigned_object_id: i64,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	 pub assigned_object_type: String,
	 pub id: i64,
	 pub display: String,
	 pub custom_fields: serde_json::value::Value,
	 pub url: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableSiteGroupRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub parent: Option<i64>,
	 pub description: String,
	 pub name: String,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackReservationList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<RackReservation>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCable {
	 pub display: String,
	 pub url: String,
	 pub id: i64,
	 pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVMInterfaceList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<VMInterface>,
	 pub next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceBayTemplateRequest {
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub device_type: i64,
	/// Physical label
	 pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedDashboardRequest {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableASNRequest {
	/// Regional Internet Registry responsible for this AS number space
	 pub rir: i64,
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	/// 16- or 32-bit autonomous system number
	 pub asn: i64,
	 pub comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecProfileRequest {
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenRequest {
	 pub last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	 pub write_enabled: bool,
	 pub description: String,
	 pub key: String,
	 pub expires: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomFieldChoiceSetList {
	 pub previous: Option<String>,
	 pub results: Vec<CustomFieldChoiceSet>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMInterface {
	 pub mode: serde_json::value::Value,
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub enabled: bool,
	 pub mtu: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub count_fhrp_groups: i64,
	 pub created: Option<String>,
	 pub name: String,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub tagged_vlans: Vec<i64>,
	 pub description: String,
	 pub mac_address: Option<String>,
	 pub count_ipaddresses: i64,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantGroup {
	 pub slug: String,
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	 pub name: String,
	 pub _depth: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedManufacturerList {
	 pub previous: Option<String>,
	 pub results: Vec<Manufacturer>,
	 pub next: Option<String>,
	 pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecPolicyList {
	 pub results: Vec<IPSecPolicy>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRIRList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<RIR>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	 pub vcpus: Option<f64>,
	 pub device: Option<i64>,
	 pub role: Option<i64>,
	 pub platform: Option<i64>,
	 pub cluster: Option<i64>,
	 pub tenant: Option<i64>,
	 pub primary_ip4: Option<i64>,
	 pub primary_ip6: Option<i64>,
	 pub disk: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub site: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub memory: Option<i64>,
	 pub name: String,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRackRequest {
	/// Starting unit for rack
	 pub starting_unit: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub tenant: Option<i64>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	 pub r#type: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	 pub outer_unit: String,
	 pub comments: String,
	/// Height in rack units
	 pub u_height: i64,
	 pub description: String,
	/// Functional role
	 pub role: Option<i64>,
	/// A unique tag used to identify this rack
	 pub asset_tag: Option<String>,
	 pub location: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: String,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub name: String,
	 pub weight: Option<f64>,
	/// Maximum load capacity for the rack
	 pub max_weight: Option<i64>,
	 pub site: i64,
	/// Units are numbered top-to-bottom
	 pub desc_units: bool,
	/// Outer dimension of rack (width)
	 pub outer_width: Option<i64>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	 pub width: i64,
	/// Outer dimension of rack (depth)
	 pub outer_depth: Option<i64>,
	 pub facility_id: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	 pub mounting_depth: Option<i64>,
	 pub serial: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerOutletList {
	 pub next: Option<String>,
	 pub results: Vec<PowerOutlet>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortTemplate {
	 pub r#type: serde_json::value::Value,
	 pub positions: i64,
	 pub color: String,
	 pub last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub id: i64,
	 pub description: String,
	 pub display: String,
	/// Physical label
	 pub label: String,
	 pub created: Option<String>,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRequest {
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVRF {
	 pub id: i64,
	 pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	 pub rd: Option<String>,
	 pub url: String,
	 pub display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNTerminationRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub assigned_object_type: String,
	 pub assigned_object_id: i64,
	 pub tags: Vec<NestedTagRequest>,
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
	 pub speed: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	/// Physical label
	 pub label: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub description: String,
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

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRoleRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPlatformList {
	 pub results: Vec<Platform>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableSiteRequest {
	 pub slug: String,
	/// Full name of the site
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	/// Local facility ID or description
	 pub facility: String,
	 pub region: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub time_zone: Option<String>,
	 pub asns: Vec<i64>,
	 pub group: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	 pub status: String,
	 pub description: String,
	/// Physical location of the building
	 pub physical_address: String,
	 pub tenant: Option<i64>,
	/// If different from the physical address
	 pub shipping_address: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInterfaceRequest {
	 pub vdcs: Vec<i64>,
	 pub mac_address: Option<String>,
	 pub bridge: Option<i64>,
	 pub tx_power: Option<i64>,
	 pub untagged_vlan: Option<i64>,
	 pub name: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub device: i64,
	/// * `pd` - PD
	/// * `pse` - PSE
	 pub poe_mode: String,
	 pub lag: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	 pub duplex: Option<String>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	 pub poe_type: String,
	 pub tagged_vlans: Vec<i64>,
	/// Physical label
	 pub label: String,
	/// Populated by selected channel (if set)
	 pub rf_channel_frequency: Option<f64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	 pub mode: String,
	 pub custom_fields: serde_json::value::Value,
	 pub vrf: Option<i64>,
	 pub enabled: bool,
	 pub module: Option<i64>,
	 pub parent: Option<i64>,
	/// This interface is used only for out-of-band management
	 pub mgmt_only: bool,
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
	 pub wireless_lans: Vec<i64>,
	/// Populated by selected channel (if set)
	 pub rf_channel_width: Option<f64>,
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
	 pub r#type: String,
	/// * `ap` - Access point
	/// * `station` - Station
	 pub rf_role: String,
	 pub speed: Option<i64>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub mtu: Option<i64>,
	 pub wwn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVLANList {
	 pub results: Vec<VLAN>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceRoleRequest {
	 pub description: String,
	 pub slug: String,
	 pub config_template: Option<i64>,
	 pub color: String,
	/// Virtual machines may be assigned to this role
	 pub vm_role: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RIRRequest {
	/// IP space managed by this RIR is considered private
	 pub is_private: bool,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub slug: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsoleServerPortList {
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<ConsoleServerPort>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRouteTargetRequest {
	 pub comments: String,
	/// Route target value (formatted in accordance with RFC 4360)
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVRFRequest {
	 pub tenant: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub export_targets: Vec<i64>,
	 pub description: String,
	 pub name: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	 pub enforce_unique: bool,
	 pub import_targets: Vec<i64>,
	 pub custom_fields: serde_json::value::Value,
	/// Unique route distinguisher (as defined in RFC 4364)
	 pub rd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInterfaceTemplateList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<InterfaceTemplate>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackReservation {
	 pub units: Vec<i64>,
	 pub created: Option<String>,
	 pub display: String,
	 pub url: String,
	 pub id: i64,
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactList {
	 pub results: Vec<Contact>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderNetwork {
	 pub url: String,
	 pub name: String,
	 pub display: String,
	 pub id: i64,
}

/// Minimal representation of some generic object identified by ContentType and PK.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericObject {
	 pub object_id: i64,
	 pub object_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderAccountList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<ProviderAccount>,
	 pub count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANGroup {
	 pub id: i64,
	 pub url: String,
	 pub display: String,
	 pub slug: String,
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayTemplate {
	/// Physical label
	 pub label: String,
	 pub url: String,
	 pub id: i64,
	/// Identifier to reference when renaming installed components
	 pub position: String,
	 pub display: String,
	 pub description: String,
	 pub last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContactGroupList {
	 pub next: Option<String>,
	 pub results: Vec<ContactGroup>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPRangeRequest {
	 pub description: String,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	 pub comments: String,
	 pub vrf: Option<i64>,
	/// The primary function of this range
	 pub role: Option<i64>,
	 pub end_address: String,
	 pub custom_fields: serde_json::value::Value,
	 pub start_address: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub tenant: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableObjectPermissionRequest {
	/// The list of actions granted by this permission
	 pub actions: Vec<String>,
	 pub description: String,
	 pub object_types: Vec<String>,
	 pub name: String,
	 pub groups: Vec<i64>,
	 pub enabled: bool,
	 pub users: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTokenRequest {
	 pub user: i64,
	 pub description: String,
	 pub last_used: Option<String>,
	 pub expires: Option<String>,
	/// Permit create/update/delete operations using this key
	 pub write_enabled: bool,
	 pub key: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDisk {
	 pub url: String,
	 pub created: Option<String>,
	 pub id: i64,
	 pub size: i64,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub last_updated: Option<String>,
	 pub name: String,
	 pub tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleTypeRequest {
	 pub description: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: String,
	 pub weight: Option<f64>,
	/// Discrete part number (optional)
	 pub part_number: String,
	 pub comments: String,
	 pub manufacturer: i64,
	 pub model: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInterfaceTemplateRequest {
	 pub enabled: bool,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	 pub poe_type: String,
	 pub bridge: Option<i64>,
	 pub module_type: Option<i64>,
	 pub device_type: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	 pub poe_mode: String,
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
	 pub r#type: String,
	 pub description: String,
	/// * `ap` - Access point
	/// * `station` - Station
	 pub rf_role: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub mgmt_only: bool,
	/// Physical label
	 pub label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVRFRequest {
	 pub description: String,
	 pub name: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	 pub enforce_unique: bool,
	 pub custom_fields: serde_json::value::Value,
	/// Unique route distinguisher (as defined in RFC 4364)
	 pub rd: Option<String>,
	 pub import_targets: Vec<i64>,
	 pub export_targets: Vec<i64>,
	 pub tenant: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerOutletRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub module: Option<i64>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub device: i64,
	 pub description: String,
	 pub name: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	 pub feed_leg: String,
	/// Physical label
	 pub label: String,
	 pub power_port: Option<i64>,
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
	 pub r#type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPAddress {
	 pub id: i64,
	 pub display: String,
	 pub family: i64,
	 pub url: String,
	 pub address: String,
}

/// Representation of an IP address which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableIP {
	 pub family: i64,
	 pub description: String,
	 pub address: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupRequest {
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub auth_key: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	 pub auth_type: String,
	 pub group_id: i64,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	 pub protocol: String,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPRangeList {
	 pub next: Option<String>,
	 pub results: Vec<IPRange>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitType {
	 pub url: String,
	 pub name: String,
	 pub slug: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub color: String,
	 pub display: String,
	 pub id: i64,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub circuit_count: i64,
	 pub created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceWithConfigContextRequest {
	 pub platform: Option<i64>,
	 pub cluster: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	 pub serial: String,
	 pub rack: Option<i64>,
	 pub primary_ip4: Option<i64>,
	/// The function this device serves
	 pub role: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	 pub airflow: String,
	 pub position: Option<f64>,
	 pub description: String,
	 pub vc_position: Option<i64>,
	/// Virtual chassis master election priority
	 pub vc_priority: Option<i64>,
	 pub virtual_chassis: Option<i64>,
	 pub site: i64,
	/// * `front` - Front
	/// * `rear` - Rear
	 pub face: String,
	 pub tenant: Option<i64>,
	 pub oob_ip: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub custom_fields: serde_json::value::Value,
	 pub location: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub config_template: Option<i64>,
	 pub primary_ip6: Option<i64>,
	 pub name: Option<String>,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub device_type: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupAssignmentRequest {
	 pub interface_id: i64,
	 pub priority: i64,
	 pub interface_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Circuit {
	 pub install_date: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub last_updated: Option<String>,
	 pub description: String,
	/// Unique circuit ID
	 pub cid: String,
	 pub created: Option<String>,
	 pub url: String,
	 pub id: i64,
	 pub display: String,
	 pub termination_date: Option<String>,
	 pub status: serde_json::value::Value,
	 pub comments: String,
	 pub tags: Vec<NestedTag>,
	/// Committed rate
	 pub commit_rate: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIPSecProfileRequest {
	 pub ike_policy: i64,
	 pub name: String,
	 pub description: String,
	 pub ipsec_policy: i64,
	/// * `esp` - ESP
	/// * `ah` - AH
	 pub mode: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEProposal {
	 pub comments: String,
	 pub authentication_algorithm: serde_json::value::Value,
	 pub created: Option<String>,
	 pub authentication_method: serde_json::value::Value,
	 pub id: i64,
	 pub last_updated: Option<String>,
	 pub group: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	/// Security association lifetime (in seconds)
	 pub sa_lifetime: Option<i64>,
	 pub name: String,
	 pub encryption_algorithm: serde_json::value::Value,
	 pub display: String,
	 pub description: String,
	 pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemList {
	 pub count: i64,
	 pub results: Vec<InventoryItem>,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDeviceRoleRequest {
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub config_template: Option<i64>,
	/// Virtual machines may be assigned to this role
	 pub vm_role: bool,
	 pub tags: Vec<NestedTagRequest>,
	 pub color: String,
	 pub slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsolePortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub module_type: Option<i64>,
	 pub description: String,
	 pub device_type: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenProvision {
	/// Permit create/update/delete operations using this key
	 pub write_enabled: bool,
	 pub last_used: String,
	 pub url: String,
	 pub id: i64,
	 pub created: String,
	 pub key: String,
	 pub description: String,
	 pub display: String,
	 pub expires: Option<String>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigTemplate {
	 pub data_synced: Option<String>,
	 pub url: String,
	/// Path to remote file (relative to data source root)
	 pub data_path: String,
	/// Jinja2 template code.
	 pub template_code: String,
	 pub id: i64,
	 pub name: String,
	 pub tags: Vec<NestedTag>,
	 pub display: String,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInventoryItemRoleRequest {
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBay {
	 pub last_updated: Option<String>,
	/// Physical label
	 pub label: String,
	 pub id: i64,
	/// Identifier to reference when renaming installed components
	 pub position: String,
	 pub name: String,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub url: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Platform {
	 pub created: Option<String>,
	 pub device_count: i64,
	 pub virtualmachine_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub slug: String,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub display: String,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteRequest {
	 pub slug: String,
	/// Full name of the site
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTermination {
	 pub url: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub _occupied: bool,
	 pub display: String,
	/// Upstream speed, if different from port speed
	 pub upstream_speed: Option<i64>,
	/// Physical circuit speed
	 pub port_speed: Option<i64>,
	/// Patch panel ID and port number(s)
	 pub pp_info: String,
	 pub last_updated: Option<String>,
	 pub cable_end: String,
	/// * `A` - A
	/// * `Z` - Z
	 pub term_side: String,
	 pub tags: Vec<NestedTag>,
	 pub link_peers: Vec<String>,
	 pub id: i64,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub created: Option<String>,
	/// ID of the local cross-connect
	 pub xconnect_id: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableServiceRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub ports: Vec<i64>,
	 pub device: Option<i64>,
	 pub description: String,
	 pub name: String,
	/// The specific IP addresses (if any) to which this service is bound
	 pub ipaddresses: Vec<i64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub virtual_machine: Option<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	 pub protocol: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFrontPortTemplateList {
	 pub count: i64,
	 pub results: Vec<FrontPortTemplate>,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Interface {
	 pub rf_role: serde_json::value::Value,
	 pub connected_endpoints: Vec<String>,
	 pub tx_power: Option<i64>,
	/// Populated by selected channel (if set)
	 pub rf_channel_width: Option<f64>,
	 pub tagged_vlans: Vec<i64>,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub wwn: Option<String>,
	 pub count_ipaddresses: i64,
	 pub id: i64,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub poe_mode: serde_json::value::Value,
	 pub cable_end: String,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub speed: Option<i64>,
	 pub mode: serde_json::value::Value,
	 pub poe_type: serde_json::value::Value,
	 pub mtu: Option<i64>,
	 pub rf_channel: serde_json::value::Value,
	 pub mac_address: Option<String>,
	 pub connected_endpoints_type: String,
	 pub connected_endpoints_reachable: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub count_fhrp_groups: i64,
	 pub _occupied: bool,
	/// Physical label
	 pub label: String,
	 pub name: String,
	 pub vdcs: Vec<i64>,
	/// Populated by selected channel (if set)
	 pub rf_channel_frequency: Option<f64>,
	 pub display: String,
	/// This interface is used only for out-of-band management
	 pub mgmt_only: bool,
	 pub wireless_lans: Vec<i64>,
	 pub r#type: serde_json::value::Value,
	 pub enabled: bool,
	 pub duplex: Option<serde_json::value::Value>,
	 pub description: String,
	 pub link_peers: Vec<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelTermination {
	 pub url: String,
	 pub created: Option<String>,
	 pub termination_id: Option<i64>,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub termination_type: String,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub role: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPanelList {
	 pub next: Option<String>,
	 pub results: Vec<PowerPanel>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedVLANGroupRequest {
	 pub tags: Vec<NestedTagRequest>,
	/// Lowest permissible ID of a child VLAN
	 pub min_vid: i64,
	 pub scope_type: Option<String>,
	/// Highest permissible ID of a child VLAN
	 pub max_vid: i64,
	 pub slug: String,
	 pub name: String,
	 pub scope_id: Option<i64>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteGroupRequest {
	 pub name: String,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRIR {
	 pub url: String,
	 pub name: String,
	 pub display: String,
	 pub slug: String,
	 pub id: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableSiteGroupRequest {
	 pub slug: String,
	 pub parent: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDataSourceList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<DataSource>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIKEProposalList {
	 pub count: i64,
	 pub results: Vec<IKEProposal>,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderNetwork {
	 pub display: String,
	 pub id: i64,
	 pub comments: String,
	 pub created: Option<String>,
	 pub name: String,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub description: String,
	 pub service_id: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProposalRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	/// Security association lifetime (in kilobytes)
	 pub sa_lifetime_data: Option<i64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Security association lifetime (seconds)
	 pub sa_lifetime_seconds: Option<i64>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	 pub authentication_algorithm: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	 pub encryption_algorithm: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPrefixList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<Prefix>,
	 pub count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRegion {
	 pub url: String,
	 pub slug: String,
	 pub _depth: i64,
	 pub id: i64,
	 pub display: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleBayRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	/// Physical label
	 pub label: String,
	 pub device: i64,
	 pub name: String,
	/// Identifier to reference when renaming installed components
	 pub position: String,
	 pub installed_module: i64,
	 pub tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortTemplate {
	 pub display: String,
	 pub url: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub id: i64,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRearPort {
	 pub name: String,
	 pub display: String,
	 pub id: i64,
	/// Physical label
	 pub label: String,
	 pub description: String,
	 pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitTypeList {
	 pub next: Option<String>,
	 pub results: Vec<CircuitType>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPanelRequest {
	 pub comments: String,
	 pub site: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub location: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedConfigTemplate {
	 pub url: String,
	 pub display: String,
	 pub id: i64,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manufacturer {
	 pub platform_count: i64,
	 pub description: String,
	 pub id: i64,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub display: String,
	 pub slug: String,
	 pub devicetype_count: i64,
	 pub last_updated: Option<String>,
	 pub inventoryitem_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceRoleList {
	 pub next: Option<String>,
	 pub results: Vec<DeviceRole>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomField {
	 pub ui_visible: serde_json::value::Value,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	 pub search_weight: i64,
	/// Replicate this value when cloning objects
	 pub is_cloneable: bool,
	 pub content_types: Vec<String>,
	/// Internal field name
	 pub name: String,
	 pub description: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	 pub required: bool,
	/// Maximum allowed value (for numeric fields)
	 pub validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	 pub validation_regex: String,
	 pub filter_logic: serde_json::value::Value,
	/// Custom fields within the same group will be displayed together
	 pub group_name: String,
	 pub ui_editable: serde_json::value::Value,
	 pub display: String,
	 pub last_updated: Option<String>,
	 pub id: i64,
	/// Fields with higher weights appear lower in a form.
	 pub weight: i64,
	 pub url: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	 pub label: String,
	/// Minimum allowed value (for numeric fields)
	 pub validation_minimum: Option<i64>,
	 pub created: Option<String>,
	 pub r#type: serde_json::value::Value,
	 pub object_type: String,
	 pub data_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleRequest {
	 pub serial: String,
	 pub device: i64,
	 pub module_bay: i64,
	 pub module_type: i64,
	 pub custom_fields: serde_json::value::Value,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableServiceTemplateRequest {
	 pub description: String,
	 pub comments: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	 pub protocol: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub ports: Vec<i64>,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderNetworkRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub service_id: String,
	 pub name: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelRequest {
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	 pub status: String,
	 pub comments: String,
	 pub tunnel_id: Option<i64>,
	 pub description: String,
	 pub name: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	 pub encapsulation: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataFile {
	/// File path relative to the data source's root
	 pub path: String,
	 pub display: String,
	 pub url: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecPolicyRequest {
	 pub description: String,
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
	 pub pfs_group: Option<i64>,
	 pub proposals: Vec<i64>,
	 pub comments: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIKEPolicyRequest {
	 pub proposals: Vec<i64>,
	 pub name: String,
	 pub description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	 pub version: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub preshared_key: String,
	 pub custom_fields: serde_json::value::Value,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	 pub mode: String,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderAccountRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub comments: String,
	 pub account: String,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTunnelTerminationRequest {
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	 pub role: String,
	 pub tunnel: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub termination_type: String,
	 pub termination_id: Option<i64>,
	 pub outside_ip: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerFeedRequest {
	/// * `ac` - AC
	/// * `dc` - DC
	 pub supply: String,
	 pub name: String,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	 pub phase: String,
	 pub amperage: i64,
	 pub voltage: i64,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub tags: Vec<NestedTagRequest>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	 pub status: String,
	 pub power_panel: i64,
	 pub rack: Option<i64>,
	/// Maximum permissible draw (percentage)
	 pub max_utilization: i64,
	 pub tenant: Option<i64>,
	 pub comments: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	 pub r#type: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRackRoleRequest {
	 pub description: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub color: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactGroup {
	 pub description: String,
	 pub display: String,
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub slug: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub _depth: i64,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub contact_count: i64,
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageAttachment {
	 pub image_width: i64,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub object_id: i64,
	 pub content_type: String,
	 pub url: String,
	 pub name: String,
	 pub display: String,
	 pub id: i64,
	 pub image: String,
	 pub image_height: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItem {
	 pub id: i64,
	 pub name: String,
	 pub component_id: Option<i64>,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub parent: Option<i64>,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	 pub _depth: i64,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	/// Physical label
	 pub label: String,
	 pub url: String,
	/// A unique tag used to identify this item
	 pub asset_tag: Option<String>,
	/// This item was automatically discovered
	 pub discovered: bool,
	 pub component_type: Option<String>,
	 pub display: String,
	 pub serial: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RIR {
	 pub description: String,
	 pub slug: String,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub url: String,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	/// IP space managed by this RIR is considered private
	 pub is_private: bool,
	 pub name: String,
	 pub aggregate_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactAssignmentRequest {
	 pub object_id: i64,
	 pub contact: i64,
	 pub content_type: String,
	 pub role: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	 pub priority: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedL2VPNList {
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<L2VPN>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterList {
	 pub results: Vec<Cluster>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigTemplateRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub description: String,
	/// Jinja2 template code.
	 pub template_code: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedObjectPermissionList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<ObjectPermission>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLANGroupRequest {
	 pub name: String,
	 pub slug: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub parent: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableL2VPNRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub export_targets: Vec<i64>,
	 pub identifier: Option<i64>,
	 pub slug: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub import_targets: Vec<i64>,
	 pub tenant: Option<i64>,
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
	 pub r#type: String,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteGroupRequest {
	 pub slug: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Full name of the provider
	 pub name: String,
	 pub description: String,
	 pub slug: String,
	 pub accounts: Vec<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub asns: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
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
	/// * `power` - Power
	 pub r#type: String,
	 pub comments: String,
	 pub b_terminations: Vec<GenericObjectRequest>,
	 pub a_terminations: Vec<GenericObjectRequest>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	 pub length_unit: String,
	 pub label: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub color: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub length: Option<f64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedClusterTypeRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub slug: String,
	 pub description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInventoryItemTemplateRequest {
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub device_type: i64,
	 pub description: String,
	 pub parent: Option<i64>,
	 pub component_type: Option<String>,
	 pub role: Option<i64>,
	 pub manufacturer: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePrefixRequest {
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	 pub tags: Vec<NestedTagRequest>,
	 pub vrf: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	 pub is_pool: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub tenant: Option<i64>,
	 pub site: Option<i64>,
	 pub prefix: String,
	/// The primary function of this prefix
	 pub role: Option<i64>,
	 pub vlan: Option<i64>,
	 pub description: String,
	 pub comments: String,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableTermination {
	 pub url: String,
	 pub cable: i64,
	 pub termination_type: String,
	 pub created: Option<String>,
	/// * `A` - A
	/// * `B` - B
	 pub cable_end: String,
	 pub display: String,
	 pub id: i64,
	 pub termination_id: i64,
	 pub last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactGroupRequest {
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceTypeRequest {
	 pub description: String,
	 pub slug: String,
	 pub comments: String,
	 pub weight: Option<f64>,
	 pub rear_image: String,
	/// Discrete part number (optional)
	 pub part_number: String,
	/// * `parent` - Parent
	/// * `child` - Child
	 pub subdevice_role: Option<String>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: Option<String>,
	 pub front_image: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub u_height: f64,
	 pub model: String,
	/// Devices of this type are excluded when calculating rack utilization.
	 pub exclude_from_utilization: bool,
	/// Device consumes both front and rear rack faces.
	 pub is_full_depth: bool,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	 pub airflow: Option<String>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterTypeRequest {
	 pub name: String,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedUser {
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	 pub username: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortTemplateRequest {
	 pub description: String,
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
	 pub r#type: Option<String>,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualChassisRequest {
	 pub domain: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerOutletTemplateRequest {
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	 pub feed_leg: String,
	 pub power_port: Option<i64>,
	 pub module_type: Option<i64>,
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
	 pub r#type: String,
	 pub device_type: Option<i64>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteTargetRequest {
	 pub description: String,
	/// Route target value (formatted in accordance with RFC 4360)
	 pub name: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRackRequest {
	/// * `mm` - Millimeters
	/// * `in` - Inches
	 pub outer_unit: String,
	/// Height in rack units
	 pub u_height: i64,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	 pub r#type: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Maximum load capacity for the rack
	 pub max_weight: Option<i64>,
	/// Units are numbered top-to-bottom
	 pub desc_units: bool,
	/// Outer dimension of rack (width)
	 pub outer_width: Option<i64>,
	 pub name: String,
	 pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: String,
	/// Outer dimension of rack (depth)
	 pub outer_depth: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	 pub mounting_depth: Option<i64>,
	 pub location: Option<i64>,
	 pub description: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub facility_id: Option<String>,
	/// A unique tag used to identify this rack
	 pub asset_tag: Option<String>,
	/// Functional role
	 pub role: Option<i64>,
	 pub tenant: Option<i64>,
	 pub site: i64,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	 pub width: i64,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	 pub status: String,
	/// Starting unit for rack
	 pub starting_unit: i64,
	 pub serial: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<Module>,
	 pub previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTemplateRequest {
	 pub content_types: Vec<String>,
	 pub name: String,
	/// Download file as attachment
	 pub as_attachment: bool,
	/// Extension to append to the rendered filename
	 pub file_extension: String,
	 pub description: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	 pub template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	 pub mime_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedInventoryItemRoleRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub name: String,
	 pub color: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRouteTargetRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub tenant: Option<i64>,
	 pub description: String,
	/// Route target value (formatted in accordance with RFC 4360)
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVMInterfaceRequest {
	 pub description: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	 pub mode: String,
	 pub untagged_vlan: Option<i64>,
	 pub tagged_vlans: Vec<i64>,
	 pub vrf: Option<i64>,
	 pub mtu: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub mac_address: Option<String>,
	 pub enabled: bool,
	 pub virtual_machine: i64,
	 pub parent: Option<i64>,
	 pub bridge: Option<i64>,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualChassisRequest {
	 pub description: String,
	 pub domain: String,
	 pub master: Option<i64>,
	 pub comments: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortTemplate {
	 pub id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
	 pub display: String,
	 pub created: Option<String>,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
	 pub r#type: Option<serde_json::value::Value>,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualMachineWithConfigContextRequest {
	 pub description: String,
	 pub memory: Option<i64>,
	 pub vcpus: Option<f64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub disk: Option<i64>,
	 pub name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	 pub status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANGroupRequest {
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRackReservationRequest {
	 pub units: Vec<i64>,
	 pub rack: i64,
	 pub tenant: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub description: String,
	 pub user: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualMachineRequest {
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecPolicy {
	 pub id: i64,
	 pub name: String,
	 pub url: String,
	 pub display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableASNRangeRequest {
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub end: i64,
	 pub start: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub rir: i64,
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
	 pub r#type: String,
	 pub module_type: Option<i64>,
	 pub device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedASNList {
	 pub results: Vec<ASN>,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPortRequest {
	 pub name: String,
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
	 pub r#type: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub module: Option<i64>,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub device: i64,
	/// Physical label
	 pub label: String,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitCircuitTerminationRequest {
	/// ID of the local cross-connect
	 pub xconnect_id: String,
	/// Upstream speed, if different from port speed
	 pub upstream_speed: Option<i64>,
	/// Physical circuit speed
	 pub port_speed: Option<i64>,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantRequest {
	 pub name: String,
	 pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedBookmarkList {
	 pub count: i64,
	 pub results: Vec<Bookmark>,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleBayTemplateList {
	 pub previous: Option<String>,
	 pub results: Vec<ModuleBayTemplate>,
	 pub next: Option<String>,
	 pub count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Prefix {
	/// All IP addresses within this prefix are considered usable
	 pub is_pool: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub description: String,
	 pub _depth: i64,
	 pub last_updated: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub family: serde_json::value::Value,
	 pub id: i64,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	 pub prefix: String,
	 pub created: Option<String>,
	 pub children: i64,
	 pub url: String,
	 pub display: String,
	 pub status: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRoleRequest {
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventRuleRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub enabled: bool,
	 pub tags: Vec<NestedTagRequest>,
	/// Triggers when a matching object is updated.
	 pub type_update: bool,
	/// Triggers when a matching object is deleted.
	 pub type_delete: bool,
	 pub description: String,
	/// Triggers when a job for a matching object terminates.
	 pub type_job_end: bool,
	 pub action_object_type: String,
	 pub name: String,
	 pub content_types: Vec<String>,
	/// Triggers when a job for a matching object is started.
	 pub type_job_start: bool,
	 pub action_object_id: Option<i64>,
	/// Triggers when a matching object is created.
	 pub type_create: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	 pub action_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRearPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub display: String,
	 pub id: i64,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableDataSourceRequest {
	 pub r#type: String,
	 pub description: String,
	 pub name: String,
	 pub source_url: String,
	 pub enabled: bool,
	 pub comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	 pub ignore_rules: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dashboard {
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModuleBay {
	 pub display: String,
	 pub name: String,
	 pub id: i64,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderRequest {
	/// Full name of the provider
	 pub name: String,
	 pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedJournalEntryList {
	 pub next: Option<String>,
	 pub results: Vec<JournalEntry>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsoleServerPortRequest {
	 pub module: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub device: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	/// Physical label
	 pub label: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
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
	 pub speed: Option<i64>,
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
	 pub r#type: String,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNTermination {
	 pub id: i64,
	 pub url: String,
	 pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVRFList {
	 pub next: Option<String>,
	 pub results: Vec<VRF>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceBayRequest {
	 pub name: String,
	 pub installed_device: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	/// Physical label
	 pub label: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub device: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLinkRequest {
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	 pub auth_type: String,
	 pub ssid: String,
	 pub interface_a: i64,
	 pub interface_b: i64,
	 pub tenant: Option<i64>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	 pub auth_cipher: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub comments: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub auth_psk: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cable {
	 pub b_terminations: Vec<GenericObject>,
	 pub length_unit: Option<serde_json::value::Value>,
	 pub tags: Vec<NestedTag>,
	 pub status: serde_json::value::Value,
	 pub color: String,
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
	 pub r#type: String,
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	 pub label: String,
	 pub length: Option<f64>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub description: String,
	 pub a_terminations: Vec<GenericObject>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTypeRequest {
	 pub color: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub slug: String,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDevice {
	 pub url: String,
	 pub id: i64,
	 pub name: Option<String>,
	 pub display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMInterfaceRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub mac_address: Option<String>,
	 pub description: String,
	 pub mtu: Option<i64>,
	 pub enabled: bool,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	 pub mode: String,
	 pub name: String,
	 pub tagged_vlans: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Group {
	 pub user_count: i64,
	 pub url: String,
	 pub display: String,
	 pub id: i64,
	 pub name: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericObjectRequest {
	 pub object_id: i64,
	 pub object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTemplate {
	 pub description: String,
	 pub data_synced: Option<String>,
	 pub url: String,
	 pub name: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	 pub template_code: String,
	 pub last_updated: Option<String>,
	 pub display: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	 pub mime_type: String,
	/// Extension to append to the rendered filename
	 pub file_extension: String,
	 pub content_types: Vec<String>,
	/// Path to remote file (relative to data source root)
	 pub data_path: String,
	/// Download file as attachment
	 pub as_attachment: bool,
	 pub created: Option<String>,
	 pub id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceTypeRequest {
	 pub model: String,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPort {
	 pub _occupied: bool,
	 pub display: String,
	 pub cable: Option<i64>,
	 pub url: String,
	 pub name: String,
	 pub id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsoleServerPortTemplateRequest {
	/// Physical label
	 pub label: String,
	 pub description: String,
	 pub module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub device_type: Option<i64>,
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

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCircuitTerminationRequest {
	/// * `A` - A
	/// * `Z` - Z
	 pub term_side: String,
	/// Physical circuit speed
	 pub port_speed: Option<i64>,
	 pub site: Option<i64>,
	 pub provider_network: Option<i64>,
	/// ID of the local cross-connect
	 pub xconnect_id: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub circuit: i64,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Patch panel ID and port number(s)
	 pub pp_info: String,
	/// Upstream speed, if different from port speed
	 pub upstream_speed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLANList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<WirelessLAN>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortTemplate {
	 pub display: String,
	/// Physical label
	 pub label: String,
	 pub url: String,
	 pub r#type: serde_json::value::Value,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub description: String,
	 pub created: Option<String>,
	 pub id: i64,
	 pub last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFrontPortRequest {
	 pub description: String,
	/// Mapped position on corresponding rear port
	 pub rear_port_position: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub device: i64,
	 pub rear_port: i64,
	/// Physical label
	 pub label: String,
	 pub module: Option<i64>,
	 pub color: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub name: String,
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
	 pub r#type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPAddress {
	 pub custom_fields: serde_json::value::Value,
	 pub assigned_object_type: Option<String>,
	 pub assigned_object_id: Option<i64>,
	 pub description: String,
	 pub address: String,
	 pub family: serde_json::value::Value,
	 pub comments: String,
	 pub id: i64,
	 pub display: String,
	/// Hostname or FQDN (not case-sensitive)
	 pub dns_name: String,
	 pub created: Option<String>,
	 pub role: serde_json::value::Value,
	 pub status: serde_json::value::Value,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub nat_outside: Vec<NestedIPAddress>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableJournalEntryRequest {
	 pub assigned_object_type: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	 pub kind: String,
	 pub custom_fields: serde_json::value::Value,
	 pub assigned_object_id: i64,
	 pub created_by: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Module {
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub status: serde_json::value::Value,
	 pub comments: String,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub display: String,
	 pub serial: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub description: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRoleRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub slug: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub color: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANRequest {
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub ssid: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	 pub auth_cipher: String,
	 pub description: String,
	 pub auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	 pub auth_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Aggregate {
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub comments: String,
	 pub date_added: Option<String>,
	 pub family: serde_json::value::Value,
	 pub prefix: String,
	 pub url: String,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVRFRequest {
	 pub name: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	 pub rd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleBayList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<ModuleBay>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableEventRuleRequest {
	/// * `webhook` - Webhook
	/// * `script` - Script
	 pub action_type: String,
	 pub content_types: Vec<String>,
	/// Triggers when a matching object is created.
	 pub type_create: bool,
	 pub action_object_type: String,
	/// Triggers when a job for a matching object is started.
	 pub type_job_start: bool,
	 pub action_object_id: Option<i64>,
	/// Triggers when a matching object is updated.
	 pub type_update: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub enabled: bool,
	/// Triggers when a matching object is deleted.
	 pub type_delete: bool,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Triggers when a job for a matching object terminates.
	 pub type_job_end: bool,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPN {
	 pub name: String,
	 pub last_updated: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub export_targets: Vec<i64>,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub identifier: Option<i64>,
	 pub import_targets: Vec<i64>,
	 pub slug: String,
	 pub display: String,
	 pub comments: String,
	 pub id: i64,
	 pub r#type: serde_json::value::Value,
	 pub description: String,
	 pub created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactRequest {
	 pub name: String,
	 pub address: String,
	 pub title: String,
	 pub link: String,
	 pub description: String,
	 pub group: Option<i64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub phone: String,
	 pub custom_fields: serde_json::value::Value,
	 pub email: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortRequest {
	 pub name: String,
	 pub cable: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCircuitRequest {
	 pub install_date: Option<String>,
	/// Committed rate
	 pub commit_rate: Option<i64>,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	 pub status: String,
	 pub termination_date: Option<String>,
	 pub tenant: Option<i64>,
	 pub r#type: i64,
	 pub description: String,
	 pub provider_account: Option<i64>,
	 pub provider: i64,
	/// Unique circuit ID
	 pub cid: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedManufacturerRequest {
	 pub name: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPRangeRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub end_address: String,
	 pub tenant: Option<i64>,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub description: String,
	 pub start_address: String,
	 pub comments: String,
	/// The primary function of this range
	 pub role: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub vrf: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelTerminationRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub termination_id: Option<i64>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	 pub role: String,
	 pub termination_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRange {
	 pub url: String,
	 pub slug: String,
	 pub start: i64,
	 pub name: String,
	 pub description: String,
	 pub created: Option<String>,
	 pub id: i64,
	 pub tags: Vec<NestedTag>,
	 pub display: String,
	 pub asn_count: i64,
	 pub end: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVLANRequest {
	/// Numeric VLAN ID (1-4094)
	 pub vid: i64,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProviderAccountRequest {
	 pub name: String,
	 pub account: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConfigContextList {
	 pub results: Vec<ConfigContext>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedModuleTypeList {
	 pub results: Vec<ModuleType>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	 pub status: String,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataSourceRequest {
	 pub enabled: bool,
	/// Patterns (one per line) matching files to ignore when syncing
	 pub ignore_rules: String,
	 pub description: String,
	 pub comments: String,
	 pub name: String,
	 pub source_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWebhookList {
	 pub next: Option<String>,
	 pub count: i64,
	 pub results: Vec<Webhook>,
	 pub previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRoleRequest {
	 pub slug: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualDiskRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub size: i64,
	 pub virtual_machine: i64,
	 pub description: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterGroupRequest {
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCustomFieldChoiceSetRequest {
	 pub name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	 pub status: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDataFileList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<DataFile>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct L2VPNRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub export_targets: Vec<i64>,
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
	 pub r#type: String,
	 pub name: String,
	 pub import_targets: Vec<i64>,
	 pub identifier: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayNestedModuleRequest {
	 pub serial: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterGroup {
	 pub url: String,
	 pub id: i64,
	 pub display: String,
	 pub slug: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceType {
	 pub display: String,
	 pub airflow: Option<serde_json::value::Value>,
	 pub power_outlet_template_count: i64,
	 pub console_port_template_count: i64,
	 pub front_image: String,
	 pub interface_template_count: i64,
	 pub rear_port_template_count: i64,
	 pub front_port_template_count: i64,
	 pub model: String,
	/// Devices of this type are excluded when calculating rack utilization.
	 pub exclude_from_utilization: bool,
	 pub module_bay_template_count: i64,
	 pub inventory_item_template_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub weight_unit: Option<serde_json::value::Value>,
	 pub url: String,
	 pub created: Option<String>,
	/// Discrete part number (optional)
	 pub part_number: String,
	/// Device consumes both front and rear rack faces.
	 pub is_full_depth: bool,
	 pub description: String,
	 pub subdevice_role: Option<serde_json::value::Value>,
	 pub console_server_port_template_count: i64,
	 pub device_count: i64,
	 pub weight: Option<f64>,
	 pub custom_fields: serde_json::value::Value,
	 pub power_port_template_count: i64,
	 pub u_height: f64,
	 pub id: i64,
	 pub device_bay_template_count: i64,
	 pub slug: String,
	 pub last_updated: Option<String>,
	 pub rear_image: String,
	 pub comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemTemplate {
	 pub id: i64,
	 pub _depth: i64,
	 pub display: String,
	 pub url: String,
	 pub description: String,
	 pub parent: Option<i64>,
	 pub component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub created: Option<String>,
	 pub component_type: Option<String>,
	/// Physical label
	 pub label: String,
	 pub last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub color: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Mapped position on corresponding rear port
	 pub rear_port_position: i64,
	/// Physical label
	 pub label: String,
	 pub description: String,
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
	 pub r#type: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceTypeList {
	 pub next: Option<String>,
	 pub results: Vec<DeviceType>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInventoryItemTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub description: String,
	/// Physical label
	 pub label: String,
	 pub component_type: Option<String>,
	 pub manufacturer: Option<i64>,
	 pub component_id: Option<i64>,
	 pub device_type: i64,
	 pub role: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecProposalRequest {
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	 pub encryption_algorithm: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	/// Security association lifetime (in kilobytes)
	 pub sa_lifetime_data: Option<i64>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	 pub authentication_algorithm: String,
	/// Security association lifetime (seconds)
	 pub sa_lifetime_seconds: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataSourceRequest {
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitType {
	 pub display: String,
	 pub slug: String,
	 pub url: String,
	 pub id: i64,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVMInterfaceRequest {
	 pub name: String,
	 pub parent: Option<i64>,
	 pub mtu: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	 pub mode: String,
	 pub enabled: bool,
	 pub description: String,
	 pub virtual_machine: i64,
	 pub bridge: Option<i64>,
	 pub untagged_vlan: Option<i64>,
	 pub vrf: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub mac_address: Option<String>,
	 pub tags: Vec<NestedTagRequest>,
	 pub tagged_vlans: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub description: String,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	 pub status: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub install_date: Option<String>,
	 pub termination_date: Option<String>,
	/// Unique circuit ID
	 pub cid: String,
	/// Committed rate
	 pub commit_rate: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceBayTemplateList {
	 pub results: Vec<DeviceBayTemplate>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedFilterRequest {
	 pub description: String,
	 pub slug: String,
	 pub user: Option<i64>,
	 pub enabled: bool,
	 pub content_types: Vec<String>,
	 pub shared: bool,
	 pub weight: i64,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub ports: Vec<i64>,
	 pub ipaddresses: Vec<i64>,
	 pub comments: String,
	 pub display: String,
	 pub description: String,
	 pub tags: Vec<NestedTag>,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub protocol: serde_json::value::Value,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableProviderRequest {
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Full name of the provider
	 pub name: String,
	 pub description: String,
	 pub comments: String,
	 pub asns: Vec<i64>,
	 pub accounts: Vec<i64>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRole {
	 pub id: i64,
	 pub slug: String,
	 pub display: String,
	 pub url: String,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedManufacturerRequest {
	 pub slug: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCableList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<Cable>,
	 pub next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVMInterfaceRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLinkRequest {
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	 pub auth_type: String,
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub interface_a: i64,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub ssid: String,
	 pub interface_b: i64,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub auth_psk: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	 pub auth_cipher: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedFilter {
	 pub user: Option<i64>,
	 pub enabled: bool,
	 pub created: Option<String>,
	 pub display: String,
	 pub content_types: Vec<String>,
	 pub id: i64,
	 pub name: String,
	 pub slug: String,
	 pub shared: bool,
	 pub url: String,
	 pub weight: i64,
	 pub description: String,
	 pub last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRack {
	 pub url: String,
	 pub id: i64,
	 pub display: String,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedProvider {
	 pub slug: String,
	 pub display: String,
	 pub url: String,
	/// Full name of the provider
	 pub name: String,
	 pub id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleNestedModuleBay {
	 pub display: String,
	 pub url: String,
	 pub name: String,
	 pub id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenantGroupRequest {
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableAggregateRequest {
	 pub prefix: String,
	 pub date_added: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub tenant: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	/// Regional Internet Registry responsible for this IP space
	 pub rir: i64,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualChassisRequest {
	 pub comments: String,
	 pub name: String,
	 pub domain: String,
	 pub master: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPort {
	 pub last_updated: Option<String>,
	/// Number of front ports which may be mapped
	 pub positions: i64,
	 pub display: String,
	 pub r#type: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub cable_end: String,
	 pub created: Option<String>,
	 pub url: String,
	 pub name: String,
	 pub id: i64,
	 pub color: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
	 pub link_peers: Vec<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub _occupied: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecPolicyRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANGroupRequest {
	 pub slug: String,
	 pub scope_id: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	/// Lowest permissible ID of a child VLAN
	 pub min_vid: i64,
	/// Highest permissible ID of a child VLAN
	 pub max_vid: i64,
	 pub description: String,
	 pub scope_type: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelGroup {
	 pub display: String,
	 pub url: String,
	 pub name: String,
	 pub id: i64,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIKEPolicyRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANRequest {
	 pub comments: String,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	/// Numeric VLAN ID (1-4094)
	 pub vid: i64,
	 pub tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableServiceRequest {
	 pub custom_fields: serde_json::value::Value,
	/// The specific IP addresses (if any) to which this service is bound
	 pub ipaddresses: Vec<i64>,
	 pub name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	 pub protocol: String,
	 pub comments: String,
	 pub virtual_machine: Option<i64>,
	 pub ports: Vec<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub device: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTokenList {
	 pub results: Vec<Token>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Used by device component serializers.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentNestedModuleRequest {
	 pub device: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPSecProfileRequest {
	 pub name: String,
	 pub ike_policy: i64,
	 pub comments: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	 pub mode: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub ipsec_policy: i64,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Device {
	 pub power_port_count: i64,
	 pub interface_count: i64,
	 pub face: serde_json::value::Value,
	 pub id: i64,
	 pub status: serde_json::value::Value,
	 pub console_server_port_count: i64,
	 pub rear_port_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub device_bay_count: i64,
	 pub name: Option<String>,
	 pub inventory_item_count: i64,
	/// Chassis serial number, assigned by the manufacturer
	 pub serial: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	 pub position: Option<f64>,
	 pub description: String,
	 pub module_bay_count: i64,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub front_port_count: i64,
	 pub power_outlet_count: i64,
	 pub vc_position: Option<i64>,
	/// Virtual chassis master election priority
	 pub vc_priority: Option<i64>,
	 pub console_port_count: i64,
	 pub comments: String,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub airflow: serde_json::value::Value,
	 pub custom_fields: serde_json::value::Value,
	 pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedServiceTemplateList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<ServiceTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceTemplate {
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub poe_type: Option<serde_json::value::Value>,
	/// Physical label
	 pub label: String,
	 pub id: i64,
	 pub poe_mode: Option<serde_json::value::Value>,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub enabled: bool,
	 pub display: String,
	 pub mgmt_only: bool,
	 pub rf_role: Option<serde_json::value::Value>,
	 pub created: Option<String>,
	 pub r#type: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualChassis {
	 pub display: String,
	 pub name: String,
	 pub id: i64,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLANGroup {
	 pub tags: Vec<NestedTag>,
	 pub name: String,
	 pub scope_type: Option<String>,
	 pub utilization: String,
	 pub created: Option<String>,
	 pub display: String,
	/// Lowest permissible ID of a child VLAN
	 pub min_vid: i64,
	 pub vlan_count: i64,
	 pub last_updated: Option<String>,
	 pub description: String,
	/// Highest permissible ID of a child VLAN
	 pub max_vid: i64,
	 pub id: i64,
	 pub url: String,
	 pub slug: String,
	 pub scope_id: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPanelRequest {
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSiteGroup {
	 pub name: String,
	 pub url: String,
	 pub display: String,
	 pub id: i64,
	 pub slug: String,
	 pub _depth: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomLinkList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<CustomLink>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBay {
	 pub id: i64,
	 pub name: String,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	/// Physical label
	 pub label: String,
	 pub url: String,
	 pub created: Option<String>,
	 pub tags: Vec<NestedTag>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRegionList {
	 pub count: i64,
	 pub results: Vec<Region>,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInterfaceRequest {
	 pub name: String,
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
	 pub r#type: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	 pub mode: String,
	 pub vrf: Option<i64>,
	 pub device: i64,
	 pub lag: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	 pub duplex: Option<String>,
	/// This interface is used only for out-of-band management
	 pub mgmt_only: bool,
	/// * `ap` - Access point
	/// * `station` - Station
	 pub rf_role: String,
	 pub bridge: Option<i64>,
	 pub parent: Option<i64>,
	 pub wwn: Option<String>,
	 pub speed: Option<i64>,
	 pub wireless_lans: Vec<i64>,
	 pub custom_fields: serde_json::value::Value,
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
	 pub vdcs: Vec<i64>,
	/// Physical label
	 pub label: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub untagged_vlan: Option<i64>,
	 pub tagged_vlans: Vec<i64>,
	 pub module: Option<i64>,
	 pub enabled: bool,
	 pub mac_address: Option<String>,
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
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub mtu: Option<i64>,
	/// Populated by selected channel (if set)
	 pub rf_channel_frequency: Option<f64>,
	/// Populated by selected channel (if set)
	 pub rf_channel_width: Option<f64>,
	 pub tx_power: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLANGroup {
	 pub name: String,
	 pub _depth: i64,
	 pub url: String,
	 pub display: String,
	 pub slug: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLink {
	 pub url: String,
	 pub status: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub id: i64,
	 pub display: String,
	 pub description: String,
	 pub last_updated: Option<String>,
	 pub auth_cipher: serde_json::value::Value,
	 pub comments: String,
	 pub auth_type: serde_json::value::Value,
	 pub auth_psk: String,
	 pub ssid: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedExportTemplateList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<ExportTemplate>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPort {
	 pub url: String,
	 pub cable_end: String,
	 pub name: String,
	 pub id: i64,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
	 pub connected_endpoints: Vec<String>,
	 pub connected_endpoints_reachable: bool,
	 pub connected_endpoints_type: String,
	 pub _occupied: bool,
	 pub display: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub tags: Vec<NestedTag>,
	 pub r#type: Option<serde_json::value::Value>,
	/// Physical label
	 pub label: String,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
	 pub link_peers: Vec<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIKEPolicyList {
	 pub previous: Option<String>,
	 pub results: Vec<IKEPolicy>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsolePortRequest {
	 pub tags: Vec<NestedTagRequest>,
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
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
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
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContentType {
	 pub id: i64,
	 pub url: String,
	 pub display: String,
	 pub app_label: String,
	 pub model: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterGroup {
	 pub created: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub cluster_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub display: String,
	 pub slug: String,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub description: String,
	 pub id: i64,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCluster {
	 pub url: String,
	 pub display: String,
	 pub name: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedClusterGroupRequest {
	 pub description: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerOutletRequest {
	 pub device: i64,
	 pub power_port: Option<i64>,
	 pub description: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	 pub feed_leg: String,
	 pub custom_fields: serde_json::value::Value,
	 pub module: Option<i64>,
	/// Physical label
	 pub label: String,
	 pub tags: Vec<NestedTagRequest>,
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
	 pub r#type: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelTerminationList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<TunnelTermination>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutlet {
	 pub created: Option<String>,
	 pub _occupied: bool,
	 pub link_peers: Vec<String>,
	 pub connected_endpoints: Vec<String>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub feed_leg: Option<serde_json::value::Value>,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub tags: Vec<NestedTag>,
	 pub connected_endpoints_type: String,
	 pub cable_end: String,
	 pub name: String,
	 pub display: String,
	 pub id: i64,
	 pub description: String,
	/// Physical label
	 pub label: String,
	 pub r#type: Option<serde_json::value::Value>,
	 pub connected_endpoints_reachable: bool,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteRequest {
	/// Physical location of the building
	 pub physical_address: String,
	/// Local facility ID or description
	 pub facility: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	 pub status: String,
	/// If different from the physical address
	 pub shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	/// Full name of the site
	 pub name: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub comments: String,
	 pub time_zone: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub asns: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRole {
	 pub name: String,
	 pub id: i64,
	 pub slug: String,
	 pub url: String,
	 pub display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedConfigTemplateRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceTemplateRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	 pub protocol: String,
	 pub description: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataSource {
	 pub status: serde_json::value::Value,
	 pub comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	 pub ignore_rules: String,
	 pub r#type: serde_json::value::Value,
	 pub enabled: bool,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	 pub name: String,
	 pub url: String,
	 pub file_count: i64,
	 pub id: i64,
	 pub display: String,
	 pub description: String,
	 pub source_url: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagRequest {
	 pub slug: String,
	 pub name: String,
	 pub description: String,
	 pub object_types: Vec<String>,
	 pub color: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomLink {
	 pub enabled: bool,
	 pub url: String,
	 pub last_updated: Option<String>,
	/// Links with the same group will appear as a dropdown menu
	 pub group_name: String,
	/// Jinja2 template code for link text
	 pub link_text: String,
	 pub name: String,
	 pub id: i64,
	 pub content_types: Vec<String>,
	/// Jinja2 template code for link URL
	 pub link_url: String,
	 pub display: String,
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
	 pub button_class: String,
	/// Force link to open in a new window
	 pub new_window: bool,
	 pub weight: i64,
	 pub created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomLinkRequest {
	 pub content_types: Vec<String>,
	 pub name: String,
	 pub weight: i64,
	 pub enabled: bool,
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
	 pub button_class: String,
	/// Jinja2 template code for link text
	 pub link_text: String,
	/// Links with the same group will appear as a dropdown menu
	 pub group_name: String,
	/// Force link to open in a new window
	 pub new_window: bool,
	/// Jinja2 template code for link URL
	 pub link_url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRole {
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Site {
	/// Full name of the site
	 pub name: String,
	 pub created: Option<String>,
	 pub description: String,
	 pub slug: String,
	/// Local facility ID or description
	 pub facility: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub circuit_count: i64,
	 pub id: i64,
	 pub asns: Vec<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub prefix_count: i64,
	 pub status: serde_json::value::Value,
	 pub comments: String,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub time_zone: Option<String>,
	 pub device_count: i64,
	 pub rack_count: i64,
	 pub virtualmachine_count: i64,
	 pub vlan_count: i64,
	 pub url: String,
	/// Physical location of the building
	 pub physical_address: String,
	/// If different from the physical address
	 pub shipping_address: String,
	 pub last_updated: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VLAN {
	/// Numeric VLAN ID (1-4094)
	 pub vid: i64,
	 pub comments: String,
	 pub name: String,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub created: Option<String>,
	 pub display: String,
	 pub prefix_count: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub id: i64,
	 pub tags: Vec<NestedTag>,
	 pub status: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRIRRequest {
	/// IP space managed by this RIR is considered private
	 pub is_private: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub slug: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableEventRuleRequest {
	/// Triggers when a job for a matching object is started.
	 pub type_job_start: bool,
	/// Triggers when a matching object is updated.
	 pub type_update: bool,
	 pub action_object_id: Option<i64>,
	 pub action_object_type: String,
	/// Triggers when a matching object is created.
	 pub type_create: bool,
	/// Triggers when a matching object is deleted.
	 pub type_delete: bool,
	 pub enabled: bool,
	 pub content_types: Vec<String>,
	/// Triggers when a job for a matching object terminates.
	 pub type_job_end: bool,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	 pub action_type: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectPermission {
	 pub description: String,
	 pub display: String,
	 pub id: i64,
	 pub object_types: Vec<String>,
	 pub users: Vec<i64>,
	 pub name: String,
	 pub enabled: bool,
	 pub groups: Vec<i64>,
	/// The list of actions granted by this permission
	 pub actions: Vec<String>,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableASNRangeRequest {
	 pub start: i64,
	 pub name: String,
	 pub description: String,
	 pub end: i64,
	 pub rir: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableTerminationRequest {
	 pub termination_id: i64,
	 pub termination_type: String,
	 pub cable: i64,
	/// * `A` - A
	/// * `B` - B
	 pub cable_end: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTunnelGroupRequest {
	 pub name: String,
	 pub slug: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConfigTemplateRequest {
	/// Remote data source
	 pub data_source: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub data_file: Option<i64>,
	 pub name: String,
	/// Jinja2 template code.
	 pub template_code: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableWirelessLANRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub tenant: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub auth_psk: String,
	 pub comments: String,
	 pub group: Option<i64>,
	 pub description: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	 pub auth_cipher: String,
	 pub ssid: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	 pub auth_type: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub vlan: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualMachineWithConfigContextList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<VirtualMachineWithConfigContext>,
	 pub count: i64,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConfigTemplateRequest {
	/// Jinja2 template code.
	 pub template_code: String,
	 pub name: String,
	 pub data_file: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Remote data source
	 pub data_source: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConfigTemplateList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub results: Vec<ConfigTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceBayTemplate {
	/// Physical label
	 pub label: String,
	 pub last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub description: String,
	 pub id: i64,
	 pub url: String,
	 pub display: String,
	 pub created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableBookmarkRequest {
	 pub object_type: String,
	 pub user: i64,
	 pub object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedModule {
	 pub url: String,
	 pub display: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedRoleRequest {
	 pub name: String,
	 pub slug: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub weight: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritablePowerPortTemplateRequest {
	 pub module_type: Option<i64>,
	/// Physical label
	 pub label: String,
	 pub description: String,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
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
	 pub r#type: String,
	 pub device_type: Option<i64>,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceTemplate {
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPRange {
	 pub comments: String,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub family: serde_json::value::Value,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	 pub id: i64,
	 pub display: String,
	 pub size: i64,
	 pub description: String,
	 pub start_address: String,
	 pub status: serde_json::value::Value,
	 pub end_address: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLANGroupList {
	 pub previous: Option<String>,
	 pub results: Vec<WirelessLANGroup>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTenant {
	 pub display: String,
	 pub url: String,
	 pub id: i64,
	 pub slug: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDataSourceRequest {
	 pub enabled: bool,
	 pub description: String,
	 pub comments: String,
	 pub r#type: String,
	 pub source_url: String,
	/// Patterns (one per line) matching files to ignore when syncing
	 pub ignore_rules: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Job {
	 pub display: String,
	 pub name: String,
	 pub started: Option<String>,
	 pub url: String,
	 pub id: i64,
	 pub scheduled: Option<String>,
	 pub error: String,
	 pub job_id: String,
	 pub object_id: Option<i64>,
	/// Recurrence interval (in minutes)
	 pub interval: Option<i64>,
	 pub object_type: String,
	 pub completed: Option<String>,
	 pub status: serde_json::value::Value,
	 pub created: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserRequest {
	/// Designates whether the user can log into this admin site.
	 pub is_staff: bool,
	 pub date_joined: String,
	 pub email: String,
	 pub password: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	 pub username: String,
	 pub first_name: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	 pub is_active: bool,
	 pub groups: Vec<i64>,
	 pub last_name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderNetworkRequest {
	 pub provider: i64,
	 pub name: String,
	 pub service_id: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableLocationRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	 pub status: String,
	 pub parent: Option<i64>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tenant: Option<i64>,
	 pub site: i64,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedFHRPGroup {
	 pub id: i64,
	 pub group_id: i64,
	 pub display: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	 pub protocol: String,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCableTerminationRequest {
	/// * `A` - A
	/// * `B` - B
	 pub cable_end: String,
	 pub cable: i64,
	 pub termination_type: String,
	 pub termination_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedSavedFilterRequest {
	 pub name: String,
	 pub enabled: bool,
	 pub shared: bool,
	 pub user: Option<i64>,
	 pub content_types: Vec<String>,
	 pub description: String,
	 pub weight: i64,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactGroup {
	 pub display: String,
	 pub id: i64,
	 pub name: String,
	 pub slug: String,
	 pub url: String,
	 pub _depth: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPSecProfile {
	 pub display: String,
	 pub id: i64,
	 pub name: String,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRole {
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	 pub name: String,
	 pub slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
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
	 pub r#type: String,
	 pub positions: i64,
	/// Physical label
	 pub label: String,
	 pub description: String,
	 pub color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedJobList {
	 pub results: Vec<Job>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTagList {
	 pub results: Vec<Tag>,
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableExportTemplateRequest {
	/// Defaults to <code>text/plain; charset=utf-8</code>
	 pub mime_type: String,
	/// Extension to append to the rendered filename
	 pub file_extension: String,
	/// Remote data source
	 pub data_source: Option<i64>,
	 pub content_types: Vec<String>,
	/// Download file as attachment
	 pub as_attachment: bool,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	 pub template_code: String,
	 pub description: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantRequest {
	 pub slug: String,
	 pub description: String,
	 pub comments: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableClusterRequest {
	 pub tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	 pub status: String,
	 pub name: String,
	 pub group: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub tenant: Option<i64>,
	 pub site: Option<i64>,
	 pub r#type: i64,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIKEProposalRequest {
	/// Security association lifetime (in seconds)
	 pub sa_lifetime: Option<i64>,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	 pub authentication_method: String,
	 pub comments: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	 pub authentication_algorithm: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	 pub encryption_algorithm: String,
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
	 pub group: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVLANRequest {
	/// Numeric VLAN ID (1-4094)
	 pub vid: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	/// The primary function of this VLAN
	 pub role: Option<i64>,
	 pub tenant: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	/// VLAN group (optional)
	 pub group: Option<i64>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	/// The specific site to which this VLAN is assigned (if any)
	 pub site: Option<i64>,
	 pub description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactGroupRequest {
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVMInterface {
	 pub id: i64,
	 pub name: String,
	 pub url: String,
	 pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInterfaceList {
	 pub next: Option<String>,
	 pub results: Vec<Interface>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContactRoleRequest {
	 pub name: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableL2VPNTerminationRequest {
	 pub assigned_object_id: i64,
	 pub assigned_object_type: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub l2vpn: i64,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSavedFilterList {
	 pub previous: Option<String>,
	 pub results: Vec<SavedFilter>,
	 pub next: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleTypeRequest {
	 pub description: String,
	 pub weight: Option<f64>,
	/// Discrete part number (optional)
	 pub part_number: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: Option<String>,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub model: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRangeRequest {
	 pub description: String,
	 pub end: i64,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub start: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPAddressRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub assigned_object_type: Option<String>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub address: String,
	/// Hostname or FQDN (not case-sensitive)
	 pub dns_name: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	 pub status: String,
	 pub assigned_object_id: Option<i64>,
	 pub comments: String,
	/// * `loopback` - Loopback
	/// * `secondary` - Secondary
	/// * `anycast` - Anycast
	/// * `vip` - VIP
	/// * `vrrp` - VRRP
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `carp` - CARP
	 pub role: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPanel {
	 pub powerfeed_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub url: String,
	 pub display: String,
	 pub name: String,
	 pub description: String,
	 pub created: Option<String>,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRequest {
	 pub facility_id: Option<String>,
	/// Starting unit for rack
	 pub starting_unit: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: Option<String>,
	/// Outer dimension of rack (depth)
	 pub outer_depth: Option<i64>,
	/// Height in rack units
	 pub u_height: i64,
	 pub description: String,
	/// Maximum load capacity for the rack
	 pub max_weight: Option<i64>,
	/// Units are numbered top-to-bottom
	 pub desc_units: bool,
	 pub name: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	 pub outer_unit: Option<String>,
	/// Outer dimension of rack (width)
	 pub outer_width: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	 pub width: i64,
	 pub serial: String,
	/// A unique tag used to identify this rack
	 pub asset_tag: Option<String>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	 pub r#type: Option<String>,
	 pub weight: Option<f64>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	 pub mounting_depth: Option<i64>,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelGroupRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTenantGroupRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub parent: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTokenRequest {
	 pub user: i64,
	 pub expires: Option<String>,
	 pub key: String,
	/// Permit create/update/delete operations using this key
	 pub write_enabled: bool,
	 pub last_used: Option<String>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TunnelGroup {
	 pub url: String,
	 pub description: String,
	 pub slug: String,
	 pub id: i64,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	 pub display: String,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tunnel_count: i64,
	 pub tags: Vec<NestedTag>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDeviceRequest {
	 pub name: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigContextRequest {
	 pub device_types: Vec<i64>,
	 pub weight: i64,
	 pub site_groups: Vec<i64>,
	 pub locations: Vec<i64>,
	 pub cluster_groups: Vec<i64>,
	 pub roles: Vec<i64>,
	 pub tenants: Vec<i64>,
	 pub clusters: Vec<i64>,
	 pub description: String,
	 pub tags: Vec<String>,
	 pub tenant_groups: Vec<i64>,
	 pub sites: Vec<i64>,
	 pub regions: Vec<i64>,
	 pub platforms: Vec<i64>,
	 pub name: String,
	 pub cluster_types: Vec<i64>,
	 pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedImageAttachmentList {
	 pub results: Vec<ImageAttachment>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactGroupRequest {
	 pub description: String,
	 pub slug: String,
	 pub parent: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInterface {
	 pub id: i64,
	 pub _occupied: bool,
	 pub display: String,
	 pub name: String,
	 pub cable: Option<i64>,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	 pub username: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCustomFieldChoiceSetRequest {
	/// Choices are automatically ordered alphabetically
	 pub order_alphabetically: bool,
	 pub extra_choices: Option<Vec<Vec<String>>>,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	 pub base_choices: String,
	 pub name: String,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualChassisRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsoleServerPortRequest {
	 pub module: Option<i64>,
	/// Physical label
	 pub label: String,
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
	 pub speed: Option<i64>,
	 pub device: i64,
	 pub tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub name: String,
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
	 pub r#type: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
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
	 pub ui_visible: String,
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
	/// Minimum allowed value (for numeric fields)
	 pub validation_minimum: Option<i64>,
	 pub content_types: Vec<String>,
	 pub object_type: String,
	/// Custom fields within the same group will be displayed together
	 pub group_name: String,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	 pub ui_editable: String,
	/// Replicate this value when cloning objects
	 pub is_cloneable: bool,
	/// Internal field name
	 pub name: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	 pub filter_logic: String,
	/// Fields with higher weights appear lower in a form.
	 pub weight: i64,
	/// Maximum allowed value (for numeric fields)
	 pub validation_maximum: Option<i64>,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	 pub validation_regex: String,
	 pub choice_set: Option<i64>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	 pub label: String,
	 pub description: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	 pub search_weight: i64,
	/// If true, this field is required when creating new objects or editing an existing object.
	 pub required: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<Tunnel>,
	 pub count: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableRegionRequest {
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub name: String,
	 pub parent: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceWithConfigContextList {
	 pub results: Vec<DeviceWithConfigContext>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegionRequest {
	 pub slug: String,
	 pub name: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ASNRequest {
	 pub description: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// 16- or 32-bit autonomous system number
	 pub asn: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	 pub interface_type: String,
	 pub priority: i64,
	 pub group: i64,
	 pub interface_id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderRequest {
	 pub slug: String,
	 pub asns: Vec<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub accounts: Vec<i64>,
	 pub comments: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	/// Full name of the provider
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFrontPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub device_type: Option<i64>,
	 pub rear_port_position: i64,
	 pub description: String,
	 pub color: String,
	 pub rear_port: i64,
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
	 pub r#type: String,
	 pub module_type: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemTemplateRequest {
	/// Physical label
	 pub label: String,
	 pub component_id: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub description: String,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub parent: Option<i64>,
	 pub component_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedIPSecProposalList {
	 pub next: Option<String>,
	 pub results: Vec<IPSecProposal>,
	 pub previous: Option<String>,
	 pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackRoleList {
	 pub count: i64,
	 pub results: Vec<RackRole>,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableL2VPNTerminationRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub l2vpn: i64,
	 pub assigned_object_id: i64,
	 pub assigned_object_type: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceRoleRequest {
	 pub color: String,
	/// Virtual machines may be assigned to this role
	 pub vm_role: bool,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedContactRoleRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTenantRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub group: Option<i64>,
	 pub description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableLocationRequest {
	 pub parent: Option<i64>,
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub site: i64,
	 pub name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	 pub status: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualMachineWithConfigContextRequest {
	 pub cluster: Option<i64>,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub device: Option<i64>,
	 pub memory: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub description: String,
	 pub disk: Option<i64>,
	 pub tenant: Option<i64>,
	 pub primary_ip4: Option<i64>,
	 pub role: Option<i64>,
	 pub platform: Option<i64>,
	 pub vcpus: Option<f64>,
	 pub primary_ip6: Option<i64>,
	 pub site: Option<i64>,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTenantGroupList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<TenantGroup>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPortRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub device: i64,
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
	 pub r#type: String,
	 pub description: String,
	 pub module: Option<i64>,
	/// Physical label
	 pub label: String,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLANGroupRequest {
	 pub slug: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub parent: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedWirelessLinkList {
	 pub next: Option<String>,
	 pub results: Vec<WirelessLink>,
	 pub previous: Option<String>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cluster {
	 pub status: serde_json::value::Value,
	 pub custom_fields: serde_json::value::Value,
	 pub id: i64,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub description: String,
	 pub comments: String,
	 pub name: String,
	 pub device_count: i64,
	 pub url: String,
	 pub virtualmachine_count: i64,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceRole {
	 pub slug: String,
	 pub color: String,
	 pub created: Option<String>,
	 pub name: String,
	/// Virtual machines may be assigned to this role
	 pub vm_role: bool,
	 pub id: i64,
	 pub device_count: i64,
	 pub display: String,
	 pub url: String,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub virtualmachine_count: i64,
	 pub description: String,
	 pub last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRegionRequest {
	 pub name: String,
	 pub slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableExportTemplateRequest {
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	 pub template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	 pub mime_type: String,
	/// Extension to append to the rendered filename
	 pub file_extension: String,
	 pub name: String,
	/// Remote data source
	 pub data_source: Option<i64>,
	 pub content_types: Vec<String>,
	 pub description: String,
	/// Download file as attachment
	 pub as_attachment: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedLocationList {
	 pub previous: Option<String>,
	 pub results: Vec<Location>,
	 pub next: Option<String>,
	 pub count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPortTemplateRequest {
	 pub device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
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
	 pub r#type: String,
	/// Physical label
	 pub label: String,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
	 pub module_type: Option<i64>,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
	 pub description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedTag {
	 pub url: String,
	 pub name: String,
	 pub slug: String,
	 pub color: String,
	 pub id: i64,
	 pub display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageAttachmentRequest {
	 pub name: String,
	 pub image: String,
	 pub image_height: i64,
	 pub image_width: i64,
	 pub content_type: String,
	 pub object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedManufacturer {
	 pub name: String,
	 pub display: String,
	 pub slug: String,
	 pub url: String,
	 pub id: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteGroup {
	 pub _depth: i64,
	 pub slug: String,
	 pub last_updated: Option<String>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub id: i64,
	 pub name: String,
	 pub site_count: i64,
	 pub display: String,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableConsolePortRequest {
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub description: String,
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
	 pub speed: Option<i64>,
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
	 pub r#type: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub device: i64,
	 pub module: Option<i64>,
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceRequest {
	 pub ports: Vec<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub ipaddresses: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	 pub protocol: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerPortTemplateList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub results: Vec<PowerPortTemplate>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TenantGroupRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableFrontPortRequest {
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub color: String,
	 pub module: Option<i64>,
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
	 pub r#type: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Physical label
	 pub label: String,
	 pub device: i64,
	 pub rear_port: i64,
	 pub tags: Vec<NestedTagRequest>,
	/// Mapped position on corresponding rear port
	 pub rear_port_position: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualDeviceContextList {
	 pub count: i64,
	 pub results: Vec<VirtualDeviceContext>,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldChoiceSetRequest {
	/// Choices are automatically ordered alphabetically
	 pub order_alphabetically: bool,
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	 pub base_choices: String,
	 pub extra_choices: Option<Vec<Vec<String>>>,
	 pub name: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableRackReservationRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub units: Vec<i64>,
	 pub user: i64,
	 pub tenant: Option<i64>,
	 pub rack: i64,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRoleRequest {
	 pub color: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDiskRequest {
	 pub name: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub size: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ManufacturerRequest {
	 pub slug: String,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedLocationRequest {
	 pub slug: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedDeviceBayList {
	 pub results: Vec<DeviceBay>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitTerminationRequest {
	/// Physical circuit speed
	 pub port_speed: Option<i64>,
	/// ID of the local cross-connect
	 pub xconnect_id: String,
	/// Patch panel ID and port number(s)
	 pub pp_info: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub tags: Vec<NestedTagRequest>,
	/// * `A` - A
	/// * `Z` - Z
	 pub term_side: String,
	/// Upstream speed, if different from port speed
	 pub upstream_speed: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedGroupList {
	 pub results: Vec<Group>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleRequest {
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub serial: String,
	 pub description: String,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCustomFieldChoiceSet {
	 pub id: i64,
	 pub url: String,
	 pub name: String,
	 pub display: String,
	 pub choices_count: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactAssignment {
	 pub url: String,
	 pub object_id: i64,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub content_type: String,
	 pub object: serde_json::value::Value,
	 pub priority: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrefixRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub prefix: String,
	/// All IP addresses within this prefix are considered usable
	 pub is_pool: bool,
	 pub description: String,
	/// Treat as 100% utilized
	 pub mark_utilized: bool,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tenant {
	 pub display: String,
	 pub circuit_count: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub site_count: i64,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub comments: String,
	 pub slug: String,
	 pub tags: Vec<NestedTag>,
	 pub device_count: i64,
	 pub prefix_count: i64,
	 pub description: String,
	 pub ipaddress_count: i64,
	 pub created: Option<String>,
	 pub rack_count: i64,
	 pub vlan_count: i64,
	 pub virtualmachine_count: i64,
	 pub name: String,
	 pub vrf_count: i64,
	 pub cluster_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteTarget {
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub created: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	/// Route target value (formatted in accordance with RFC 4360)
	 pub name: String,
	 pub description: String,
	 pub comments: String,
	 pub last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIPAddressRequest {
	 pub address: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecPolicyRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	 pub proposals: Vec<i64>,
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
	 pub pfs_group: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedVirtualMachine {
	 pub display: String,
	 pub name: String,
	 pub id: i64,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEProposalRequest {
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
	/// * `3des-cbc` - DES
	 pub encryption_algorithm: String,
	 pub description: String,
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
	 pub group: i64,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	 pub authentication_algorithm: String,
	/// Security association lifetime (in seconds)
	 pub sa_lifetime: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLANGroupRequest {
	 pub slug: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableClusterRequest {
	 pub group: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub site: Option<i64>,
	 pub r#type: i64,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	 pub status: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub tenant: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualDiskList {
	 pub previous: Option<String>,
	 pub next: Option<String>,
	 pub results: Vec<VirtualDisk>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
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
	/// * `power` - Power
	 pub r#type: String,
	 pub length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	 pub length_unit: String,
	 pub b_terminations: Vec<GenericObjectRequest>,
	 pub tenant: Option<i64>,
	 pub color: String,
	 pub a_terminations: Vec<GenericObjectRequest>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub description: String,
	 pub label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCableRequest {
	 pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCustomFieldList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<CustomField>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRoleRequest {
	 pub name: String,
	 pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTenantList {
	 pub previous: Option<String>,
	 pub results: Vec<Tenant>,
	 pub count: i64,
	 pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFrontPortList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<FrontPort>,
	 pub count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPort {
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub r#type: serde_json::value::Value,
	 pub cable_end: String,
	 pub description: String,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub url: String,
	 pub _occupied: bool,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub color: String,
	 pub tags: Vec<NestedTag>,
	 pub link_peers: Vec<String>,
	 pub created: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Mapped position on corresponding rear port
	 pub rear_port_position: i64,
	 pub id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConsolePortTemplateRequest {
	 pub module_type: Option<i64>,
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub device_type: Option<i64>,
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
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleTypeRequest {
	 pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: String,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub manufacturer: i64,
	 pub model: String,
	 pub comments: String,
	/// Discrete part number (optional)
	 pub part_number: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Token {
	 pub display: String,
	 pub created: String,
	 pub key: String,
	/// Permit create/update/delete operations using this key
	 pub write_enabled: bool,
	 pub description: String,
	 pub last_used: Option<String>,
	 pub id: i64,
	 pub url: String,
	 pub expires: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLAN {
	 pub custom_fields: serde_json::value::Value,
	 pub ssid: String,
	 pub url: String,
	 pub auth_psk: String,
	 pub status: serde_json::value::Value,
	 pub auth_cipher: serde_json::value::Value,
	 pub auth_type: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub display: String,
	 pub comments: String,
	 pub id: i64,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTenantRequest {
	 pub slug: String,
	 pub group: Option<i64>,
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedClusterGroupList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<ClusterGroup>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RackRole {
	 pub last_updated: Option<String>,
	 pub id: i64,
	 pub url: String,
	 pub name: String,
	 pub rack_count: i64,
	 pub slug: String,
	 pub display: String,
	 pub color: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub description: String,
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
	 pub protocol: String,
	 pub group_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLinkRequest {
	 pub ssid: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedRackRequest {
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPSecProposal {
	 pub id: i64,
	 pub url: String,
	/// Security association lifetime (in kilobytes)
	 pub sa_lifetime_data: Option<i64>,
	 pub comments: String,
	 pub created: Option<String>,
	 pub authentication_algorithm: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub custom_fields: serde_json::value::Value,
	 pub last_updated: Option<String>,
	 pub display: String,
	 pub description: String,
	 pub encryption_algorithm: serde_json::value::Value,
	/// Security association lifetime (seconds)
	 pub sa_lifetime_seconds: Option<i64>,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JournalEntry {
	 pub comments: String,
	 pub last_updated: Option<String>,
	 pub tags: Vec<NestedTag>,
	 pub assigned_object_type: String,
	 pub assigned_object_id: i64,
	 pub kind: serde_json::value::Value,
	 pub created_by: Option<i64>,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub url: String,
	 pub created: Option<String>,
	 pub display: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableCircuitTerminationRequest {
	 pub provider_network: Option<i64>,
	/// Patch panel ID and port number(s)
	 pub pp_info: String,
	/// Upstream speed, if different from port speed
	 pub upstream_speed: Option<i64>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub site: Option<i64>,
	 pub description: String,
	 pub circuit: i64,
	 pub tags: Vec<NestedTagRequest>,
	/// Physical circuit speed
	 pub port_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	 pub term_side: String,
	/// ID of the local cross-connect
	 pub xconnect_id: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerPanelRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
	 pub name: String,
	 pub site: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub location: Option<i64>,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableSiteRequest {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	 pub slug: String,
	 pub group: Option<i64>,
	 pub comments: String,
	 pub region: Option<i64>,
	 pub tenant: Option<i64>,
	 pub description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	 pub time_zone: Option<String>,
	 pub tags: Vec<NestedTagRequest>,
	/// Local facility ID or description
	 pub facility: String,
	 pub custom_fields: serde_json::value::Value,
	/// If different from the physical address
	 pub shipping_address: String,
	/// Physical location of the building
	 pub physical_address: String,
	/// Full name of the site
	 pub name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	 pub status: String,
	 pub asns: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualMachineWithConfigContext {
	 pub disk: Option<i64>,
	 pub vcpus: Option<f64>,
	 pub comments: String,
	 pub name: String,
	 pub interface_count: i64,
	 pub virtual_disk_count: i64,
	 pub display: String,
	 pub url: String,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub memory: Option<i64>,
	 pub tags: Vec<NestedTag>,
	 pub status: serde_json::value::Value,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	 pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectChange {
	 pub time: String,
	 pub action: serde_json::value::Value,
	 pub changed_object_id: i64,
	 pub request_id: String,
	 pub display: String,
	 pub user_name: String,
	 pub id: i64,
	 pub url: String,
	 pub changed_object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedCustomLinkRequest {
	 pub enabled: bool,
	/// Jinja2 template code for link text
	 pub link_text: String,
	 pub name: String,
	/// Links with the same group will appear as a dropdown menu
	 pub group_name: String,
	 pub content_types: Vec<String>,
	/// Jinja2 template code for link URL
	 pub link_url: String,
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
	 pub button_class: String,
	 pub weight: i64,
	/// Force link to open in a new window
	 pub new_window: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedConsolePortTemplateList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub results: Vec<ConsolePortTemplate>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableModuleRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub serial: String,
	 pub custom_fields: serde_json::value::Value,
	 pub device: i64,
	 pub module_bay: i64,
	 pub description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub module_type: i64,
	 pub comments: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCircuitTerminationList {
	 pub next: Option<String>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub results: Vec<CircuitTermination>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedTunnelGroupList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<TunnelGroup>,
	 pub previous: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRackList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<Rack>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableProviderAccountRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub account: String,
	 pub description: String,
	 pub provider: i64,
}

/// Representation of an ASN which does not exist in the database.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AvailableASN {
	 pub description: String,
	 pub asn: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedRearPortTemplateList {
	 pub results: Vec<RearPortTemplate>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub serial: String,
	/// This item was automatically discovered
	 pub discovered: bool,
	 pub description: String,
	/// A unique tag used to identify this item
	 pub asset_tag: Option<String>,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	/// Physical label
	 pub label: String,
	 pub component_id: Option<i64>,
	 pub component_type: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub parent: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JournalEntryRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
	 pub created_by: Option<i64>,
	 pub assigned_object_type: String,
	 pub assigned_object_id: i64,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	 pub kind: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableAggregateRequest {
	 pub tags: Vec<NestedTagRequest>,
	 pub date_added: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub prefix: String,
	 pub tenant: Option<i64>,
	 pub comments: String,
	 pub description: String,
	/// Regional Internet Registry responsible for this IP space
	 pub rir: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANGroupRequest {
	 pub description: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedVirtualChassisList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<VirtualChassis>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableInventoryItemRequest {
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub role: Option<i64>,
	 pub parent: Option<i64>,
	/// This item was automatically discovered
	 pub discovered: bool,
	 pub component_id: Option<i64>,
	/// Physical label
	 pub label: String,
	 pub manufacturer: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub serial: String,
	 pub component_type: Option<String>,
	 pub device: i64,
	/// A unique tag used to identify this item
	 pub asset_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedProviderList {
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub results: Vec<Provider>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleServerPort {
	 pub connected_endpoints_reachable: bool,
	 pub tags: Vec<NestedTag>,
	/// Physical label
	 pub label: String,
	 pub last_updated: Option<String>,
	 pub speed: Option<serde_json::value::Value>,
	 pub connected_endpoints: Vec<String>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// Return the type of the peer link terminations, or None.
	 pub link_peers_type: String,
	 pub connected_endpoints_type: String,
	 pub created: Option<String>,
	 pub r#type: serde_json::value::Value,
	 pub cable_end: String,
	 pub id: i64,
	 pub _occupied: bool,
	 pub url: String,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub display: String,
	 pub link_peers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedServiceList {
	 pub next: Option<String>,
	 pub count: i64,
	 pub results: Vec<Service>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualDiskRequest {
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub virtual_machine: i64,
	 pub size: i64,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableWirelessLANRequest {
	 pub description: String,
	 pub vlan: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	 pub auth_type: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	 pub auth_cipher: String,
	 pub group: Option<i64>,
	 pub auth_psk: String,
	 pub comments: String,
	 pub tenant: Option<i64>,
	 pub ssid: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	 pub status: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedSite {
	 pub slug: String,
	 pub id: i64,
	 pub url: String,
	/// Full name of the site
	 pub name: String,
	 pub display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldChoiceSet {
	 pub choices_count: String,
	 pub id: i64,
	 pub url: String,
	 pub extra_choices: Option<Vec<Vec<String>>>,
	 pub name: String,
	 pub created: Option<String>,
	/// Choices are automatically ordered alphabetically
	 pub order_alphabetically: bool,
	 pub last_updated: Option<String>,
	 pub description: String,
	 pub display: String,
	 pub base_choices: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigContext {
	 pub url: String,
	 pub is_active: bool,
	 pub cluster_types: Vec<i64>,
	 pub cluster_groups: Vec<i64>,
	 pub data_synced: Option<String>,
	 pub last_updated: Option<String>,
	 pub clusters: Vec<i64>,
	 pub site_groups: Vec<i64>,
	 pub regions: Vec<i64>,
	 pub device_types: Vec<i64>,
	 pub platforms: Vec<i64>,
	 pub display: String,
	 pub tenants: Vec<i64>,
	 pub locations: Vec<i64>,
	 pub description: String,
	 pub roles: Vec<i64>,
	/// Path to remote file (relative to data source root)
	 pub data_path: String,
	 pub created: Option<String>,
	 pub tenant_groups: Vec<i64>,
	 pub tags: Vec<String>,
	 pub name: String,
	 pub sites: Vec<i64>,
	 pub weight: i64,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceWithConfigContextRequest {
	 pub description: String,
	 pub vc_position: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	 pub serial: String,
	/// A unique tag used to identify this device
	 pub asset_tag: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub longitude: Option<f64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	 pub airflow: String,
	 pub name: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub tags: Vec<NestedTagRequest>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	 pub latitude: Option<f64>,
	/// * `front` - Front
	/// * `rear` - Rear
	 pub face: String,
	 pub position: Option<f64>,
	 pub comments: String,
	/// Virtual chassis master election priority
	 pub vc_priority: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedPowerFeedList {
	 pub previous: Option<String>,
	 pub results: Vec<PowerFeed>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
	 pub status: serde_json::value::Value,
	 pub id: i64,
	 pub device_count: i64,
	 pub name: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub created: Option<String>,
	 pub slug: String,
	 pub url: String,
	 pub _depth: i64,
	 pub rack_count: i64,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceBayTemplateRequest {
	/// Physical label
	 pub label: String,
	 pub device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableDeviceTypeRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	 pub airflow: String,
	 pub slug: String,
	 pub u_height: f64,
	 pub default_platform: Option<i64>,
	 pub model: String,
	 pub manufacturer: i64,
	 pub front_image: String,
	/// Discrete part number (optional)
	 pub part_number: String,
	/// Devices of this type are excluded when calculating rack utilization.
	 pub exclude_from_utilization: bool,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	 pub subdevice_role: String,
	 pub comments: String,
	 pub weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	 pub weight_unit: String,
	/// Device consumes both front and rear rack faces.
	 pub is_full_depth: bool,
	 pub rear_image: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableVirtualDeviceContextRequest {
	 pub device: Option<i64>,
	 pub comments: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	 pub status: String,
	 pub primary_ip6: Option<i64>,
	 pub name: String,
	/// Numeric identifier unique to the parent device
	 pub identifier: Option<i64>,
	 pub primary_ip4: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub tenant: Option<i64>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableIKEPolicyRequest {
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	 pub version: i64,
	 pub tags: Vec<NestedTagRequest>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	 pub mode: String,
	 pub comments: String,
	 pub custom_fields: serde_json::value::Value,
	 pub proposals: Vec<i64>,
	 pub description: String,
	 pub preshared_key: String,
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RearPortRequest {
	 pub name: String,
	/// Physical label
	 pub label: String,
	/// Number of front ports which may be mapped
	 pub positions: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub color: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
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
	 pub r#type: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
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
	 pub r#type: String,
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	 pub cid: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableTenantGroupRequest {
	 pub parent: Option<i64>,
	 pub description: String,
	 pub name: String,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRole {
	 pub url: String,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub slug: String,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
	 pub display: String,
	 pub id: i64,
	 pub name: String,
	 pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DashboardRequest {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfaceRequest {
	/// * `pd` - PD
	/// * `pse` - PSE
	 pub poe_mode: String,
	 pub wireless_lans: Vec<i64>,
	 pub vdcs: Vec<i64>,
	 pub speed: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	/// Populated by selected channel (if set)
	 pub rf_channel_width: Option<f64>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	 pub mode: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	 pub duplex: Option<String>,
	 pub mtu: Option<i64>,
	 pub name: String,
	/// Physical label
	 pub label: String,
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
	 pub custom_fields: serde_json::value::Value,
	 pub enabled: bool,
	/// This interface is used only for out-of-band management
	 pub mgmt_only: bool,
	/// * `ap` - Access point
	/// * `station` - Station
	 pub rf_role: String,
	 pub tagged_vlans: Vec<i64>,
	 pub mac_address: Option<String>,
	 pub wwn: Option<String>,
	 pub tx_power: Option<i64>,
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
	 pub r#type: String,
	/// Populated by selected channel (if set)
	 pub rf_channel_frequency: Option<f64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	 pub poe_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritablePowerFeedRequest {
	 pub tenant: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	 pub status: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	 pub r#type: String,
	 pub description: String,
	 pub voltage: i64,
	/// * `ac` - AC
	/// * `dc` - DC
	 pub supply: String,
	 pub rack: Option<i64>,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub custom_fields: serde_json::value::Value,
	 pub power_panel: i64,
	 pub amperage: i64,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	 pub phase: String,
	/// Maximum permissible draw (percentage)
	 pub max_utilization: i64,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletTemplate {
	 pub url: String,
	/// Physical label
	 pub label: String,
	 pub feed_leg: Option<serde_json::value::Value>,
	 pub id: i64,
	 pub r#type: Option<serde_json::value::Value>,
	 pub description: String,
	 pub display: String,
	 pub created: Option<String>,
	 pub last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableVirtualDeviceContextRequest {
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	 pub status: String,
	 pub primary_ip4: Option<i64>,
	 pub primary_ip6: Option<i64>,
	/// Numeric identifier unique to the parent device
	 pub identifier: Option<i64>,
	 pub tenant: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
	 pub name: String,
	 pub description: String,
	 pub comments: String,
	 pub device: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPortRequest {
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
	 pub r#type: Option<String>,
	/// Allocated power draw (watts)
	 pub allocated_draw: Option<i64>,
	/// Physical label
	 pub label: String,
	 pub description: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub custom_fields: serde_json::value::Value,
	/// Maximum power draw (watts)
	 pub maximum_draw: Option<i64>,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IKEPolicyRequest {
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	 pub version: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub proposals: Vec<i64>,
	 pub name: String,
	 pub description: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	 pub mode: String,
	 pub preshared_key: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedL2VPNTerminationRequest {
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedTunnelGroupRequest {
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTagRequest>,
	 pub slug: String,
	 pub description: String,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CircuitCircuitTermination {
	/// Upstream speed, if different from port speed
	 pub upstream_speed: Option<i64>,
	/// Physical circuit speed
	 pub port_speed: Option<i64>,
	 pub display: String,
	 pub url: String,
	 pub id: i64,
	 pub description: String,
	/// ID of the local cross-connect
	 pub xconnect_id: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedContact {
	 pub url: String,
	 pub id: i64,
	 pub display: String,
	 pub name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Region {
	 pub url: String,
	 pub display: String,
	 pub name: String,
	 pub site_count: i64,
	 pub id: i64,
	 pub slug: String,
	 pub created: Option<String>,
	 pub description: String,
	 pub _depth: i64,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub custom_fields: serde_json::value::Value,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenProvisionRequest {
	 pub expires: Option<String>,
	 pub username: String,
	 pub description: String,
	 pub password: String,
	/// Permit create/update/delete operations using this key
	 pub write_enabled: bool,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrontPortRearPortRequest {
	 pub description: String,
	 pub name: String,
	/// Physical label
	 pub label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterGroupRequest {
	 pub slug: String,
	 pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedInventoryItemTemplateList {
	 pub results: Vec<InventoryItemTemplate>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub next: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTunnelRequest {
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	 pub encapsulation: String,
	 pub ipsec_profile: Option<i64>,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub tenant: Option<i64>,
	 pub tunnel_id: Option<i64>,
	 pub name: String,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	 pub status: String,
	 pub group: Option<i64>,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedDataSource {
	 pub display: String,
	 pub url: String,
	 pub id: i64,
	 pub name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPlatformRequest {
	 pub name: String,
	 pub slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableConfigContextRequest {
	 pub tenants: Vec<i64>,
	 pub name: String,
	 pub device_types: Vec<i64>,
	 pub roles: Vec<i64>,
	 pub cluster_groups: Vec<i64>,
	 pub regions: Vec<i64>,
	 pub tags: Vec<String>,
	 pub weight: i64,
	 pub locations: Vec<i64>,
	 pub platforms: Vec<i64>,
	 pub tenant_groups: Vec<i64>,
	/// Remote data source
	 pub data_source: Option<i64>,
	 pub clusters: Vec<i64>,
	 pub cluster_types: Vec<i64>,
	 pub is_active: bool,
	 pub site_groups: Vec<i64>,
	 pub description: String,
	 pub sites: Vec<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleRequest {
	 pub weight: i64,
	 pub name: String,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModuleBayRequest {
	/// Physical label
	 pub label: String,
	/// Identifier to reference when renaming installed components
	 pub position: String,
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub description: String,
	 pub custom_fields: serde_json::value::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedFHRPGroupAssignmentList {
	 pub results: Vec<FHRPGroupAssignment>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedTagRequest {
	 pub object_types: Vec<String>,
	 pub slug: String,
	 pub color: String,
	 pub name: String,
	 pub description: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableInventoryItemRequest {
	 pub name: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub manufacturer: Option<i64>,
	 pub description: String,
	/// A unique tag used to identify this item
	 pub asset_tag: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	/// Manufacturer-assigned part identifier
	 pub part_id: String,
	 pub role: Option<i64>,
	 pub serial: String,
	 pub component_type: Option<String>,
	/// This item was automatically discovered
	 pub discovered: bool,
	 pub component_id: Option<i64>,
	 pub device: i64,
	 pub parent: Option<i64>,
	/// Physical label
	 pub label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VRF {
	 pub comments: String,
	 pub description: String,
	 pub tags: Vec<NestedTag>,
	 pub prefix_count: i64,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	 pub enforce_unique: bool,
	 pub display: String,
	 pub id: i64,
	 pub url: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	 pub rd: Option<String>,
	 pub import_targets: Vec<i64>,
	 pub export_targets: Vec<i64>,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	 pub ipaddress_count: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableIPAddressRequest {
	 pub address: String,
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
	 pub role: String,
	 pub custom_fields: serde_json::value::Value,
	 pub assigned_object_id: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub comments: String,
	 pub assigned_object_type: Option<String>,
	/// Hostname or FQDN (not case-sensitive)
	 pub dns_name: String,
	 pub description: String,
	 pub vrf: Option<i64>,
	 pub tenant: Option<i64>,
	/// The IP for which this address is the "outside" IP
	 pub nat_inside: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedSiteList {
	 pub results: Vec<Site>,
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactAssignmentRequest {
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	 pub priority: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub object_id: i64,
	 pub content_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroupAssignment {
	 pub url: String,
	 pub interface_id: i64,
	 pub interface_type: String,
	 pub last_updated: Option<String>,
	 pub priority: i64,
	 pub id: i64,
	 pub display: String,
	 pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedASNRangeList {
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<ASNRange>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableModuleBayTemplateRequest {
	/// Identifier to reference when renaming installed components
	 pub position: String,
	 pub device_type: i64,
	 pub description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryItemRole {
	 pub slug: String,
	 pub display: String,
	 pub id: i64,
	 pub description: String,
	 pub name: String,
	 pub created: Option<String>,
	 pub color: String,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub tags: Vec<NestedTag>,
	 pub inventoryitem_count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	 pub description: String,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	 pub base_choices: String,
	 pub name: String,
	/// Choices are automatically ordered alphabetically
	 pub order_alphabetically: bool,
	 pub extra_choices: Option<Vec<Vec<String>>>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLANGroup {
	 pub wirelesslan_count: i64,
	 pub created: Option<String>,
	 pub display: String,
	 pub tags: Vec<NestedTag>,
	 pub name: String,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub _depth: i64,
	 pub slug: String,
	 pub id: i64,
	 pub custom_fields: serde_json::value::Value,
	 pub description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableFrontPortTemplateRequest {
	 pub rear_port_position: i64,
	 pub description: String,
	 pub color: String,
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
	 pub r#type: String,
	 pub rear_port: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub module_type: Option<i64>,
	 pub device_type: Option<i64>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventRule {
	 pub action_object_id: Option<i64>,
	/// Triggers when a matching object is deleted.
	 pub type_delete: bool,
	/// Triggers when a job for a matching object terminates.
	 pub type_job_end: bool,
	 pub content_types: Vec<String>,
	/// Triggers when a job for a matching object is started.
	 pub type_job_start: bool,
	 pub action_object: serde_json::value::Value,
	 pub custom_fields: serde_json::value::Value,
	 pub action_type: serde_json::value::Value,
	 pub url: String,
	 pub action_object_type: String,
	/// Triggers when a matching object is created.
	 pub type_create: bool,
	 pub tags: Vec<NestedTag>,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
	 pub id: i64,
	 pub name: String,
	 pub enabled: bool,
	 pub display: String,
	 pub description: String,
	/// Triggers when a matching object is updated.
	 pub type_update: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPowerPanel {
	 pub name: String,
	 pub id: i64,
	 pub display: String,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FHRPGroup {
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	 pub auth_type: String,
	 pub custom_fields: serde_json::value::Value,
	 pub display: String,
	 pub created: Option<String>,
	 pub id: i64,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	 pub protocol: String,
	 pub group_id: i64,
	 pub auth_key: String,
	 pub ip_addresses: Vec<NestedIPAddress>,
	 pub tags: Vec<NestedTag>,
	 pub name: String,
	 pub description: String,
	 pub url: String,
	 pub last_updated: Option<String>,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerOutletRequest {
	 pub name: String,
	/// Physical label
	 pub label: String,
	 pub description: String,
	/// Treat as if a cable is connected
	 pub mark_connected: bool,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
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
	 pub r#type: Option<String>,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	 pub feed_leg: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WritableContactRequest {
	 pub comments: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub address: String,
	 pub group: Option<i64>,
	 pub link: String,
	 pub email: String,
	 pub description: String,
	 pub phone: String,
	 pub name: String,
	 pub title: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedLocation {
	 pub url: String,
	 pub _depth: i64,
	 pub name: String,
	 pub slug: String,
	 pub id: i64,
	 pub display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedPlatform {
	 pub url: String,
	 pub name: String,
	 pub slug: String,
	 pub display: String,
	 pub id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BookmarkRequest {
	 pub object_id: i64,
	 pub object_type: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableCircuitRequest {
	 pub description: String,
	/// Unique circuit ID
	 pub cid: String,
	 pub comments: String,
	 pub tenant: Option<i64>,
	 pub provider_account: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub termination_date: Option<String>,
	 pub custom_fields: serde_json::value::Value,
	 pub provider: i64,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	 pub status: String,
	 pub install_date: Option<String>,
	 pub r#type: i64,
	/// Committed rate
	 pub commit_rate: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedIKEPolicy {
	 pub display: String,
	 pub name: String,
	 pub id: i64,
	 pub url: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedClusterType {
	 pub id: i64,
	 pub url: String,
	 pub name: String,
	 pub display: String,
	 pub slug: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VirtualDeviceContextRequest {
	 pub name: String,
	/// Numeric identifier unique to the parent device
	 pub identifier: Option<i64>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	 pub status: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub comments: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WirelessLinkRequest {
	 pub ssid: String,
	 pub comments: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub tags: Vec<NestedTagRequest>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	 pub auth_cipher: String,
	 pub description: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	 pub auth_type: String,
	 pub auth_psk: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedWirelessLink {
	 pub ssid: String,
	 pub display: String,
	 pub id: i64,
	 pub url: String,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataFile {
	/// File path relative to the data source's root
	 pub path: String,
	 pub url: String,
	 pub display: String,
	 pub last_updated: String,
	 pub size: i64,
	 pub id: i64,
	/// SHA256 hash of the file data
	 pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedCableTerminationList {
	 pub count: i64,
	 pub next: Option<String>,
	 pub results: Vec<CableTermination>,
	 pub previous: Option<String>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableTunnelTerminationRequest {
	 pub tunnel: i64,
	 pub termination_id: Option<i64>,
	 pub outside_ip: Option<i64>,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	 pub role: String,
	 pub termination_type: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedWritableContactGroupRequest {
	 pub description: String,
	 pub parent: Option<i64>,
	 pub name: String,
	 pub slug: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub custom_fields: serde_json::value::Value,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceTemplate {
	 pub display: String,
	 pub ports: Vec<i64>,
	 pub id: i64,
	 pub name: String,
	 pub description: String,
	 pub protocol: serde_json::value::Value,
	 pub comments: String,
	 pub tags: Vec<NestedTag>,
	 pub url: String,
	 pub custom_fields: serde_json::value::Value,
	 pub last_updated: Option<String>,
	 pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedContentTypeList {
	 pub next: Option<String>,
	 pub count: i64,
	 pub previous: Option<String>,
	 pub results: Vec<ContentType>,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PowerPanelRequest {
	 pub comments: String,
	 pub description: String,
	 pub tags: Vec<NestedTagRequest>,
	 pub name: String,
	 pub custom_fields: serde_json::value::Value,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NestedInventoryItemRole {
	 pub url: String,
	 pub name: String,
	 pub slug: String,
	 pub display: String,
	 pub id: i64,
}

/// Adds support for custom fields and tags.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CableRequest {
	 pub tags: Vec<NestedTagRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	 pub status: String,
	 pub description: String,
	 pub comments: String,
	 pub b_terminations: Vec<GenericObjectRequest>,
	 pub custom_fields: serde_json::value::Value,
	 pub label: String,
	 pub a_terminations: Vec<GenericObjectRequest>,
	 pub color: String,
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
	 pub r#type: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	 pub length_unit: Option<String>,
	 pub length: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginatedAggregateList {
	 pub results: Vec<Aggregate>,
	 pub previous: Option<String>,
	 pub count: i64,
	 pub next: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchedGroupRequest {
	 pub name: String,
}


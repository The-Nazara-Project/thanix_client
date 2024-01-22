use serde_qs;
use reqwest::Url;
use crate::util::ThanixClient;
/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplateRequest {
	/// Physical label
	label: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContextRequest {
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	comments: String,
	description: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
}

pub struct PaginatedClusterTypeList {
	next: Option<Url>,
	count: i64,
	results: Vec<ClusterType>,
	previous: Option<Url>,
}

/// Representation of an IP address which does not exist in the database.
pub struct AvailableIP {
	description: String,
	family: i64,
	address: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInterfaceRequest {
	vrf: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	mac_address: Option<String>,
	enabled: bool,
	wwn: Option<String>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	wireless_lans: Vec<i64>,
	/// Physical label
	label: String,
	parent: Option<i64>,
	vdcs: Vec<i64>,
	mtu: Option<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	device: i64,
	module: Option<i64>,
	custom_fields: String,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
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
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	description: String,
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
	untagged_vlan: Option<i64>,
	speed: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	bridge: Option<i64>,
	tx_power: Option<i64>,
	lag: Option<i64>,
	name: String,
	tagged_vlans: Vec<i64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableConsolePortRequest {
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
	custom_fields: String,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	description: String,
	module: Option<i64>,
	/// Physical label
	label: String,
}

pub struct PaginatedRackList {
	results: Vec<Rack>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PowerPanelRequest {
	custom_fields: String,
	name: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenRequest {
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	expires: Option<String>,
	description: String,
	key: String,
	last_used: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct RoleRequest {
	name: String,
	slug: String,
	weight: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct Device {
	name: Option<String>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	airflow: String,
	position: Option<f64>,
	id: i64,
	display: String,
	power_port_count: i64,
	console_port_count: i64,
	url: Url,
	console_server_port_count: i64,
	front_port_count: i64,
	rear_port_count: i64,
	comments: String,
	inventory_item_count: i64,
	tags: Vec<NestedTag>,
	description: String,
	face: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	vc_position: Option<i64>,
	created: Option<String>,
	custom_fields: String,
	last_updated: Option<String>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	device_bay_count: i64,
	power_outlet_count: i64,
	interface_count: i64,
	module_bay_count: i64,
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegionRequest {
	slug: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedImageAttachmentRequest {
	image_width: i64,
	object_id: i64,
	name: String,
	content_type: String,
	image: String,
	image_height: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitRequest {
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	tenant: Option<i64>,
	install_date: Option<String>,
	provider_account: Option<i64>,
	/// Unique circuit ID
	cid: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	termination_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	provider: i64,
	r#type: i64,
}

pub struct PaginatedSiteGroupList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<SiteGroup>,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignment {
	object_id: i64,
	display: String,
	content_type: String,
	priority: String,
	object: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	url: Url,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContextRequest {
	regions: Vec<i64>,
	device_types: Vec<i64>,
	locations: Vec<i64>,
	tenants: Vec<i64>,
	tenant_groups: Vec<i64>,
	clusters: Vec<i64>,
	platforms: Vec<i64>,
	name: String,
	is_active: bool,
	site_groups: Vec<i64>,
	weight: i64,
	roles: Vec<i64>,
	cluster_types: Vec<i64>,
	cluster_groups: Vec<i64>,
	sites: Vec<i64>,
	description: String,
	tags: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct ServiceRequest {
	name: String,
	description: String,
	ipaddresses: Vec<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	custom_fields: String,
}

pub struct PaginatedIPSecPolicyList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<IPSecPolicy>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLink {
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
	display: String,
	id: i64,
	name: String,
	/// Jinja2 template code for link URL
	link_url: String,
	content_types: Vec<String>,
	url: Url,
	weight: i64,
	/// Force link to open in a new window
	new_window: bool,
	last_updated: Option<String>,
	created: Option<String>,
	enabled: bool,
	/// Jinja2 template code for link text
	link_text: String,
}

pub struct ContentType {
	model: String,
	app_label: String,
	id: i64,
	display: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
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

pub struct PaginatedModuleBayList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ModuleBay>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassis {
	id: i64,
	display: String,
	name: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedTagRequest {
	slug: String,
	description: String,
	object_types: Vec<String>,
	name: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleBayRequest {
	device: i64,
	custom_fields: String,
	/// Physical label
	label: String,
	installed_module: i64,
	description: String,
	/// Identifier to reference when renaming installed components
	position: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConfigContextRequest {
	name: String,
	locations: Vec<i64>,
	is_active: bool,
	tenants: Vec<i64>,
	cluster_types: Vec<i64>,
	roles: Vec<i64>,
	site_groups: Vec<i64>,
	regions: Vec<i64>,
	sites: Vec<i64>,
	tags: Vec<String>,
	device_types: Vec<i64>,
	platforms: Vec<i64>,
	description: String,
	clusters: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	cluster_groups: Vec<i64>,
	tenant_groups: Vec<i64>,
	weight: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedSavedFilterRequest {
	weight: i64,
	shared: bool,
	name: String,
	content_types: Vec<String>,
	description: String,
	user: Option<i64>,
	slug: String,
	enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProposalRequest {
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
	description: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableConsoleServerPortRequest {
	device: i64,
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
	name: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplate {
	url: Url,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	last_updated: Option<String>,
	id: i64,
	r#type: Option<String>,
	description: String,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterface {
	id: i64,
	url: Url,
	name: String,
	display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplateRequest {
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleBayRequest {
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	custom_fields: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroupRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRoleRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelTerminationRequest {
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	custom_fields: String,
	termination_type: String,
	tags: Vec<NestedTagRequest>,
	termination_id: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRFRequest {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleTypeRequest {
	model: String,
}

/// Adds support for custom fields and tags.
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
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	comments: String,
	/// Full name of the provider
	name: String,
	asns: Vec<i64>,
	accounts: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceTypeRequest {
	default_platform: Option<i64>,
	slug: String,
	/// Discrete part number (optional)
	part_number: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	u_height: f64,
	weight: Option<f64>,
	model: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	comments: String,
	custom_fields: String,
	front_image: String,
	rear_image: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	manufacturer: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProposalRequest {
	name: String,
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
	comments: String,
	description: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPortRequest {
	/// Physical label
	label: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	module: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	device: i64,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPN {
	identifier: Option<i64>,
	comments: String,
	name: String,
	tags: Vec<NestedTag>,
	description: String,
	r#type: String,
	custom_fields: String,
	import_targets: Vec<i64>,
	created: Option<String>,
	export_targets: Vec<i64>,
	url: Url,
	id: i64,
	display: String,
	last_updated: Option<String>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceType {
	model: String,
	id: i64,
	slug: String,
	url: Url,
	display: String,
}

pub struct PaginatedServiceList {
	results: Vec<Service>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableBookmarkRequest {
	object_id: i64,
	object_type: String,
	user: i64,
}

pub struct PaginatedProviderNetworkList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ProviderNetwork>,
}

pub struct PaginatedConsoleServerPortList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ConsoleServerPort>,
}

pub struct PaginatedL2VPNTerminationList {
	previous: Option<Url>,
	results: Vec<L2VPNTermination>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelGroupRequest {
	slug: String,
	name: String,
}

pub struct PaginatedFHRPGroupAssignmentList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<FHRPGroupAssignment>,
}

/// Adds support for custom fields and tags.
pub struct ModuleType {
	weight: Option<f64>,
	created: Option<String>,
	/// Discrete part number (optional)
	part_number: String,
	last_updated: Option<String>,
	model: String,
	id: i64,
	weight_unit: Option<String>,
	display: String,
	comments: String,
	url: Url,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

pub struct PaginatedUserList {
	previous: Option<Url>,
	count: i64,
	results: Vec<User>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderAccountRequest {
	account: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	comments: String,
	provider: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableLocationRequest {
	slug: String,
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	description: String,
	tenant: Option<i64>,
	custom_fields: String,
	site: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRoleRequest {
	slug: String,
	name: String,
}

/// Representation of a VLAN which does not exist in the database.
pub struct AvailableVLAN {
	vid: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableEventRuleRequest {
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	name: String,
	action_object_type: String,
	tags: Vec<NestedTagRequest>,
	content_types: Vec<String>,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	enabled: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	description: String,
	custom_fields: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_id: Option<i64>,
	/// Triggers when a matching object is created.
	type_create: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableExportTemplateRequest {
	name: String,
	content_types: Vec<String>,
	description: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Download file as attachment
	as_attachment: bool,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Remote data source
	data_source: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Platform {
	description: String,
	custom_fields: String,
	created: Option<String>,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	display: String,
	id: i64,
	last_updated: Option<String>,
	device_count: i64,
	virtualmachine_count: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleBay {
	/// Physical label
	label: String,
	url: Url,
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	id: i64,
	last_updated: Option<String>,
	description: String,
	display: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableAggregateRequest {
	comments: String,
	date_added: Option<String>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	custom_fields: String,
	prefix: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceRequest {
	device: Option<i64>,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	description: String,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	virtual_machine: Option<i64>,
	comments: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroup {
	url: Url,
	id: i64,
	name: String,
	slug: String,
	_depth: i64,
	display: String,
}

pub struct PaginatedPowerPanelList {
	results: Vec<PowerPanel>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ModuleTypeRequest {
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	description: String,
	weight: Option<f64>,
	tags: Vec<NestedTagRequest>,
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	custom_fields: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct RIR {
	/// IP space managed by this RIR is considered private
	is_private: bool,
	name: String,
	id: i64,
	url: Url,
	slug: String,
	description: String,
	custom_fields: String,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	aggregate_count: i64,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceTemplateRequest {
	custom_fields: String,
	description: String,
	comments: String,
	name: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableVRFRequest {
	description: String,
	comments: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	import_targets: Vec<i64>,
	tenant: Option<i64>,
	export_targets: Vec<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	custom_fields: String,
}

pub struct PaginatedConsolePortTemplateList {
	results: Vec<ConsolePortTemplate>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

pub struct PaginatedConsoleServerPortTemplateList {
	results: Vec<ConsoleServerPortTemplate>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct VRFRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
	export_targets: Vec<i64>,
	comments: String,
	description: String,
	import_targets: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderNetworkRequest {
	custom_fields: String,
	service_id: String,
	comments: String,
	provider: i64,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplate {
	id: i64,
	r#type: String,
	description: String,
	/// Physical label
	label: String,
	last_updated: Option<String>,
	created: Option<String>,
	display: String,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitRequest {
	/// Committed rate
	commit_rate: Option<i64>,
	comments: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	/// Unique circuit ID
	cid: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	termination_date: Option<String>,
	install_date: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedVLANGroupRequest {
	scope_id: Option<i64>,
	description: String,
	scope_type: Option<String>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplateRequest {
	/// Physical label
	label: String,
	description: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPanelRequest {
	description: String,
	site: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	location: Option<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterTypeRequest {
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	url: Url,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableModuleBayTemplateRequest {
	device_type: i64,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
}

pub struct PaginatedL2VPNList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<L2VPN>,
	count: i64,
}

pub struct PaginatedDeviceBayTemplateList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceBayTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	positions: i64,
	module_type: Option<i64>,
	device_type: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRole {
	id: i64,
	url: Url,
	display: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleRequest {
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	module_type: i64,
	device: i64,
	comments: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	module_bay: i64,
	serial: String,
}

/// Adds support for custom fields and tags.
pub struct RouteTarget {
	id: i64,
	created: Option<String>,
	tags: Vec<NestedTag>,
	url: Url,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	custom_fields: String,
	display: String,
	comments: String,
	last_updated: Option<String>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePrefixRequest {
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	custom_fields: String,
	tenant: Option<i64>,
	description: String,
	site: Option<i64>,
	/// The primary function of this prefix
	role: Option<i64>,
	vlan: Option<i64>,
	prefix: String,
	vrf: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Treat as 100% utilized
	mark_utilized: bool,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetworkRequest {
	name: String,
}

pub struct PaginatedWirelessLinkList {
	results: Vec<WirelessLink>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleRequest {
	module_type: i64,
	tags: Vec<NestedTagRequest>,
	comments: String,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	device: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	custom_fields: String,
	module_bay: i64,
	serial: String,
}

pub struct PaginatedObjectPermissionList {
	previous: Option<Url>,
	results: Vec<ObjectPermission>,
	next: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachment {
	created: Option<String>,
	object_id: i64,
	image_height: i64,
	image: Url,
	url: Url,
	id: i64,
	name: String,
	last_updated: Option<String>,
	content_type: String,
	image_width: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableConsoleServerPortRequest {
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
	tags: Vec<NestedTagRequest>,
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
	device: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	/// Physical label
	label: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableFrontPortTemplateRequest {
	color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	device_type: Option<i64>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVMInterfaceRequest {
	name: String,
	mtu: Option<i64>,
	virtual_machine: i64,
	bridge: Option<i64>,
	untagged_vlan: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	mac_address: Option<String>,
	parent: Option<i64>,
	tagged_vlans: Vec<i64>,
	enabled: bool,
	custom_fields: String,
	description: String,
	vrf: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVRFRequest {
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	comments: String,
	tenant: Option<i64>,
	description: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	import_targets: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	export_targets: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDeviceContextRequest {
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	tenant: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	primary_ip4: Option<i64>,
	primary_ip6: Option<i64>,
	comments: String,
	device: Option<i64>,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterTypeRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ASNRange {
	asn_count: i64,
	slug: String,
	name: String,
	id: i64,
	start: i64,
	display: String,
	description: String,
	custom_fields: String,
	url: Url,
	last_updated: Option<String>,
	created: Option<String>,
	tags: Vec<NestedTag>,
	end: i64,
}

/// Adds support for custom fields and tags.
pub struct RackReservation {
	id: i64,
	display: String,
	units: Vec<i64>,
	description: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	url: Url,
	comments: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
pub struct WritableRouteTargetRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	comments: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IPRange {
	family: String,
	display: String,
	tags: Vec<NestedTag>,
	size: i64,
	status: String,
	last_updated: Option<String>,
	comments: String,
	custom_fields: String,
	created: Option<String>,
	description: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	id: i64,
	end_address: String,
	url: Url,
	start_address: String,
}

/// Adds support for custom fields and tags.
pub struct IPRangeRequest {
	tags: Vec<NestedTagRequest>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	end_address: String,
	comments: String,
	start_address: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicy {
	name: String,
	url: Url,
	display: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSetRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignmentRequest {
	interface_id: i64,
	priority: i64,
	interface_type: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	module_type: Option<i64>,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplate {
	id: i64,
	display: String,
	description: String,
	url: Url,
	last_updated: Option<String>,
	created: Option<String>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilter {
	display: String,
	content_types: Vec<String>,
	user: Option<i64>,
	slug: String,
	weight: i64,
	id: i64,
	name: String,
	description: String,
	enabled: bool,
	shared: bool,
	url: Url,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct VirtualDiskRequest {
	custom_fields: String,
	name: String,
	size: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecPolicyRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	proposals: Vec<i64>,
	name: String,
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
	comments: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	parent: Option<i64>,
	description: String,
	component_id: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct VLANGroup {
	id: i64,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	utilization: String,
	created: Option<String>,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	scope_type: Option<String>,
	display: String,
	scope_id: Option<i64>,
	custom_fields: String,
	url: Url,
	description: String,
	slug: String,
	vlan_count: i64,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableExportTemplateRequest {
	/// Extension to append to the rendered filename
	file_extension: String,
	name: String,
	content_types: Vec<String>,
	/// Remote data source
	data_source: Option<i64>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Download file as attachment
	as_attachment: bool,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceRole {
	display: String,
	custom_fields: String,
	description: String,
	last_updated: Option<String>,
	slug: String,
	device_count: i64,
	color: String,
	url: Url,
	tags: Vec<NestedTag>,
	name: String,
	created: Option<String>,
	virtualmachine_count: i64,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	id: i64,
}

pub struct PaginatedWebhookList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Webhook>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroup {
	custom_fields: String,
	slug: String,
	display: String,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	_depth: i64,
	url: Url,
	id: i64,
	name: String,
	last_updated: Option<String>,
	contact_count: i64,
}

/// Adds support for custom fields and tags.
pub struct ASN {
	/// 16- or 32-bit autonomous system number
	asn: i64,
	provider_count: i64,
	created: Option<String>,
	description: String,
	display: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	url: Url,
	last_updated: Option<String>,
	comments: String,
	id: i64,
	site_count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplate {
	url: Url,
	r#type: String,
	/// Physical label
	label: String,
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLink {
	display: String,
	created: Option<String>,
	auth_type: String,
	id: i64,
	status: String,
	auth_psk: String,
	description: String,
	url: Url,
	tags: Vec<NestedTag>,
	custom_fields: String,
	comments: String,
	last_updated: Option<String>,
	auth_cipher: String,
	ssid: String,
}

/// Adds support for custom fields and tags.
pub struct EventRule {
	display: String,
	enabled: bool,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	action_type: String,
	name: String,
	action_object_id: Option<i64>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	last_updated: Option<String>,
	action_object: String,
	description: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	content_types: Vec<String>,
	custom_fields: String,
	action_object_type: String,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	tags: Vec<NestedTag>,
	created: Option<String>,
	url: Url,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderRequest {
	comments: String,
	/// Full name of the provider
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	asns: Vec<i64>,
	accounts: Vec<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplate {
	id: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRole {
	id: i64,
	url: Url,
	display: String,
	name: String,
	slug: String,
}

pub struct PaginatedASNList {
	results: Vec<ASN>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedRegionList {
	results: Vec<Region>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTag {
	slug: String,
	name: String,
	id: i64,
	color: String,
	display: String,
	url: Url,
}

pub struct PaginatedInventoryItemTemplateList {
	previous: Option<Url>,
	count: i64,
	results: Vec<InventoryItemTemplate>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccountRequest {
	account: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDataSourceRequest {
	enabled: bool,
	description: String,
	comments: String,
	r#type: String,
	source_url: String,
	name: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

/// Adds support for custom fields and tags.
pub struct RearPort {
	id: i64,
	/// Physical label
	label: String,
	last_updated: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// Number of front ports which may be mapped
	positions: i64,
	cable_end: String,
	link_peers: Vec<String>,
	name: String,
	custom_fields: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	_occupied: bool,
	url: Url,
	r#type: String,
	description: String,
	color: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	display: String,
}

pub struct PaginatedClusterList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Cluster>,
}

/// Adds support for custom fields and tags.
pub struct ConsolePortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	description: String,
	custom_fields: String,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroupRequest {
	name: String,
	slug: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroup {
	name: String,
	slug: String,
	description: String,
	custom_fields: String,
	display: String,
	url: Url,
	tenant_count: i64,
	last_updated: Option<String>,
	created: Option<String>,
	id: i64,
	_depth: i64,
	tags: Vec<NestedTag>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitTypeRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ContactRoleRequest {
	description: String,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccountRequest {
	name: String,
	account: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceBayRequest {
	device: i64,
	/// Physical label
	label: String,
	installed_device: Option<i64>,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplateRequest {
	description: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: Option<String>,
	mgmt_only: bool,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// Physical label
	label: String,
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

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInterfaceTemplateRequest {
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	enabled: bool,
	bridge: Option<i64>,
	/// Physical label
	label: String,
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
	module_type: Option<i64>,
	mgmt_only: bool,
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
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

pub struct PaginatedIPRangeList {
	count: i64,
	previous: Option<Url>,
	results: Vec<IPRange>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct InventoryItem {
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	custom_fields: String,
	display: String,
	component_type: Option<String>,
	/// This item was automatically discovered
	discovered: bool,
	url: Url,
	component_id: Option<i64>,
	name: String,
	description: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	_depth: i64,
	id: i64,
	/// Physical label
	label: String,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	serial: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct L2VPNTerminationRequest {
	assigned_object_id: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	assigned_object_type: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRIR {
	slug: String,
	id: i64,
	url: Url,
	display: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRoleRequest {
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	color: String,
	custom_fields: String,
}

pub struct PaginatedTunnelGroupList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<TunnelGroup>,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroupRequest {
	description: String,
	name: String,
	custom_fields: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRF {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
	id: i64,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPNTermination {
	id: i64,
	assigned_object_id: i64,
	url: Url,
	display: String,
	assigned_object_type: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedManufacturerRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturer {
	url: Url,
	id: i64,
	display: String,
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignment {
	last_updated: Option<String>,
	id: i64,
	created: Option<String>,
	interface_id: i64,
	priority: i64,
	display: String,
	url: Url,
	interface_type: String,
}

pub struct PaginatedCustomFieldChoiceSetList {
	previous: Option<Url>,
	results: Vec<CustomFieldChoiceSet>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedRoleRequest {
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	weight: i64,
	custom_fields: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuit {
	id: i64,
	display: String,
	url: Url,
	/// Unique circuit ID
	cid: String,
}

pub struct PaginatedTagList {
	results: Vec<Tag>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvision {
	id: i64,
	description: String,
	last_used: String,
	url: Url,
	display: String,
	key: String,
	created: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	expires: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRole {
	id: i64,
	name: String,
	slug: String,
	display: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	cid: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceType {
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	front_port_template_count: i64,
	device_bay_template_count: i64,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	inventory_item_template_count: i64,
	/// Discrete part number (optional)
	part_number: String,
	model: String,
	u_height: f64,
	airflow: Option<String>,
	console_port_template_count: i64,
	url: Url,
	interface_template_count: i64,
	tags: Vec<NestedTag>,
	comments: String,
	power_port_template_count: i64,
	weight_unit: Option<String>,
	description: String,
	created: Option<String>,
	rear_port_template_count: i64,
	id: i64,
	last_updated: Option<String>,
	device_count: i64,
	console_server_port_template_count: i64,
	slug: String,
	custom_fields: String,
	power_outlet_template_count: i64,
	subdevice_role: Option<String>,
	module_bay_template_count: i64,
	display: String,
	front_image: Url,
	weight: Option<f64>,
	rear_image: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCable {
	display: String,
	id: i64,
	url: Url,
	label: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitTypeRequest {
	name: String,
	custom_fields: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRoleRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBayRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Rack {
	custom_fields: String,
	powerfeed_count: i64,
	display: String,
	weight_unit: Option<String>,
	/// Starting unit for rack
	starting_unit: i64,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	outer_unit: Option<String>,
	description: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	weight: Option<f64>,
	name: String,
	comments: String,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	device_count: i64,
	status: String,
	id: i64,
	url: Url,
	r#type: Option<String>,
	width: String,
	serial: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	facility_id: Option<String>,
	/// Height in rack units
	u_height: i64,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceRequest {
	name: String,
	cable: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilterRequest {
	enabled: bool,
	shared: bool,
	description: String,
	name: String,
	slug: String,
	user: Option<i64>,
	content_types: Vec<String>,
	weight: i64,
}

pub struct PaginatedServiceTemplateList {
	results: Vec<ServiceTemplate>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedVLANGroupList {
	previous: Option<Url>,
	results: Vec<VLANGroup>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct Manufacturer {
	platform_count: i64,
	id: i64,
	display: String,
	slug: String,
	tags: Vec<NestedTag>,
	name: String,
	url: Url,
	custom_fields: String,
	created: Option<String>,
	description: String,
	devicetype_count: i64,
	last_updated: Option<String>,
	inventoryitem_count: i64,
}

/// Adds support for custom fields and tags.
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
	description: String,
	proposals: Vec<i64>,
	comments: String,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroupRequest {
	slug: String,
	name: String,
}

pub struct PaginatedProviderList {
	count: i64,
	previous: Option<Url>,
	results: Vec<Provider>,
	next: Option<Url>,
}

pub struct PaginatedIPSecProposalList {
	results: Vec<IPSecProposal>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableRegionRequest {
	custom_fields: String,
	name: String,
	slug: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WebhookRequest {
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	description: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	custom_fields: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
}

/// Adds support for custom fields and tags.
pub struct FrontPortRequest {
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	name: String,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	color: String,
	tags: Vec<NestedTagRequest>,
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
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelRequest {
	custom_fields: String,
	group: Option<i64>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	tenant: Option<i64>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	description: String,
	ipsec_profile: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	tunnel_id: Option<i64>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLink {
	url: Url,
	ssid: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PowerFeed {
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	amperage: i64,
	phase: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints_reachable: bool,
	_occupied: bool,
	status: String,
	link_peers: Vec<String>,
	r#type: String,
	display: String,
	name: String,
	cable_end: String,
	supply: String,
	comments: String,
	last_updated: Option<String>,
	created: Option<String>,
	connected_endpoints: Vec<String>,
	description: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	url: Url,
	id: i64,
	connected_endpoints_type: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	voltage: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRack {
	url: Url,
	display: String,
	id: i64,
	name: String,
}

pub struct PaginatedCircuitTypeList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<CircuitType>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct VirtualDisk {
	url: Url,
	tags: Vec<NestedTag>,
	size: i64,
	description: String,
	id: i64,
	custom_fields: String,
	name: String,
	last_updated: Option<String>,
	created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSet {
	id: i64,
	extra_choices: Option<Vec<Vec<String>>>,
	display: String,
	name: String,
	base_choices: String,
	choices_count: String,
	last_updated: Option<String>,
	url: Url,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	description: String,
	created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldRequest {
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Internal field name
	name: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
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
	object_type: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	content_types: Vec<String>,
	description: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerFeedRequest {
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	custom_fields: String,
	description: String,
	tenant: Option<i64>,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	name: String,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	comments: String,
	rack: Option<i64>,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	tags: Vec<NestedTagRequest>,
	amperage: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	power_panel: i64,
	voltage: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfileRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContact {
	id: i64,
	display: String,
	name: String,
	url: Url,
}

pub struct PaginatedBookmarkList {
	count: i64,
	results: Vec<Bookmark>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetwork {
	name: String,
	url: Url,
	display: String,
	id: i64,
}

pub struct PaginatedProviderAccountList {
	count: i64,
	next: Option<Url>,
	results: Vec<ProviderAccount>,
	previous: Option<Url>,
}

pub struct PaginatedIKEPolicyList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<IKEPolicy>,
}

/// Adds support for custom fields and tags.
pub struct IKEProposal {
	description: String,
	group: String,
	authentication_method: String,
	last_updated: Option<String>,
	encryption_algorithm: String,
	authentication_algorithm: String,
	id: i64,
	tags: Vec<NestedTag>,
	url: Url,
	name: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	created: Option<String>,
	custom_fields: String,
	display: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicy {
	display: String,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	url: Url,
	description: String,
	id: i64,
	mode: String,
	proposals: Vec<i64>,
	version: String,
	preshared_key: String,
	comments: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableContactGroupRequest {
	custom_fields: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePrefixRequest {
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	site: Option<i64>,
	vrf: Option<i64>,
	custom_fields: String,
	prefix: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	vlan: Option<i64>,
	comments: String,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// The primary function of this prefix
	role: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
}

/// Adds support for custom fields and tags.
pub struct Role {
	prefix_count: i64,
	vlan_count: i64,
	name: String,
	url: Url,
	custom_fields: String,
	created: Option<String>,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	weight: i64,
	description: String,
	display: String,
	id: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	priority: i64,
	interface_id: i64,
	group: i64,
	interface_type: String,
}

/// Adds support for custom fields and tags.
pub struct VLANRequest {
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	custom_fields: String,
}

pub struct Dashboard {
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsolePortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerPortTemplateRequest {
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
	module_type: Option<i64>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	device_type: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnel {
	display: String,
	url: Url,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroup {
	custom_fields: String,
	display: String,
	auth_key: String,
	last_updated: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	name: String,
	description: String,
	created: Option<String>,
	url: Url,
	group_id: i64,
	comments: String,
	ip_addresses: Vec<NestedIPAddress>,
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
pub struct ProviderNetworkRequest {
	custom_fields: String,
	name: String,
	service_id: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableRackReservationRequest {
	description: String,
	rack: i64,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	user: i64,
	units: Vec<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableSiteRequest {
	/// Local facility ID or description
	facility: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	custom_fields: String,
	slug: String,
	/// If different from the physical address
	shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	asns: Vec<i64>,
	description: String,
	region: Option<i64>,
	comments: String,
	tenant: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	tags: Vec<NestedTagRequest>,
	group: Option<i64>,
	/// Full name of the site
	name: String,
	/// Physical location of the building
	physical_address: String,
	time_zone: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableSiteRequest {
	/// If different from the physical address
	shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	group: Option<i64>,
	tags: Vec<NestedTagRequest>,
	time_zone: Option<String>,
	asns: Vec<i64>,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// Local facility ID or description
	facility: String,
	comments: String,
	slug: String,
	region: Option<i64>,
	/// Full name of the site
	name: String,
	tenant: Option<i64>,
	description: String,
	/// Physical location of the building
	physical_address: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	name: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualChassisRequest {
	description: String,
	comments: String,
	master: Option<i64>,
	domain: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroup {
	id: i64,
	display: String,
	name: String,
	_depth: i64,
	slug: String,
	url: Url,
}

pub struct PaginatedContactRoleList {
	count: i64,
	next: Option<Url>,
	results: Vec<ContactRole>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerFeedRequest {
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	tenant: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	custom_fields: String,
	rack: Option<i64>,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	tags: Vec<NestedTagRequest>,
	voltage: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	power_panel: i64,
	amperage: i64,
	description: String,
	comments: String,
	name: String,
}

pub struct PaginatedPowerPortTemplateList {
	next: Option<Url>,
	results: Vec<PowerPortTemplate>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableEventRuleRequest {
	content_types: Vec<String>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	action_object_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	description: String,
	enabled: bool,
	action_object_type: String,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLANRequest {
	description: String,
	vlan: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	ssid: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tenant: Option<i64>,
	auth_psk: String,
	group: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplate {
	/// Physical label
	label: String,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	created: Option<String>,
	id: i64,
	description: String,
	display: String,
	last_updated: Option<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableSiteGroupRequest {
	name: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	slug: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroupRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelTermination {
	display: String,
	role: String,
	last_updated: Option<String>,
	termination_type: String,
	id: i64,
	tags: Vec<NestedTag>,
	termination_id: Option<i64>,
	custom_fields: String,
	url: Url,
	created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsolePortTemplateRequest {
	description: String,
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: Option<i64>,
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

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSet {
	name: String,
	choices_count: String,
	id: i64,
	url: Url,
	display: String,
}

pub struct PaginatedInterfaceList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Interface>,
	count: i64,
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
pub struct NestedTunnelGroup {
	id: i64,
	name: String,
	display: String,
	url: Url,
	slug: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct PatchedWritableConfigTemplateRequest {
	description: String,
	name: String,
	/// Jinja2 template code.
	template_code: String,
	/// Remote data source
	data_source: Option<i64>,
	tags: Vec<NestedTagRequest>,
	data_file: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct DataSource {
	url: Url,
	status: String,
	file_count: i64,
	display: String,
	enabled: bool,
	r#type: String,
	created: Option<String>,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	name: String,
	description: String,
	id: i64,
	last_updated: Option<String>,
	source_url: String,
	comments: String,
}

pub struct PaginatedRouteTargetList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<RouteTarget>,
}

pub struct PaginatedPowerPortList {
	next: Option<Url>,
	results: Vec<PowerPort>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitTerminationRequest {
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	site: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	tags: Vec<NestedTagRequest>,
	provider_network: Option<i64>,
	custom_fields: String,
	circuit: i64,
	description: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IPAddress {
	role: String,
	display: String,
	family: String,
	address: String,
	status: String,
	custom_fields: String,
	url: Url,
	nat_outside: Vec<NestedIPAddress>,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	description: String,
	id: i64,
	assigned_object_type: Option<String>,
	created: Option<String>,
	last_updated: Option<String>,
	comments: String,
	tags: Vec<NestedTag>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPortRequest {
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
	/// * `1200` - 1200 bps
	/// * `2400` - 2400 bps
	/// * `4800` - 4800 bps
	/// * `9600` - 9600 bps
	/// * `19200` - 19.2 kbps
	/// * `38400` - 38.4 kbps
	/// * `57600` - 57.6 kbps
	/// * `115200` - 115.2 kbps
	speed: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct RackReservationRequest {
	custom_fields: String,
	units: Vec<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct CableTermination {
	created: Option<String>,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	cable: i64,
	url: Url,
	last_updated: Option<String>,
	display: String,
	termination_type: String,
	termination_id: i64,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachineRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProfileRequest {
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	ike_policy: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	comments: String,
	name: String,
	ipsec_policy: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanelRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroupRequest {
	slug: String,
	name: String,
}

pub struct PaginatedEventRuleList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<EventRule>,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicyRequest {
	comments: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	proposals: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	preshared_key: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	description: String,
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceTemplateRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ports: Vec<i64>,
	description: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassisRequest {
	custom_fields: String,
	domain: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLinkRequest {
	custom_fields: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	auth_psk: String,
	ssid: String,
	interface_a: i64,
	interface_b: i64,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	tenant: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TagRequest {
	slug: String,
	name: String,
	color: String,
	description: String,
	object_types: Vec<String>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableTenantGroupRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	parent: Option<i64>,
	description: String,
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
pub struct NestedTenantGroup {
	url: Url,
	display: String,
	id: i64,
	name: String,
	slug: String,
	_depth: i64,
}

/// Adds support for custom fields and tags.
pub struct L2VPNRequest {
	import_targets: Vec<i64>,
	slug: String,
	identifier: Option<i64>,
	export_targets: Vec<i64>,
	custom_fields: String,
	name: String,
	comments: String,
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
	tags: Vec<NestedTagRequest>,
	description: String,
}

pub struct PaginatedFHRPGroupList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<FHRPGroup>,
}

/// Adds support for custom fields and tags.
pub struct Contact {
	url: Url,
	id: i64,
	title: String,
	email: String,
	description: String,
	last_updated: Option<String>,
	address: String,
	comments: String,
	link: Url,
	display: String,
	custom_fields: String,
	created: Option<String>,
	name: String,
	tags: Vec<NestedTag>,
	phone: String,
}

/// Adds support for custom fields and tags.
pub struct CableRequest {
	custom_fields: String,
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
	b_terminations: Vec<GenericObjectRequest>,
	label: String,
	a_terminations: Vec<GenericObjectRequest>,
	color: String,
	length: Option<f64>,
	comments: String,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct Aggregate {
	comments: String,
	family: String,
	date_added: Option<String>,
	url: Url,
	id: i64,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	last_updated: Option<String>,
	prefix: String,
	display: String,
	created: Option<String>,
}

pub struct PaginatedTunnelList {
	previous: Option<Url>,
	results: Vec<Tunnel>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccount {
	description: String,
	display: String,
	last_updated: Option<String>,
	url: Url,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	account: String,
	id: i64,
	created: Option<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct RIRRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	description: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ASNRequest {
	description: String,
	custom_fields: String,
	comments: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ConsolePort {
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	link_peers: Vec<String>,
	last_updated: Option<String>,
	r#type: String,
	connected_endpoints_type: String,
	connected_endpoints: Vec<String>,
	display: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Physical label
	label: String,
	created: Option<String>,
	id: i64,
	description: String,
	url: Url,
	name: String,
	speed: Option<String>,
	cable_end: String,
	tags: Vec<NestedTag>,
	_occupied: bool,
	connected_endpoints_reachable: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTermination {
	id: i64,
	display: String,
	url: Url,
}

pub struct PaginatedFrontPortList {
	count: i64,
	results: Vec<FrontPort>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerOutletRequest {
	name: String,
	/// Physical label
	label: String,
	power_port: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	tags: Vec<NestedTagRequest>,
	device: i64,
	custom_fields: String,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassisRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNRequest {
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
	slug: String,
	tenant: Option<i64>,
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
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	identifier: Option<i64>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct TenantRequest {
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	description: String,
	name: String,
}

pub struct PaginatedPrefixList {
	previous: Option<Url>,
	results: Vec<Prefix>,
	count: i64,
	next: Option<Url>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct LocationRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	slug: String,
}

pub struct PaginatedPowerFeedList {
	results: Vec<PowerFeed>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRole {
	id: i64,
	display: String,
	url: Url,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct Prefix {
	id: i64,
	display: String,
	prefix: String,
	status: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Treat as 100% utilized
	mark_utilized: bool,
	comments: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	last_updated: Option<String>,
	children: i64,
	_depth: i64,
	family: String,
	url: Url,
	created: Option<String>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackReservationRequest {
	tenant: Option<i64>,
	units: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	user: i64,
	comments: String,
	custom_fields: String,
	rack: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUserRequest {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroup {
	_depth: i64,
	slug: String,
	id: i64,
	custom_fields: String,
	description: String,
	url: Url,
	display: String,
	name: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	wirelesslan_count: i64,
}

/// Adds support for custom fields and tags.
pub struct Circuit {
	/// Unique circuit ID
	cid: String,
	id: i64,
	tags: Vec<NestedTag>,
	description: String,
	last_updated: Option<String>,
	status: String,
	display: String,
	comments: String,
	url: Url,
	install_date: Option<String>,
	custom_fields: String,
	created: Option<String>,
	termination_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
}

pub struct PaginatedConfigContextList {
	results: Vec<ConfigContext>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedContentTypeList {
	results: Vec<ContentType>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRearPortTemplate {
	id: i64,
	url: Url,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct BookmarkRequest {
	object_id: i64,
	object_type: String,
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
pub struct JournalEntryRequest {
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	assigned_object_id: i64,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	custom_fields: String,
	created_by: Option<i64>,
	comments: String,
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

pub struct PaginatedDeviceBayList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<DeviceBay>,
	count: i64,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	rear_port_position: i64,
	color: String,
	description: String,
}

pub struct PaginatedInventoryItemRoleList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<InventoryItemRole>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPAddressRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	vrf: Option<i64>,
	tenant: Option<i64>,
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
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	address: String,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	description: String,
	assigned_object_type: Option<String>,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInventoryItemTemplateRequest {
	component_type: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	role: Option<i64>,
	/// Physical label
	label: String,
	component_id: Option<i64>,
	manufacturer: Option<i64>,
	description: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	parent: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortRequest {
	name: String,
	cable: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualMachineWithConfigContextRequest {
	site: Option<i64>,
	cluster: Option<i64>,
	primary_ip4: Option<i64>,
	platform: Option<i64>,
	primary_ip6: Option<i64>,
	comments: String,
	tenant: Option<i64>,
	role: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	device: Option<i64>,
	disk: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
	memory: Option<i64>,
	vcpus: Option<f64>,
}

pub struct PaginatedConfigTemplateList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ConfigTemplate>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplateRequest {
	description: String,
	/// Jinja2 template code.
	template_code: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

pub struct PaginatedVirtualChassisList {
	next: Option<Url>,
	count: i64,
	results: Vec<VirtualChassis>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceRequest {
	custom_fields: String,
	description: String,
	comments: String,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	name: String,
	device: Option<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	virtual_machine: Option<i64>,
	tags: Vec<NestedTagRequest>,
	ports: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSite {
	display: String,
	id: i64,
	url: Url,
	/// Full name of the site
	name: String,
	slug: String,
}

pub struct PaginatedCustomLinkList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<CustomLink>,
}

pub struct PaginatedPowerOutletTemplateList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<PowerOutletTemplate>,
}

/// Adds support for custom fields and tags.
pub struct PatchedTunnelGroupRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplateRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Service {
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	comments: String,
	id: i64,
	custom_fields: String,
	protocol: String,
	url: Url,
	ports: Vec<i64>,
	description: String,
	ipaddresses: Vec<i64>,
	name: String,
	tags: Vec<NestedTag>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct User {
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	groups: Vec<i64>,
	id: i64,
	email: String,
	url: Url,
	first_name: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	date_joined: String,
	display: String,
	last_name: String,
}

/// Adds support for custom fields and tags.
pub struct EventRuleRequest {
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	name: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	description: String,
	enabled: bool,
	custom_fields: String,
	action_object_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	tags: Vec<NestedTagRequest>,
	action_object_id: Option<i64>,
	content_types: Vec<String>,
	/// Triggers when a matching object is created.
	type_create: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableDeviceBayTemplateRequest {
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	device_type: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSourceRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPort {
	display: String,
	tags: Vec<NestedTag>,
	/// Physical label
	label: String,
	connected_endpoints_type: String,
	last_updated: Option<String>,
	cable_end: String,
	link_peers: Vec<String>,
	url: Url,
	r#type: String,
	speed: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints: Vec<String>,
	custom_fields: String,
	description: String,
	name: String,
	connected_endpoints_reachable: bool,
	created: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	id: i64,
	_occupied: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableConsolePortRequest {
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
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
	/// Physical label
	label: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct InterfaceRequest {
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	tagged_vlans: Vec<i64>,
	vdcs: Vec<i64>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	mac_address: Option<String>,
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
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
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
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	custom_fields: String,
	/// Physical label
	label: String,
	mtu: Option<i64>,
	tx_power: Option<i64>,
	name: String,
	wireless_lans: Vec<i64>,
	enabled: bool,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
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
	wwn: Option<String>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachine {
	name: String,
	url: Url,
	id: i64,
	display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableObjectPermissionRequest {
	object_types: Vec<String>,
	name: String,
	description: String,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	enabled: bool,
	users: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitType {
	slug: String,
	url: Url,
	name: String,
	display: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProvider {
	slug: String,
	display: String,
	url: Url,
	/// Full name of the provider
	name: String,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Bookmark {
	id: i64,
	display: String,
	url: Url,
	object_type: String,
	object_id: i64,
	created: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSource {
	id: i64,
	name: String,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct RackRoleRequest {
	custom_fields: String,
	color: String,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableCableRequest {
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	tenant: Option<i64>,
	custom_fields: String,
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
	length: Option<f64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	b_terminations: Vec<GenericObjectRequest>,
	label: String,
	a_terminations: Vec<GenericObjectRequest>,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContext {
	last_updated: Option<String>,
	inventory_item_count: i64,
	description: String,
	url: Url,
	comments: String,
	power_port_count: i64,
	status: String,
	console_server_port_count: i64,
	id: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	name: Option<String>,
	created: Option<String>,
	module_bay_count: i64,
	interface_count: i64,
	power_outlet_count: i64,
	console_port_count: i64,
	front_port_count: i64,
	face: String,
	rear_port_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	display: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	position: Option<f64>,
	device_bay_count: i64,
	tags: Vec<NestedTag>,
	vc_position: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	airflow: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPN {
	identifier: Option<i64>,
	display: String,
	id: i64,
	name: String,
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
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct CircuitTerminationRequest {
	tags: Vec<NestedTagRequest>,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
}

pub struct PaginatedCircuitList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Circuit>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedGroupRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPanelRequest {
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	site: i64,
	name: String,
	description: String,
	location: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct Module {
	tags: Vec<NestedTag>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	description: String,
	last_updated: Option<String>,
	comments: String,
	display: String,
	url: Url,
	status: String,
	custom_fields: String,
	serial: String,
	id: i64,
	created: Option<String>,
}

pub struct PaginatedLocationList {
	count: i64,
	next: Option<Url>,
	results: Vec<Location>,
	previous: Option<Url>,
}

pub struct PaginatedSiteList {
	results: Vec<Site>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfileRequest {
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	description: String,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct JournalEntry {
	id: i64,
	kind: String,
	display: String,
	comments: String,
	tags: Vec<NestedTag>,
	assigned_object_id: i64,
	assigned_object_type: String,
	created: Option<String>,
	created_by: Option<i64>,
	last_updated: Option<String>,
	url: Url,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedInventoryItemRoleRequest {
	name: String,
	custom_fields: String,
	description: String,
	color: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PowerFeedRequest {
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	voltage: i64,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	comments: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	custom_fields: String,
	amperage: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContext {
	cluster_types: Vec<i64>,
	tags: Vec<String>,
	platforms: Vec<i64>,
	roles: Vec<i64>,
	is_active: bool,
	regions: Vec<i64>,
	device_types: Vec<i64>,
	locations: Vec<i64>,
	sites: Vec<i64>,
	url: Url,
	tenants: Vec<i64>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	description: String,
	tenant_groups: Vec<i64>,
	cluster_groups: Vec<i64>,
	created: Option<String>,
	weight: i64,
	site_groups: Vec<i64>,
	clusters: Vec<i64>,
	last_updated: Option<String>,
	data_synced: Option<String>,
	id: i64,
	display: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTerminationRequest {
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	description: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

pub struct PaginatedFrontPortTemplateList {
	count: i64,
	results: Vec<FrontPortTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldRequest {
	/// Internal field name
	name: String,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
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
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	content_types: Vec<String>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	choice_set: Option<i64>,
	description: String,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	object_type: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroup {
	id: i64,
	display: String,
	url: Url,
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVLANRequest {
	tenant: Option<i64>,
	custom_fields: String,
	name: String,
	/// VLAN group (optional)
	group: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	comments: String,
	/// The primary function of this VLAN
	role: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatformRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceBayRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	/// Physical label
	label: String,
	description: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplateRequest {
	description: String,
	content_types: Vec<String>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Download file as attachment
	as_attachment: bool,
	name: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableFrontPortTemplateRequest {
	description: String,
	rear_port_position: i64,
	rear_port: i64,
	color: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	module_type: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDeviceContextRequest {
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	tenant: Option<i64>,
	primary_ip4: Option<i64>,
	device: Option<i64>,
	custom_fields: String,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	primary_ip6: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomField {
	ui_visible: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	ui_editable: String,
	r#type: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	created: Option<String>,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	content_types: Vec<String>,
	filter_logic: String,
	url: Url,
	object_type: String,
	/// Internal field name
	name: String,
	data_type: String,
	id: i64,
	description: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	last_updated: Option<String>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	display: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedCableTerminationRequest {
	termination_type: String,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	cable: i64,
	termination_id: i64,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplate {
	/// Jinja2 template code.
	template_code: String,
	/// Path to remote file (relative to data source root)
	data_path: String,
	description: String,
	data_synced: Option<String>,
	id: i64,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	display: String,
	last_updated: Option<String>,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCableRequest {
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplate {
	r#type: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	id: i64,
	/// Physical label
	label: String,
	feed_leg: Option<String>,
	display: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableUserRequest {
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	first_name: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	password: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	date_joined: String,
	last_name: String,
	email: String,
}

pub struct PaginatedIKEProposalList {
	count: i64,
	previous: Option<Url>,
	results: Vec<IKEProposal>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackRequest {
	custom_fields: String,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	/// Height in rack units
	u_height: i64,
	name: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	tenant: Option<i64>,
	facility_id: Option<String>,
	/// Functional role
	role: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	location: Option<i64>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	/// Starting unit for rack
	starting_unit: i64,
	tags: Vec<NestedTagRequest>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	weight: Option<f64>,
	site: i64,
	serial: String,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	description: String,
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
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTenantRequest {
	name: String,
	slug: String,
	group: Option<i64>,
	description: String,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPort {
	url: Url,
	description: String,
	name: String,
	/// Physical label
	label: String,
	display: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenant {
	id: i64,
	display: String,
	name: String,
	url: Url,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVLANRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	/// The primary function of this VLAN
	role: Option<i64>,
	/// VLAN group (optional)
	group: Option<i64>,
	custom_fields: String,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	tenant: Option<i64>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct Provider {
	url: Url,
	asns: Vec<i64>,
	accounts: Vec<i64>,
	id: i64,
	description: String,
	/// Full name of the provider
	name: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	circuit_count: i64,
	display: String,
	slug: String,
	comments: String,
	last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct UserRequest {
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	groups: Vec<i64>,
	password: String,
	email: String,
	last_name: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	first_name: String,
	date_joined: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplate {
	id: i64,
	color: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	rear_port_position: i64,
	display: String,
	r#type: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ProviderRequest {
	custom_fields: String,
	accounts: Vec<i64>,
	comments: String,
	asns: Vec<i64>,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Full name of the provider
	name: String,
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

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Tag {
	color: String,
	description: String,
	id: i64,
	object_types: Vec<String>,
	slug: String,
	url: Url,
	tagged_items: i64,
	display: String,
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModuleRequest {
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct GroupRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct AggregateRequest {
	date_added: Option<String>,
	prefix: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleRequest {
	comments: String,
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
	custom_fields: String,
	serial: String,
}

pub struct PaginatedCableTerminationList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<CableTermination>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableContactRequest {
	comments: String,
	phone: String,
	custom_fields: String,
	description: String,
	title: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	link: Url,
	address: String,
	group: Option<i64>,
	email: String,
}

/// Adds support for custom fields and tags.
pub struct Interface {
	id: i64,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	vdcs: Vec<i64>,
	mode: String,
	r#type: String,
	tagged_vlans: Vec<i64>,
	wireless_lans: Vec<i64>,
	tags: Vec<NestedTag>,
	display: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	url: Url,
	/// Physical label
	label: String,
	name: String,
	mac_address: Option<String>,
	poe_mode: String,
	connected_endpoints_type: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	enabled: bool,
	connected_endpoints: Vec<String>,
	count_ipaddresses: i64,
	wwn: Option<String>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	rf_channel: String,
	_occupied: bool,
	connected_endpoints_reachable: bool,
	poe_type: String,
	cable_end: String,
	speed: Option<i64>,
	count_fhrp_groups: i64,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	duplex: Option<String>,
	link_peers: Vec<String>,
	mtu: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	rf_role: String,
	tx_power: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceRoleRequest {
	config_template: Option<i64>,
	color: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	custom_fields: String,
	slug: String,
}

pub struct PaginatedConsolePortList {
	results: Vec<ConsolePort>,
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturerRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRole {
	slug: String,
	display: String,
	id: i64,
	url: Url,
	name: String,
}

pub struct PaginatedDeviceTypeList {
	results: Vec<DeviceType>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	device_type: Option<i64>,
	description: String,
	/// Physical label
	label: String,
	module_type: Option<i64>,
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

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableModuleBayTemplateRequest {
	/// Physical label
	label: String,
	device_type: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct IPAddressRequest {
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
	address: String,
	assigned_object_id: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	assigned_object_type: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelTerminationRequest {
	outside_ip: Option<i64>,
	termination_type: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	termination_id: Option<i64>,
	tunnel: i64,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
}

pub struct PaginatedIPSecProfileList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<IPSecProfile>,
	count: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableLocationRequest {
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	parent: Option<i64>,
	name: String,
	slug: String,
	site: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	tenant: Option<i64>,
}

pub struct PaginatedVirtualMachineWithConfigContextList {
	previous: Option<Url>,
	results: Vec<VirtualMachineWithConfigContext>,
	next: Option<Url>,
	count: i64,
}

pub struct PaginatedCircuitTerminationList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<CircuitTermination>,
}

/// Adds support for custom fields and tags.
pub struct PatchedFHRPGroupRequest {
	name: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	group_id: i64,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	auth_key: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicyRequest {
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
	description: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	proposals: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct RouteTargetRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceBayRequest {
	installed_device: Option<i64>,
	custom_fields: String,
	name: String,
	description: String,
	/// Physical label
	label: String,
	device: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRequest {
	parent: Option<i64>,
	component_id: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
	name: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	description: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	/// Physical label
	label: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	component_type: Option<String>,
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInterfaceTemplateRequest {
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
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
	mgmt_only: bool,
	module_type: Option<i64>,
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
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableTokenRequest {
	description: String,
	expires: Option<String>,
	user: i64,
	last_used: Option<String>,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct WritableInventoryItemRequest {
	parent: Option<i64>,
	component_type: Option<String>,
	serial: String,
	/// This item was automatically discovered
	discovered: bool,
	custom_fields: String,
	component_id: Option<i64>,
	device: i64,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	description: String,
	/// Physical label
	label: String,
	manufacturer: Option<i64>,
	name: String,
	role: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableDataSourceRequest {
	enabled: bool,
	description: String,
	source_url: String,
	r#type: String,
	name: String,
	comments: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
}

/// Adds support for custom fields and tags.
pub struct WritableJournalEntryRequest {
	assigned_object_type: String,
	comments: String,
	custom_fields: String,
	assigned_object_id: i64,
	created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct DataFile {
	/// SHA256 hash of the file data
	hash: String,
	url: Url,
	id: i64,
	size: i64,
	display: String,
	/// File path relative to the data source's root
	path: String,
	last_updated: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfile {
	name: String,
	id: i64,
	comments: String,
	display: String,
	tags: Vec<NestedTag>,
	description: String,
	created: Option<String>,
	custom_fields: String,
	mode: String,
	url: Url,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDiskRequest {
	virtual_machine: i64,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	size: i64,
}

pub struct PaginatedExportTemplateList {
	next: Option<Url>,
	results: Vec<ExportTemplate>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedPowerOutletList {
	results: Vec<PowerOutlet>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
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
pub struct DeviceRoleRequest {
	custom_fields: String,
	slug: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	color: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNTerminationRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	assigned_object_id: i64,
	assigned_object_type: String,
	l2vpn: i64,
}

pub struct ObjectChange {
	display: String,
	changed_object_type: String,
	action: String,
	url: Url,
	changed_object_id: i64,
	user_name: String,
	id: i64,
	time: String,
	request_id: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRouteTargetRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
	tenant: Option<i64>,
	description: String,
	/// Route target value (formatted in accordance with RFC 4360)
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
pub struct WritableASNRangeRequest {
	tags: Vec<NestedTagRequest>,
	end: i64,
	start: i64,
	slug: String,
	name: String,
	rir: i64,
	custom_fields: String,
	tenant: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterTypeRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObjectRequest {
	object_type: String,
	object_id: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerOutletRequest {
	name: String,
	custom_fields: String,
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
	/// Physical label
	label: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ClusterGroup {
	last_updated: Option<String>,
	custom_fields: String,
	slug: String,
	name: String,
	description: String,
	cluster_count: i64,
	tags: Vec<NestedTag>,
	display: String,
	url: Url,
	id: i64,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInventoryItemRequest {
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	description: String,
	component_type: Option<String>,
	/// Physical label
	label: String,
	parent: Option<i64>,
	name: String,
	device: i64,
	tags: Vec<NestedTagRequest>,
	/// This item was automatically discovered
	discovered: bool,
	component_id: Option<i64>,
	role: Option<i64>,
	custom_fields: String,
	serial: String,
	/// Manufacturer-assigned part identifier
	part_id: String,
	manufacturer: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct VRF {
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	name: String,
	import_targets: Vec<i64>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: String,
	display: String,
	id: i64,
	last_updated: Option<String>,
	url: Url,
	ipaddress_count: i64,
	comments: String,
	prefix_count: i64,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	description: String,
	export_targets: Vec<i64>,
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
pub struct PatchedWritableRearPortRequest {
	/// Number of front ports which may be mapped
	positions: i64,
	device: i64,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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
	module: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PowerPortRequest {
	/// Physical label
	label: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	description: String,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
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
	/// Physical label
	label: String,
	positions: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct PowerPanel {
	custom_fields: String,
	tags: Vec<NestedTag>,
	comments: String,
	display: String,
	description: String,
	name: String,
	url: Url,
	created: Option<String>,
	powerfeed_count: i64,
	last_updated: Option<String>,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PrefixRequest {
	custom_fields: String,
	description: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	prefix: String,
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	tags: Vec<NestedTagRequest>,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInventoryItemTemplateRequest {
	description: String,
	component_id: Option<i64>,
	role: Option<i64>,
	device_type: i64,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	component_type: Option<String>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	manufacturer: Option<i64>,
	parent: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct Site {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// If different from the physical address
	shipping_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	/// Physical location of the building
	physical_address: String,
	/// Full name of the site
	name: String,
	slug: String,
	created: Option<String>,
	last_updated: Option<String>,
	circuit_count: i64,
	status: String,
	prefix_count: i64,
	time_zone: Option<String>,
	rack_count: i64,
	virtualmachine_count: i64,
	vlan_count: i64,
	display: String,
	description: String,
	/// Local facility ID or description
	facility: String,
	comments: String,
	asns: Vec<i64>,
	device_count: i64,
	url: Url,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	custom_fields: String,
	disk: Option<i64>,
	tags: Vec<NestedTagRequest>,
	cluster: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	primary_ip4: Option<i64>,
	memory: Option<i64>,
	name: String,
	tenant: Option<i64>,
	comments: String,
	site: Option<i64>,
	platform: Option<i64>,
	primary_ip6: Option<i64>,
	vcpus: Option<f64>,
	description: String,
	role: Option<i64>,
	device: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct RackRequest {
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: Option<String>,
	comments: String,
	serial: String,
	weight: Option<f64>,
	description: String,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Starting unit for rack
	starting_unit: i64,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
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
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	facility_id: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	custom_fields: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct SiteRequest {
	/// Local facility ID or description
	facility: String,
	/// Full name of the site
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	asns: Vec<i64>,
	custom_fields: String,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	slug: String,
	/// Physical location of the building
	physical_address: String,
	/// If different from the physical address
	shipping_address: String,
	comments: String,
	time_zone: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModule {
	id: i64,
	display: String,
	url: Url,
	serial: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddress {
	id: i64,
	family: i64,
	address: String,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceTypeRequest {
	tags: Vec<NestedTagRequest>,
	rear_image: String,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	manufacturer: i64,
	description: String,
	comments: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	front_image: String,
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
	custom_fields: String,
	slug: String,
	u_height: f64,
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	default_platform: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignmentRequest {
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	content_type: String,
	custom_fields: String,
	object_id: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroupRequest {
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPortRequest {
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	device: i64,
	module: Option<i64>,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PowerOutlet {
	display: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints: Vec<String>,
	connected_endpoints_reachable: bool,
	custom_fields: String,
	feed_leg: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	url: Url,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	description: String,
	r#type: Option<String>,
	cable_end: String,
	connected_endpoints_type: String,
	created: Option<String>,
	last_updated: Option<String>,
	_occupied: bool,
	name: String,
	id: i64,
	link_peers: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNTerminationRequest {
	tags: Vec<NestedTagRequest>,
	assigned_object_id: i64,
	custom_fields: String,
	assigned_object_type: String,
	l2vpn: i64,
}

/// Used by device component serializers.
pub struct ComponentNestedModule {
	device: i64,
	url: Url,
	id: i64,
	display: String,
}

pub struct PaginatedRearPortTemplateList {
	results: Vec<RearPortTemplate>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedFHRPGroup {
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	url: Url,
	group_id: i64,
	display: String,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableRearPortTemplateRequest {
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
	/// Physical label
	label: String,
	device_type: Option<i64>,
	module_type: Option<i64>,
	description: String,
	color: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplate {
	id: i64,
	name: String,
	url: Url,
	display: String,
}

pub struct PatchedDashboardRequest {
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDiskRequest {
	size: i64,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	virtual_machine: i64,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableAggregateRequest {
	date_added: Option<String>,
	prefix: String,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPRangeRequest {
	/// Treat as 100% utilized
	mark_utilized: bool,
	vrf: Option<i64>,
	custom_fields: String,
	start_address: String,
	tenant: Option<i64>,
	comments: String,
	end_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// The primary function of this range
	role: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBay {
	name: String,
	id: i64,
	display: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupRequest {
	custom_fields: String,
	auth_key: String,
	comments: String,
	/// * `vrrp2` - VRRPv2
	/// * `vrrp3` - VRRPv3
	/// * `carp` - CARP
	/// * `clusterxl` - ClusterXL
	/// * `hsrp` - HSRP
	/// * `glbp` - GLBP
	/// * `other` - Other
	protocol: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `plaintext` - Plaintext
	/// * `md5` - MD5
	auth_type: String,
	group_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterface {
	url: Url,
	name: String,
	display: String,
	_occupied: bool,
	id: i64,
	cable: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableFHRPGroupAssignmentRequest {
	group: i64,
	interface_id: i64,
	priority: i64,
	interface_type: String,
}

pub struct PaginatedDeviceRoleList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<DeviceRole>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTermination {
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	url: Url,
	description: String,
	id: i64,
	display: String,
}

pub struct PaginatedGroupList {
	count: i64,
	results: Vec<Group>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ASNRangeRequest {
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	end: i64,
	custom_fields: String,
	start: i64,
}

/// Adds support for custom fields and tags.
pub struct VLAN {
	id: i64,
	status: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	display: String,
	created: Option<String>,
	name: String,
	last_updated: Option<String>,
	prefix_count: i64,
	url: Url,
	tags: Vec<NestedTag>,
	comments: String,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplate {
	last_updated: Option<String>,
	comments: String,
	created: Option<String>,
	protocol: String,
	ports: Vec<i64>,
	id: i64,
	description: String,
	tags: Vec<NestedTag>,
	display: String,
	name: String,
	custom_fields: String,
	url: Url,
}

pub struct PaginatedManufacturerList {
	next: Option<Url>,
	results: Vec<Manufacturer>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableRackRequest {
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	custom_fields: String,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	name: String,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// Starting unit for rack
	starting_unit: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	description: String,
	weight: Option<f64>,
	tenant: Option<i64>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	serial: String,
	/// Height in rack units
	u_height: i64,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	/// Functional role
	role: Option<i64>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	location: Option<i64>,
	site: i64,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	facility_id: Option<String>,
}

pub struct PaginatedCableList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Cable>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecPolicyRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelRequest {
	comments: String,
	tunnel_id: Option<i64>,
	description: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	tenant: Option<i64>,
	custom_fields: String,
	name: String,
	ipsec_profile: Option<i64>,
	tags: Vec<NestedTagRequest>,
	group: Option<i64>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
}

/// Adds support for custom fields and tags.
pub struct WritableFrontPortRequest {
	device: i64,
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
	tags: Vec<NestedTagRequest>,
	name: String,
	color: String,
	module: Option<i64>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	/// Physical label
	label: String,
	custom_fields: String,
	rear_port: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

/// Adds support for custom fields and tags.
pub struct CircuitType {
	slug: String,
	last_updated: Option<String>,
	color: String,
	circuit_count: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	url: Url,
	display: String,
	custom_fields: String,
	description: String,
	id: i64,
	name: String,
}

pub struct PaginatedModuleList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Module>,
}

/// Adds support for custom fields and tags.
pub struct Tenant {
	prefix_count: i64,
	vlan_count: i64,
	tags: Vec<NestedTag>,
	id: i64,
	custom_fields: String,
	device_count: i64,
	comments: String,
	display: String,
	site_count: i64,
	ipaddress_count: i64,
	slug: String,
	cluster_count: i64,
	virtualmachine_count: i64,
	description: String,
	url: Url,
	circuit_count: i64,
	vrf_count: i64,
	name: String,
	rack_count: i64,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Representation of an ASN which does not exist in the database.
pub struct AvailableASN {
	asn: i64,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroup {
	name: String,
	display: String,
	id: i64,
	url: Url,
	slug: String,
}

pub struct PaginatedCustomFieldList {
	next: Option<Url>,
	results: Vec<CustomField>,
	count: i64,
	previous: Option<Url>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableRegionRequest {
	parent: Option<i64>,
	slug: String,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLinkRequest {
	interface_a: i64,
	interface_b: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	ssid: String,
	custom_fields: String,
	auth_psk: String,
	tenant: Option<i64>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
}

/// Adds support for custom fields and tags.
pub struct WritableInterfaceRequest {
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
	enabled: bool,
	wwn: Option<String>,
	untagged_vlan: Option<i64>,
	speed: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
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
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	vrf: Option<i64>,
	tx_power: Option<i64>,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	vdcs: Vec<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	description: String,
	tagged_vlans: Vec<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	custom_fields: String,
	lag: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	module: Option<i64>,
	device: i64,
	bridge: Option<i64>,
	mac_address: Option<String>,
	parent: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRequest {
	name: Option<String>,
}

pub struct PaginatedRIRList {
	next: Option<Url>,
	results: Vec<RIR>,
	count: i64,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterType {
	slug: String,
	id: i64,
	name: String,
	url: Url,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRoleRequest {
	slug: String,
	name: String,
}

pub struct PaginatedTunnelTerminationList {
	count: i64,
	results: Vec<TunnelTermination>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceWithConfigContextRequest {
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	platform: Option<i64>,
	primary_ip6: Option<i64>,
	oob_ip: Option<i64>,
	comments: String,
	config_template: Option<i64>,
	tenant: Option<i64>,
	name: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	cluster: Option<i64>,
	site: i64,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	virtual_chassis: Option<i64>,
	description: String,
	vc_position: Option<i64>,
	custom_fields: String,
	/// The function this device serves
	role: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	position: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	tags: Vec<NestedTagRequest>,
	primary_ip4: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	location: Option<i64>,
	device_type: i64,
	rack: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ClusterGroupRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRequest {
	/// 16- or 32-bit autonomous system number
	asn: i64,
	description: String,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	comments: String,
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
pub struct RearPortRequest {
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Number of front ports which may be mapped
	positions: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Physical label
	label: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableTenantRequest {
	comments: String,
	group: Option<i64>,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEPolicyRequest {
	tags: Vec<NestedTagRequest>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	description: String,
	custom_fields: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	preshared_key: String,
	name: String,
	proposals: Vec<i64>,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegion {
	slug: String,
	url: Url,
	_depth: i64,
	display: String,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFrontPortRequest {
	rear_port: i64,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	name: String,
	/// Physical label
	label: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	color: String,
	module: Option<i64>,
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
	device: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplate {
	enabled: bool,
	display: String,
	created: Option<String>,
	poe_mode: Option<String>,
	last_updated: Option<String>,
	poe_type: Option<String>,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	id: i64,
	r#type: String,
	mgmt_only: bool,
	description: String,
	rf_role: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct FrontPort {
	id: i64,
	r#type: String,
	/// Physical label
	label: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	color: String,
	description: String,
	custom_fields: String,
	_occupied: bool,
	url: Url,
	link_peers: Vec<String>,
	created: Option<String>,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	display: String,
	tags: Vec<NestedTag>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	cable_end: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Location {
	slug: String,
	id: i64,
	description: String,
	device_count: i64,
	custom_fields: String,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	status: String,
	created: Option<String>,
	rack_count: i64,
	_depth: i64,
	last_updated: Option<String>,
	display: String,
}

pub struct PaginatedContactGroupList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<ContactGroup>,
}

pub struct PaginatedJournalEntryList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<JournalEntry>,
}

pub struct PaginatedJobList {
	count: i64,
	results: Vec<Job>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroupRequest {
	custom_fields: String,
	slug: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct Tunnel {
	name: String,
	tags: Vec<NestedTag>,
	url: Url,
	display: String,
	tunnel_id: Option<i64>,
	encapsulation: String,
	created: Option<String>,
	id: i64,
	description: String,
	custom_fields: String,
	last_updated: Option<String>,
	comments: String,
	status: String,
}

/// Adds support for custom fields and tags.
pub struct PowerPort {
	/// Physical label
	label: String,
	custom_fields: String,
	last_updated: Option<String>,
	connected_endpoints_type: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	url: Url,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	cable_end: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	connected_endpoints_reachable: bool,
	display: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints: Vec<String>,
	_occupied: bool,
	link_peers: Vec<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	id: i64,
	name: String,
	description: String,
	r#type: Option<String>,
}

pub struct PaginatedTenantList {
	count: i64,
	results: Vec<Tenant>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDevice {
	name: Option<String>,
	id: i64,
	url: Url,
	display: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct WritableConfigTemplateRequest {
	/// Jinja2 template code.
	template_code: String,
	data_file: Option<i64>,
	/// Remote data source
	data_source: Option<i64>,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VMInterfaceRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tagged_vlans: Vec<i64>,
	mac_address: Option<String>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	enabled: bool,
	name: String,
	description: String,
	mtu: Option<i64>,
}

/// Used by device component serializers.
pub struct ComponentNestedModuleRequest {
	device: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanel {
	url: Url,
	display: String,
	id: i64,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldChoiceSetRequest {
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	description: String,
	name: String,
	extra_choices: Option<Vec<Vec<String>>>,
}

/// Adds support for custom fields and tags.
pub struct PatchedRackRoleRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	custom_fields: String,
	color: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Webhook {
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	created: Option<String>,
	display: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	tags: Vec<NestedTag>,
	id: i64,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	last_updated: Option<String>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	name: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	custom_fields: String,
	description: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct WritableRearPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
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
	device: i64,
	module: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	color: String,
	/// Number of front ports which may be mapped
	positions: i64,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicyRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachmentRequest {
	name: String,
	image_width: i64,
	image: String,
	image_height: i64,
	content_type: String,
	object_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConfigContextRequest {
	regions: Vec<i64>,
	weight: i64,
	device_types: Vec<i64>,
	clusters: Vec<i64>,
	platforms: Vec<i64>,
	tags: Vec<String>,
	name: String,
	tenant_groups: Vec<i64>,
	is_active: bool,
	/// Remote data source
	data_source: Option<i64>,
	tenants: Vec<i64>,
	site_groups: Vec<i64>,
	roles: Vec<i64>,
	description: String,
	locations: Vec<i64>,
	cluster_groups: Vec<i64>,
	sites: Vec<i64>,
	cluster_types: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplateRequest {
	/// Physical label
	label: String,
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUser {
	url: Url,
	display: String,
	id: i64,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

/// Adds support for custom fields and tags.
pub struct ManufacturerRequest {
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	description: String,
	custom_fields: String,
}

pub struct PaginatedClusterGroupList {
	count: i64,
	results: Vec<ClusterGroup>,
	previous: Option<Url>,
	next: Option<Url>,
}

pub struct PaginatedContactList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<Contact>,
}

pub struct PaginatedDeviceWithConfigContextList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceWithConfigContext>,
}

pub struct PaginatedRackRoleList {
	count: i64,
	results: Vec<RackRole>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLANRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	ssid: String,
	group: Option<i64>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
	vlan: Option<i64>,
	auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	tenant: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRIRRequest {
	name: String,
	slug: String,
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
pub struct PatchedCustomLinkRequest {
	/// Force link to open in a new window
	new_window: bool,
	weight: i64,
	content_types: Vec<String>,
	/// Jinja2 template code for link URL
	link_url: String,
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
	/// Jinja2 template code for link text
	link_text: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelTerminationRequest {
	termination_id: Option<i64>,
	tunnel: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	termination_type: String,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	outside_ip: Option<i64>,
}

pub struct PaginatedASNRangeList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ASNRange>,
	count: i64,
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
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	custom_fields: String,
	description: String,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableDeviceBayTemplateRequest {
	device_type: i64,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderAccountRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	provider: i64,
	name: String,
	account: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplateRequest {
	name: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroup {
	id: i64,
	created: Option<String>,
	slug: String,
	custom_fields: String,
	tunnel_count: i64,
	url: Url,
	last_updated: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLANRequest {
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
	ssid: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	auth_psk: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerOutletTemplateRequest {
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
	description: String,
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

/// Adds support for custom fields and tags.
pub struct VLANGroupRequest {
	name: String,
	scope_id: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	slug: String,
	scope_type: Option<String>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PlatformRequest {
	custom_fields: String,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	description: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplate {
	name: String,
	/// Download file as attachment
	as_attachment: bool,
	id: i64,
	/// Path to remote file (relative to data source root)
	data_path: String,
	content_types: Vec<String>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	last_updated: Option<String>,
	url: Url,
	data_synced: Option<String>,
	display: String,
	created: Option<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEProposalRequest {
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	name: String,
	description: String,
	comments: String,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Token {
	display: String,
	last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	id: i64,
	url: Url,
	created: String,
	expires: Option<String>,
	key: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceTypeRequest {
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	rear_image: String,
	weight: Option<f64>,
	slug: String,
	/// Discrete part number (optional)
	part_number: String,
	description: String,
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: Option<String>,
	comments: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	custom_fields: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	front_image: String,
	model: String,
	tags: Vec<NestedTagRequest>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: Option<String>,
	u_height: f64,
}

pub struct PaginatedPlatformList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Platform>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWebhookRequest {
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitTermination {
	/// Physical circuit speed
	port_speed: Option<i64>,
	_occupied: bool,
	id: i64,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	url: Url,
	/// ID of the local cross-connect
	xconnect_id: String,
	tags: Vec<NestedTag>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	cable_end: String,
	created: Option<String>,
	display: String,
	link_peers: Vec<String>,
	custom_fields: String,
	last_updated: Option<String>,
	description: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCableRequest {
	a_terminations: Vec<GenericObjectRequest>,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
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
	description: String,
	label: String,
	b_terminations: Vec<GenericObjectRequest>,
	tenant: Option<i64>,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	color: String,
	custom_fields: String,
}

/// Representation of a prefix which does not exist in the database.
pub struct AvailablePrefix {
	family: i64,
	prefix: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableUserRequest {
	password: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	date_joined: String,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	last_name: String,
	first_name: String,
	email: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
}

/// Adds support for custom fields and tags.
pub struct ContactRole {
	url: Url,
	last_updated: Option<String>,
	created: Option<String>,
	slug: String,
	description: String,
	display: String,
	id: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	name: String,
}

pub struct PaginatedDataFileList {
	results: Vec<DataFile>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceWithConfigContextRequest {
	/// The function this device serves
	role: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	site: i64,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	tenant: Option<i64>,
	platform: Option<i64>,
	custom_fields: String,
	primary_ip4: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	vc_position: Option<i64>,
	name: Option<String>,
	virtual_chassis: Option<i64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	position: Option<f64>,
	device_type: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	oob_ip: Option<i64>,
	description: String,
	rack: Option<i64>,
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	cluster: Option<i64>,
	comments: String,
	primary_ip6: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	location: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProfileRequest {
	tags: Vec<NestedTagRequest>,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	description: String,
	ipsec_policy: i64,
	custom_fields: String,
	name: String,
	ike_policy: i64,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableWirelessLANGroupRequest {
	name: String,
	description: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermission {
	users: Vec<i64>,
	id: i64,
	display: String,
	object_types: Vec<String>,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	name: String,
	description: String,
	url: Url,
	enabled: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccount {
	name: String,
	account: String,
	display: String,
	url: Url,
	id: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableTenantGroupRequest {
	parent: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetwork {
	service_id: String,
	id: i64,
	created: Option<String>,
	display: String,
	description: String,
	last_updated: Option<String>,
	url: Url,
	name: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	comments: String,
}

pub struct PaginatedTenantGroupList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<TenantGroup>,
}

pub struct PaginatedWirelessLANGroupList {
	results: Vec<WirelessLANGroup>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableTokenRequest {
	last_used: Option<String>,
	description: String,
	expires: Option<String>,
	user: i64,
	key: String,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
}

pub struct PaginatedRackReservationList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<RackReservation>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct RegionRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderNetworkRequest {
	service_id: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	provider: i64,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterfaceRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPAddressRequest {
	description: String,
	assigned_object_id: Option<i64>,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	vrf: Option<i64>,
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
	comments: String,
	assigned_object_type: Option<String>,
	address: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePlatformRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	slug: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	config_template: Option<i64>,
	description: String,
}

pub struct PaginatedInventoryItemList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<InventoryItem>,
	count: i64,
}

pub struct PaginatedWirelessLANList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<WirelessLAN>,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContextRequest {
	tags: Vec<NestedTagRequest>,
	disk: Option<i64>,
	name: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	memory: Option<i64>,
	custom_fields: String,
	vcpus: Option<f64>,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleTypeRequest {
	custom_fields: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	manufacturer: i64,
	tags: Vec<NestedTagRequest>,
	model: String,
	comments: String,
	description: String,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLinkRequest {
	ssid: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceBay {
	created: Option<String>,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	description: String,
	last_updated: Option<String>,
	name: String,
	display: String,
	id: i64,
	custom_fields: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRoleRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleTypeRequest {
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	model: String,
	manufacturer: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableObjectPermissionRequest {
	enabled: bool,
	object_types: Vec<String>,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	name: String,
	description: String,
	users: Vec<i64>,
}

pub struct PaginatedInterfaceTemplateList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<InterfaceTemplate>,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRole {
	color: String,
	url: Url,
	description: String,
	last_updated: Option<String>,
	inventoryitem_count: i64,
	tags: Vec<NestedTag>,
	id: i64,
	slug: String,
	display: String,
	name: String,
	custom_fields: String,
	created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	url: Url,
	display: String,
	r#type: String,
	color: String,
	created: Option<String>,
	last_updated: Option<String>,
	positions: i64,
	description: String,
	id: i64,
}

pub struct PaginatedImageAttachmentList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ImageAttachment>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassis {
	display: String,
	custom_fields: String,
	description: String,
	comments: String,
	tags: Vec<NestedTag>,
	name: String,
	url: Url,
	id: i64,
	created: Option<String>,
	member_count: i64,
	domain: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCluster {
	display: String,
	id: i64,
	name: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteRequest {
	slug: String,
	/// Full name of the site
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLinkRequest {
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	content_types: Vec<String>,
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
	/// Force link to open in a new window
	new_window: bool,
	enabled: bool,
	name: String,
	weight: i64,
	/// Jinja2 template code for link text
	link_text: String,
}

pub struct PaginatedAggregateList {
	count: i64,
	previous: Option<Url>,
	results: Vec<Aggregate>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEPolicyRequest {
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	preshared_key: String,
	proposals: Vec<i64>,
	custom_fields: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualChassisRequest {
	domain: String,
	master: Option<i64>,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	comments: String,
}

pub struct DashboardRequest {
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNRequest {
	comments: String,
	export_targets: Vec<i64>,
	slug: String,
	description: String,
	tenant: Option<i64>,
	name: String,
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
	custom_fields: String,
	import_targets: Vec<i64>,
	identifier: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct CableTerminationRequest {
	cable: i64,
	termination_id: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	termination_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerPortTemplateRequest {
	module_type: Option<i64>,
	description: String,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Physical label
	label: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermissionRequest {
	name: String,
	description: String,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	object_types: Vec<String>,
	users: Vec<i64>,
	enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactRequest {
	group: Option<i64>,
	description: String,
	title: String,
	email: String,
	phone: String,
	link: Url,
	tags: Vec<NestedTagRequest>,
	name: String,
	address: String,
	comments: String,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Region {
	id: i64,
	url: Url,
	slug: String,
	name: String,
	custom_fields: String,
	last_updated: Option<String>,
	description: String,
	site_count: i64,
	_depth: i64,
	display: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

pub struct PaginatedVirtualDiskList {
	count: i64,
	next: Option<Url>,
	results: Vec<VirtualDisk>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedContactRoleRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	slug: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableContactGroupRequest {
	slug: String,
	parent: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVMInterfaceRequest {
	tagged_vlans: Vec<i64>,
	virtual_machine: i64,
	mac_address: Option<String>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	mtu: Option<i64>,
	name: String,
	bridge: Option<i64>,
	description: String,
	enabled: bool,
	untagged_vlan: Option<i64>,
	vrf: Option<i64>,
	custom_fields: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct DataSourceRequest {
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	source_url: String,
	name: String,
	enabled: bool,
	description: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLAN {
	display: String,
	name: String,
	url: Url,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	id: i64,
}

pub struct PaginatedVRFList {
	results: Vec<VRF>,
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRangeRequest {
	tags: Vec<NestedTagRequest>,
	slug: String,
	tenant: Option<i64>,
	end: i64,
	description: String,
	name: String,
	start: i64,
	custom_fields: String,
	rir: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitTerminationRequest {
	/// ID of the local cross-connect
	xconnect_id: String,
	site: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	custom_fields: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	circuit: i64,
	provider_network: Option<i64>,
	description: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPRangeRequest {
	tenant: Option<i64>,
	start_address: String,
	end_address: String,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	vrf: Option<i64>,
	/// The primary function of this range
	role: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	custom_fields: String,
	description: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableJournalEntryRequest {
	comments: String,
	created_by: Option<i64>,
	assigned_object_type: String,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_id: i64,
	custom_fields: String,
}

pub struct PaginatedVMInterfaceList {
	results: Vec<VMInterface>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactAssignmentRequest {
	object_id: i64,
	content_type: String,
	contact: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	role: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct Cable {
	tags: Vec<NestedTag>,
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
	color: String,
	comments: String,
	label: String,
	a_terminations: Vec<GenericObject>,
	custom_fields: String,
	b_terminations: Vec<GenericObject>,
	description: String,
	id: i64,
	status: String,
	url: Url,
	display: String,
	length: Option<f64>,
	length_unit: Option<String>,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct VMInterface {
	id: i64,
	url: Url,
	description: String,
	display: String,
	name: String,
	tags: Vec<NestedTag>,
	mac_address: Option<String>,
	mode: String,
	mtu: Option<i64>,
	last_updated: Option<String>,
	count_ipaddresses: i64,
	custom_fields: String,
	count_fhrp_groups: i64,
	enabled: bool,
	tagged_vlans: Vec<i64>,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleBayRequest {
	/// Identifier to reference when renaming installed components
	position: String,
	name: String,
	custom_fields: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	installed_module: i64,
	description: String,
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContextRequest {
	position: Option<f64>,
	vc_position: Option<i64>,
	description: String,
	custom_fields: String,
	name: Option<String>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	tags: Vec<NestedTagRequest>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	comments: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSetRequest {
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	description: String,
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	extra_choices: Option<Vec<Vec<String>>>,
	name: String,
}

pub struct PaginatedVLANList {
	next: Option<Url>,
	results: Vec<VLAN>,
	previous: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableBookmarkRequest {
	object_id: i64,
	object_type: String,
	user: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddressRequest {
	address: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPortRequest {
	description: String,
	name: String,
	/// Physical label
	label: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableSiteGroupRequest {
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	custom_fields: String,
	description: String,
}

pub struct PaginatedRoleList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<Role>,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContext {
	status: String,
	virtual_disk_count: i64,
	disk: Option<i64>,
	description: String,
	display: String,
	url: Url,
	vcpus: Option<f64>,
	memory: Option<i64>,
	comments: String,
	name: String,
	created: Option<String>,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	interface_count: i64,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocation {
	slug: String,
	_depth: i64,
	url: Url,
	display: String,
	id: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroup {
	url: Url,
	name: String,
	display: String,
	id: i64,
	slug: String,
	_depth: i64,
}

pub struct PaginatedDataSourceList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<DataSource>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTagRequest {
	name: String,
	slug: String,
	color: String,
}

pub struct PaginatedIPAddressList {
	count: i64,
	previous: Option<Url>,
	results: Vec<IPAddress>,
	next: Option<Url>,
}

pub struct PaginatedModuleBayTemplateList {
	results: Vec<ModuleBayTemplate>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

pub struct PaginatedContactAssignmentList {
	results: Vec<ContactAssignment>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedModuleTypeList {
	count: i64,
	previous: Option<Url>,
	results: Vec<ModuleType>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Group {
	name: String,
	url: Url,
	id: i64,
	display: String,
	user_count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRequest {
	comments: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	custom_fields: String,
	description: String,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvisionRequest {
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	username: String,
	expires: Option<String>,
	password: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContext {
	comments: String,
	created: Option<String>,
	name: String,
	custom_fields: String,
	interface_count: i64,
	status: String,
	url: Url,
	description: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	tags: Vec<NestedTag>,
	id: i64,
	display: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceRoleRequest {
	custom_fields: String,
	config_template: Option<i64>,
	name: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	description: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTerminationRequest {
}

/// Adds support for custom fields and tags.
pub struct TunnelRequest {
	tunnel_id: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	comments: String,
	custom_fields: String,
	description: String,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceTypeRequest {
	slug: String,
	model: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplateRequest {
	description: String,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterType {
	slug: String,
	name: String,
	created: Option<String>,
	url: Url,
	id: i64,
	custom_fields: String,
	tags: Vec<NestedTag>,
	description: String,
	display: String,
	last_updated: Option<String>,
	cluster_count: i64,
}

pub struct PaginatedSavedFilterList {
	results: Vec<SavedFilter>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroupRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProposal {
	encryption_algorithm: String,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	display: String,
	authentication_algorithm: String,
	name: String,
	url: Url,
	created: Option<String>,
	id: i64,
	last_updated: Option<String>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderRequest {
	slug: String,
	/// Full name of the provider
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplateRequest {
	description: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
}

pub struct PaginatedRearPortList {
	results: Vec<RearPort>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedCircuitTypeRequest {
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	color: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLinkRequest {
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	auth_psk: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	comments: String,
	custom_fields: String,
	description: String,
	ssid: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritablePlatformRequest {
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	config_template: Option<i64>,
	slug: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableContactAssignmentRequest {
	object_id: i64,
	role: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	contact: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	content_type: String,
}

/// Adds support for custom fields and tags.
pub struct Cluster {
	id: i64,
	status: String,
	comments: String,
	name: String,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	display: String,
	created: Option<String>,
	virtualmachine_count: i64,
	device_count: i64,
	last_updated: Option<String>,
	url: Url,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroup {
	site_count: i64,
	_depth: i64,
	description: String,
	name: String,
	slug: String,
	custom_fields: String,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	url: Url,
	display: String,
	id: i64,
	created: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WirelessLAN {
	description: String,
	auth_psk: String,
	id: i64,
	comments: String,
	last_updated: Option<String>,
	display: String,
	ssid: String,
	url: Url,
	auth_cipher: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	auth_type: String,
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRequest {
	name: String,
}

pub struct PaginatedVirtualDeviceContextList {
	results: Vec<VirtualDeviceContext>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatform {
	display: String,
	name: String,
	slug: String,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerOutletRequest {
	device: i64,
	custom_fields: String,
	description: String,
	name: String,
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
	module: Option<i64>,
	power_port: Option<i64>,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitRequest {
	/// Committed rate
	commit_rate: Option<i64>,
	provider_account: Option<i64>,
	install_date: Option<String>,
	provider: i64,
	r#type: i64,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	termination_date: Option<String>,
	description: String,
	/// Unique circuit ID
	cid: String,
	comments: String,
	custom_fields: String,
}

pub struct PaginatedTokenList {
	results: Vec<Token>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicy {
	last_updated: Option<String>,
	display: String,
	created: Option<String>,
	id: i64,
	name: String,
	pfs_group: String,
	proposals: Vec<i64>,
	url: Url,
	description: String,
	comments: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
}

pub struct Job {
	object_type: String,
	name: String,
	display: String,
	id: i64,
	/// Recurrence interval (in minutes)
	interval: Option<i64>,
	error: String,
	status: String,
	url: Url,
	scheduled: Option<String>,
	started: Option<String>,
	job_id: String,
	object_id: Option<i64>,
	completed: Option<String>,
	created: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldRequest {
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// Specifies whether the custom field is displayed in the UI
	/// 
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	description: String,
	choice_set: Option<i64>,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
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
	object_type: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	content_types: Vec<String>,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ContactRequest {
	phone: String,
	address: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	link: Url,
	title: String,
	description: String,
	name: String,
	email: String,
	comments: String,
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

/// Adds support for custom fields and tags.
pub struct PatchedWritableClusterRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	r#type: i64,
	tenant: Option<i64>,
	site: Option<i64>,
	description: String,
	comments: String,
	group: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableClusterRequest {
	r#type: i64,
	tenant: Option<i64>,
	custom_fields: String,
	group: Option<i64>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	name: String,
	comments: String,
	site: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsoleServerPortTemplateRequest {
	module_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
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
}

/// Adds support for custom fields and tags.
pub struct RackRole {
	slug: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	rack_count: i64,
	name: String,
	custom_fields: String,
	url: Url,
	last_updated: Option<String>,
	description: String,
	display: String,
	color: String,
	id: i64,
}

pub struct PaginatedObjectChangeList {
	count: i64,
	results: Vec<ObjectChange>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPort {
	cable: Option<i64>,
	_occupied: bool,
	name: String,
	url: Url,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedRIRRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	name: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableWirelessLANGroupRequest {
	description: String,
	slug: String,
	parent: Option<i64>,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEProposalRequest {
	/// * `aes-128-cbc` - 128-bit AES (CBC)
	/// * `aes-128-gcm` - 128-bit AES (GCM)
	/// * `aes-192-cbc` - 192-bit AES (CBC)
	/// * `aes-192-gcm` - 192-bit AES (GCM)
	/// * `aes-256-cbc` - 256-bit AES (CBC)
	/// * `aes-256-gcm` - 256-bit AES (GCM)
	/// * `3des-cbc` - DES
	encryption_algorithm: String,
	description: String,
	name: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
	comments: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	custom_fields: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplate {
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	last_updated: Option<String>,
	component_id: Option<i64>,
	/// Physical label
	label: String,
	parent: Option<i64>,
	id: i64,
	display: String,
	created: Option<String>,
	description: String,
	_depth: i64,
	component_type: Option<String>,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObject {
	object_id: i64,
	object_type: String,
}


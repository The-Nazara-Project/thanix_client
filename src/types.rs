use serde_qs;
use reqwest::Url;
use crate::util::{REQWEST_BASE_URL, REQWEST_CLIENT};

pub struct PaginatedTokenList {
	count: i64,
	previous: Option<Url>,
	results: Vec<Token>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerFeedRequest {
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	voltage: i64,
	description: String,
	comments: String,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	rack: Option<i64>,
	tenant: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	name: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	power_panel: i64,
	tags: Vec<NestedTagRequest>,
	amperage: i64,
}

/// Used by device component serializers.
pub struct ComponentNestedModuleRequest {
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct InterfaceRequest {
	vdcs: Vec<i64>,
	enabled: bool,
	speed: Option<i64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
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
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// Physical label
	label: String,
	mac_address: Option<String>,
	custom_fields: String,
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
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	mtu: Option<i64>,
	wwn: Option<String>,
	tx_power: Option<i64>,
	tagged_vlans: Vec<i64>,
	wireless_lans: Vec<i64>,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	tags: Vec<NestedTagRequest>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
}

/// Adds support for custom fields and tags.
pub struct CircuitTerminationRequest {
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableUserRequest {
	password: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	date_joined: String,
	email: String,
	last_name: String,
	first_name: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableRearPortTemplateRequest {
	/// Physical label
	label: String,
	device_type: Option<i64>,
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
	color: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	module_type: Option<i64>,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroup {
	custom_fields: String,
	url: Url,
	last_updated: Option<String>,
	_depth: i64,
	wirelesslan_count: i64,
	slug: String,
	created: Option<String>,
	name: String,
	tags: Vec<NestedTag>,
	display: String,
	id: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRequest {
	custom_fields: String,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	comments: String,
	tenant: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldChoiceSetRequest {
	extra_choices: Option<Vec<Vec<String>>>,
	name: String,
	description: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	/// Base set of predefined choices (optional)
	/// 
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
}

pub struct PaginatedCustomFieldList {
	count: i64,
	results: Vec<CustomField>,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedGroupList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<Group>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceTemplateRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	ports: Vec<i64>,
	custom_fields: String,
	description: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VLANGroup {
	tags: Vec<NestedTag>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	scope_type: Option<String>,
	id: i64,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	description: String,
	scope_id: Option<i64>,
	utilization: String,
	slug: String,
	custom_fields: String,
	name: String,
	created: Option<String>,
	display: String,
	vlan_count: i64,
	last_updated: Option<String>,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct VMInterfaceRequest {
	tags: Vec<NestedTagRequest>,
	mtu: Option<i64>,
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	name: String,
	tagged_vlans: Vec<i64>,
	custom_fields: String,
	description: String,
	mac_address: Option<String>,
	enabled: bool,
}

/// Adds support for custom fields and tags.
pub struct ModuleBayRequest {
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfileRequest {
	description: String,
	custom_fields: String,
	comments: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVMInterfaceRequest {
	vrf: Option<i64>,
	enabled: bool,
	parent: Option<i64>,
	virtual_machine: i64,
	mtu: Option<i64>,
	mac_address: Option<String>,
	name: String,
	tagged_vlans: Vec<i64>,
	bridge: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	custom_fields: String,
	untagged_vlan: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

pub struct PaginatedProviderAccountList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<ProviderAccount>,
}

pub struct PaginatedConsolePortTemplateList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<ConsolePortTemplate>,
}

pub struct PaginatedTagList {
	next: Option<Url>,
	count: i64,
	results: Vec<Tag>,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplate {
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	url: Url,
	description: String,
	id: i64,
	last_updated: Option<String>,
	/// Physical label
	label: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Region {
	_depth: i64,
	display: String,
	tags: Vec<NestedTag>,
	id: i64,
	site_count: i64,
	custom_fields: String,
	description: String,
	name: String,
	slug: String,
	created: Option<String>,
	url: Url,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRole {
	url: Url,
	display: String,
	id: i64,
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct User {
	groups: Vec<i64>,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	url: Url,
	date_joined: String,
	display: String,
	id: i64,
	first_name: String,
	email: String,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	last_name: String,
}

pub struct PaginatedPlatformList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Platform>,
}

/// Adds support for custom fields and tags.
pub struct VMInterface {
	id: i64,
	enabled: bool,
	last_updated: Option<String>,
	tagged_vlans: Vec<i64>,
	description: String,
	created: Option<String>,
	name: String,
	display: String,
	mac_address: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	count_ipaddresses: i64,
	mtu: Option<i64>,
	mode: String,
	url: Url,
	count_fhrp_groups: i64,
}

/// Adds support for custom fields and tags.
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
	tags: Vec<NestedTagRequest>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	comments: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableBookmarkRequest {
	object_type: String,
	object_id: i64,
	user: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableLocationRequest {
	site: i64,
	custom_fields: String,
	parent: Option<i64>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	description: String,
}

pub struct PaginatedDeviceBayTemplateList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceBayTemplate>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicyRequest {
	description: String,
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
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPort {
	name: String,
	/// Physical label
	label: String,
	description: String,
	id: i64,
	display: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplate {
	id: i64,
	description: String,
	url: Url,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	display: String,
	r#type: String,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddress {
	display: String,
	id: i64,
	family: i64,
	url: Url,
	address: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTag {
	id: i64,
	color: String,
	url: Url,
	display: String,
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceRequest {
	cable: Option<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableASNRangeRequest {
	name: String,
	slug: String,
	description: String,
	custom_fields: String,
	tenant: Option<i64>,
	start: i64,
	tags: Vec<NestedTagRequest>,
	rir: i64,
	end: i64,
}

pub struct PaginatedConfigTemplateList {
	next: Option<Url>,
	results: Vec<ConfigTemplate>,
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
pub struct ClusterRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	custom_fields: String,
	comments: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleTypeRequest {
	model: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedImageAttachmentRequest {
	image: String,
	object_id: i64,
	name: String,
	image_height: i64,
	content_type: String,
	image_width: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplateRequest {
	/// Physical label
	label: String,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
}

/// Adds support for custom fields and tags.
pub struct WritableInterfaceRequest {
	speed: Option<i64>,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
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
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	bridge: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	tx_power: Option<i64>,
	tagged_vlans: Vec<i64>,
	vrf: Option<i64>,
	parent: Option<i64>,
	vdcs: Vec<i64>,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	lag: Option<i64>,
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
	wwn: Option<String>,
	description: String,
	/// Physical label
	label: String,
	mac_address: Option<String>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	wireless_lans: Vec<i64>,
	module: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	untagged_vlan: Option<i64>,
	name: String,
	device: i64,
	mtu: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRole {
	url: Url,
	id: i64,
	slug: String,
	display: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilter {
	id: i64,
	shared: bool,
	name: String,
	slug: String,
	user: Option<i64>,
	enabled: bool,
	content_types: Vec<String>,
	url: Url,
	description: String,
	display: String,
	weight: i64,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Representation of a VLAN which does not exist in the database.
pub struct AvailableVLAN {
	vid: i64,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRoleRequest {
	color: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	description: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContextRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	custom_fields: String,
	comments: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableL2VPNRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
	tenant: Option<i64>,
	custom_fields: String,
	identifier: Option<i64>,
	slug: String,
	description: String,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterface {
	name: String,
	cable: Option<i64>,
	_occupied: bool,
	id: i64,
	display: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableObjectPermissionRequest {
	groups: Vec<i64>,
	users: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	enabled: bool,
	name: String,
	description: String,
	object_types: Vec<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedTagRequest {
	description: String,
	color: String,
	slug: String,
	object_types: Vec<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PowerFeed {
	custom_fields: String,
	tags: Vec<NestedTag>,
	r#type: String,
	voltage: i64,
	link_peers: Vec<String>,
	comments: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	cable_end: String,
	phase: String,
	last_updated: Option<String>,
	id: i64,
	/// Treat as if a cable is connected
	mark_connected: bool,
	status: String,
	url: Url,
	supply: String,
	amperage: i64,
	connected_endpoints_reachable: bool,
	description: String,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	name: String,
	_occupied: bool,
	created: Option<String>,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct WritableContactAssignmentRequest {
	role: i64,
	object_id: i64,
	contact: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	content_type: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct RegionRequest {
	description: String,
	slug: String,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableFrontPortTemplateRequest {
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
	color: String,
	rear_port_position: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	device_type: Option<i64>,
	rear_port: i64,
	/// Physical label
	label: String,
	module_type: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct Site {
	/// Full name of the site
	name: String,
	comments: String,
	id: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	last_updated: Option<String>,
	url: Url,
	prefix_count: i64,
	virtualmachine_count: i64,
	device_count: i64,
	slug: String,
	/// If different from the physical address
	shipping_address: String,
	time_zone: Option<String>,
	/// Physical location of the building
	physical_address: String,
	/// Local facility ID or description
	facility: String,
	created: Option<String>,
	display: String,
	description: String,
	rack_count: i64,
	circuit_count: i64,
	status: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	asns: Vec<i64>,
	vlan_count: i64,
}

/// Adds support for custom fields and tags.
pub struct FrontPortRequest {
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	color: String,
	tags: Vec<NestedTagRequest>,
	description: String,
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
}

/// Adds support for custom fields and tags.
pub struct VirtualDisk {
	last_updated: Option<String>,
	size: i64,
	custom_fields: String,
	name: String,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
	description: String,
	url: Url,
}

pub struct PaginatedExportTemplateList {
	previous: Option<Url>,
	count: i64,
	results: Vec<ExportTemplate>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ModuleBay {
	/// Physical label
	label: String,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	id: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
	description: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPAddressRequest {
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
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
	assigned_object_type: Option<String>,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	description: String,
	tenant: Option<i64>,
	custom_fields: String,
	vrf: Option<i64>,
	assigned_object_id: Option<i64>,
	/// The operational status of this IP
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	/// * `dhcp` - DHCP
	/// * `slaac` - SLAAC
	status: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceTypeRequest {
	model: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerOutletRequest {
	device: i64,
	name: String,
	/// Physical label
	label: String,
	power_port: Option<i64>,
	module: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
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
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PowerPort {
	display: String,
	/// Physical label
	label: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	_occupied: bool,
	id: i64,
	name: String,
	r#type: Option<String>,
	link_peers: Vec<String>,
	connected_endpoints_reachable: bool,
	custom_fields: String,
	description: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	url: Url,
	connected_endpoints_type: String,
	cable_end: String,
	connected_endpoints: Vec<String>,
}

pub struct PaginatedClusterList {
	count: i64,
	results: Vec<Cluster>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ASNRequest {
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedRoleRequest {
	weight: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PrefixRequest {
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	description: String,
	custom_fields: String,
	comments: String,
	prefix: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableJournalEntryRequest {
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
	comments: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: String,
	assigned_object_id: i64,
	created_by: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct TenantRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	slug: String,
	name: String,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecPolicy {
	name: String,
	description: String,
	proposals: Vec<i64>,
	pfs_group: String,
	display: String,
	comments: String,
	tags: Vec<NestedTag>,
	url: Url,
	last_updated: Option<String>,
	id: i64,
	created: Option<String>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct RoleRequest {
	custom_fields: String,
	description: String,
	slug: String,
	name: String,
	weight: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedCircuitTypeRequest {
	name: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	color: String,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderNetworkRequest {
	custom_fields: String,
	name: String,
	provider: i64,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	service_id: String,
}

/// Adds support for custom fields and tags.
pub struct Device {
	display: String,
	rear_port_count: i64,
	console_port_count: i64,
	console_server_port_count: i64,
	description: String,
	position: Option<f64>,
	comments: String,
	tags: Vec<NestedTag>,
	device_bay_count: i64,
	vc_position: Option<i64>,
	created: Option<String>,
	front_port_count: i64,
	inventory_item_count: i64,
	interface_count: i64,
	module_bay_count: i64,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	id: i64,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	url: Url,
	name: Option<String>,
	face: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	airflow: String,
	status: String,
	last_updated: Option<String>,
	power_port_count: i64,
	custom_fields: String,
	power_outlet_count: i64,
}

/// Adds support for custom fields and tags.
pub struct L2VPNTerminationRequest {
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	assigned_object_id: i64,
}

pub struct PaginatedInterfaceList {
	count: i64,
	next: Option<Url>,
	results: Vec<Interface>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableFHRPGroupAssignmentRequest {
	interface_id: i64,
	interface_type: String,
	priority: i64,
	group: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLinkRequest {
	ssid: String,
}

pub struct PaginatedRearPortTemplateList {
	results: Vec<RearPortTemplate>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfile {
	name: String,
	id: i64,
	display: String,
	url: Url,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecPolicy {
	id: i64,
	display: String,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedManufacturerRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	slug: String,
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableWirelessLANGroupRequest {
	name: String,
	custom_fields: String,
	slug: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroup {
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	id: i64,
	display: String,
	slug: String,
	url: Url,
	tunnel_count: i64,
	created: Option<String>,
	last_updated: Option<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualMachineWithConfigContextRequest {
	role: Option<i64>,
	disk: Option<i64>,
	site: Option<i64>,
	primary_ip4: Option<i64>,
	memory: Option<i64>,
	custom_fields: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	name: String,
	device: Option<i64>,
	comments: String,
	tenant: Option<i64>,
	platform: Option<i64>,
	primary_ip6: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	cluster: Option<i64>,
	vcpus: Option<f64>,
}

/// Adds support for custom fields and tags.
pub struct WritableTenantRequest {
	group: Option<i64>,
	custom_fields: String,
	description: String,
	comments: String,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
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
	id: i64,
	group_id: i64,
	display: String,
}

pub struct PaginatedRackReservationList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<RackReservation>,
}

pub struct PaginatedL2VPNList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<L2VPN>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNTerminationRequest {
	assigned_object_type: String,
	l2vpn: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	assigned_object_id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModuleRequest {
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplateRequest {
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
	color: String,
	positions: i64,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ContactRequest {
	link: Url,
	phone: String,
	email: String,
	description: String,
	comments: String,
	address: String,
	title: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct VLAN {
	display: String,
	name: String,
	last_updated: Option<String>,
	id: i64,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	created: Option<String>,
	prefix_count: i64,
	description: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	status: String,
	url: Url,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WirelessLANGroupRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	name: String,
	slug: String,
}

pub struct PaginatedIKEProposalList {
	previous: Option<Url>,
	results: Vec<IKEProposal>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContextRequest {
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	description: String,
	comments: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	position: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `inventory` - Inventory
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	vc_position: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	name: Option<String>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualDeviceContext {
	url: Url,
	last_updated: Option<String>,
	custom_fields: String,
	description: String,
	name: String,
	display: String,
	id: i64,
	comments: String,
	status: String,
	interface_count: i64,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerFeedRequest {
	amperage: i64,
	rack: Option<i64>,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
	tenant: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	description: String,
	comments: String,
	custom_fields: String,
	power_panel: i64,
	voltage: i64,
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct WritableConfigTemplateRequest {
	name: String,
	data_file: Option<i64>,
	description: String,
	/// Jinja2 template code.
	template_code: String,
	/// Remote data source
	data_source: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableVLANRequest {
	/// The primary function of this VLAN
	role: Option<i64>,
	custom_fields: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	name: String,
	tags: Vec<NestedTagRequest>,
	/// VLAN group (optional)
	group: Option<i64>,
	tenant: Option<i64>,
	description: String,
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuit {
	url: Url,
	/// Unique circuit ID
	cid: String,
	display: String,
	id: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroupRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableTenantGroupRequest {
	custom_fields: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	parent: Option<i64>,
}

pub struct PaginatedDataSourceList {
	count: i64,
	next: Option<Url>,
	results: Vec<DataSource>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct IPSecProfile {
	name: String,
	last_updated: Option<String>,
	mode: String,
	display: String,
	custom_fields: String,
	description: String,
	url: Url,
	tags: Vec<NestedTag>,
	id: i64,
	created: Option<String>,
	comments: String,
}

pub struct PaginatedTenantList {
	results: Vec<Tenant>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
pub struct FrontPortRearPortRequest {
	name: String,
	description: String,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccountRequest {
	name: String,
	account: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTerminationRequest {
	description: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ASNRange {
	last_updated: Option<String>,
	id: i64,
	description: String,
	slug: String,
	name: String,
	end: i64,
	custom_fields: String,
	asn_count: i64,
	url: Url,
	display: String,
	created: Option<String>,
	start: i64,
	tags: Vec<NestedTag>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableContactGroupRequest {
	custom_fields: String,
	slug: String,
	description: String,
	name: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableSiteRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	time_zone: Option<String>,
	/// Local facility ID or description
	facility: String,
	/// If different from the physical address
	shipping_address: String,
	asns: Vec<i64>,
	custom_fields: String,
	/// Physical location of the building
	physical_address: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	comments: String,
	/// Full name of the site
	name: String,
	region: Option<i64>,
	group: Option<i64>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTermination {
	id: i64,
	url: Url,
	display: String,
}

pub struct PaginatedModuleBayTemplateList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<ModuleBayTemplate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsolePortTemplateRequest {
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
	device_type: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct IPRange {
	/// Treat as 100% utilized
	mark_utilized: bool,
	end_address: String,
	display: String,
	status: String,
	comments: String,
	start_address: String,
	url: Url,
	family: String,
	created: Option<String>,
	id: i64,
	description: String,
	custom_fields: String,
	size: i64,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
pub struct Module {
	serial: String,
	id: i64,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	comments: String,
	last_updated: Option<String>,
	custom_fields: String,
	url: Url,
	display: String,
	status: String,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct VLANGroupRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	scope_type: Option<String>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
	scope_id: Option<i64>,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WebhookRequest {
	custom_fields: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	tags: Vec<NestedTagRequest>,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	name: String,
	description: String,
}

pub struct PaginatedDeviceRoleList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<DeviceRole>,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObjectRequest {
	object_id: i64,
	object_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLinkRequest {
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
	weight: i64,
	name: String,
	/// Jinja2 template code for link text
	link_text: String,
	/// Jinja2 template code for link URL
	link_url: String,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	content_types: Vec<String>,
	enabled: bool,
	/// Force link to open in a new window
	new_window: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNTerminationRequest {
}

pub struct PaginatedContentTypeList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ContentType>,
	count: i64,
}

pub struct PaginatedModuleBayList {
	next: Option<Url>,
	results: Vec<ModuleBay>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRequest {
	/// 16- or 32-bit autonomous system number
	asn: i64,
	/// Regional Internet Registry responsible for this AS number space
	rir: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	tenant: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRole {
	id: i64,
	display: String,
	slug: String,
	color: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	inventoryitem_count: i64,
	name: String,
	last_updated: Option<String>,
	url: Url,
	description: String,
	created: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	component_type: Option<String>,
	description: String,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_id: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IKEProposal {
	display: String,
	authentication_method: String,
	id: i64,
	created: Option<String>,
	url: Url,
	authentication_algorithm: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	comments: String,
	tags: Vec<NestedTag>,
	name: String,
	last_updated: Option<String>,
	encryption_algorithm: String,
	description: String,
	custom_fields: String,
	group: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroup {
	description: String,
	custom_fields: String,
	created: Option<String>,
	display: String,
	slug: String,
	last_updated: Option<String>,
	_depth: i64,
	url: Url,
	tags: Vec<NestedTag>,
	id: i64,
	contact_count: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturer {
	id: i64,
	display: String,
	name: String,
	url: Url,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicy {
	custom_fields: String,
	proposals: Vec<i64>,
	display: String,
	comments: String,
	last_updated: Option<String>,
	url: Url,
	version: String,
	name: String,
	mode: String,
	preshared_key: String,
	id: i64,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct Circuit {
	status: String,
	/// Committed rate
	commit_rate: Option<i64>,
	url: Url,
	last_updated: Option<String>,
	display: String,
	comments: String,
	created: Option<String>,
	install_date: Option<String>,
	termination_date: Option<String>,
	tags: Vec<NestedTag>,
	custom_fields: String,
	id: i64,
	/// Unique circuit ID
	cid: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteGroup {
	display: String,
	id: i64,
	_depth: i64,
	slug: String,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignmentRequest {
	object_id: i64,
	content_type: String,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

pub struct PaginatedCableList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<Cable>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroup {
	slug: String,
	_depth: i64,
	name: String,
	display: String,
	id: i64,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConsoleServerPortTemplateRequest {
	device_type: Option<i64>,
	description: String,
	module_type: Option<i64>,
	/// Physical label
	label: String,
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

/// Representation of an IP address which does not exist in the database.
pub struct AvailableIP {
	family: i64,
	description: String,
	address: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPAddressRequest {
	address: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPN {
	url: Url,
	display: String,
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
	id: i64,
	identifier: Option<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInventoryItemRequest {
	role: Option<i64>,
	/// This item was automatically discovered
	discovered: bool,
	description: String,
	/// Physical label
	label: String,
	serial: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	device: i64,
	component_type: Option<String>,
	manufacturer: Option<i64>,
	component_id: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccount {
	custom_fields: String,
	id: i64,
	name: String,
	last_updated: Option<String>,
	comments: String,
	url: Url,
	display: String,
	account: String,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ClusterType {
	cluster_count: i64,
	description: String,
	url: Url,
	custom_fields: String,
	created: Option<String>,
	id: i64,
	name: String,
	slug: String,
	display: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct CircuitTermination {
	link_peers: Vec<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	last_updated: Option<String>,
	id: i64,
	custom_fields: String,
	description: String,
	_occupied: bool,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	tags: Vec<NestedTag>,
	display: String,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	url: Url,
	cable_end: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ProviderAccountRequest {
	account: String,
	description: String,
	comments: String,
	custom_fields: String,
	name: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct Role {
	slug: String,
	id: i64,
	created: Option<String>,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	prefix_count: i64,
	weight: i64,
	url: Url,
	vlan_count: i64,
	description: String,
	name: String,
	custom_fields: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct Webhook {
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	id: i64,
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	tags: Vec<NestedTag>,
	description: String,
	name: String,
	created: Option<String>,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	display: String,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	url: Url,
	last_updated: Option<String>,
	custom_fields: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableExportTemplateRequest {
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	description: String,
	/// Extension to append to the rendered filename
	file_extension: String,
	name: String,
	content_types: Vec<String>,
	/// Download file as attachment
	as_attachment: bool,
	/// Remote data source
	data_source: Option<i64>,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
}

/// Adds support for custom fields and tags.
pub struct WritableFrontPortRequest {
	rear_port: i64,
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
	module: Option<i64>,
	color: String,
	name: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPRangeRequest {
	description: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	end_address: String,
	custom_fields: String,
	tenant: Option<i64>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	start_address: String,
	vrf: Option<i64>,
	/// The primary function of this range
	role: Option<i64>,
	comments: String,
}

pub struct PaginatedCustomLinkList {
	count: i64,
	results: Vec<CustomLink>,
	previous: Option<Url>,
	next: Option<Url>,
}

pub struct PaginatedModuleTypeList {
	results: Vec<ModuleType>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WirelessLink {
	auth_psk: String,
	auth_cipher: String,
	url: Url,
	description: String,
	created: Option<String>,
	id: i64,
	last_updated: Option<String>,
	auth_type: String,
	comments: String,
	ssid: String,
	display: String,
	custom_fields: String,
	tags: Vec<NestedTag>,
	status: String,
}

pub struct ObjectChange {
	changed_object_type: String,
	changed_object_id: i64,
	action: String,
	request_id: String,
	id: i64,
	time: String,
	user_name: String,
	display: String,
	url: Url,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableContactGroupRequest {
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	parent: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRackRequest {
	site: i64,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	weight: Option<f64>,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	facility_id: Option<String>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	/// Starting unit for rack
	starting_unit: i64,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: String,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	/// Height in rack units
	u_height: i64,
	serial: String,
	tenant: Option<i64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	name: String,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	description: String,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	custom_fields: String,
	/// Functional role
	role: Option<i64>,
	location: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceType {
	model: String,
	slug: String,
	id: i64,
	url: Url,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterface {
	name: String,
	url: Url,
	display: String,
	id: i64,
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
pub struct WritableBookmarkRequest {
	user: i64,
	object_type: String,
	object_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Group {
	name: String,
	user_count: i64,
	display: String,
	id: i64,
	url: Url,
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
	description: String,
	/// Physical label
	label: String,
	device: i64,
	module: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableTenantGroupRequest {
	custom_fields: String,
	name: String,
	parent: Option<i64>,
	slug: String,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct EventRule {
	description: String,
	action_object_type: String,
	custom_fields: String,
	display: String,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	action_object: String,
	enabled: bool,
	name: String,
	id: i64,
	/// Triggers when a matching object is updated.
	type_update: bool,
	content_types: Vec<String>,
	action_object_id: Option<i64>,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	url: Url,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	created: Option<String>,
	action_type: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroup {
	group_id: i64,
	url: Url,
	created: Option<String>,
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
	custom_fields: String,
	auth_key: String,
	id: i64,
	comments: String,
	name: String,
	ip_addresses: Vec<NestedIPAddress>,
	display: String,
	last_updated: Option<String>,
	description: String,
	tags: Vec<NestedTag>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleBayNestedModule {
	display: String,
	id: i64,
	url: Url,
	serial: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCluster {
	id: i64,
	display: String,
	url: Url,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderNetworkRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	provider: i64,
	service_id: String,
	comments: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPortRequest {
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	module: Option<i64>,
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
	description: String,
	/// Physical label
	label: String,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	device: i64,
}

pub struct PaginatedIPSecPolicyList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<IPSecPolicy>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableTokenRequest {
	key: String,
	expires: Option<String>,
	last_used: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	user: i64,
}

/// Adds support for custom fields and tags.
pub struct WirelessLAN {
	url: Url,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	status: String,
	comments: String,
	auth_type: String,
	ssid: String,
	display: String,
	description: String,
	auth_psk: String,
	id: i64,
	auth_cipher: String,
	custom_fields: String,
}

pub struct PaginatedDataFileList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<DataFile>,
}

/// Adds support for custom fields and tags.
pub struct RackRequest {
	/// Starting unit for rack
	starting_unit: i64,
	/// * `mm` - Millimeters
	/// * `in` - Inches
	outer_unit: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	description: String,
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	/// Height in rack units
	u_height: i64,
	tags: Vec<NestedTagRequest>,
	/// * `2-post-frame` - 2-post frame
	/// * `4-post-frame` - 4-post frame
	/// * `4-post-cabinet` - 4-post cabinet
	/// * `wall-frame` - Wall-mounted frame
	/// * `wall-frame-vertical` - Wall-mounted frame (vertical)
	/// * `wall-cabinet` - Wall-mounted cabinet
	/// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
	r#type: Option<String>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	custom_fields: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	comments: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	serial: String,
	weight: Option<f64>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	facility_id: Option<String>,
	name: String,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplateRequest {
	description: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	content_types: Vec<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	/// Download file as attachment
	as_attachment: bool,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct JournalEntryRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_id: i64,
	comments: String,
	assigned_object_type: String,
	created_by: Option<i64>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
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

pub struct PaginatedCircuitTerminationList {
	results: Vec<CircuitTermination>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedUserList {
	results: Vec<User>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableInterfaceRequest {
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	device: i64,
	vrf: Option<i64>,
	vdcs: Vec<i64>,
	mac_address: Option<String>,
	custom_fields: String,
	module: Option<i64>,
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
	enabled: bool,
	/// * `half` - Half
	/// * `full` - Full
	/// * `auto` - Auto
	duplex: Option<String>,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
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
	tags: Vec<NestedTagRequest>,
	tagged_vlans: Vec<i64>,
	speed: Option<i64>,
	parent: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// * `type1-ieee802.3af` - 802.3af (Type 1)
	/// * `type2-ieee802.3at` - 802.3at (Type 2)
	/// * `type3-ieee802.3bt` - 802.3bt (Type 3)
	/// * `type4-ieee802.3bt` - 802.3bt (Type 4)
	/// * `passive-24v-2pair` - Passive 24V (2-pair)
	/// * `passive-24v-4pair` - Passive 24V (4-pair)
	/// * `passive-48v-2pair` - Passive 48V (2-pair)
	/// * `passive-48v-4pair` - Passive 48V (4-pair)
	poe_type: String,
	name: String,
	wwn: Option<String>,
	lag: Option<i64>,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	tx_power: Option<i64>,
	/// Physical label
	label: String,
	untagged_vlan: Option<i64>,
	mtu: Option<i64>,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	bridge: Option<i64>,
	description: String,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerOutletTemplateRequest {
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	power_port: Option<i64>,
	/// Physical label
	label: String,
	module_type: Option<i64>,
	device_type: Option<i64>,
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
	r#type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplateRequest {
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRouteTargetRequest {
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	custom_fields: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableEventRuleRequest {
	content_types: Vec<String>,
	action_object_id: Option<i64>,
	action_object_type: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	name: String,
	enabled: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Tenant {
	url: Url,
	display: String,
	slug: String,
	description: String,
	comments: String,
	custom_fields: String,
	prefix_count: i64,
	id: i64,
	device_count: i64,
	last_updated: Option<String>,
	tags: Vec<NestedTag>,
	name: String,
	circuit_count: i64,
	site_count: i64,
	created: Option<String>,
	vlan_count: i64,
	vrf_count: i64,
	rack_count: i64,
	cluster_count: i64,
	ipaddress_count: i64,
	virtualmachine_count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplate {
	/// Physical label
	label: String,
	r#type: String,
	rear_port_position: i64,
	created: Option<String>,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	url: Url,
	id: i64,
	color: String,
	description: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSourceRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableAggregateRequest {
	custom_fields: String,
	tenant: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	prefix: String,
	date_added: Option<String>,
	description: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassisRequest {
	domain: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
	name: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct CableTermination {
	display: String,
	last_updated: Option<String>,
	termination_id: i64,
	url: Url,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	cable: i64,
	created: Option<String>,
	id: i64,
	termination_type: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsolePortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	module_type: Option<i64>,
	description: String,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSet {
	id: i64,
	choices_count: String,
	display: String,
	name: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelTerminationRequest {
	outside_ip: Option<i64>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	tunnel: i64,
	termination_id: Option<i64>,
	termination_type: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct Location {
	rack_count: i64,
	description: String,
	slug: String,
	created: Option<String>,
	url: Url,
	display: String,
	status: String,
	last_updated: Option<String>,
	name: String,
	_depth: i64,
	custom_fields: String,
	id: i64,
	device_count: i64,
	tags: Vec<NestedTag>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterType {
	name: String,
	slug: String,
	display: String,
	url: Url,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanel {
	id: i64,
	url: Url,
	display: String,
	name: String,
}

pub struct PaginatedRackList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<Rack>,
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

pub struct PatchedDashboardRequest {
}

/// Adds support for custom fields and tags.
pub struct L2VPNTermination {
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	assigned_object_id: i64,
	assigned_object_type: String,
	custom_fields: String,
	created: Option<String>,
	id: i64,
	url: Url,
	display: String,
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
pub struct NestedProviderNetwork {
	display: String,
	id: i64,
	url: Url,
	name: String,
}

pub struct PaginatedObjectChangeList {
	results: Vec<ObjectChange>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedTenantGroupList {
	previous: Option<Url>,
	results: Vec<TenantGroup>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct Aggregate {
	tags: Vec<NestedTag>,
	family: String,
	comments: String,
	created: Option<String>,
	date_added: Option<String>,
	display: String,
	url: Url,
	prefix: String,
	description: String,
	id: i64,
	custom_fields: String,
	last_updated: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassis {
	name: String,
	id: i64,
	url: Url,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDeviceContextRequest {
	device: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	primary_ip6: Option<i64>,
	tenant: Option<i64>,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	custom_fields: String,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
	primary_ip4: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct CircuitCircuitTermination {
	id: i64,
	/// Physical circuit speed
	port_speed: Option<i64>,
	description: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	/// ID of the local cross-connect
	xconnect_id: String,
	display: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PowerPortRequest {
	name: String,
	custom_fields: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Physical label
	label: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleBayRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Rack {
	last_updated: Option<String>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	device_count: i64,
	comments: String,
	width: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	r#type: Option<String>,
	url: Url,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
	name: String,
	powerfeed_count: i64,
	display: String,
	facility_id: Option<String>,
	/// Height in rack units
	u_height: i64,
	status: String,
	/// Starting unit for rack
	starting_unit: i64,
	weight_unit: Option<String>,
	description: String,
	serial: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	created: Option<String>,
	id: i64,
	weight: Option<f64>,
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	outer_unit: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct RIRRequest {
	name: String,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	custom_fields: String,
}

/// Representation of a prefix which does not exist in the database.
pub struct AvailablePrefix {
	family: i64,
	prefix: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleTypeRequest {
	custom_fields: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	model: String,
	comments: String,
	weight: Option<f64>,
	/// Discrete part number (optional)
	part_number: String,
	manufacturer: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocationRequest {
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedGroupRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelGroupRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnel {
	name: String,
	url: Url,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct ManufacturerRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct BookmarkRequest {
	object_type: String,
	object_id: i64,
}

/// Adds support for custom fields and tags.
pub struct RackRoleRequest {
	custom_fields: String,
	slug: String,
	color: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDataSourceRequest {
	enabled: bool,
	description: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	name: String,
	r#type: String,
	source_url: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableRearPortRequest {
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Number of front ports which may be mapped
	positions: i64,
	device: i64,
	custom_fields: String,
	name: String,
	color: String,
	description: String,
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
pub struct PatchedWritablePrefixRequest {
	vrf: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	comments: String,
	vlan: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	prefix: String,
	/// The primary function of this prefix
	role: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	site: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldRequest {
	/// Internal field name
	name: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	object_type: String,
	description: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// * `always` - Always
	/// * `if-set` - If set
	/// * `hidden` - Hidden
	ui_visible: String,
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
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
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableCableRequest {
	custom_fields: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	a_terminations: Vec<GenericObjectRequest>,
	label: String,
	color: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
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
	b_terminations: Vec<GenericObjectRequest>,
	length: Option<f64>,
	description: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
}

pub struct PaginatedRIRList {
	results: Vec<RIR>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ASN {
	custom_fields: String,
	created: Option<String>,
	display: String,
	url: Url,
	/// 16- or 32-bit autonomous system number
	asn: i64,
	comments: String,
	last_updated: Option<String>,
	provider_count: i64,
	id: i64,
	description: String,
	site_count: i64,
	tags: Vec<NestedTag>,
}

pub struct PaginatedTunnelList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Tunnel>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	address: String,
	email: String,
	group: Option<i64>,
	description: String,
	comments: String,
	link: Url,
	title: String,
	phone: String,
}

/// Adds support for custom fields and tags.
pub struct WritablePlatformRequest {
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
	name: String,
	custom_fields: String,
	config_template: Option<i64>,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPAddressRequest {
	address: String,
	tenant: Option<i64>,
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
	assigned_object_type: Option<String>,
	/// The IP for which this address is the "outside" IP
	nat_inside: Option<i64>,
	description: String,
	assigned_object_id: Option<i64>,
	comments: String,
	vrf: Option<i64>,
	custom_fields: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitRequest {
	/// Unique circuit ID
	cid: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegionRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct DataSourceRequest {
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	comments: String,
	enabled: bool,
	description: String,
	source_url: String,
	name: String,
}

pub struct PaginatedVLANList {
	next: Option<Url>,
	results: Vec<VLAN>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ModuleBayTemplate {
	/// Identifier to reference when renaming installed components
	position: String,
	display: String,
	url: Url,
	description: String,
	last_updated: Option<String>,
	id: i64,
	created: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct SiteRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// Full name of the site
	name: String,
	slug: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	description: String,
	custom_fields: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// If different from the physical address
	shipping_address: String,
	asns: Vec<i64>,
	/// Local facility ID or description
	facility: String,
	comments: String,
	/// Physical location of the building
	physical_address: String,
	time_zone: Option<String>,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicyRequest {
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecPolicyRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualChassisRequest {
	comments: String,
	domain: String,
	custom_fields: String,
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	master: Option<i64>,
}

pub struct PaginatedVRFList {
	results: Vec<VRF>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPanelRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Cluster {
	id: i64,
	display: String,
	description: String,
	url: Url,
	virtualmachine_count: i64,
	last_updated: Option<String>,
	device_count: i64,
	custom_fields: String,
	status: String,
	comments: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ConsolePortRequest {
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	description: String,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct Service {
	protocol: String,
	ports: Vec<i64>,
	id: i64,
	comments: String,
	last_updated: Option<String>,
	ipaddresses: Vec<i64>,
	created: Option<String>,
	url: Url,
	name: String,
	display: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct IPAddressRequest {
	assigned_object_id: Option<i64>,
	address: String,
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
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	assigned_object_type: Option<String>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerOutletTemplate {
	r#type: Option<String>,
	/// Physical label
	label: String,
	id: i64,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	last_updated: Option<String>,
	feed_leg: Option<String>,
	url: Url,
	description: String,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDeviceContextRequest {
	custom_fields: String,
	/// Numeric identifier unique to the parent device
	identifier: Option<i64>,
	name: String,
	primary_ip6: Option<i64>,
	tenant: Option<i64>,
	device: Option<i64>,
	description: String,
	comments: String,
	primary_ip4: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `offline` - Offline
	status: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableObjectPermissionRequest {
	object_types: Vec<String>,
	description: String,
	enabled: bool,
	users: Vec<i64>,
	groups: Vec<i64>,
	name: String,
	/// The list of actions granted by this permission
	actions: Vec<String>,
}

pub struct PaginatedWebhookList {
	results: Vec<Webhook>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct ModuleRequest {
	custom_fields: String,
	serial: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualMachineRequest {
	name: String,
}

pub struct PaginatedWirelessLANGroupList {
	previous: Option<Url>,
	count: i64,
	results: Vec<WirelessLANGroup>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceTemplateRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ports: Vec<i64>,
	description: String,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct ContactGroupRequest {
	description: String,
	slug: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritablePowerPortTemplateRequest {
	module_type: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Physical label
	label: String,
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
	description: String,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	device_type: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct JournalEntry {
	created_by: Option<i64>,
	last_updated: Option<String>,
	kind: String,
	display: String,
	comments: String,
	url: Url,
	assigned_object_type: String,
	assigned_object_id: i64,
	tags: Vec<NestedTag>,
	custom_fields: String,
	id: i64,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplate {
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	custom_fields: String,
	ports: Vec<i64>,
	protocol: String,
	comments: String,
	description: String,
	url: Url,
	display: String,
	name: String,
}

pub struct PaginatedCircuitList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<Circuit>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroup {
	_depth: i64,
	name: String,
	tags: Vec<NestedTag>,
	display: String,
	created: Option<String>,
	last_updated: Option<String>,
	url: Url,
	description: String,
	slug: String,
	custom_fields: String,
	id: i64,
	tenant_count: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortRequest {
	name: String,
	cable: Option<i64>,
}

pub struct PaginatedIKEPolicyList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<IKEPolicy>,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEProposalRequest {
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
	comments: String,
	custom_fields: String,
	description: String,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVirtualChassisRequest {
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermission {
	url: Url,
	users: Vec<i64>,
	name: String,
	object_types: Vec<String>,
	enabled: bool,
	id: i64,
	groups: Vec<i64>,
	/// The list of actions granted by this permission
	actions: Vec<String>,
	description: String,
	display: String,
}

pub struct PaginatedProviderList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Provider>,
}

/// Adds support for custom fields and tags.
pub struct PatchedRackRoleRequest {
	color: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleRequest {
	module_bay: i64,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	device: i64,
	comments: String,
	custom_fields: String,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	serial: String,
	tags: Vec<NestedTagRequest>,
	module_type: i64,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderNetworkRequest {
	name: String,
}

pub struct PaginatedIPAddressList {
	count: i64,
	results: Vec<IPAddress>,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerPanelRequest {
	comments: String,
	description: String,
	name: String,
	location: Option<i64>,
	tags: Vec<NestedTagRequest>,
	site: i64,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableUserRequest {
	/// The groups this user belongs to. A user will get all permissions granted to each of their groups.
	groups: Vec<i64>,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	password: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	first_name: String,
	email: String,
	last_name: String,
	date_joined: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFHRPGroupAssignmentRequest {
	interface_type: String,
	group: i64,
	priority: i64,
	interface_id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceRoleRequest {
	color: String,
	slug: String,
	name: String,
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	description: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceWithConfigContextRequest {
	position: Option<f64>,
	/// The function this device serves
	role: i64,
	location: Option<i64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
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
	tenant: Option<i64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	description: String,
	comments: String,
	config_template: Option<i64>,
	cluster: Option<i64>,
	custom_fields: String,
	primary_ip4: Option<i64>,
	virtual_chassis: Option<i64>,
	name: Option<String>,
	device_type: i64,
	primary_ip6: Option<i64>,
	platform: Option<i64>,
	rack: Option<i64>,
	oob_ip: Option<i64>,
	site: i64,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	vc_position: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPanelRequest {
	location: Option<i64>,
	site: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
	name: String,
}

pub struct PaginatedConsoleServerPortList {
	count: i64,
	results: Vec<ConsoleServerPort>,
	next: Option<Url>,
	previous: Option<Url>,
}

pub struct PaginatedEventRuleList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<EventRule>,
}

pub struct PaginatedVirtualChassisList {
	results: Vec<VirtualChassis>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBayRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct Contact {
	description: String,
	comments: String,
	title: String,
	phone: String,
	last_updated: Option<String>,
	url: Url,
	address: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	name: String,
	created: Option<String>,
	id: i64,
	display: String,
	link: Url,
	email: String,
}

/// Used by device component serializers.
pub struct ComponentNestedModule {
	url: Url,
	id: i64,
	display: String,
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct DeviceRole {
	tags: Vec<NestedTag>,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	device_count: i64,
	custom_fields: String,
	created: Option<String>,
	slug: String,
	last_updated: Option<String>,
	color: String,
	display: String,
	virtualmachine_count: i64,
	description: String,
	url: Url,
	id: i64,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleType {
	model: String,
	display: String,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceRoleRequest {
	description: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	name: String,
	slug: String,
	color: String,
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignment {
	interface_id: i64,
	id: i64,
	priority: i64,
	last_updated: Option<String>,
	interface_type: String,
	display: String,
	url: Url,
	created: Option<String>,
}

pub struct PaginatedCircuitTypeList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<CircuitType>,
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
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableDeviceBayTemplateRequest {
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	device_type: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelRequest {
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	group: Option<i64>,
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	tunnel_id: Option<i64>,
	name: String,
	tenant: Option<i64>,
	ipsec_profile: Option<i64>,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct RearPortTemplate {
	created: Option<String>,
	last_updated: Option<String>,
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	display: String,
	/// Physical label
	label: String,
	url: Url,
	r#type: String,
	color: String,
	positions: i64,
	id: i64,
}

pub struct PaginatedContactGroupList {
	count: i64,
	results: Vec<ContactGroup>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct Manufacturer {
	created: Option<String>,
	devicetype_count: i64,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	platform_count: i64,
	id: i64,
	name: String,
	url: Url,
	inventoryitem_count: i64,
	slug: String,
	display: String,
	description: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableConfigContextRequest {
	regions: Vec<i64>,
	cluster_groups: Vec<i64>,
	clusters: Vec<i64>,
	locations: Vec<i64>,
	tags: Vec<String>,
	sites: Vec<i64>,
	cluster_types: Vec<i64>,
	roles: Vec<i64>,
	is_active: bool,
	site_groups: Vec<i64>,
	device_types: Vec<i64>,
	weight: i64,
	platforms: Vec<i64>,
	tenant_groups: Vec<i64>,
	tenants: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSite {
	display: String,
	id: i64,
	/// Full name of the site
	name: String,
	url: Url,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContextRequest {
	site_groups: Vec<i64>,
	regions: Vec<i64>,
	description: String,
	is_active: bool,
	roles: Vec<i64>,
	cluster_types: Vec<i64>,
	tenant_groups: Vec<i64>,
	locations: Vec<i64>,
	tenants: Vec<i64>,
	weight: i64,
	cluster_groups: Vec<i64>,
	clusters: Vec<i64>,
	device_types: Vec<i64>,
	tags: Vec<String>,
	sites: Vec<i64>,
	platforms: Vec<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelTerminationRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	termination_id: Option<i64>,
	termination_type: String,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInventoryItemTemplateRequest {
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	manufacturer: Option<i64>,
	/// Physical label
	label: String,
	device_type: i64,
	component_type: Option<String>,
	component_id: Option<i64>,
	parent: Option<i64>,
	role: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
}

/// Adds support for custom fields and tags.
pub struct ContactAssignment {
	id: i64,
	tags: Vec<NestedTag>,
	object: String,
	priority: String,
	display: String,
	content_type: String,
	object_id: i64,
	url: Url,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceTypeRequest {
	custom_fields: String,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	/// 
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: String,
	default_platform: Option<i64>,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	tags: Vec<NestedTagRequest>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	weight: Option<f64>,
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
	u_height: f64,
	front_image: String,
	description: String,
	/// Discrete part number (optional)
	part_number: String,
	rear_image: String,
	model: String,
	comments: String,
	manufacturer: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableFrontPortRequest {
	/// Physical label
	label: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	custom_fields: String,
	module: Option<i64>,
	device: i64,
	name: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	color: String,
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
	rear_port: i64,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTunnelRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceWithConfigContextRequest {
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
	config_template: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `front` - Front
	/// * `rear` - Rear
	face: String,
	platform: Option<i64>,
	location: Option<i64>,
	position: Option<f64>,
	oob_ip: Option<i64>,
	site: i64,
	device_type: i64,
	description: String,
	/// The function this device serves
	role: i64,
	cluster: Option<i64>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	tenant: Option<i64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	comments: String,
	primary_ip6: Option<i64>,
	vc_position: Option<i64>,
	rack: Option<i64>,
	primary_ip4: Option<i64>,
	name: Option<String>,
	custom_fields: String,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
	virtual_chassis: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRole {
	name: String,
	slug: String,
	display: String,
	id: i64,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct WritablePowerOutletRequest {
	name: String,
	custom_fields: String,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
	/// Physical label
	label: String,
	description: String,
	module: Option<i64>,
	device: i64,
	power_port: Option<i64>,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
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
	b_terminations: Vec<GenericObjectRequest>,
	color: String,
	a_terminations: Vec<GenericObjectRequest>,
	label: String,
	description: String,
	comments: String,
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
	length: Option<f64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroup {
	slug: String,
	display: String,
	url: Url,
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRouteTargetRequest {
	description: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceRequest {
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	ipaddresses: Vec<i64>,
	ports: Vec<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	comments: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsolePortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ModuleType {
	model: String,
	created: Option<String>,
	/// Discrete part number (optional)
	part_number: String,
	custom_fields: String,
	id: i64,
	url: Url,
	description: String,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	display: String,
	weight: Option<f64>,
	weight_unit: Option<String>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedCableTerminationRequest {
	termination_type: String,
	cable: i64,
	/// * `A` - A
	/// * `B` - B
	cable_end: String,
	termination_id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitRequest {
	provider: i64,
	/// Unique circuit ID
	cid: String,
	termination_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	provider_account: Option<i64>,
	/// Committed rate
	commit_rate: Option<i64>,
	comments: String,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	install_date: Option<String>,
	description: String,
	custom_fields: String,
	tenant: Option<i64>,
	r#type: i64,
}

pub struct PaginatedCustomFieldChoiceSetList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<CustomFieldChoiceSet>,
}

pub struct PaginatedVMInterfaceList {
	next: Option<Url>,
	results: Vec<VMInterface>,
	previous: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerOutletTemplateRequest {
	description: String,
	module_type: Option<i64>,
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
	/// Physical label
	label: String,
	device_type: Option<i64>,
	power_port: Option<i64>,
	/// Phase (for three-phase feeds)
	/// 
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TagRequest {
	color: String,
	name: String,
	slug: String,
	description: String,
	object_types: Vec<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRoleRequest {
	name: String,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct FrontPortTemplateRequest {
	color: String,
	description: String,
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
	rear_port_position: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableASNRangeRequest {
	tags: Vec<NestedTagRequest>,
	start: i64,
	custom_fields: String,
	name: String,
	tenant: Option<i64>,
	rir: i64,
	end: i64,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecPolicyRequest {
	proposals: Vec<i64>,
	comments: String,
	custom_fields: String,
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
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct TunnelTermination {
	role: String,
	termination_id: Option<i64>,
	url: Url,
	custom_fields: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	termination_type: String,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContextRequest {
	memory: Option<i64>,
	comments: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	description: String,
	name: String,
	vcpus: Option<f64>,
	disk: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVLANRequest {
	/// Operational status of this VLAN
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	tags: Vec<NestedTagRequest>,
	/// VLAN group (optional)
	group: Option<i64>,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	/// The specific site to which this VLAN is assigned (if any)
	site: Option<i64>,
	name: String,
	custom_fields: String,
	description: String,
	comments: String,
	tenant: Option<i64>,
	/// The primary function of this VLAN
	role: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct IPRangeRequest {
	tags: Vec<NestedTagRequest>,
	start_address: String,
	end_address: String,
	custom_fields: String,
	/// Treat as 100% utilized
	mark_utilized: bool,
	description: String,
	comments: String,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderAccountRequest {
	account: String,
	name: String,
	comments: String,
	description: String,
	custom_fields: String,
	provider: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ModuleTypeRequest {
	model: String,
	/// Discrete part number (optional)
	part_number: String,
	weight: Option<f64>,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	tags: Vec<NestedTagRequest>,
	description: String,
	comments: String,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplateRequest {
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: Option<String>,
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
	poe_type: Option<String>,
	mgmt_only: bool,
	enabled: bool,
	description: String,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: Option<String>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRearPortTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitType {
	id: i64,
	slug: String,
	url: Url,
	display: String,
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
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRoleRequest {
	slug: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIKEPolicy {
	name: String,
	url: Url,
	id: i64,
	display: String,
}

pub struct PaginatedContactList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<Contact>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedL2VPNRequest {
	identifier: Option<i64>,
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
	name: String,
}

pub struct PaginatedVLANGroupList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<VLANGroup>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableModuleBayTemplateRequest {
	/// Physical label
	label: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct TunnelRequest {
	custom_fields: String,
	comments: String,
	name: String,
	tunnel_id: Option<i64>,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WirelessLANRequest {
	ssid: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	description: String,
	comments: String,
	auth_psk: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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

pub struct PaginatedConfigContextList {
	results: Vec<ConfigContext>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RearPortRequest {
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	custom_fields: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
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
	/// Number of front ports which may be mapped
	positions: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableConsoleServerPortRequest {
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
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
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
	module: Option<i64>,
	/// Physical label
	label: String,
	device: i64,
}

/// Adds support for custom fields and tags.
pub struct DataSource {
	display: String,
	source_url: String,
	enabled: bool,
	url: Url,
	id: i64,
	name: String,
	created: Option<String>,
	comments: String,
	r#type: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	description: String,
	file_count: i64,
	last_updated: Option<String>,
	status: String,
}

/// Adds support for custom fields and tags.
pub struct InventoryItem {
	serial: String,
	component_id: Option<i64>,
	display: String,
	last_updated: Option<String>,
	component_type: Option<String>,
	name: String,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	url: Url,
	/// Manufacturer-assigned part identifier
	part_id: String,
	tags: Vec<NestedTag>,
	_depth: i64,
	/// This item was automatically discovered
	discovered: bool,
	description: String,
	created: Option<String>,
	id: i64,
	/// Physical label
	label: String,
	parent: Option<i64>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableConsoleServerPortRequest {
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
	tags: Vec<NestedTagRequest>,
	device: i64,
	module: Option<i64>,
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct FrontPort {
	created: Option<String>,
	r#type: String,
	custom_fields: String,
	/// Mapped position on corresponding rear port
	rear_port_position: i64,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	display: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	color: String,
	name: String,
	link_peers: Vec<String>,
	description: String,
	cable_end: String,
	id: i64,
	_occupied: bool,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InventoryItemTemplate {
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	created: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
	display: String,
	description: String,
	_depth: i64,
	url: Url,
	last_updated: Option<String>,
	/// Physical label
	label: String,
	component_id: Option<i64>,
	parent: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProposalRequest {
	name: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
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
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	description: String,
}

pub struct PaginatedASNList {
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ASN>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceTypeRequest {
	manufacturer: i64,
	description: String,
	front_image: String,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: String,
	weight: Option<f64>,
	rear_image: String,
	tags: Vec<NestedTagRequest>,
	/// Discrete part number (optional)
	part_number: String,
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
	comments: String,
	u_height: f64,
	default_platform: Option<i64>,
	model: String,
	custom_fields: String,
	slug: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
}

/// Adds support for custom fields and tags.
pub struct AggregateRequest {
	prefix: String,
	date_added: Option<String>,
	comments: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableCircuitTerminationRequest {
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	/// ID of the local cross-connect
	xconnect_id: String,
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	provider_network: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	description: String,
	site: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	circuit: i64,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableSiteGroupRequest {
	custom_fields: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	parent: Option<i64>,
	description: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableWirelessLANGroupRequest {
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	parent: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableClusterRequest {
	comments: String,
	r#type: i64,
	description: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	tenant: Option<i64>,
	name: String,
	group: Option<i64>,
	site: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct Cable {
	status: String,
	last_updated: Option<String>,
	length: Option<f64>,
	custom_fields: String,
	description: String,
	created: Option<String>,
	b_terminations: Vec<GenericObject>,
	id: i64,
	display: String,
	tags: Vec<NestedTag>,
	url: Url,
	label: String,
	comments: String,
	color: String,
	length_unit: Option<String>,
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
	a_terminations: Vec<GenericObject>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomField {
	ui_visible: String,
	id: i64,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	data_type: String,
	created: Option<String>,
	description: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	object_type: String,
	last_updated: Option<String>,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	ui_editable: String,
	url: Url,
	content_types: Vec<String>,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	r#type: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	display: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	filter_logic: String,
	/// Internal field name
	name: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedLocation {
	display: String,
	name: String,
	url: Url,
	_depth: i64,
	slug: String,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRIR {
	slug: String,
	name: String,
	id: i64,
	display: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedTunnelGroupRequest {
	custom_fields: String,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePlatformRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	config_template: Option<i64>,
	slug: String,
	description: String,
	name: String,
	/// Optionally limit this platform to devices of a certain manufacturer
	manufacturer: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableVMInterfaceRequest {
	tags: Vec<NestedTagRequest>,
	mac_address: Option<String>,
	enabled: bool,
	bridge: Option<i64>,
	tagged_vlans: Vec<i64>,
	custom_fields: String,
	name: String,
	/// IEEE 802.1Q tagging strategy
	/// 
	/// * `access` - Access
	/// * `tagged` - Tagged
	/// * `tagged-all` - Tagged (All)
	mode: String,
	virtual_machine: i64,
	mtu: Option<i64>,
	untagged_vlan: Option<i64>,
	parent: Option<i64>,
	vrf: Option<i64>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLinkRequest {
	comments: String,
	interface_a: i64,
	auth_psk: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	interface_b: i64,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ssid: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRoleRequest {
	slug: String,
	name: String,
}

pub struct PaginatedIPSecProfileList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<IPSecProfile>,
}

pub struct PaginatedObjectPermissionList {
	next: Option<Url>,
	results: Vec<ObjectPermission>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedWirelessLinkList {
	next: Option<Url>,
	results: Vec<WirelessLink>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedInventoryItemList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<InventoryItem>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroupRequest {
	slug: String,
	name: String,
}

pub struct PaginatedVirtualDiskList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<VirtualDisk>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedCustomLinkRequest {
	name: String,
	enabled: bool,
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	/// Jinja2 template code for link URL
	link_url: String,
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
	/// Force link to open in a new window
	new_window: bool,
	/// Jinja2 template code for link text
	link_text: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableModuleBayTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
	device_type: i64,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
}

pub struct PaginatedBookmarkList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<Bookmark>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackReservationRequest {
	units: Vec<i64>,
	tenant: Option<i64>,
	user: i64,
	description: String,
	comments: String,
	custom_fields: String,
	rack: i64,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct WritableProviderRequest {
	comments: String,
	/// Full name of the provider
	name: String,
	description: String,
	asns: Vec<i64>,
	tags: Vec<NestedTagRequest>,
	accounts: Vec<i64>,
	custom_fields: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct IPSecProposal {
	tags: Vec<NestedTag>,
	display: String,
	description: String,
	encryption_algorithm: String,
	authentication_algorithm: String,
	created: Option<String>,
	custom_fields: String,
	url: Url,
	id: i64,
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	comments: String,
	name: String,
	last_updated: Option<String>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Tag {
	tagged_items: i64,
	created: Option<String>,
	name: String,
	color: String,
	url: Url,
	display: String,
	slug: String,
	id: i64,
	description: String,
	last_updated: Option<String>,
	object_types: Vec<String>,
}

pub struct PaginatedContactRoleList {
	results: Vec<ContactRole>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachmentRequest {
	image_height: i64,
	content_type: String,
	image: String,
	name: String,
	object_id: i64,
	image_width: i64,
}

pub struct PaginatedVirtualMachineWithConfigContextList {
	next: Option<Url>,
	results: Vec<VirtualMachineWithConfigContext>,
	previous: Option<Url>,
	count: i64,
}

pub struct PaginatedContactAssignmentList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ContactAssignment>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataSource {
	url: Url,
	id: i64,
	display: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritablePowerPortTemplateRequest {
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
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	module_type: Option<i64>,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Physical label
	label: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantRequest {
	slug: String,
	name: String,
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
pub struct NestedDevice {
	url: Url,
	id: i64,
	name: Option<String>,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRackRequest {
	/// Outer dimension of rack (depth)
	outer_depth: Option<i64>,
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
	/// Outer dimension of rack (width)
	outer_width: Option<i64>,
	/// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
	mounting_depth: Option<i64>,
	description: String,
	/// * `reserved` - Reserved
	/// * `available` - Available
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `deprecated` - Deprecated
	status: String,
	custom_fields: String,
	site: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	/// Maximum load capacity for the rack
	max_weight: Option<i64>,
	tags: Vec<NestedTagRequest>,
	weight: Option<f64>,
	comments: String,
	/// Units are numbered top-to-bottom
	desc_units: bool,
	tenant: Option<i64>,
	/// Height in rack units
	u_height: i64,
	name: String,
	/// Starting unit for rack
	starting_unit: i64,
	/// Functional role
	role: Option<i64>,
	facility_id: Option<String>,
	/// A unique tag used to identify this rack
	asset_tag: Option<String>,
	/// Rail-to-rail width
	/// 
	/// * `10` - 10 inches
	/// * `19` - 19 inches
	/// * `21` - 21 inches
	/// * `23` - 23 inches
	width: i64,
	location: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct CircuitTypeRequest {
	name: String,
	color: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct GroupRequest {
	name: String,
}

pub struct PaginatedDeviceWithConfigContextList {
	count: i64,
	next: Option<Url>,
	results: Vec<DeviceWithConfigContext>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualDiskRequest {
	name: String,
	virtual_machine: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	size: i64,
	description: String,
}

pub struct PaginatedInterfaceTemplateList {
	results: Vec<InterfaceTemplate>,
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedManufacturerRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PowerPanelRequest {
	comments: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct VRFRequest {
	import_targets: Vec<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
	comments: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	export_targets: Vec<i64>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANRequest {
	name: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterTypeRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInventoryItemTemplateRequest {
	device_type: i64,
	/// Manufacturer-assigned part identifier
	part_id: String,
	component_type: Option<String>,
	component_id: Option<i64>,
	role: Option<i64>,
	/// Physical label
	label: String,
	description: String,
	parent: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	manufacturer: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct DeviceBayTemplateRequest {
	description: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableServiceRequest {
	device: Option<i64>,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	name: String,
	comments: String,
	custom_fields: String,
	description: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	tags: Vec<NestedTagRequest>,
	virtual_machine: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct ContactRoleRequest {
	custom_fields: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEProposalRequest {
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
	name: String,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	/// Security association lifetime (in seconds)
	sa_lifetime: Option<i64>,
	/// * `preshared-keys` - Pre-shared keys
	/// * `certificates` - Certificates
	/// * `rsa-signatures` - RSA signatures
	/// * `dsa-signatures` - DSA signatures
	authentication_method: String,
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
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct IPAddress {
	assigned_object_id: Option<i64>,
	assigned_object_type: Option<String>,
	id: i64,
	url: Url,
	family: String,
	role: String,
	/// Hostname or FQDN (not case-sensitive)
	dns_name: String,
	description: String,
	display: String,
	created: Option<String>,
	nat_outside: Vec<NestedIPAddress>,
	tags: Vec<NestedTag>,
	comments: String,
	custom_fields: String,
	status: String,
	address: String,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct VirtualDiskRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	name: String,
	size: i64,
	description: String,
}

pub struct PaginatedDeviceTypeList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<DeviceType>,
}

pub struct PaginatedRouteTargetList {
	results: Vec<RouteTarget>,
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCircuitTypeRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceBayRequest {
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	custom_fields: String,
	description: String,
}

pub struct PaginatedDeviceBayList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<DeviceBay>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableRegionRequest {
	slug: String,
	name: String,
	parent: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct PowerFeedRequest {
	/// Maximum permissible draw (percentage)
	max_utilization: i64,
	voltage: i64,
	/// * `single-phase` - Single phase
	/// * `three-phase` - Three-phase
	phase: String,
	custom_fields: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	name: String,
	/// * `ac` - AC
	/// * `dc` - DC
	supply: String,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `failed` - Failed
	status: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	amperage: i64,
	/// * `primary` - Primary
	/// * `redundant` - Redundant
	r#type: String,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitRequest {
	description: String,
	install_date: Option<String>,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	tenant: Option<i64>,
	termination_date: Option<String>,
	/// Committed rate
	commit_rate: Option<i64>,
	comments: String,
	provider_account: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// Unique circuit ID
	cid: String,
	r#type: i64,
	custom_fields: String,
	provider: i64,
}

pub struct PaginatedSiteGroupList {
	count: i64,
	results: Vec<SiteGroup>,
	previous: Option<Url>,
	next: Option<Url>,
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
pub struct WritableL2VPNTerminationRequest {
	custom_fields: String,
	assigned_object_id: i64,
	l2vpn: i64,
	assigned_object_type: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplateRequest {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedRIRRequest {
	custom_fields: String,
	name: String,
	description: String,
	slug: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ConsolePort {
	last_updated: Option<String>,
	_occupied: bool,
	name: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	url: Url,
	cable_end: String,
	display: String,
	id: i64,
	r#type: String,
	/// Physical label
	label: String,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints: Vec<String>,
	connected_endpoints_type: String,
	link_peers: Vec<String>,
	speed: Option<String>,
	connected_endpoints_reachable: bool,
	created: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct InventoryItemRequest {
	component_type: Option<String>,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	tags: Vec<NestedTagRequest>,
	component_id: Option<i64>,
	name: String,
	/// Physical label
	label: String,
	custom_fields: String,
	parent: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	serial: String,
	/// This item was automatically discovered
	discovered: bool,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModuleBay {
	name: String,
	id: i64,
	display: String,
	url: Url,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ExportTemplate {
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	url: Url,
	description: String,
	content_types: Vec<String>,
	created: Option<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	display: String,
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	id: i64,
	name: String,
	data_synced: Option<String>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	/// Download file as attachment
	as_attachment: bool,
	last_updated: Option<String>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplateRequest {
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomLink {
	/// Links with the same group will appear as a dropdown menu
	group_name: String,
	content_types: Vec<String>,
	name: String,
	url: Url,
	enabled: bool,
	/// Jinja2 template code for link text
	link_text: String,
	display: String,
	/// Jinja2 template code for link URL
	link_url: String,
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
	/// Force link to open in a new window
	new_window: bool,
	last_updated: Option<String>,
	id: i64,
	created: Option<String>,
}

pub struct PaginatedJournalEntryList {
	results: Vec<JournalEntry>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleRequest {
	comments: String,
	device: i64,
	module_type: i64,
	tags: Vec<NestedTagRequest>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	serial: String,
	custom_fields: String,
	description: String,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	module_bay: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableDeviceBayRequest {
	device: i64,
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	installed_device: Option<i64>,
	/// Physical label
	label: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDataFile {
	id: i64,
	url: Url,
	display: String,
	/// File path relative to the data source's root
	path: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInterfaceTemplate {
	display: String,
	id: i64,
	url: Url,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRoleRequest {
	slug: String,
	name: String,
}

pub struct PaginatedFrontPortList {
	count: i64,
	previous: Option<Url>,
	results: Vec<FrontPort>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPRangeRequest {
	comments: String,
	vrf: Option<i64>,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// The primary function of this range
	role: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	tenant: Option<i64>,
	/// Operational status of this range
	/// 
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	start_address: String,
	description: String,
	end_address: String,
}

pub struct PaginatedConsolePortList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<ConsolePort>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableL2VPNRequest {
	identifier: Option<i64>,
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
	comments: String,
	description: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	import_targets: Vec<i64>,
	export_targets: Vec<i64>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableWirelessLinkRequest {
	interface_b: i64,
	ssid: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	auth_psk: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	tenant: Option<i64>,
	interface_a: i64,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenRequest {
	key: String,
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	last_used: Option<String>,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct Interface {
	enabled: bool,
	/// Physical label
	label: String,
	tagged_vlans: Vec<i64>,
	url: Url,
	rf_role: String,
	/// Populated by selected channel (if set)
	rf_channel_width: Option<f64>,
	mtu: Option<i64>,
	_occupied: bool,
	/// Treat as if a cable is connected
	mark_connected: bool,
	wireless_lans: Vec<i64>,
	last_updated: Option<String>,
	tx_power: Option<i64>,
	duplex: Option<String>,
	connected_endpoints: Vec<String>,
	mac_address: Option<String>,
	wwn: Option<String>,
	vdcs: Vec<i64>,
	link_peers: Vec<String>,
	rf_channel: String,
	mode: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	description: String,
	/// Populated by selected channel (if set)
	rf_channel_frequency: Option<f64>,
	custom_fields: String,
	created: Option<String>,
	r#type: String,
	name: String,
	id: i64,
	/// This interface is used only for out-of-band management
	mgmt_only: bool,
	display: String,
	poe_mode: String,
	speed: Option<i64>,
	tags: Vec<NestedTag>,
	count_ipaddresses: i64,
	count_fhrp_groups: i64,
	connected_endpoints_type: String,
	connected_endpoints_reachable: bool,
	cable_end: String,
	poe_type: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPNRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
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
	description: String,
	export_targets: Vec<i64>,
	slug: String,
	name: String,
	import_targets: Vec<i64>,
	comments: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRole {
	slug: String,
	url: Url,
	display: String,
	id: i64,
	name: String,
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
pub struct NestedTunnelGroup {
	slug: String,
	id: i64,
	url: Url,
	display: String,
	name: String,
}

pub struct PaginatedInventoryItemRoleList {
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
	results: Vec<InventoryItemRole>,
}

pub struct PaginatedPowerOutletList {
	next: Option<Url>,
	count: i64,
	results: Vec<PowerOutlet>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RearPort {
	/// Physical label
	label: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// Number of front ports which may be mapped
	positions: i64,
	r#type: String,
	display: String,
	url: Url,
	name: String,
	link_peers: Vec<String>,
	tags: Vec<NestedTag>,
	last_updated: Option<String>,
	cable_end: String,
	color: String,
	created: Option<String>,
	custom_fields: String,
	_occupied: bool,
	description: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritablePrefixRequest {
	site: Option<i64>,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	/// The primary function of this prefix
	role: Option<i64>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	/// Operational status of this prefix
	/// 
	/// * `container` - Container
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	description: String,
	comments: String,
	custom_fields: String,
	prefix: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	vrf: Option<i64>,
	vlan: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableServiceRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	ports: Vec<i64>,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	virtual_machine: Option<i64>,
	name: String,
	/// The specific IP addresses (if any) to which this service is bound
	ipaddresses: Vec<i64>,
	description: String,
	device: Option<i64>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIPSecProfileRequest {
	comments: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
	ike_policy: i64,
	ipsec_policy: i64,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualMachineWithConfigContextRequest {
	cluster: Option<i64>,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	site: Option<i64>,
	memory: Option<i64>,
	/// * `offline` - Offline
	/// * `active` - Active
	/// * `planned` - Planned
	/// * `staged` - Staged
	/// * `failed` - Failed
	/// * `decommissioning` - Decommissioning
	status: String,
	tenant: Option<i64>,
	comments: String,
	role: Option<i64>,
	primary_ip4: Option<i64>,
	primary_ip6: Option<i64>,
	platform: Option<i64>,
	name: String,
	disk: Option<i64>,
	vcpus: Option<f64>,
	device: Option<i64>,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct PatchedWritableConfigTemplateRequest {
	name: String,
	description: String,
	/// Jinja2 template code.
	template_code: String,
	/// Remote data source
	data_source: Option<i64>,
	data_file: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct ClusterGroup {
	url: Url,
	last_updated: Option<String>,
	cluster_count: i64,
	id: i64,
	name: String,
	tags: Vec<NestedTag>,
	description: String,
	custom_fields: String,
	display: String,
	slug: String,
	created: Option<String>,
}

pub struct PaginatedConsoleServerPortTemplateList {
	results: Vec<ConsoleServerPortTemplate>,
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableDataSourceRequest {
	enabled: bool,
	r#type: String,
	source_url: String,
	description: String,
	/// Patterns (one per line) matching files to ignore when syncing
	ignore_rules: String,
	name: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct ServiceTemplateRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	ports: Vec<i64>,
	custom_fields: String,
	description: String,
	/// * `tcp` - TCP
	/// * `udp` - UDP
	/// * `sctp` - SCTP
	protocol: String,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLAN {
	id: i64,
	url: Url,
	display: String,
	name: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
}

pub struct PaginatedPowerFeedList {
	results: Vec<PowerFeed>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVMInterfaceRequest {
	name: String,
}

pub struct PaginatedModuleList {
	count: i64,
	next: Option<Url>,
	results: Vec<Module>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct EventRuleRequest {
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	content_types: Vec<String>,
	name: String,
	/// Triggers when a matching object is updated.
	type_update: bool,
	action_object_id: Option<i64>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	action_object_type: String,
	/// Triggers when a matching object is created.
	type_create: bool,
	tags: Vec<NestedTagRequest>,
	enabled: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	custom_fields: String,
	description: String,
}

pub struct PaginatedPowerPortList {
	previous: Option<Url>,
	results: Vec<PowerPort>,
	next: Option<Url>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetwork {
	custom_fields: String,
	description: String,
	last_updated: Option<String>,
	display: String,
	tags: Vec<NestedTag>,
	url: Url,
	created: Option<String>,
	id: i64,
	name: String,
	service_id: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderNetworkRequest {
	name: String,
	custom_fields: String,
	service_id: String,
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRegion {
	url: Url,
	display: String,
	name: String,
	slug: String,
	_depth: i64,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedConfigTemplate {
	url: Url,
	name: String,
	display: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecPolicyRequest {
	name: String,
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
	description: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableInterfaceTemplateRequest {
	device_type: Option<i64>,
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	bridge: Option<i64>,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
	description: String,
	module_type: Option<i64>,
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
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	mgmt_only: bool,
}

pub struct PaginatedPowerPanelList {
	count: i64,
	results: Vec<PowerPanel>,
	previous: Option<Url>,
	next: Option<Url>,
}

pub struct PaginatedPrefixList {
	previous: Option<Url>,
	count: i64,
	results: Vec<Prefix>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct CircuitRequest {
	termination_date: Option<String>,
	tags: Vec<NestedTagRequest>,
	comments: String,
	/// Unique circuit ID
	cid: String,
	custom_fields: String,
	/// * `planned` - Planned
	/// * `provisioning` - Provisioning
	/// * `active` - Active
	/// * `offline` - Offline
	/// * `deprovisioning` - Deprovisioning
	/// * `decommissioned` - Decommissioned
	status: String,
	/// Committed rate
	commit_rate: Option<i64>,
	description: String,
	install_date: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContactRequest {
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableTunnelRequest {
	/// * `ipsec-transport` - IPsec - Transport
	/// * `ipsec-tunnel` - IPsec - Tunnel
	/// * `ip-ip` - IP-in-IP
	/// * `gre` - GRE
	encapsulation: String,
	ipsec_profile: Option<i64>,
	description: String,
	tags: Vec<NestedTagRequest>,
	tenant: Option<i64>,
	custom_fields: String,
	name: String,
	tunnel_id: Option<i64>,
	group: Option<i64>,
	/// * `planned` - Planned
	/// * `active` - Active
	/// * `disabled` - Disabled
	status: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceRoleRequest {
	custom_fields: String,
	color: String,
	/// Virtual machines may be assigned to this role
	vm_role: bool,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
	description: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceTypeRequest {
	/// Discrete part number (optional)
	part_number: String,
	custom_fields: String,
	model: String,
	description: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	tags: Vec<NestedTagRequest>,
	slug: String,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: Option<String>,
	front_image: String,
	u_height: f64,
	/// * `parent` - Parent
	/// * `child` - Child
	subdevice_role: Option<String>,
	weight: Option<f64>,
	/// * `front-to-rear` - Front to rear
	/// * `rear-to-front` - Rear to front
	/// * `left-to-right` - Left to right
	/// * `right-to-left` - Right to left
	/// * `side-to-rear` - Side to rear
	/// * `passive` - Passive
	/// * `mixed` - Mixed
	airflow: Option<String>,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	rear_image: String,
	comments: String,
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
pub struct NestedClusterTypeRequest {
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableContactAssignmentRequest {
	content_type: String,
	contact: i64,
	custom_fields: String,
	object_id: i64,
	/// * `primary` - Primary
	/// * `secondary` - Secondary
	/// * `tertiary` - Tertiary
	/// * `inactive` - Inactive
	priority: String,
	tags: Vec<NestedTagRequest>,
	role: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConfigContextRequest {
	tenant_groups: Vec<i64>,
	weight: i64,
	sites: Vec<i64>,
	regions: Vec<i64>,
	clusters: Vec<i64>,
	roles: Vec<i64>,
	tenants: Vec<i64>,
	cluster_groups: Vec<i64>,
	platforms: Vec<i64>,
	/// Remote data source
	data_source: Option<i64>,
	description: String,
	site_groups: Vec<i64>,
	locations: Vec<i64>,
	cluster_types: Vec<i64>,
	name: String,
	device_types: Vec<i64>,
	tags: Vec<String>,
	is_active: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedVLANGroupRequest {
	name: String,
	scope_type: Option<String>,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	scope_id: Option<i64>,
	/// Lowest permissible ID of a child VLAN
	min_vid: i64,
	/// Highest permissible ID of a child VLAN
	max_vid: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVLANGroup {
	slug: String,
	display: String,
	name: String,
	url: Url,
	id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Bookmark {
	id: i64,
	object_id: i64,
	object_type: String,
	url: Url,
	created: String,
	display: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplate {
	/// Path to remote file (relative to data source root)
	data_path: String,
	data_synced: Option<String>,
	display: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	url: Url,
	id: i64,
	/// Jinja2 template code.
	template_code: String,
	name: String,
	description: String,
}

pub struct PaginatedSiteList {
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<Site>,
	count: i64,
}

/// Adds support for custom fields and tags.
pub struct RackReservation {
	id: i64,
	created: Option<String>,
	display: String,
	last_updated: Option<String>,
	custom_fields: String,
	description: String,
	url: Url,
	units: Vec<i64>,
	tags: Vec<NestedTag>,
	comments: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableSiteGroupRequest {
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	name: String,
	parent: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProposalRequest {
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
	/// Security association lifetime (seconds)
	sa_lifetime_seconds: Option<i64>,
	/// Security association lifetime (in kilobytes)
	sa_lifetime_data: Option<i64>,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `hmac-sha1` - SHA-1 HMAC
	/// * `hmac-sha256` - SHA-256 HMAC
	/// * `hmac-sha384` - SHA-384 HMAC
	/// * `hmac-sha512` - SHA-512 HMAC
	/// * `hmac-md5` - MD5 HMAC
	authentication_algorithm: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ImageAttachment {
	id: i64,
	image_height: i64,
	created: Option<String>,
	image_width: i64,
	last_updated: Option<String>,
	image: Url,
	name: String,
	object_id: i64,
	content_type: String,
	url: Url,
	display: String,
}

pub struct PaginatedFrontPortTemplateList {
	count: i64,
	results: Vec<FrontPortTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConfigContext {
	id: i64,
	site_groups: Vec<i64>,
	roles: Vec<i64>,
	platforms: Vec<i64>,
	is_active: bool,
	url: Url,
	clusters: Vec<i64>,
	/// Path to remote file (relative to data source root)
	data_path: String,
	tags: Vec<String>,
	last_updated: Option<String>,
	data_synced: Option<String>,
	tenant_groups: Vec<i64>,
	tenants: Vec<i64>,
	regions: Vec<i64>,
	device_types: Vec<i64>,
	description: String,
	locations: Vec<i64>,
	sites: Vec<i64>,
	display: String,
	created: Option<String>,
	name: String,
	cluster_types: Vec<i64>,
	weight: i64,
	cluster_groups: Vec<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableConsoleServerPortTemplateRequest {
	/// Physical label
	label: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
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
	device_type: Option<i64>,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ObjectPermissionRequest {
	description: String,
	groups: Vec<i64>,
	object_types: Vec<String>,
	users: Vec<i64>,
	enabled: bool,
	name: String,
	/// The list of actions granted by this permission
	actions: Vec<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedClusterGroupRequest {
	name: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPortTemplate {
	url: Url,
	display: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct DataFile {
	id: i64,
	display: String,
	size: i64,
	url: Url,
	/// File path relative to the data source's root
	path: String,
	last_updated: String,
	/// SHA256 hash of the file data
	hash: String,
}

pub struct Job {
	scheduled: Option<String>,
	error: String,
	/// Recurrence interval (in minutes)
	interval: Option<i64>,
	completed: Option<String>,
	id: i64,
	display: String,
	url: Url,
	job_id: String,
	object_id: Option<i64>,
	object_type: String,
	name: String,
	started: Option<String>,
	created: String,
	status: String,
}

pub struct PaginatedWirelessLANList {
	previous: Option<Url>,
	results: Vec<WirelessLAN>,
	count: i64,
	next: Option<Url>,
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

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct LocationRequest {
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	name: String,
	slug: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct ProviderRequest {
	accounts: Vec<i64>,
	slug: String,
	asns: Vec<i64>,
	custom_fields: String,
	description: String,
	/// Full name of the provider
	name: String,
	tags: Vec<NestedTagRequest>,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableInventoryItemRequest {
	name: String,
	role: Option<i64>,
	manufacturer: Option<i64>,
	/// Manufacturer-assigned part identifier
	part_id: String,
	serial: String,
	/// This item was automatically discovered
	discovered: bool,
	/// A unique tag used to identify this item
	asset_tag: Option<String>,
	device: i64,
	tags: Vec<NestedTagRequest>,
	description: String,
	component_type: Option<String>,
	component_id: Option<i64>,
	/// Physical label
	label: String,
	parent: Option<i64>,
	custom_fields: String,
}

pub struct PaginatedRackRoleList {
	count: i64,
	previous: Option<Url>,
	results: Vec<RackRole>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct TunnelGroupRequest {
	name: String,
	description: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRF {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	url: Url,
	id: i64,
	display: String,
	name: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatform {
	url: Url,
	slug: String,
	display: String,
	id: i64,
	name: String,
}

pub struct ContentType {
	display: String,
	id: i64,
	url: Url,
	app_label: String,
	model: String,
}

/// Adds support for custom fields and tags.
pub struct Provider {
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	asns: Vec<i64>,
	/// Full name of the provider
	name: String,
	description: String,
	url: Url,
	circuit_count: i64,
	comments: String,
	display: String,
	accounts: Vec<i64>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterGroupRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritablePowerPortRequest {
	tags: Vec<NestedTagRequest>,
	device: i64,
	custom_fields: String,
	name: String,
	module: Option<i64>,
	description: String,
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
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
}

pub struct PaginatedCableTerminationList {
	results: Vec<CableTermination>,
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
}

pub struct PaginatedRoleList {
	next: Option<Url>,
	count: i64,
	results: Vec<Role>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderAccountRequest {
	tags: Vec<NestedTagRequest>,
	account: String,
	name: String,
	provider: i64,
	custom_fields: String,
	description: String,
	comments: String,
}

pub struct PaginatedPowerPortTemplateList {
	previous: Option<Url>,
	count: i64,
	next: Option<Url>,
	results: Vec<PowerPortTemplate>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRackRequest {
	name: String,
}

pub struct PaginatedImageAttachmentList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<ImageAttachment>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedIPSecProfileRequest {
	name: String,
}

pub struct PaginatedVirtualDeviceContextList {
	count: i64,
	next: Option<Url>,
	previous: Option<Url>,
	results: Vec<VirtualDeviceContext>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableIKEPolicyRequest {
	custom_fields: String,
	name: String,
	description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	proposals: Vec<i64>,
	preshared_key: String,
	tags: Vec<NestedTagRequest>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct PowerOutletRequest {
	name: String,
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
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	/// * `A` - A
	/// * `B` - B
	/// * `C` - C
	feed_leg: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct Prefix {
	custom_fields: String,
	_depth: i64,
	last_updated: Option<String>,
	description: String,
	id: i64,
	display: String,
	prefix: String,
	/// All IP addresses within this prefix are considered usable
	is_pool: bool,
	comments: String,
	tags: Vec<NestedTag>,
	url: Url,
	status: String,
	created: Option<String>,
	/// Treat as 100% utilized
	mark_utilized: bool,
	family: String,
	children: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVirtualDiskRequest {
	virtual_machine: i64,
	tags: Vec<NestedTagRequest>,
	name: String,
	size: i64,
	description: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPowerPort {
	url: Url,
	name: String,
	_occupied: bool,
	cable: Option<i64>,
	id: i64,
	display: String,
}

pub struct PaginatedTunnelGroupList {
	next: Option<Url>,
	count: i64,
	results: Vec<TunnelGroup>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct PowerOutlet {
	name: String,
	/// Physical label
	label: String,
	custom_fields: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	feed_leg: Option<String>,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	_occupied: bool,
	url: Url,
	/// Treat as if a cable is connected
	mark_connected: bool,
	connected_endpoints: Vec<String>,
	id: i64,
	display: String,
	r#type: Option<String>,
	connected_endpoints_reachable: bool,
	cable_end: String,
	link_peers: Vec<String>,
	connected_endpoints_type: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
pub struct RackRole {
	created: Option<String>,
	url: Url,
	tags: Vec<NestedTag>,
	slug: String,
	rack_count: i64,
	description: String,
	color: String,
	id: i64,
	custom_fields: String,
	name: String,
	last_updated: Option<String>,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct IKEPolicyRequest {
	name: String,
	preshared_key: String,
	tags: Vec<NestedTagRequest>,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	custom_fields: String,
	description: String,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	comments: String,
	proposals: Vec<i64>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRoleRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedInventoryItemRoleRequest {
	name: String,
	slug: String,
	description: String,
	color: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvision {
	key: String,
	id: i64,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	expires: Option<String>,
	display: String,
	last_used: String,
	url: Url,
	created: String,
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
pub struct NestedSiteGroupRequest {
	name: String,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedUser {
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
	id: i64,
	display: String,
	url: Url,
}

pub struct PaginatedLocationList {
	count: i64,
	next: Option<Url>,
	results: Vec<Location>,
	previous: Option<Url>,
}

pub struct PaginatedRegionList {
	next: Option<Url>,
	results: Vec<Region>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldRequest {
	/// Internal field name
	name: String,
	object_type: String,
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	content_types: Vec<String>,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	description: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	choice_set: Option<i64>,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct SavedFilterRequest {
	enabled: bool,
	name: String,
	content_types: Vec<String>,
	description: String,
	slug: String,
	user: Option<i64>,
	shared: bool,
	weight: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct TokenProvisionRequest {
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	username: String,
	password: String,
	description: String,
	expires: Option<String>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderAccount {
	account: String,
	url: Url,
	id: i64,
	name: String,
	display: String,
}

/// Minimal representation of some generic object identified by ContentType and PK.
pub struct GenericObject {
	object_type: String,
	object_id: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct UserRequest {
	date_joined: String,
	groups: Vec<i64>,
	last_name: String,
	password: String,
	email: String,
	/// Designates whether the user can log into this admin site.
	is_staff: bool,
	/// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
	is_active: bool,
	first_name: String,
	/// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
	username: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct InterfaceTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	enabled: bool,
	description: String,
	id: i64,
	/// Physical label
	label: String,
	url: Url,
	rf_role: Option<String>,
	display: String,
	r#type: String,
	created: Option<String>,
	poe_type: Option<String>,
	mgmt_only: bool,
	poe_mode: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableCircuitTerminationRequest {
	/// Patch panel ID and port number(s)
	pp_info: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	/// ID of the local cross-connect
	xconnect_id: String,
	provider_network: Option<i64>,
	/// Physical circuit speed
	port_speed: Option<i64>,
	/// Upstream speed, if different from port speed
	upstream_speed: Option<i64>,
	description: String,
	custom_fields: String,
	circuit: i64,
	/// * `A` - A
	/// * `Z` - Z
	term_side: String,
	tags: Vec<NestedTagRequest>,
	site: Option<i64>,
}

/// Adds support for custom fields and tags.
pub struct CircuitType {
	description: String,
	url: Url,
	color: String,
	created: Option<String>,
	custom_fields: String,
	name: String,
	display: String,
	last_updated: Option<String>,
	id: i64,
	tags: Vec<NestedTag>,
	circuit_count: i64,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct ContactRole {
	custom_fields: String,
	url: Url,
	name: String,
	slug: String,
	id: i64,
	tags: Vec<NestedTag>,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCustomFieldChoiceSetRequest {
	name: String,
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
	device: i64,
	name: String,
	custom_fields: String,
	module: Option<i64>,
	/// Treat as if a cable is connected
	mark_connected: bool,
	description: String,
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
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableDeviceBayRequest {
	device: i64,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	/// Physical label
	label: String,
	description: String,
	installed_device: Option<i64>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLANGroup {
	slug: String,
	_depth: i64,
	url: Url,
	id: i64,
	display: String,
	name: String,
}

pub struct PaginatedL2VPNTerminationList {
	next: Option<Url>,
	results: Vec<L2VPNTermination>,
	previous: Option<Url>,
	count: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSetRequest {
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
	/// * `IATA` - IATA (Airport codes)
	/// * `ISO_3166` - ISO 3166 (Country codes)
	/// * `UN_LOCODE` - UN/LOCODE (Location codes)
	base_choices: String,
	name: String,
	description: String,
	extra_choices: Option<Vec<Vec<String>>>,
}

/// Adds support for custom fields and tags.
pub struct VLANRequest {
	name: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `deprecated` - Deprecated
	status: String,
	/// Numeric VLAN ID (1-4094)
	vid: i64,
	custom_fields: String,
	comments: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableFrontPortTemplateRequest {
	description: String,
	rear_port_position: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	/// Physical label
	label: String,
	rear_port: i64,
	device_type: Option<i64>,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProviderRequest {
	/// Full name of the provider
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct WritableModuleBayRequest {
	/// Identifier to reference when renaming installed components
	position: String,
	custom_fields: String,
	installed_module: i64,
	/// Physical label
	label: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	device: i64,
}

pub struct PaginatedInventoryItemTemplateList {
	results: Vec<InventoryItemTemplate>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

pub struct PaginatedJobList {
	previous: Option<Url>,
	count: i64,
	results: Vec<Job>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenant {
	slug: String,
	display: String,
	url: Url,
	id: i64,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableClusterRequest {
	comments: String,
	description: String,
	name: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `offline` - Offline
	status: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	site: Option<i64>,
	tenant: Option<i64>,
	group: Option<i64>,
	r#type: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedProvider {
	display: String,
	slug: String,
	id: i64,
	/// Full name of the provider
	name: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedContactRoleRequest {
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	name: String,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupRequest {
	comments: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	auth_key: String,
	group_id: i64,
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
	name: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedDeviceRequest {
	name: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleTypeRequest {
	description: String,
	model: String,
	manufacturer: i64,
	/// * `kg` - Kilograms
	/// * `g` - Grams
	/// * `lb` - Pounds
	/// * `oz` - Ounces
	weight_unit: String,
	tags: Vec<NestedTagRequest>,
	weight: Option<f64>,
	/// Discrete part number (optional)
	part_number: String,
	custom_fields: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct Platform {
	tags: Vec<NestedTag>,
	url: Url,
	display: String,
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	device_count: i64,
	virtualmachine_count: i64,
	id: i64,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedTenantGroup {
	url: Url,
	name: String,
	id: i64,
	display: String,
	slug: String,
	_depth: i64,
}

/// Adds support for custom fields and tags.
pub struct PowerPanel {
	tags: Vec<NestedTag>,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	comments: String,
	description: String,
	id: i64,
	url: Url,
	custom_fields: String,
	name: String,
	powerfeed_count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableSiteRequest {
	/// Full name of the site
	name: String,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	/// Physical location of the building
	physical_address: String,
	group: Option<i64>,
	tenant: Option<i64>,
	/// If different from the physical address
	shipping_address: String,
	comments: String,
	/// Local facility ID or description
	facility: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	tags: Vec<NestedTagRequest>,
	region: Option<i64>,
	custom_fields: String,
	slug: String,
	time_zone: Option<String>,
	asns: Vec<i64>,
}

/// Adds support for custom fields and tags.
pub struct WritableRackReservationRequest {
	tenant: Option<i64>,
	description: String,
	units: Vec<i64>,
	user: i64,
	custom_fields: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	rack: i64,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableDeviceBayTemplateRequest {
	/// Physical label
	label: String,
	device_type: i64,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableRearPortTemplateRequest {
	device_type: Option<i64>,
	color: String,
	/// Physical label
	label: String,
	module_type: Option<i64>,
	positions: i64,
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
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct ModuleNestedModuleBay {
	id: i64,
	url: Url,
	name: String,
	display: String,
}

/// Adds support for custom fields and tags.
pub struct DeviceType {
	subdevice_role: Option<String>,
	module_bay_template_count: i64,
	inventory_item_template_count: i64,
	tags: Vec<NestedTag>,
	power_port_template_count: i64,
	device_bay_template_count: i64,
	device_count: i64,
	model: String,
	/// Device consumes both front and rear rack faces.
	is_full_depth: bool,
	slug: String,
	comments: String,
	airflow: Option<String>,
	last_updated: Option<String>,
	front_image: Url,
	id: i64,
	/// Devices of this type are excluded when calculating rack utilization.
	exclude_from_utilization: bool,
	rear_port_template_count: i64,
	power_outlet_template_count: i64,
	u_height: f64,
	created: Option<String>,
	/// Discrete part number (optional)
	part_number: String,
	url: Url,
	weight_unit: Option<String>,
	description: String,
	console_server_port_template_count: i64,
	rear_image: Url,
	weight: Option<f64>,
	interface_template_count: i64,
	front_port_template_count: i64,
	custom_fields: String,
	display: String,
	console_port_template_count: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableWirelessLANRequest {
	custom_fields: String,
	ssid: String,
	tenant: Option<i64>,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	description: String,
	vlan: Option<i64>,
	auth_psk: String,
	comments: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	group: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct Token {
	display: String,
	key: String,
	expires: Option<String>,
	url: Url,
	last_used: Option<String>,
	id: i64,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	description: String,
	created: String,
}

/// Adds support for custom fields and tags.
pub struct WritableVirtualChassisRequest {
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	domain: String,
	description: String,
	master: Option<i64>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct CustomFieldChoiceSet {
	name: String,
	id: i64,
	extra_choices: Option<Vec<Vec<String>>>,
	display: String,
	last_updated: Option<String>,
	created: Option<String>,
	choices_count: String,
	base_choices: String,
	url: Url,
	description: String,
	/// Choices are automatically ordered alphabetically
	order_alphabetically: bool,
}

/// Adds support for custom fields and tags.
pub struct PatchedFHRPGroupRequest {
	group_id: i64,
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
	comments: String,
	auth_key: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct WritableRegionRequest {
	description: String,
	name: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	parent: Option<i64>,
	slug: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedCableRequest {
	label: String,
}

/// Adds support for custom fields and tags.
pub struct ClusterTypeRequest {
	name: String,
	custom_fields: String,
	description: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWebhookRequest {
	/// The complete list of official content types is available <a href="https://www.iana.org/assignments/media-types/media-types.xhtml">here</a>.
	http_content_type: String,
	name: String,
	/// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
	additional_headers: String,
	/// * `GET` - GET
	/// * `POST` - POST
	/// * `PUT` - PUT
	/// * `PATCH` - PATCH
	/// * `DELETE` - DELETE
	http_method: String,
	/// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
	body_template: String,
	/// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
	secret: String,
	description: String,
	/// Enable SSL certificate verification. Disable with caution!
	ssl_verification: bool,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	/// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
	payload_url: String,
	/// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
	ca_file_path: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTunnelTerminationRequest {
	tunnel: i64,
	termination_type: String,
	tags: Vec<NestedTagRequest>,
	/// * `peer` - Peer
	/// * `hub` - Hub
	/// * `spoke` - Spoke
	role: String,
	termination_id: Option<i64>,
	outside_ip: Option<i64>,
	custom_fields: String,
}

/// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment
/// on create() and update().
pub struct ConfigTemplateRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Jinja2 template code.
	template_code: String,
}

/// Adds support for custom fields and tags.
pub struct ConsoleServerPort {
	/// Treat as if a cable is connected
	mark_connected: bool,
	r#type: String,
	description: String,
	_occupied: bool,
	url: Url,
	connected_endpoints: Vec<String>,
	speed: Option<String>,
	name: String,
	cable_end: String,
	id: i64,
	display: String,
	/// Return the type of the peer link terminations, or None.
	link_peers_type: String,
	connected_endpoints_reachable: bool,
	last_updated: Option<String>,
	connected_endpoints_type: String,
	/// Physical label
	label: String,
	link_peers: Vec<String>,
	tags: Vec<NestedTag>,
	created: Option<String>,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedContact {
	name: String,
	id: i64,
	display: String,
	url: Url,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableModuleBayRequest {
	installed_module: i64,
	device: i64,
	name: String,
	/// Identifier to reference when renaming installed components
	position: String,
	/// Physical label
	label: String,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct VirtualMachineWithConfigContext {
	last_updated: Option<String>,
	vcpus: Option<f64>,
	name: String,
	display: String,
	disk: Option<i64>,
	description: String,
	tags: Vec<NestedTag>,
	created: Option<String>,
	interface_count: i64,
	url: Url,
	virtual_disk_count: i64,
	custom_fields: String,
	status: String,
	comments: String,
	memory: Option<i64>,
	id: i64,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableTenantRequest {
	tags: Vec<NestedTagRequest>,
	description: String,
	name: String,
	group: Option<i64>,
	slug: String,
	comments: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct L2VPN {
	description: String,
	comments: String,
	url: Url,
	id: i64,
	last_updated: Option<String>,
	created: Option<String>,
	r#type: String,
	display: String,
	identifier: Option<i64>,
	tags: Vec<NestedTag>,
	name: String,
	slug: String,
	export_targets: Vec<i64>,
	import_targets: Vec<i64>,
	custom_fields: String,
}

pub struct PaginatedPowerOutletTemplateList {
	count: i64,
	results: Vec<PowerOutletTemplate>,
	previous: Option<Url>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct WritableContactRequest {
	address: String,
	comments: String,
	link: Url,
	title: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	custom_fields: String,
	email: String,
	group: Option<i64>,
	name: String,
	phone: String,
}

pub struct PaginatedASNRangeList {
	count: i64,
	previous: Option<Url>,
	results: Vec<ASNRange>,
	next: Option<Url>,
}

pub struct PaginatedIPSecProposalList {
	count: i64,
	next: Option<Url>,
	results: Vec<IPSecProposal>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct VRF {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	export_targets: Vec<i64>,
	ipaddress_count: i64,
	tags: Vec<NestedTag>,
	description: String,
	id: i64,
	custom_fields: String,
	url: Url,
	name: String,
	comments: String,
	created: Option<String>,
	last_updated: Option<String>,
	display: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	import_targets: Vec<i64>,
	prefix_count: i64,
}

pub struct PaginatedFHRPGroupAssignmentList {
	count: i64,
	previous: Option<Url>,
	next: Option<Url>,
	results: Vec<FHRPGroupAssignment>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableInterfaceTemplateRequest {
	bridge: Option<i64>,
	/// * `ap` - Access point
	/// * `station` - Station
	rf_role: String,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	mgmt_only: bool,
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
	enabled: bool,
	description: String,
	/// * `pd` - PD
	/// * `pse` - PSE
	poe_mode: String,
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
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableTokenRequest {
	description: String,
	user: i64,
	expires: Option<String>,
	/// Permit create/update/delete operations using this key
	write_enabled: bool,
	last_used: Option<String>,
	key: String,
}

pub struct PaginatedIPRangeList {
	count: i64,
	previous: Option<Url>,
	results: Vec<IPRange>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct DeviceBay {
	custom_fields: String,
	created: Option<String>,
	last_updated: Option<String>,
	id: i64,
	display: String,
	/// Physical label
	label: String,
	tags: Vec<NestedTag>,
	name: String,
	url: Url,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PowerPortTemplate {
	display: String,
	id: i64,
	url: Url,
	/// Maximum power draw (watts)
	maximum_draw: Option<i64>,
	description: String,
	last_updated: Option<String>,
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	created: Option<String>,
	/// Allocated power draw (watts)
	allocated_draw: Option<i64>,
	r#type: Option<String>,
	/// Physical label
	label: String,
}

/// Adds support for custom fields and tags.
pub struct Tunnel {
	id: i64,
	tunnel_id: Option<i64>,
	comments: String,
	description: String,
	last_updated: Option<String>,
	encapsulation: String,
	url: Url,
	created: Option<String>,
	custom_fields: String,
	name: String,
	display: String,
	status: String,
	tags: Vec<NestedTag>,
}

/// Adds support for custom fields and tags.
pub struct WirelessLinkRequest {
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	ssid: String,
	description: String,
	custom_fields: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	auth_psk: String,
}

/// Adds support for custom fields and tags.
pub struct WritableJournalEntryRequest {
	custom_fields: String,
	assigned_object_type: String,
	assigned_object_id: i64,
	comments: String,
	created_by: Option<i64>,
	tags: Vec<NestedTagRequest>,
	/// * `info` - Info
	/// * `success` - Success
	/// * `warning` - Warning
	/// * `danger` - Danger
	kind: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedInventoryItemRole {
	name: String,
	slug: String,
	url: Url,
	id: i64,
	display: String,
}

/// Adds support for custom fields and tags.
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
	r#type: String,
	/// * `km` - Kilometers
	/// * `m` - Meters
	/// * `cm` - Centimeters
	/// * `mi` - Miles
	/// * `ft` - Feet
	/// * `in` - Inches
	length_unit: String,
	b_terminations: Vec<GenericObjectRequest>,
	tenant: Option<i64>,
	a_terminations: Vec<GenericObjectRequest>,
	label: String,
	color: String,
	length: Option<f64>,
	description: String,
	comments: String,
	/// * `connected` - Connected
	/// * `planned` - Planned
	/// * `decommissioning` - Decommissioning
	status: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct RackReservationRequest {
	tags: Vec<NestedTagRequest>,
	units: Vec<i64>,
	description: String,
	comments: String,
	custom_fields: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct SiteGroup {
	slug: String,
	tags: Vec<NestedTag>,
	custom_fields: String,
	name: String,
	_depth: i64,
	id: i64,
	url: Url,
	site_count: i64,
	display: String,
	description: String,
	created: Option<String>,
	last_updated: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct WritableVRFRequest {
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
	export_targets: Vec<i64>,
	import_targets: Vec<i64>,
	description: String,
	name: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	tenant: Option<i64>,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	comments: String,
}

pub struct Dashboard {
}

/// Adds support for custom fields and tags.
pub struct ClusterGroupRequest {
	description: String,
	name: String,
	slug: String,
	tags: Vec<NestedTagRequest>,
	custom_fields: String,
}

pub struct PaginatedAggregateList {
	next: Option<Url>,
	previous: Option<Url>,
	count: i64,
	results: Vec<Aggregate>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct ConsoleServerPortTemplate {
	/// {module} is accepted as a substitution for the module bay position when attached to a module type.
	name: String,
	last_updated: Option<String>,
	display: String,
	url: Url,
	description: String,
	/// Physical label
	label: String,
	created: Option<String>,
	r#type: String,
	id: i64,
}

pub struct PaginatedSavedFilterList {
	count: i64,
	previous: Option<Url>,
	results: Vec<SavedFilter>,
	next: Option<Url>,
}

pub struct PaginatedServiceList {
	count: i64,
	previous: Option<Url>,
	results: Vec<Service>,
	next: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct ASNRangeRequest {
	end: i64,
	slug: String,
	name: String,
	start: i64,
	description: String,
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
}

/// Adds support for custom fields and tags.
pub struct RIR {
	url: Url,
	display: String,
	/// IP space managed by this RIR is considered private
	is_private: bool,
	description: String,
	id: i64,
	tags: Vec<NestedTag>,
	name: String,
	slug: String,
	created: Option<String>,
	last_updated: Option<String>,
	aggregate_count: i64,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableProviderRequest {
	asns: Vec<i64>,
	description: String,
	custom_fields: String,
	accounts: Vec<i64>,
	/// Full name of the provider
	name: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
}

/// Adds support for custom fields and tags.
pub struct RouteTarget {
	display: String,
	comments: String,
	tags: Vec<NestedTag>,
	url: Url,
	created: Option<String>,
	last_updated: Option<String>,
	custom_fields: String,
	id: i64,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedClusterRequest {
	name: String,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct TenantGroupRequest {
	tags: Vec<NestedTagRequest>,
	name: String,
	description: String,
	slug: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedVRFRequest {
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	name: String,
}

pub struct PaginatedTunnelTerminationList {
	count: i64,
	previous: Option<Url>,
	results: Vec<TunnelTermination>,
	next: Option<Url>,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedRack {
	display: String,
	id: i64,
	name: String,
	url: Url,
}

pub struct PaginatedFHRPGroupList {
	previous: Option<Url>,
	next: Option<Url>,
	count: i64,
	results: Vec<FHRPGroup>,
}

/// Adds support for custom fields and tags.
pub struct VirtualChassis {
	member_count: i64,
	url: Url,
	domain: String,
	comments: String,
	name: String,
	display: String,
	custom_fields: String,
	last_updated: Option<String>,
	description: String,
	id: i64,
	tags: Vec<NestedTag>,
	created: Option<String>,
}

/// Representation of an ASN which does not exist in the database.
pub struct AvailableASN {
	description: String,
	asn: i64,
}

/// Adds support for custom fields and tags.
pub struct WritableEventRuleRequest {
	custom_fields: String,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// Triggers when a matching object is deleted.
	type_delete: bool,
	/// Triggers when a job for a matching object terminates.
	type_job_end: bool,
	/// Triggers when a matching object is updated.
	type_update: bool,
	enabled: bool,
	/// Triggers when a matching object is created.
	type_create: bool,
	action_object_type: String,
	content_types: Vec<String>,
	/// * `webhook` - Webhook
	/// * `script` - Script
	action_type: String,
	action_object_id: Option<i64>,
	/// Triggers when a job for a matching object is started.
	type_job_start: bool,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIKEPolicyRequest {
	name: String,
	description: String,
	/// * `1` - IKEv1
	/// * `2` - IKEv2
	version: i64,
	proposals: Vec<i64>,
	/// * `aggressive` - Aggressive
	/// * `main` - Main
	mode: String,
	comments: String,
	tags: Vec<NestedTagRequest>,
	preshared_key: String,
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableAggregateRequest {
	tags: Vec<NestedTagRequest>,
	comments: String,
	custom_fields: String,
	/// Regional Internet Registry responsible for this IP space
	rir: i64,
	description: String,
	prefix: String,
	tenant: Option<i64>,
	date_added: Option<String>,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableRearPortRequest {
	device: i64,
	/// Number of front ports which may be mapped
	positions: i64,
	description: String,
	/// Treat as if a cable is connected
	mark_connected: bool,
	tags: Vec<NestedTagRequest>,
	color: String,
	module: Option<i64>,
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
	custom_fields: String,
}

/// Adds support for custom fields and tags.
pub struct FHRPGroupAssignmentRequest {
	priority: i64,
	interface_id: i64,
	interface_type: String,
}

/// Adds support for custom fields and tags.
pub struct PatchedWritableVRFRequest {
	description: String,
	/// Unique route distinguisher (as defined in RFC 4364)
	rd: Option<String>,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	export_targets: Vec<i64>,
	comments: String,
	/// Prevent duplicate prefixes/IP addresses within this VRF
	enforce_unique: bool,
	custom_fields: String,
	import_targets: Vec<i64>,
	name: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedWirelessLink {
	url: Url,
	id: i64,
	ssid: String,
	display: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct WritableCustomFieldRequest {
	content_types: Vec<String>,
	/// Replicate this value when cloning objects
	is_cloneable: bool,
	description: String,
	/// Minimum allowed value (for numeric fields)
	validation_minimum: Option<i64>,
	choice_set: Option<i64>,
	object_type: String,
	/// Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored.
	search_weight: i64,
	/// Specifies whether the custom field value can be edited in the UI
	/// 
	/// * `yes` - Yes
	/// * `no` - No
	/// * `hidden` - Hidden
	ui_editable: String,
	/// Fields with higher weights appear lower in a form.
	weight: i64,
	/// Internal field name
	name: String,
	/// Custom fields within the same group will be displayed together
	group_name: String,
	/// Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters.
	validation_regex: String,
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
	/// Maximum allowed value (for numeric fields)
	validation_maximum: Option<i64>,
	/// Loose matches any instance of a given string; exact matches the entire field.
	/// 
	/// * `disabled` - Disabled
	/// * `loose` - Loose
	/// * `exact` - Exact
	filter_logic: String,
	/// Name of the field as displayed to users (if not provided, 'the field's name will be used)
	label: String,
	/// If true, this field is required when creating new objects or editing an existing object.
	required: bool,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedPlatformRequest {
	slug: String,
	name: String,
}

/// Adds support for custom fields and tags.
pub struct PlatformRequest {
	name: String,
	tags: Vec<NestedTagRequest>,
	slug: String,
	description: String,
	custom_fields: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedSiteRequest {
	slug: String,
	/// Full name of the site
	name: String,
}

pub struct PaginatedServiceTemplateList {
	next: Option<Url>,
	count: i64,
	results: Vec<ServiceTemplate>,
	previous: Option<Url>,
}

/// Adds support for custom fields and tags.
pub struct RouteTargetRequest {
	tags: Vec<NestedTagRequest>,
	/// Route target value (formatted in accordance with RFC 4360)
	name: String,
	description: String,
	custom_fields: String,
	comments: String,
}

/// Adds support for custom fields and tags.
pub struct WritableIPSecProfileRequest {
	name: String,
	description: String,
	ipsec_policy: i64,
	comments: String,
	tags: Vec<NestedTagRequest>,
	ike_policy: i64,
	custom_fields: String,
	/// * `esp` - ESP
	/// * `ah` - AH
	mode: String,
}

/// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a
/// dictionary of attributes which can be used to uniquely identify the related object. This class should be
/// subclassed to return a full representation of the related object on read.
pub struct NestedModule {
	display: String,
	id: i64,
	url: Url,
}

/// Extends PrimaryModelSerializer to include MPTT support.
pub struct PatchedWritableLocationRequest {
	slug: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
	description: String,
	/// * `planned` - Planned
	/// * `staging` - Staging
	/// * `active` - Active
	/// * `decommissioning` - Decommissioning
	/// * `retired` - Retired
	status: String,
	name: String,
	site: i64,
	custom_fields: String,
	parent: Option<i64>,
}

pub struct PaginatedManufacturerList {
	results: Vec<Manufacturer>,
	next: Option<Url>,
	count: i64,
	previous: Option<Url>,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableCustomFieldChoiceSetRequest {
	description: String,
	/// Base set of predefined choices (optional)
	/// 
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
pub struct PatchedWritableWirelessLANRequest {
	custom_fields: String,
	/// * `open` - Open
	/// * `wep` - WEP
	/// * `wpa-personal` - WPA Personal (PSK)
	/// * `wpa-enterprise` - WPA Enterprise
	auth_type: String,
	ssid: String,
	description: String,
	vlan: Option<i64>,
	auth_psk: String,
	comments: String,
	/// * `auto` - Auto
	/// * `tkip` - TKIP
	/// * `aes` - AES
	auth_cipher: String,
	group: Option<i64>,
	/// * `active` - Active
	/// * `reserved` - Reserved
	/// * `disabled` - Disabled
	/// * `deprecated` - Deprecated
	status: String,
	tenant: Option<i64>,
	tags: Vec<NestedTagRequest>,
}

pub struct DashboardRequest {
}

/// Adds support for custom fields and tags.
pub struct DeviceWithConfigContext {
	position: Option<f64>,
	comments: String,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	longitude: Option<f64>,
	power_outlet_count: i64,
	name: Option<String>,
	last_updated: Option<String>,
	created: Option<String>,
	/// A unique tag used to identify this device
	asset_tag: Option<String>,
	device_bay_count: i64,
	interface_count: i64,
	inventory_item_count: i64,
	console_port_count: i64,
	front_port_count: i64,
	rear_port_count: i64,
	airflow: String,
	/// Virtual chassis master election priority
	vc_priority: Option<i64>,
	custom_fields: String,
	url: Url,
	/// GPS coordinate in decimal format (xx.yyyyyy)
	latitude: Option<f64>,
	face: String,
	vc_position: Option<i64>,
	description: String,
	tags: Vec<NestedTag>,
	display: String,
	console_server_port_count: i64,
	power_port_count: i64,
	module_bay_count: i64,
	status: String,
	id: i64,
	/// Chassis serial number, assigned by the manufacturer
	serial: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedSavedFilterRequest {
	slug: String,
	content_types: Vec<String>,
	enabled: bool,
	weight: i64,
	user: Option<i64>,
	shared: bool,
	name: String,
	description: String,
}

/// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during
/// validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
pub struct PatchedWritableExportTemplateRequest {
	/// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
	template_code: String,
	description: String,
	/// Remote data source
	data_source: Option<i64>,
	content_types: Vec<String>,
	/// Extension to append to the rendered filename
	file_extension: String,
	name: String,
	/// Defaults to <code>text/plain; charset=utf-8</code>
	mime_type: String,
	/// Download file as attachment
	as_attachment: bool,
}

pub struct PaginatedRearPortList {
	next: Option<Url>,
	count: i64,
	results: Vec<RearPort>,
	previous: Option<Url>,
}

pub struct PaginatedClusterGroupList {
	previous: Option<Url>,
	count: i64,
	results: Vec<ClusterGroup>,
	next: Option<Url>,
}


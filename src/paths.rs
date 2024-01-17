use serde_qs;
use reqwest::Url;
use crate::util::{REQWEST_BASE_URL, REQWEST_CLIENT};

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsListQuery {
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	circuit_id: Vec<i64>,
	circuit_id__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	occupied: bool,
	offset: i64,
	ordering: String,
	port_speed: Vec<i32>,
	port_speed__empty: bool,
	port_speed__gt: Vec<i32>,
	port_speed__gte: Vec<i32>,
	port_speed__lt: Vec<i32>,
	port_speed__lte: Vec<i32>,
	port_speed__n: Vec<i32>,
	provider_network_id: Vec<i64>,
	provider_network_id__n: Vec<i64>,
	q: String,
	site: Vec<String>,
	site__n: Vec<String>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	term_side: String,
	term_side__n: String,
	updated_by_request: String,
	upstream_speed: Vec<i32>,
	upstream_speed__empty: bool,
	upstream_speed__gt: Vec<i32>,
	upstream_speed__gte: Vec<i32>,
	upstream_speed__lt: Vec<i32>,
	upstream_speed__lte: Vec<i32>,
	upstream_speed__n: Vec<i32>,
	xconnect_id: Vec<String>,
	xconnect_id__empty: bool,
	xconnect_id__ic: Vec<String>,
	xconnect_id__ie: Vec<String>,
	xconnect_id__iew: Vec<String>,
	xconnect_id__isw: Vec<String>,
	xconnect_id__n: Vec<String>,
	xconnect_id__nic: Vec<String>,
	xconnect_id__nie: Vec<String>,
	xconnect_id__niew: Vec<String>,
	xconnect_id__nisw: Vec<String>
}
/// Get a list of circuit termination objects.

pub fn circuits_circuit_terminations_list(query: CircuitsCircuitTerminationsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuit-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsBulkUpdateQuery {
}
/// Put a list of circuit termination objects.

pub fn circuits_circuit_terminations_bulk_update(query: CircuitsCircuitTerminationsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/circuit-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsCreateQuery {
}
/// Post a list of circuit termination objects.

pub fn circuits_circuit_terminations_create(query: CircuitsCircuitTerminationsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/circuits/circuit-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsBulkPartialUpdateQuery {
}
/// Patch a list of circuit termination objects.

pub fn circuits_circuit_terminations_bulk_partial_update(query: CircuitsCircuitTerminationsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/circuit-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsBulkDestroyQuery {
}
/// Delete a list of circuit termination objects.

pub fn circuits_circuit_terminations_bulk_destroy(query: CircuitsCircuitTerminationsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/circuit-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsListQuery {
	auth_key: Vec<String>,
	auth_key__empty: bool,
	auth_key__ic: Vec<String>,
	auth_key__ie: Vec<String>,
	auth_key__iew: Vec<String>,
	auth_key__isw: Vec<String>,
	auth_key__n: Vec<String>,
	auth_key__nic: Vec<String>,
	auth_key__nie: Vec<String>,
	auth_key__niew: Vec<String>,
	auth_key__nisw: Vec<String>,
	auth_typ: Vec<String>,
	auth_typ__n: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	group_id: Vec<i32>,
	group_id__empty: bool,
	group_id__gt: Vec<i32>,
	group_id__gte: Vec<i32>,
	group_id__lt: Vec<i32>,
	group_id__lte: Vec<i32>,
	group_id__n: Vec<i32>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	protocol: Vec<String>,
	protocol__n: Vec<String>,
	q: String,
	related_ip: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of FHRP group objects.

pub fn ipam_fhrp_groups_list(query: IpamFhrpGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/fhrp-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsBulkUpdateQuery {
}
/// Put a list of FHRP group objects.

pub fn ipam_fhrp_groups_bulk_update(query: IpamFhrpGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/fhrp-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsCreateQuery {
}
/// Post a list of FHRP group objects.

pub fn ipam_fhrp_groups_create(query: IpamFhrpGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/fhrp-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsBulkPartialUpdateQuery {
}
/// Patch a list of FHRP group objects.

pub fn ipam_fhrp_groups_bulk_partial_update(query: IpamFhrpGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/fhrp-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsBulkDestroyQuery {
}
/// Delete a list of FHRP group objects.

pub fn ipam_fhrp_groups_bulk_destroy(query: IpamFhrpGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/fhrp-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesRetrieveQuery {
}
/// Get a cluster type object.

pub fn virtualization_cluster_types_retrieve(query: VirtualizationClusterTypesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/cluster-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesUpdateQuery {
}
/// Put a cluster type object.

pub fn virtualization_cluster_types_update(query: VirtualizationClusterTypesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/cluster-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesPartialUpdateQuery {
}
/// Patch a cluster type object.

pub fn virtualization_cluster_types_partial_update(query: VirtualizationClusterTypesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/cluster-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesDestroyQuery {
}
/// Delete a cluster type object.

pub fn virtualization_cluster_types_destroy(query: VirtualizationClusterTypesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/cluster-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataFilesRetrieveQuery {
}
/// Get a data file object.

pub fn core_data_files_retrieve(query: CoreDataFilesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/core/data-files/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsRetrieveQuery {
}
/// Get a image attachment object.

pub fn extras_image_attachments_retrieve(query: ExtrasImageAttachmentsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/image-attachments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsUpdateQuery {
}
/// Put a image attachment object.

pub fn extras_image_attachments_update(query: ExtrasImageAttachmentsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/image-attachments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsPartialUpdateQuery {
}
/// Patch a image attachment object.

pub fn extras_image_attachments_partial_update(query: ExtrasImageAttachmentsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/image-attachments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsDestroyQuery {
}
/// Delete a image attachment object.

pub fn extras_image_attachments_destroy(query: ExtrasImageAttachmentsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/image-attachments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesRetrieveQuery {
}
/// Get a contact role object.

pub fn tenancy_contact_roles_retrieve(query: TenancyContactRolesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contact-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesUpdateQuery {
}
/// Put a contact role object.

pub fn tenancy_contact_roles_update(query: TenancyContactRolesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contact-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesPartialUpdateQuery {
}
/// Patch a contact role object.

pub fn tenancy_contact_roles_partial_update(query: TenancyContactRolesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contact-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesDestroyQuery {
}
/// Delete a contact role object.

pub fn tenancy_contact_roles_destroy(query: TenancyContactRolesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contact-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsRetrieveQuery {
}
/// Get a console port object.

pub fn dcim_console_ports_retrieve(query: DcimConsolePortsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsUpdateQuery {
}
/// Put a console port object.

pub fn dcim_console_ports_update(query: DcimConsolePortsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsPartialUpdateQuery {
}
/// Patch a console port object.

pub fn dcim_console_ports_partial_update(query: DcimConsolePortsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsDestroyQuery {
}
/// Delete a console port object.

pub fn dcim_console_ports_destroy(query: DcimConsolePortsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent: Vec<i64>,
	parent__n: Vec<i64>,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of location objects.

pub fn dcim_locations_list(query: DcimLocationsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/locations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsBulkUpdateQuery {
}
/// Put a list of location objects.

pub fn dcim_locations_bulk_update(query: DcimLocationsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/locations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsCreateQuery {
}
/// Post a list of location objects.

pub fn dcim_locations_create(query: DcimLocationsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/locations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsBulkPartialUpdateQuery {
}
/// Patch a list of location objects.

pub fn dcim_locations_bulk_partial_update(query: DcimLocationsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/locations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsBulkDestroyQuery {
}
/// Delete a list of location objects.

pub fn dcim_locations_bulk_destroy(query: DcimLocationsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/locations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysRetrieveQuery {
}
/// Get a module bay object.

pub fn dcim_module_bays_retrieve(query: DcimModuleBaysRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/module-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysUpdateQuery {
}
/// Put a module bay object.

pub fn dcim_module_bays_update(query: DcimModuleBaysUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/module-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysPartialUpdateQuery {
}
/// Patch a module bay object.

pub fn dcim_module_bays_partial_update(query: DcimModuleBaysPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/module-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysDestroyQuery {
}
/// Delete a module bay object.

pub fn dcim_module_bays_destroy(query: DcimModuleBaysDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/module-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of rack role objects.

pub fn dcim_rack_roles_list(query: DcimRackRolesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rack-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesBulkUpdateQuery {
}
/// Put a list of rack role objects.

pub fn dcim_rack_roles_bulk_update(query: DcimRackRolesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rack-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesCreateQuery {
}
/// Post a list of rack role objects.

pub fn dcim_rack_roles_create(query: DcimRackRolesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/rack-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesBulkPartialUpdateQuery {
}
/// Patch a list of rack role objects.

pub fn dcim_rack_roles_bulk_partial_update(query: DcimRackRolesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rack-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesBulkDestroyQuery {
}
/// Delete a list of rack role objects.

pub fn dcim_rack_roles_bulk_destroy(query: DcimRackRolesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rack-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesRetrieveQuery {
}
/// Get a aggregate object.

pub fn ipam_aggregates_retrieve(query: IpamAggregatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/aggregates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesUpdateQuery {
}
/// Put a aggregate object.

pub fn ipam_aggregates_update(query: IpamAggregatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/aggregates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesPartialUpdateQuery {
}
/// Patch a aggregate object.

pub fn ipam_aggregates_partial_update(query: IpamAggregatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/aggregates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesDestroyQuery {
}
/// Delete a aggregate object.

pub fn ipam_aggregates_destroy(query: IpamAggregatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/aggregates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesRetrieveQuery {
}
/// Get a ASN range object.

pub fn ipam_asn_ranges_retrieve(query: IpamAsnRangesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/asn-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesUpdateQuery {
}
/// Put a ASN range object.

pub fn ipam_asn_ranges_update(query: IpamAsnRangesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/asn-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesPartialUpdateQuery {
}
/// Patch a ASN range object.

pub fn ipam_asn_ranges_partial_update(query: IpamAsnRangesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/asn-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesDestroyQuery {
}
/// Delete a ASN range object.

pub fn ipam_asn_ranges_destroy(query: IpamAsnRangesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/asn-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsRetrieveQuery {
}
/// Get a IKE proposal object.

pub fn vpn_ike_proposals_retrieve(query: VpnIkeProposalsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ike-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsUpdateQuery {
}
/// Put a IKE proposal object.

pub fn vpn_ike_proposals_update(query: VpnIkeProposalsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ike-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsPartialUpdateQuery {
}
/// Patch a IKE proposal object.

pub fn vpn_ike_proposals_partial_update(query: VpnIkeProposalsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ike-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsDestroyQuery {
}
/// Delete a IKE proposal object.

pub fn vpn_ike_proposals_destroy(query: VpnIkeProposalsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ike-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersListQuery {
	asn_id: Vec<i64>,
	asn_id__n: Vec<i64>,
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of provider objects.

pub fn circuits_providers_list(query: CircuitsProvidersListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/providers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersBulkUpdateQuery {
}
/// Put a list of provider objects.

pub fn circuits_providers_bulk_update(query: CircuitsProvidersBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/providers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersCreateQuery {
}
/// Post a list of provider objects.

pub fn circuits_providers_create(query: CircuitsProvidersCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/circuits/providers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersBulkPartialUpdateQuery {
}
/// Patch a list of provider objects.

pub fn circuits_providers_bulk_partial_update(query: CircuitsProvidersBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/providers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersBulkDestroyQuery {
}
/// Delete a list of provider objects.

pub fn circuits_providers_bulk_destroy(query: CircuitsProvidersBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/providers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	is_private: bool,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of RIR objects.

pub fn ipam_rirs_list(query: IpamRirsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/rirs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsBulkUpdateQuery {
}
/// Put a list of RIR objects.

pub fn ipam_rirs_bulk_update(query: IpamRirsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/rirs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsCreateQuery {
}
/// Post a list of RIR objects.

pub fn ipam_rirs_create(query: IpamRirsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/rirs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsBulkPartialUpdateQuery {
}
/// Patch a list of RIR objects.

pub fn ipam_rirs_bulk_partial_update(query: IpamRirsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/rirs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsBulkDestroyQuery {
}
/// Delete a list of RIR objects.

pub fn ipam_rirs_bulk_destroy(query: IpamRirsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/rirs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsListQuery {
	cluster_group: Vec<String>,
	cluster_group__n: Vec<String>,
	cluster_group_id: Vec<i64>,
	cluster_group_id__n: Vec<i64>,
	cluster_id: Vec<i64>,
	cluster_id__n: Vec<i64>,
	cluster_typ: Vec<String>,
	cluster_typ__n: Vec<String>,
	cluster_typ_id: Vec<i64>,
	cluster_typ_id__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	data_file_id: Vec<i64>,
	data_file_id__n: Vec<i64>,
	data_source_id: Vec<i64>,
	data_source_id__n: Vec<i64>,
	data_synced: Vec<String>,
	data_synced__empty: bool,
	data_synced__gt: Vec<String>,
	data_synced__gte: Vec<String>,
	data_synced__lt: Vec<String>,
	data_synced__lte: Vec<String>,
	data_synced__n: Vec<String>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	is_active: bool,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	platform: Vec<String>,
	platform__n: Vec<String>,
	platform_id: Vec<i64>,
	platform_id__n: Vec<i64>,
	q: String,
	region: Vec<String>,
	region__n: Vec<String>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<String>,
	site_group__n: Vec<String>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tag_id: Vec<i64>,
	tag_id__n: Vec<i64>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<String>,
	tenant_group__n: Vec<String>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of config context objects.

pub fn extras_config_contexts_list(query: ExtrasConfigContextsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/config-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsBulkUpdateQuery {
}
/// Put a list of config context objects.

pub fn extras_config_contexts_bulk_update(query: ExtrasConfigContextsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/config-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsCreateQuery {
}
/// Post a list of config context objects.

pub fn extras_config_contexts_create(query: ExtrasConfigContextsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/config-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsBulkPartialUpdateQuery {
}
/// Patch a list of config context objects.

pub fn extras_config_contexts_bulk_partial_update(query: ExtrasConfigContextsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/config-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsBulkDestroyQuery {
}
/// Delete a list of config context objects.

pub fn extras_config_contexts_bulk_destroy(query: ExtrasConfigContextsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/config-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsRetrieveQuery {
}
/// Get a contact assignment object.

pub fn tenancy_contact_assignments_retrieve(query: TenancyContactAssignmentsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contact-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsUpdateQuery {
}
/// Put a contact assignment object.

pub fn tenancy_contact_assignments_update(query: TenancyContactAssignmentsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contact-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsPartialUpdateQuery {
}
/// Patch a contact assignment object.

pub fn tenancy_contact_assignments_partial_update(query: TenancyContactAssignmentsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contact-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsDestroyQuery {
}
/// Delete a contact assignment object.

pub fn tenancy_contact_assignments_destroy(query: TenancyContactAssignmentsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contact-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesRetrieveQuery {
}
/// Get a cable object.

pub fn dcim_cables_retrieve(query: DcimCablesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/cables/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesUpdateQuery {
}
/// Put a cable object.

pub fn dcim_cables_update(query: DcimCablesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/cables/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesPartialUpdateQuery {
}
/// Patch a cable object.

pub fn dcim_cables_partial_update(query: DcimCablesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/cables/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesDestroyQuery {
}
/// Delete a cable object.

pub fn dcim_cables_destroy(query: DcimCablesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/cables/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsRetrieveQuery {
}
/// Get a IPSec proposal object.

pub fn vpn_ipsec_proposals_retrieve(query: VpnIpsecProposalsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ipsec-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsUpdateQuery {
}
/// Put a IPSec proposal object.

pub fn vpn_ipsec_proposals_update(query: VpnIpsecProposalsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ipsec-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsPartialUpdateQuery {
}
/// Patch a IPSec proposal object.

pub fn vpn_ipsec_proposals_partial_update(query: VpnIpsecProposalsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ipsec-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsDestroyQuery {
}
/// Delete a IPSec proposal object.

pub fn vpn_ipsec_proposals_destroy(query: VpnIpsecProposalsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ipsec-proposals/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesListQuery {
	asset_tag: Vec<String>,
	asset_tag__empty: bool,
	asset_tag__ic: Vec<String>,
	asset_tag__ie: Vec<String>,
	asset_tag__iew: Vec<String>,
	asset_tag__isw: Vec<String>,
	asset_tag__n: Vec<String>,
	asset_tag__nic: Vec<String>,
	asset_tag__nie: Vec<String>,
	asset_tag__niew: Vec<String>,
	asset_tag__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	modified_by_request: String,
	module_bay_id: Vec<i64>,
	module_bay_id__n: Vec<i64>,
	module_typ: Vec<String>,
	module_typ__n: Vec<String>,
	module_typ_id: Vec<i64>,
	module_typ_id__n: Vec<i64>,
	offset: i64,
	ordering: String,
	q: String,
	serial: Vec<String>,
	serial__empty: bool,
	serial__ic: Vec<String>,
	serial__ie: Vec<String>,
	serial__iew: Vec<String>,
	serial__isw: Vec<String>,
	serial__n: Vec<String>,
	serial__nic: Vec<String>,
	serial__nie: Vec<String>,
	serial__niew: Vec<String>,
	serial__nisw: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of module objects.

pub fn dcim_modules_list(query: DcimModulesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/modules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesBulkUpdateQuery {
}
/// Put a list of module objects.

pub fn dcim_modules_bulk_update(query: DcimModulesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/modules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesCreateQuery {
}
/// Post a list of module objects.

pub fn dcim_modules_create(query: DcimModulesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/modules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesBulkPartialUpdateQuery {
}
/// Patch a list of module objects.

pub fn dcim_modules_bulk_partial_update(query: DcimModulesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/modules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesBulkDestroyQuery {
}
/// Delete a list of module objects.

pub fn dcim_modules_bulk_destroy(query: DcimModulesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/modules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of tunnel group objects.

pub fn vpn_tunnel_groups_list(query: VpnTunnelGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/tunnel-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsBulkUpdateQuery {
}
/// Put a list of tunnel group objects.

pub fn vpn_tunnel_groups_bulk_update(query: VpnTunnelGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/tunnel-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsCreateQuery {
}
/// Post a list of tunnel group objects.

pub fn vpn_tunnel_groups_create(query: VpnTunnelGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/tunnel-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsBulkPartialUpdateQuery {
}
/// Patch a list of tunnel group objects.

pub fn vpn_tunnel_groups_bulk_partial_update(query: VpnTunnelGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/tunnel-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsBulkDestroyQuery {
}
/// Delete a list of tunnel group objects.

pub fn vpn_tunnel_groups_bulk_destroy(query: VpnTunnelGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/tunnel-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesListQuery {
	component_id: Vec<i32>,
	component_id__empty: Vec<i32>,
	component_id__gt: Vec<i32>,
	component_id__gte: Vec<i32>,
	component_id__lt: Vec<i32>,
	component_id__lte: Vec<i32>,
	component_id__n: Vec<i32>,
	component_typ: String,
	component_typ__n: String,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	part_id: Vec<String>,
	part_id__empty: bool,
	part_id__ic: Vec<String>,
	part_id__ie: Vec<String>,
	part_id__iew: Vec<String>,
	part_id__isw: Vec<String>,
	part_id__n: Vec<String>,
	part_id__nic: Vec<String>,
	part_id__nie: Vec<String>,
	part_id__niew: Vec<String>,
	part_id__nisw: Vec<String>,
	q: String,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of inventory item template objects.

pub fn dcim_inventory_item_templates_list(query: DcimInventoryItemTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/inventory-item-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesBulkUpdateQuery {
}
/// Put a list of inventory item template objects.

pub fn dcim_inventory_item_templates_bulk_update(query: DcimInventoryItemTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/inventory-item-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesCreateQuery {
}
/// Post a list of inventory item template objects.

pub fn dcim_inventory_item_templates_create(query: DcimInventoryItemTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/inventory-item-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of inventory item template objects.

pub fn dcim_inventory_item_templates_bulk_partial_update(query: DcimInventoryItemTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/inventory-item-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesBulkDestroyQuery {
}
/// Delete a list of inventory item template objects.

pub fn dcim_inventory_item_templates_bulk_destroy(query: DcimInventoryItemTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/inventory-item-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of manufacturer objects.

pub fn dcim_manufacturers_list(query: DcimManufacturersListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/manufacturers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersBulkUpdateQuery {
}
/// Put a list of manufacturer objects.

pub fn dcim_manufacturers_bulk_update(query: DcimManufacturersBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/manufacturers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersCreateQuery {
}
/// Post a list of manufacturer objects.

pub fn dcim_manufacturers_create(query: DcimManufacturersCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/manufacturers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersBulkPartialUpdateQuery {
}
/// Patch a list of manufacturer objects.

pub fn dcim_manufacturers_bulk_partial_update(query: DcimManufacturersBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/manufacturers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersBulkDestroyQuery {
}
/// Delete a list of manufacturer objects.

pub fn dcim_manufacturers_bulk_destroy(query: DcimManufacturersBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/manufacturers/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesAvailableAsnsListQuery {
}
/// Get a ASN object.

pub fn ipam_asn_ranges_available_asns_list(query: IpamAsnRangesAvailableAsnsListQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/asn-ranges/{id}/available-asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesAvailableAsnsCreateQuery {
}
/// Post a ASN object.

pub fn ipam_asn_ranges_available_asns_create(query: IpamAsnRangesAvailableAsnsCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/asn-ranges/{id}/available-asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsListQuery {
	base_choices: String,
	base_choices__n: String,
	choice: Vec<String>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	order_alphabetically: bool,
	ordering: String,
	q: String
}
/// Get a list of custom field choice set objects.

pub fn extras_custom_field_choice_sets_list(query: ExtrasCustomFieldChoiceSetsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-field-choice-sets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsBulkUpdateQuery {
}
/// Put a list of custom field choice set objects.

pub fn extras_custom_field_choice_sets_bulk_update(query: ExtrasCustomFieldChoiceSetsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/custom-field-choice-sets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsCreateQuery {
}
/// Post a list of custom field choice set objects.

pub fn extras_custom_field_choice_sets_create(query: ExtrasCustomFieldChoiceSetsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/custom-field-choice-sets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsBulkPartialUpdateQuery {
}
/// Patch a list of custom field choice set objects.

pub fn extras_custom_field_choice_sets_bulk_partial_update(query: ExtrasCustomFieldChoiceSetsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/custom-field-choice-sets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsBulkDestroyQuery {
}
/// Delete a list of custom field choice set objects.

pub fn extras_custom_field_choice_sets_bulk_destroy(query: ExtrasCustomFieldChoiceSetsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/custom-field-choice-sets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	provider: Vec<String>,
	provider__n: Vec<String>,
	provider_id: Vec<i64>,
	provider_id__n: Vec<i64>,
	q: String,
	service_id: Vec<String>,
	service_id__empty: bool,
	service_id__ic: Vec<String>,
	service_id__ie: Vec<String>,
	service_id__iew: Vec<String>,
	service_id__isw: Vec<String>,
	service_id__n: Vec<String>,
	service_id__nic: Vec<String>,
	service_id__nie: Vec<String>,
	service_id__niew: Vec<String>,
	service_id__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of provider network objects.

pub fn circuits_provider_networks_list(query: CircuitsProviderNetworksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/provider-networks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksBulkUpdateQuery {
}
/// Put a list of provider network objects.

pub fn circuits_provider_networks_bulk_update(query: CircuitsProviderNetworksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/provider-networks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksCreateQuery {
}
/// Post a list of provider network objects.

pub fn circuits_provider_networks_create(query: CircuitsProviderNetworksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/circuits/provider-networks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksBulkPartialUpdateQuery {
}
/// Patch a list of provider network objects.

pub fn circuits_provider_networks_bulk_partial_update(query: CircuitsProviderNetworksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/provider-networks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksBulkDestroyQuery {
}
/// Delete a list of provider network objects.

pub fn circuits_provider_networks_bulk_destroy(query: CircuitsProviderNetworksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/provider-networks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesListQuery {
	console_ports: bool,
	console_server_ports: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interfaces: bool,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	model: Vec<String>,
	model__empty: bool,
	model__ic: Vec<String>,
	model__ie: Vec<String>,
	model__iew: Vec<String>,
	model__isw: Vec<String>,
	model__n: Vec<String>,
	model__nic: Vec<String>,
	model__nie: Vec<String>,
	model__niew: Vec<String>,
	model__nisw: Vec<String>,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	part_number: Vec<String>,
	part_number__empty: bool,
	part_number__ic: Vec<String>,
	part_number__ie: Vec<String>,
	part_number__iew: Vec<String>,
	part_number__isw: Vec<String>,
	part_number__n: Vec<String>,
	part_number__nic: Vec<String>,
	part_number__nie: Vec<String>,
	part_number__niew: Vec<String>,
	part_number__nisw: Vec<String>,
	pass_through_ports: bool,
	power_outlets: bool,
	power_ports: bool,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	weight: Vec<f64>,
	weight__empty: bool,
	weight__gt: Vec<f64>,
	weight__gte: Vec<f64>,
	weight__lt: Vec<f64>,
	weight__lte: Vec<f64>,
	weight__n: Vec<f64>,
	weight_unit: String,
	weight_unit__n: String
}
/// Get a list of module type objects.

pub fn dcim_module_types_list(query: DcimModuleTypesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/module-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesBulkUpdateQuery {
}
/// Put a list of module type objects.

pub fn dcim_module_types_bulk_update(query: DcimModuleTypesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/module-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesCreateQuery {
}
/// Post a list of module type objects.

pub fn dcim_module_types_create(query: DcimModuleTypesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/module-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesBulkPartialUpdateQuery {
}
/// Patch a list of module type objects.

pub fn dcim_module_types_bulk_partial_update(query: DcimModuleTypesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/module-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesBulkDestroyQuery {
}
/// Delete a list of module type objects.

pub fn dcim_module_types_bulk_destroy(query: DcimModuleTypesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/module-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesListQuery {
	content_typ_id: Vec<i32>,
	content_typ_id__empty: Vec<i32>,
	content_typ_id__gt: Vec<i32>,
	content_typ_id__gte: Vec<i32>,
	content_typ_id__lt: Vec<i32>,
	content_typ_id__lte: Vec<i32>,
	content_typ_id__n: Vec<i32>,
	content_typs: String,
	content_typs__ic: String,
	content_typs__ie: String,
	content_typs__iew: String,
	content_typs__isw: String,
	content_typs__n: String,
	content_typs__nic: String,
	content_typs__nie: String,
	content_typs__niew: String,
	content_typs__nisw: String,
	data_file_id: Vec<i64>,
	data_file_id__n: Vec<i64>,
	data_source_id: Vec<i64>,
	data_source_id__n: Vec<i64>,
	data_synced: Vec<String>,
	data_synced__empty: bool,
	data_synced__gt: Vec<String>,
	data_synced__gte: Vec<String>,
	data_synced__lt: Vec<String>,
	data_synced__lte: Vec<String>,
	data_synced__n: Vec<String>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String
}
/// Get a list of export template objects.

pub fn extras_export_templates_list(query: ExtrasExportTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/export-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesBulkUpdateQuery {
}
/// Put a list of export template objects.

pub fn extras_export_templates_bulk_update(query: ExtrasExportTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/export-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesCreateQuery {
}
/// Post a list of export template objects.

pub fn extras_export_templates_create(query: ExtrasExportTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/export-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of export template objects.

pub fn extras_export_templates_bulk_partial_update(query: ExtrasExportTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/export-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesBulkDestroyQuery {
}
/// Delete a list of export template objects.

pub fn extras_export_templates_bulk_destroy(query: ExtrasExportTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/export-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	group: Vec<i64>,
	group__n: Vec<i64>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of tenant objects.

pub fn tenancy_tenants_list(query: TenancyTenantsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/tenants/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsBulkUpdateQuery {
}
/// Put a list of tenant objects.

pub fn tenancy_tenants_bulk_update(query: TenancyTenantsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/tenants/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsCreateQuery {
}
/// Post a list of tenant objects.

pub fn tenancy_tenants_create(query: TenancyTenantsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/tenancy/tenants/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsBulkPartialUpdateQuery {
}
/// Patch a list of tenant objects.

pub fn tenancy_tenants_bulk_partial_update(query: TenancyTenantsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/tenants/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsBulkDestroyQuery {
}
/// Delete a list of tenant objects.

pub fn tenancy_tenants_bulk_destroy(query: TenancyTenantsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/tenants/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsRetrieveQuery {
}
/// Get a inventory item object.

pub fn dcim_inventory_items_retrieve(query: DcimInventoryItemsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/inventory-items/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsUpdateQuery {
}
/// Put a inventory item object.

pub fn dcim_inventory_items_update(query: DcimInventoryItemsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/inventory-items/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsPartialUpdateQuery {
}
/// Patch a inventory item object.

pub fn dcim_inventory_items_partial_update(query: DcimInventoryItemsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/inventory-items/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsDestroyQuery {
}
/// Delete a inventory item object.

pub fn dcim_inventory_items_destroy(query: DcimInventoryItemsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/inventory-items/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksRetrieveQuery {
}
/// Get a provider network object.

pub fn circuits_provider_networks_retrieve(query: CircuitsProviderNetworksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/provider-networks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksUpdateQuery {
}
/// Put a provider network object.

pub fn circuits_provider_networks_update(query: CircuitsProviderNetworksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/provider-networks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksPartialUpdateQuery {
}
/// Patch a provider network object.

pub fn circuits_provider_networks_partial_update(query: CircuitsProviderNetworksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/provider-networks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderNetworksDestroyQuery {
}
/// Delete a provider network object.

pub fn circuits_provider_networks_destroy(query: CircuitsProviderNetworksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/provider-networks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsRetrieveQuery {
}
/// Get a location object.

pub fn dcim_locations_retrieve(query: DcimLocationsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/locations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsUpdateQuery {
}
/// Put a location object.

pub fn dcim_locations_update(query: DcimLocationsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/locations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsPartialUpdateQuery {
}
/// Patch a location object.

pub fn dcim_locations_partial_update(query: DcimLocationsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/locations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimLocationsDestroyQuery {
}
/// Delete a location object.

pub fn dcim_locations_destroy(query: DcimLocationsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/locations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksListQuery {
	ca_file_path: Vec<String>,
	ca_file_path__empty: bool,
	ca_file_path__ic: Vec<String>,
	ca_file_path__ie: Vec<String>,
	ca_file_path__iew: Vec<String>,
	ca_file_path__isw: Vec<String>,
	ca_file_path__n: Vec<String>,
	ca_file_path__nic: Vec<String>,
	ca_file_path__nie: Vec<String>,
	ca_file_path__niew: Vec<String>,
	ca_file_path__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	http_content_typ: Vec<String>,
	http_content_typ__empty: bool,
	http_content_typ__ic: Vec<String>,
	http_content_typ__ie: Vec<String>,
	http_content_typ__iew: Vec<String>,
	http_content_typ__isw: Vec<String>,
	http_content_typ__n: Vec<String>,
	http_content_typ__nic: Vec<String>,
	http_content_typ__nie: Vec<String>,
	http_content_typ__niew: Vec<String>,
	http_content_typ__nisw: Vec<String>,
	http_method: Vec<String>,
	http_method__n: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	payload_url: Vec<String>,
	q: String,
	secret: Vec<String>,
	secret__empty: bool,
	secret__ic: Vec<String>,
	secret__ie: Vec<String>,
	secret__iew: Vec<String>,
	secret__isw: Vec<String>,
	secret__n: Vec<String>,
	secret__nic: Vec<String>,
	secret__nie: Vec<String>,
	secret__niew: Vec<String>,
	secret__nisw: Vec<String>,
	ssl_verification: bool,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of webhook objects.

pub fn extras_webhooks_list(query: ExtrasWebhooksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/webhooks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksBulkUpdateQuery {
}
/// Put a list of webhook objects.

pub fn extras_webhooks_bulk_update(query: ExtrasWebhooksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/webhooks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksCreateQuery {
}
/// Post a list of webhook objects.

pub fn extras_webhooks_create(query: ExtrasWebhooksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/webhooks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksBulkPartialUpdateQuery {
}
/// Patch a list of webhook objects.

pub fn extras_webhooks_bulk_partial_update(query: ExtrasWebhooksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/webhooks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksBulkDestroyQuery {
}
/// Delete a list of webhook objects.

pub fn extras_webhooks_bulk_destroy(query: ExtrasWebhooksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/webhooks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsTraceRetrieveQuery {
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).

pub fn dcim_power_feeds_trace_retrieve(query: DcimPowerFeedsTraceRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-feeds/{id}/trace/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsListQuery {
	allocated_draw: Vec<i32>,
	allocated_draw__empty: bool,
	allocated_draw__gt: Vec<i32>,
	allocated_draw__gte: Vec<i32>,
	allocated_draw__lt: Vec<i32>,
	allocated_draw__lte: Vec<i32>,
	allocated_draw__n: Vec<i32>,
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	connected: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	maximum_draw: Vec<i32>,
	maximum_draw__empty: bool,
	maximum_draw__gt: Vec<i32>,
	maximum_draw__gte: Vec<i32>,
	maximum_draw__lt: Vec<i32>,
	maximum_draw__lte: Vec<i32>,
	maximum_draw__n: Vec<i32>,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of power port objects.

pub fn dcim_power_ports_list(query: DcimPowerPortsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsBulkUpdateQuery {
}
/// Put a list of power port objects.

pub fn dcim_power_ports_bulk_update(query: DcimPowerPortsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsCreateQuery {
}
/// Post a list of power port objects.

pub fn dcim_power_ports_create(query: DcimPowerPortsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/power-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsBulkPartialUpdateQuery {
}
/// Patch a list of power port objects.

pub fn dcim_power_ports_bulk_partial_update(query: DcimPowerPortsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsBulkDestroyQuery {
}
/// Delete a list of power port objects.

pub fn dcim_power_ports_bulk_destroy(query: DcimPowerPortsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	device: Vec<String>,
	device_id: Vec<i32>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interface_id: Vec<i32>,
	interface_id__empty: bool,
	interface_id__gt: Vec<i32>,
	interface_id__gte: Vec<i32>,
	interface_id__lt: Vec<i32>,
	interface_id__lte: Vec<i32>,
	interface_id__n: Vec<i32>,
	interface_typ: String,
	interface_typ__n: String,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	priority: Vec<i32>,
	priority__empty: bool,
	priority__gt: Vec<i32>,
	priority__gte: Vec<i32>,
	priority__lt: Vec<i32>,
	priority__lte: Vec<i32>,
	priority__n: Vec<i32>,
	updated_by_request: String,
	virtual_machine: Vec<String>,
	virtual_machine_id: Vec<i32>
}
/// Get a list of FHRP group assignment objects.

pub fn ipam_fhrp_group_assignments_list(query: IpamFhrpGroupAssignmentsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/fhrp-group-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsBulkUpdateQuery {
}
/// Put a list of FHRP group assignment objects.

pub fn ipam_fhrp_group_assignments_bulk_update(query: IpamFhrpGroupAssignmentsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/fhrp-group-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsCreateQuery {
}
/// Post a list of FHRP group assignment objects.

pub fn ipam_fhrp_group_assignments_create(query: IpamFhrpGroupAssignmentsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/fhrp-group-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsBulkPartialUpdateQuery {
}
/// Patch a list of FHRP group assignment objects.

pub fn ipam_fhrp_group_assignments_bulk_partial_update(query: IpamFhrpGroupAssignmentsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/fhrp-group-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsBulkDestroyQuery {
}
/// Delete a list of FHRP group assignment objects.

pub fn ipam_fhrp_group_assignments_bulk_destroy(query: IpamFhrpGroupAssignmentsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/fhrp-group-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesAvailablePrefixesListQuery {
}
/// Get a prefix object.

pub fn ipam_prefixes_available_prefixes_list(query: IpamPrefixesAvailablePrefixesListQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/prefixes/{id}/available-prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesAvailablePrefixesCreateQuery {
}
/// Post a prefix object.

pub fn ipam_prefixes_available_prefixes_create(query: IpamPrefixesAvailablePrefixesCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/prefixes/{id}/available-prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	enabled: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of data source objects.

pub fn core_data_sources_list(query: CoreDataSourcesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/core/data-sources/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesBulkUpdateQuery {
}
/// Put a list of data source objects.

pub fn core_data_sources_bulk_update(query: CoreDataSourcesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/core/data-sources/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesCreateQuery {
}
/// Post a list of data source objects.

pub fn core_data_sources_create(query: CoreDataSourcesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/core/data-sources/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesBulkPartialUpdateQuery {
}
/// Patch a list of data source objects.

pub fn core_data_sources_bulk_partial_update(query: CoreDataSourcesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/core/data-sources/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesBulkDestroyQuery {
}
/// Delete a list of data source objects.

pub fn core_data_sources_bulk_destroy(query: CoreDataSourcesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/core/data-sources/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreJobsListQuery {
	completed: String,
	completed__after: String,
	completed__before: String,
	created: String,
	created__after: String,
	created__before: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interval: Vec<i32>,
	interval__empty: bool,
	interval__gt: Vec<i32>,
	interval__gte: Vec<i32>,
	interval__lt: Vec<i32>,
	interval__lte: Vec<i32>,
	interval__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	object_id: Vec<i32>,
	object_id__empty: bool,
	object_id__gt: Vec<i32>,
	object_id__gte: Vec<i32>,
	object_id__lt: Vec<i32>,
	object_id__lte: Vec<i32>,
	object_id__n: Vec<i32>,
	object_typ: i64,
	object_typ__n: i64,
	offset: i64,
	ordering: String,
	q: String,
	scheduled: String,
	scheduled__after: String,
	scheduled__before: String,
	started: String,
	started__after: String,
	started__before: String,
	status: Vec<String>,
	status__n: Vec<String>,
	user: i64,
	user__n: i64
}
/// Retrieve a list of job results

pub fn core_jobs_list(query: CoreJobsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/core/jobs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	ike_policy: Vec<String>,
	ike_policy__n: Vec<String>,
	ike_policy_id: Vec<i64>,
	ike_policy_id__n: Vec<i64>,
	ipsec_policy: Vec<String>,
	ipsec_policy__n: Vec<String>,
	ipsec_policy_id: Vec<i64>,
	ipsec_policy_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mode: Vec<String>,
	mode__n: Vec<String>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of IPSec profile objects.

pub fn vpn_ipsec_profiles_list(query: VpnIpsecProfilesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ipsec-profiles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesBulkUpdateQuery {
}
/// Put a list of IPSec profile objects.

pub fn vpn_ipsec_profiles_bulk_update(query: VpnIpsecProfilesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ipsec-profiles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesCreateQuery {
}
/// Post a list of IPSec profile objects.

pub fn vpn_ipsec_profiles_create(query: VpnIpsecProfilesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/ipsec-profiles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesBulkPartialUpdateQuery {
}
/// Patch a list of IPSec profile objects.

pub fn vpn_ipsec_profiles_bulk_partial_update(query: VpnIpsecProfilesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ipsec-profiles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesBulkDestroyQuery {
}
/// Delete a list of IPSec profile objects.

pub fn vpn_ipsec_profiles_bulk_destroy(query: VpnIpsecProfilesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ipsec-profiles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesListQuery {
	children: Vec<i32>,
	children__empty: Vec<i32>,
	children__gt: Vec<i32>,
	children__gte: Vec<i32>,
	children__lt: Vec<i32>,
	children__lte: Vec<i32>,
	children__n: Vec<i32>,
	contains: String,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	depth: Vec<i32>,
	depth__empty: Vec<i32>,
	depth__gt: Vec<i32>,
	depth__gte: Vec<i32>,
	depth__lt: Vec<i32>,
	depth__lte: Vec<i32>,
	depth__n: Vec<i32>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	family: f64,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	is_pool: bool,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mark_utilized: bool,
	mask_length: Vec<i32>,
	mask_length__gte: f64,
	mask_length__lte: f64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	prefix: Vec<String>,
	present_in_vrf: String,
	present_in_vrf_id: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	vlan_id: Vec<i64>,
	vlan_id__n: Vec<i64>,
	vlan_vid: i64,
	vlan_vid__empty: i64,
	vlan_vid__gt: i64,
	vlan_vid__gte: i64,
	vlan_vid__lt: i64,
	vlan_vid__lte: i64,
	vlan_vid__n: i64,
	vrf: Vec<String>,
	vrf__n: Vec<String>,
	vrf_id: Vec<i64>,
	vrf_id__n: Vec<i64>,
	within: String,
	within_include: String
}
/// Get a list of prefix objects.

pub fn ipam_prefixes_list(query: IpamPrefixesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesBulkUpdateQuery {
}
/// Put a list of prefix objects.

pub fn ipam_prefixes_bulk_update(query: IpamPrefixesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesCreateQuery {
}
/// Post a list of prefix objects.

pub fn ipam_prefixes_create(query: IpamPrefixesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesBulkPartialUpdateQuery {
}
/// Patch a list of prefix objects.

pub fn ipam_prefixes_bulk_partial_update(query: IpamPrefixesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesBulkDestroyQuery {
}
/// Delete a list of prefix objects.

pub fn ipam_prefixes_bulk_destroy(query: IpamPrefixesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/prefixes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsTraceRetrieveQuery {
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).

pub fn dcim_console_ports_trace_retrieve(query: DcimConsolePortsTraceRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-ports/{id}/trace/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of cluster type objects.

pub fn virtualization_cluster_types_list(query: VirtualizationClusterTypesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/cluster-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesBulkUpdateQuery {
}
/// Put a list of cluster type objects.

pub fn virtualization_cluster_types_bulk_update(query: VirtualizationClusterTypesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/cluster-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesCreateQuery {
}
/// Post a list of cluster type objects.

pub fn virtualization_cluster_types_create(query: VirtualizationClusterTypesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/cluster-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesBulkPartialUpdateQuery {
}
/// Patch a list of cluster type objects.

pub fn virtualization_cluster_types_bulk_partial_update(query: VirtualizationClusterTypesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/cluster-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterTypesBulkDestroyQuery {
}
/// Delete a list of cluster type objects.

pub fn virtualization_cluster_types_bulk_destroy(query: VirtualizationClusterTypesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/cluster-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesListQuery {
	bridge_id: Vec<i64>,
	bridge_id__n: Vec<i64>,
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	connected: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	duplex: Vec<String>,
	duplex__n: Vec<String>,
	enabled: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	kind: String,
	l2vpn: Vec<i64>,
	l2vpn__n: Vec<i64>,
	l2vpn_id: Vec<i64>,
	l2vpn_id__n: Vec<i64>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	lag_id: Vec<i64>,
	lag_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	mac_address: Vec<String>,
	mac_address__ic: Vec<String>,
	mac_address__ie: Vec<String>,
	mac_address__iew: Vec<String>,
	mac_address__isw: Vec<String>,
	mac_address__n: Vec<String>,
	mac_address__nic: Vec<String>,
	mac_address__nie: Vec<String>,
	mac_address__niew: Vec<String>,
	mac_address__nisw: Vec<String>,
	mgmt_only: bool,
	mode: String,
	mode__n: String,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	mtu: Vec<i32>,
	mtu__empty: bool,
	mtu__gt: Vec<i32>,
	mtu__gte: Vec<i32>,
	mtu__lt: Vec<i32>,
	mtu__lte: Vec<i32>,
	mtu__n: Vec<i32>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	poe_mode: Vec<String>,
	poe_mode__n: Vec<String>,
	poe_typ: Vec<String>,
	poe_typ__n: Vec<String>,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	rf_channel: Vec<String>,
	rf_channel__n: Vec<String>,
	rf_channel_frequency: Vec<f64>,
	rf_channel_frequency__empty: bool,
	rf_channel_frequency__gt: Vec<f64>,
	rf_channel_frequency__gte: Vec<f64>,
	rf_channel_frequency__lt: Vec<f64>,
	rf_channel_frequency__lte: Vec<f64>,
	rf_channel_frequency__n: Vec<f64>,
	rf_channel_width: Vec<f64>,
	rf_channel_width__empty: bool,
	rf_channel_width__gt: Vec<f64>,
	rf_channel_width__gte: Vec<f64>,
	rf_channel_width__lt: Vec<f64>,
	rf_channel_width__lte: Vec<f64>,
	rf_channel_width__n: Vec<f64>,
	rf_role: Vec<String>,
	rf_role__n: Vec<String>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	speed: Vec<i32>,
	speed__empty: Vec<i32>,
	speed__gt: Vec<i32>,
	speed__gte: Vec<i32>,
	speed__lt: Vec<i32>,
	speed__lte: Vec<i32>,
	speed__n: Vec<i32>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tx_power: Vec<i32>,
	tx_power__empty: bool,
	tx_power__gt: Vec<i32>,
	tx_power__gte: Vec<i32>,
	tx_power__lt: Vec<i32>,
	tx_power__lte: Vec<i32>,
	tx_power__n: Vec<i32>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	vdc: Vec<String>,
	vdc__n: Vec<String>,
	vdc_id: Vec<i64>,
	vdc_id__n: Vec<i64>,
	vdc_identifier: Vec<i64>,
	vdc_identifier__n: Vec<i64>,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>,
	virtual_chassis_member: Vec<String>,
	virtual_chassis_member_id: Vec<i32>,
	vlan: String,
	vlan_id: String,
	vrf: Vec<String>,
	vrf__n: Vec<String>,
	vrf_id: Vec<i64>,
	vrf_id__n: Vec<i64>,
	wwn: Vec<String>,
	wwn__ic: Vec<String>,
	wwn__ie: Vec<String>,
	wwn__iew: Vec<String>,
	wwn__isw: Vec<String>,
	wwn__n: Vec<String>,
	wwn__nic: Vec<String>,
	wwn__nie: Vec<String>,
	wwn__niew: Vec<String>,
	wwn__nisw: Vec<String>
}
/// Get a list of interface objects.

pub fn dcim_interfaces_list(query: DcimInterfacesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesBulkUpdateQuery {
}
/// Put a list of interface objects.

pub fn dcim_interfaces_bulk_update(query: DcimInterfacesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesCreateQuery {
}
/// Post a list of interface objects.

pub fn dcim_interfaces_create(query: DcimInterfacesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesBulkPartialUpdateQuery {
}
/// Patch a list of interface objects.

pub fn dcim_interfaces_bulk_partial_update(query: DcimInterfacesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesBulkDestroyQuery {
}
/// Delete a list of interface objects.

pub fn dcim_interfaces_bulk_destroy(query: DcimInterfacesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsListQuery {
	address: Vec<String>,
	address__empty: bool,
	address__ic: Vec<String>,
	address__ie: Vec<String>,
	address__iew: Vec<String>,
	address__isw: Vec<String>,
	address__n: Vec<String>,
	address__nic: Vec<String>,
	address__nie: Vec<String>,
	address__niew: Vec<String>,
	address__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	email: Vec<String>,
	email__empty: bool,
	email__ic: Vec<String>,
	email__ie: Vec<String>,
	email__iew: Vec<String>,
	email__isw: Vec<String>,
	email__n: Vec<String>,
	email__nic: Vec<String>,
	email__nie: Vec<String>,
	email__niew: Vec<String>,
	email__nisw: Vec<String>,
	group: Vec<i64>,
	group__n: Vec<i64>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	link: Vec<String>,
	link__empty: bool,
	link__ic: Vec<String>,
	link__ie: Vec<String>,
	link__iew: Vec<String>,
	link__isw: Vec<String>,
	link__n: Vec<String>,
	link__nic: Vec<String>,
	link__nie: Vec<String>,
	link__niew: Vec<String>,
	link__nisw: Vec<String>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	phone: Vec<String>,
	phone__empty: bool,
	phone__ic: Vec<String>,
	phone__ie: Vec<String>,
	phone__iew: Vec<String>,
	phone__isw: Vec<String>,
	phone__n: Vec<String>,
	phone__nic: Vec<String>,
	phone__nie: Vec<String>,
	phone__niew: Vec<String>,
	phone__nisw: Vec<String>,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	title: Vec<String>,
	title__empty: bool,
	title__ic: Vec<String>,
	title__ie: Vec<String>,
	title__iew: Vec<String>,
	title__isw: Vec<String>,
	title__n: Vec<String>,
	title__nic: Vec<String>,
	title__nie: Vec<String>,
	title__niew: Vec<String>,
	title__nisw: Vec<String>,
	updated_by_request: String
}
/// Get a list of contact objects.

pub fn tenancy_contacts_list(query: TenancyContactsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contacts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsBulkUpdateQuery {
}
/// Put a list of contact objects.

pub fn tenancy_contacts_bulk_update(query: TenancyContactsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contacts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsCreateQuery {
}
/// Post a list of contact objects.

pub fn tenancy_contacts_create(query: TenancyContactsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/tenancy/contacts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsBulkPartialUpdateQuery {
}
/// Patch a list of contact objects.

pub fn tenancy_contacts_bulk_partial_update(query: TenancyContactsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contacts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsBulkDestroyQuery {
}
/// Delete a list of contact objects.

pub fn tenancy_contacts_bulk_destroy(query: TenancyContactsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contacts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of front port template objects.

pub fn dcim_front_port_templates_list(query: DcimFrontPortTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/front-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesBulkUpdateQuery {
}
/// Put a list of front port template objects.

pub fn dcim_front_port_templates_bulk_update(query: DcimFrontPortTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/front-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesCreateQuery {
}
/// Post a list of front port template objects.

pub fn dcim_front_port_templates_create(query: DcimFrontPortTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/front-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of front port template objects.

pub fn dcim_front_port_templates_bulk_partial_update(query: DcimFrontPortTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/front-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesBulkDestroyQuery {
}
/// Delete a list of front port template objects.

pub fn dcim_front_port_templates_bulk_destroy(query: DcimFrontPortTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/front-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsListQuery {
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	connected: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	feed_leg: Vec<String>,
	feed_leg__n: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of power outlet objects.

pub fn dcim_power_outlets_list(query: DcimPowerOutletsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-outlets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsBulkUpdateQuery {
}
/// Put a list of power outlet objects.

pub fn dcim_power_outlets_bulk_update(query: DcimPowerOutletsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-outlets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsCreateQuery {
}
/// Post a list of power outlet objects.

pub fn dcim_power_outlets_create(query: DcimPowerOutletsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/power-outlets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsBulkPartialUpdateQuery {
}
/// Patch a list of power outlet objects.

pub fn dcim_power_outlets_bulk_partial_update(query: DcimPowerOutletsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-outlets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsBulkDestroyQuery {
}
/// Delete a list of power outlet objects.

pub fn dcim_power_outlets_bulk_destroy(query: DcimPowerOutletsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-outlets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesListQuery {
	circuittermination_id: Vec<i32>,
	color: Vec<String>,
	color__n: Vec<String>,
	consoleport_id: Vec<i32>,
	consoleserverport_id: Vec<i32>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device_id: Vec<i32>,
	frontport_id: Vec<i32>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interface_id: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	length: Vec<f64>,
	length__empty: bool,
	length__gt: Vec<f64>,
	length__gte: Vec<f64>,
	length__lt: Vec<f64>,
	length__lte: Vec<f64>,
	length__n: Vec<f64>,
	length_unit: String,
	length_unit__n: String,
	limit: i64,
	location: Vec<String>,
	location_id: Vec<i32>,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	powerfeed_id: Vec<i32>,
	poweroutlet_id: Vec<i32>,
	powerport_id: Vec<i32>,
	q: String,
	rack: Vec<String>,
	rack_id: Vec<i32>,
	rearport_id: Vec<i32>,
	site: Vec<String>,
	site_id: Vec<i32>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	termination_a_id: Vec<i32>,
	termination_a_typ: String,
	termination_a_typ__n: String,
	termination_b_id: Vec<i32>,
	termination_b_typ: String,
	termination_b_typ__n: String,
	typ: Vec<String>,
	typ__n: Vec<String>,
	unterminated: bool,
	updated_by_request: String
}
/// Get a list of cable objects.

pub fn dcim_cables_list(query: DcimCablesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/cables/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesBulkUpdateQuery {
}
/// Put a list of cable objects.

pub fn dcim_cables_bulk_update(query: DcimCablesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/cables/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesCreateQuery {
}
/// Post a list of cable objects.

pub fn dcim_cables_create(query: DcimCablesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/cables/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesBulkPartialUpdateQuery {
}
/// Patch a list of cable objects.

pub fn dcim_cables_bulk_partial_update(query: DcimCablesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/cables/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCablesBulkDestroyQuery {
}
/// Delete a list of cable objects.

pub fn dcim_cables_bulk_destroy(query: DcimCablesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/cables/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	end: Vec<i32>,
	end__empty: bool,
	end__gt: Vec<i32>,
	end__gte: Vec<i32>,
	end__lt: Vec<i32>,
	end__lte: Vec<i32>,
	end__n: Vec<i32>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	rir: Vec<String>,
	rir__n: Vec<String>,
	rir_id: Vec<i64>,
	rir_id__n: Vec<i64>,
	start: Vec<i32>,
	start__empty: bool,
	start__gt: Vec<i32>,
	start__gte: Vec<i32>,
	start__lt: Vec<i32>,
	start__lte: Vec<i32>,
	start__n: Vec<i32>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of ASN range objects.

pub fn ipam_asn_ranges_list(query: IpamAsnRangesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/asn-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesBulkUpdateQuery {
}
/// Put a list of ASN range objects.

pub fn ipam_asn_ranges_bulk_update(query: IpamAsnRangesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/asn-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesCreateQuery {
}
/// Post a list of ASN range objects.

pub fn ipam_asn_ranges_create(query: IpamAsnRangesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/asn-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesBulkPartialUpdateQuery {
}
/// Patch a list of ASN range objects.

pub fn ipam_asn_ranges_bulk_partial_update(query: IpamAsnRangesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/asn-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnRangesBulkDestroyQuery {
}
/// Delete a list of ASN range objects.

pub fn ipam_asn_ranges_bulk_destroy(query: IpamAsnRangesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/asn-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsRetrieveQuery {
}
/// Get a custom field object.

pub fn extras_custom_fields_retrieve(query: ExtrasCustomFieldsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-fields/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsUpdateQuery {
}
/// Put a custom field object.

pub fn extras_custom_fields_update(query: ExtrasCustomFieldsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/custom-fields/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsPartialUpdateQuery {
}
/// Patch a custom field object.

pub fn extras_custom_fields_partial_update(query: ExtrasCustomFieldsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/custom-fields/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsDestroyQuery {
}
/// Delete a custom field object.

pub fn extras_custom_fields_destroy(query: ExtrasCustomFieldsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/custom-fields/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasDashboardRetrieveQuery {
}
/// Get a list of dashboard objects.

pub fn extras_dashboard_retrieve(query: ExtrasDashboardRetrieveQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/dashboard/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasDashboardUpdateQuery {
}
/// Put a list of dashboard objects.

pub fn extras_dashboard_update(query: ExtrasDashboardUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/dashboard/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasDashboardPartialUpdateQuery {
}
/// Patch a list of dashboard objects.

pub fn extras_dashboard_partial_update(query: ExtrasDashboardPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/dashboard/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasDashboardDestroyQuery {
}
/// Delete a list of dashboard objects.

pub fn extras_dashboard_destroy(query: ExtrasDashboardDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/dashboard/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of inventory item role objects.

pub fn dcim_inventory_item_roles_list(query: DcimInventoryItemRolesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/inventory-item-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesBulkUpdateQuery {
}
/// Put a list of inventory item role objects.

pub fn dcim_inventory_item_roles_bulk_update(query: DcimInventoryItemRolesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/inventory-item-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesCreateQuery {
}
/// Post a list of inventory item role objects.

pub fn dcim_inventory_item_roles_create(query: DcimInventoryItemRolesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/inventory-item-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesBulkPartialUpdateQuery {
}
/// Patch a list of inventory item role objects.

pub fn dcim_inventory_item_roles_bulk_partial_update(query: DcimInventoryItemRolesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/inventory-item-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesBulkDestroyQuery {
}
/// Delete a list of inventory item role objects.

pub fn dcim_inventory_item_roles_bulk_destroy(query: DcimInventoryItemRolesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/inventory-item-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksRetrieveQuery {
}
/// Get a wireless link object.

pub fn wireless_wireless_links_retrieve(query: WirelessWirelessLinksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/wireless/wireless-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksUpdateQuery {
}
/// Put a wireless link object.

pub fn wireless_wireless_links_update(query: WirelessWirelessLinksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/wireless/wireless-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksPartialUpdateQuery {
}
/// Patch a wireless link object.

pub fn wireless_wireless_links_partial_update(query: WirelessWirelessLinksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/wireless/wireless-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksDestroyQuery {
}
/// Delete a wireless link object.

pub fn wireless_wireless_links_destroy(query: WirelessWirelessLinksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/wireless/wireless-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesSyncCreateQuery {
}
/// Enqueue a job to synchronize the DataSource.

pub fn core_data_sources_sync_create(query: CoreDataSourcesSyncCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/core/data-sources/{id}/sync/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	positions: Vec<i32>,
	positions__empty: bool,
	positions__gt: Vec<i32>,
	positions__gte: Vec<i32>,
	positions__lt: Vec<i32>,
	positions__lte: Vec<i32>,
	positions__n: Vec<i32>,
	q: String,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of rear port template objects.

pub fn dcim_rear_port_templates_list(query: DcimRearPortTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rear-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesBulkUpdateQuery {
}
/// Put a list of rear port template objects.

pub fn dcim_rear_port_templates_bulk_update(query: DcimRearPortTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rear-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesCreateQuery {
}
/// Post a list of rear port template objects.

pub fn dcim_rear_port_templates_create(query: DcimRearPortTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/rear-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of rear port template objects.

pub fn dcim_rear_port_templates_bulk_partial_update(query: DcimRearPortTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rear-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesBulkDestroyQuery {
}
/// Delete a list of rear port template objects.

pub fn dcim_rear_port_templates_bulk_destroy(query: DcimRearPortTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rear-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaRetrieveQuery {
	format: String
}
/// OpenApi3 schema for this API. Format can be selected via content negotiation.
/// 
/// - YAML: application/vnd.oai.openapi
/// - JSON: application/vnd.oai.openapi+json

pub fn schema_retrieve(query: SchemaRetrieveQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/schema/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasContentTypesRetrieveQuery {
}
/// Read-only list of ContentTypes. Limit results to ContentTypes pertinent to NetBox objects.

pub fn extras_content_types_retrieve(query: ExtrasContentTypesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/content-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesRetrieveQuery {
}
/// Get a site object.

pub fn dcim_sites_retrieve(query: DcimSitesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/sites/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesUpdateQuery {
}
/// Put a site object.

pub fn dcim_sites_update(query: DcimSitesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/sites/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesPartialUpdateQuery {
}
/// Patch a site object.

pub fn dcim_sites_partial_update(query: DcimSitesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/sites/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesDestroyQuery {
}
/// Delete a site object.

pub fn dcim_sites_destroy(query: DcimSitesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/sites/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsListQuery {
	assigned_object_typ: String,
	assigned_object_typ__n: String,
	assigned_object_typ_id: i64,
	assigned_object_typ_id__n: i64,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interface: Vec<String>,
	interface__n: Vec<String>,
	interface_id: Vec<i64>,
	interface_id__n: Vec<i64>,
	l2vpn: Vec<String>,
	l2vpn__n: Vec<String>,
	l2vpn_id: Vec<i64>,
	l2vpn_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<String>,
	region_id: Vec<i32>,
	site: Vec<String>,
	site_id: Vec<i32>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_machine: Vec<String>,
	virtual_machine__n: Vec<String>,
	virtual_machine_id: Vec<i64>,
	virtual_machine_id__n: Vec<i64>,
	vlan: Vec<String>,
	vlan__n: Vec<String>,
	vlan_id: Vec<i64>,
	vlan_id__n: Vec<i64>,
	vlan_vid: i64,
	vlan_vid__empty: i64,
	vlan_vid__gt: i64,
	vlan_vid__gte: i64,
	vlan_vid__lt: i64,
	vlan_vid__lte: i64,
	vlan_vid__n: i64,
	vminterface: Vec<String>,
	vminterface__n: Vec<String>,
	vminterface_id: Vec<i64>,
	vminterface_id__n: Vec<i64>
}
/// Get a list of L2VPN termination objects.

pub fn vpn_l2vpn_terminations_list(query: VpnL2VpnTerminationsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/l2vpn-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsBulkUpdateQuery {
}
/// Put a list of L2VPN termination objects.

pub fn vpn_l2vpn_terminations_bulk_update(query: VpnL2VpnTerminationsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/l2vpn-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsCreateQuery {
}
/// Post a list of L2VPN termination objects.

pub fn vpn_l2vpn_terminations_create(query: VpnL2VpnTerminationsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/l2vpn-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsBulkPartialUpdateQuery {
}
/// Patch a list of L2VPN termination objects.

pub fn vpn_l2vpn_terminations_bulk_partial_update(query: VpnL2VpnTerminationsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/l2vpn-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsBulkDestroyQuery {
}
/// Delete a list of L2VPN termination objects.

pub fn vpn_l2vpn_terminations_bulk_destroy(query: VpnL2VpnTerminationsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/l2vpn-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	date_added: Vec<String>,
	date_added__empty: bool,
	date_added__gt: Vec<String>,
	date_added__gte: Vec<String>,
	date_added__lt: Vec<String>,
	date_added__lte: Vec<String>,
	date_added__n: Vec<String>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	family: f64,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	prefix: String,
	q: String,
	rir: Vec<String>,
	rir__n: Vec<String>,
	rir_id: Vec<i64>,
	rir_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of aggregate objects.

pub fn ipam_aggregates_list(query: IpamAggregatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/aggregates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesBulkUpdateQuery {
}
/// Put a list of aggregate objects.

pub fn ipam_aggregates_bulk_update(query: IpamAggregatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/aggregates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesCreateQuery {
}
/// Post a list of aggregate objects.

pub fn ipam_aggregates_create(query: IpamAggregatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/aggregates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesBulkPartialUpdateQuery {
}
/// Patch a list of aggregate objects.

pub fn ipam_aggregates_bulk_partial_update(query: IpamAggregatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/aggregates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAggregatesBulkDestroyQuery {
}
/// Delete a list of aggregate objects.

pub fn ipam_aggregates_bulk_destroy(query: IpamAggregatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/aggregates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsPathsRetrieveQuery {
}
/// Return all CablePaths which traverse a given pass-through port.

pub fn dcim_rear_ports_paths_retrieve(query: DcimRearPortsPathsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rear-ports/{id}/paths/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesListQuery {
	asn: Vec<i64>,
	asn__n: Vec<i64>,
	asn_id: Vec<i64>,
	asn_id__n: Vec<i64>,
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	facility: Vec<String>,
	facility__empty: bool,
	facility__ic: Vec<String>,
	facility__ie: Vec<String>,
	facility__iew: Vec<String>,
	facility__isw: Vec<String>,
	facility__n: Vec<String>,
	facility__nic: Vec<String>,
	facility__nie: Vec<String>,
	facility__niew: Vec<String>,
	facility__nisw: Vec<String>,
	group: Vec<i64>,
	group__n: Vec<i64>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	latitude: Vec<f64>,
	latitude__empty: bool,
	latitude__gt: Vec<f64>,
	latitude__gte: Vec<f64>,
	latitude__lt: Vec<f64>,
	latitude__lte: Vec<f64>,
	latitude__n: Vec<f64>,
	limit: i64,
	longitude: Vec<f64>,
	longitude__empty: bool,
	longitude__gt: Vec<f64>,
	longitude__gte: Vec<f64>,
	longitude__lt: Vec<f64>,
	longitude__lte: Vec<f64>,
	longitude__n: Vec<f64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of site objects.

pub fn dcim_sites_list(query: DcimSitesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/sites/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesBulkUpdateQuery {
}
/// Put a list of site objects.

pub fn dcim_sites_bulk_update(query: DcimSitesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/sites/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesCreateQuery {
}
/// Post a list of site objects.

pub fn dcim_sites_create(query: DcimSitesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/sites/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesBulkPartialUpdateQuery {
}
/// Patch a list of site objects.

pub fn dcim_sites_bulk_partial_update(query: DcimSitesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/sites/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSitesBulkDestroyQuery {
}
/// Delete a list of site objects.

pub fn dcim_sites_bulk_destroy(query: DcimSitesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/sites/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsListQuery {
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	connected: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of console port objects.

pub fn dcim_console_ports_list(query: DcimConsolePortsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsBulkUpdateQuery {
}
/// Put a list of console port objects.

pub fn dcim_console_ports_bulk_update(query: DcimConsolePortsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsCreateQuery {
}
/// Post a list of console port objects.

pub fn dcim_console_ports_create(query: DcimConsolePortsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/console-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsBulkPartialUpdateQuery {
}
/// Patch a list of console port objects.

pub fn dcim_console_ports_bulk_partial_update(query: DcimConsolePortsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortsBulkDestroyQuery {
}
/// Delete a list of console port objects.

pub fn dcim_console_ports_bulk_destroy(query: DcimConsolePortsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesRetrieveQuery {
}
/// Get a power port template object.

pub fn dcim_power_port_templates_retrieve(query: DcimPowerPortTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesUpdateQuery {
}
/// Put a power port template object.

pub fn dcim_power_port_templates_update(query: DcimPowerPortTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesPartialUpdateQuery {
}
/// Patch a power port template object.

pub fn dcim_power_port_templates_partial_update(query: DcimPowerPortTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesDestroyQuery {
}
/// Delete a power port template object.

pub fn dcim_power_port_templates_destroy(query: DcimPowerPortTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesRetrieveQuery {
}
/// Get a IPSec policy object.

pub fn vpn_ipsec_policies_retrieve(query: VpnIpsecPoliciesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ipsec-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesUpdateQuery {
}
/// Put a IPSec policy object.

pub fn vpn_ipsec_policies_update(query: VpnIpsecPoliciesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ipsec-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesPartialUpdateQuery {
}
/// Patch a IPSec policy object.

pub fn vpn_ipsec_policies_partial_update(query: VpnIpsecPoliciesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ipsec-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesDestroyQuery {
}
/// Delete a IPSec policy object.

pub fn vpn_ipsec_policies_destroy(query: VpnIpsecPoliciesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ipsec-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsListQuery {
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String
}
/// Get a list of group objects.

pub fn users_groups_list(query: UsersGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsBulkUpdateQuery {
}
/// Put a list of group objects.

pub fn users_groups_bulk_update(query: UsersGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsCreateQuery {
}
/// Post a list of group objects.

pub fn users_groups_create(query: UsersGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/users/groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsBulkPartialUpdateQuery {
}
/// Patch a list of group objects.

pub fn users_groups_bulk_partial_update(query: UsersGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsBulkDestroyQuery {
}
/// Delete a list of group objects.

pub fn users_groups_bulk_destroy(query: UsersGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesRetrieveQuery {
}
/// Get a device bay template object.

pub fn dcim_device_bay_templates_retrieve(query: DcimDeviceBayTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesUpdateQuery {
}
/// Put a device bay template object.

pub fn dcim_device_bay_templates_update(query: DcimDeviceBayTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesPartialUpdateQuery {
}
/// Patch a device bay template object.

pub fn dcim_device_bay_templates_partial_update(query: DcimDeviceBayTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesDestroyQuery {
}
/// Delete a device bay template object.

pub fn dcim_device_bay_templates_destroy(query: DcimDeviceBayTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsRetrieveQuery {
}
/// Get a ASN object.

pub fn ipam_asns_retrieve(query: IpamAsnsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/asns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsUpdateQuery {
}
/// Put a ASN object.

pub fn ipam_asns_update(query: IpamAsnsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/asns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsPartialUpdateQuery {
}
/// Patch a ASN object.

pub fn ipam_asns_partial_update(query: IpamAsnsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/asns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsDestroyQuery {
}
/// Delete a ASN object.

pub fn ipam_asns_destroy(query: IpamAsnsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/asns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesRetrieveQuery {
}
/// Get a module object.

pub fn dcim_modules_retrieve(query: DcimModulesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/modules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesUpdateQuery {
}
/// Put a module object.

pub fn dcim_modules_update(query: DcimModulesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/modules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesPartialUpdateQuery {
}
/// Patch a module object.

pub fn dcim_modules_partial_update(query: DcimModulesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/modules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModulesDestroyQuery {
}
/// Delete a module object.

pub fn dcim_modules_destroy(query: DcimModulesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/modules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksRetrieveQuery {
}
/// Get a virtual disk object.

pub fn virtualization_virtual_disks_retrieve(query: VirtualizationVirtualDisksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/virtual-disks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksUpdateQuery {
}
/// Put a virtual disk object.

pub fn virtualization_virtual_disks_update(query: VirtualizationVirtualDisksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/virtual-disks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksPartialUpdateQuery {
}
/// Patch a virtual disk object.

pub fn virtualization_virtual_disks_partial_update(query: VirtualizationVirtualDisksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/virtual-disks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksDestroyQuery {
}
/// Delete a virtual disk object.

pub fn virtualization_virtual_disks_destroy(query: VirtualizationVirtualDisksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/virtual-disks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsSyncCreateQuery {
}
/// Provide a /sync API endpoint to synchronize an object's data from its associated DataFile (if any).

pub fn extras_config_contexts_sync_create(query: ExtrasConfigContextsSyncCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/config-contexts/{id}/sync/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsRetrieveQuery {
}
/// Get a FHRP group object.

pub fn ipam_fhrp_groups_retrieve(query: IpamFhrpGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/fhrp-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsUpdateQuery {
}
/// Put a FHRP group object.

pub fn ipam_fhrp_groups_update(query: IpamFhrpGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/fhrp-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsPartialUpdateQuery {
}
/// Patch a FHRP group object.

pub fn ipam_fhrp_groups_partial_update(query: IpamFhrpGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/fhrp-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupsDestroyQuery {
}
/// Delete a FHRP group object.

pub fn ipam_fhrp_groups_destroy(query: IpamFhrpGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/fhrp-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsRetrieveQuery {
}
/// Get a front port object.

pub fn dcim_front_ports_retrieve(query: DcimFrontPortsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/front-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsUpdateQuery {
}
/// Put a front port object.

pub fn dcim_front_ports_update(query: DcimFrontPortsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/front-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsPartialUpdateQuery {
}
/// Patch a front port object.

pub fn dcim_front_ports_partial_update(query: DcimFrontPortsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/front-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsDestroyQuery {
}
/// Delete a front port object.

pub fn dcim_front_ports_destroy(query: DcimFrontPortsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/front-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	exporting_vrf: Vec<String>,
	exporting_vrf__n: Vec<String>,
	exporting_vrf_id: Vec<i64>,
	exporting_vrf_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	importing_vrf: Vec<String>,
	importing_vrf__n: Vec<String>,
	importing_vrf_id: Vec<i64>,
	importing_vrf_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of route target objects.

pub fn ipam_route_targets_list(query: IpamRouteTargetsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/route-targets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsBulkUpdateQuery {
}
/// Put a list of route target objects.

pub fn ipam_route_targets_bulk_update(query: IpamRouteTargetsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/route-targets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsCreateQuery {
}
/// Post a list of route target objects.

pub fn ipam_route_targets_create(query: IpamRouteTargetsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/route-targets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsBulkPartialUpdateQuery {
}
/// Patch a list of route target objects.

pub fn ipam_route_targets_bulk_partial_update(query: IpamRouteTargetsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/route-targets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsBulkDestroyQuery {
}
/// Delete a list of route target objects.

pub fn ipam_route_targets_bulk_destroy(query: IpamRouteTargetsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/route-targets/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of circuit type objects.

pub fn circuits_circuit_types_list(query: CircuitsCircuitTypesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuit-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesBulkUpdateQuery {
}
/// Put a list of circuit type objects.

pub fn circuits_circuit_types_bulk_update(query: CircuitsCircuitTypesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/circuit-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesCreateQuery {
}
/// Post a list of circuit type objects.

pub fn circuits_circuit_types_create(query: CircuitsCircuitTypesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/circuits/circuit-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesBulkPartialUpdateQuery {
}
/// Patch a list of circuit type objects.

pub fn circuits_circuit_types_bulk_partial_update(query: CircuitsCircuitTypesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/circuit-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesBulkDestroyQuery {
}
/// Delete a list of circuit type objects.

pub fn circuits_circuit_types_bulk_destroy(query: CircuitsCircuitTypesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/circuit-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsRetrieveQuery {
}
/// Get a power port object.

pub fn dcim_power_ports_retrieve(query: DcimPowerPortsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsUpdateQuery {
}
/// Put a power port object.

pub fn dcim_power_ports_update(query: DcimPowerPortsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsPartialUpdateQuery {
}
/// Patch a power port object.

pub fn dcim_power_ports_partial_update(query: DcimPowerPortsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsDestroyQuery {
}
/// Delete a power port object.

pub fn dcim_power_ports_destroy(query: DcimPowerPortsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsListQuery {
	account: Vec<String>,
	account__empty: bool,
	account__ic: Vec<String>,
	account__ie: Vec<String>,
	account__iew: Vec<String>,
	account__isw: Vec<String>,
	account__n: Vec<String>,
	account__nic: Vec<String>,
	account__nie: Vec<String>,
	account__niew: Vec<String>,
	account__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	provider: Vec<String>,
	provider__n: Vec<String>,
	provider_id: Vec<i64>,
	provider_id__n: Vec<i64>,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of provider account objects.

pub fn circuits_provider_accounts_list(query: CircuitsProviderAccountsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/provider-accounts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsBulkUpdateQuery {
}
/// Put a list of provider account objects.

pub fn circuits_provider_accounts_bulk_update(query: CircuitsProviderAccountsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/provider-accounts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsCreateQuery {
}
/// Post a list of provider account objects.

pub fn circuits_provider_accounts_create(query: CircuitsProviderAccountsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/circuits/provider-accounts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsBulkPartialUpdateQuery {
}
/// Patch a list of provider account objects.

pub fn circuits_provider_accounts_bulk_partial_update(query: CircuitsProviderAccountsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/provider-accounts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsBulkDestroyQuery {
}
/// Delete a list of provider account objects.

pub fn circuits_provider_accounts_bulk_destroy(query: CircuitsProviderAccountsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/provider-accounts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesRetrieveQuery {
}
/// Get a IP address object.

pub fn ipam_ip_addresses_retrieve(query: IpamIpAddressesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/ip-addresses/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesUpdateQuery {
}
/// Put a IP address object.

pub fn ipam_ip_addresses_update(query: IpamIpAddressesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/ip-addresses/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesPartialUpdateQuery {
}
/// Patch a IP address object.

pub fn ipam_ip_addresses_partial_update(query: IpamIpAddressesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/ip-addresses/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesDestroyQuery {
}
/// Delete a IP address object.

pub fn ipam_ip_addresses_destroy(query: IpamIpAddressesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/ip-addresses/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	size: Vec<i32>,
	size__empty: bool,
	size__gt: Vec<i32>,
	size__gte: Vec<i32>,
	size__lt: Vec<i32>,
	size__lte: Vec<i32>,
	size__n: Vec<i32>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_machine: Vec<String>,
	virtual_machine__n: Vec<String>,
	virtual_machine_id: Vec<i64>,
	virtual_machine_id__n: Vec<i64>
}
/// Get a list of virtual disk objects.

pub fn virtualization_virtual_disks_list(query: VirtualizationVirtualDisksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/virtual-disks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksBulkUpdateQuery {
}
/// Put a list of virtual disk objects.

pub fn virtualization_virtual_disks_bulk_update(query: VirtualizationVirtualDisksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/virtual-disks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksCreateQuery {
}
/// Post a list of virtual disk objects.

pub fn virtualization_virtual_disks_create(query: VirtualizationVirtualDisksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/virtual-disks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksBulkPartialUpdateQuery {
}
/// Patch a list of virtual disk objects.

pub fn virtualization_virtual_disks_bulk_partial_update(query: VirtualizationVirtualDisksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/virtual-disks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualDisksBulkDestroyQuery {
}
/// Delete a list of virtual disk objects.

pub fn virtualization_virtual_disks_bulk_destroy(query: VirtualizationVirtualDisksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/virtual-disks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesListQuery {
	cluster: Vec<String>,
	cluster__n: Vec<String>,
	cluster_group: Vec<String>,
	cluster_group__n: Vec<String>,
	cluster_group_id: Vec<i64>,
	cluster_group_id__n: Vec<i64>,
	cluster_id: Vec<i64>,
	cluster_id__n: Vec<i64>,
	cluster_typ: Vec<String>,
	cluster_typ__n: Vec<String>,
	cluster_typ_id: Vec<i64>,
	cluster_typ_id__n: Vec<i64>,
	config_template_id: Vec<i64>,
	config_template_id__n: Vec<i64>,
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	disk: Vec<i32>,
	disk__empty: bool,
	disk__gt: Vec<i32>,
	disk__gte: Vec<i32>,
	disk__lt: Vec<i32>,
	disk__lte: Vec<i32>,
	disk__n: Vec<i32>,
	has_primary_ip: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	local_context_data: bool,
	mac_address: Vec<String>,
	mac_address__ic: Vec<String>,
	mac_address__ie: Vec<String>,
	mac_address__iew: Vec<String>,
	mac_address__isw: Vec<String>,
	mac_address__n: Vec<String>,
	mac_address__nic: Vec<String>,
	mac_address__nie: Vec<String>,
	mac_address__niew: Vec<String>,
	mac_address__nisw: Vec<String>,
	memory: Vec<i32>,
	memory__empty: bool,
	memory__gt: Vec<i32>,
	memory__gte: Vec<i32>,
	memory__lt: Vec<i32>,
	memory__lte: Vec<i32>,
	memory__n: Vec<i32>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	platform: Vec<String>,
	platform__n: Vec<String>,
	platform_id: Vec<i64>,
	platform_id__n: Vec<i64>,
	primary_ip4_id: Vec<i64>,
	primary_ip4_id__n: Vec<i64>,
	primary_ip6_id: Vec<i64>,
	primary_ip6_id__n: Vec<i64>,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	vcpus: Vec<f64>,
	vcpus__empty: bool,
	vcpus__gt: Vec<f64>,
	vcpus__gte: Vec<f64>,
	vcpus__lt: Vec<f64>,
	vcpus__lte: Vec<f64>,
	vcpus__n: Vec<f64>
}
/// Get a list of virtual machine objects.

pub fn virtualization_virtual_machines_list(query: VirtualizationVirtualMachinesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/virtual-machines/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesBulkUpdateQuery {
}
/// Put a list of virtual machine objects.

pub fn virtualization_virtual_machines_bulk_update(query: VirtualizationVirtualMachinesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/virtual-machines/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesCreateQuery {
}
/// Post a list of virtual machine objects.

pub fn virtualization_virtual_machines_create(query: VirtualizationVirtualMachinesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/virtual-machines/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesBulkPartialUpdateQuery {
}
/// Patch a list of virtual machine objects.

pub fn virtualization_virtual_machines_bulk_partial_update(query: VirtualizationVirtualMachinesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/virtual-machines/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesBulkDestroyQuery {
}
/// Delete a list of virtual machine objects.

pub fn virtualization_virtual_machines_bulk_destroy(query: VirtualizationVirtualMachinesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/virtual-machines/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsListQuery {
	amperage: Vec<i32>,
	amperage__empty: bool,
	amperage__gt: Vec<i32>,
	amperage__gte: Vec<i32>,
	amperage__lt: Vec<i32>,
	amperage__lte: Vec<i32>,
	amperage__n: Vec<i32>,
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	connected: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	max_utilization: Vec<i32>,
	max_utilization__empty: bool,
	max_utilization__gt: Vec<i32>,
	max_utilization__gte: Vec<i32>,
	max_utilization__lt: Vec<i32>,
	max_utilization__lte: Vec<i32>,
	max_utilization__n: Vec<i32>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	phase: String,
	phase__n: String,
	power_panel_id: Vec<i64>,
	power_panel_id__n: Vec<i64>,
	q: String,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	supply: String,
	supply__n: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	typ: String,
	typ__n: String,
	updated_by_request: String,
	voltage: Vec<i32>,
	voltage__empty: bool,
	voltage__gt: Vec<i32>,
	voltage__gte: Vec<i32>,
	voltage__lt: Vec<i32>,
	voltage__lte: Vec<i32>,
	voltage__n: Vec<i32>
}
/// Get a list of power feed objects.

pub fn dcim_power_feeds_list(query: DcimPowerFeedsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-feeds/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsBulkUpdateQuery {
}
/// Put a list of power feed objects.

pub fn dcim_power_feeds_bulk_update(query: DcimPowerFeedsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-feeds/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsCreateQuery {
}
/// Post a list of power feed objects.

pub fn dcim_power_feeds_create(query: DcimPowerFeedsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/power-feeds/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsBulkPartialUpdateQuery {
}
/// Patch a list of power feed objects.

pub fn dcim_power_feeds_bulk_partial_update(query: DcimPowerFeedsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-feeds/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsBulkDestroyQuery {
}
/// Delete a list of power feed objects.

pub fn dcim_power_feeds_bulk_destroy(query: DcimPowerFeedsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-feeds/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesRetrieveQuery {
}
/// Get a front port template object.

pub fn dcim_front_port_templates_retrieve(query: DcimFrontPortTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/front-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesUpdateQuery {
}
/// Put a front port template object.

pub fn dcim_front_port_templates_update(query: DcimFrontPortTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/front-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesPartialUpdateQuery {
}
/// Patch a front port template object.

pub fn dcim_front_port_templates_partial_update(query: DcimFrontPortTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/front-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortTemplatesDestroyQuery {
}
/// Delete a front port template object.

pub fn dcim_front_port_templates_destroy(query: DcimFrontPortTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/front-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesSyncCreateQuery {
}
/// Provide a /sync API endpoint to synchronize an object's data from its associated DataFile (if any).

pub fn extras_config_templates_sync_create(query: ExtrasConfigTemplatesSyncCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/config-templates/{id}/sync/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesRetrieveQuery {
}
/// Get a module bay template object.

pub fn dcim_module_bay_templates_retrieve(query: DcimModuleBayTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/module-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesUpdateQuery {
}
/// Put a module bay template object.

pub fn dcim_module_bay_templates_update(query: DcimModuleBayTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/module-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesPartialUpdateQuery {
}
/// Patch a module bay template object.

pub fn dcim_module_bay_templates_partial_update(query: DcimModuleBayTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/module-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesDestroyQuery {
}
/// Delete a module bay template object.

pub fn dcim_module_bay_templates_destroy(query: DcimModuleBayTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/module-bay-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of power panel objects.

pub fn dcim_power_panels_list(query: DcimPowerPanelsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-panels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsBulkUpdateQuery {
}
/// Put a list of power panel objects.

pub fn dcim_power_panels_bulk_update(query: DcimPowerPanelsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-panels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsCreateQuery {
}
/// Post a list of power panel objects.

pub fn dcim_power_panels_create(query: DcimPowerPanelsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/power-panels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsBulkPartialUpdateQuery {
}
/// Patch a list of power panel objects.

pub fn dcim_power_panels_bulk_partial_update(query: DcimPowerPanelsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-panels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsBulkDestroyQuery {
}
/// Delete a list of power panel objects.

pub fn dcim_power_panels_bulk_destroy(query: DcimPowerPanelsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-panels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksListQuery {
	created: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	object_id: Vec<i32>,
	object_id__empty: bool,
	object_id__gt: Vec<i32>,
	object_id__gte: Vec<i32>,
	object_id__lt: Vec<i32>,
	object_id__lte: Vec<i32>,
	object_id__n: Vec<i32>,
	object_typ: String,
	object_typ__n: String,
	object_typ_id: Vec<i32>,
	object_typ_id__empty: Vec<i32>,
	object_typ_id__gt: Vec<i32>,
	object_typ_id__gte: Vec<i32>,
	object_typ_id__lt: Vec<i32>,
	object_typ_id__lte: Vec<i32>,
	object_typ_id__n: Vec<i32>,
	offset: i64,
	ordering: String,
	user: Vec<String>,
	user__n: Vec<String>,
	user_id: Vec<i64>,
	user_id__n: Vec<i64>
}
/// Get a list of bookmark objects.

pub fn extras_bookmarks_list(query: ExtrasBookmarksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/bookmarks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksBulkUpdateQuery {
}
/// Put a list of bookmark objects.

pub fn extras_bookmarks_bulk_update(query: ExtrasBookmarksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/bookmarks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksCreateQuery {
}
/// Post a list of bookmark objects.

pub fn extras_bookmarks_create(query: ExtrasBookmarksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/bookmarks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksBulkPartialUpdateQuery {
}
/// Patch a list of bookmark objects.

pub fn extras_bookmarks_bulk_partial_update(query: ExtrasBookmarksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/bookmarks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksBulkDestroyQuery {
}
/// Delete a list of bookmark objects.

pub fn extras_bookmarks_bulk_destroy(query: ExtrasBookmarksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/bookmarks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksRetrieveQuery {
}
/// Get a rack object.

pub fn dcim_racks_retrieve(query: DcimRacksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/racks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksUpdateQuery {
}
/// Put a rack object.

pub fn dcim_racks_update(query: DcimRacksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/racks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksPartialUpdateQuery {
}
/// Patch a rack object.

pub fn dcim_racks_partial_update(query: DcimRacksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/racks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksDestroyQuery {
}
/// Delete a rack object.

pub fn dcim_racks_destroy(query: DcimRacksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/racks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesAvailableIpsListQuery {
}
/// Get a IP address object.

pub fn ipam_ip_ranges_available_ips_list(query: IpamIpRangesAvailableIpsListQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/ip-ranges/{id}/available-ips/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesAvailableIpsCreateQuery {
}
/// Post a IP address object.

pub fn ipam_ip_ranges_available_ips_create(query: IpamIpRangesAvailableIpsCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/ip-ranges/{id}/available-ips/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	encapsulation: Vec<String>,
	encapsulation__n: Vec<String>,
	group: Vec<String>,
	group__n: Vec<String>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	ipsec_profile: Vec<String>,
	ipsec_profile__n: Vec<String>,
	ipsec_profile_id: Vec<i64>,
	ipsec_profile_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	tunnel_id: Vec<i32>,
	tunnel_id__empty: bool,
	tunnel_id__gt: Vec<i32>,
	tunnel_id__gte: Vec<i32>,
	tunnel_id__lt: Vec<i32>,
	tunnel_id__lte: Vec<i32>,
	tunnel_id__n: Vec<i32>,
	updated_by_request: String
}
/// Get a list of tunnel objects.

pub fn vpn_tunnels_list(query: VpnTunnelsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/tunnels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsBulkUpdateQuery {
}
/// Put a list of tunnel objects.

pub fn vpn_tunnels_bulk_update(query: VpnTunnelsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/tunnels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsCreateQuery {
}
/// Post a list of tunnel objects.

pub fn vpn_tunnels_create(query: VpnTunnelsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/tunnels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsBulkPartialUpdateQuery {
}
/// Patch a list of tunnel objects.

pub fn vpn_tunnels_bulk_partial_update(query: VpnTunnelsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/tunnels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsBulkDestroyQuery {
}
/// Delete a list of tunnel objects.

pub fn vpn_tunnels_bulk_destroy(query: VpnTunnelsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/tunnels/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsListQuery {
	asn: Vec<i32>,
	asn__empty: bool,
	asn__gt: Vec<i32>,
	asn__gte: Vec<i32>,
	asn__lt: Vec<i32>,
	asn__lte: Vec<i32>,
	asn__n: Vec<i32>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	q: String,
	rir: Vec<String>,
	rir__n: Vec<String>,
	rir_id: Vec<i64>,
	rir_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of ASN objects.

pub fn ipam_asns_list(query: IpamAsnsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsBulkUpdateQuery {
}
/// Put a list of ASN objects.

pub fn ipam_asns_bulk_update(query: IpamAsnsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsCreateQuery {
}
/// Post a list of ASN objects.

pub fn ipam_asns_create(query: IpamAsnsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsBulkPartialUpdateQuery {
}
/// Patch a list of ASN objects.

pub fn ipam_asns_bulk_partial_update(query: IpamAsnsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamAsnsBulkDestroyQuery {
}
/// Delete a list of ASN objects.

pub fn ipam_asns_bulk_destroy(query: IpamAsnsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/asns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansListQuery {
	auth_cipher: Vec<String>,
	auth_cipher__n: Vec<String>,
	auth_psk: Vec<String>,
	auth_psk__empty: bool,
	auth_psk__ic: Vec<String>,
	auth_psk__ie: Vec<String>,
	auth_psk__iew: Vec<String>,
	auth_psk__isw: Vec<String>,
	auth_psk__n: Vec<String>,
	auth_psk__nic: Vec<String>,
	auth_psk__nie: Vec<String>,
	auth_psk__niew: Vec<String>,
	auth_psk__nisw: Vec<String>,
	auth_typ: Vec<String>,
	auth_typ__n: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	group: Vec<i64>,
	group__n: Vec<i64>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	q: String,
	ssid: Vec<String>,
	ssid__empty: bool,
	ssid__ic: Vec<String>,
	ssid__ie: Vec<String>,
	ssid__iew: Vec<String>,
	ssid__isw: Vec<String>,
	ssid__n: Vec<String>,
	ssid__nic: Vec<String>,
	ssid__nie: Vec<String>,
	ssid__niew: Vec<String>,
	ssid__nisw: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	vlan_id: Vec<i64>,
	vlan_id__n: Vec<i64>
}
/// Get a list of wireless LAN objects.

pub fn wireless_wireless_lans_list(query: WirelessWirelessLansListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/wireless/wireless-lans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansBulkUpdateQuery {
}
/// Put a list of wireless LAN objects.

pub fn wireless_wireless_lans_bulk_update(query: WirelessWirelessLansBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/wireless/wireless-lans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansCreateQuery {
}
/// Post a list of wireless LAN objects.

pub fn wireless_wireless_lans_create(query: WirelessWirelessLansCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/wireless/wireless-lans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansBulkPartialUpdateQuery {
}
/// Patch a list of wireless LAN objects.

pub fn wireless_wireless_lans_bulk_partial_update(query: WirelessWirelessLansBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/wireless/wireless-lans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansBulkDestroyQuery {
}
/// Delete a list of wireless LAN objects.

pub fn wireless_wireless_lans_bulk_destroy(query: WirelessWirelessLansBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/wireless/wireless-lans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksListQuery {
	asset_tag: Vec<String>,
	asset_tag__empty: bool,
	asset_tag__ic: Vec<String>,
	asset_tag__ie: Vec<String>,
	asset_tag__iew: Vec<String>,
	asset_tag__isw: Vec<String>,
	asset_tag__n: Vec<String>,
	asset_tag__nic: Vec<String>,
	asset_tag__nie: Vec<String>,
	asset_tag__niew: Vec<String>,
	asset_tag__nisw: Vec<String>,
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	desc_units: bool,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	facility_id: Vec<String>,
	facility_id__empty: bool,
	facility_id__ic: Vec<String>,
	facility_id__ie: Vec<String>,
	facility_id__iew: Vec<String>,
	facility_id__isw: Vec<String>,
	facility_id__n: Vec<String>,
	facility_id__nic: Vec<String>,
	facility_id__nie: Vec<String>,
	facility_id__niew: Vec<String>,
	facility_id__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<i64>,
	location__n: Vec<i64>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	max_weight: Vec<i32>,
	max_weight__empty: bool,
	max_weight__gt: Vec<i32>,
	max_weight__gte: Vec<i32>,
	max_weight__lt: Vec<i32>,
	max_weight__lte: Vec<i32>,
	max_weight__n: Vec<i32>,
	modified_by_request: String,
	mounting_depth: Vec<i32>,
	mounting_depth__empty: bool,
	mounting_depth__gt: Vec<i32>,
	mounting_depth__gte: Vec<i32>,
	mounting_depth__lt: Vec<i32>,
	mounting_depth__lte: Vec<i32>,
	mounting_depth__n: Vec<i32>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	outer_depth: Vec<i32>,
	outer_depth__empty: bool,
	outer_depth__gt: Vec<i32>,
	outer_depth__gte: Vec<i32>,
	outer_depth__lt: Vec<i32>,
	outer_depth__lte: Vec<i32>,
	outer_depth__n: Vec<i32>,
	outer_unit: String,
	outer_unit__n: String,
	outer_width: Vec<i32>,
	outer_width__empty: bool,
	outer_width__gt: Vec<i32>,
	outer_width__gte: Vec<i32>,
	outer_width__lt: Vec<i32>,
	outer_width__lte: Vec<i32>,
	outer_width__n: Vec<i32>,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	serial: Vec<String>,
	serial__empty: bool,
	serial__ic: Vec<String>,
	serial__ie: Vec<String>,
	serial__iew: Vec<String>,
	serial__isw: Vec<String>,
	serial__n: Vec<String>,
	serial__nic: Vec<String>,
	serial__nie: Vec<String>,
	serial__niew: Vec<String>,
	serial__nisw: Vec<String>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	starting_unit: Vec<i32>,
	starting_unit__empty: bool,
	starting_unit__gt: Vec<i32>,
	starting_unit__gte: Vec<i32>,
	starting_unit__lt: Vec<i32>,
	starting_unit__lte: Vec<i32>,
	starting_unit__n: Vec<i32>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	u_height: Vec<i32>,
	u_height__empty: bool,
	u_height__gt: Vec<i32>,
	u_height__gte: Vec<i32>,
	u_height__lt: Vec<i32>,
	u_height__lte: Vec<i32>,
	u_height__n: Vec<i32>,
	updated_by_request: String,
	weight: Vec<f64>,
	weight__empty: bool,
	weight__gt: Vec<f64>,
	weight__gte: Vec<f64>,
	weight__lt: Vec<f64>,
	weight__lte: Vec<f64>,
	weight__n: Vec<f64>,
	weight_unit: String,
	weight_unit__n: String,
	width: Vec<i64>,
	width__n: Vec<i64>
}
/// Get a list of rack objects.

pub fn dcim_racks_list(query: DcimRacksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/racks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksBulkUpdateQuery {
}
/// Put a list of rack objects.

pub fn dcim_racks_bulk_update(query: DcimRacksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/racks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksCreateQuery {
}
/// Post a list of rack objects.

pub fn dcim_racks_create(query: DcimRacksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/racks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksBulkPartialUpdateQuery {
}
/// Patch a list of rack objects.

pub fn dcim_racks_bulk_partial_update(query: DcimRacksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/racks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksBulkDestroyQuery {
}
/// Delete a list of rack objects.

pub fn dcim_racks_bulk_destroy(query: DcimRacksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/racks/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsListQuery {
	choice_set: Vec<String>,
	choice_set__n: Vec<String>,
	choice_set_id: Vec<i64>,
	choice_set_id__n: Vec<i64>,
	content_typ_id: Vec<i32>,
	content_typ_id__empty: Vec<i32>,
	content_typ_id__gt: Vec<i32>,
	content_typ_id__gte: Vec<i32>,
	content_typ_id__lt: Vec<i32>,
	content_typ_id__lte: Vec<i32>,
	content_typ_id__n: Vec<i32>,
	content_typs: String,
	content_typs__ic: String,
	content_typs__ie: String,
	content_typs__iew: String,
	content_typs__isw: String,
	content_typs__n: String,
	content_typs__nic: String,
	content_typs__nie: String,
	content_typs__niew: String,
	content_typs__nisw: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	filter_logic: String,
	filter_logic__n: String,
	group_name: Vec<String>,
	group_name__empty: bool,
	group_name__ic: Vec<String>,
	group_name__ie: Vec<String>,
	group_name__iew: Vec<String>,
	group_name__isw: Vec<String>,
	group_name__n: Vec<String>,
	group_name__nic: Vec<String>,
	group_name__nie: Vec<String>,
	group_name__niew: Vec<String>,
	group_name__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	is_cloneable: bool,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	required: bool,
	search_weight: Vec<i32>,
	search_weight__empty: bool,
	search_weight__gt: Vec<i32>,
	search_weight__gte: Vec<i32>,
	search_weight__lt: Vec<i32>,
	search_weight__lte: Vec<i32>,
	search_weight__n: Vec<i32>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	ui_editable: String,
	ui_editable__n: String,
	ui_visible: String,
	ui_visible__n: String,
	weight: Vec<i32>,
	weight__empty: bool,
	weight__gt: Vec<i32>,
	weight__gte: Vec<i32>,
	weight__lt: Vec<i32>,
	weight__lte: Vec<i32>,
	weight__n: Vec<i32>
}
/// Get a list of custom field objects.

pub fn extras_custom_fields_list(query: ExtrasCustomFieldsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-fields/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsBulkUpdateQuery {
}
/// Put a list of custom field objects.

pub fn extras_custom_fields_bulk_update(query: ExtrasCustomFieldsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/custom-fields/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsCreateQuery {
}
/// Post a list of custom field objects.

pub fn extras_custom_fields_create(query: ExtrasCustomFieldsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/custom-fields/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsBulkPartialUpdateQuery {
}
/// Patch a list of custom field objects.

pub fn extras_custom_fields_bulk_partial_update(query: ExtrasCustomFieldsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/custom-fields/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldsBulkDestroyQuery {
}
/// Delete a list of custom field objects.

pub fn extras_custom_fields_bulk_destroy(query: ExtrasCustomFieldsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/custom-fields/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensRetrieveQuery {
}
/// Get a token object.

pub fn users_tokens_retrieve(query: UsersTokensRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/tokens/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensUpdateQuery {
}
/// Put a token object.

pub fn users_tokens_update(query: UsersTokensUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/tokens/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensPartialUpdateQuery {
}
/// Patch a token object.

pub fn users_tokens_partial_update(query: UsersTokensPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/tokens/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensDestroyQuery {
}
/// Delete a token object.

pub fn users_tokens_destroy(query: UsersTokensDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/tokens/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesListQuery {
	bridge_id: Vec<i64>,
	bridge_id__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	enabled: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mgmt_only: bool,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	poe_mode: Vec<String>,
	poe_mode__n: Vec<String>,
	poe_typ: Vec<String>,
	poe_typ__n: Vec<String>,
	q: String,
	rf_role: Vec<String>,
	rf_role__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of interface template objects.

pub fn dcim_interface_templates_list(query: DcimInterfaceTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/interface-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesBulkUpdateQuery {
}
/// Put a list of interface template objects.

pub fn dcim_interface_templates_bulk_update(query: DcimInterfaceTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/interface-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesCreateQuery {
}
/// Post a list of interface template objects.

pub fn dcim_interface_templates_create(query: DcimInterfaceTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/interface-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of interface template objects.

pub fn dcim_interface_templates_bulk_partial_update(query: DcimInterfaceTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/interface-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesBulkDestroyQuery {
}
/// Delete a list of interface template objects.

pub fn dcim_interface_templates_bulk_destroy(query: DcimInterfaceTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/interface-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsRetrieveQuery {
}
/// Get a rack reservation object.

pub fn dcim_rack_reservations_retrieve(query: DcimRackReservationsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rack-reservations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsUpdateQuery {
}
/// Put a rack reservation object.

pub fn dcim_rack_reservations_update(query: DcimRackReservationsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rack-reservations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsPartialUpdateQuery {
}
/// Patch a rack reservation object.

pub fn dcim_rack_reservations_partial_update(query: DcimRackReservationsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rack-reservations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsDestroyQuery {
}
/// Delete a rack reservation object.

pub fn dcim_rack_reservations_destroy(query: DcimRackReservationsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rack-reservations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsListQuery {
	authentication_algorithm: Vec<String>,
	authentication_algorithm__n: Vec<String>,
	authentication_method: Vec<String>,
	authentication_method__n: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	encryption_algorithm: Vec<String>,
	encryption_algorithm__n: Vec<String>,
	group: Vec<i64>,
	group__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	sa_lifetime: Vec<i32>,
	sa_lifetime__empty: bool,
	sa_lifetime__gt: Vec<i32>,
	sa_lifetime__gte: Vec<i32>,
	sa_lifetime__lt: Vec<i32>,
	sa_lifetime__lte: Vec<i32>,
	sa_lifetime__n: Vec<i32>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of IKE proposal objects.

pub fn vpn_ike_proposals_list(query: VpnIkeProposalsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ike-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsBulkUpdateQuery {
}
/// Put a list of IKE proposal objects.

pub fn vpn_ike_proposals_bulk_update(query: VpnIkeProposalsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ike-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsCreateQuery {
}
/// Post a list of IKE proposal objects.

pub fn vpn_ike_proposals_create(query: VpnIkeProposalsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/ike-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsBulkPartialUpdateQuery {
}
/// Patch a list of IKE proposal objects.

pub fn vpn_ike_proposals_bulk_partial_update(query: VpnIkeProposalsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ike-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkeProposalsBulkDestroyQuery {
}
/// Delete a list of IKE proposal objects.

pub fn vpn_ike_proposals_bulk_destroy(query: VpnIkeProposalsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ike-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	port: f64,
	protocol: String,
	protocol__n: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of service template objects.

pub fn ipam_service_templates_list(query: IpamServiceTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/service-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesBulkUpdateQuery {
}
/// Put a list of service template objects.

pub fn ipam_service_templates_bulk_update(query: IpamServiceTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/service-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesCreateQuery {
}
/// Post a list of service template objects.

pub fn ipam_service_templates_create(query: IpamServiceTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/service-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of service template objects.

pub fn ipam_service_templates_bulk_partial_update(query: IpamServiceTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/service-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesBulkDestroyQuery {
}
/// Delete a list of service template objects.

pub fn ipam_service_templates_bulk_destroy(query: IpamServiceTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/service-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsRetrieveQuery {
}
/// Get a circuit termination object.

pub fn circuits_circuit_terminations_retrieve(query: CircuitsCircuitTerminationsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuit-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsUpdateQuery {
}
/// Put a circuit termination object.

pub fn circuits_circuit_terminations_update(query: CircuitsCircuitTerminationsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/circuit-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsPartialUpdateQuery {
}
/// Patch a circuit termination object.

pub fn circuits_circuit_terminations_partial_update(query: CircuitsCircuitTerminationsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/circuit-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsDestroyQuery {
}
/// Delete a circuit termination object.

pub fn circuits_circuit_terminations_destroy(query: CircuitsCircuitTerminationsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/circuit-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesRetrieveQuery {
}
/// Get a console port template object.

pub fn dcim_console_port_templates_retrieve(query: DcimConsolePortTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesUpdateQuery {
}
/// Put a console port template object.

pub fn dcim_console_port_templates_update(query: DcimConsolePortTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesPartialUpdateQuery {
}
/// Patch a console port template object.

pub fn dcim_console_port_templates_partial_update(query: DcimConsolePortTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesDestroyQuery {
}
/// Delete a console port template object.

pub fn dcim_console_port_templates_destroy(query: DcimConsolePortTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksRetrieveQuery {
}
/// Get a custom link object.

pub fn extras_custom_links_retrieve(query: ExtrasCustomLinksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksUpdateQuery {
}
/// Put a custom link object.

pub fn extras_custom_links_update(query: ExtrasCustomLinksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/custom-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksPartialUpdateQuery {
}
/// Patch a custom link object.

pub fn extras_custom_links_partial_update(query: ExtrasCustomLinksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/custom-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksDestroyQuery {
}
/// Delete a custom link object.

pub fn extras_custom_links_destroy(query: ExtrasCustomLinksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/custom-links/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesListQuery {
	address: Vec<String>,
	assigned: bool,
	assigned_to_interface: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device_id: Vec<i32>,
	dns_name: Vec<String>,
	dns_name__empty: bool,
	dns_name__ic: Vec<String>,
	dns_name__ie: Vec<String>,
	dns_name__iew: Vec<String>,
	dns_name__isw: Vec<String>,
	dns_name__n: Vec<String>,
	dns_name__nic: Vec<String>,
	dns_name__nie: Vec<String>,
	dns_name__niew: Vec<String>,
	dns_name__nisw: Vec<String>,
	family: f64,
	fhrpgroup_id: Vec<i64>,
	fhrpgroup_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interface: Vec<String>,
	interface__n: Vec<String>,
	interface_id: Vec<i64>,
	interface_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mask_length: Vec<i32>,
	mask_length__gte: f64,
	mask_length__lte: f64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	present_in_vrf: String,
	present_in_vrf_id: String,
	q: String,
	role: Vec<String>,
	role__n: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	virtual_machine: Vec<String>,
	virtual_machine_id: Vec<i32>,
	vminterface: Vec<String>,
	vminterface__n: Vec<String>,
	vminterface_id: Vec<i64>,
	vminterface_id__n: Vec<i64>,
	vrf: Vec<String>,
	vrf__n: Vec<String>,
	vrf_id: Vec<i64>,
	vrf_id__n: Vec<i64>
}
/// Get a list of IP address objects.

pub fn ipam_ip_addresses_list(query: IpamIpAddressesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/ip-addresses/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesBulkUpdateQuery {
}
/// Put a list of IP address objects.

pub fn ipam_ip_addresses_bulk_update(query: IpamIpAddressesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/ip-addresses/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesCreateQuery {
}
/// Post a list of IP address objects.

pub fn ipam_ip_addresses_create(query: IpamIpAddressesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/ip-addresses/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesBulkPartialUpdateQuery {
}
/// Patch a list of IP address objects.

pub fn ipam_ip_addresses_bulk_partial_update(query: IpamIpAddressesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/ip-addresses/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpAddressesBulkDestroyQuery {
}
/// Delete a list of IP address objects.

pub fn ipam_ip_addresses_bulk_destroy(query: IpamIpAddressesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/ip-addresses/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesRetrieveQuery {
}
/// Get a config template object.

pub fn extras_config_templates_retrieve(query: ExtrasConfigTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/config-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesUpdateQuery {
}
/// Put a config template object.

pub fn extras_config_templates_update(query: ExtrasConfigTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/config-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesPartialUpdateQuery {
}
/// Patch a config template object.

pub fn extras_config_templates_partial_update(query: ExtrasConfigTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/config-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesDestroyQuery {
}
/// Delete a config template object.

pub fn extras_config_templates_destroy(query: ExtrasConfigTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/config-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsRetrieveQuery {
}
/// Get a tunnel object.

pub fn vpn_tunnels_retrieve(query: VpnTunnelsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/tunnels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsUpdateQuery {
}
/// Put a tunnel object.

pub fn vpn_tunnels_update(query: VpnTunnelsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/tunnels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsPartialUpdateQuery {
}
/// Patch a tunnel object.

pub fn vpn_tunnels_partial_update(query: VpnTunnelsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/tunnels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelsDestroyQuery {
}
/// Delete a tunnel object.

pub fn vpn_tunnels_destroy(query: VpnTunnelsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/tunnels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsRetrieveQuery {
}
/// Get a tag object.

pub fn extras_tags_retrieve(query: ExtrasTagsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/tags/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsUpdateQuery {
}
/// Put a tag object.

pub fn extras_tags_update(query: ExtrasTagsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/tags/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsPartialUpdateQuery {
}
/// Patch a tag object.

pub fn extras_tags_partial_update(query: ExtrasTagsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/tags/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsDestroyQuery {
}
/// Delete a tag object.

pub fn extras_tags_destroy(query: ExtrasTagsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/tags/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksRetrieveQuery {
}
/// Get a webhook object.

pub fn extras_webhooks_retrieve(query: ExtrasWebhooksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/webhooks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksUpdateQuery {
}
/// Put a webhook object.

pub fn extras_webhooks_update(query: ExtrasWebhooksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/webhooks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksPartialUpdateQuery {
}
/// Patch a webhook object.

pub fn extras_webhooks_partial_update(query: ExtrasWebhooksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/webhooks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasWebhooksDestroyQuery {
}
/// Delete a webhook object.

pub fn extras_webhooks_destroy(query: ExtrasWebhooksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/webhooks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesRetrieveQuery {
}
/// Get a device object.

pub fn dcim_devices_retrieve(query: DcimDevicesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/devices/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesUpdateQuery {
}
/// Put a device object.

pub fn dcim_devices_update(query: DcimDevicesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/devices/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesPartialUpdateQuery {
}
/// Patch a device object.

pub fn dcim_devices_partial_update(query: DcimDevicesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/devices/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesDestroyQuery {
}
/// Delete a device object.

pub fn dcim_devices_destroy(query: DcimDevicesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/devices/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsListQuery {
	cluster: i64,
	clustergroup: f64,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: i64,
	max_vid: Vec<i32>,
	max_vid__empty: bool,
	max_vid__gt: Vec<i32>,
	max_vid__gte: Vec<i32>,
	max_vid__lt: Vec<i32>,
	max_vid__lte: Vec<i32>,
	max_vid__n: Vec<i32>,
	min_vid: Vec<i32>,
	min_vid__empty: bool,
	min_vid__gt: Vec<i32>,
	min_vid__gte: Vec<i32>,
	min_vid__lt: Vec<i32>,
	min_vid__lte: Vec<i32>,
	min_vid__n: Vec<i32>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	rack: i64,
	region: i64,
	scope_id: Vec<i32>,
	scope_id__empty: bool,
	scope_id__gt: Vec<i32>,
	scope_id__gte: Vec<i32>,
	scope_id__lt: Vec<i32>,
	scope_id__lte: Vec<i32>,
	scope_id__n: Vec<i32>,
	scope_typ: String,
	scope_typ__n: String,
	site: i64,
	sitegroup: f64,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of VLAN group objects.

pub fn ipam_vlan_groups_list(query: IpamVlanGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vlan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsBulkUpdateQuery {
}
/// Put a list of VLAN group objects.

pub fn ipam_vlan_groups_bulk_update(query: IpamVlanGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/vlan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsCreateQuery {
}
/// Post a list of VLAN group objects.

pub fn ipam_vlan_groups_create(query: IpamVlanGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/vlan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsBulkPartialUpdateQuery {
}
/// Patch a list of VLAN group objects.

pub fn ipam_vlan_groups_bulk_partial_update(query: IpamVlanGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/vlan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsBulkDestroyQuery {
}
/// Delete a list of VLAN group objects.

pub fn ipam_vlan_groups_bulk_destroy(query: IpamVlanGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/vlan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	parent__n: Vec<String>,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of region objects.

pub fn dcim_regions_list(query: DcimRegionsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/regions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsBulkUpdateQuery {
}
/// Put a list of region objects.

pub fn dcim_regions_bulk_update(query: DcimRegionsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/regions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsCreateQuery {
}
/// Post a list of region objects.

pub fn dcim_regions_create(query: DcimRegionsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/regions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsBulkPartialUpdateQuery {
}
/// Patch a list of region objects.

pub fn dcim_regions_bulk_partial_update(query: DcimRegionsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/regions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsBulkDestroyQuery {
}
/// Delete a list of region objects.

pub fn dcim_regions_bulk_destroy(query: DcimRegionsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/regions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsRetrieveQuery {
}
/// Get a region object.

pub fn dcim_regions_retrieve(query: DcimRegionsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/regions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsUpdateQuery {
}
/// Put a region object.

pub fn dcim_regions_update(query: DcimRegionsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/regions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsPartialUpdateQuery {
}
/// Patch a region object.

pub fn dcim_regions_partial_update(query: DcimRegionsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/regions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRegionsDestroyQuery {
}
/// Delete a region object.

pub fn dcim_regions_destroy(query: DcimRegionsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/regions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	typ: String,
	typ__n: String,
	updated_by_request: String
}
/// Get a list of console server port template objects.

pub fn dcim_console_server_port_templates_list(query: DcimConsoleServerPortTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-server-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesBulkUpdateQuery {
}
/// Put a list of console server port template objects.

pub fn dcim_console_server_port_templates_bulk_update(query: DcimConsoleServerPortTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-server-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesCreateQuery {
}
/// Post a list of console server port template objects.

pub fn dcim_console_server_port_templates_create(query: DcimConsoleServerPortTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/console-server-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of console server port template objects.

pub fn dcim_console_server_port_templates_bulk_partial_update(query: DcimConsoleServerPortTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-server-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesBulkDestroyQuery {
}
/// Delete a list of console server port template objects.

pub fn dcim_console_server_port_templates_bulk_destroy(query: DcimConsoleServerPortTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-server-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsRetrieveQuery {
}
/// Get a console server port object.

pub fn dcim_console_server_ports_retrieve(query: DcimConsoleServerPortsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-server-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsUpdateQuery {
}
/// Put a console server port object.

pub fn dcim_console_server_ports_update(query: DcimConsoleServerPortsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-server-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsPartialUpdateQuery {
}
/// Patch a console server port object.

pub fn dcim_console_server_ports_partial_update(query: DcimConsoleServerPortsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-server-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsDestroyQuery {
}
/// Delete a console server port object.

pub fn dcim_console_server_ports_destroy(query: DcimConsoleServerPortsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-server-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesRetrieveQuery {
}
/// Get a data source object.

pub fn core_data_sources_retrieve(query: CoreDataSourcesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/core/data-sources/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesUpdateQuery {
}
/// Put a data source object.

pub fn core_data_sources_update(query: CoreDataSourcesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/core/data-sources/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesPartialUpdateQuery {
}
/// Patch a data source object.

pub fn core_data_sources_partial_update(query: CoreDataSourcesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/core/data-sources/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataSourcesDestroyQuery {
}
/// Delete a data source object.

pub fn core_data_sources_destroy(query: CoreDataSourcesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/core/data-sources/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsRetrieveQuery {
}
/// Get a wireless LAN group object.

pub fn wireless_wireless_lan_groups_retrieve(query: WirelessWirelessLanGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/wireless/wireless-lan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsUpdateQuery {
}
/// Put a wireless LAN group object.

pub fn wireless_wireless_lan_groups_update(query: WirelessWirelessLanGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/wireless/wireless-lan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsPartialUpdateQuery {
}
/// Patch a wireless LAN group object.

pub fn wireless_wireless_lan_groups_partial_update(query: WirelessWirelessLanGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/wireless/wireless-lan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsDestroyQuery {
}
/// Delete a wireless LAN group object.

pub fn wireless_wireless_lan_groups_destroy(query: WirelessWirelessLanGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/wireless/wireless-lan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsRetrieveQuery {
}
/// Get a group object.

pub fn users_groups_retrieve(query: UsersGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsUpdateQuery {
}
/// Put a group object.

pub fn users_groups_update(query: UsersGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsPartialUpdateQuery {
}
/// Patch a group object.

pub fn users_groups_partial_update(query: UsersGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersGroupsDestroyQuery {
}
/// Delete a group object.

pub fn users_groups_destroy(query: UsersGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsRetrieveQuery {
}
/// Get a route target object.

pub fn ipam_route_targets_retrieve(query: IpamRouteTargetsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/route-targets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsUpdateQuery {
}
/// Put a route target object.

pub fn ipam_route_targets_update(query: IpamRouteTargetsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/route-targets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsPartialUpdateQuery {
}
/// Patch a route target object.

pub fn ipam_route_targets_partial_update(query: IpamRouteTargetsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/route-targets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRouteTargetsDestroyQuery {
}
/// Delete a route target object.

pub fn ipam_route_targets_destroy(query: IpamRouteTargetsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/route-targets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRacksElevationRetrieveQuery {
}
/// Rack elevation representing the list of rack units. Also supports rendering the elevation as an SVG.

pub fn dcim_racks_elevation_retrieve(query: DcimRacksElevationRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/racks/{id}/elevation/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersRetrieveQuery {
}
/// Get a manufacturer object.

pub fn dcim_manufacturers_retrieve(query: DcimManufacturersRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/manufacturers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersUpdateQuery {
}
/// Put a manufacturer object.

pub fn dcim_manufacturers_update(query: DcimManufacturersUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/manufacturers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersPartialUpdateQuery {
}
/// Patch a manufacturer object.

pub fn dcim_manufacturers_partial_update(query: DcimManufacturersPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/manufacturers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimManufacturersDestroyQuery {
}
/// Delete a manufacturer object.

pub fn dcim_manufacturers_destroy(query: DcimManufacturersDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/manufacturers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasObjectChangesListQuery {
	action: String,
	action__n: String,
	changed_object_id: Vec<i32>,
	changed_object_id__empty: bool,
	changed_object_id__gt: Vec<i32>,
	changed_object_id__gte: Vec<i32>,
	changed_object_id__lt: Vec<i32>,
	changed_object_id__lte: Vec<i32>,
	changed_object_id__n: Vec<i32>,
	changed_object_typ: String,
	changed_object_typ__n: String,
	changed_object_typ_id: Vec<i64>,
	changed_object_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	object_repr: Vec<String>,
	object_repr__empty: bool,
	object_repr__ic: Vec<String>,
	object_repr__ie: Vec<String>,
	object_repr__iew: Vec<String>,
	object_repr__isw: Vec<String>,
	object_repr__n: Vec<String>,
	object_repr__nic: Vec<String>,
	object_repr__nie: Vec<String>,
	object_repr__niew: Vec<String>,
	object_repr__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	request_id: String,
	time_after: String,
	time_before: String,
	user: Vec<String>,
	user__n: Vec<String>,
	user_id: Vec<i64>,
	user_id__n: Vec<i64>,
	user_name: Vec<String>,
	user_name__empty: bool,
	user_name__ic: Vec<String>,
	user_name__ie: Vec<String>,
	user_name__iew: Vec<String>,
	user_name__isw: Vec<String>,
	user_name__n: Vec<String>,
	user_name__nic: Vec<String>,
	user_name__nie: Vec<String>,
	user_name__niew: Vec<String>,
	user_name__nisw: Vec<String>
}
/// Retrieve a list of recent changes.

pub fn extras_object_changes_list(query: ExtrasObjectChangesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/object-changes/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesRetrieveQuery {
}
/// Get a interface template object.

pub fn dcim_interface_templates_retrieve(query: DcimInterfaceTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/interface-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesUpdateQuery {
}
/// Put a interface template object.

pub fn dcim_interface_templates_update(query: DcimInterfaceTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/interface-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesPartialUpdateQuery {
}
/// Patch a interface template object.

pub fn dcim_interface_templates_partial_update(query: DcimInterfaceTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/interface-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfaceTemplatesDestroyQuery {
}
/// Delete a interface template object.

pub fn dcim_interface_templates_destroy(query: DcimInterfaceTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/interface-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesListQuery {
	data_file_id: Vec<i64>,
	data_file_id__n: Vec<i64>,
	data_source_id: Vec<i64>,
	data_source_id__n: Vec<i64>,
	data_synced: Vec<String>,
	data_synced__empty: bool,
	data_synced__gt: Vec<String>,
	data_synced__gte: Vec<String>,
	data_synced__lt: Vec<String>,
	data_synced__lte: Vec<String>,
	data_synced__n: Vec<String>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>
}
/// Get a list of config template objects.

pub fn extras_config_templates_list(query: ExtrasConfigTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/config-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesBulkUpdateQuery {
}
/// Put a list of config template objects.

pub fn extras_config_templates_bulk_update(query: ExtrasConfigTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/config-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesCreateQuery {
}
/// Post a list of config template objects.

pub fn extras_config_templates_create(query: ExtrasConfigTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/config-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of config template objects.

pub fn extras_config_templates_bulk_partial_update(query: ExtrasConfigTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/config-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesBulkDestroyQuery {
}
/// Delete a list of config template objects.

pub fn extras_config_templates_bulk_destroy(query: ExtrasConfigTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/config-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesListQuery {
	airflow: String,
	airflow__n: String,
	console_ports: bool,
	console_server_ports: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	default_platform: Vec<String>,
	default_platform__n: Vec<String>,
	default_platform_id: Vec<i64>,
	default_platform_id__n: Vec<i64>,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device_bays: bool,
	exclude_from_utilization: bool,
	has_front_image: bool,
	has_rear_image: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interfaces: bool,
	inventory_items: bool,
	is_full_depth: bool,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	model: Vec<String>,
	model__empty: bool,
	model__ic: Vec<String>,
	model__ie: Vec<String>,
	model__iew: Vec<String>,
	model__isw: Vec<String>,
	model__n: Vec<String>,
	model__nic: Vec<String>,
	model__nie: Vec<String>,
	model__niew: Vec<String>,
	model__nisw: Vec<String>,
	modified_by_request: String,
	module_bays: bool,
	offset: i64,
	ordering: String,
	part_number: Vec<String>,
	part_number__empty: bool,
	part_number__ic: Vec<String>,
	part_number__ie: Vec<String>,
	part_number__iew: Vec<String>,
	part_number__isw: Vec<String>,
	part_number__n: Vec<String>,
	part_number__nic: Vec<String>,
	part_number__nie: Vec<String>,
	part_number__niew: Vec<String>,
	part_number__nisw: Vec<String>,
	pass_through_ports: bool,
	power_outlets: bool,
	power_ports: bool,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	subdevice_role: String,
	subdevice_role__n: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	u_height: Vec<f64>,
	u_height__empty: bool,
	u_height__gt: Vec<f64>,
	u_height__gte: Vec<f64>,
	u_height__lt: Vec<f64>,
	u_height__lte: Vec<f64>,
	u_height__n: Vec<f64>,
	updated_by_request: String,
	weight: Vec<f64>,
	weight__empty: bool,
	weight__gt: Vec<f64>,
	weight__gte: Vec<f64>,
	weight__lt: Vec<f64>,
	weight__lte: Vec<f64>,
	weight__n: Vec<f64>,
	weight_unit: String,
	weight_unit__n: String
}
/// Get a list of device type objects.

pub fn dcim_device_types_list(query: DcimDeviceTypesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesBulkUpdateQuery {
}
/// Put a list of device type objects.

pub fn dcim_device_types_bulk_update(query: DcimDeviceTypesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesCreateQuery {
}
/// Post a list of device type objects.

pub fn dcim_device_types_create(query: DcimDeviceTypesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/device-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesBulkPartialUpdateQuery {
}
/// Patch a list of device type objects.

pub fn dcim_device_types_bulk_partial_update(query: DcimDeviceTypesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesBulkDestroyQuery {
}
/// Delete a list of device type objects.

pub fn dcim_device_types_bulk_destroy(query: DcimDeviceTypesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsRetrieveQuery {
}
/// Get a config context object.

pub fn extras_config_contexts_retrieve(query: ExtrasConfigContextsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/config-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsUpdateQuery {
}
/// Put a config context object.

pub fn extras_config_contexts_update(query: ExtrasConfigContextsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/config-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsPartialUpdateQuery {
}
/// Patch a config context object.

pub fn extras_config_contexts_partial_update(query: ExtrasConfigContextsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/config-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigContextsDestroyQuery {
}
/// Delete a config context object.

pub fn extras_config_contexts_destroy(query: ExtrasConfigContextsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/config-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysRetrieveQuery {
}
/// Get a device bay object.

pub fn dcim_device_bays_retrieve(query: DcimDeviceBaysRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysUpdateQuery {
}
/// Put a device bay object.

pub fn dcim_device_bays_update(query: DcimDeviceBaysUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysPartialUpdateQuery {
}
/// Patch a device bay object.

pub fn dcim_device_bays_partial_update(query: DcimDeviceBaysPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysDestroyQuery {
}
/// Delete a device bay object.

pub fn dcim_device_bays_destroy(query: DcimDeviceBaysDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-bays/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesRetrieveQuery {
}
/// Get a power outlet template object.

pub fn dcim_power_outlet_templates_retrieve(query: DcimPowerOutletTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-outlet-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesUpdateQuery {
}
/// Put a power outlet template object.

pub fn dcim_power_outlet_templates_update(query: DcimPowerOutletTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-outlet-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesPartialUpdateQuery {
}
/// Patch a power outlet template object.

pub fn dcim_power_outlet_templates_partial_update(query: DcimPowerOutletTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-outlet-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesDestroyQuery {
}
/// Delete a power outlet template object.

pub fn dcim_power_outlet_templates_destroy(query: DcimPowerOutletTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-outlet-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsRetrieveQuery {
}
/// Get a power outlet object.

pub fn dcim_power_outlets_retrieve(query: DcimPowerOutletsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-outlets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsUpdateQuery {
}
/// Put a power outlet object.

pub fn dcim_power_outlets_update(query: DcimPowerOutletsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-outlets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsPartialUpdateQuery {
}
/// Patch a power outlet object.

pub fn dcim_power_outlets_partial_update(query: DcimPowerOutletsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-outlets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsDestroyQuery {
}
/// Delete a power outlet object.

pub fn dcim_power_outlets_destroy(query: DcimPowerOutletsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-outlets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsListQuery {
	asset_tag: Vec<String>,
	asset_tag__empty: bool,
	asset_tag__ic: Vec<String>,
	asset_tag__ie: Vec<String>,
	asset_tag__iew: Vec<String>,
	asset_tag__isw: Vec<String>,
	asset_tag__n: Vec<String>,
	asset_tag__nic: Vec<String>,
	asset_tag__nie: Vec<String>,
	asset_tag__niew: Vec<String>,
	asset_tag__nisw: Vec<String>,
	component_id: Vec<i32>,
	component_id__empty: Vec<i32>,
	component_id__gt: Vec<i32>,
	component_id__gte: Vec<i32>,
	component_id__lt: Vec<i32>,
	component_id__lte: Vec<i32>,
	component_id__n: Vec<i32>,
	component_typ: String,
	component_typ__n: String,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	discovered: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	part_id: Vec<String>,
	part_id__empty: bool,
	part_id__ic: Vec<String>,
	part_id__ie: Vec<String>,
	part_id__iew: Vec<String>,
	part_id__isw: Vec<String>,
	part_id__n: Vec<String>,
	part_id__nic: Vec<String>,
	part_id__nie: Vec<String>,
	part_id__niew: Vec<String>,
	part_id__nisw: Vec<String>,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	serial: Vec<String>,
	serial__empty: bool,
	serial__ic: Vec<String>,
	serial__ie: Vec<String>,
	serial__iew: Vec<String>,
	serial__isw: Vec<String>,
	serial__n: Vec<String>,
	serial__nic: Vec<String>,
	serial__nie: Vec<String>,
	serial__niew: Vec<String>,
	serial__nisw: Vec<String>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of inventory item objects.

pub fn dcim_inventory_items_list(query: DcimInventoryItemsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/inventory-items/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsBulkUpdateQuery {
}
/// Put a list of inventory item objects.

pub fn dcim_inventory_items_bulk_update(query: DcimInventoryItemsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/inventory-items/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsCreateQuery {
}
/// Post a list of inventory item objects.

pub fn dcim_inventory_items_create(query: DcimInventoryItemsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/inventory-items/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsBulkPartialUpdateQuery {
}
/// Patch a list of inventory item objects.

pub fn dcim_inventory_items_bulk_partial_update(query: DcimInventoryItemsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/inventory-items/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemsBulkDestroyQuery {
}
/// Delete a list of inventory item objects.

pub fn dcim_inventory_items_bulk_destroy(query: DcimInventoryItemsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/inventory-items/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	export_target: Vec<String>,
	export_target__n: Vec<String>,
	export_target_id: Vec<i64>,
	export_target_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	identifier: Vec<i32>,
	identifier__empty: bool,
	identifier__gt: Vec<i32>,
	identifier__gte: Vec<i32>,
	identifier__lt: Vec<i32>,
	identifier__lte: Vec<i32>,
	identifier__n: Vec<i32>,
	import_target: Vec<String>,
	import_target__n: Vec<String>,
	import_target_id: Vec<i64>,
	import_target_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of L2VPN objects.

pub fn vpn_l2vpns_list(query: VpnL2VpnsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/l2vpns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsBulkUpdateQuery {
}
/// Put a list of L2VPN objects.

pub fn vpn_l2vpns_bulk_update(query: VpnL2VpnsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/l2vpns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsCreateQuery {
}
/// Post a list of L2VPN objects.

pub fn vpn_l2vpns_create(query: VpnL2VpnsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/l2vpns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsBulkPartialUpdateQuery {
}
/// Patch a list of L2VPN objects.

pub fn vpn_l2vpns_bulk_partial_update(query: VpnL2VpnsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/l2vpns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsBulkDestroyQuery {
}
/// Delete a list of L2VPN objects.

pub fn vpn_l2vpns_bulk_destroy(query: VpnL2VpnsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/l2vpns/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsRetrieveQuery {
}
/// Get a tunnel group object.

pub fn vpn_tunnel_groups_retrieve(query: VpnTunnelGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/tunnel-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsUpdateQuery {
}
/// Put a tunnel group object.

pub fn vpn_tunnel_groups_update(query: VpnTunnelGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/tunnel-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsPartialUpdateQuery {
}
/// Patch a tunnel group object.

pub fn vpn_tunnel_groups_partial_update(query: VpnTunnelGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/tunnel-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelGroupsDestroyQuery {
}
/// Delete a tunnel group object.

pub fn vpn_tunnel_groups_destroy(query: VpnTunnelGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/tunnel-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsRetrieveQuery {
}
/// Get a site group object.

pub fn dcim_site_groups_retrieve(query: DcimSiteGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/site-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsUpdateQuery {
}
/// Put a site group object.

pub fn dcim_site_groups_update(query: DcimSiteGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/site-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsPartialUpdateQuery {
}
/// Patch a site group object.

pub fn dcim_site_groups_partial_update(query: DcimSiteGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/site-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsDestroyQuery {
}
/// Delete a site group object.

pub fn dcim_site_groups_destroy(query: DcimSiteGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/site-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsChoicesRetrieveQuery {
}
/// Provides an endpoint to iterate through each choice in a set.

pub fn extras_custom_field_choice_sets_choices_retrieve(query: ExtrasCustomFieldChoiceSetsChoicesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-field-choice-sets/{id}/choices/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesListQuery {
	assigned_object_id: Vec<i32>,
	assigned_object_id__empty: bool,
	assigned_object_id__gt: Vec<i32>,
	assigned_object_id__gte: Vec<i32>,
	assigned_object_id__lt: Vec<i32>,
	assigned_object_id__lte: Vec<i32>,
	assigned_object_id__n: Vec<i32>,
	assigned_object_typ: String,
	assigned_object_typ__n: String,
	assigned_object_typ_id: Vec<i64>,
	assigned_object_typ_id__n: Vec<i64>,
	created_after: String,
	created_before: String,
	created_by: Vec<String>,
	created_by__n: Vec<String>,
	created_by_id: Vec<i64>,
	created_by_id__n: Vec<i64>,
	created_by_request: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	kind: Vec<String>,
	kind__n: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of journal entry objects.

pub fn extras_journal_entries_list(query: ExtrasJournalEntriesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/journal-entries/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesBulkUpdateQuery {
}
/// Put a list of journal entry objects.

pub fn extras_journal_entries_bulk_update(query: ExtrasJournalEntriesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/journal-entries/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesCreateQuery {
}
/// Post a list of journal entry objects.

pub fn extras_journal_entries_create(query: ExtrasJournalEntriesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/journal-entries/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesBulkPartialUpdateQuery {
}
/// Patch a list of journal entry objects.

pub fn extras_journal_entries_bulk_partial_update(query: ExtrasJournalEntriesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/journal-entries/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesBulkDestroyQuery {
}
/// Delete a list of journal entry objects.

pub fn extras_journal_entries_bulk_destroy(query: ExtrasJournalEntriesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/journal-entries/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsRetrieveQuery {
}
/// Get a power panel object.

pub fn dcim_power_panels_retrieve(query: DcimPowerPanelsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-panels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsUpdateQuery {
}
/// Put a power panel object.

pub fn dcim_power_panels_update(query: DcimPowerPanelsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-panels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsPartialUpdateQuery {
}
/// Patch a power panel object.

pub fn dcim_power_panels_partial_update(query: DcimPowerPanelsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-panels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPanelsDestroyQuery {
}
/// Delete a power panel object.

pub fn dcim_power_panels_destroy(query: DcimPowerPanelsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-panels/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesRetrieveQuery {
}
/// Get a service object.

pub fn ipam_services_retrieve(query: IpamServicesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/services/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesUpdateQuery {
}
/// Put a service object.

pub fn ipam_services_update(query: IpamServicesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/services/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesPartialUpdateQuery {
}
/// Patch a service object.

pub fn ipam_services_partial_update(query: IpamServicesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/services/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesDestroyQuery {
}
/// Delete a service object.

pub fn ipam_services_destroy(query: IpamServicesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/services/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsRetrieveQuery {
}
/// Get a contact object.

pub fn tenancy_contacts_retrieve(query: TenancyContactsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contacts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsUpdateQuery {
}
/// Put a contact object.

pub fn tenancy_contacts_update(query: TenancyContactsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contacts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsPartialUpdateQuery {
}
/// Patch a contact object.

pub fn tenancy_contacts_partial_update(query: TenancyContactsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contacts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactsDestroyQuery {
}
/// Delete a contact object.

pub fn tenancy_contacts_destroy(query: TenancyContactsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contacts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasContentTypesListQuery {
	app_label: String,
	id: i64,
	limit: i64,
	model: String,
	offset: i64,
	ordering: String,
	q: String
}
/// Read-only list of ContentTypes. Limit results to ContentTypes pertinent to NetBox objects.

pub fn extras_content_types_list(query: ExtrasContentTypesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/content-types/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	content_typ: Vec<String>,
	content_typ_id: Vec<i32>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	for_object_typ_id: Vec<i32>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	object_typs: Vec<i64>,
	object_typs__n: Vec<i64>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	updated_by_request: String
}
/// Get a list of tag objects.

pub fn extras_tags_list(query: ExtrasTagsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/tags/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsBulkUpdateQuery {
}
/// Put a list of tag objects.

pub fn extras_tags_bulk_update(query: ExtrasTagsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/tags/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsCreateQuery {
}
/// Post a list of tag objects.

pub fn extras_tags_create(query: ExtrasTagsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/tags/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsBulkPartialUpdateQuery {
}
/// Patch a list of tag objects.

pub fn extras_tags_bulk_partial_update(query: ExtrasTagsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/tags/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasTagsBulkDestroyQuery {
}
/// Delete a list of tag objects.

pub fn extras_tags_bulk_destroy(query: ExtrasTagsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/tags/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesListQuery {
	contains: String,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	end_address: Vec<String>,
	family: f64,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mark_utilized: bool,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	q: String,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	start_address: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	vrf: Vec<String>,
	vrf__n: Vec<String>,
	vrf_id: Vec<i64>,
	vrf_id__n: Vec<i64>
}
/// Get a list of IP range objects.

pub fn ipam_ip_ranges_list(query: IpamIpRangesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/ip-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesBulkUpdateQuery {
}
/// Put a list of IP range objects.

pub fn ipam_ip_ranges_bulk_update(query: IpamIpRangesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/ip-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesCreateQuery {
}
/// Post a list of IP range objects.

pub fn ipam_ip_ranges_create(query: IpamIpRangesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/ip-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesBulkPartialUpdateQuery {
}
/// Patch a list of IP range objects.

pub fn ipam_ip_ranges_bulk_partial_update(query: IpamIpRangesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/ip-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesBulkDestroyQuery {
}
/// Delete a list of IP range objects.

pub fn ipam_ip_ranges_bulk_destroy(query: IpamIpRangesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/ip-ranges/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsListQuery {
	config_template_id: Vec<i64>,
	config_template_id__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of platform objects.

pub fn dcim_platforms_list(query: DcimPlatformsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/platforms/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsBulkUpdateQuery {
}
/// Put a list of platform objects.

pub fn dcim_platforms_bulk_update(query: DcimPlatformsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/platforms/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsCreateQuery {
}
/// Post a list of platform objects.

pub fn dcim_platforms_create(query: DcimPlatformsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/platforms/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsBulkPartialUpdateQuery {
}
/// Patch a list of platform objects.

pub fn dcim_platforms_bulk_partial_update(query: DcimPlatformsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/platforms/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsBulkDestroyQuery {
}
/// Delete a list of platform objects.

pub fn dcim_platforms_bulk_destroy(query: DcimPlatformsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/platforms/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersListQuery {
	content_typ_id: Vec<i32>,
	content_typ_id__empty: Vec<i32>,
	content_typ_id__gt: Vec<i32>,
	content_typ_id__gte: Vec<i32>,
	content_typ_id__lt: Vec<i32>,
	content_typ_id__lte: Vec<i32>,
	content_typ_id__n: Vec<i32>,
	content_typs: String,
	content_typs__ic: String,
	content_typs__ie: String,
	content_typs__iew: String,
	content_typs__isw: String,
	content_typs__n: String,
	content_typs__nic: String,
	content_typs__nie: String,
	content_typs__niew: String,
	content_typs__nisw: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	enabled: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	shared: bool,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	usable: bool,
	user: Vec<String>,
	user__n: Vec<String>,
	user_id: Vec<i64>,
	user_id__n: Vec<i64>,
	weight: Vec<i32>,
	weight__empty: bool,
	weight__gt: Vec<i32>,
	weight__gte: Vec<i32>,
	weight__lt: Vec<i32>,
	weight__lte: Vec<i32>,
	weight__n: Vec<i32>
}
/// Get a list of saved filter objects.

pub fn extras_saved_filters_list(query: ExtrasSavedFiltersListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/saved-filters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersBulkUpdateQuery {
}
/// Put a list of saved filter objects.

pub fn extras_saved_filters_bulk_update(query: ExtrasSavedFiltersBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/saved-filters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersCreateQuery {
}
/// Post a list of saved filter objects.

pub fn extras_saved_filters_create(query: ExtrasSavedFiltersCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/saved-filters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersBulkPartialUpdateQuery {
}
/// Patch a list of saved filter objects.

pub fn extras_saved_filters_bulk_partial_update(query: ExtrasSavedFiltersBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/saved-filters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersBulkDestroyQuery {
}
/// Delete a list of saved filter objects.

pub fn extras_saved_filters_bulk_destroy(query: ExtrasSavedFiltersBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/saved-filters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesListQuery {
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	config_template_id: Vec<i64>,
	config_template_id__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	vm_role: bool
}
/// Get a list of device role objects.

pub fn dcim_device_roles_list(query: DcimDeviceRolesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesBulkUpdateQuery {
}
/// Put a list of device role objects.

pub fn dcim_device_roles_bulk_update(query: DcimDeviceRolesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesCreateQuery {
}
/// Post a list of device role objects.

pub fn dcim_device_roles_create(query: DcimDeviceRolesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/device-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesBulkPartialUpdateQuery {
}
/// Patch a list of device role objects.

pub fn dcim_device_roles_bulk_partial_update(query: DcimDeviceRolesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesBulkDestroyQuery {
}
/// Delete a list of device role objects.

pub fn dcim_device_roles_bulk_destroy(query: DcimDeviceRolesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesRetrieveQuery {
}
/// Get a prefix object.

pub fn ipam_prefixes_retrieve(query: IpamPrefixesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/prefixes/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesUpdateQuery {
}
/// Put a prefix object.

pub fn ipam_prefixes_update(query: IpamPrefixesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/prefixes/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesPartialUpdateQuery {
}
/// Patch a prefix object.

pub fn ipam_prefixes_partial_update(query: IpamPrefixesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/prefixes/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesDestroyQuery {
}
/// Delete a prefix object.

pub fn ipam_prefixes_destroy(query: IpamPrefixesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/prefixes/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsRetrieveQuery {
}
/// Get a tenant group object.

pub fn tenancy_tenant_groups_retrieve(query: TenancyTenantGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/tenant-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsUpdateQuery {
}
/// Put a tenant group object.

pub fn tenancy_tenant_groups_update(query: TenancyTenantGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/tenant-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsPartialUpdateQuery {
}
/// Patch a tenant group object.

pub fn tenancy_tenant_groups_partial_update(query: TenancyTenantGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/tenant-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsDestroyQuery {
}
/// Delete a tenant group object.

pub fn tenancy_tenant_groups_destroy(query: TenancyTenantGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/tenant-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesRetrieveQuery {
}
/// Get a device type object.

pub fn dcim_device_types_retrieve(query: DcimDeviceTypesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesUpdateQuery {
}
/// Put a device type object.

pub fn dcim_device_types_update(query: DcimDeviceTypesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesPartialUpdateQuery {
}
/// Patch a device type object.

pub fn dcim_device_types_partial_update(query: DcimDeviceTypesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceTypesDestroyQuery {
}
/// Delete a device type object.

pub fn dcim_device_types_destroy(query: DcimDeviceTypesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsRetrieveQuery {
}
/// Get a cable termination object.

pub fn dcim_cable_terminations_retrieve(query: DcimCableTerminationsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/cable-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsUpdateQuery {
}
/// Put a cable termination object.

pub fn dcim_cable_terminations_update(query: DcimCableTerminationsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/cable-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsPartialUpdateQuery {
}
/// Patch a cable termination object.

pub fn dcim_cable_terminations_partial_update(query: DcimCableTerminationsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/cable-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsDestroyQuery {
}
/// Delete a cable termination object.

pub fn dcim_cable_terminations_destroy(query: DcimCableTerminationsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/cable-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	updated_by_request: String
}
/// Get a list of module bay template objects.

pub fn dcim_module_bay_templates_list(query: DcimModuleBayTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/module-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesBulkUpdateQuery {
}
/// Put a list of module bay template objects.

pub fn dcim_module_bay_templates_bulk_update(query: DcimModuleBayTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/module-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesCreateQuery {
}
/// Post a list of module bay template objects.

pub fn dcim_module_bay_templates_create(query: DcimModuleBayTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/module-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of module bay template objects.

pub fn dcim_module_bay_templates_bulk_partial_update(query: DcimModuleBayTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/module-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBayTemplatesBulkDestroyQuery {
}
/// Delete a list of module bay template objects.

pub fn dcim_module_bay_templates_bulk_destroy(query: DcimModuleBayTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/module-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsRetrieveQuery {
}
/// Get a L2VPN object.

pub fn vpn_l2vpns_retrieve(query: VpnL2VpnsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/l2vpns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsUpdateQuery {
}
/// Put a L2VPN object.

pub fn vpn_l2vpns_update(query: VpnL2VpnsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/l2vpns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsPartialUpdateQuery {
}
/// Patch a L2VPN object.

pub fn vpn_l2vpns_partial_update(query: VpnL2VpnsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/l2vpns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnsDestroyQuery {
}
/// Delete a L2VPN object.

pub fn vpn_l2vpns_destroy(query: VpnL2VpnsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/l2vpns/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of role objects.

pub fn ipam_roles_list(query: IpamRolesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesBulkUpdateQuery {
}
/// Put a list of role objects.

pub fn ipam_roles_bulk_update(query: IpamRolesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesCreateQuery {
}
/// Post a list of role objects.

pub fn ipam_roles_create(query: IpamRolesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesBulkPartialUpdateQuery {
}
/// Patch a list of role objects.

pub fn ipam_roles_bulk_partial_update(query: IpamRolesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesBulkDestroyQuery {
}
/// Delete a list of role objects.

pub fn ipam_roles_bulk_destroy(query: IpamRolesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesRetrieveQuery {
}
/// Get a role object.

pub fn ipam_roles_retrieve(query: IpamRolesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesUpdateQuery {
}
/// Put a role object.

pub fn ipam_roles_update(query: IpamRolesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesPartialUpdateQuery {
}
/// Patch a role object.

pub fn ipam_roles_partial_update(query: IpamRolesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRolesDestroyQuery {
}
/// Delete a role object.

pub fn ipam_roles_destroy(query: IpamRolesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsListQuery {
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	connected: bool,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of console server port objects.

pub fn dcim_console_server_ports_list(query: DcimConsoleServerPortsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-server-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsBulkUpdateQuery {
}
/// Put a list of console server port objects.

pub fn dcim_console_server_ports_bulk_update(query: DcimConsoleServerPortsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-server-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsCreateQuery {
}
/// Post a list of console server port objects.

pub fn dcim_console_server_ports_create(query: DcimConsoleServerPortsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/console-server-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsBulkPartialUpdateQuery {
}
/// Patch a list of console server port objects.

pub fn dcim_console_server_ports_bulk_partial_update(query: DcimConsoleServerPortsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-server-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsBulkDestroyQuery {
}
/// Delete a list of console server port objects.

pub fn dcim_console_server_ports_bulk_destroy(query: DcimConsoleServerPortsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-server-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesListQuery {
	bridge_id: Vec<i64>,
	bridge_id__n: Vec<i64>,
	cluster: Vec<String>,
	cluster__n: Vec<String>,
	cluster_id: Vec<i64>,
	cluster_id__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	enabled: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	l2vpn: Vec<i64>,
	l2vpn__n: Vec<i64>,
	l2vpn_id: Vec<i64>,
	l2vpn_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mac_address: Vec<String>,
	mac_address__ic: Vec<String>,
	mac_address__ie: Vec<String>,
	mac_address__iew: Vec<String>,
	mac_address__isw: Vec<String>,
	mac_address__n: Vec<String>,
	mac_address__nic: Vec<String>,
	mac_address__nie: Vec<String>,
	mac_address__niew: Vec<String>,
	mac_address__nisw: Vec<String>,
	modified_by_request: String,
	mtu: Vec<i32>,
	mtu__empty: bool,
	mtu__gt: Vec<i32>,
	mtu__gte: Vec<i32>,
	mtu__lt: Vec<i32>,
	mtu__lte: Vec<i32>,
	mtu__n: Vec<i32>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_machine: Vec<String>,
	virtual_machine__n: Vec<String>,
	virtual_machine_id: Vec<i64>,
	virtual_machine_id__n: Vec<i64>,
	vlan: String,
	vlan_id: String,
	vrf: Vec<String>,
	vrf__n: Vec<String>,
	vrf_id: Vec<i64>,
	vrf_id__n: Vec<i64>
}
/// Get a list of interface objects.

pub fn virtualization_interfaces_list(query: VirtualizationInterfacesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesBulkUpdateQuery {
}
/// Put a list of interface objects.

pub fn virtualization_interfaces_bulk_update(query: VirtualizationInterfacesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesCreateQuery {
}
/// Post a list of interface objects.

pub fn virtualization_interfaces_create(query: VirtualizationInterfacesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesBulkPartialUpdateQuery {
}
/// Patch a list of interface objects.

pub fn virtualization_interfaces_bulk_partial_update(query: VirtualizationInterfacesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesBulkDestroyQuery {
}
/// Delete a list of interface objects.

pub fn virtualization_interfaces_bulk_destroy(query: VirtualizationInterfacesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/interfaces/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	typ: String,
	typ__n: String,
	updated_by_request: String
}
/// Get a list of console port template objects.

pub fn dcim_console_port_templates_list(query: DcimConsolePortTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesBulkUpdateQuery {
}
/// Put a list of console port template objects.

pub fn dcim_console_port_templates_bulk_update(query: DcimConsolePortTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesCreateQuery {
}
/// Post a list of console port template objects.

pub fn dcim_console_port_templates_create(query: DcimConsolePortTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/console-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of console port template objects.

pub fn dcim_console_port_templates_bulk_partial_update(query: DcimConsolePortTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsolePortTemplatesBulkDestroyQuery {
}
/// Delete a list of console port template objects.

pub fn dcim_console_port_templates_bulk_destroy(query: DcimConsolePortTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsRetrieveQuery {
}
/// Get a virtual device context object.

pub fn dcim_virtual_device_contexts_retrieve(query: DcimVirtualDeviceContextsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/virtual-device-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsUpdateQuery {
}
/// Put a virtual device context object.

pub fn dcim_virtual_device_contexts_update(query: DcimVirtualDeviceContextsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/virtual-device-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsPartialUpdateQuery {
}
/// Patch a virtual device context object.

pub fn dcim_virtual_device_contexts_partial_update(query: DcimVirtualDeviceContextsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/virtual-device-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsDestroyQuery {
}
/// Delete a virtual device context object.

pub fn dcim_virtual_device_contexts_destroy(query: DcimVirtualDeviceContextsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/virtual-device-contexts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasObjectChangesRetrieveQuery {
}
/// Retrieve a list of recent changes.

pub fn extras_object_changes_retrieve(query: ExtrasObjectChangesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/object-changes/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesListQuery {
	airflow: String,
	airflow__n: String,
	asset_tag: Vec<String>,
	asset_tag__empty: bool,
	asset_tag__ic: Vec<String>,
	asset_tag__ie: Vec<String>,
	asset_tag__iew: Vec<String>,
	asset_tag__isw: Vec<String>,
	asset_tag__n: Vec<String>,
	asset_tag__nic: Vec<String>,
	asset_tag__nie: Vec<String>,
	asset_tag__niew: Vec<String>,
	asset_tag__nisw: Vec<String>,
	cluster_id: Vec<i64>,
	cluster_id__n: Vec<i64>,
	config_template_id: Vec<i64>,
	config_template_id__n: Vec<i64>,
	console_ports: bool,
	console_server_ports: bool,
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device_bays: bool,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	face: String,
	face__n: String,
	has_oob_ip: bool,
	has_primary_ip: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interfaces: bool,
	is_full_depth: bool,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	latitude: Vec<f64>,
	latitude__empty: bool,
	latitude__gt: Vec<f64>,
	latitude__gte: Vec<f64>,
	latitude__lt: Vec<f64>,
	latitude__lte: Vec<f64>,
	latitude__n: Vec<f64>,
	limit: i64,
	local_context_data: bool,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	longitude: Vec<f64>,
	longitude__empty: bool,
	longitude__gt: Vec<f64>,
	longitude__gte: Vec<f64>,
	longitude__lt: Vec<f64>,
	longitude__lte: Vec<f64>,
	longitude__n: Vec<f64>,
	mac_address: Vec<String>,
	mac_address__ic: Vec<String>,
	mac_address__ie: Vec<String>,
	mac_address__iew: Vec<String>,
	mac_address__isw: Vec<String>,
	mac_address__n: Vec<String>,
	mac_address__nic: Vec<String>,
	mac_address__nie: Vec<String>,
	mac_address__niew: Vec<String>,
	mac_address__nisw: Vec<String>,
	manufacturer: Vec<String>,
	manufacturer__n: Vec<String>,
	manufacturer_id: Vec<i64>,
	manufacturer_id__n: Vec<i64>,
	model: Vec<String>,
	model__n: Vec<String>,
	modified_by_request: String,
	module_bays: bool,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	oob_ip_id: Vec<i64>,
	oob_ip_id__n: Vec<i64>,
	ordering: String,
	parent_device_id: Vec<i64>,
	parent_device_id__n: Vec<i64>,
	pass_through_ports: bool,
	platform: Vec<String>,
	platform__n: Vec<String>,
	platform_id: Vec<i64>,
	platform_id__n: Vec<i64>,
	position: Vec<f64>,
	position__empty: bool,
	position__gt: Vec<f64>,
	position__gte: Vec<f64>,
	position__lt: Vec<f64>,
	position__lte: Vec<f64>,
	position__n: Vec<f64>,
	power_outlets: bool,
	power_ports: bool,
	primary_ip4_id: Vec<i64>,
	primary_ip4_id__n: Vec<i64>,
	primary_ip6_id: Vec<i64>,
	primary_ip6_id__n: Vec<i64>,
	q: String,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	serial: Vec<String>,
	serial__empty: bool,
	serial__ic: Vec<String>,
	serial__ie: Vec<String>,
	serial__iew: Vec<String>,
	serial__isw: Vec<String>,
	serial__n: Vec<String>,
	serial__nic: Vec<String>,
	serial__nie: Vec<String>,
	serial__niew: Vec<String>,
	serial__nisw: Vec<String>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	vc_position: Vec<i32>,
	vc_position__empty: bool,
	vc_position__gt: Vec<i32>,
	vc_position__gte: Vec<i32>,
	vc_position__lt: Vec<i32>,
	vc_position__lte: Vec<i32>,
	vc_position__n: Vec<i32>,
	vc_priority: Vec<i32>,
	vc_priority__empty: bool,
	vc_priority__gt: Vec<i32>,
	vc_priority__gte: Vec<i32>,
	vc_priority__lt: Vec<i32>,
	vc_priority__lte: Vec<i32>,
	vc_priority__n: Vec<i32>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>,
	virtual_chassis_member: bool
}
/// Get a list of device objects.

pub fn dcim_devices_list(query: DcimDevicesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/devices/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesBulkUpdateQuery {
}
/// Put a list of device objects.

pub fn dcim_devices_bulk_update(query: DcimDevicesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/devices/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesCreateQuery {
}
/// Post a list of device objects.

pub fn dcim_devices_create(query: DcimDevicesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/devices/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesBulkPartialUpdateQuery {
}
/// Patch a list of device objects.

pub fn dcim_devices_bulk_partial_update(query: DcimDevicesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/devices/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesBulkDestroyQuery {
}
/// Delete a list of device objects.

pub fn dcim_devices_bulk_destroy(query: DcimDevicesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/devices/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsRetrieveQuery {
}
/// Get a RIR object.

pub fn ipam_rirs_retrieve(query: IpamRirsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/rirs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsUpdateQuery {
}
/// Put a RIR object.

pub fn ipam_rirs_update(query: IpamRirsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/rirs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsPartialUpdateQuery {
}
/// Patch a RIR object.

pub fn ipam_rirs_partial_update(query: IpamRirsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/rirs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamRirsDestroyQuery {
}
/// Delete a RIR object.

pub fn ipam_rirs_destroy(query: IpamRirsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/rirs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesRetrieveQuery {
}
/// Get a IPSec profile object.

pub fn vpn_ipsec_profiles_retrieve(query: VpnIpsecProfilesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ipsec-profiles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesUpdateQuery {
}
/// Put a IPSec profile object.

pub fn vpn_ipsec_profiles_update(query: VpnIpsecProfilesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ipsec-profiles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesPartialUpdateQuery {
}
/// Patch a IPSec profile object.

pub fn vpn_ipsec_profiles_partial_update(query: VpnIpsecProfilesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ipsec-profiles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProfilesDestroyQuery {
}
/// Delete a IPSec profile object.

pub fn vpn_ipsec_profiles_destroy(query: VpnIpsecProfilesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ipsec-profiles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	group: Vec<String>,
	group__n: Vec<String>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	typ_id: Vec<i64>,
	typ_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of cluster objects.

pub fn virtualization_clusters_list(query: VirtualizationClustersListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/clusters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersBulkUpdateQuery {
}
/// Put a list of cluster objects.

pub fn virtualization_clusters_bulk_update(query: VirtualizationClustersBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/clusters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersCreateQuery {
}
/// Post a list of cluster objects.

pub fn virtualization_clusters_create(query: VirtualizationClustersCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/clusters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersBulkPartialUpdateQuery {
}
/// Patch a list of cluster objects.

pub fn virtualization_clusters_bulk_partial_update(query: VirtualizationClustersBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/clusters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersBulkDestroyQuery {
}
/// Delete a list of cluster objects.

pub fn virtualization_clusters_bulk_destroy(query: VirtualizationClustersBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/clusters/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesRetrieveQuery {
}
/// Get a IP range object.

pub fn ipam_ip_ranges_retrieve(query: IpamIpRangesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/ip-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesUpdateQuery {
}
/// Put a IP range object.

pub fn ipam_ip_ranges_update(query: IpamIpRangesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/ip-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesPartialUpdateQuery {
}
/// Patch a IP range object.

pub fn ipam_ip_ranges_partial_update(query: IpamIpRangesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/ip-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamIpRangesDestroyQuery {
}
/// Delete a IP range object.

pub fn ipam_ip_ranges_destroy(query: IpamIpRangesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/ip-ranges/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesTraceRetrieveQuery {
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).

pub fn dcim_interfaces_trace_retrieve(query: DcimInterfacesTraceRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/interfaces/{id}/trace/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksListQuery {
	auth_cipher: Vec<String>,
	auth_cipher__n: Vec<String>,
	auth_psk: Vec<String>,
	auth_psk__empty: bool,
	auth_psk__ic: Vec<String>,
	auth_psk__ie: Vec<String>,
	auth_psk__iew: Vec<String>,
	auth_psk__isw: Vec<String>,
	auth_psk__n: Vec<String>,
	auth_psk__nic: Vec<String>,
	auth_psk__nie: Vec<String>,
	auth_psk__niew: Vec<String>,
	auth_psk__nisw: Vec<String>,
	auth_typ: Vec<String>,
	auth_typ__n: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interface_a_id: Vec<i32>,
	interface_a_id__empty: Vec<i32>,
	interface_a_id__gt: Vec<i32>,
	interface_a_id__gte: Vec<i32>,
	interface_a_id__lt: Vec<i32>,
	interface_a_id__lte: Vec<i32>,
	interface_a_id__n: Vec<i32>,
	interface_b_id: Vec<i32>,
	interface_b_id__empty: Vec<i32>,
	interface_b_id__gt: Vec<i32>,
	interface_b_id__gte: Vec<i32>,
	interface_b_id__lt: Vec<i32>,
	interface_b_id__lte: Vec<i32>,
	interface_b_id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	q: String,
	ssid: Vec<String>,
	ssid__empty: bool,
	ssid__ic: Vec<String>,
	ssid__ie: Vec<String>,
	ssid__iew: Vec<String>,
	ssid__isw: Vec<String>,
	ssid__n: Vec<String>,
	ssid__nic: Vec<String>,
	ssid__nie: Vec<String>,
	ssid__niew: Vec<String>,
	ssid__nisw: Vec<String>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of wireless link objects.

pub fn wireless_wireless_links_list(query: WirelessWirelessLinksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/wireless/wireless-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksBulkUpdateQuery {
}
/// Put a list of wireless link objects.

pub fn wireless_wireless_links_bulk_update(query: WirelessWirelessLinksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/wireless/wireless-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksCreateQuery {
}
/// Post a list of wireless link objects.

pub fn wireless_wireless_links_create(query: WirelessWirelessLinksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/wireless/wireless-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksBulkPartialUpdateQuery {
}
/// Patch a list of wireless link objects.

pub fn wireless_wireless_links_bulk_partial_update(query: WirelessWirelessLinksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/wireless/wireless-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLinksBulkDestroyQuery {
}
/// Delete a list of wireless link objects.

pub fn wireless_wireless_links_bulk_destroy(query: WirelessWirelessLinksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/wireless/wireless-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsRetrieveQuery {
}
/// Get a cluster group object.

pub fn virtualization_cluster_groups_retrieve(query: VirtualizationClusterGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/cluster-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsUpdateQuery {
}
/// Put a cluster group object.

pub fn virtualization_cluster_groups_update(query: VirtualizationClusterGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/cluster-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsPartialUpdateQuery {
}
/// Patch a cluster group object.

pub fn virtualization_cluster_groups_partial_update(query: VirtualizationClusterGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/cluster-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsDestroyQuery {
}
/// Delete a cluster group object.

pub fn virtualization_cluster_groups_destroy(query: VirtualizationClusterGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/cluster-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesRetrieveQuery {
}
/// Get a event rule object.

pub fn extras_event_rules_retrieve(query: ExtrasEventRulesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/event-rules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesUpdateQuery {
}
/// Put a event rule object.

pub fn extras_event_rules_update(query: ExtrasEventRulesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/event-rules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesPartialUpdateQuery {
}
/// Patch a event rule object.

pub fn extras_event_rules_partial_update(query: ExtrasEventRulesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/event-rules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesDestroyQuery {
}
/// Delete a event rule object.

pub fn extras_event_rules_destroy(query: ExtrasEventRulesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/event-rules/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsListQuery {
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	positions: Vec<i32>,
	positions__empty: bool,
	positions__gt: Vec<i32>,
	positions__gte: Vec<i32>,
	positions__lt: Vec<i32>,
	positions__lte: Vec<i32>,
	positions__n: Vec<i32>,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of rear port objects.

pub fn dcim_rear_ports_list(query: DcimRearPortsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rear-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsBulkUpdateQuery {
}
/// Put a list of rear port objects.

pub fn dcim_rear_ports_bulk_update(query: DcimRearPortsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rear-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsCreateQuery {
}
/// Post a list of rear port objects.

pub fn dcim_rear_ports_create(query: DcimRearPortsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/rear-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsBulkPartialUpdateQuery {
}
/// Patch a list of rear port objects.

pub fn dcim_rear_ports_bulk_partial_update(query: DcimRearPortsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rear-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsBulkDestroyQuery {
}
/// Delete a list of rear port objects.

pub fn dcim_rear_ports_bulk_destroy(query: DcimRearPortsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rear-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	ipaddress: Vec<String>,
	ipaddress__n: Vec<String>,
	ipaddress_id: Vec<i64>,
	ipaddress_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	port: f64,
	protocol: String,
	protocol__n: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_machine: Vec<String>,
	virtual_machine__n: Vec<String>,
	virtual_machine_id: Vec<i64>,
	virtual_machine_id__n: Vec<i64>
}
/// Get a list of service objects.

pub fn ipam_services_list(query: IpamServicesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/services/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesBulkUpdateQuery {
}
/// Put a list of service objects.

pub fn ipam_services_bulk_update(query: IpamServicesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/services/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesCreateQuery {
}
/// Post a list of service objects.

pub fn ipam_services_create(query: IpamServicesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/services/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesBulkPartialUpdateQuery {
}
/// Patch a list of service objects.

pub fn ipam_services_bulk_partial_update(query: IpamServicesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/services/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServicesBulkDestroyQuery {
}
/// Delete a list of service objects.

pub fn ipam_services_bulk_destroy(query: IpamServicesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/services/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersRetrieveQuery {
}
/// Get a user object.

pub fn users_users_retrieve(query: UsersUsersRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/users/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersUpdateQuery {
}
/// Put a user object.

pub fn users_users_update(query: UsersUsersUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/users/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersPartialUpdateQuery {
}
/// Patch a user object.

pub fn users_users_partial_update(query: UsersUsersPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/users/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersDestroyQuery {
}
/// Delete a user object.

pub fn users_users_destroy(query: UsersUsersDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/users/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesSyncCreateQuery {
}
/// Provide a /sync API endpoint to synchronize an object's data from its associated DataFile (if any).

pub fn extras_export_templates_sync_create(query: ExtrasExportTemplatesSyncCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/export-templates/{id}/sync/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersRetrieveQuery {
}
/// Get a saved filter object.

pub fn extras_saved_filters_retrieve(query: ExtrasSavedFiltersRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/saved-filters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersUpdateQuery {
}
/// Put a saved filter object.

pub fn extras_saved_filters_update(query: ExtrasSavedFiltersUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/saved-filters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersPartialUpdateQuery {
}
/// Patch a saved filter object.

pub fn extras_saved_filters_partial_update(query: ExtrasSavedFiltersPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/saved-filters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasSavedFiltersDestroyQuery {
}
/// Delete a saved filter object.

pub fn extras_saved_filters_destroy(query: ExtrasSavedFiltersDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/saved-filters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConnectedDeviceListQuery {
	peer_device: String,
	peer_interface: String
}
/// This endpoint allows a user to determine what device (if any) is connected to a given peer device and peer
/// interface. This is useful in a situation where a device boots with no configuration, but can detect its neighbors
/// via a protocol such as LLDP. Two query parameters must be included in the request:
/// 
/// * `peer_device`: The name of the peer device
/// * `peer_interface`: The name of the peer interface

pub fn dcim_connected_device_list(query: DcimConnectedDeviceListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/connected-device/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of module bay objects.

pub fn dcim_module_bays_list(query: DcimModuleBaysListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/module-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysBulkUpdateQuery {
}
/// Put a list of module bay objects.

pub fn dcim_module_bays_bulk_update(query: DcimModuleBaysBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/module-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysCreateQuery {
}
/// Post a list of module bay objects.

pub fn dcim_module_bays_create(query: DcimModuleBaysCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/module-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysBulkPartialUpdateQuery {
}
/// Patch a list of module bay objects.

pub fn dcim_module_bays_bulk_partial_update(query: DcimModuleBaysBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/module-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleBaysBulkDestroyQuery {
}
/// Delete a list of module bay objects.

pub fn dcim_module_bays_bulk_destroy(query: DcimModuleBaysBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/module-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansRetrieveQuery {
}
/// Get a VLAN object.

pub fn ipam_vlans_retrieve(query: IpamVlansRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vlans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansUpdateQuery {
}
/// Put a VLAN object.

pub fn ipam_vlans_update(query: IpamVlansUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/vlans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansPartialUpdateQuery {
}
/// Patch a VLAN object.

pub fn ipam_vlans_partial_update(query: IpamVlansPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/vlans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansDestroyQuery {
}
/// Delete a VLAN object.

pub fn ipam_vlans_destroy(query: IpamVlansDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/vlans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesRetrieveQuery {
}
/// Get a interface object.

pub fn dcim_interfaces_retrieve(query: DcimInterfacesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesUpdateQuery {
}
/// Put a interface object.

pub fn dcim_interfaces_update(query: DcimInterfacesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesPartialUpdateQuery {
}
/// Patch a interface object.

pub fn dcim_interfaces_partial_update(query: DcimInterfacesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInterfacesDestroyQuery {
}
/// Delete a interface object.

pub fn dcim_interfaces_destroy(query: DcimInterfacesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisRetrieveQuery {
}
/// Get a virtual chassis object.

pub fn dcim_virtual_chassis_retrieve(query: DcimVirtualChassisRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/virtual-chassis/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisUpdateQuery {
}
/// Put a virtual chassis object.

pub fn dcim_virtual_chassis_update(query: DcimVirtualChassisUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/virtual-chassis/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisPartialUpdateQuery {
}
/// Patch a virtual chassis object.

pub fn dcim_virtual_chassis_partial_update(query: DcimVirtualChassisPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/virtual-chassis/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisDestroyQuery {
}
/// Delete a virtual chassis object.

pub fn dcim_virtual_chassis_destroy(query: DcimVirtualChassisDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/virtual-chassis/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<i64>,
	device__n: Vec<i64>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	has_primary_ip: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	primary_ip4_id: Vec<i64>,
	primary_ip4_id__n: Vec<i64>,
	primary_ip6_id: Vec<i64>,
	primary_ip6_id__n: Vec<i64>,
	q: String,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of virtual device context objects.

pub fn dcim_virtual_device_contexts_list(query: DcimVirtualDeviceContextsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/virtual-device-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsBulkUpdateQuery {
}
/// Put a list of virtual device context objects.

pub fn dcim_virtual_device_contexts_bulk_update(query: DcimVirtualDeviceContextsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/virtual-device-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsCreateQuery {
}
/// Post a list of virtual device context objects.

pub fn dcim_virtual_device_contexts_create(query: DcimVirtualDeviceContextsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/virtual-device-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsBulkPartialUpdateQuery {
}
/// Patch a list of virtual device context objects.

pub fn dcim_virtual_device_contexts_bulk_partial_update(query: DcimVirtualDeviceContextsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/virtual-device-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualDeviceContextsBulkDestroyQuery {
}
/// Delete a list of virtual device context objects.

pub fn dcim_virtual_device_contexts_bulk_destroy(query: DcimVirtualDeviceContextsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/virtual-device-contexts/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	enforce_unique: bool,
	export_target: Vec<String>,
	export_target__n: Vec<String>,
	export_target_id: Vec<i64>,
	export_target_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	import_target: Vec<String>,
	import_target__n: Vec<String>,
	import_target_id: Vec<i64>,
	import_target_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	rd: Vec<String>,
	rd__empty: bool,
	rd__ic: Vec<String>,
	rd__ie: Vec<String>,
	rd__iew: Vec<String>,
	rd__isw: Vec<String>,
	rd__n: Vec<String>,
	rd__nic: Vec<String>,
	rd__nie: Vec<String>,
	rd__niew: Vec<String>,
	rd__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of VRF objects.

pub fn ipam_vrfs_list(query: IpamVrfsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vrfs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsBulkUpdateQuery {
}
/// Put a list of VRF objects.

pub fn ipam_vrfs_bulk_update(query: IpamVrfsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/vrfs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsCreateQuery {
}
/// Post a list of VRF objects.

pub fn ipam_vrfs_create(query: IpamVrfsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/vrfs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsBulkPartialUpdateQuery {
}
/// Patch a list of VRF objects.

pub fn ipam_vrfs_bulk_partial_update(query: IpamVrfsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/vrfs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsBulkDestroyQuery {
}
/// Delete a list of VRF objects.

pub fn ipam_vrfs_bulk_destroy(query: IpamVrfsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/vrfs/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsRetrieveQuery {
}
/// Get a tenant object.

pub fn tenancy_tenants_retrieve(query: TenancyTenantsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/tenants/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsUpdateQuery {
}
/// Put a tenant object.

pub fn tenancy_tenants_update(query: TenancyTenantsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/tenants/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsPartialUpdateQuery {
}
/// Patch a tenant object.

pub fn tenancy_tenants_partial_update(query: TenancyTenantsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/tenants/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantsDestroyQuery {
}
/// Delete a tenant object.

pub fn tenancy_tenants_destroy(query: TenancyTenantsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/tenants/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersRetrieveQuery {
}
/// Get a cluster object.

pub fn virtualization_clusters_retrieve(query: VirtualizationClustersRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/clusters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersUpdateQuery {
}
/// Put a cluster object.

pub fn virtualization_clusters_update(query: VirtualizationClustersUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/clusters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersPartialUpdateQuery {
}
/// Patch a cluster object.

pub fn virtualization_clusters_partial_update(query: VirtualizationClustersPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/clusters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClustersDestroyQuery {
}
/// Delete a cluster object.

pub fn virtualization_clusters_destroy(query: VirtualizationClustersDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/clusters/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsAvailableVlansListQuery {
}
/// Get a VLAN object.

pub fn ipam_vlan_groups_available_vlans_list(query: IpamVlanGroupsAvailableVlansListQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vlan-groups/{id}/available-vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsAvailableVlansCreateQuery {
}
/// Post a VLAN object.

pub fn ipam_vlan_groups_available_vlans_create(query: IpamVlanGroupsAvailableVlansCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/vlan-groups/{id}/available-vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsRetrieveQuery {
}
/// Get a L2VPN termination object.

pub fn vpn_l2vpn_terminations_retrieve(query: VpnL2VpnTerminationsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/l2vpn-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsUpdateQuery {
}
/// Put a L2VPN termination object.

pub fn vpn_l2vpn_terminations_update(query: VpnL2VpnTerminationsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/l2vpn-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsPartialUpdateQuery {
}
/// Patch a L2VPN termination object.

pub fn vpn_l2vpn_terminations_partial_update(query: VpnL2VpnTerminationsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/l2vpn-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnL2VpnTerminationsDestroyQuery {
}
/// Delete a L2VPN termination object.

pub fn vpn_l2vpn_terminations_destroy(query: VpnL2VpnTerminationsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/l2vpn-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesRetrieveQuery {
}
/// Get a export template object.

pub fn extras_export_templates_retrieve(query: ExtrasExportTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/export-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesUpdateQuery {
}
/// Put a export template object.

pub fn extras_export_templates_update(query: ExtrasExportTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/export-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesPartialUpdateQuery {
}
/// Patch a export template object.

pub fn extras_export_templates_partial_update(query: ExtrasExportTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/export-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasExportTemplatesDestroyQuery {
}
/// Delete a export template object.

pub fn extras_export_templates_destroy(query: ExtrasExportTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/export-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesRetrieveQuery {
}
/// Get a inventory item template object.

pub fn dcim_inventory_item_templates_retrieve(query: DcimInventoryItemTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/inventory-item-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesUpdateQuery {
}
/// Put a inventory item template object.

pub fn dcim_inventory_item_templates_update(query: DcimInventoryItemTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/inventory-item-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesPartialUpdateQuery {
}
/// Patch a inventory item template object.

pub fn dcim_inventory_item_templates_partial_update(query: DcimInventoryItemTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/inventory-item-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemTemplatesDestroyQuery {
}
/// Delete a inventory item template object.

pub fn dcim_inventory_item_templates_destroy(query: DcimInventoryItemTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/inventory-item-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesRetrieveQuery {
}
/// Get a journal entry object.

pub fn extras_journal_entries_retrieve(query: ExtrasJournalEntriesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/journal-entries/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesUpdateQuery {
}
/// Put a journal entry object.

pub fn extras_journal_entries_update(query: ExtrasJournalEntriesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/journal-entries/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesPartialUpdateQuery {
}
/// Patch a journal entry object.

pub fn extras_journal_entries_partial_update(query: ExtrasJournalEntriesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/journal-entries/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasJournalEntriesDestroyQuery {
}
/// Delete a journal entry object.

pub fn extras_journal_entries_destroy(query: ExtrasJournalEntriesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/journal-entries/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	domain: Vec<String>,
	domain__empty: bool,
	domain__ic: Vec<String>,
	domain__ie: Vec<String>,
	domain__iew: Vec<String>,
	domain__isw: Vec<String>,
	domain__n: Vec<String>,
	domain__nic: Vec<String>,
	domain__nie: Vec<String>,
	domain__niew: Vec<String>,
	domain__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	master: Vec<String>,
	master__n: Vec<String>,
	master_id: Vec<i64>,
	master_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of virtual chassis objects.

pub fn dcim_virtual_chassis_list(query: DcimVirtualChassisListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/virtual-chassis/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisBulkUpdateQuery {
}
/// Put a list of virtual chassis objects.

pub fn dcim_virtual_chassis_bulk_update(query: DcimVirtualChassisBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/virtual-chassis/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisCreateQuery {
}
/// Post a list of virtual chassis objects.

pub fn dcim_virtual_chassis_create(query: DcimVirtualChassisCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/virtual-chassis/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisBulkPartialUpdateQuery {
}
/// Patch a list of virtual chassis objects.

pub fn dcim_virtual_chassis_bulk_partial_update(query: DcimVirtualChassisBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/virtual-chassis/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimVirtualChassisBulkDestroyQuery {
}
/// Delete a list of virtual chassis objects.

pub fn dcim_virtual_chassis_bulk_destroy(query: DcimVirtualChassisBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/virtual-chassis/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsRetrieveQuery {
}
/// Get a custom field choice set object.

pub fn extras_custom_field_choice_sets_retrieve(query: ExtrasCustomFieldChoiceSetsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-field-choice-sets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsUpdateQuery {
}
/// Put a custom field choice set object.

pub fn extras_custom_field_choice_sets_update(query: ExtrasCustomFieldChoiceSetsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/custom-field-choice-sets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsPartialUpdateQuery {
}
/// Patch a custom field choice set object.

pub fn extras_custom_field_choice_sets_partial_update(query: ExtrasCustomFieldChoiceSetsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/custom-field-choice-sets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomFieldChoiceSetsDestroyQuery {
}
/// Delete a custom field choice set object.

pub fn extras_custom_field_choice_sets_destroy(query: ExtrasCustomFieldChoiceSetsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/custom-field-choice-sets/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensListQuery {
	created: String,
	created__gte: String,
	created__lte: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	expires: String,
	expires__gte: String,
	expires__lte: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	key: Vec<String>,
	key__empty: bool,
	key__ic: Vec<String>,
	key__ie: Vec<String>,
	key__iew: Vec<String>,
	key__isw: Vec<String>,
	key__n: Vec<String>,
	key__nic: Vec<String>,
	key__nie: Vec<String>,
	key__niew: Vec<String>,
	key__nisw: Vec<String>,
	limit: i64,
	offset: i64,
	ordering: String,
	q: String,
	user: Vec<String>,
	user__n: Vec<String>,
	user_id: Vec<i64>,
	user_id__n: Vec<i64>,
	write_enabled: bool
}
/// Get a list of token objects.

pub fn users_tokens_list(query: UsersTokensListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/tokens/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensBulkUpdateQuery {
}
/// Put a list of token objects.

pub fn users_tokens_bulk_update(query: UsersTokensBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/tokens/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensCreateQuery {
}
/// Post a list of token objects.

pub fn users_tokens_create(query: UsersTokensCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/users/tokens/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensBulkPartialUpdateQuery {
}
/// Patch a list of token objects.

pub fn users_tokens_bulk_partial_update(query: UsersTokensBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/tokens/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensBulkDestroyQuery {
}
/// Delete a list of token objects.

pub fn users_tokens_bulk_destroy(query: UsersTokensBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/tokens/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	parent__n: Vec<String>,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of wireless LAN group objects.

pub fn wireless_wireless_lan_groups_list(query: WirelessWirelessLanGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/wireless/wireless-lan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsBulkUpdateQuery {
}
/// Put a list of wireless LAN group objects.

pub fn wireless_wireless_lan_groups_bulk_update(query: WirelessWirelessLanGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/wireless/wireless-lan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsCreateQuery {
}
/// Post a list of wireless LAN group objects.

pub fn wireless_wireless_lan_groups_create(query: WirelessWirelessLanGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/wireless/wireless-lan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsBulkPartialUpdateQuery {
}
/// Patch a list of wireless LAN group objects.

pub fn wireless_wireless_lan_groups_bulk_partial_update(query: WirelessWirelessLanGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/wireless/wireless-lan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLanGroupsBulkDestroyQuery {
}
/// Delete a list of wireless LAN group objects.

pub fn wireless_wireless_lan_groups_bulk_destroy(query: WirelessWirelessLanGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/wireless/wireless-lan-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksRetrieveQuery {
}
/// Get a bookmark object.

pub fn extras_bookmarks_retrieve(query: ExtrasBookmarksRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/bookmarks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksUpdateQuery {
}
/// Put a bookmark object.

pub fn extras_bookmarks_update(query: ExtrasBookmarksUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/bookmarks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksPartialUpdateQuery {
}
/// Patch a bookmark object.

pub fn extras_bookmarks_partial_update(query: ExtrasBookmarksPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/bookmarks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasBookmarksDestroyQuery {
}
/// Delete a bookmark object.

pub fn extras_bookmarks_destroy(query: ExtrasBookmarksDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/bookmarks/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesRetrieveQuery {
}
/// Get a device role object.

pub fn dcim_device_roles_retrieve(query: DcimDeviceRolesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesUpdateQuery {
}
/// Put a device role object.

pub fn dcim_device_roles_update(query: DcimDeviceRolesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesPartialUpdateQuery {
}
/// Patch a device role object.

pub fn dcim_device_roles_partial_update(query: DcimDeviceRolesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceRolesDestroyQuery {
}
/// Delete a device role object.

pub fn dcim_device_roles_destroy(query: DcimDeviceRolesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsRetrieveQuery {
}
/// Get a permission object.

pub fn users_permissions_retrieve(query: UsersPermissionsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/permissions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsUpdateQuery {
}
/// Put a permission object.

pub fn users_permissions_update(query: UsersPermissionsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/permissions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsPartialUpdateQuery {
}
/// Patch a permission object.

pub fn users_permissions_partial_update(query: UsersPermissionsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/permissions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsDestroyQuery {
}
/// Delete a permission object.

pub fn users_permissions_destroy(query: UsersPermissionsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/permissions/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsRetrieveQuery {
}
/// Get a contact group object.

pub fn tenancy_contact_groups_retrieve(query: TenancyContactGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contact-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsUpdateQuery {
}
/// Put a contact group object.

pub fn tenancy_contact_groups_update(query: TenancyContactGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contact-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsPartialUpdateQuery {
}
/// Patch a contact group object.

pub fn tenancy_contact_groups_partial_update(query: TenancyContactGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contact-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsDestroyQuery {
}
/// Delete a contact group object.

pub fn tenancy_contact_groups_destroy(query: TenancyContactGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contact-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreJobsRetrieveQuery {
}
/// Retrieve a list of job results

pub fn core_jobs_retrieve(query: CoreJobsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/core/jobs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsListQuery {
	cid: Vec<String>,
	cid__empty: bool,
	cid__ic: Vec<String>,
	cid__ie: Vec<String>,
	cid__iew: Vec<String>,
	cid__isw: Vec<String>,
	cid__n: Vec<String>,
	cid__nic: Vec<String>,
	cid__nie: Vec<String>,
	cid__niew: Vec<String>,
	cid__nisw: Vec<String>,
	commit_rate: Vec<i32>,
	commit_rate__empty: bool,
	commit_rate__gt: Vec<i32>,
	commit_rate__gte: Vec<i32>,
	commit_rate__lt: Vec<i32>,
	commit_rate__lte: Vec<i32>,
	commit_rate__n: Vec<i32>,
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	install_date: Vec<String>,
	install_date__empty: bool,
	install_date__gt: Vec<String>,
	install_date__gte: Vec<String>,
	install_date__lt: Vec<String>,
	install_date__lte: Vec<String>,
	install_date__n: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	provider: Vec<String>,
	provider__n: Vec<String>,
	provider_account_id: Vec<i64>,
	provider_account_id__n: Vec<i64>,
	provider_id: Vec<i64>,
	provider_id__n: Vec<i64>,
	provider_network_id: Vec<i64>,
	provider_network_id__n: Vec<i64>,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	termination_date: Vec<String>,
	termination_date__empty: bool,
	termination_date__gt: Vec<String>,
	termination_date__gte: Vec<String>,
	termination_date__lt: Vec<String>,
	termination_date__lte: Vec<String>,
	termination_date__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	typ_id: Vec<i64>,
	typ_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of circuit objects.

pub fn circuits_circuits_list(query: CircuitsCircuitsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuits/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsBulkUpdateQuery {
}
/// Put a list of circuit objects.

pub fn circuits_circuits_bulk_update(query: CircuitsCircuitsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/circuits/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsCreateQuery {
}
/// Post a list of circuit objects.

pub fn circuits_circuits_create(query: CircuitsCircuitsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/circuits/circuits/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsBulkPartialUpdateQuery {
}
/// Patch a list of circuit objects.

pub fn circuits_circuits_bulk_partial_update(query: CircuitsCircuitsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/circuits/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsBulkDestroyQuery {
}
/// Delete a list of circuit objects.

pub fn circuits_circuits_bulk_destroy(query: CircuitsCircuitsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/circuits/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersTokensProvisionCreateQuery {
}
/// Non-authenticated REST API endpoint via which a user may create a Token.

pub fn users_tokens_provision_create(query: UsersTokensProvisionCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/users/tokens/provision/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<i64>,
	location__n: Vec<i64>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	q: String,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	user: Vec<String>,
	user__n: Vec<String>,
	user_id: Vec<i64>,
	user_id__n: Vec<i64>
}
/// Get a list of rack reservation objects.

pub fn dcim_rack_reservations_list(query: DcimRackReservationsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rack-reservations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsBulkUpdateQuery {
}
/// Put a list of rack reservation objects.

pub fn dcim_rack_reservations_bulk_update(query: DcimRackReservationsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rack-reservations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsCreateQuery {
}
/// Post a list of rack reservation objects.

pub fn dcim_rack_reservations_create(query: DcimRackReservationsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/rack-reservations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsBulkPartialUpdateQuery {
}
/// Patch a list of rack reservation objects.

pub fn dcim_rack_reservations_bulk_partial_update(query: DcimRackReservationsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rack-reservations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackReservationsBulkDestroyQuery {
}
/// Delete a list of rack reservation objects.

pub fn dcim_rack_reservations_bulk_destroy(query: DcimRackReservationsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rack-reservations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasConfigTemplatesRenderCreateQuery {
	format: String,
}
/// Render a ConfigTemplate using the context data provided (if any). If the client requests "text/plain" data,
/// return the raw rendered content, rather than serialized JSON.

pub fn extras_config_templates_render_create(query: ExtrasConfigTemplatesRenderCreateQuery, id: i64,) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/config-templates/{id}/render/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsRetrieveQuery {
}
/// Get a VLAN group object.

pub fn ipam_vlan_groups_retrieve(query: IpamVlanGroupsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vlan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsUpdateQuery {
}
/// Put a VLAN group object.

pub fn ipam_vlan_groups_update(query: IpamVlanGroupsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/vlan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsPartialUpdateQuery {
}
/// Patch a VLAN group object.

pub fn ipam_vlan_groups_partial_update(query: IpamVlanGroupsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/vlan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlanGroupsDestroyQuery {
}
/// Delete a VLAN group object.

pub fn ipam_vlan_groups_destroy(query: IpamVlanGroupsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/vlan-groups/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsListQuery {
	cable: i64,
	cable__n: i64,
	cable_end: String,
	cable_end__n: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	offset: i64,
	ordering: String,
	termination_id: Vec<i32>,
	termination_id__empty: bool,
	termination_id__gt: Vec<i32>,
	termination_id__gte: Vec<i32>,
	termination_id__lt: Vec<i32>,
	termination_id__lte: Vec<i32>,
	termination_id__n: Vec<i32>,
	termination_typ: String,
	termination_typ__n: String
}
/// Get a list of cable termination objects.

pub fn dcim_cable_terminations_list(query: DcimCableTerminationsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/cable-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsBulkUpdateQuery {
}
/// Put a list of cable termination objects.

pub fn dcim_cable_terminations_bulk_update(query: DcimCableTerminationsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/cable-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsCreateQuery {
}
/// Post a list of cable termination objects.

pub fn dcim_cable_terminations_create(query: DcimCableTerminationsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/cable-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsBulkPartialUpdateQuery {
}
/// Patch a list of cable termination objects.

pub fn dcim_cable_terminations_bulk_partial_update(query: DcimCableTerminationsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/cable-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimCableTerminationsBulkDestroyQuery {
}
/// Delete a list of cable termination objects.

pub fn dcim_cable_terminations_bulk_destroy(query: DcimCableTerminationsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/cable-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortsTraceRetrieveQuery {
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).

pub fn dcim_console_server_ports_trace_retrieve(query: DcimConsoleServerPortsTraceRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-server-ports/{id}/trace/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesRetrieveQuery {
}
/// Get a virtual machine object.

pub fn virtualization_virtual_machines_retrieve(query: VirtualizationVirtualMachinesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/virtual-machines/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesUpdateQuery {
}
/// Put a virtual machine object.

pub fn virtualization_virtual_machines_update(query: VirtualizationVirtualMachinesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/virtual-machines/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesPartialUpdateQuery {
}
/// Patch a virtual machine object.

pub fn virtualization_virtual_machines_partial_update(query: VirtualizationVirtualMachinesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/virtual-machines/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesDestroyQuery {
}
/// Delete a virtual machine object.

pub fn virtualization_virtual_machines_destroy(query: VirtualizationVirtualMachinesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/virtual-machines/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	feed_leg: Vec<String>,
	feed_leg__n: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	typ: String,
	typ__n: String,
	updated_by_request: String
}
/// Get a list of power outlet template objects.

pub fn dcim_power_outlet_templates_list(query: DcimPowerOutletTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-outlet-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesBulkUpdateQuery {
}
/// Put a list of power outlet template objects.

pub fn dcim_power_outlet_templates_bulk_update(query: DcimPowerOutletTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-outlet-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesCreateQuery {
}
/// Post a list of power outlet template objects.

pub fn dcim_power_outlet_templates_create(query: DcimPowerOutletTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/power-outlet-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of power outlet template objects.

pub fn dcim_power_outlet_templates_bulk_partial_update(query: DcimPowerOutletTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-outlet-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletTemplatesBulkDestroyQuery {
}
/// Delete a list of power outlet template objects.

pub fn dcim_power_outlet_templates_bulk_destroy(query: DcimPowerOutletTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-outlet-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	parent__n: Vec<String>,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of tenant group objects.

pub fn tenancy_tenant_groups_list(query: TenancyTenantGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/tenant-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsBulkUpdateQuery {
}
/// Put a list of tenant group objects.

pub fn tenancy_tenant_groups_bulk_update(query: TenancyTenantGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/tenant-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsCreateQuery {
}
/// Post a list of tenant group objects.

pub fn tenancy_tenant_groups_create(query: TenancyTenantGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/tenancy/tenant-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsBulkPartialUpdateQuery {
}
/// Patch a list of tenant group objects.

pub fn tenancy_tenant_groups_bulk_partial_update(query: TenancyTenantGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/tenant-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyTenantGroupsBulkDestroyQuery {
}
/// Delete a list of tenant group objects.

pub fn tenancy_tenant_groups_bulk_destroy(query: TenancyTenantGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/tenant-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesRetrieveQuery {
}
/// Get a rear port template object.

pub fn dcim_rear_port_templates_retrieve(query: DcimRearPortTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rear-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesUpdateQuery {
}
/// Put a rear port template object.

pub fn dcim_rear_port_templates_update(query: DcimRearPortTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rear-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesPartialUpdateQuery {
}
/// Patch a rear port template object.

pub fn dcim_rear_port_templates_partial_update(query: DcimRearPortTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rear-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortTemplatesDestroyQuery {
}
/// Delete a rear port template object.

pub fn dcim_rear_port_templates_destroy(query: DcimRearPortTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rear-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDevicesRenderConfigCreateQuery {
	format: String,
}
/// Resolve and render the preferred ConfigTemplate for this Device.

pub fn dcim_devices_render_config_create(query: DcimDevicesRenderConfigCreateQuery, id: i64,) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/devices/{id}/render-config/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesAvailableIpsListQuery {
}
/// Get a IP address object.

pub fn ipam_prefixes_available_ips_list(query: IpamPrefixesAvailableIpsListQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/prefixes/{id}/available-ips/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPrefixesAvailableIpsCreateQuery {
}
/// Post a IP address object.

pub fn ipam_prefixes_available_ips_create(query: IpamPrefixesAvailableIpsCreateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/prefixes/{id}/available-ips/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersListQuery {
	email: Vec<String>,
	email__empty: bool,
	email__ic: Vec<String>,
	email__ie: Vec<String>,
	email__iew: Vec<String>,
	email__isw: Vec<String>,
	email__n: Vec<String>,
	email__nic: Vec<String>,
	email__nie: Vec<String>,
	email__niew: Vec<String>,
	email__nisw: Vec<String>,
	first_name: Vec<String>,
	first_name__empty: bool,
	first_name__ic: Vec<String>,
	first_name__ie: Vec<String>,
	first_name__iew: Vec<String>,
	first_name__isw: Vec<String>,
	first_name__n: Vec<String>,
	first_name__nic: Vec<String>,
	first_name__nie: Vec<String>,
	first_name__niew: Vec<String>,
	first_name__nisw: Vec<String>,
	group: Vec<String>,
	group__n: Vec<String>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	is_active: bool,
	is_staff: bool,
	is_superuser: bool,
	last_name: Vec<String>,
	last_name__empty: bool,
	last_name__ic: Vec<String>,
	last_name__ie: Vec<String>,
	last_name__iew: Vec<String>,
	last_name__isw: Vec<String>,
	last_name__n: Vec<String>,
	last_name__nic: Vec<String>,
	last_name__nie: Vec<String>,
	last_name__niew: Vec<String>,
	last_name__nisw: Vec<String>,
	limit: i64,
	offset: i64,
	ordering: String,
	q: String,
	username: Vec<String>,
	username__empty: bool,
	username__ic: Vec<String>,
	username__ie: Vec<String>,
	username__iew: Vec<String>,
	username__isw: Vec<String>,
	username__n: Vec<String>,
	username__nic: Vec<String>,
	username__nie: Vec<String>,
	username__niew: Vec<String>,
	username__nisw: Vec<String>
}
/// Get a list of user objects.

pub fn users_users_list(query: UsersUsersListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/users/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersBulkUpdateQuery {
}
/// Put a list of user objects.

pub fn users_users_bulk_update(query: UsersUsersBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/users/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersCreateQuery {
}
/// Post a list of user objects.

pub fn users_users_create(query: UsersUsersCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/users/users/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersBulkPartialUpdateQuery {
}
/// Patch a list of user objects.

pub fn users_users_bulk_partial_update(query: UsersUsersBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/users/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUsersBulkDestroyQuery {
}
/// Delete a list of user objects.

pub fn users_users_bulk_destroy(query: UsersUsersBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/users/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	interface: Vec<String>,
	interface__n: Vec<String>,
	interface_id: Vec<i64>,
	interface_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	outside_ip_id: Vec<i64>,
	outside_ip_id__n: Vec<i64>,
	q: String,
	role: Vec<String>,
	role__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	termination_typ: String,
	termination_typ__n: String,
	tunnel: Vec<String>,
	tunnel__n: Vec<String>,
	tunnel_id: Vec<i64>,
	tunnel_id__n: Vec<i64>,
	updated_by_request: String,
	vminterface: Vec<String>,
	vminterface__n: Vec<String>,
	vminterface_id: Vec<i64>,
	vminterface_id__n: Vec<i64>
}
/// Get a list of tunnel termination objects.

pub fn vpn_tunnel_terminations_list(query: VpnTunnelTerminationsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/tunnel-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsBulkUpdateQuery {
}
/// Put a list of tunnel termination objects.

pub fn vpn_tunnel_terminations_bulk_update(query: VpnTunnelTerminationsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/tunnel-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsCreateQuery {
}
/// Post a list of tunnel termination objects.

pub fn vpn_tunnel_terminations_create(query: VpnTunnelTerminationsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/tunnel-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsBulkPartialUpdateQuery {
}
/// Patch a list of tunnel termination objects.

pub fn vpn_tunnel_terminations_bulk_partial_update(query: VpnTunnelTerminationsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/tunnel-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsBulkDestroyQuery {
}
/// Delete a list of tunnel termination objects.

pub fn vpn_tunnel_terminations_bulk_destroy(query: VpnTunnelTerminationsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/tunnel-terminations/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerOutletsTraceRetrieveQuery {
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).

pub fn dcim_power_outlets_trace_retrieve(query: DcimPowerOutletsTraceRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-outlets/{id}/trace/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsRetrieveQuery {
}
/// Get a FHRP group assignment object.

pub fn ipam_fhrp_group_assignments_retrieve(query: IpamFhrpGroupAssignmentsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/fhrp-group-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsUpdateQuery {
}
/// Put a FHRP group assignment object.

pub fn ipam_fhrp_group_assignments_update(query: IpamFhrpGroupAssignmentsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/fhrp-group-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsPartialUpdateQuery {
}
/// Patch a FHRP group assignment object.

pub fn ipam_fhrp_group_assignments_partial_update(query: IpamFhrpGroupAssignmentsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/fhrp-group-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamFhrpGroupAssignmentsDestroyQuery {
}
/// Delete a FHRP group assignment object.

pub fn ipam_fhrp_group_assignments_destroy(query: IpamFhrpGroupAssignmentsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/fhrp-group-assignments/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersRetrieveQuery {
}
/// Get a provider object.

pub fn circuits_providers_retrieve(query: CircuitsProvidersRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/providers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersUpdateQuery {
}
/// Put a provider object.

pub fn circuits_providers_update(query: CircuitsProvidersUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/providers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersPartialUpdateQuery {
}
/// Patch a provider object.

pub fn circuits_providers_partial_update(query: CircuitsProvidersPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/providers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProvidersDestroyQuery {
}
/// Delete a provider object.

pub fn circuits_providers_destroy(query: CircuitsProvidersDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/providers/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesRetrieveQuery {
}
/// Get a IKE policy object.

pub fn vpn_ike_policies_retrieve(query: VpnIkePoliciesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ike-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesUpdateQuery {
}
/// Put a IKE policy object.

pub fn vpn_ike_policies_update(query: VpnIkePoliciesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ike-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesPartialUpdateQuery {
}
/// Patch a IKE policy object.

pub fn vpn_ike_policies_partial_update(query: VpnIkePoliciesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ike-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesDestroyQuery {
}
/// Delete a IKE policy object.

pub fn vpn_ike_policies_destroy(query: VpnIkePoliciesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ike-policies/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsListQuery {
	authentication_algorithm: Vec<String>,
	authentication_algorithm__n: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	encryption_algorithm: Vec<String>,
	encryption_algorithm__n: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	sa_lifetime_data: Vec<i32>,
	sa_lifetime_data__empty: bool,
	sa_lifetime_data__gt: Vec<i32>,
	sa_lifetime_data__gte: Vec<i32>,
	sa_lifetime_data__lt: Vec<i32>,
	sa_lifetime_data__lte: Vec<i32>,
	sa_lifetime_data__n: Vec<i32>,
	sa_lifetime_seconds: Vec<i32>,
	sa_lifetime_seconds__empty: bool,
	sa_lifetime_seconds__gt: Vec<i32>,
	sa_lifetime_seconds__gte: Vec<i32>,
	sa_lifetime_seconds__lt: Vec<i32>,
	sa_lifetime_seconds__lte: Vec<i32>,
	sa_lifetime_seconds__n: Vec<i32>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of IPSec proposal objects.

pub fn vpn_ipsec_proposals_list(query: VpnIpsecProposalsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ipsec-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsBulkUpdateQuery {
}
/// Put a list of IPSec proposal objects.

pub fn vpn_ipsec_proposals_bulk_update(query: VpnIpsecProposalsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ipsec-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsCreateQuery {
}
/// Post a list of IPSec proposal objects.

pub fn vpn_ipsec_proposals_create(query: VpnIpsecProposalsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/ipsec-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsBulkPartialUpdateQuery {
}
/// Patch a list of IPSec proposal objects.

pub fn vpn_ipsec_proposals_bulk_partial_update(query: VpnIpsecProposalsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ipsec-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecProposalsBulkDestroyQuery {
}
/// Delete a list of IPSec proposal objects.

pub fn vpn_ipsec_proposals_bulk_destroy(query: VpnIpsecProposalsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ipsec-proposals/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsRetrieveQuery {
}
/// Get a circuit object.

pub fn circuits_circuits_retrieve(query: CircuitsCircuitsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuits/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsUpdateQuery {
}
/// Put a circuit object.

pub fn circuits_circuits_update(query: CircuitsCircuitsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/circuits/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsPartialUpdateQuery {
}
/// Patch a circuit object.

pub fn circuits_circuits_partial_update(query: CircuitsCircuitsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/circuits/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitsDestroyQuery {
}
/// Delete a circuit object.

pub fn circuits_circuits_destroy(query: CircuitsCircuitsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/circuits/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationVirtualMachinesRenderConfigCreateQuery {
	format: String,
}
/// Resolve and render the preferred ConfigTemplate for this Device.

pub fn virtualization_virtual_machines_render_config_create(query: VirtualizationVirtualMachinesRenderConfigCreateQuery, id: i64,) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/virtual-machines/{id}/render-config/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of contact role objects.

pub fn tenancy_contact_roles_list(query: TenancyContactRolesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contact-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesBulkUpdateQuery {
}
/// Put a list of contact role objects.

pub fn tenancy_contact_roles_bulk_update(query: TenancyContactRolesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contact-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesCreateQuery {
}
/// Post a list of contact role objects.

pub fn tenancy_contact_roles_create(query: TenancyContactRolesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/tenancy/contact-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesBulkPartialUpdateQuery {
}
/// Patch a list of contact role objects.

pub fn tenancy_contact_roles_bulk_partial_update(query: TenancyContactRolesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contact-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactRolesBulkDestroyQuery {
}
/// Delete a list of contact role objects.

pub fn tenancy_contact_roles_bulk_destroy(query: TenancyContactRolesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contact-roles/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansListQuery {
	available_at_site: String,
	available_on_device: String,
	available_on_virtualmachine: String,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	group: Vec<String>,
	group__n: Vec<String>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	l2vpn: Vec<i64>,
	l2vpn__n: Vec<i64>,
	l2vpn_id: Vec<i64>,
	l2vpn_id__n: Vec<i64>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	status: Vec<String>,
	status__n: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	tenant: Vec<String>,
	tenant__n: Vec<String>,
	tenant_group: Vec<i64>,
	tenant_group__n: Vec<i64>,
	tenant_group_id: Vec<i64>,
	tenant_group_id__n: Vec<i64>,
	tenant_id: Vec<i64>,
	tenant_id__n: Vec<i64>,
	updated_by_request: String,
	vid: Vec<i32>,
	vid__empty: bool,
	vid__gt: Vec<i32>,
	vid__gte: Vec<i32>,
	vid__lt: Vec<i32>,
	vid__lte: Vec<i32>,
	vid__n: Vec<i32>
}
/// Get a list of VLAN objects.

pub fn ipam_vlans_list(query: IpamVlansListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansBulkUpdateQuery {
}
/// Put a list of VLAN objects.

pub fn ipam_vlans_bulk_update(query: IpamVlansBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansCreateQuery {
}
/// Post a list of VLAN objects.

pub fn ipam_vlans_create(query: IpamVlansCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/ipam/vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansBulkPartialUpdateQuery {
}
/// Patch a list of VLAN objects.

pub fn ipam_vlans_bulk_partial_update(query: IpamVlansBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVlansBulkDestroyQuery {
}
/// Delete a list of VLAN objects.

pub fn ipam_vlans_bulk_destroy(query: IpamVlansBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/vlans/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTerminationsPathsRetrieveQuery {
}
/// Return all CablePaths which traverse a given pass-through port.

pub fn circuits_circuit_terminations_paths_retrieve(query: CircuitsCircuitTerminationsPathsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuit-terminations/{id}/paths/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesRetrieveQuery {
}
/// Get a rack role object.

pub fn dcim_rack_roles_retrieve(query: DcimRackRolesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rack-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesUpdateQuery {
}
/// Put a rack role object.

pub fn dcim_rack_roles_update(query: DcimRackRolesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rack-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesPartialUpdateQuery {
}
/// Patch a rack role object.

pub fn dcim_rack_roles_partial_update(query: DcimRackRolesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rack-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRackRolesDestroyQuery {
}
/// Delete a rack role object.

pub fn dcim_rack_roles_destroy(query: DcimRackRolesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rack-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsListQuery {
	cable_end: String,
	cable_end__n: String,
	cabled: bool,
	color: Vec<String>,
	color__empty: bool,
	color__ic: Vec<String>,
	color__ie: Vec<String>,
	color__iew: Vec<String>,
	color__isw: Vec<String>,
	color__n: Vec<String>,
	color__nic: Vec<String>,
	color__nie: Vec<String>,
	color__niew: Vec<String>,
	color__nisw: Vec<String>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	module_id: Vec<i64>,
	module_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	occupied: bool,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ: Vec<String>,
	typ__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of front port objects.

pub fn dcim_front_ports_list(query: DcimFrontPortsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/front-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsBulkUpdateQuery {
}
/// Put a list of front port objects.

pub fn dcim_front_ports_bulk_update(query: DcimFrontPortsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/front-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsCreateQuery {
}
/// Post a list of front port objects.

pub fn dcim_front_ports_create(query: DcimFrontPortsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/front-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsBulkPartialUpdateQuery {
}
/// Patch a list of front port objects.

pub fn dcim_front_ports_bulk_partial_update(query: DcimFrontPortsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/front-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsBulkDestroyQuery {
}
/// Delete a list of front port objects.

pub fn dcim_front_ports_bulk_destroy(query: DcimFrontPortsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/front-ports/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	updated_by_request: String
}
/// Get a list of device bay template objects.

pub fn dcim_device_bay_templates_list(query: DcimDeviceBayTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesBulkUpdateQuery {
}
/// Put a list of device bay template objects.

pub fn dcim_device_bay_templates_bulk_update(query: DcimDeviceBayTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesCreateQuery {
}
/// Post a list of device bay template objects.

pub fn dcim_device_bay_templates_create(query: DcimDeviceBayTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/device-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of device bay template objects.

pub fn dcim_device_bay_templates_bulk_partial_update(query: DcimDeviceBayTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBayTemplatesBulkDestroyQuery {
}
/// Delete a list of device bay template objects.

pub fn dcim_device_bay_templates_bulk_destroy(query: DcimDeviceBayTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-bay-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksListQuery {
	content_typ_id: Vec<i32>,
	content_typ_id__empty: Vec<i32>,
	content_typ_id__gt: Vec<i32>,
	content_typ_id__gte: Vec<i32>,
	content_typ_id__lt: Vec<i32>,
	content_typ_id__lte: Vec<i32>,
	content_typ_id__n: Vec<i32>,
	content_typs: String,
	content_typs__ic: String,
	content_typs__ie: String,
	content_typs__iew: String,
	content_typs__isw: String,
	content_typs__n: String,
	content_typs__nic: String,
	content_typs__nie: String,
	content_typs__niew: String,
	content_typs__nisw: String,
	enabled: bool,
	group_name: Vec<String>,
	group_name__empty: bool,
	group_name__ic: Vec<String>,
	group_name__ie: Vec<String>,
	group_name__iew: Vec<String>,
	group_name__isw: Vec<String>,
	group_name__n: Vec<String>,
	group_name__nic: Vec<String>,
	group_name__nie: Vec<String>,
	group_name__niew: Vec<String>,
	group_name__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	link_text: String,
	link_text__ic: String,
	link_text__ie: String,
	link_text__iew: String,
	link_text__isw: String,
	link_text__n: String,
	link_text__nic: String,
	link_text__nie: String,
	link_text__niew: String,
	link_text__nisw: String,
	link_url: String,
	link_url__ic: String,
	link_url__ie: String,
	link_url__iew: String,
	link_url__isw: String,
	link_url__n: String,
	link_url__nic: String,
	link_url__nie: String,
	link_url__niew: String,
	link_url__nisw: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	new_window: bool,
	offset: i64,
	ordering: String,
	q: String,
	weight: Vec<i32>,
	weight__empty: bool,
	weight__gt: Vec<i32>,
	weight__gte: Vec<i32>,
	weight__lt: Vec<i32>,
	weight__lte: Vec<i32>,
	weight__n: Vec<i32>
}
/// Get a list of custom link objects.

pub fn extras_custom_links_list(query: ExtrasCustomLinksListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/custom-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksBulkUpdateQuery {
}
/// Put a list of custom link objects.

pub fn extras_custom_links_bulk_update(query: ExtrasCustomLinksBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/custom-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksCreateQuery {
}
/// Post a list of custom link objects.

pub fn extras_custom_links_create(query: ExtrasCustomLinksCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/custom-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksBulkPartialUpdateQuery {
}
/// Patch a list of custom link objects.

pub fn extras_custom_links_bulk_partial_update(query: ExtrasCustomLinksBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/custom-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasCustomLinksBulkDestroyQuery {
}
/// Delete a list of custom link objects.

pub fn extras_custom_links_bulk_destroy(query: ExtrasCustomLinksBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/custom-links/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesListQuery {
	action_object_id: Vec<i32>,
	action_object_id__empty: Vec<i32>,
	action_object_id__gt: Vec<i32>,
	action_object_id__gte: Vec<i32>,
	action_object_id__lt: Vec<i32>,
	action_object_id__lte: Vec<i32>,
	action_object_id__n: Vec<i32>,
	action_object_typ: String,
	action_object_typ__n: String,
	action_typ: Vec<String>,
	action_typ__n: Vec<String>,
	content_typ_id: Vec<i32>,
	content_typ_id__empty: Vec<i32>,
	content_typ_id__gt: Vec<i32>,
	content_typ_id__gte: Vec<i32>,
	content_typ_id__lt: Vec<i32>,
	content_typ_id__lte: Vec<i32>,
	content_typ_id__n: Vec<i32>,
	content_typs: String,
	content_typs__ic: String,
	content_typs__ie: String,
	content_typs__iew: String,
	content_typs__isw: String,
	content_typs__n: String,
	content_typs__nic: String,
	content_typs__nie: String,
	content_typs__niew: String,
	content_typs__nisw: String,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	enabled: bool,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	typ_create: bool,
	typ_delete: bool,
	typ_job_end: bool,
	typ_job_start: bool,
	typ_update: bool,
	updated_by_request: String
}
/// Get a list of event rule objects.

pub fn extras_event_rules_list(query: ExtrasEventRulesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/event-rules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesBulkUpdateQuery {
}
/// Put a list of event rule objects.

pub fn extras_event_rules_bulk_update(query: ExtrasEventRulesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/event-rules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesCreateQuery {
}
/// Post a list of event rule objects.

pub fn extras_event_rules_create(query: ExtrasEventRulesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/event-rules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesBulkPartialUpdateQuery {
}
/// Patch a list of event rule objects.

pub fn extras_event_rules_bulk_partial_update(query: ExtrasEventRulesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/event-rules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasEventRulesBulkDestroyQuery {
}
/// Delete a list of event rule objects.

pub fn extras_event_rules_bulk_destroy(query: ExtrasEventRulesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/event-rules/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesListQuery {
	allocated_draw: Vec<i32>,
	allocated_draw__empty: bool,
	allocated_draw__gt: Vec<i32>,
	allocated_draw__gte: Vec<i32>,
	allocated_draw__lt: Vec<i32>,
	allocated_draw__lte: Vec<i32>,
	allocated_draw__n: Vec<i32>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	devicetyp_id: Vec<i64>,
	devicetyp_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	maximum_draw: Vec<i32>,
	maximum_draw__empty: bool,
	maximum_draw__gt: Vec<i32>,
	maximum_draw__gte: Vec<i32>,
	maximum_draw__lt: Vec<i32>,
	maximum_draw__lte: Vec<i32>,
	maximum_draw__n: Vec<i32>,
	modified_by_request: String,
	moduletyp_id: Vec<i64>,
	moduletyp_id__n: Vec<i64>,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	typ: String,
	typ__n: String,
	updated_by_request: String
}
/// Get a list of power port template objects.

pub fn dcim_power_port_templates_list(query: DcimPowerPortTemplatesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesBulkUpdateQuery {
}
/// Put a list of power port template objects.

pub fn dcim_power_port_templates_bulk_update(query: DcimPowerPortTemplatesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesCreateQuery {
}
/// Post a list of power port template objects.

pub fn dcim_power_port_templates_create(query: DcimPowerPortTemplatesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/power-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesBulkPartialUpdateQuery {
}
/// Patch a list of power port template objects.

pub fn dcim_power_port_templates_bulk_partial_update(query: DcimPowerPortTemplatesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortTemplatesBulkDestroyQuery {
}
/// Delete a list of power port template objects.

pub fn dcim_power_port_templates_bulk_destroy(query: DcimPowerPortTemplatesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-port-templates/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsListQuery {
	content_typ: String,
	content_typ__n: String,
	content_typ_id: i64,
	content_typ_id__n: i64,
	created: String,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	object_id: Vec<i32>,
	object_id__empty: bool,
	object_id__gt: Vec<i32>,
	object_id__gte: Vec<i32>,
	object_id__lt: Vec<i32>,
	object_id__lte: Vec<i32>,
	object_id__n: Vec<i32>,
	offset: i64,
	ordering: String,
	q: String
}
/// Get a list of image attachment objects.

pub fn extras_image_attachments_list(query: ExtrasImageAttachmentsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/extras/image-attachments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsBulkUpdateQuery {
}
/// Put a list of image attachment objects.

pub fn extras_image_attachments_bulk_update(query: ExtrasImageAttachmentsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/extras/image-attachments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsCreateQuery {
}
/// Post a list of image attachment objects.

pub fn extras_image_attachments_create(query: ExtrasImageAttachmentsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/extras/image-attachments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsBulkPartialUpdateQuery {
}
/// Patch a list of image attachment objects.

pub fn extras_image_attachments_bulk_partial_update(query: ExtrasImageAttachmentsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/extras/image-attachments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrasImageAttachmentsBulkDestroyQuery {
}
/// Delete a list of image attachment objects.

pub fn extras_image_attachments_bulk_destroy(query: ExtrasImageAttachmentsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/extras/image-attachments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimFrontPortsPathsRetrieveQuery {
}
/// Return all CablePaths which traverse a given pass-through port.

pub fn dcim_front_ports_paths_retrieve(query: DcimFrontPortsPathsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/front-ports/{id}/paths/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsRetrieveQuery {
}
/// Get a provider account object.

pub fn circuits_provider_accounts_retrieve(query: CircuitsProviderAccountsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/provider-accounts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsUpdateQuery {
}
/// Put a provider account object.

pub fn circuits_provider_accounts_update(query: CircuitsProviderAccountsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/provider-accounts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsPartialUpdateQuery {
}
/// Patch a provider account object.

pub fn circuits_provider_accounts_partial_update(query: CircuitsProviderAccountsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/provider-accounts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsProviderAccountsDestroyQuery {
}
/// Delete a provider account object.

pub fn circuits_provider_accounts_destroy(query: CircuitsProviderAccountsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/provider-accounts/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesRetrieveQuery {
}
/// Get a console server port template object.

pub fn dcim_console_server_port_templates_retrieve(query: DcimConsoleServerPortTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/console-server-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesUpdateQuery {
}
/// Put a console server port template object.

pub fn dcim_console_server_port_templates_update(query: DcimConsoleServerPortTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/console-server-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesPartialUpdateQuery {
}
/// Patch a console server port template object.

pub fn dcim_console_server_port_templates_partial_update(query: DcimConsoleServerPortTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/console-server-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimConsoleServerPortTemplatesDestroyQuery {
}
/// Delete a console server port template object.

pub fn dcim_console_server_port_templates_destroy(query: DcimConsoleServerPortTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/console-server-port-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesRetrieveQuery {
}
/// Get a interface object.

pub fn virtualization_interfaces_retrieve(query: VirtualizationInterfacesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesUpdateQuery {
}
/// Put a interface object.

pub fn virtualization_interfaces_update(query: VirtualizationInterfacesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesPartialUpdateQuery {
}
/// Patch a interface object.

pub fn virtualization_interfaces_partial_update(query: VirtualizationInterfacesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationInterfacesDestroyQuery {
}
/// Delete a interface object.

pub fn virtualization_interfaces_destroy(query: VirtualizationInterfacesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/interfaces/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesRetrieveQuery {
}
/// Get a inventory item role object.

pub fn dcim_inventory_item_roles_retrieve(query: DcimInventoryItemRolesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/inventory-item-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesUpdateQuery {
}
/// Put a inventory item role object.

pub fn dcim_inventory_item_roles_update(query: DcimInventoryItemRolesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/inventory-item-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesPartialUpdateQuery {
}
/// Patch a inventory item role object.

pub fn dcim_inventory_item_roles_partial_update(query: DcimInventoryItemRolesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/inventory-item-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimInventoryItemRolesDestroyQuery {
}
/// Delete a inventory item role object.

pub fn dcim_inventory_item_roles_destroy(query: DcimInventoryItemRolesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/inventory-item-roles/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesRetrieveQuery {
}
/// Get a circuit type object.

pub fn circuits_circuit_types_retrieve(query: CircuitsCircuitTypesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/circuits/circuit-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesUpdateQuery {
}
/// Put a circuit type object.

pub fn circuits_circuit_types_update(query: CircuitsCircuitTypesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/circuits/circuit-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesPartialUpdateQuery {
}
/// Patch a circuit type object.

pub fn circuits_circuit_types_partial_update(query: CircuitsCircuitTypesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/circuits/circuit-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsCircuitTypesDestroyQuery {
}
/// Delete a circuit type object.

pub fn circuits_circuit_types_destroy(query: CircuitsCircuitTypesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/circuits/circuit-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsRetrieveQuery {
}
/// Get a rear port object.

pub fn dcim_rear_ports_retrieve(query: DcimRearPortsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/rear-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsUpdateQuery {
}
/// Put a rear port object.

pub fn dcim_rear_ports_update(query: DcimRearPortsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/rear-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsPartialUpdateQuery {
}
/// Patch a rear port object.

pub fn dcim_rear_ports_partial_update(query: DcimRearPortsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/rear-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimRearPortsDestroyQuery {
}
/// Delete a rear port object.

pub fn dcim_rear_ports_destroy(query: DcimRearPortsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/rear-ports/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesRetrieveQuery {
}
/// Get a service template object.

pub fn ipam_service_templates_retrieve(query: IpamServiceTemplatesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/service-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesUpdateQuery {
}
/// Put a service template object.

pub fn ipam_service_templates_update(query: IpamServiceTemplatesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/service-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesPartialUpdateQuery {
}
/// Patch a service template object.

pub fn ipam_service_templates_partial_update(query: IpamServiceTemplatesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/service-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamServiceTemplatesDestroyQuery {
}
/// Delete a service template object.

pub fn ipam_service_templates_destroy(query: IpamServiceTemplatesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/service-templates/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusRetrieveQuery {
}
/// A lightweight read-only endpoint for conveying NetBox's current operational status.

pub fn status_retrieve(query: StatusRetrieveQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/status/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	pfs_group: Vec<i64>,
	pfs_group__n: Vec<i64>,
	proposal: Vec<String>,
	proposal__empty: bool,
	proposal__ic: Vec<String>,
	proposal__ie: Vec<String>,
	proposal__iew: Vec<String>,
	proposal__isw: Vec<String>,
	proposal__n: Vec<String>,
	proposal__nic: Vec<String>,
	proposal__nie: Vec<String>,
	proposal__niew: Vec<String>,
	proposal__nisw: Vec<String>,
	proposal_id: Vec<i32>,
	proposal_id__empty: Vec<i32>,
	proposal_id__gt: Vec<i32>,
	proposal_id__gte: Vec<i32>,
	proposal_id__lt: Vec<i32>,
	proposal_id__lte: Vec<i32>,
	proposal_id__n: Vec<i32>,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of IPSec policy objects.

pub fn vpn_ipsec_policies_list(query: VpnIpsecPoliciesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ipsec-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesBulkUpdateQuery {
}
/// Put a list of IPSec policy objects.

pub fn vpn_ipsec_policies_bulk_update(query: VpnIpsecPoliciesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ipsec-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesCreateQuery {
}
/// Post a list of IPSec policy objects.

pub fn vpn_ipsec_policies_create(query: VpnIpsecPoliciesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/ipsec-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesBulkPartialUpdateQuery {
}
/// Patch a list of IPSec policy objects.

pub fn vpn_ipsec_policies_bulk_partial_update(query: VpnIpsecPoliciesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ipsec-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIpsecPoliciesBulkDestroyQuery {
}
/// Delete a list of IPSec policy objects.

pub fn vpn_ipsec_policies_bulk_destroy(query: VpnIpsecPoliciesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ipsec-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsRetrieveQuery {
}
/// Get a platform object.

pub fn dcim_platforms_retrieve(query: DcimPlatformsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/platforms/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsUpdateQuery {
}
/// Put a platform object.

pub fn dcim_platforms_update(query: DcimPlatformsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/platforms/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsPartialUpdateQuery {
}
/// Patch a platform object.

pub fn dcim_platforms_partial_update(query: DcimPlatformsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/platforms/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPlatformsDestroyQuery {
}
/// Delete a platform object.

pub fn dcim_platforms_destroy(query: DcimPlatformsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/platforms/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreDataFilesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	hash: Vec<String>,
	hash__empty: bool,
	hash__ic: Vec<String>,
	hash__ie: Vec<String>,
	hash__iew: Vec<String>,
	hash__isw: Vec<String>,
	hash__n: Vec<String>,
	hash__nic: Vec<String>,
	hash__nie: Vec<String>,
	hash__niew: Vec<String>,
	hash__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	offset: i64,
	ordering: String,
	path: Vec<String>,
	path__empty: bool,
	path__ic: Vec<String>,
	path__ie: Vec<String>,
	path__iew: Vec<String>,
	path__isw: Vec<String>,
	path__n: Vec<String>,
	path__nic: Vec<String>,
	path__nie: Vec<String>,
	path__niew: Vec<String>,
	path__nisw: Vec<String>,
	q: String,
	size: Vec<i32>,
	size__empty: bool,
	size__gt: Vec<i32>,
	size__gte: Vec<i32>,
	size__lt: Vec<i32>,
	size__lte: Vec<i32>,
	size__n: Vec<i32>,
	source: Vec<String>,
	source__n: Vec<String>,
	source_id: Vec<i64>,
	source_id__n: Vec<i64>,
	updated_by_request: String
}
/// Get a list of data file objects.

pub fn core_data_files_list(query: CoreDataFilesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/core/data-files/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansRetrieveQuery {
}
/// Get a wireless LAN object.

pub fn wireless_wireless_lans_retrieve(query: WirelessWirelessLansRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/wireless/wireless-lans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansUpdateQuery {
}
/// Put a wireless LAN object.

pub fn wireless_wireless_lans_update(query: WirelessWirelessLansUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/wireless/wireless-lans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansPartialUpdateQuery {
}
/// Patch a wireless LAN object.

pub fn wireless_wireless_lans_partial_update(query: WirelessWirelessLansPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/wireless/wireless-lans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WirelessWirelessLansDestroyQuery {
}
/// Delete a wireless LAN object.

pub fn wireless_wireless_lans_destroy(query: WirelessWirelessLansDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/wireless/wireless-lans/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsRetrieveQuery {
}
/// Get a power feed object.

pub fn dcim_power_feeds_retrieve(query: DcimPowerFeedsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-feeds/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsUpdateQuery {
}
/// Put a power feed object.

pub fn dcim_power_feeds_update(query: DcimPowerFeedsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/power-feeds/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsPartialUpdateQuery {
}
/// Patch a power feed object.

pub fn dcim_power_feeds_partial_update(query: DcimPowerFeedsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/power-feeds/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerFeedsDestroyQuery {
}
/// Delete a power feed object.

pub fn dcim_power_feeds_destroy(query: DcimPowerFeedsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/power-feeds/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsRetrieveQuery {
}
/// Get a tunnel termination object.

pub fn vpn_tunnel_terminations_retrieve(query: VpnTunnelTerminationsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/tunnel-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsUpdateQuery {
}
/// Put a tunnel termination object.

pub fn vpn_tunnel_terminations_update(query: VpnTunnelTerminationsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/tunnel-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsPartialUpdateQuery {
}
/// Patch a tunnel termination object.

pub fn vpn_tunnel_terminations_partial_update(query: VpnTunnelTerminationsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/tunnel-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnTunnelTerminationsDestroyQuery {
}
/// Delete a tunnel termination object.

pub fn vpn_tunnel_terminations_destroy(query: VpnTunnelTerminationsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/tunnel-terminations/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	device: Vec<String>,
	device__n: Vec<String>,
	device_id: Vec<i64>,
	device_id__n: Vec<i64>,
	device_role: Vec<String>,
	device_role__n: Vec<String>,
	device_role_id: Vec<i64>,
	device_role_id__n: Vec<i64>,
	device_typ: Vec<String>,
	device_typ__n: Vec<String>,
	device_typ_id: Vec<i64>,
	device_typ_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	label: Vec<String>,
	label__empty: bool,
	label__ic: Vec<String>,
	label__ie: Vec<String>,
	label__iew: Vec<String>,
	label__isw: Vec<String>,
	label__n: Vec<String>,
	label__nic: Vec<String>,
	label__nie: Vec<String>,
	label__niew: Vec<String>,
	label__nisw: Vec<String>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	location: Vec<String>,
	location__n: Vec<String>,
	location_id: Vec<i64>,
	location_id__n: Vec<i64>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	rack: Vec<String>,
	rack__n: Vec<String>,
	rack_id: Vec<i64>,
	rack_id__n: Vec<i64>,
	region: Vec<i64>,
	region__n: Vec<i64>,
	region_id: Vec<i64>,
	region_id__n: Vec<i64>,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	site: Vec<String>,
	site__n: Vec<String>,
	site_group: Vec<i64>,
	site_group__n: Vec<i64>,
	site_group_id: Vec<i64>,
	site_group_id__n: Vec<i64>,
	site_id: Vec<i64>,
	site_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	virtual_chassis: Vec<String>,
	virtual_chassis__n: Vec<String>,
	virtual_chassis_id: Vec<i64>,
	virtual_chassis_id__n: Vec<i64>
}
/// Get a list of device bay objects.

pub fn dcim_device_bays_list(query: DcimDeviceBaysListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/device-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysBulkUpdateQuery {
}
/// Put a list of device bay objects.

pub fn dcim_device_bays_bulk_update(query: DcimDeviceBaysBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/device-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysCreateQuery {
}
/// Post a list of device bay objects.

pub fn dcim_device_bays_create(query: DcimDeviceBaysCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/device-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysBulkPartialUpdateQuery {
}
/// Patch a list of device bay objects.

pub fn dcim_device_bays_bulk_partial_update(query: DcimDeviceBaysBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/device-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimDeviceBaysBulkDestroyQuery {
}
/// Delete a list of device bay objects.

pub fn dcim_device_bays_bulk_destroy(query: DcimDeviceBaysBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/device-bays/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimPowerPortsTraceRetrieveQuery {
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).

pub fn dcim_power_ports_trace_retrieve(query: DcimPowerPortsTraceRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/power-ports/{id}/trace/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsListQuery {
	contact_id: Vec<i64>,
	contact_id__n: Vec<i64>,
	content_typ: String,
	content_typ__n: String,
	content_typ_id: i64,
	content_typ_id__n: i64,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	group: Vec<i64>,
	group__n: Vec<i64>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	object_id: Vec<i32>,
	object_id__empty: bool,
	object_id__gt: Vec<i32>,
	object_id__gte: Vec<i32>,
	object_id__lt: Vec<i32>,
	object_id__lte: Vec<i32>,
	object_id__n: Vec<i32>,
	offset: i64,
	ordering: String,
	priority: String,
	priority__n: String,
	q: String,
	role: Vec<String>,
	role__n: Vec<String>,
	role_id: Vec<i64>,
	role_id__n: Vec<i64>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of contact assignment objects.

pub fn tenancy_contact_assignments_list(query: TenancyContactAssignmentsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contact-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsBulkUpdateQuery {
}
/// Put a list of contact assignment objects.

pub fn tenancy_contact_assignments_bulk_update(query: TenancyContactAssignmentsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contact-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsCreateQuery {
}
/// Post a list of contact assignment objects.

pub fn tenancy_contact_assignments_create(query: TenancyContactAssignmentsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/tenancy/contact-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsBulkPartialUpdateQuery {
}
/// Patch a list of contact assignment objects.

pub fn tenancy_contact_assignments_bulk_partial_update(query: TenancyContactAssignmentsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contact-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactAssignmentsBulkDestroyQuery {
}
/// Delete a list of contact assignment objects.

pub fn tenancy_contact_assignments_bulk_destroy(query: TenancyContactAssignmentsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contact-assignments/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of cluster group objects.

pub fn virtualization_cluster_groups_list(query: VirtualizationClusterGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/virtualization/cluster-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsBulkUpdateQuery {
}
/// Put a list of cluster group objects.

pub fn virtualization_cluster_groups_bulk_update(query: VirtualizationClusterGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/virtualization/cluster-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsCreateQuery {
}
/// Post a list of cluster group objects.

pub fn virtualization_cluster_groups_create(query: VirtualizationClusterGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/virtualization/cluster-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsBulkPartialUpdateQuery {
}
/// Patch a list of cluster group objects.

pub fn virtualization_cluster_groups_bulk_partial_update(query: VirtualizationClusterGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/virtualization/cluster-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationClusterGroupsBulkDestroyQuery {
}
/// Delete a list of cluster group objects.

pub fn virtualization_cluster_groups_bulk_destroy(query: VirtualizationClusterGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/virtualization/cluster-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	mode: Vec<String>,
	mode__n: Vec<String>,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	preshared_key: String,
	preshared_key__ic: String,
	preshared_key__ie: String,
	preshared_key__iew: String,
	preshared_key__isw: String,
	preshared_key__n: String,
	preshared_key__nic: String,
	preshared_key__nie: String,
	preshared_key__niew: String,
	preshared_key__nisw: String,
	proposal: Vec<String>,
	proposal__empty: bool,
	proposal__ic: Vec<String>,
	proposal__ie: Vec<String>,
	proposal__iew: Vec<String>,
	proposal__isw: Vec<String>,
	proposal__n: Vec<String>,
	proposal__nic: Vec<String>,
	proposal__nie: Vec<String>,
	proposal__niew: Vec<String>,
	proposal__nisw: Vec<String>,
	proposal_id: Vec<i32>,
	proposal_id__empty: Vec<i32>,
	proposal_id__gt: Vec<i32>,
	proposal_id__gte: Vec<i32>,
	proposal_id__lt: Vec<i32>,
	proposal_id__lte: Vec<i32>,
	proposal_id__n: Vec<i32>,
	q: String,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String,
	version: Vec<i64>,
	version__n: Vec<i64>
}
/// Get a list of IKE policy objects.

pub fn vpn_ike_policies_list(query: VpnIkePoliciesListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/vpn/ike-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesBulkUpdateQuery {
}
/// Put a list of IKE policy objects.

pub fn vpn_ike_policies_bulk_update(query: VpnIkePoliciesBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/vpn/ike-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesCreateQuery {
}
/// Post a list of IKE policy objects.

pub fn vpn_ike_policies_create(query: VpnIkePoliciesCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/vpn/ike-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesBulkPartialUpdateQuery {
}
/// Patch a list of IKE policy objects.

pub fn vpn_ike_policies_bulk_partial_update(query: VpnIkePoliciesBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/vpn/ike-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnIkePoliciesBulkDestroyQuery {
}
/// Delete a list of IKE policy objects.

pub fn vpn_ike_policies_bulk_destroy(query: VpnIkePoliciesBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/vpn/ike-policies/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersConfigRetrieveQuery {
}
/// An API endpoint via which a user can update his or her own UserConfig data (but no one else's).

pub fn users_config_retrieve(query: UsersConfigRetrieveQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/config/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesRetrieveQuery {
}
/// Get a module type object.

pub fn dcim_module_types_retrieve(query: DcimModuleTypesRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/module-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesUpdateQuery {
}
/// Put a module type object.

pub fn dcim_module_types_update(query: DcimModuleTypesUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/module-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesPartialUpdateQuery {
}
/// Patch a module type object.

pub fn dcim_module_types_partial_update(query: DcimModuleTypesPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/module-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimModuleTypesDestroyQuery {
}
/// Delete a module type object.

pub fn dcim_module_types_destroy(query: DcimModuleTypesDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/module-types/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsListQuery {
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	parent__n: Vec<String>,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of contact group objects.

pub fn tenancy_contact_groups_list(query: TenancyContactGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/tenancy/contact-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsBulkUpdateQuery {
}
/// Put a list of contact group objects.

pub fn tenancy_contact_groups_bulk_update(query: TenancyContactGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/tenancy/contact-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsCreateQuery {
}
/// Post a list of contact group objects.

pub fn tenancy_contact_groups_create(query: TenancyContactGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/tenancy/contact-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsBulkPartialUpdateQuery {
}
/// Patch a list of contact group objects.

pub fn tenancy_contact_groups_bulk_partial_update(query: TenancyContactGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/tenancy/contact-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenancyContactGroupsBulkDestroyQuery {
}
/// Delete a list of contact group objects.

pub fn tenancy_contact_groups_bulk_destroy(query: TenancyContactGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/tenancy/contact-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsListQuery {
	contact: Vec<i64>,
	contact__n: Vec<i64>,
	contact_group: Vec<i64>,
	contact_group__n: Vec<i64>,
	contact_role: Vec<i64>,
	contact_role__n: Vec<i64>,
	created: Vec<String>,
	created__empty: Vec<String>,
	created__gt: Vec<String>,
	created__gte: Vec<String>,
	created__lt: Vec<String>,
	created__lte: Vec<String>,
	created__n: Vec<String>,
	created_by_request: String,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	last_updated: Vec<String>,
	last_updated__empty: Vec<String>,
	last_updated__gt: Vec<String>,
	last_updated__gte: Vec<String>,
	last_updated__lt: Vec<String>,
	last_updated__lte: Vec<String>,
	last_updated__n: Vec<String>,
	limit: i64,
	modified_by_request: String,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	offset: i64,
	ordering: String,
	parent: Vec<String>,
	parent__n: Vec<String>,
	parent_id: Vec<i64>,
	parent_id__n: Vec<i64>,
	q: String,
	slug: Vec<String>,
	slug__empty: bool,
	slug__ic: Vec<String>,
	slug__ie: Vec<String>,
	slug__iew: Vec<String>,
	slug__isw: Vec<String>,
	slug__n: Vec<String>,
	slug__nic: Vec<String>,
	slug__nie: Vec<String>,
	slug__niew: Vec<String>,
	slug__nisw: Vec<String>,
	tag: Vec<String>,
	tag__n: Vec<String>,
	updated_by_request: String
}
/// Get a list of site group objects.

pub fn dcim_site_groups_list(query: DcimSiteGroupsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/dcim/site-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsBulkUpdateQuery {
}
/// Put a list of site group objects.

pub fn dcim_site_groups_bulk_update(query: DcimSiteGroupsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/dcim/site-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsCreateQuery {
}
/// Post a list of site group objects.

pub fn dcim_site_groups_create(query: DcimSiteGroupsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/dcim/site-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsBulkPartialUpdateQuery {
}
/// Patch a list of site group objects.

pub fn dcim_site_groups_bulk_partial_update(query: DcimSiteGroupsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/dcim/site-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcimSiteGroupsBulkDestroyQuery {
}
/// Delete a list of site group objects.

pub fn dcim_site_groups_bulk_destroy(query: DcimSiteGroupsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/dcim/site-groups/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsListQuery {
	can_add: bool,
	can_change: bool,
	can_delete: bool,
	can_view: bool,
	description: Vec<String>,
	description__empty: bool,
	description__ic: Vec<String>,
	description__ie: Vec<String>,
	description__iew: Vec<String>,
	description__isw: Vec<String>,
	description__n: Vec<String>,
	description__nic: Vec<String>,
	description__nie: Vec<String>,
	description__niew: Vec<String>,
	description__nisw: Vec<String>,
	enabled: bool,
	group: Vec<String>,
	group__n: Vec<String>,
	group_id: Vec<i64>,
	group_id__n: Vec<i64>,
	id: Vec<i32>,
	id__empty: bool,
	id__gt: Vec<i32>,
	id__gte: Vec<i32>,
	id__lt: Vec<i32>,
	id__lte: Vec<i32>,
	id__n: Vec<i32>,
	limit: i64,
	name: Vec<String>,
	name__empty: bool,
	name__ic: Vec<String>,
	name__ie: Vec<String>,
	name__iew: Vec<String>,
	name__isw: Vec<String>,
	name__n: Vec<String>,
	name__nic: Vec<String>,
	name__nie: Vec<String>,
	name__niew: Vec<String>,
	name__nisw: Vec<String>,
	object_typs: Vec<i64>,
	object_typs__n: Vec<i64>,
	offset: i64,
	ordering: String,
	q: String,
	user: Vec<String>,
	user__n: Vec<String>,
	user_id: Vec<i64>,
	user_id__n: Vec<i64>
}
/// Get a list of permission objects.

pub fn users_permissions_list(query: UsersPermissionsListQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/users/permissions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsBulkUpdateQuery {
}
/// Put a list of permission objects.

pub fn users_permissions_bulk_update(query: UsersPermissionsBulkUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/users/permissions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsCreateQuery {
}
/// Post a list of permission objects.

pub fn users_permissions_create(query: UsersPermissionsCreateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().post(format!("{}/api/users/permissions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsBulkPartialUpdateQuery {
}
/// Patch a list of permission objects.

pub fn users_permissions_bulk_partial_update(query: UsersPermissionsBulkPartialUpdateQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/users/permissions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPermissionsBulkDestroyQuery {
}
/// Delete a list of permission objects.

pub fn users_permissions_bulk_destroy(query: UsersPermissionsBulkDestroyQuery) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/users/permissions/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsRetrieveQuery {
}
/// Get a VRF object.

pub fn ipam_vrfs_retrieve(query: IpamVrfsRetrieveQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().get(format!("{}/api/ipam/vrfs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsUpdateQuery {
}
/// Put a VRF object.

pub fn ipam_vrfs_update(query: IpamVrfsUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().put(format!("{}/api/ipam/vrfs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsPartialUpdateQuery {
}
/// Patch a VRF object.

pub fn ipam_vrfs_partial_update(query: IpamVrfsPartialUpdateQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().patch(format!("{}/api/ipam/vrfs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamVrfsDestroyQuery {
}
/// Delete a VRF object.

pub fn ipam_vrfs_destroy(query: IpamVrfsDestroyQuery, id: i64) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return REQWEST_CLIENT.lock().unwrap().as_ref().unwrap().delete(format!("{}/api/ipam/vrfs/{id}/?{}", REQWEST_BASE_URL, serde_qs::to_string(&query).unwrap())).send();
}


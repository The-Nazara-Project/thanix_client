#![allow(warnings)]

use crate::util::{ThanixClient, remove_square_braces};
use crate::types::*;
use serde_qs;
use reqwest::{Error, blocking::Response};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitsCircuitTerminationsListQuery {
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	/// Circuit
	pub circuit_id: Option<Vec<i64>>,
	/// Circuit
	pub circuit_id__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub port_speed: Option<Vec<i64>>,
	pub port_speed__empty: Option<bool>,
	pub port_speed__gt: Option<Vec<i64>>,
	pub port_speed__gte: Option<Vec<i64>>,
	pub port_speed__lt: Option<Vec<i64>>,
	pub port_speed__lte: Option<Vec<i64>>,
	pub port_speed__n: Option<Vec<i64>>,
	/// ProviderNetwork (ID)
	pub provider_network_id: Option<Vec<Option<i64>>>,
	/// ProviderNetwork (ID)
	pub provider_network_id__n: Option<Vec<Option<i64>>>,
	/// Search
	pub q: Option<String>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site (ID)
	pub site_id: Option<Vec<Option<i64>>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<Option<i64>>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub term_side: Option<String>,
	pub term_side__n: Option<String>,
	pub updated_by_request: Option<String>,
	pub upstream_speed: Option<Vec<i64>>,
	pub upstream_speed__empty: Option<bool>,
	pub upstream_speed__gt: Option<Vec<i64>>,
	pub upstream_speed__gte: Option<Vec<i64>>,
	pub upstream_speed__lt: Option<Vec<i64>>,
	pub upstream_speed__lte: Option<Vec<i64>>,
	pub upstream_speed__n: Option<Vec<i64>>,
	pub xconnect_id: Option<Vec<String>>,
	pub xconnect_id__empty: Option<bool>,
	pub xconnect_id__ic: Option<Vec<String>>,
	pub xconnect_id__ie: Option<Vec<String>>,
	pub xconnect_id__iew: Option<Vec<String>>,
	pub xconnect_id__isw: Option<Vec<String>>,
	pub xconnect_id__n: Option<Vec<String>>,
	pub xconnect_id__nic: Option<Vec<String>>,
	pub xconnect_id__nie: Option<Vec<String>>,
	pub xconnect_id__niew: Option<Vec<String>>,
	pub xconnect_id__nisw: Option<Vec<String>>,

}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsListResponse {
	Http200(PaginatedCircuitTerminationList),
	Other(Response)
}
/// Get a list of circuit termination objects.
pub fn circuits_circuit_terminations_list(state: &ThanixClient, query: CircuitsCircuitTerminationsListQuery) -> Result<CircuitsCircuitTerminationsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/circuits/circuit-terminations/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsListResponse::Http200(r#response.json::<PaginatedCircuitTerminationList>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsBulkUpdateResponse {
	Http200(Vec<CircuitTermination>),
	Other(Response)
}
/// Put a list of circuit termination objects.
pub fn circuits_circuit_terminations_bulk_update(state: &ThanixClient, body: Vec<CircuitTerminationRequest>) -> Result<CircuitsCircuitTerminationsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/circuit-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsBulkUpdateResponse::Http200(r#response.json::<Vec<CircuitTermination>>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsCreateResponse {
	Http201(CircuitTermination),
	Other(Response)
}
/// Post a list of circuit termination objects.
pub fn circuits_circuit_terminations_create(state: &ThanixClient, body: WritableCircuitTerminationRequest) -> Result<CircuitsCircuitTerminationsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/circuits/circuit-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CircuitsCircuitTerminationsCreateResponse::Http201(r#response.json::<CircuitTermination>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of circuit termination objects.
pub fn circuits_circuit_terminations_bulk_destroy(state: &ThanixClient, body: Vec<CircuitTerminationRequest>) -> Result<CircuitsCircuitTerminationsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/circuit-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsCircuitTerminationsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsBulkPartialUpdateResponse {
	Http200(Vec<CircuitTermination>),
	Other(Response)
}
/// Patch a list of circuit termination objects.
pub fn circuits_circuit_terminations_bulk_partial_update(state: &ThanixClient, body: Vec<CircuitTerminationRequest>) -> Result<CircuitsCircuitTerminationsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/circuit-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<CircuitTermination>>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsRetrieveResponse {
	Http200(CircuitTermination),
	Other(Response)
}
/// Get a circuit termination object.
pub fn circuits_circuit_terminations_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitTerminationsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/circuit-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsRetrieveResponse::Http200(r#response.json::<CircuitTermination>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsUpdateResponse {
	Http200(CircuitTermination),
	Other(Response)
}
/// Put a circuit termination object.
pub fn circuits_circuit_terminations_update(state: &ThanixClient, body: WritableCircuitTerminationRequest, id: i64) -> Result<CircuitsCircuitTerminationsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/circuit-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsUpdateResponse::Http200(r#response.json::<CircuitTermination>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a circuit termination object.
pub fn circuits_circuit_terminations_destroy(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitTerminationsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/circuit-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsCircuitTerminationsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsPartialUpdateResponse {
	Http200(CircuitTermination),
	Other(Response)
}
/// Patch a circuit termination object.
pub fn circuits_circuit_terminations_partial_update(state: &ThanixClient, body: PatchedWritableCircuitTerminationRequest, id: i64) -> Result<CircuitsCircuitTerminationsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/circuit-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsPartialUpdateResponse::Http200(r#response.json::<CircuitTermination>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTerminationsPathsRetrieveResponse {
	Http200(CircuitTermination),
	Other(Response)
}
/// Return all CablePaths which traverse a given pass-through port.
pub fn circuits_circuit_terminations_paths_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitTerminationsPathsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/circuit-terminations/{id}/paths/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTerminationsPathsRetrieveResponse::Http200(r#response.json::<CircuitTermination>()?)) },
		r#other_status => { Ok(CircuitsCircuitTerminationsPathsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitsCircuitTypesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CircuitsCircuitTypesListResponse {
	Http200(PaginatedCircuitTypeList),
	Other(Response)
}
/// Get a list of circuit type objects.
pub fn circuits_circuit_types_list(state: &ThanixClient, query: CircuitsCircuitTypesListQuery) -> Result<CircuitsCircuitTypesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/circuits/circuit-types/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTypesListResponse::Http200(r#response.json::<PaginatedCircuitTypeList>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesBulkUpdateResponse {
	Http200(Vec<CircuitType>),
	Other(Response)
}
/// Put a list of circuit type objects.
pub fn circuits_circuit_types_bulk_update(state: &ThanixClient, body: Vec<CircuitTypeRequest>) -> Result<CircuitsCircuitTypesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/circuit-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTypesBulkUpdateResponse::Http200(r#response.json::<Vec<CircuitType>>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesCreateResponse {
	Http201(CircuitType),
	Other(Response)
}
/// Post a list of circuit type objects.
pub fn circuits_circuit_types_create(state: &ThanixClient, body: CircuitTypeRequest) -> Result<CircuitsCircuitTypesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/circuits/circuit-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CircuitsCircuitTypesCreateResponse::Http201(r#response.json::<CircuitType>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of circuit type objects.
pub fn circuits_circuit_types_bulk_destroy(state: &ThanixClient, body: Vec<CircuitTypeRequest>) -> Result<CircuitsCircuitTypesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/circuit-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsCircuitTypesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesBulkPartialUpdateResponse {
	Http200(Vec<CircuitType>),
	Other(Response)
}
/// Patch a list of circuit type objects.
pub fn circuits_circuit_types_bulk_partial_update(state: &ThanixClient, body: Vec<CircuitTypeRequest>) -> Result<CircuitsCircuitTypesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/circuit-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTypesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<CircuitType>>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesRetrieveResponse {
	Http200(CircuitType),
	Other(Response)
}
/// Get a circuit type object.
pub fn circuits_circuit_types_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitTypesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/circuit-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTypesRetrieveResponse::Http200(r#response.json::<CircuitType>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesUpdateResponse {
	Http200(CircuitType),
	Other(Response)
}
/// Put a circuit type object.
pub fn circuits_circuit_types_update(state: &ThanixClient, body: CircuitTypeRequest, id: i64) -> Result<CircuitsCircuitTypesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/circuit-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTypesUpdateResponse::Http200(r#response.json::<CircuitType>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a circuit type object.
pub fn circuits_circuit_types_destroy(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitTypesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/circuit-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsCircuitTypesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitTypesPartialUpdateResponse {
	Http200(CircuitType),
	Other(Response)
}
/// Patch a circuit type object.
pub fn circuits_circuit_types_partial_update(state: &ThanixClient, body: PatchedCircuitTypeRequest, id: i64) -> Result<CircuitsCircuitTypesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/circuit-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitTypesPartialUpdateResponse::Http200(r#response.json::<CircuitType>()?)) },
		r#other_status => { Ok(CircuitsCircuitTypesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitsCircuitsListQuery {
	pub cid: Option<Vec<String>>,
	pub cid__empty: Option<bool>,
	pub cid__ic: Option<Vec<String>>,
	pub cid__ie: Option<Vec<String>>,
	pub cid__iew: Option<Vec<String>>,
	pub cid__isw: Option<Vec<String>>,
	pub cid__n: Option<Vec<String>>,
	pub cid__nic: Option<Vec<String>>,
	pub cid__nie: Option<Vec<String>>,
	pub cid__niew: Option<Vec<String>>,
	pub cid__nisw: Option<Vec<String>>,
	pub commit_rate: Option<Vec<i64>>,
	pub commit_rate__empty: Option<bool>,
	pub commit_rate__gt: Option<Vec<i64>>,
	pub commit_rate__gte: Option<Vec<i64>>,
	pub commit_rate__lt: Option<Vec<i64>>,
	pub commit_rate__lte: Option<Vec<i64>>,
	pub commit_rate__n: Option<Vec<i64>>,
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub install_date: Option<Vec<String>>,
	pub install_date__empty: Option<bool>,
	pub install_date__gt: Option<Vec<String>>,
	pub install_date__gte: Option<Vec<String>>,
	pub install_date__lt: Option<Vec<String>>,
	pub install_date__lte: Option<Vec<String>>,
	pub install_date__n: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Provider (slug)
	pub provider: Option<Vec<String>>,
	/// Provider (slug)
	pub provider__n: Option<Vec<String>>,
	/// ProviderAccount (ID)
	pub provider_account_id: Option<Vec<i64>>,
	/// ProviderAccount (ID)
	pub provider_account_id__n: Option<Vec<i64>>,
	/// Provider (ID)
	pub provider_id: Option<Vec<i64>>,
	/// Provider (ID)
	pub provider_id__n: Option<Vec<i64>>,
	/// ProviderNetwork (ID)
	pub provider_network_id: Option<Vec<i64>>,
	/// ProviderNetwork (ID)
	pub provider_network_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub termination_date: Option<Vec<String>>,
	pub termination_date__empty: Option<bool>,
	pub termination_date__gt: Option<Vec<String>>,
	pub termination_date__gte: Option<Vec<String>>,
	pub termination_date__lt: Option<Vec<String>>,
	pub termination_date__lte: Option<Vec<String>>,
	pub termination_date__n: Option<Vec<String>>,
	/// Circuit type (slug)
	pub r#type: Option<Vec<String>>,
	/// Circuit type (slug)
	pub type__n: Option<Vec<String>>,
	/// Circuit type (ID)
	pub type_id: Option<Vec<i64>>,
	/// Circuit type (ID)
	pub type_id__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CircuitsCircuitsListResponse {
	Http200(PaginatedCircuitList),
	Other(Response)
}
/// Get a list of circuit objects.
pub fn circuits_circuits_list(state: &ThanixClient, query: CircuitsCircuitsListQuery) -> Result<CircuitsCircuitsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/circuits/circuits/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitsListResponse::Http200(r#response.json::<PaginatedCircuitList>()?)) },
		r#other_status => { Ok(CircuitsCircuitsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsBulkUpdateResponse {
	Http200(Vec<Circuit>),
	Other(Response)
}
/// Put a list of circuit objects.
pub fn circuits_circuits_bulk_update(state: &ThanixClient, body: Vec<CircuitRequest>) -> Result<CircuitsCircuitsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/circuits/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitsBulkUpdateResponse::Http200(r#response.json::<Vec<Circuit>>()?)) },
		r#other_status => { Ok(CircuitsCircuitsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsCreateResponse {
	Http201(Circuit),
	Other(Response)
}
/// Post a list of circuit objects.
pub fn circuits_circuits_create(state: &ThanixClient, body: WritableCircuitRequest) -> Result<CircuitsCircuitsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/circuits/circuits/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CircuitsCircuitsCreateResponse::Http201(r#response.json::<Circuit>()?)) },
		r#other_status => { Ok(CircuitsCircuitsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of circuit objects.
pub fn circuits_circuits_bulk_destroy(state: &ThanixClient, body: Vec<CircuitRequest>) -> Result<CircuitsCircuitsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/circuits/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsCircuitsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsBulkPartialUpdateResponse {
	Http200(Vec<Circuit>),
	Other(Response)
}
/// Patch a list of circuit objects.
pub fn circuits_circuits_bulk_partial_update(state: &ThanixClient, body: Vec<CircuitRequest>) -> Result<CircuitsCircuitsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/circuits/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Circuit>>()?)) },
		r#other_status => { Ok(CircuitsCircuitsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsRetrieveResponse {
	Http200(Circuit),
	Other(Response)
}
/// Get a circuit object.
pub fn circuits_circuits_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/circuits/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitsRetrieveResponse::Http200(r#response.json::<Circuit>()?)) },
		r#other_status => { Ok(CircuitsCircuitsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsUpdateResponse {
	Http200(Circuit),
	Other(Response)
}
/// Put a circuit object.
pub fn circuits_circuits_update(state: &ThanixClient, body: WritableCircuitRequest, id: i64) -> Result<CircuitsCircuitsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/circuits/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitsUpdateResponse::Http200(r#response.json::<Circuit>()?)) },
		r#other_status => { Ok(CircuitsCircuitsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a circuit object.
pub fn circuits_circuits_destroy(state: &ThanixClient, id: i64) -> Result<CircuitsCircuitsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/circuits/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsCircuitsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsCircuitsPartialUpdateResponse {
	Http200(Circuit),
	Other(Response)
}
/// Patch a circuit object.
pub fn circuits_circuits_partial_update(state: &ThanixClient, body: PatchedWritableCircuitRequest, id: i64) -> Result<CircuitsCircuitsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/circuits/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsCircuitsPartialUpdateResponse::Http200(r#response.json::<Circuit>()?)) },
		r#other_status => { Ok(CircuitsCircuitsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitsProviderAccountsListQuery {
	pub account: Option<Vec<String>>,
	pub account__empty: Option<bool>,
	pub account__ic: Option<Vec<String>>,
	pub account__ie: Option<Vec<String>>,
	pub account__iew: Option<Vec<String>>,
	pub account__isw: Option<Vec<String>>,
	pub account__n: Option<Vec<String>>,
	pub account__nic: Option<Vec<String>>,
	pub account__nie: Option<Vec<String>>,
	pub account__niew: Option<Vec<String>>,
	pub account__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Provider (slug)
	pub provider: Option<Vec<String>>,
	/// Provider (slug)
	pub provider__n: Option<Vec<String>>,
	/// Provider (ID)
	pub provider_id: Option<Vec<i64>>,
	/// Provider (ID)
	pub provider_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CircuitsProviderAccountsListResponse {
	Http200(PaginatedProviderAccountList),
	Other(Response)
}
/// Get a list of provider account objects.
pub fn circuits_provider_accounts_list(state: &ThanixClient, query: CircuitsProviderAccountsListQuery) -> Result<CircuitsProviderAccountsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/circuits/provider-accounts/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderAccountsListResponse::Http200(r#response.json::<PaginatedProviderAccountList>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsBulkUpdateResponse {
	Http200(Vec<ProviderAccount>),
	Other(Response)
}
/// Put a list of provider account objects.
pub fn circuits_provider_accounts_bulk_update(state: &ThanixClient, body: Vec<ProviderAccountRequest>) -> Result<CircuitsProviderAccountsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/provider-accounts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderAccountsBulkUpdateResponse::Http200(r#response.json::<Vec<ProviderAccount>>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsCreateResponse {
	Http201(ProviderAccount),
	Other(Response)
}
/// Post a list of provider account objects.
pub fn circuits_provider_accounts_create(state: &ThanixClient, body: WritableProviderAccountRequest) -> Result<CircuitsProviderAccountsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/circuits/provider-accounts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CircuitsProviderAccountsCreateResponse::Http201(r#response.json::<ProviderAccount>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of provider account objects.
pub fn circuits_provider_accounts_bulk_destroy(state: &ThanixClient, body: Vec<ProviderAccountRequest>) -> Result<CircuitsProviderAccountsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/provider-accounts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsProviderAccountsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsBulkPartialUpdateResponse {
	Http200(Vec<ProviderAccount>),
	Other(Response)
}
/// Patch a list of provider account objects.
pub fn circuits_provider_accounts_bulk_partial_update(state: &ThanixClient, body: Vec<ProviderAccountRequest>) -> Result<CircuitsProviderAccountsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/provider-accounts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderAccountsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ProviderAccount>>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsRetrieveResponse {
	Http200(ProviderAccount),
	Other(Response)
}
/// Get a provider account object.
pub fn circuits_provider_accounts_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsProviderAccountsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/provider-accounts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderAccountsRetrieveResponse::Http200(r#response.json::<ProviderAccount>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsUpdateResponse {
	Http200(ProviderAccount),
	Other(Response)
}
/// Put a provider account object.
pub fn circuits_provider_accounts_update(state: &ThanixClient, body: WritableProviderAccountRequest, id: i64) -> Result<CircuitsProviderAccountsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/provider-accounts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderAccountsUpdateResponse::Http200(r#response.json::<ProviderAccount>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a provider account object.
pub fn circuits_provider_accounts_destroy(state: &ThanixClient, id: i64) -> Result<CircuitsProviderAccountsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/provider-accounts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsProviderAccountsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderAccountsPartialUpdateResponse {
	Http200(ProviderAccount),
	Other(Response)
}
/// Patch a provider account object.
pub fn circuits_provider_accounts_partial_update(state: &ThanixClient, body: PatchedWritableProviderAccountRequest, id: i64) -> Result<CircuitsProviderAccountsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/provider-accounts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderAccountsPartialUpdateResponse::Http200(r#response.json::<ProviderAccount>()?)) },
		r#other_status => { Ok(CircuitsProviderAccountsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitsProviderNetworksListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Provider (slug)
	pub provider: Option<Vec<String>>,
	/// Provider (slug)
	pub provider__n: Option<Vec<String>>,
	/// Provider (ID)
	pub provider_id: Option<Vec<i64>>,
	/// Provider (ID)
	pub provider_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	pub service_id: Option<Vec<String>>,
	pub service_id__empty: Option<bool>,
	pub service_id__ic: Option<Vec<String>>,
	pub service_id__ie: Option<Vec<String>>,
	pub service_id__iew: Option<Vec<String>>,
	pub service_id__isw: Option<Vec<String>>,
	pub service_id__n: Option<Vec<String>>,
	pub service_id__nic: Option<Vec<String>>,
	pub service_id__nie: Option<Vec<String>>,
	pub service_id__niew: Option<Vec<String>>,
	pub service_id__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CircuitsProviderNetworksListResponse {
	Http200(PaginatedProviderNetworkList),
	Other(Response)
}
/// Get a list of provider network objects.
pub fn circuits_provider_networks_list(state: &ThanixClient, query: CircuitsProviderNetworksListQuery) -> Result<CircuitsProviderNetworksListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/circuits/provider-networks/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderNetworksListResponse::Http200(r#response.json::<PaginatedProviderNetworkList>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksBulkUpdateResponse {
	Http200(Vec<ProviderNetwork>),
	Other(Response)
}
/// Put a list of provider network objects.
pub fn circuits_provider_networks_bulk_update(state: &ThanixClient, body: Vec<ProviderNetworkRequest>) -> Result<CircuitsProviderNetworksBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/provider-networks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderNetworksBulkUpdateResponse::Http200(r#response.json::<Vec<ProviderNetwork>>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksCreateResponse {
	Http201(ProviderNetwork),
	Other(Response)
}
/// Post a list of provider network objects.
pub fn circuits_provider_networks_create(state: &ThanixClient, body: WritableProviderNetworkRequest) -> Result<CircuitsProviderNetworksCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/circuits/provider-networks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CircuitsProviderNetworksCreateResponse::Http201(r#response.json::<ProviderNetwork>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of provider network objects.
pub fn circuits_provider_networks_bulk_destroy(state: &ThanixClient, body: Vec<ProviderNetworkRequest>) -> Result<CircuitsProviderNetworksBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/provider-networks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsProviderNetworksBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksBulkPartialUpdateResponse {
	Http200(Vec<ProviderNetwork>),
	Other(Response)
}
/// Patch a list of provider network objects.
pub fn circuits_provider_networks_bulk_partial_update(state: &ThanixClient, body: Vec<ProviderNetworkRequest>) -> Result<CircuitsProviderNetworksBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/provider-networks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderNetworksBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ProviderNetwork>>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksRetrieveResponse {
	Http200(ProviderNetwork),
	Other(Response)
}
/// Get a provider network object.
pub fn circuits_provider_networks_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsProviderNetworksRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/provider-networks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderNetworksRetrieveResponse::Http200(r#response.json::<ProviderNetwork>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksUpdateResponse {
	Http200(ProviderNetwork),
	Other(Response)
}
/// Put a provider network object.
pub fn circuits_provider_networks_update(state: &ThanixClient, body: WritableProviderNetworkRequest, id: i64) -> Result<CircuitsProviderNetworksUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/provider-networks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderNetworksUpdateResponse::Http200(r#response.json::<ProviderNetwork>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a provider network object.
pub fn circuits_provider_networks_destroy(state: &ThanixClient, id: i64) -> Result<CircuitsProviderNetworksDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/provider-networks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsProviderNetworksDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProviderNetworksPartialUpdateResponse {
	Http200(ProviderNetwork),
	Other(Response)
}
/// Patch a provider network object.
pub fn circuits_provider_networks_partial_update(state: &ThanixClient, body: PatchedWritableProviderNetworkRequest, id: i64) -> Result<CircuitsProviderNetworksPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/provider-networks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProviderNetworksPartialUpdateResponse::Http200(r#response.json::<ProviderNetwork>()?)) },
		r#other_status => { Ok(CircuitsProviderNetworksPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CircuitsProvidersListQuery {
	/// ASN (ID)
	pub asn_id: Option<Vec<i64>>,
	/// ASN (ID)
	pub asn_id__n: Option<Vec<i64>>,
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site
	pub site_id: Option<Vec<i64>>,
	/// Site
	pub site_id__n: Option<Vec<i64>>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CircuitsProvidersListResponse {
	Http200(PaginatedProviderList),
	Other(Response)
}
/// Get a list of provider objects.
pub fn circuits_providers_list(state: &ThanixClient, query: CircuitsProvidersListQuery) -> Result<CircuitsProvidersListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/circuits/providers/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProvidersListResponse::Http200(r#response.json::<PaginatedProviderList>()?)) },
		r#other_status => { Ok(CircuitsProvidersListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersBulkUpdateResponse {
	Http200(Vec<Provider>),
	Other(Response)
}
/// Put a list of provider objects.
pub fn circuits_providers_bulk_update(state: &ThanixClient, body: Vec<ProviderRequest>) -> Result<CircuitsProvidersBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/providers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProvidersBulkUpdateResponse::Http200(r#response.json::<Vec<Provider>>()?)) },
		r#other_status => { Ok(CircuitsProvidersBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersCreateResponse {
	Http201(Provider),
	Other(Response)
}
/// Post a list of provider objects.
pub fn circuits_providers_create(state: &ThanixClient, body: WritableProviderRequest) -> Result<CircuitsProvidersCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/circuits/providers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CircuitsProvidersCreateResponse::Http201(r#response.json::<Provider>()?)) },
		r#other_status => { Ok(CircuitsProvidersCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of provider objects.
pub fn circuits_providers_bulk_destroy(state: &ThanixClient, body: Vec<ProviderRequest>) -> Result<CircuitsProvidersBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/providers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsProvidersBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersBulkPartialUpdateResponse {
	Http200(Vec<Provider>),
	Other(Response)
}
/// Patch a list of provider objects.
pub fn circuits_providers_bulk_partial_update(state: &ThanixClient, body: Vec<ProviderRequest>) -> Result<CircuitsProvidersBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/providers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProvidersBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Provider>>()?)) },
		r#other_status => { Ok(CircuitsProvidersBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersRetrieveResponse {
	Http200(Provider),
	Other(Response)
}
/// Get a provider object.
pub fn circuits_providers_retrieve(state: &ThanixClient, id: i64) -> Result<CircuitsProvidersRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/circuits/providers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProvidersRetrieveResponse::Http200(r#response.json::<Provider>()?)) },
		r#other_status => { Ok(CircuitsProvidersRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersUpdateResponse {
	Http200(Provider),
	Other(Response)
}
/// Put a provider object.
pub fn circuits_providers_update(state: &ThanixClient, body: WritableProviderRequest, id: i64) -> Result<CircuitsProvidersUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/circuits/providers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProvidersUpdateResponse::Http200(r#response.json::<Provider>()?)) },
		r#other_status => { Ok(CircuitsProvidersUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a provider object.
pub fn circuits_providers_destroy(state: &ThanixClient, id: i64) -> Result<CircuitsProvidersDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/circuits/providers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CircuitsProvidersDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CircuitsProvidersPartialUpdateResponse {
	Http200(Provider),
	Other(Response)
}
/// Patch a provider object.
pub fn circuits_providers_partial_update(state: &ThanixClient, body: PatchedWritableProviderRequest, id: i64) -> Result<CircuitsProvidersPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/circuits/providers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CircuitsProvidersPartialUpdateResponse::Http200(r#response.json::<Provider>()?)) },
		r#other_status => { Ok(CircuitsProvidersPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreDataFilesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub hash: Option<Vec<String>>,
	pub hash__empty: Option<bool>,
	pub hash__ic: Option<Vec<String>>,
	pub hash__ie: Option<Vec<String>>,
	pub hash__iew: Option<Vec<String>>,
	pub hash__isw: Option<Vec<String>>,
	pub hash__n: Option<Vec<String>>,
	pub hash__nic: Option<Vec<String>>,
	pub hash__nie: Option<Vec<String>>,
	pub hash__niew: Option<Vec<String>>,
	pub hash__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub path: Option<Vec<String>>,
	pub path__empty: Option<bool>,
	pub path__ic: Option<Vec<String>>,
	pub path__ie: Option<Vec<String>>,
	pub path__iew: Option<Vec<String>>,
	pub path__isw: Option<Vec<String>>,
	pub path__n: Option<Vec<String>>,
	pub path__nic: Option<Vec<String>>,
	pub path__nie: Option<Vec<String>>,
	pub path__niew: Option<Vec<String>>,
	pub path__nisw: Option<Vec<String>>,
	pub q: Option<String>,
	pub size: Option<Vec<i64>>,
	pub size__empty: Option<bool>,
	pub size__gt: Option<Vec<i64>>,
	pub size__gte: Option<Vec<i64>>,
	pub size__lt: Option<Vec<i64>>,
	pub size__lte: Option<Vec<i64>>,
	pub size__n: Option<Vec<i64>>,
	/// Data source (name)
	pub source: Option<Vec<String>>,
	/// Data source (name)
	pub source__n: Option<Vec<String>>,
	/// Data source (ID)
	pub source_id: Option<Vec<i64>>,
	/// Data source (ID)
	pub source_id__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CoreDataFilesListResponse {
	Http200(PaginatedDataFileList),
	Other(Response)
}
/// Get a list of data file objects.
pub fn core_data_files_list(state: &ThanixClient, query: CoreDataFilesListQuery) -> Result<CoreDataFilesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/core/data-files/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataFilesListResponse::Http200(r#response.json::<PaginatedDataFileList>()?)) },
		r#other_status => { Ok(CoreDataFilesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataFilesRetrieveResponse {
	Http200(DataFile),
	Other(Response)
}
/// Get a data file object.
pub fn core_data_files_retrieve(state: &ThanixClient, id: i64) -> Result<CoreDataFilesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/core/data-files/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataFilesRetrieveResponse::Http200(r#response.json::<DataFile>()?)) },
		r#other_status => { Ok(CoreDataFilesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreDataSourcesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub enabled: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum CoreDataSourcesListResponse {
	Http200(PaginatedDataSourceList),
	Other(Response)
}
/// Get a list of data source objects.
pub fn core_data_sources_list(state: &ThanixClient, query: CoreDataSourcesListQuery) -> Result<CoreDataSourcesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/core/data-sources/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesListResponse::Http200(r#response.json::<PaginatedDataSourceList>()?)) },
		r#other_status => { Ok(CoreDataSourcesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesBulkUpdateResponse {
	Http200(Vec<DataSource>),
	Other(Response)
}
/// Put a list of data source objects.
pub fn core_data_sources_bulk_update(state: &ThanixClient, body: Vec<DataSourceRequest>) -> Result<CoreDataSourcesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/core/data-sources/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesBulkUpdateResponse::Http200(r#response.json::<Vec<DataSource>>()?)) },
		r#other_status => { Ok(CoreDataSourcesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesCreateResponse {
	Http201(DataSource),
	Other(Response)
}
/// Post a list of data source objects.
pub fn core_data_sources_create(state: &ThanixClient, body: WritableDataSourceRequest) -> Result<CoreDataSourcesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/core/data-sources/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(CoreDataSourcesCreateResponse::Http201(r#response.json::<DataSource>()?)) },
		r#other_status => { Ok(CoreDataSourcesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of data source objects.
pub fn core_data_sources_bulk_destroy(state: &ThanixClient, body: Vec<DataSourceRequest>) -> Result<CoreDataSourcesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/core/data-sources/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CoreDataSourcesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesBulkPartialUpdateResponse {
	Http200(Vec<DataSource>),
	Other(Response)
}
/// Patch a list of data source objects.
pub fn core_data_sources_bulk_partial_update(state: &ThanixClient, body: Vec<DataSourceRequest>) -> Result<CoreDataSourcesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/core/data-sources/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<DataSource>>()?)) },
		r#other_status => { Ok(CoreDataSourcesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesRetrieveResponse {
	Http200(DataSource),
	Other(Response)
}
/// Get a data source object.
pub fn core_data_sources_retrieve(state: &ThanixClient, id: i64) -> Result<CoreDataSourcesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/core/data-sources/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesRetrieveResponse::Http200(r#response.json::<DataSource>()?)) },
		r#other_status => { Ok(CoreDataSourcesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesUpdateResponse {
	Http200(DataSource),
	Other(Response)
}
/// Put a data source object.
pub fn core_data_sources_update(state: &ThanixClient, body: WritableDataSourceRequest, id: i64) -> Result<CoreDataSourcesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/core/data-sources/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesUpdateResponse::Http200(r#response.json::<DataSource>()?)) },
		r#other_status => { Ok(CoreDataSourcesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a data source object.
pub fn core_data_sources_destroy(state: &ThanixClient, id: i64) -> Result<CoreDataSourcesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/core/data-sources/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(CoreDataSourcesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesPartialUpdateResponse {
	Http200(DataSource),
	Other(Response)
}
/// Patch a data source object.
pub fn core_data_sources_partial_update(state: &ThanixClient, body: PatchedWritableDataSourceRequest, id: i64) -> Result<CoreDataSourcesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/core/data-sources/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesPartialUpdateResponse::Http200(r#response.json::<DataSource>()?)) },
		r#other_status => { Ok(CoreDataSourcesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreDataSourcesSyncCreateResponse {
	Http200(DataSource),
	Other(Response)
}
/// Enqueue a job to synchronize the DataSource.
pub fn core_data_sources_sync_create(state: &ThanixClient, body: WritableDataSourceRequest, id: i64) -> Result<CoreDataSourcesSyncCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/core/data-sources/{id}/sync/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreDataSourcesSyncCreateResponse::Http200(r#response.json::<DataSource>()?)) },
		r#other_status => { Ok(CoreDataSourcesSyncCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreJobsListQuery {
	pub completed: Option<String>,
	pub completed__after: Option<String>,
	pub completed__before: Option<String>,
	pub created: Option<String>,
	pub created__after: Option<String>,
	pub created__before: Option<String>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub interval: Option<Vec<i64>>,
	pub interval__empty: Option<bool>,
	pub interval__gt: Option<Vec<i64>>,
	pub interval__gte: Option<Vec<i64>>,
	pub interval__lt: Option<Vec<i64>>,
	pub interval__lte: Option<Vec<i64>>,
	pub interval__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub object_id: Option<Vec<i64>>,
	pub object_id__empty: Option<bool>,
	pub object_id__gt: Option<Vec<i64>>,
	pub object_id__gte: Option<Vec<i64>>,
	pub object_id__lt: Option<Vec<i64>>,
	pub object_id__lte: Option<Vec<i64>>,
	pub object_id__n: Option<Vec<i64>>,
	pub object_type: Option<i64>,
	pub object_type__n: Option<i64>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub scheduled: Option<String>,
	pub scheduled__after: Option<String>,
	pub scheduled__before: Option<String>,
	pub started: Option<String>,
	pub started__after: Option<String>,
	pub started__before: Option<String>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub user: Option<i64>,
	pub user__n: Option<i64>,

}
#[derive(Debug)]
pub enum CoreJobsListResponse {
	Http200(PaginatedJobList),
	Other(Response)
}
/// Retrieve a list of job results
pub fn core_jobs_list(state: &ThanixClient, query: CoreJobsListQuery) -> Result<CoreJobsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/core/jobs/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreJobsListResponse::Http200(r#response.json::<PaginatedJobList>()?)) },
		r#other_status => { Ok(CoreJobsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum CoreJobsRetrieveResponse {
	Http200(Job),
	Other(Response)
}
/// Retrieve a list of job results
pub fn core_jobs_retrieve(state: &ThanixClient, id: i64) -> Result<CoreJobsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/core/jobs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(CoreJobsRetrieveResponse::Http200(r#response.json::<Job>()?)) },
		r#other_status => { Ok(CoreJobsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimCableTerminationsListQuery {
	pub cable: Option<i64>,
	pub cable__n: Option<i64>,
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub termination_id: Option<Vec<i64>>,
	pub termination_id__empty: Option<bool>,
	pub termination_id__gt: Option<Vec<i64>>,
	pub termination_id__gte: Option<Vec<i64>>,
	pub termination_id__lt: Option<Vec<i64>>,
	pub termination_id__lte: Option<Vec<i64>>,
	pub termination_id__n: Option<Vec<i64>>,
	pub termination_type: Option<String>,
	pub termination_type__n: Option<String>,

}
#[derive(Debug)]
pub enum DcimCableTerminationsListResponse {
	Http200(PaginatedCableTerminationList),
	Other(Response)
}
/// Get a list of cable termination objects.
pub fn dcim_cable_terminations_list(state: &ThanixClient, query: DcimCableTerminationsListQuery) -> Result<DcimCableTerminationsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/cable-terminations/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCableTerminationsListResponse::Http200(r#response.json::<PaginatedCableTerminationList>()?)) },
		r#other_status => { Ok(DcimCableTerminationsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsBulkUpdateResponse {
	Http200(Vec<CableTermination>),
	Other(Response)
}
/// Put a list of cable termination objects.
pub fn dcim_cable_terminations_bulk_update(state: &ThanixClient, body: Vec<CableTerminationRequest>) -> Result<DcimCableTerminationsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/cable-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCableTerminationsBulkUpdateResponse::Http200(r#response.json::<Vec<CableTermination>>()?)) },
		r#other_status => { Ok(DcimCableTerminationsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsCreateResponse {
	Http201(CableTermination),
	Other(Response)
}
/// Post a list of cable termination objects.
pub fn dcim_cable_terminations_create(state: &ThanixClient, body: CableTerminationRequest) -> Result<DcimCableTerminationsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/cable-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimCableTerminationsCreateResponse::Http201(r#response.json::<CableTermination>()?)) },
		r#other_status => { Ok(DcimCableTerminationsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of cable termination objects.
pub fn dcim_cable_terminations_bulk_destroy(state: &ThanixClient, body: Vec<CableTerminationRequest>) -> Result<DcimCableTerminationsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/cable-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimCableTerminationsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsBulkPartialUpdateResponse {
	Http200(Vec<CableTermination>),
	Other(Response)
}
/// Patch a list of cable termination objects.
pub fn dcim_cable_terminations_bulk_partial_update(state: &ThanixClient, body: Vec<CableTerminationRequest>) -> Result<DcimCableTerminationsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/cable-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCableTerminationsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<CableTermination>>()?)) },
		r#other_status => { Ok(DcimCableTerminationsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsRetrieveResponse {
	Http200(CableTermination),
	Other(Response)
}
/// Get a cable termination object.
pub fn dcim_cable_terminations_retrieve(state: &ThanixClient, id: i64) -> Result<DcimCableTerminationsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/cable-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCableTerminationsRetrieveResponse::Http200(r#response.json::<CableTermination>()?)) },
		r#other_status => { Ok(DcimCableTerminationsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsUpdateResponse {
	Http200(CableTermination),
	Other(Response)
}
/// Put a cable termination object.
pub fn dcim_cable_terminations_update(state: &ThanixClient, body: CableTerminationRequest, id: i64) -> Result<DcimCableTerminationsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/cable-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCableTerminationsUpdateResponse::Http200(r#response.json::<CableTermination>()?)) },
		r#other_status => { Ok(DcimCableTerminationsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a cable termination object.
pub fn dcim_cable_terminations_destroy(state: &ThanixClient, id: i64) -> Result<DcimCableTerminationsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/cable-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimCableTerminationsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCableTerminationsPartialUpdateResponse {
	Http200(CableTermination),
	Other(Response)
}
/// Patch a cable termination object.
pub fn dcim_cable_terminations_partial_update(state: &ThanixClient, body: PatchedCableTerminationRequest, id: i64) -> Result<DcimCableTerminationsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/cable-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCableTerminationsPartialUpdateResponse::Http200(r#response.json::<CableTermination>()?)) },
		r#other_status => { Ok(DcimCableTerminationsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimCablesListQuery {
	pub color: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub device: Option<Vec<String>>,
	pub device_id: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	pub length: Option<Vec<f64>>,
	pub length__empty: Option<bool>,
	pub length__gt: Option<Vec<f64>>,
	pub length__gte: Option<Vec<f64>>,
	pub length__lt: Option<Vec<f64>>,
	pub length__lte: Option<Vec<f64>>,
	pub length__n: Option<Vec<f64>>,
	pub length_unit: Option<String>,
	pub length_unit__n: Option<String>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub location: Option<Vec<String>>,
	pub location_id: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub rack: Option<Vec<String>>,
	pub rack_id: Option<Vec<i64>>,
	pub site: Option<Vec<String>>,
	pub site_id: Option<Vec<i64>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub termination_a_id: Option<Vec<i64>>,
	pub termination_a_type: Option<String>,
	pub termination_a_type__n: Option<String>,
	pub termination_b_id: Option<Vec<i64>>,
	pub termination_b_type: Option<String>,
	pub termination_b_type__n: Option<String>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	/// Unterminated
	pub unterminated: Option<bool>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimCablesListResponse {
	Http200(PaginatedCableList),
	Other(Response)
}
/// Get a list of cable objects.
pub fn dcim_cables_list(state: &ThanixClient, query: DcimCablesListQuery) -> Result<DcimCablesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/cables/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCablesListResponse::Http200(r#response.json::<PaginatedCableList>()?)) },
		r#other_status => { Ok(DcimCablesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesBulkUpdateResponse {
	Http200(Vec<Cable>),
	Other(Response)
}
/// Put a list of cable objects.
pub fn dcim_cables_bulk_update(state: &ThanixClient, body: Vec<CableRequest>) -> Result<DcimCablesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/cables/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCablesBulkUpdateResponse::Http200(r#response.json::<Vec<Cable>>()?)) },
		r#other_status => { Ok(DcimCablesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesCreateResponse {
	Http201(Cable),
	Other(Response)
}
/// Post a list of cable objects.
pub fn dcim_cables_create(state: &ThanixClient, body: WritableCableRequest) -> Result<DcimCablesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/cables/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimCablesCreateResponse::Http201(r#response.json::<Cable>()?)) },
		r#other_status => { Ok(DcimCablesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of cable objects.
pub fn dcim_cables_bulk_destroy(state: &ThanixClient, body: Vec<CableRequest>) -> Result<DcimCablesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/cables/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimCablesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesBulkPartialUpdateResponse {
	Http200(Vec<Cable>),
	Other(Response)
}
/// Patch a list of cable objects.
pub fn dcim_cables_bulk_partial_update(state: &ThanixClient, body: Vec<CableRequest>) -> Result<DcimCablesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/cables/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCablesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Cable>>()?)) },
		r#other_status => { Ok(DcimCablesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesRetrieveResponse {
	Http200(Cable),
	Other(Response)
}
/// Get a cable object.
pub fn dcim_cables_retrieve(state: &ThanixClient, id: i64) -> Result<DcimCablesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/cables/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCablesRetrieveResponse::Http200(r#response.json::<Cable>()?)) },
		r#other_status => { Ok(DcimCablesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesUpdateResponse {
	Http200(Cable),
	Other(Response)
}
/// Put a cable object.
pub fn dcim_cables_update(state: &ThanixClient, body: WritableCableRequest, id: i64) -> Result<DcimCablesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/cables/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCablesUpdateResponse::Http200(r#response.json::<Cable>()?)) },
		r#other_status => { Ok(DcimCablesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a cable object.
pub fn dcim_cables_destroy(state: &ThanixClient, id: i64) -> Result<DcimCablesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/cables/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimCablesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimCablesPartialUpdateResponse {
	Http200(Cable),
	Other(Response)
}
/// Patch a cable object.
pub fn dcim_cables_partial_update(state: &ThanixClient, body: PatchedWritableCableRequest, id: i64) -> Result<DcimCablesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/cables/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimCablesPartialUpdateResponse::Http200(r#response.json::<Cable>()?)) },
		r#other_status => { Ok(DcimCablesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimConnectedDeviceListQuery {
	/// The name of the peer device
	pub peer_device: Option<String>,
	/// The name of the peer interface
	pub peer_interface: Option<String>,

}
#[derive(Debug)]
pub enum DcimConnectedDeviceListResponse {
	Http200(Vec<Device>),
	Other(Response)
}
/// This endpoint allows a user to determine what device (if any) is connected to a given peer device and peer
/// interface. This is useful in a situation where a device boots with no configuration, but can detect its neighbors
/// via a protocol such as LLDP. Two query parameters must be included in the request:
/// 
/// * `peer_device`: The name of the peer device
/// * `peer_interface`: The name of the peer interface
pub fn dcim_connected_device_list(state: &ThanixClient, query: DcimConnectedDeviceListQuery) -> Result<DcimConnectedDeviceListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/connected-device/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConnectedDeviceListResponse::Http200(r#response.json::<Vec<Device>>()?)) },
		r#other_status => { Ok(DcimConnectedDeviceListResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimConsolePortTemplatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub r#type: Option<String>,
	pub type__n: Option<String>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesListResponse {
	Http200(PaginatedConsolePortTemplateList),
	Other(Response)
}
/// Get a list of console port template objects.
pub fn dcim_console_port_templates_list(state: &ThanixClient, query: DcimConsolePortTemplatesListQuery) -> Result<DcimConsolePortTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/console-port-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortTemplatesListResponse::Http200(r#response.json::<PaginatedConsolePortTemplateList>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesBulkUpdateResponse {
	Http200(Vec<ConsolePortTemplate>),
	Other(Response)
}
/// Put a list of console port template objects.
pub fn dcim_console_port_templates_bulk_update(state: &ThanixClient, body: Vec<ConsolePortTemplateRequest>) -> Result<DcimConsolePortTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<ConsolePortTemplate>>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesCreateResponse {
	Http201(ConsolePortTemplate),
	Other(Response)
}
/// Post a list of console port template objects.
pub fn dcim_console_port_templates_create(state: &ThanixClient, body: WritableConsolePortTemplateRequest) -> Result<DcimConsolePortTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/console-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimConsolePortTemplatesCreateResponse::Http201(r#response.json::<ConsolePortTemplate>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of console port template objects.
pub fn dcim_console_port_templates_bulk_destroy(state: &ThanixClient, body: Vec<ConsolePortTemplateRequest>) -> Result<DcimConsolePortTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsolePortTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesBulkPartialUpdateResponse {
	Http200(Vec<ConsolePortTemplate>),
	Other(Response)
}
/// Patch a list of console port template objects.
pub fn dcim_console_port_templates_bulk_partial_update(state: &ThanixClient, body: Vec<ConsolePortTemplateRequest>) -> Result<DcimConsolePortTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ConsolePortTemplate>>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesRetrieveResponse {
	Http200(ConsolePortTemplate),
	Other(Response)
}
/// Get a console port template object.
pub fn dcim_console_port_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimConsolePortTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/console-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortTemplatesRetrieveResponse::Http200(r#response.json::<ConsolePortTemplate>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesUpdateResponse {
	Http200(ConsolePortTemplate),
	Other(Response)
}
/// Put a console port template object.
pub fn dcim_console_port_templates_update(state: &ThanixClient, body: WritableConsolePortTemplateRequest, id: i64) -> Result<DcimConsolePortTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortTemplatesUpdateResponse::Http200(r#response.json::<ConsolePortTemplate>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a console port template object.
pub fn dcim_console_port_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimConsolePortTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsolePortTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortTemplatesPartialUpdateResponse {
	Http200(ConsolePortTemplate),
	Other(Response)
}
/// Patch a console port template object.
pub fn dcim_console_port_templates_partial_update(state: &ThanixClient, body: PatchedWritableConsolePortTemplateRequest, id: i64) -> Result<DcimConsolePortTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortTemplatesPartialUpdateResponse::Http200(r#response.json::<ConsolePortTemplate>()?)) },
		r#other_status => { Ok(DcimConsolePortTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimConsolePortsListQuery {
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub connected: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Physical port type
	pub r#type: Option<Vec<String>>,
	/// Physical port type
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimConsolePortsListResponse {
	Http200(PaginatedConsolePortList),
	Other(Response)
}
/// Get a list of console port objects.
pub fn dcim_console_ports_list(state: &ThanixClient, query: DcimConsolePortsListQuery) -> Result<DcimConsolePortsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/console-ports/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsListResponse::Http200(r#response.json::<PaginatedConsolePortList>()?)) },
		r#other_status => { Ok(DcimConsolePortsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsBulkUpdateResponse {
	Http200(Vec<ConsolePort>),
	Other(Response)
}
/// Put a list of console port objects.
pub fn dcim_console_ports_bulk_update(state: &ThanixClient, body: Vec<ConsolePortRequest>) -> Result<DcimConsolePortsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsBulkUpdateResponse::Http200(r#response.json::<Vec<ConsolePort>>()?)) },
		r#other_status => { Ok(DcimConsolePortsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsCreateResponse {
	Http201(ConsolePort),
	Other(Response)
}
/// Post a list of console port objects.
pub fn dcim_console_ports_create(state: &ThanixClient, body: WritableConsolePortRequest) -> Result<DcimConsolePortsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/console-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimConsolePortsCreateResponse::Http201(r#response.json::<ConsolePort>()?)) },
		r#other_status => { Ok(DcimConsolePortsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of console port objects.
pub fn dcim_console_ports_bulk_destroy(state: &ThanixClient, body: Vec<ConsolePortRequest>) -> Result<DcimConsolePortsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsolePortsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsBulkPartialUpdateResponse {
	Http200(Vec<ConsolePort>),
	Other(Response)
}
/// Patch a list of console port objects.
pub fn dcim_console_ports_bulk_partial_update(state: &ThanixClient, body: Vec<ConsolePortRequest>) -> Result<DcimConsolePortsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ConsolePort>>()?)) },
		r#other_status => { Ok(DcimConsolePortsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsRetrieveResponse {
	Http200(ConsolePort),
	Other(Response)
}
/// Get a console port object.
pub fn dcim_console_ports_retrieve(state: &ThanixClient, id: i64) -> Result<DcimConsolePortsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/console-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsRetrieveResponse::Http200(r#response.json::<ConsolePort>()?)) },
		r#other_status => { Ok(DcimConsolePortsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsUpdateResponse {
	Http200(ConsolePort),
	Other(Response)
}
/// Put a console port object.
pub fn dcim_console_ports_update(state: &ThanixClient, body: WritableConsolePortRequest, id: i64) -> Result<DcimConsolePortsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsUpdateResponse::Http200(r#response.json::<ConsolePort>()?)) },
		r#other_status => { Ok(DcimConsolePortsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a console port object.
pub fn dcim_console_ports_destroy(state: &ThanixClient, id: i64) -> Result<DcimConsolePortsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsolePortsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsPartialUpdateResponse {
	Http200(ConsolePort),
	Other(Response)
}
/// Patch a console port object.
pub fn dcim_console_ports_partial_update(state: &ThanixClient, body: PatchedWritableConsolePortRequest, id: i64) -> Result<DcimConsolePortsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsPartialUpdateResponse::Http200(r#response.json::<ConsolePort>()?)) },
		r#other_status => { Ok(DcimConsolePortsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsolePortsTraceRetrieveResponse {
	Http200(ConsolePort),
	Other(Response)
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).
pub fn dcim_console_ports_trace_retrieve(state: &ThanixClient, id: i64) -> Result<DcimConsolePortsTraceRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/console-ports/{id}/trace/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsolePortsTraceRetrieveResponse::Http200(r#response.json::<ConsolePort>()?)) },
		r#other_status => { Ok(DcimConsolePortsTraceRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimConsoleServerPortTemplatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub r#type: Option<String>,
	pub type__n: Option<String>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesListResponse {
	Http200(PaginatedConsoleServerPortTemplateList),
	Other(Response)
}
/// Get a list of console server port template objects.
pub fn dcim_console_server_port_templates_list(state: &ThanixClient, query: DcimConsoleServerPortTemplatesListQuery) -> Result<DcimConsoleServerPortTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/console-server-port-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortTemplatesListResponse::Http200(r#response.json::<PaginatedConsoleServerPortTemplateList>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesBulkUpdateResponse {
	Http200(Vec<ConsoleServerPortTemplate>),
	Other(Response)
}
/// Put a list of console server port template objects.
pub fn dcim_console_server_port_templates_bulk_update(state: &ThanixClient, body: Vec<ConsoleServerPortTemplateRequest>) -> Result<DcimConsoleServerPortTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-server-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<ConsoleServerPortTemplate>>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesCreateResponse {
	Http201(ConsoleServerPortTemplate),
	Other(Response)
}
/// Post a list of console server port template objects.
pub fn dcim_console_server_port_templates_create(state: &ThanixClient, body: WritableConsoleServerPortTemplateRequest) -> Result<DcimConsoleServerPortTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/console-server-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimConsoleServerPortTemplatesCreateResponse::Http201(r#response.json::<ConsoleServerPortTemplate>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of console server port template objects.
pub fn dcim_console_server_port_templates_bulk_destroy(state: &ThanixClient, body: Vec<ConsoleServerPortTemplateRequest>) -> Result<DcimConsoleServerPortTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-server-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsoleServerPortTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesBulkPartialUpdateResponse {
	Http200(Vec<ConsoleServerPortTemplate>),
	Other(Response)
}
/// Patch a list of console server port template objects.
pub fn dcim_console_server_port_templates_bulk_partial_update(state: &ThanixClient, body: Vec<ConsoleServerPortTemplateRequest>) -> Result<DcimConsoleServerPortTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-server-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ConsoleServerPortTemplate>>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesRetrieveResponse {
	Http200(ConsoleServerPortTemplate),
	Other(Response)
}
/// Get a console server port template object.
pub fn dcim_console_server_port_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimConsoleServerPortTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/console-server-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortTemplatesRetrieveResponse::Http200(r#response.json::<ConsoleServerPortTemplate>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesUpdateResponse {
	Http200(ConsoleServerPortTemplate),
	Other(Response)
}
/// Put a console server port template object.
pub fn dcim_console_server_port_templates_update(state: &ThanixClient, body: WritableConsoleServerPortTemplateRequest, id: i64) -> Result<DcimConsoleServerPortTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-server-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortTemplatesUpdateResponse::Http200(r#response.json::<ConsoleServerPortTemplate>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a console server port template object.
pub fn dcim_console_server_port_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimConsoleServerPortTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-server-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsoleServerPortTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortTemplatesPartialUpdateResponse {
	Http200(ConsoleServerPortTemplate),
	Other(Response)
}
/// Patch a console server port template object.
pub fn dcim_console_server_port_templates_partial_update(state: &ThanixClient, body: PatchedWritableConsoleServerPortTemplateRequest, id: i64) -> Result<DcimConsoleServerPortTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-server-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortTemplatesPartialUpdateResponse::Http200(r#response.json::<ConsoleServerPortTemplate>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimConsoleServerPortsListQuery {
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub connected: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Physical port type
	pub r#type: Option<Vec<String>>,
	/// Physical port type
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimConsoleServerPortsListResponse {
	Http200(PaginatedConsoleServerPortList),
	Other(Response)
}
/// Get a list of console server port objects.
pub fn dcim_console_server_ports_list(state: &ThanixClient, query: DcimConsoleServerPortsListQuery) -> Result<DcimConsoleServerPortsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/console-server-ports/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsListResponse::Http200(r#response.json::<PaginatedConsoleServerPortList>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsBulkUpdateResponse {
	Http200(Vec<ConsoleServerPort>),
	Other(Response)
}
/// Put a list of console server port objects.
pub fn dcim_console_server_ports_bulk_update(state: &ThanixClient, body: Vec<ConsoleServerPortRequest>) -> Result<DcimConsoleServerPortsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-server-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsBulkUpdateResponse::Http200(r#response.json::<Vec<ConsoleServerPort>>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsCreateResponse {
	Http201(ConsoleServerPort),
	Other(Response)
}
/// Post a list of console server port objects.
pub fn dcim_console_server_ports_create(state: &ThanixClient, body: WritableConsoleServerPortRequest) -> Result<DcimConsoleServerPortsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/console-server-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimConsoleServerPortsCreateResponse::Http201(r#response.json::<ConsoleServerPort>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of console server port objects.
pub fn dcim_console_server_ports_bulk_destroy(state: &ThanixClient, body: Vec<ConsoleServerPortRequest>) -> Result<DcimConsoleServerPortsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-server-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsoleServerPortsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsBulkPartialUpdateResponse {
	Http200(Vec<ConsoleServerPort>),
	Other(Response)
}
/// Patch a list of console server port objects.
pub fn dcim_console_server_ports_bulk_partial_update(state: &ThanixClient, body: Vec<ConsoleServerPortRequest>) -> Result<DcimConsoleServerPortsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-server-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ConsoleServerPort>>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsRetrieveResponse {
	Http200(ConsoleServerPort),
	Other(Response)
}
/// Get a console server port object.
pub fn dcim_console_server_ports_retrieve(state: &ThanixClient, id: i64) -> Result<DcimConsoleServerPortsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/console-server-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsRetrieveResponse::Http200(r#response.json::<ConsoleServerPort>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsUpdateResponse {
	Http200(ConsoleServerPort),
	Other(Response)
}
/// Put a console server port object.
pub fn dcim_console_server_ports_update(state: &ThanixClient, body: WritableConsoleServerPortRequest, id: i64) -> Result<DcimConsoleServerPortsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/console-server-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsUpdateResponse::Http200(r#response.json::<ConsoleServerPort>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a console server port object.
pub fn dcim_console_server_ports_destroy(state: &ThanixClient, id: i64) -> Result<DcimConsoleServerPortsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/console-server-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimConsoleServerPortsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsPartialUpdateResponse {
	Http200(ConsoleServerPort),
	Other(Response)
}
/// Patch a console server port object.
pub fn dcim_console_server_ports_partial_update(state: &ThanixClient, body: PatchedWritableConsoleServerPortRequest, id: i64) -> Result<DcimConsoleServerPortsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/console-server-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsPartialUpdateResponse::Http200(r#response.json::<ConsoleServerPort>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimConsoleServerPortsTraceRetrieveResponse {
	Http200(ConsoleServerPort),
	Other(Response)
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).
pub fn dcim_console_server_ports_trace_retrieve(state: &ThanixClient, id: i64) -> Result<DcimConsoleServerPortsTraceRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/console-server-ports/{id}/trace/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimConsoleServerPortsTraceRetrieveResponse::Http200(r#response.json::<ConsoleServerPort>()?)) },
		r#other_status => { Ok(DcimConsoleServerPortsTraceRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimDeviceBayTemplatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesListResponse {
	Http200(PaginatedDeviceBayTemplateList),
	Other(Response)
}
/// Get a list of device bay template objects.
pub fn dcim_device_bay_templates_list(state: &ThanixClient, query: DcimDeviceBayTemplatesListQuery) -> Result<DcimDeviceBayTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/device-bay-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBayTemplatesListResponse::Http200(r#response.json::<PaginatedDeviceBayTemplateList>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesBulkUpdateResponse {
	Http200(Vec<DeviceBayTemplate>),
	Other(Response)
}
/// Put a list of device bay template objects.
pub fn dcim_device_bay_templates_bulk_update(state: &ThanixClient, body: Vec<DeviceBayTemplateRequest>) -> Result<DcimDeviceBayTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBayTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<DeviceBayTemplate>>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesCreateResponse {
	Http201(DeviceBayTemplate),
	Other(Response)
}
/// Post a list of device bay template objects.
pub fn dcim_device_bay_templates_create(state: &ThanixClient, body: WritableDeviceBayTemplateRequest) -> Result<DcimDeviceBayTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/device-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimDeviceBayTemplatesCreateResponse::Http201(r#response.json::<DeviceBayTemplate>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of device bay template objects.
pub fn dcim_device_bay_templates_bulk_destroy(state: &ThanixClient, body: Vec<DeviceBayTemplateRequest>) -> Result<DcimDeviceBayTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceBayTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesBulkPartialUpdateResponse {
	Http200(Vec<DeviceBayTemplate>),
	Other(Response)
}
/// Patch a list of device bay template objects.
pub fn dcim_device_bay_templates_bulk_partial_update(state: &ThanixClient, body: Vec<DeviceBayTemplateRequest>) -> Result<DcimDeviceBayTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBayTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<DeviceBayTemplate>>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesRetrieveResponse {
	Http200(DeviceBayTemplate),
	Other(Response)
}
/// Get a device bay template object.
pub fn dcim_device_bay_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimDeviceBayTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/device-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBayTemplatesRetrieveResponse::Http200(r#response.json::<DeviceBayTemplate>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesUpdateResponse {
	Http200(DeviceBayTemplate),
	Other(Response)
}
/// Put a device bay template object.
pub fn dcim_device_bay_templates_update(state: &ThanixClient, body: WritableDeviceBayTemplateRequest, id: i64) -> Result<DcimDeviceBayTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBayTemplatesUpdateResponse::Http200(r#response.json::<DeviceBayTemplate>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a device bay template object.
pub fn dcim_device_bay_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimDeviceBayTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceBayTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBayTemplatesPartialUpdateResponse {
	Http200(DeviceBayTemplate),
	Other(Response)
}
/// Patch a device bay template object.
pub fn dcim_device_bay_templates_partial_update(state: &ThanixClient, body: PatchedWritableDeviceBayTemplateRequest, id: i64) -> Result<DcimDeviceBayTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBayTemplatesPartialUpdateResponse::Http200(r#response.json::<DeviceBayTemplate>()?)) },
		r#other_status => { Ok(DcimDeviceBayTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimDeviceBaysListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimDeviceBaysListResponse {
	Http200(PaginatedDeviceBayList),
	Other(Response)
}
/// Get a list of device bay objects.
pub fn dcim_device_bays_list(state: &ThanixClient, query: DcimDeviceBaysListQuery) -> Result<DcimDeviceBaysListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/device-bays/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBaysListResponse::Http200(r#response.json::<PaginatedDeviceBayList>()?)) },
		r#other_status => { Ok(DcimDeviceBaysListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysBulkUpdateResponse {
	Http200(Vec<DeviceBay>),
	Other(Response)
}
/// Put a list of device bay objects.
pub fn dcim_device_bays_bulk_update(state: &ThanixClient, body: Vec<DeviceBayRequest>) -> Result<DcimDeviceBaysBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBaysBulkUpdateResponse::Http200(r#response.json::<Vec<DeviceBay>>()?)) },
		r#other_status => { Ok(DcimDeviceBaysBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysCreateResponse {
	Http201(DeviceBay),
	Other(Response)
}
/// Post a list of device bay objects.
pub fn dcim_device_bays_create(state: &ThanixClient, body: WritableDeviceBayRequest) -> Result<DcimDeviceBaysCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/device-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimDeviceBaysCreateResponse::Http201(r#response.json::<DeviceBay>()?)) },
		r#other_status => { Ok(DcimDeviceBaysCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of device bay objects.
pub fn dcim_device_bays_bulk_destroy(state: &ThanixClient, body: Vec<DeviceBayRequest>) -> Result<DcimDeviceBaysBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceBaysBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysBulkPartialUpdateResponse {
	Http200(Vec<DeviceBay>),
	Other(Response)
}
/// Patch a list of device bay objects.
pub fn dcim_device_bays_bulk_partial_update(state: &ThanixClient, body: Vec<DeviceBayRequest>) -> Result<DcimDeviceBaysBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBaysBulkPartialUpdateResponse::Http200(r#response.json::<Vec<DeviceBay>>()?)) },
		r#other_status => { Ok(DcimDeviceBaysBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysRetrieveResponse {
	Http200(DeviceBay),
	Other(Response)
}
/// Get a device bay object.
pub fn dcim_device_bays_retrieve(state: &ThanixClient, id: i64) -> Result<DcimDeviceBaysRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/device-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBaysRetrieveResponse::Http200(r#response.json::<DeviceBay>()?)) },
		r#other_status => { Ok(DcimDeviceBaysRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysUpdateResponse {
	Http200(DeviceBay),
	Other(Response)
}
/// Put a device bay object.
pub fn dcim_device_bays_update(state: &ThanixClient, body: WritableDeviceBayRequest, id: i64) -> Result<DcimDeviceBaysUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBaysUpdateResponse::Http200(r#response.json::<DeviceBay>()?)) },
		r#other_status => { Ok(DcimDeviceBaysUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a device bay object.
pub fn dcim_device_bays_destroy(state: &ThanixClient, id: i64) -> Result<DcimDeviceBaysDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceBaysDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceBaysPartialUpdateResponse {
	Http200(DeviceBay),
	Other(Response)
}
/// Patch a device bay object.
pub fn dcim_device_bays_partial_update(state: &ThanixClient, body: PatchedWritableDeviceBayRequest, id: i64) -> Result<DcimDeviceBaysPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceBaysPartialUpdateResponse::Http200(r#response.json::<DeviceBay>()?)) },
		r#other_status => { Ok(DcimDeviceBaysPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimDeviceRolesListQuery {
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	/// Config template (ID)
	pub config_template_id: Option<Vec<Option<i64>>>,
	/// Config template (ID)
	pub config_template_id__n: Option<Vec<Option<i64>>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	pub vm_role: Option<bool>,

}
#[derive(Debug)]
pub enum DcimDeviceRolesListResponse {
	Http200(PaginatedDeviceRoleList),
	Other(Response)
}
/// Get a list of device role objects.
pub fn dcim_device_roles_list(state: &ThanixClient, query: DcimDeviceRolesListQuery) -> Result<DcimDeviceRolesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/device-roles/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceRolesListResponse::Http200(r#response.json::<PaginatedDeviceRoleList>()?)) },
		r#other_status => { Ok(DcimDeviceRolesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesBulkUpdateResponse {
	Http200(Vec<DeviceRole>),
	Other(Response)
}
/// Put a list of device role objects.
pub fn dcim_device_roles_bulk_update(state: &ThanixClient, body: Vec<DeviceRoleRequest>) -> Result<DcimDeviceRolesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceRolesBulkUpdateResponse::Http200(r#response.json::<Vec<DeviceRole>>()?)) },
		r#other_status => { Ok(DcimDeviceRolesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesCreateResponse {
	Http201(DeviceRole),
	Other(Response)
}
/// Post a list of device role objects.
pub fn dcim_device_roles_create(state: &ThanixClient, body: WritableDeviceRoleRequest) -> Result<DcimDeviceRolesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/device-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimDeviceRolesCreateResponse::Http201(r#response.json::<DeviceRole>()?)) },
		r#other_status => { Ok(DcimDeviceRolesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of device role objects.
pub fn dcim_device_roles_bulk_destroy(state: &ThanixClient, body: Vec<DeviceRoleRequest>) -> Result<DcimDeviceRolesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceRolesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesBulkPartialUpdateResponse {
	Http200(Vec<DeviceRole>),
	Other(Response)
}
/// Patch a list of device role objects.
pub fn dcim_device_roles_bulk_partial_update(state: &ThanixClient, body: Vec<DeviceRoleRequest>) -> Result<DcimDeviceRolesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceRolesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<DeviceRole>>()?)) },
		r#other_status => { Ok(DcimDeviceRolesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesRetrieveResponse {
	Http200(DeviceRole),
	Other(Response)
}
/// Get a device role object.
pub fn dcim_device_roles_retrieve(state: &ThanixClient, id: i64) -> Result<DcimDeviceRolesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/device-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceRolesRetrieveResponse::Http200(r#response.json::<DeviceRole>()?)) },
		r#other_status => { Ok(DcimDeviceRolesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesUpdateResponse {
	Http200(DeviceRole),
	Other(Response)
}
/// Put a device role object.
pub fn dcim_device_roles_update(state: &ThanixClient, body: WritableDeviceRoleRequest, id: i64) -> Result<DcimDeviceRolesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceRolesUpdateResponse::Http200(r#response.json::<DeviceRole>()?)) },
		r#other_status => { Ok(DcimDeviceRolesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a device role object.
pub fn dcim_device_roles_destroy(state: &ThanixClient, id: i64) -> Result<DcimDeviceRolesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceRolesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceRolesPartialUpdateResponse {
	Http200(DeviceRole),
	Other(Response)
}
/// Patch a device role object.
pub fn dcim_device_roles_partial_update(state: &ThanixClient, body: PatchedWritableDeviceRoleRequest, id: i64) -> Result<DcimDeviceRolesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceRolesPartialUpdateResponse::Http200(r#response.json::<DeviceRole>()?)) },
		r#other_status => { Ok(DcimDeviceRolesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimDeviceTypesListQuery {
	pub airflow: Option<String>,
	pub airflow__n: Option<String>,
	/// Has console ports
	pub console_ports: Option<bool>,
	/// Has console server ports
	pub console_server_ports: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	/// Default platform (slug)
	pub default_platform: Option<Vec<String>>,
	/// Default platform (slug)
	pub default_platform__n: Option<Vec<String>>,
	/// Default platform (ID)
	pub default_platform_id: Option<Vec<Option<i64>>>,
	/// Default platform (ID)
	pub default_platform_id__n: Option<Vec<Option<i64>>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Has device bays
	pub device_bays: Option<bool>,
	/// Has a front image
	pub has_front_image: Option<bool>,
	/// Has a rear image
	pub has_rear_image: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Has interfaces
	pub interfaces: Option<bool>,
	/// Has inventory items
	pub inventory_items: Option<bool>,
	pub is_full_depth: Option<bool>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<i64>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<i64>>,
	pub model: Option<Vec<String>>,
	pub model__empty: Option<bool>,
	pub model__ic: Option<Vec<String>>,
	pub model__ie: Option<Vec<String>>,
	pub model__iew: Option<Vec<String>>,
	pub model__isw: Option<Vec<String>>,
	pub model__n: Option<Vec<String>>,
	pub model__nic: Option<Vec<String>>,
	pub model__nie: Option<Vec<String>>,
	pub model__niew: Option<Vec<String>>,
	pub model__nisw: Option<Vec<String>>,
	pub modified_by_request: Option<String>,
	/// Has module bays
	pub module_bays: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub part_number: Option<Vec<String>>,
	pub part_number__empty: Option<bool>,
	pub part_number__ic: Option<Vec<String>>,
	pub part_number__ie: Option<Vec<String>>,
	pub part_number__iew: Option<Vec<String>>,
	pub part_number__isw: Option<Vec<String>>,
	pub part_number__n: Option<Vec<String>>,
	pub part_number__nic: Option<Vec<String>>,
	pub part_number__nie: Option<Vec<String>>,
	pub part_number__niew: Option<Vec<String>>,
	pub part_number__nisw: Option<Vec<String>>,
	/// Has pass-through ports
	pub pass_through_ports: Option<bool>,
	/// Has power outlets
	pub power_outlets: Option<bool>,
	/// Has power ports
	pub power_ports: Option<bool>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	pub subdevice_role: Option<String>,
	/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
	pub subdevice_role__n: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub u_height: Option<Vec<f64>>,
	pub u_height__empty: Option<bool>,
	pub u_height__gt: Option<Vec<f64>>,
	pub u_height__gte: Option<Vec<f64>>,
	pub u_height__lt: Option<Vec<f64>>,
	pub u_height__lte: Option<Vec<f64>>,
	pub u_height__n: Option<Vec<f64>>,
	pub updated_by_request: Option<String>,
	pub weight: Option<Vec<f64>>,
	pub weight__empty: Option<bool>,
	pub weight__gt: Option<Vec<f64>>,
	pub weight__gte: Option<Vec<f64>>,
	pub weight__lt: Option<Vec<f64>>,
	pub weight__lte: Option<Vec<f64>>,
	pub weight__n: Option<Vec<f64>>,
	pub weight_unit: Option<String>,
	pub weight_unit__n: Option<String>,

}
#[derive(Debug)]
pub enum DcimDeviceTypesListResponse {
	Http200(PaginatedDeviceTypeList),
	Other(Response)
}
/// Get a list of device type objects.
pub fn dcim_device_types_list(state: &ThanixClient, query: DcimDeviceTypesListQuery) -> Result<DcimDeviceTypesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/device-types/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceTypesListResponse::Http200(r#response.json::<PaginatedDeviceTypeList>()?)) },
		r#other_status => { Ok(DcimDeviceTypesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesBulkUpdateResponse {
	Http200(Vec<DeviceType>),
	Other(Response)
}
/// Put a list of device type objects.
pub fn dcim_device_types_bulk_update(state: &ThanixClient, body: Vec<DeviceTypeRequest>) -> Result<DcimDeviceTypesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceTypesBulkUpdateResponse::Http200(r#response.json::<Vec<DeviceType>>()?)) },
		r#other_status => { Ok(DcimDeviceTypesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesCreateResponse {
	Http201(DeviceType),
	Other(Response)
}
/// Post a list of device type objects.
pub fn dcim_device_types_create(state: &ThanixClient, body: WritableDeviceTypeRequest) -> Result<DcimDeviceTypesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/device-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimDeviceTypesCreateResponse::Http201(r#response.json::<DeviceType>()?)) },
		r#other_status => { Ok(DcimDeviceTypesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of device type objects.
pub fn dcim_device_types_bulk_destroy(state: &ThanixClient, body: Vec<DeviceTypeRequest>) -> Result<DcimDeviceTypesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceTypesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesBulkPartialUpdateResponse {
	Http200(Vec<DeviceType>),
	Other(Response)
}
/// Patch a list of device type objects.
pub fn dcim_device_types_bulk_partial_update(state: &ThanixClient, body: Vec<DeviceTypeRequest>) -> Result<DcimDeviceTypesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceTypesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<DeviceType>>()?)) },
		r#other_status => { Ok(DcimDeviceTypesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesRetrieveResponse {
	Http200(DeviceType),
	Other(Response)
}
/// Get a device type object.
pub fn dcim_device_types_retrieve(state: &ThanixClient, id: i64) -> Result<DcimDeviceTypesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/device-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceTypesRetrieveResponse::Http200(r#response.json::<DeviceType>()?)) },
		r#other_status => { Ok(DcimDeviceTypesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesUpdateResponse {
	Http200(DeviceType),
	Other(Response)
}
/// Put a device type object.
pub fn dcim_device_types_update(state: &ThanixClient, body: WritableDeviceTypeRequest, id: i64) -> Result<DcimDeviceTypesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/device-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceTypesUpdateResponse::Http200(r#response.json::<DeviceType>()?)) },
		r#other_status => { Ok(DcimDeviceTypesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a device type object.
pub fn dcim_device_types_destroy(state: &ThanixClient, id: i64) -> Result<DcimDeviceTypesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/device-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDeviceTypesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDeviceTypesPartialUpdateResponse {
	Http200(DeviceType),
	Other(Response)
}
/// Patch a device type object.
pub fn dcim_device_types_partial_update(state: &ThanixClient, body: PatchedWritableDeviceTypeRequest, id: i64) -> Result<DcimDeviceTypesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/device-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDeviceTypesPartialUpdateResponse::Http200(r#response.json::<DeviceType>()?)) },
		r#other_status => { Ok(DcimDeviceTypesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimDevicesListQuery {
	pub airflow: Option<String>,
	pub airflow__n: Option<String>,
	pub asset_tag: Option<Vec<String>>,
	pub asset_tag__empty: Option<bool>,
	pub asset_tag__ic: Option<Vec<String>>,
	pub asset_tag__ie: Option<Vec<String>>,
	pub asset_tag__iew: Option<Vec<String>>,
	pub asset_tag__isw: Option<Vec<String>>,
	pub asset_tag__n: Option<Vec<String>>,
	pub asset_tag__nic: Option<Vec<String>>,
	pub asset_tag__nie: Option<Vec<String>>,
	pub asset_tag__niew: Option<Vec<String>>,
	pub asset_tag__nisw: Option<Vec<String>>,
	/// VM cluster (ID)
	pub cluster_id: Option<Vec<Option<i64>>>,
	/// VM cluster (ID)
	pub cluster_id__n: Option<Vec<Option<i64>>>,
	/// Config template (ID)
	pub config_template_id: Option<Vec<Option<i64>>>,
	/// Config template (ID)
	pub config_template_id__n: Option<Vec<Option<i64>>>,
	/// Has console ports
	pub console_ports: Option<bool>,
	/// Has console server ports
	pub console_server_ports: Option<bool>,
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Has device bays
	pub device_bays: Option<bool>,
	/// Device type (slug)
	pub device_type: Option<Vec<String>>,
	/// Device type (slug)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub face: Option<String>,
	pub face__n: Option<String>,
	/// Has an out-of-band IP
	pub has_oob_ip: Option<bool>,
	/// Has a primary IP
	pub has_primary_ip: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Has interfaces
	pub interfaces: Option<bool>,
	/// Is full depth
	pub is_full_depth: Option<bool>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	pub latitude: Option<Vec<f64>>,
	pub latitude__empty: Option<bool>,
	pub latitude__gt: Option<Vec<f64>>,
	pub latitude__gte: Option<Vec<f64>>,
	pub latitude__lt: Option<Vec<f64>>,
	pub latitude__lte: Option<Vec<f64>>,
	pub latitude__n: Option<Vec<f64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Has local config context data
	pub local_context_data: Option<bool>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub longitude: Option<Vec<f64>>,
	pub longitude__empty: Option<bool>,
	pub longitude__gt: Option<Vec<f64>>,
	pub longitude__gte: Option<Vec<f64>>,
	pub longitude__lt: Option<Vec<f64>>,
	pub longitude__lte: Option<Vec<f64>>,
	pub longitude__n: Option<Vec<f64>>,
	pub mac_address: Option<Vec<String>>,
	pub mac_address__ic: Option<Vec<String>>,
	pub mac_address__ie: Option<Vec<String>>,
	pub mac_address__iew: Option<Vec<String>>,
	pub mac_address__isw: Option<Vec<String>>,
	pub mac_address__n: Option<Vec<String>>,
	pub mac_address__nic: Option<Vec<String>>,
	pub mac_address__nie: Option<Vec<String>>,
	pub mac_address__niew: Option<Vec<String>>,
	pub mac_address__nisw: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<i64>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<i64>>,
	/// Device model (slug)
	pub model: Option<Vec<String>>,
	/// Device model (slug)
	pub model__n: Option<Vec<String>>,
	pub modified_by_request: Option<String>,
	/// Has module bays
	pub module_bays: Option<bool>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// OOB IP (ID)
	pub oob_ip_id: Option<Vec<i64>>,
	/// OOB IP (ID)
	pub oob_ip_id__n: Option<Vec<i64>>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent Device (ID)
	pub parent_device_id: Option<Vec<i64>>,
	/// Parent Device (ID)
	pub parent_device_id__n: Option<Vec<i64>>,
	/// Has pass-through ports
	pub pass_through_ports: Option<bool>,
	/// Platform (slug)
	pub platform: Option<Vec<String>>,
	/// Platform (slug)
	pub platform__n: Option<Vec<String>>,
	/// Platform (ID)
	pub platform_id: Option<Vec<Option<i64>>>,
	/// Platform (ID)
	pub platform_id__n: Option<Vec<Option<i64>>>,
	pub position: Option<Vec<f64>>,
	pub position__empty: Option<bool>,
	pub position__gt: Option<Vec<f64>>,
	pub position__gte: Option<Vec<f64>>,
	pub position__lt: Option<Vec<f64>>,
	pub position__lte: Option<Vec<f64>>,
	pub position__n: Option<Vec<f64>>,
	/// Has power outlets
	pub power_outlets: Option<bool>,
	/// Has power ports
	pub power_ports: Option<bool>,
	/// Primary IPv4 (ID)
	pub primary_ip4_id: Option<Vec<i64>>,
	/// Primary IPv4 (ID)
	pub primary_ip4_id__n: Option<Vec<i64>>,
	/// Primary IPv6 (ID)
	pub primary_ip6_id: Option<Vec<i64>>,
	/// Primary IPv6 (ID)
	pub primary_ip6_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<i64>>,
	pub serial: Option<Vec<String>>,
	pub serial__empty: Option<bool>,
	pub serial__ic: Option<Vec<String>>,
	pub serial__ie: Option<Vec<String>>,
	pub serial__iew: Option<Vec<String>>,
	pub serial__isw: Option<Vec<String>>,
	pub serial__n: Option<Vec<String>>,
	pub serial__nic: Option<Vec<String>>,
	pub serial__nie: Option<Vec<String>>,
	pub serial__niew: Option<Vec<String>>,
	pub serial__nisw: Option<Vec<String>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	pub vc_position: Option<Vec<i64>>,
	pub vc_position__empty: Option<bool>,
	pub vc_position__gt: Option<Vec<i64>>,
	pub vc_position__gte: Option<Vec<i64>>,
	pub vc_position__lt: Option<Vec<i64>>,
	pub vc_position__lte: Option<Vec<i64>>,
	pub vc_position__n: Option<Vec<i64>>,
	pub vc_priority: Option<Vec<i64>>,
	pub vc_priority__empty: Option<bool>,
	pub vc_priority__gt: Option<Vec<i64>>,
	pub vc_priority__gte: Option<Vec<i64>>,
	pub vc_priority__lt: Option<Vec<i64>>,
	pub vc_priority__lte: Option<Vec<i64>>,
	pub vc_priority__n: Option<Vec<i64>>,
	/// Virtual chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,
	/// Is a virtual chassis member
	pub virtual_chassis_member: Option<bool>,

}
#[derive(Debug)]
pub enum DcimDevicesListResponse {
	Http200(PaginatedDeviceWithConfigContextList),
	Other(Response)
}
/// Get a list of device objects.
pub fn dcim_devices_list(state: &ThanixClient, query: DcimDevicesListQuery) -> Result<DcimDevicesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/devices/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesListResponse::Http200(r#response.json::<PaginatedDeviceWithConfigContextList>()?)) },
		r#other_status => { Ok(DcimDevicesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesBulkUpdateResponse {
	Http200(Vec<DeviceWithConfigContext>),
	Other(Response)
}
/// Put a list of device objects.
pub fn dcim_devices_bulk_update(state: &ThanixClient, body: Vec<DeviceWithConfigContextRequest>) -> Result<DcimDevicesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/devices/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesBulkUpdateResponse::Http200(r#response.json::<Vec<DeviceWithConfigContext>>()?)) },
		r#other_status => { Ok(DcimDevicesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesCreateResponse {
	Http201(DeviceWithConfigContext),
	Other(Response)
}
/// Post a list of device objects.
pub fn dcim_devices_create(state: &ThanixClient, body: WritableDeviceWithConfigContextRequest) -> Result<DcimDevicesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/devices/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimDevicesCreateResponse::Http201(r#response.json::<DeviceWithConfigContext>()?)) },
		r#other_status => { Ok(DcimDevicesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of device objects.
pub fn dcim_devices_bulk_destroy(state: &ThanixClient, body: Vec<DeviceWithConfigContextRequest>) -> Result<DcimDevicesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/devices/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDevicesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesBulkPartialUpdateResponse {
	Http200(Vec<DeviceWithConfigContext>),
	Other(Response)
}
/// Patch a list of device objects.
pub fn dcim_devices_bulk_partial_update(state: &ThanixClient, body: Vec<DeviceWithConfigContextRequest>) -> Result<DcimDevicesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/devices/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<DeviceWithConfigContext>>()?)) },
		r#other_status => { Ok(DcimDevicesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesRetrieveResponse {
	Http200(DeviceWithConfigContext),
	Other(Response)
}
/// Get a device object.
pub fn dcim_devices_retrieve(state: &ThanixClient, id: i64) -> Result<DcimDevicesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/devices/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesRetrieveResponse::Http200(r#response.json::<DeviceWithConfigContext>()?)) },
		r#other_status => { Ok(DcimDevicesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesUpdateResponse {
	Http200(DeviceWithConfigContext),
	Other(Response)
}
/// Put a device object.
pub fn dcim_devices_update(state: &ThanixClient, body: WritableDeviceWithConfigContextRequest, id: i64) -> Result<DcimDevicesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/devices/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesUpdateResponse::Http200(r#response.json::<DeviceWithConfigContext>()?)) },
		r#other_status => { Ok(DcimDevicesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a device object.
pub fn dcim_devices_destroy(state: &ThanixClient, id: i64) -> Result<DcimDevicesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/devices/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimDevicesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimDevicesPartialUpdateResponse {
	Http200(DeviceWithConfigContext),
	Other(Response)
}
/// Patch a device object.
pub fn dcim_devices_partial_update(state: &ThanixClient, body: PatchedWritableDeviceWithConfigContextRequest, id: i64) -> Result<DcimDevicesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/devices/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesPartialUpdateResponse::Http200(r#response.json::<DeviceWithConfigContext>()?)) },
		r#other_status => { Ok(DcimDevicesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimDevicesRenderConfigCreateQuery {
	pub format: Option<String>,

}
#[derive(Debug)]
pub enum DcimDevicesRenderConfigCreateResponse {
	Http200(DeviceWithConfigContext),
	Other(Response)
}
/// Resolve and render the preferred ConfigTemplate for this Device.
pub fn dcim_devices_render_config_create(state: &ThanixClient, query: DcimDevicesRenderConfigCreateQuery, body: WritableDeviceWithConfigContextRequest, id: i64) -> Result<DcimDevicesRenderConfigCreateResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.post(format!("{}/api/dcim/devices/{id}/render-config/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimDevicesRenderConfigCreateResponse::Http200(r#response.json::<DeviceWithConfigContext>()?)) },
		r#other_status => { Ok(DcimDevicesRenderConfigCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimFrontPortTemplatesListQuery {
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesListResponse {
	Http200(PaginatedFrontPortTemplateList),
	Other(Response)
}
/// Get a list of front port template objects.
pub fn dcim_front_port_templates_list(state: &ThanixClient, query: DcimFrontPortTemplatesListQuery) -> Result<DcimFrontPortTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/front-port-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortTemplatesListResponse::Http200(r#response.json::<PaginatedFrontPortTemplateList>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesBulkUpdateResponse {
	Http200(Vec<FrontPortTemplate>),
	Other(Response)
}
/// Put a list of front port template objects.
pub fn dcim_front_port_templates_bulk_update(state: &ThanixClient, body: Vec<FrontPortTemplateRequest>) -> Result<DcimFrontPortTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/front-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<FrontPortTemplate>>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesCreateResponse {
	Http201(FrontPortTemplate),
	Other(Response)
}
/// Post a list of front port template objects.
pub fn dcim_front_port_templates_create(state: &ThanixClient, body: WritableFrontPortTemplateRequest) -> Result<DcimFrontPortTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/front-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimFrontPortTemplatesCreateResponse::Http201(r#response.json::<FrontPortTemplate>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of front port template objects.
pub fn dcim_front_port_templates_bulk_destroy(state: &ThanixClient, body: Vec<FrontPortTemplateRequest>) -> Result<DcimFrontPortTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/front-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimFrontPortTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesBulkPartialUpdateResponse {
	Http200(Vec<FrontPortTemplate>),
	Other(Response)
}
/// Patch a list of front port template objects.
pub fn dcim_front_port_templates_bulk_partial_update(state: &ThanixClient, body: Vec<FrontPortTemplateRequest>) -> Result<DcimFrontPortTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/front-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<FrontPortTemplate>>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesRetrieveResponse {
	Http200(FrontPortTemplate),
	Other(Response)
}
/// Get a front port template object.
pub fn dcim_front_port_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimFrontPortTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/front-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortTemplatesRetrieveResponse::Http200(r#response.json::<FrontPortTemplate>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesUpdateResponse {
	Http200(FrontPortTemplate),
	Other(Response)
}
/// Put a front port template object.
pub fn dcim_front_port_templates_update(state: &ThanixClient, body: WritableFrontPortTemplateRequest, id: i64) -> Result<DcimFrontPortTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/front-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortTemplatesUpdateResponse::Http200(r#response.json::<FrontPortTemplate>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a front port template object.
pub fn dcim_front_port_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimFrontPortTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/front-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimFrontPortTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortTemplatesPartialUpdateResponse {
	Http200(FrontPortTemplate),
	Other(Response)
}
/// Patch a front port template object.
pub fn dcim_front_port_templates_partial_update(state: &ThanixClient, body: PatchedWritableFrontPortTemplateRequest, id: i64) -> Result<DcimFrontPortTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/front-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortTemplatesPartialUpdateResponse::Http200(r#response.json::<FrontPortTemplate>()?)) },
		r#other_status => { Ok(DcimFrontPortTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimFrontPortsListQuery {
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimFrontPortsListResponse {
	Http200(PaginatedFrontPortList),
	Other(Response)
}
/// Get a list of front port objects.
pub fn dcim_front_ports_list(state: &ThanixClient, query: DcimFrontPortsListQuery) -> Result<DcimFrontPortsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/front-ports/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsListResponse::Http200(r#response.json::<PaginatedFrontPortList>()?)) },
		r#other_status => { Ok(DcimFrontPortsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsBulkUpdateResponse {
	Http200(Vec<FrontPort>),
	Other(Response)
}
/// Put a list of front port objects.
pub fn dcim_front_ports_bulk_update(state: &ThanixClient, body: Vec<FrontPortRequest>) -> Result<DcimFrontPortsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/front-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsBulkUpdateResponse::Http200(r#response.json::<Vec<FrontPort>>()?)) },
		r#other_status => { Ok(DcimFrontPortsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsCreateResponse {
	Http201(FrontPort),
	Other(Response)
}
/// Post a list of front port objects.
pub fn dcim_front_ports_create(state: &ThanixClient, body: WritableFrontPortRequest) -> Result<DcimFrontPortsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/front-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimFrontPortsCreateResponse::Http201(r#response.json::<FrontPort>()?)) },
		r#other_status => { Ok(DcimFrontPortsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of front port objects.
pub fn dcim_front_ports_bulk_destroy(state: &ThanixClient, body: Vec<FrontPortRequest>) -> Result<DcimFrontPortsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/front-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimFrontPortsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsBulkPartialUpdateResponse {
	Http200(Vec<FrontPort>),
	Other(Response)
}
/// Patch a list of front port objects.
pub fn dcim_front_ports_bulk_partial_update(state: &ThanixClient, body: Vec<FrontPortRequest>) -> Result<DcimFrontPortsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/front-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<FrontPort>>()?)) },
		r#other_status => { Ok(DcimFrontPortsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsRetrieveResponse {
	Http200(FrontPort),
	Other(Response)
}
/// Get a front port object.
pub fn dcim_front_ports_retrieve(state: &ThanixClient, id: i64) -> Result<DcimFrontPortsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/front-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsRetrieveResponse::Http200(r#response.json::<FrontPort>()?)) },
		r#other_status => { Ok(DcimFrontPortsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsUpdateResponse {
	Http200(FrontPort),
	Other(Response)
}
/// Put a front port object.
pub fn dcim_front_ports_update(state: &ThanixClient, body: WritableFrontPortRequest, id: i64) -> Result<DcimFrontPortsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/front-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsUpdateResponse::Http200(r#response.json::<FrontPort>()?)) },
		r#other_status => { Ok(DcimFrontPortsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a front port object.
pub fn dcim_front_ports_destroy(state: &ThanixClient, id: i64) -> Result<DcimFrontPortsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/front-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimFrontPortsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsPartialUpdateResponse {
	Http200(FrontPort),
	Other(Response)
}
/// Patch a front port object.
pub fn dcim_front_ports_partial_update(state: &ThanixClient, body: PatchedWritableFrontPortRequest, id: i64) -> Result<DcimFrontPortsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/front-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsPartialUpdateResponse::Http200(r#response.json::<FrontPort>()?)) },
		r#other_status => { Ok(DcimFrontPortsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimFrontPortsPathsRetrieveResponse {
	Http200(FrontPort),
	Other(Response)
}
/// Return all CablePaths which traverse a given pass-through port.
pub fn dcim_front_ports_paths_retrieve(state: &ThanixClient, id: i64) -> Result<DcimFrontPortsPathsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/front-ports/{id}/paths/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimFrontPortsPathsRetrieveResponse::Http200(r#response.json::<FrontPort>()?)) },
		r#other_status => { Ok(DcimFrontPortsPathsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimInterfaceTemplatesListQuery {
	pub bridge_id: Option<Vec<i64>>,
	pub bridge_id__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	pub enabled: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub mgmt_only: Option<bool>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub poe_mode: Option<Vec<String>>,
	pub poe_mode__n: Option<Vec<String>>,
	pub poe_type: Option<Vec<String>>,
	pub poe_type__n: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	pub rf_role: Option<Vec<String>>,
	pub rf_role__n: Option<Vec<String>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesListResponse {
	Http200(PaginatedInterfaceTemplateList),
	Other(Response)
}
/// Get a list of interface template objects.
pub fn dcim_interface_templates_list(state: &ThanixClient, query: DcimInterfaceTemplatesListQuery) -> Result<DcimInterfaceTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/interface-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfaceTemplatesListResponse::Http200(r#response.json::<PaginatedInterfaceTemplateList>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesBulkUpdateResponse {
	Http200(Vec<InterfaceTemplate>),
	Other(Response)
}
/// Put a list of interface template objects.
pub fn dcim_interface_templates_bulk_update(state: &ThanixClient, body: Vec<InterfaceTemplateRequest>) -> Result<DcimInterfaceTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/interface-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfaceTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<InterfaceTemplate>>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesCreateResponse {
	Http201(InterfaceTemplate),
	Other(Response)
}
/// Post a list of interface template objects.
pub fn dcim_interface_templates_create(state: &ThanixClient, body: WritableInterfaceTemplateRequest) -> Result<DcimInterfaceTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/interface-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimInterfaceTemplatesCreateResponse::Http201(r#response.json::<InterfaceTemplate>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of interface template objects.
pub fn dcim_interface_templates_bulk_destroy(state: &ThanixClient, body: Vec<InterfaceTemplateRequest>) -> Result<DcimInterfaceTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/interface-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInterfaceTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesBulkPartialUpdateResponse {
	Http200(Vec<InterfaceTemplate>),
	Other(Response)
}
/// Patch a list of interface template objects.
pub fn dcim_interface_templates_bulk_partial_update(state: &ThanixClient, body: Vec<InterfaceTemplateRequest>) -> Result<DcimInterfaceTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/interface-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfaceTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<InterfaceTemplate>>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesRetrieveResponse {
	Http200(InterfaceTemplate),
	Other(Response)
}
/// Get a interface template object.
pub fn dcim_interface_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimInterfaceTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/interface-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfaceTemplatesRetrieveResponse::Http200(r#response.json::<InterfaceTemplate>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesUpdateResponse {
	Http200(InterfaceTemplate),
	Other(Response)
}
/// Put a interface template object.
pub fn dcim_interface_templates_update(state: &ThanixClient, body: WritableInterfaceTemplateRequest, id: i64) -> Result<DcimInterfaceTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/interface-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfaceTemplatesUpdateResponse::Http200(r#response.json::<InterfaceTemplate>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a interface template object.
pub fn dcim_interface_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimInterfaceTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/interface-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInterfaceTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfaceTemplatesPartialUpdateResponse {
	Http200(InterfaceTemplate),
	Other(Response)
}
/// Patch a interface template object.
pub fn dcim_interface_templates_partial_update(state: &ThanixClient, body: PatchedWritableInterfaceTemplateRequest, id: i64) -> Result<DcimInterfaceTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/interface-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfaceTemplatesPartialUpdateResponse::Http200(r#response.json::<InterfaceTemplate>()?)) },
		r#other_status => { Ok(DcimInterfaceTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimInterfacesListQuery {
	/// Bridged interface (ID)
	pub bridge_id: Option<Vec<i64>>,
	/// Bridged interface (ID)
	pub bridge_id__n: Option<Vec<i64>>,
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub connected: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub duplex: Option<Vec<Option<String>>>,
	pub duplex__n: Option<Vec<Option<String>>>,
	pub enabled: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Kind of interface
	pub kind: Option<String>,
	/// L2VPN
	pub l2vpn: Option<Vec<Option<i64>>>,
	/// L2VPN
	pub l2vpn__n: Option<Vec<Option<i64>>>,
	/// L2VPN (ID)
	pub l2vpn_id: Option<Vec<i64>>,
	/// L2VPN (ID)
	pub l2vpn_id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	/// LAG interface (ID)
	pub lag_id: Option<Vec<i64>>,
	/// LAG interface (ID)
	pub lag_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub mac_address: Option<Vec<String>>,
	pub mac_address__ic: Option<Vec<String>>,
	pub mac_address__ie: Option<Vec<String>>,
	pub mac_address__iew: Option<Vec<String>>,
	pub mac_address__isw: Option<Vec<String>>,
	pub mac_address__n: Option<Vec<String>>,
	pub mac_address__nic: Option<Vec<String>>,
	pub mac_address__nie: Option<Vec<String>>,
	pub mac_address__niew: Option<Vec<String>>,
	pub mac_address__nisw: Option<Vec<String>>,
	pub mgmt_only: Option<bool>,
	/// IEEE 802.1Q tagging strategy
	pub mode: Option<String>,
	/// IEEE 802.1Q tagging strategy
	pub mode__n: Option<String>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub mtu: Option<Vec<i64>>,
	pub mtu__empty: Option<bool>,
	pub mtu__gt: Option<Vec<i64>>,
	pub mtu__gte: Option<Vec<i64>>,
	pub mtu__lt: Option<Vec<i64>>,
	pub mtu__lte: Option<Vec<i64>>,
	pub mtu__n: Option<Vec<i64>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent interface (ID)
	pub parent_id: Option<Vec<i64>>,
	/// Parent interface (ID)
	pub parent_id__n: Option<Vec<i64>>,
	pub poe_mode: Option<Vec<String>>,
	pub poe_mode__n: Option<Vec<String>>,
	pub poe_type: Option<Vec<String>>,
	pub poe_type__n: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	pub rf_channel: Option<Vec<String>>,
	pub rf_channel__n: Option<Vec<String>>,
	pub rf_channel_frequency: Option<Vec<f64>>,
	pub rf_channel_frequency__empty: Option<bool>,
	pub rf_channel_frequency__gt: Option<Vec<f64>>,
	pub rf_channel_frequency__gte: Option<Vec<f64>>,
	pub rf_channel_frequency__lt: Option<Vec<f64>>,
	pub rf_channel_frequency__lte: Option<Vec<f64>>,
	pub rf_channel_frequency__n: Option<Vec<f64>>,
	pub rf_channel_width: Option<Vec<f64>>,
	pub rf_channel_width__empty: Option<bool>,
	pub rf_channel_width__gt: Option<Vec<f64>>,
	pub rf_channel_width__gte: Option<Vec<f64>>,
	pub rf_channel_width__lt: Option<Vec<f64>>,
	pub rf_channel_width__lte: Option<Vec<f64>>,
	pub rf_channel_width__n: Option<Vec<f64>>,
	pub rf_role: Option<Vec<String>>,
	pub rf_role__n: Option<Vec<String>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub speed: Option<Vec<i64>>,
	pub speed__empty: Option<Vec<i64>>,
	pub speed__gt: Option<Vec<i64>>,
	pub speed__gte: Option<Vec<i64>>,
	pub speed__lt: Option<Vec<i64>>,
	pub speed__lte: Option<Vec<i64>>,
	pub speed__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub tx_power: Option<Vec<i64>>,
	pub tx_power__empty: Option<bool>,
	pub tx_power__gt: Option<Vec<i64>>,
	pub tx_power__gte: Option<Vec<i64>>,
	pub tx_power__lt: Option<Vec<i64>>,
	pub tx_power__lte: Option<Vec<i64>>,
	pub tx_power__n: Option<Vec<i64>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Device Context
	pub vdc: Option<Vec<String>>,
	/// Virtual Device Context
	pub vdc__n: Option<Vec<String>>,
	/// Virtual Device Context
	pub vdc_id: Option<Vec<i64>>,
	/// Virtual Device Context
	pub vdc_id__n: Option<Vec<i64>>,
	/// Virtual Device Context (Identifier)
	pub vdc_identifier: Option<Vec<Option<u16>>>,
	/// Virtual Device Context (Identifier)
	pub vdc_identifier__n: Option<Vec<Option<u16>>>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,
	pub virtual_chassis_member: Option<Vec<String>>,
	pub virtual_chassis_member_id: Option<Vec<i64>>,
	/// Assigned VID
	pub vlan: Option<String>,
	/// Assigned VLAN
	pub vlan_id: Option<String>,
	/// VRF (RD)
	pub vrf: Option<Vec<Option<String>>>,
	/// VRF (RD)
	pub vrf__n: Option<Vec<Option<String>>>,
	/// VRF
	pub vrf_id: Option<Vec<i64>>,
	/// VRF
	pub vrf_id__n: Option<Vec<i64>>,
	pub wwn: Option<Vec<String>>,
	pub wwn__ic: Option<Vec<String>>,
	pub wwn__ie: Option<Vec<String>>,
	pub wwn__iew: Option<Vec<String>>,
	pub wwn__isw: Option<Vec<String>>,
	pub wwn__n: Option<Vec<String>>,
	pub wwn__nic: Option<Vec<String>>,
	pub wwn__nie: Option<Vec<String>>,
	pub wwn__niew: Option<Vec<String>>,
	pub wwn__nisw: Option<Vec<String>>,

}
#[derive(Debug)]
pub enum DcimInterfacesListResponse {
	Http200(PaginatedInterfaceList),
	Other(Response)
}
/// Get a list of interface objects.
pub fn dcim_interfaces_list(state: &ThanixClient, query: DcimInterfacesListQuery) -> Result<DcimInterfacesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/interfaces/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesListResponse::Http200(r#response.json::<PaginatedInterfaceList>()?)) },
		r#other_status => { Ok(DcimInterfacesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesBulkUpdateResponse {
	Http200(Vec<Interface>),
	Other(Response)
}
/// Put a list of interface objects.
pub fn dcim_interfaces_bulk_update(state: &ThanixClient, body: Vec<InterfaceRequest>) -> Result<DcimInterfacesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesBulkUpdateResponse::Http200(r#response.json::<Vec<Interface>>()?)) },
		r#other_status => { Ok(DcimInterfacesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesCreateResponse {
	Http201(Interface),
	Other(Response)
}
/// Post a list of interface objects.
pub fn dcim_interfaces_create(state: &ThanixClient, body: WritableInterfaceRequest) -> Result<DcimInterfacesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimInterfacesCreateResponse::Http201(r#response.json::<Interface>()?)) },
		r#other_status => { Ok(DcimInterfacesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of interface objects.
pub fn dcim_interfaces_bulk_destroy(state: &ThanixClient, body: Vec<InterfaceRequest>) -> Result<DcimInterfacesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInterfacesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesBulkPartialUpdateResponse {
	Http200(Vec<Interface>),
	Other(Response)
}
/// Patch a list of interface objects.
pub fn dcim_interfaces_bulk_partial_update(state: &ThanixClient, body: Vec<InterfaceRequest>) -> Result<DcimInterfacesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Interface>>()?)) },
		r#other_status => { Ok(DcimInterfacesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesRetrieveResponse {
	Http200(Interface),
	Other(Response)
}
/// Get a interface object.
pub fn dcim_interfaces_retrieve(state: &ThanixClient, id: i64) -> Result<DcimInterfacesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesRetrieveResponse::Http200(r#response.json::<Interface>()?)) },
		r#other_status => { Ok(DcimInterfacesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesUpdateResponse {
	Http200(Interface),
	Other(Response)
}
/// Put a interface object.
pub fn dcim_interfaces_update(state: &ThanixClient, body: WritableInterfaceRequest, id: i64) -> Result<DcimInterfacesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesUpdateResponse::Http200(r#response.json::<Interface>()?)) },
		r#other_status => { Ok(DcimInterfacesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a interface object.
pub fn dcim_interfaces_destroy(state: &ThanixClient, id: i64) -> Result<DcimInterfacesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInterfacesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesPartialUpdateResponse {
	Http200(Interface),
	Other(Response)
}
/// Patch a interface object.
pub fn dcim_interfaces_partial_update(state: &ThanixClient, body: PatchedWritableInterfaceRequest, id: i64) -> Result<DcimInterfacesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesPartialUpdateResponse::Http200(r#response.json::<Interface>()?)) },
		r#other_status => { Ok(DcimInterfacesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInterfacesTraceRetrieveResponse {
	Http200(Interface),
	Other(Response)
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).
pub fn dcim_interfaces_trace_retrieve(state: &ThanixClient, id: i64) -> Result<DcimInterfacesTraceRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/interfaces/{id}/trace/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInterfacesTraceRetrieveResponse::Http200(r#response.json::<Interface>()?)) },
		r#other_status => { Ok(DcimInterfacesTraceRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimInventoryItemRolesListQuery {
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimInventoryItemRolesListResponse {
	Http200(PaginatedInventoryItemRoleList),
	Other(Response)
}
/// Get a list of inventory item role objects.
pub fn dcim_inventory_item_roles_list(state: &ThanixClient, query: DcimInventoryItemRolesListQuery) -> Result<DcimInventoryItemRolesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/inventory-item-roles/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemRolesListResponse::Http200(r#response.json::<PaginatedInventoryItemRoleList>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesBulkUpdateResponse {
	Http200(Vec<InventoryItemRole>),
	Other(Response)
}
/// Put a list of inventory item role objects.
pub fn dcim_inventory_item_roles_bulk_update(state: &ThanixClient, body: Vec<InventoryItemRoleRequest>) -> Result<DcimInventoryItemRolesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/inventory-item-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemRolesBulkUpdateResponse::Http200(r#response.json::<Vec<InventoryItemRole>>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesCreateResponse {
	Http201(InventoryItemRole),
	Other(Response)
}
/// Post a list of inventory item role objects.
pub fn dcim_inventory_item_roles_create(state: &ThanixClient, body: InventoryItemRoleRequest) -> Result<DcimInventoryItemRolesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/inventory-item-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimInventoryItemRolesCreateResponse::Http201(r#response.json::<InventoryItemRole>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of inventory item role objects.
pub fn dcim_inventory_item_roles_bulk_destroy(state: &ThanixClient, body: Vec<InventoryItemRoleRequest>) -> Result<DcimInventoryItemRolesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/inventory-item-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInventoryItemRolesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesBulkPartialUpdateResponse {
	Http200(Vec<InventoryItemRole>),
	Other(Response)
}
/// Patch a list of inventory item role objects.
pub fn dcim_inventory_item_roles_bulk_partial_update(state: &ThanixClient, body: Vec<InventoryItemRoleRequest>) -> Result<DcimInventoryItemRolesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/inventory-item-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemRolesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<InventoryItemRole>>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesRetrieveResponse {
	Http200(InventoryItemRole),
	Other(Response)
}
/// Get a inventory item role object.
pub fn dcim_inventory_item_roles_retrieve(state: &ThanixClient, id: i64) -> Result<DcimInventoryItemRolesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/inventory-item-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemRolesRetrieveResponse::Http200(r#response.json::<InventoryItemRole>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesUpdateResponse {
	Http200(InventoryItemRole),
	Other(Response)
}
/// Put a inventory item role object.
pub fn dcim_inventory_item_roles_update(state: &ThanixClient, body: InventoryItemRoleRequest, id: i64) -> Result<DcimInventoryItemRolesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/inventory-item-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemRolesUpdateResponse::Http200(r#response.json::<InventoryItemRole>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a inventory item role object.
pub fn dcim_inventory_item_roles_destroy(state: &ThanixClient, id: i64) -> Result<DcimInventoryItemRolesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/inventory-item-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInventoryItemRolesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemRolesPartialUpdateResponse {
	Http200(InventoryItemRole),
	Other(Response)
}
/// Patch a inventory item role object.
pub fn dcim_inventory_item_roles_partial_update(state: &ThanixClient, body: PatchedInventoryItemRoleRequest, id: i64) -> Result<DcimInventoryItemRolesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/inventory-item-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemRolesPartialUpdateResponse::Http200(r#response.json::<InventoryItemRole>()?)) },
		r#other_status => { Ok(DcimInventoryItemRolesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimInventoryItemTemplatesListQuery {
	pub component_id: Option<Vec<i64>>,
	pub component_id__empty: Option<Vec<i64>>,
	pub component_id__gt: Option<Vec<i64>>,
	pub component_id__gte: Option<Vec<i64>>,
	pub component_id__lt: Option<Vec<i64>>,
	pub component_id__lte: Option<Vec<i64>>,
	pub component_id__n: Option<Vec<i64>>,
	pub component_type: Option<String>,
	pub component_type__n: Option<String>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<Option<i64>>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<Option<i64>>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent inventory item (ID)
	pub parent_id: Option<Vec<Option<i64>>>,
	/// Parent inventory item (ID)
	pub parent_id__n: Option<Vec<Option<i64>>>,
	pub part_id: Option<Vec<String>>,
	pub part_id__empty: Option<bool>,
	pub part_id__ic: Option<Vec<String>>,
	pub part_id__ie: Option<Vec<String>>,
	pub part_id__iew: Option<Vec<String>>,
	pub part_id__isw: Option<Vec<String>>,
	pub part_id__n: Option<Vec<String>>,
	pub part_id__nic: Option<Vec<String>>,
	pub part_id__nie: Option<Vec<String>>,
	pub part_id__niew: Option<Vec<String>>,
	pub part_id__nisw: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesListResponse {
	Http200(PaginatedInventoryItemTemplateList),
	Other(Response)
}
/// Get a list of inventory item template objects.
pub fn dcim_inventory_item_templates_list(state: &ThanixClient, query: DcimInventoryItemTemplatesListQuery) -> Result<DcimInventoryItemTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/inventory-item-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemTemplatesListResponse::Http200(r#response.json::<PaginatedInventoryItemTemplateList>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesBulkUpdateResponse {
	Http200(Vec<InventoryItemTemplate>),
	Other(Response)
}
/// Put a list of inventory item template objects.
pub fn dcim_inventory_item_templates_bulk_update(state: &ThanixClient, body: Vec<InventoryItemTemplateRequest>) -> Result<DcimInventoryItemTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/inventory-item-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<InventoryItemTemplate>>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesCreateResponse {
	Http201(InventoryItemTemplate),
	Other(Response)
}
/// Post a list of inventory item template objects.
pub fn dcim_inventory_item_templates_create(state: &ThanixClient, body: WritableInventoryItemTemplateRequest) -> Result<DcimInventoryItemTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/inventory-item-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimInventoryItemTemplatesCreateResponse::Http201(r#response.json::<InventoryItemTemplate>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of inventory item template objects.
pub fn dcim_inventory_item_templates_bulk_destroy(state: &ThanixClient, body: Vec<InventoryItemTemplateRequest>) -> Result<DcimInventoryItemTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/inventory-item-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInventoryItemTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesBulkPartialUpdateResponse {
	Http200(Vec<InventoryItemTemplate>),
	Other(Response)
}
/// Patch a list of inventory item template objects.
pub fn dcim_inventory_item_templates_bulk_partial_update(state: &ThanixClient, body: Vec<InventoryItemTemplateRequest>) -> Result<DcimInventoryItemTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/inventory-item-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<InventoryItemTemplate>>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesRetrieveResponse {
	Http200(InventoryItemTemplate),
	Other(Response)
}
/// Get a inventory item template object.
pub fn dcim_inventory_item_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimInventoryItemTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/inventory-item-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemTemplatesRetrieveResponse::Http200(r#response.json::<InventoryItemTemplate>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesUpdateResponse {
	Http200(InventoryItemTemplate),
	Other(Response)
}
/// Put a inventory item template object.
pub fn dcim_inventory_item_templates_update(state: &ThanixClient, body: WritableInventoryItemTemplateRequest, id: i64) -> Result<DcimInventoryItemTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/inventory-item-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemTemplatesUpdateResponse::Http200(r#response.json::<InventoryItemTemplate>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a inventory item template object.
pub fn dcim_inventory_item_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimInventoryItemTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/inventory-item-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInventoryItemTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemTemplatesPartialUpdateResponse {
	Http200(InventoryItemTemplate),
	Other(Response)
}
/// Patch a inventory item template object.
pub fn dcim_inventory_item_templates_partial_update(state: &ThanixClient, body: PatchedWritableInventoryItemTemplateRequest, id: i64) -> Result<DcimInventoryItemTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/inventory-item-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemTemplatesPartialUpdateResponse::Http200(r#response.json::<InventoryItemTemplate>()?)) },
		r#other_status => { Ok(DcimInventoryItemTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimInventoryItemsListQuery {
	pub asset_tag: Option<Vec<String>>,
	pub asset_tag__empty: Option<bool>,
	pub asset_tag__ic: Option<Vec<String>>,
	pub asset_tag__ie: Option<Vec<String>>,
	pub asset_tag__iew: Option<Vec<String>>,
	pub asset_tag__isw: Option<Vec<String>>,
	pub asset_tag__n: Option<Vec<String>>,
	pub asset_tag__nic: Option<Vec<String>>,
	pub asset_tag__nie: Option<Vec<String>>,
	pub asset_tag__niew: Option<Vec<String>>,
	pub asset_tag__nisw: Option<Vec<String>>,
	pub component_id: Option<Vec<i64>>,
	pub component_id__empty: Option<Vec<i64>>,
	pub component_id__gt: Option<Vec<i64>>,
	pub component_id__gte: Option<Vec<i64>>,
	pub component_id__lt: Option<Vec<i64>>,
	pub component_id__lte: Option<Vec<i64>>,
	pub component_id__n: Option<Vec<i64>>,
	pub component_type: Option<String>,
	pub component_type__n: Option<String>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub discovered: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<Option<i64>>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<Option<i64>>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent inventory item (ID)
	pub parent_id: Option<Vec<Option<i64>>>,
	/// Parent inventory item (ID)
	pub parent_id__n: Option<Vec<Option<i64>>>,
	pub part_id: Option<Vec<String>>,
	pub part_id__empty: Option<bool>,
	pub part_id__ic: Option<Vec<String>>,
	pub part_id__ie: Option<Vec<String>>,
	pub part_id__iew: Option<Vec<String>>,
	pub part_id__isw: Option<Vec<String>>,
	pub part_id__n: Option<Vec<String>>,
	pub part_id__nic: Option<Vec<String>>,
	pub part_id__nie: Option<Vec<String>>,
	pub part_id__niew: Option<Vec<String>>,
	pub part_id__nisw: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	pub serial: Option<Vec<String>>,
	pub serial__empty: Option<bool>,
	pub serial__ic: Option<Vec<String>>,
	pub serial__ie: Option<Vec<String>>,
	pub serial__iew: Option<Vec<String>>,
	pub serial__isw: Option<Vec<String>>,
	pub serial__n: Option<Vec<String>>,
	pub serial__nic: Option<Vec<String>>,
	pub serial__nie: Option<Vec<String>>,
	pub serial__niew: Option<Vec<String>>,
	pub serial__nisw: Option<Vec<String>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimInventoryItemsListResponse {
	Http200(PaginatedInventoryItemList),
	Other(Response)
}
/// Get a list of inventory item objects.
pub fn dcim_inventory_items_list(state: &ThanixClient, query: DcimInventoryItemsListQuery) -> Result<DcimInventoryItemsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/inventory-items/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemsListResponse::Http200(r#response.json::<PaginatedInventoryItemList>()?)) },
		r#other_status => { Ok(DcimInventoryItemsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsBulkUpdateResponse {
	Http200(Vec<InventoryItem>),
	Other(Response)
}
/// Put a list of inventory item objects.
pub fn dcim_inventory_items_bulk_update(state: &ThanixClient, body: Vec<InventoryItemRequest>) -> Result<DcimInventoryItemsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/inventory-items/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemsBulkUpdateResponse::Http200(r#response.json::<Vec<InventoryItem>>()?)) },
		r#other_status => { Ok(DcimInventoryItemsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsCreateResponse {
	Http201(InventoryItem),
	Other(Response)
}
/// Post a list of inventory item objects.
pub fn dcim_inventory_items_create(state: &ThanixClient, body: WritableInventoryItemRequest) -> Result<DcimInventoryItemsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/inventory-items/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimInventoryItemsCreateResponse::Http201(r#response.json::<InventoryItem>()?)) },
		r#other_status => { Ok(DcimInventoryItemsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of inventory item objects.
pub fn dcim_inventory_items_bulk_destroy(state: &ThanixClient, body: Vec<InventoryItemRequest>) -> Result<DcimInventoryItemsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/inventory-items/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInventoryItemsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsBulkPartialUpdateResponse {
	Http200(Vec<InventoryItem>),
	Other(Response)
}
/// Patch a list of inventory item objects.
pub fn dcim_inventory_items_bulk_partial_update(state: &ThanixClient, body: Vec<InventoryItemRequest>) -> Result<DcimInventoryItemsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/inventory-items/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<InventoryItem>>()?)) },
		r#other_status => { Ok(DcimInventoryItemsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsRetrieveResponse {
	Http200(InventoryItem),
	Other(Response)
}
/// Get a inventory item object.
pub fn dcim_inventory_items_retrieve(state: &ThanixClient, id: i64) -> Result<DcimInventoryItemsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/inventory-items/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemsRetrieveResponse::Http200(r#response.json::<InventoryItem>()?)) },
		r#other_status => { Ok(DcimInventoryItemsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsUpdateResponse {
	Http200(InventoryItem),
	Other(Response)
}
/// Put a inventory item object.
pub fn dcim_inventory_items_update(state: &ThanixClient, body: WritableInventoryItemRequest, id: i64) -> Result<DcimInventoryItemsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/inventory-items/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemsUpdateResponse::Http200(r#response.json::<InventoryItem>()?)) },
		r#other_status => { Ok(DcimInventoryItemsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a inventory item object.
pub fn dcim_inventory_items_destroy(state: &ThanixClient, id: i64) -> Result<DcimInventoryItemsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/inventory-items/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimInventoryItemsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimInventoryItemsPartialUpdateResponse {
	Http200(InventoryItem),
	Other(Response)
}
/// Patch a inventory item object.
pub fn dcim_inventory_items_partial_update(state: &ThanixClient, body: PatchedWritableInventoryItemRequest, id: i64) -> Result<DcimInventoryItemsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/inventory-items/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimInventoryItemsPartialUpdateResponse::Http200(r#response.json::<InventoryItem>()?)) },
		r#other_status => { Ok(DcimInventoryItemsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimLocationsListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Location (slug)
	pub parent: Option<Vec<i64>>,
	/// Location (slug)
	pub parent__n: Option<Vec<i64>>,
	/// Location (ID)
	pub parent_id: Option<Vec<i64>>,
	/// Location (ID)
	pub parent_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimLocationsListResponse {
	Http200(PaginatedLocationList),
	Other(Response)
}
/// Get a list of location objects.
pub fn dcim_locations_list(state: &ThanixClient, query: DcimLocationsListQuery) -> Result<DcimLocationsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/locations/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimLocationsListResponse::Http200(r#response.json::<PaginatedLocationList>()?)) },
		r#other_status => { Ok(DcimLocationsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsBulkUpdateResponse {
	Http200(Vec<Location>),
	Other(Response)
}
/// Put a list of location objects.
pub fn dcim_locations_bulk_update(state: &ThanixClient, body: Vec<LocationRequest>) -> Result<DcimLocationsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/locations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimLocationsBulkUpdateResponse::Http200(r#response.json::<Vec<Location>>()?)) },
		r#other_status => { Ok(DcimLocationsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsCreateResponse {
	Http201(Location),
	Other(Response)
}
/// Post a list of location objects.
pub fn dcim_locations_create(state: &ThanixClient, body: WritableLocationRequest) -> Result<DcimLocationsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/locations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimLocationsCreateResponse::Http201(r#response.json::<Location>()?)) },
		r#other_status => { Ok(DcimLocationsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of location objects.
pub fn dcim_locations_bulk_destroy(state: &ThanixClient, body: Vec<LocationRequest>) -> Result<DcimLocationsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/locations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimLocationsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsBulkPartialUpdateResponse {
	Http200(Vec<Location>),
	Other(Response)
}
/// Patch a list of location objects.
pub fn dcim_locations_bulk_partial_update(state: &ThanixClient, body: Vec<LocationRequest>) -> Result<DcimLocationsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/locations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimLocationsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Location>>()?)) },
		r#other_status => { Ok(DcimLocationsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsRetrieveResponse {
	Http200(Location),
	Other(Response)
}
/// Get a location object.
pub fn dcim_locations_retrieve(state: &ThanixClient, id: i64) -> Result<DcimLocationsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/locations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimLocationsRetrieveResponse::Http200(r#response.json::<Location>()?)) },
		r#other_status => { Ok(DcimLocationsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsUpdateResponse {
	Http200(Location),
	Other(Response)
}
/// Put a location object.
pub fn dcim_locations_update(state: &ThanixClient, body: WritableLocationRequest, id: i64) -> Result<DcimLocationsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/locations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimLocationsUpdateResponse::Http200(r#response.json::<Location>()?)) },
		r#other_status => { Ok(DcimLocationsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a location object.
pub fn dcim_locations_destroy(state: &ThanixClient, id: i64) -> Result<DcimLocationsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/locations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimLocationsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimLocationsPartialUpdateResponse {
	Http200(Location),
	Other(Response)
}
/// Patch a location object.
pub fn dcim_locations_partial_update(state: &ThanixClient, body: PatchedWritableLocationRequest, id: i64) -> Result<DcimLocationsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/locations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimLocationsPartialUpdateResponse::Http200(r#response.json::<Location>()?)) },
		r#other_status => { Ok(DcimLocationsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimManufacturersListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimManufacturersListResponse {
	Http200(PaginatedManufacturerList),
	Other(Response)
}
/// Get a list of manufacturer objects.
pub fn dcim_manufacturers_list(state: &ThanixClient, query: DcimManufacturersListQuery) -> Result<DcimManufacturersListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/manufacturers/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimManufacturersListResponse::Http200(r#response.json::<PaginatedManufacturerList>()?)) },
		r#other_status => { Ok(DcimManufacturersListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersBulkUpdateResponse {
	Http200(Vec<Manufacturer>),
	Other(Response)
}
/// Put a list of manufacturer objects.
pub fn dcim_manufacturers_bulk_update(state: &ThanixClient, body: Vec<ManufacturerRequest>) -> Result<DcimManufacturersBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/manufacturers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimManufacturersBulkUpdateResponse::Http200(r#response.json::<Vec<Manufacturer>>()?)) },
		r#other_status => { Ok(DcimManufacturersBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersCreateResponse {
	Http201(Manufacturer),
	Other(Response)
}
/// Post a list of manufacturer objects.
pub fn dcim_manufacturers_create(state: &ThanixClient, body: ManufacturerRequest) -> Result<DcimManufacturersCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/manufacturers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimManufacturersCreateResponse::Http201(r#response.json::<Manufacturer>()?)) },
		r#other_status => { Ok(DcimManufacturersCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of manufacturer objects.
pub fn dcim_manufacturers_bulk_destroy(state: &ThanixClient, body: Vec<ManufacturerRequest>) -> Result<DcimManufacturersBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/manufacturers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimManufacturersBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersBulkPartialUpdateResponse {
	Http200(Vec<Manufacturer>),
	Other(Response)
}
/// Patch a list of manufacturer objects.
pub fn dcim_manufacturers_bulk_partial_update(state: &ThanixClient, body: Vec<ManufacturerRequest>) -> Result<DcimManufacturersBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/manufacturers/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimManufacturersBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Manufacturer>>()?)) },
		r#other_status => { Ok(DcimManufacturersBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersRetrieveResponse {
	Http200(Manufacturer),
	Other(Response)
}
/// Get a manufacturer object.
pub fn dcim_manufacturers_retrieve(state: &ThanixClient, id: i64) -> Result<DcimManufacturersRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/manufacturers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimManufacturersRetrieveResponse::Http200(r#response.json::<Manufacturer>()?)) },
		r#other_status => { Ok(DcimManufacturersRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersUpdateResponse {
	Http200(Manufacturer),
	Other(Response)
}
/// Put a manufacturer object.
pub fn dcim_manufacturers_update(state: &ThanixClient, body: ManufacturerRequest, id: i64) -> Result<DcimManufacturersUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/manufacturers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimManufacturersUpdateResponse::Http200(r#response.json::<Manufacturer>()?)) },
		r#other_status => { Ok(DcimManufacturersUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a manufacturer object.
pub fn dcim_manufacturers_destroy(state: &ThanixClient, id: i64) -> Result<DcimManufacturersDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/manufacturers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimManufacturersDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimManufacturersPartialUpdateResponse {
	Http200(Manufacturer),
	Other(Response)
}
/// Patch a manufacturer object.
pub fn dcim_manufacturers_partial_update(state: &ThanixClient, body: PatchedManufacturerRequest, id: i64) -> Result<DcimManufacturersPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/manufacturers/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimManufacturersPartialUpdateResponse::Http200(r#response.json::<Manufacturer>()?)) },
		r#other_status => { Ok(DcimManufacturersPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimModuleBayTemplatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesListResponse {
	Http200(PaginatedModuleBayTemplateList),
	Other(Response)
}
/// Get a list of module bay template objects.
pub fn dcim_module_bay_templates_list(state: &ThanixClient, query: DcimModuleBayTemplatesListQuery) -> Result<DcimModuleBayTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/module-bay-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBayTemplatesListResponse::Http200(r#response.json::<PaginatedModuleBayTemplateList>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesBulkUpdateResponse {
	Http200(Vec<ModuleBayTemplate>),
	Other(Response)
}
/// Put a list of module bay template objects.
pub fn dcim_module_bay_templates_bulk_update(state: &ThanixClient, body: Vec<ModuleBayTemplateRequest>) -> Result<DcimModuleBayTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/module-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBayTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<ModuleBayTemplate>>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesCreateResponse {
	Http201(ModuleBayTemplate),
	Other(Response)
}
/// Post a list of module bay template objects.
pub fn dcim_module_bay_templates_create(state: &ThanixClient, body: WritableModuleBayTemplateRequest) -> Result<DcimModuleBayTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/module-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimModuleBayTemplatesCreateResponse::Http201(r#response.json::<ModuleBayTemplate>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of module bay template objects.
pub fn dcim_module_bay_templates_bulk_destroy(state: &ThanixClient, body: Vec<ModuleBayTemplateRequest>) -> Result<DcimModuleBayTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/module-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModuleBayTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesBulkPartialUpdateResponse {
	Http200(Vec<ModuleBayTemplate>),
	Other(Response)
}
/// Patch a list of module bay template objects.
pub fn dcim_module_bay_templates_bulk_partial_update(state: &ThanixClient, body: Vec<ModuleBayTemplateRequest>) -> Result<DcimModuleBayTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/module-bay-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBayTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ModuleBayTemplate>>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesRetrieveResponse {
	Http200(ModuleBayTemplate),
	Other(Response)
}
/// Get a module bay template object.
pub fn dcim_module_bay_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimModuleBayTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/module-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBayTemplatesRetrieveResponse::Http200(r#response.json::<ModuleBayTemplate>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesUpdateResponse {
	Http200(ModuleBayTemplate),
	Other(Response)
}
/// Put a module bay template object.
pub fn dcim_module_bay_templates_update(state: &ThanixClient, body: WritableModuleBayTemplateRequest, id: i64) -> Result<DcimModuleBayTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/module-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBayTemplatesUpdateResponse::Http200(r#response.json::<ModuleBayTemplate>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a module bay template object.
pub fn dcim_module_bay_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimModuleBayTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/module-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModuleBayTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBayTemplatesPartialUpdateResponse {
	Http200(ModuleBayTemplate),
	Other(Response)
}
/// Patch a module bay template object.
pub fn dcim_module_bay_templates_partial_update(state: &ThanixClient, body: PatchedWritableModuleBayTemplateRequest, id: i64) -> Result<DcimModuleBayTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/module-bay-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBayTemplatesPartialUpdateResponse::Http200(r#response.json::<ModuleBayTemplate>()?)) },
		r#other_status => { Ok(DcimModuleBayTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimModuleBaysListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimModuleBaysListResponse {
	Http200(PaginatedModuleBayList),
	Other(Response)
}
/// Get a list of module bay objects.
pub fn dcim_module_bays_list(state: &ThanixClient, query: DcimModuleBaysListQuery) -> Result<DcimModuleBaysListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/module-bays/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBaysListResponse::Http200(r#response.json::<PaginatedModuleBayList>()?)) },
		r#other_status => { Ok(DcimModuleBaysListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysBulkUpdateResponse {
	Http200(Vec<ModuleBay>),
	Other(Response)
}
/// Put a list of module bay objects.
pub fn dcim_module_bays_bulk_update(state: &ThanixClient, body: Vec<ModuleBayRequest>) -> Result<DcimModuleBaysBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/module-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBaysBulkUpdateResponse::Http200(r#response.json::<Vec<ModuleBay>>()?)) },
		r#other_status => { Ok(DcimModuleBaysBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysCreateResponse {
	Http201(ModuleBay),
	Other(Response)
}
/// Post a list of module bay objects.
pub fn dcim_module_bays_create(state: &ThanixClient, body: WritableModuleBayRequest) -> Result<DcimModuleBaysCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/module-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimModuleBaysCreateResponse::Http201(r#response.json::<ModuleBay>()?)) },
		r#other_status => { Ok(DcimModuleBaysCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of module bay objects.
pub fn dcim_module_bays_bulk_destroy(state: &ThanixClient, body: Vec<ModuleBayRequest>) -> Result<DcimModuleBaysBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/module-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModuleBaysBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysBulkPartialUpdateResponse {
	Http200(Vec<ModuleBay>),
	Other(Response)
}
/// Patch a list of module bay objects.
pub fn dcim_module_bays_bulk_partial_update(state: &ThanixClient, body: Vec<ModuleBayRequest>) -> Result<DcimModuleBaysBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/module-bays/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBaysBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ModuleBay>>()?)) },
		r#other_status => { Ok(DcimModuleBaysBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysRetrieveResponse {
	Http200(ModuleBay),
	Other(Response)
}
/// Get a module bay object.
pub fn dcim_module_bays_retrieve(state: &ThanixClient, id: i64) -> Result<DcimModuleBaysRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/module-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBaysRetrieveResponse::Http200(r#response.json::<ModuleBay>()?)) },
		r#other_status => { Ok(DcimModuleBaysRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysUpdateResponse {
	Http200(ModuleBay),
	Other(Response)
}
/// Put a module bay object.
pub fn dcim_module_bays_update(state: &ThanixClient, body: WritableModuleBayRequest, id: i64) -> Result<DcimModuleBaysUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/module-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBaysUpdateResponse::Http200(r#response.json::<ModuleBay>()?)) },
		r#other_status => { Ok(DcimModuleBaysUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a module bay object.
pub fn dcim_module_bays_destroy(state: &ThanixClient, id: i64) -> Result<DcimModuleBaysDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/module-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModuleBaysDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleBaysPartialUpdateResponse {
	Http200(ModuleBay),
	Other(Response)
}
/// Patch a module bay object.
pub fn dcim_module_bays_partial_update(state: &ThanixClient, body: PatchedWritableModuleBayRequest, id: i64) -> Result<DcimModuleBaysPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/module-bays/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleBaysPartialUpdateResponse::Http200(r#response.json::<ModuleBay>()?)) },
		r#other_status => { Ok(DcimModuleBaysPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimModuleTypesListQuery {
	/// Has console ports
	pub console_ports: Option<bool>,
	/// Has console server ports
	pub console_server_ports: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Has interfaces
	pub interfaces: Option<bool>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<i64>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<i64>>,
	pub model: Option<Vec<String>>,
	pub model__empty: Option<bool>,
	pub model__ic: Option<Vec<String>>,
	pub model__ie: Option<Vec<String>>,
	pub model__iew: Option<Vec<String>>,
	pub model__isw: Option<Vec<String>>,
	pub model__n: Option<Vec<String>>,
	pub model__nic: Option<Vec<String>>,
	pub model__nie: Option<Vec<String>>,
	pub model__niew: Option<Vec<String>>,
	pub model__nisw: Option<Vec<String>>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub part_number: Option<Vec<String>>,
	pub part_number__empty: Option<bool>,
	pub part_number__ic: Option<Vec<String>>,
	pub part_number__ie: Option<Vec<String>>,
	pub part_number__iew: Option<Vec<String>>,
	pub part_number__isw: Option<Vec<String>>,
	pub part_number__n: Option<Vec<String>>,
	pub part_number__nic: Option<Vec<String>>,
	pub part_number__nie: Option<Vec<String>>,
	pub part_number__niew: Option<Vec<String>>,
	pub part_number__nisw: Option<Vec<String>>,
	/// Has pass-through ports
	pub pass_through_ports: Option<bool>,
	/// Has power outlets
	pub power_outlets: Option<bool>,
	/// Has power ports
	pub power_ports: Option<bool>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	pub weight: Option<Vec<f64>>,
	pub weight__empty: Option<bool>,
	pub weight__gt: Option<Vec<f64>>,
	pub weight__gte: Option<Vec<f64>>,
	pub weight__lt: Option<Vec<f64>>,
	pub weight__lte: Option<Vec<f64>>,
	pub weight__n: Option<Vec<f64>>,
	pub weight_unit: Option<String>,
	pub weight_unit__n: Option<String>,

}
#[derive(Debug)]
pub enum DcimModuleTypesListResponse {
	Http200(PaginatedModuleTypeList),
	Other(Response)
}
/// Get a list of module type objects.
pub fn dcim_module_types_list(state: &ThanixClient, query: DcimModuleTypesListQuery) -> Result<DcimModuleTypesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/module-types/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleTypesListResponse::Http200(r#response.json::<PaginatedModuleTypeList>()?)) },
		r#other_status => { Ok(DcimModuleTypesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesBulkUpdateResponse {
	Http200(Vec<ModuleType>),
	Other(Response)
}
/// Put a list of module type objects.
pub fn dcim_module_types_bulk_update(state: &ThanixClient, body: Vec<ModuleTypeRequest>) -> Result<DcimModuleTypesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/module-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleTypesBulkUpdateResponse::Http200(r#response.json::<Vec<ModuleType>>()?)) },
		r#other_status => { Ok(DcimModuleTypesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesCreateResponse {
	Http201(ModuleType),
	Other(Response)
}
/// Post a list of module type objects.
pub fn dcim_module_types_create(state: &ThanixClient, body: WritableModuleTypeRequest) -> Result<DcimModuleTypesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/module-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimModuleTypesCreateResponse::Http201(r#response.json::<ModuleType>()?)) },
		r#other_status => { Ok(DcimModuleTypesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of module type objects.
pub fn dcim_module_types_bulk_destroy(state: &ThanixClient, body: Vec<ModuleTypeRequest>) -> Result<DcimModuleTypesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/module-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModuleTypesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesBulkPartialUpdateResponse {
	Http200(Vec<ModuleType>),
	Other(Response)
}
/// Patch a list of module type objects.
pub fn dcim_module_types_bulk_partial_update(state: &ThanixClient, body: Vec<ModuleTypeRequest>) -> Result<DcimModuleTypesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/module-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleTypesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ModuleType>>()?)) },
		r#other_status => { Ok(DcimModuleTypesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesRetrieveResponse {
	Http200(ModuleType),
	Other(Response)
}
/// Get a module type object.
pub fn dcim_module_types_retrieve(state: &ThanixClient, id: i64) -> Result<DcimModuleTypesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/module-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleTypesRetrieveResponse::Http200(r#response.json::<ModuleType>()?)) },
		r#other_status => { Ok(DcimModuleTypesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesUpdateResponse {
	Http200(ModuleType),
	Other(Response)
}
/// Put a module type object.
pub fn dcim_module_types_update(state: &ThanixClient, body: WritableModuleTypeRequest, id: i64) -> Result<DcimModuleTypesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/module-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleTypesUpdateResponse::Http200(r#response.json::<ModuleType>()?)) },
		r#other_status => { Ok(DcimModuleTypesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a module type object.
pub fn dcim_module_types_destroy(state: &ThanixClient, id: i64) -> Result<DcimModuleTypesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/module-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModuleTypesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModuleTypesPartialUpdateResponse {
	Http200(ModuleType),
	Other(Response)
}
/// Patch a module type object.
pub fn dcim_module_types_partial_update(state: &ThanixClient, body: PatchedWritableModuleTypeRequest, id: i64) -> Result<DcimModuleTypesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/module-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModuleTypesPartialUpdateResponse::Http200(r#response.json::<ModuleType>()?)) },
		r#other_status => { Ok(DcimModuleTypesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimModulesListQuery {
	pub asset_tag: Option<Vec<String>>,
	pub asset_tag__empty: Option<bool>,
	pub asset_tag__ic: Option<Vec<String>>,
	pub asset_tag__ie: Option<Vec<String>>,
	pub asset_tag__iew: Option<Vec<String>>,
	pub asset_tag__isw: Option<Vec<String>>,
	pub asset_tag__n: Option<Vec<String>>,
	pub asset_tag__nic: Option<Vec<String>>,
	pub asset_tag__nie: Option<Vec<String>>,
	pub asset_tag__niew: Option<Vec<String>>,
	pub asset_tag__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<i64>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module Bay (ID)
	pub module_bay_id: Option<Vec<i64>>,
	/// Module Bay (ID)
	pub module_bay_id__n: Option<Vec<i64>>,
	/// Module type (model)
	pub module_type: Option<Vec<String>>,
	/// Module type (model)
	pub module_type__n: Option<Vec<String>>,
	/// Module type (ID)
	pub module_type_id: Option<Vec<i64>>,
	/// Module type (ID)
	pub module_type_id__n: Option<Vec<i64>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub serial: Option<Vec<String>>,
	pub serial__empty: Option<bool>,
	pub serial__ic: Option<Vec<String>>,
	pub serial__ie: Option<Vec<String>>,
	pub serial__iew: Option<Vec<String>>,
	pub serial__isw: Option<Vec<String>>,
	pub serial__n: Option<Vec<String>>,
	pub serial__nic: Option<Vec<String>>,
	pub serial__nie: Option<Vec<String>>,
	pub serial__niew: Option<Vec<String>>,
	pub serial__nisw: Option<Vec<String>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimModulesListResponse {
	Http200(PaginatedModuleList),
	Other(Response)
}
/// Get a list of module objects.
pub fn dcim_modules_list(state: &ThanixClient, query: DcimModulesListQuery) -> Result<DcimModulesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/modules/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModulesListResponse::Http200(r#response.json::<PaginatedModuleList>()?)) },
		r#other_status => { Ok(DcimModulesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesBulkUpdateResponse {
	Http200(Vec<Module>),
	Other(Response)
}
/// Put a list of module objects.
pub fn dcim_modules_bulk_update(state: &ThanixClient, body: Vec<ModuleRequest>) -> Result<DcimModulesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/modules/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModulesBulkUpdateResponse::Http200(r#response.json::<Vec<Module>>()?)) },
		r#other_status => { Ok(DcimModulesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesCreateResponse {
	Http201(Module),
	Other(Response)
}
/// Post a list of module objects.
pub fn dcim_modules_create(state: &ThanixClient, body: WritableModuleRequest) -> Result<DcimModulesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/modules/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimModulesCreateResponse::Http201(r#response.json::<Module>()?)) },
		r#other_status => { Ok(DcimModulesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of module objects.
pub fn dcim_modules_bulk_destroy(state: &ThanixClient, body: Vec<ModuleRequest>) -> Result<DcimModulesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/modules/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModulesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesBulkPartialUpdateResponse {
	Http200(Vec<Module>),
	Other(Response)
}
/// Patch a list of module objects.
pub fn dcim_modules_bulk_partial_update(state: &ThanixClient, body: Vec<ModuleRequest>) -> Result<DcimModulesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/modules/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModulesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Module>>()?)) },
		r#other_status => { Ok(DcimModulesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesRetrieveResponse {
	Http200(Module),
	Other(Response)
}
/// Get a module object.
pub fn dcim_modules_retrieve(state: &ThanixClient, id: i64) -> Result<DcimModulesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/modules/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModulesRetrieveResponse::Http200(r#response.json::<Module>()?)) },
		r#other_status => { Ok(DcimModulesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesUpdateResponse {
	Http200(Module),
	Other(Response)
}
/// Put a module object.
pub fn dcim_modules_update(state: &ThanixClient, body: WritableModuleRequest, id: i64) -> Result<DcimModulesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/modules/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModulesUpdateResponse::Http200(r#response.json::<Module>()?)) },
		r#other_status => { Ok(DcimModulesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a module object.
pub fn dcim_modules_destroy(state: &ThanixClient, id: i64) -> Result<DcimModulesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/modules/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimModulesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimModulesPartialUpdateResponse {
	Http200(Module),
	Other(Response)
}
/// Patch a module object.
pub fn dcim_modules_partial_update(state: &ThanixClient, body: PatchedWritableModuleRequest, id: i64) -> Result<DcimModulesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/modules/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimModulesPartialUpdateResponse::Http200(r#response.json::<Module>()?)) },
		r#other_status => { Ok(DcimModulesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPlatformsListQuery {
	/// Config template (ID)
	pub config_template_id: Option<Vec<Option<i64>>>,
	/// Config template (ID)
	pub config_template_id__n: Option<Vec<Option<i64>>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Manufacturer (slug)
	pub manufacturer: Option<Vec<String>>,
	/// Manufacturer (slug)
	pub manufacturer__n: Option<Vec<String>>,
	/// Manufacturer (ID)
	pub manufacturer_id: Option<Vec<i64>>,
	/// Manufacturer (ID)
	pub manufacturer_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimPlatformsListResponse {
	Http200(PaginatedPlatformList),
	Other(Response)
}
/// Get a list of platform objects.
pub fn dcim_platforms_list(state: &ThanixClient, query: DcimPlatformsListQuery) -> Result<DcimPlatformsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/platforms/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPlatformsListResponse::Http200(r#response.json::<PaginatedPlatformList>()?)) },
		r#other_status => { Ok(DcimPlatformsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsBulkUpdateResponse {
	Http200(Vec<Platform>),
	Other(Response)
}
/// Put a list of platform objects.
pub fn dcim_platforms_bulk_update(state: &ThanixClient, body: Vec<PlatformRequest>) -> Result<DcimPlatformsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/platforms/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPlatformsBulkUpdateResponse::Http200(r#response.json::<Vec<Platform>>()?)) },
		r#other_status => { Ok(DcimPlatformsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsCreateResponse {
	Http201(Platform),
	Other(Response)
}
/// Post a list of platform objects.
pub fn dcim_platforms_create(state: &ThanixClient, body: WritablePlatformRequest) -> Result<DcimPlatformsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/platforms/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPlatformsCreateResponse::Http201(r#response.json::<Platform>()?)) },
		r#other_status => { Ok(DcimPlatformsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of platform objects.
pub fn dcim_platforms_bulk_destroy(state: &ThanixClient, body: Vec<PlatformRequest>) -> Result<DcimPlatformsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/platforms/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPlatformsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsBulkPartialUpdateResponse {
	Http200(Vec<Platform>),
	Other(Response)
}
/// Patch a list of platform objects.
pub fn dcim_platforms_bulk_partial_update(state: &ThanixClient, body: Vec<PlatformRequest>) -> Result<DcimPlatformsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/platforms/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPlatformsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Platform>>()?)) },
		r#other_status => { Ok(DcimPlatformsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsRetrieveResponse {
	Http200(Platform),
	Other(Response)
}
/// Get a platform object.
pub fn dcim_platforms_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPlatformsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/platforms/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPlatformsRetrieveResponse::Http200(r#response.json::<Platform>()?)) },
		r#other_status => { Ok(DcimPlatformsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsUpdateResponse {
	Http200(Platform),
	Other(Response)
}
/// Put a platform object.
pub fn dcim_platforms_update(state: &ThanixClient, body: WritablePlatformRequest, id: i64) -> Result<DcimPlatformsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/platforms/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPlatformsUpdateResponse::Http200(r#response.json::<Platform>()?)) },
		r#other_status => { Ok(DcimPlatformsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a platform object.
pub fn dcim_platforms_destroy(state: &ThanixClient, id: i64) -> Result<DcimPlatformsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/platforms/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPlatformsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPlatformsPartialUpdateResponse {
	Http200(Platform),
	Other(Response)
}
/// Patch a platform object.
pub fn dcim_platforms_partial_update(state: &ThanixClient, body: PatchedWritablePlatformRequest, id: i64) -> Result<DcimPlatformsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/platforms/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPlatformsPartialUpdateResponse::Http200(r#response.json::<Platform>()?)) },
		r#other_status => { Ok(DcimPlatformsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPowerFeedsListQuery {
	pub amperage: Option<Vec<i64>>,
	pub amperage__empty: Option<bool>,
	pub amperage__gt: Option<Vec<i64>>,
	pub amperage__gte: Option<Vec<i64>>,
	pub amperage__lt: Option<Vec<i64>>,
	pub amperage__lte: Option<Vec<i64>>,
	pub amperage__n: Option<Vec<i64>>,
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub connected: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub max_utilization: Option<Vec<i64>>,
	pub max_utilization__empty: Option<bool>,
	pub max_utilization__gt: Option<Vec<i64>>,
	pub max_utilization__gte: Option<Vec<i64>>,
	pub max_utilization__lt: Option<Vec<i64>>,
	pub max_utilization__lte: Option<Vec<i64>>,
	pub max_utilization__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub phase: Option<String>,
	pub phase__n: Option<String>,
	/// Power panel (ID)
	pub power_panel_id: Option<Vec<i64>>,
	/// Power panel (ID)
	pub power_panel_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub supply: Option<String>,
	pub supply__n: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub r#type: Option<String>,
	pub type__n: Option<String>,
	pub updated_by_request: Option<String>,
	pub voltage: Option<Vec<i64>>,
	pub voltage__empty: Option<bool>,
	pub voltage__gt: Option<Vec<i64>>,
	pub voltage__gte: Option<Vec<i64>>,
	pub voltage__lt: Option<Vec<i64>>,
	pub voltage__lte: Option<Vec<i64>>,
	pub voltage__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimPowerFeedsListResponse {
	Http200(PaginatedPowerFeedList),
	Other(Response)
}
/// Get a list of power feed objects.
pub fn dcim_power_feeds_list(state: &ThanixClient, query: DcimPowerFeedsListQuery) -> Result<DcimPowerFeedsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/power-feeds/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsListResponse::Http200(r#response.json::<PaginatedPowerFeedList>()?)) },
		r#other_status => { Ok(DcimPowerFeedsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsBulkUpdateResponse {
	Http200(Vec<PowerFeed>),
	Other(Response)
}
/// Put a list of power feed objects.
pub fn dcim_power_feeds_bulk_update(state: &ThanixClient, body: Vec<PowerFeedRequest>) -> Result<DcimPowerFeedsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-feeds/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsBulkUpdateResponse::Http200(r#response.json::<Vec<PowerFeed>>()?)) },
		r#other_status => { Ok(DcimPowerFeedsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsCreateResponse {
	Http201(PowerFeed),
	Other(Response)
}
/// Post a list of power feed objects.
pub fn dcim_power_feeds_create(state: &ThanixClient, body: WritablePowerFeedRequest) -> Result<DcimPowerFeedsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/power-feeds/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPowerFeedsCreateResponse::Http201(r#response.json::<PowerFeed>()?)) },
		r#other_status => { Ok(DcimPowerFeedsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of power feed objects.
pub fn dcim_power_feeds_bulk_destroy(state: &ThanixClient, body: Vec<PowerFeedRequest>) -> Result<DcimPowerFeedsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-feeds/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerFeedsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsBulkPartialUpdateResponse {
	Http200(Vec<PowerFeed>),
	Other(Response)
}
/// Patch a list of power feed objects.
pub fn dcim_power_feeds_bulk_partial_update(state: &ThanixClient, body: Vec<PowerFeedRequest>) -> Result<DcimPowerFeedsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-feeds/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<PowerFeed>>()?)) },
		r#other_status => { Ok(DcimPowerFeedsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsRetrieveResponse {
	Http200(PowerFeed),
	Other(Response)
}
/// Get a power feed object.
pub fn dcim_power_feeds_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerFeedsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-feeds/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsRetrieveResponse::Http200(r#response.json::<PowerFeed>()?)) },
		r#other_status => { Ok(DcimPowerFeedsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsUpdateResponse {
	Http200(PowerFeed),
	Other(Response)
}
/// Put a power feed object.
pub fn dcim_power_feeds_update(state: &ThanixClient, body: WritablePowerFeedRequest, id: i64) -> Result<DcimPowerFeedsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-feeds/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsUpdateResponse::Http200(r#response.json::<PowerFeed>()?)) },
		r#other_status => { Ok(DcimPowerFeedsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a power feed object.
pub fn dcim_power_feeds_destroy(state: &ThanixClient, id: i64) -> Result<DcimPowerFeedsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-feeds/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerFeedsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsPartialUpdateResponse {
	Http200(PowerFeed),
	Other(Response)
}
/// Patch a power feed object.
pub fn dcim_power_feeds_partial_update(state: &ThanixClient, body: PatchedWritablePowerFeedRequest, id: i64) -> Result<DcimPowerFeedsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-feeds/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsPartialUpdateResponse::Http200(r#response.json::<PowerFeed>()?)) },
		r#other_status => { Ok(DcimPowerFeedsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerFeedsTraceRetrieveResponse {
	Http200(PowerFeed),
	Other(Response)
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).
pub fn dcim_power_feeds_trace_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerFeedsTraceRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-feeds/{id}/trace/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerFeedsTraceRetrieveResponse::Http200(r#response.json::<PowerFeed>()?)) },
		r#other_status => { Ok(DcimPowerFeedsTraceRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPowerOutletTemplatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	/// Phase (for three-phase feeds)
	pub feed_leg: Option<Vec<String>>,
	/// Phase (for three-phase feeds)
	pub feed_leg__n: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub r#type: Option<String>,
	pub type__n: Option<String>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesListResponse {
	Http200(PaginatedPowerOutletTemplateList),
	Other(Response)
}
/// Get a list of power outlet template objects.
pub fn dcim_power_outlet_templates_list(state: &ThanixClient, query: DcimPowerOutletTemplatesListQuery) -> Result<DcimPowerOutletTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/power-outlet-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletTemplatesListResponse::Http200(r#response.json::<PaginatedPowerOutletTemplateList>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesBulkUpdateResponse {
	Http200(Vec<PowerOutletTemplate>),
	Other(Response)
}
/// Put a list of power outlet template objects.
pub fn dcim_power_outlet_templates_bulk_update(state: &ThanixClient, body: Vec<PowerOutletTemplateRequest>) -> Result<DcimPowerOutletTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-outlet-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<PowerOutletTemplate>>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesCreateResponse {
	Http201(PowerOutletTemplate),
	Other(Response)
}
/// Post a list of power outlet template objects.
pub fn dcim_power_outlet_templates_create(state: &ThanixClient, body: WritablePowerOutletTemplateRequest) -> Result<DcimPowerOutletTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/power-outlet-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPowerOutletTemplatesCreateResponse::Http201(r#response.json::<PowerOutletTemplate>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of power outlet template objects.
pub fn dcim_power_outlet_templates_bulk_destroy(state: &ThanixClient, body: Vec<PowerOutletTemplateRequest>) -> Result<DcimPowerOutletTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-outlet-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerOutletTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesBulkPartialUpdateResponse {
	Http200(Vec<PowerOutletTemplate>),
	Other(Response)
}
/// Patch a list of power outlet template objects.
pub fn dcim_power_outlet_templates_bulk_partial_update(state: &ThanixClient, body: Vec<PowerOutletTemplateRequest>) -> Result<DcimPowerOutletTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-outlet-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<PowerOutletTemplate>>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesRetrieveResponse {
	Http200(PowerOutletTemplate),
	Other(Response)
}
/// Get a power outlet template object.
pub fn dcim_power_outlet_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerOutletTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-outlet-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletTemplatesRetrieveResponse::Http200(r#response.json::<PowerOutletTemplate>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesUpdateResponse {
	Http200(PowerOutletTemplate),
	Other(Response)
}
/// Put a power outlet template object.
pub fn dcim_power_outlet_templates_update(state: &ThanixClient, body: WritablePowerOutletTemplateRequest, id: i64) -> Result<DcimPowerOutletTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-outlet-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletTemplatesUpdateResponse::Http200(r#response.json::<PowerOutletTemplate>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a power outlet template object.
pub fn dcim_power_outlet_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimPowerOutletTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-outlet-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerOutletTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletTemplatesPartialUpdateResponse {
	Http200(PowerOutletTemplate),
	Other(Response)
}
/// Patch a power outlet template object.
pub fn dcim_power_outlet_templates_partial_update(state: &ThanixClient, body: PatchedWritablePowerOutletTemplateRequest, id: i64) -> Result<DcimPowerOutletTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-outlet-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletTemplatesPartialUpdateResponse::Http200(r#response.json::<PowerOutletTemplate>()?)) },
		r#other_status => { Ok(DcimPowerOutletTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPowerOutletsListQuery {
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub connected: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	/// Phase (for three-phase feeds)
	pub feed_leg: Option<Vec<String>>,
	/// Phase (for three-phase feeds)
	pub feed_leg__n: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Physical port type
	pub r#type: Option<Vec<String>>,
	/// Physical port type
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimPowerOutletsListResponse {
	Http200(PaginatedPowerOutletList),
	Other(Response)
}
/// Get a list of power outlet objects.
pub fn dcim_power_outlets_list(state: &ThanixClient, query: DcimPowerOutletsListQuery) -> Result<DcimPowerOutletsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/power-outlets/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsListResponse::Http200(r#response.json::<PaginatedPowerOutletList>()?)) },
		r#other_status => { Ok(DcimPowerOutletsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsBulkUpdateResponse {
	Http200(Vec<PowerOutlet>),
	Other(Response)
}
/// Put a list of power outlet objects.
pub fn dcim_power_outlets_bulk_update(state: &ThanixClient, body: Vec<PowerOutletRequest>) -> Result<DcimPowerOutletsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-outlets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsBulkUpdateResponse::Http200(r#response.json::<Vec<PowerOutlet>>()?)) },
		r#other_status => { Ok(DcimPowerOutletsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsCreateResponse {
	Http201(PowerOutlet),
	Other(Response)
}
/// Post a list of power outlet objects.
pub fn dcim_power_outlets_create(state: &ThanixClient, body: WritablePowerOutletRequest) -> Result<DcimPowerOutletsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/power-outlets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPowerOutletsCreateResponse::Http201(r#response.json::<PowerOutlet>()?)) },
		r#other_status => { Ok(DcimPowerOutletsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of power outlet objects.
pub fn dcim_power_outlets_bulk_destroy(state: &ThanixClient, body: Vec<PowerOutletRequest>) -> Result<DcimPowerOutletsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-outlets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerOutletsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsBulkPartialUpdateResponse {
	Http200(Vec<PowerOutlet>),
	Other(Response)
}
/// Patch a list of power outlet objects.
pub fn dcim_power_outlets_bulk_partial_update(state: &ThanixClient, body: Vec<PowerOutletRequest>) -> Result<DcimPowerOutletsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-outlets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<PowerOutlet>>()?)) },
		r#other_status => { Ok(DcimPowerOutletsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsRetrieveResponse {
	Http200(PowerOutlet),
	Other(Response)
}
/// Get a power outlet object.
pub fn dcim_power_outlets_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerOutletsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-outlets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsRetrieveResponse::Http200(r#response.json::<PowerOutlet>()?)) },
		r#other_status => { Ok(DcimPowerOutletsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsUpdateResponse {
	Http200(PowerOutlet),
	Other(Response)
}
/// Put a power outlet object.
pub fn dcim_power_outlets_update(state: &ThanixClient, body: WritablePowerOutletRequest, id: i64) -> Result<DcimPowerOutletsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-outlets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsUpdateResponse::Http200(r#response.json::<PowerOutlet>()?)) },
		r#other_status => { Ok(DcimPowerOutletsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a power outlet object.
pub fn dcim_power_outlets_destroy(state: &ThanixClient, id: i64) -> Result<DcimPowerOutletsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-outlets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerOutletsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsPartialUpdateResponse {
	Http200(PowerOutlet),
	Other(Response)
}
/// Patch a power outlet object.
pub fn dcim_power_outlets_partial_update(state: &ThanixClient, body: PatchedWritablePowerOutletRequest, id: i64) -> Result<DcimPowerOutletsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-outlets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsPartialUpdateResponse::Http200(r#response.json::<PowerOutlet>()?)) },
		r#other_status => { Ok(DcimPowerOutletsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerOutletsTraceRetrieveResponse {
	Http200(PowerOutlet),
	Other(Response)
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).
pub fn dcim_power_outlets_trace_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerOutletsTraceRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-outlets/{id}/trace/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerOutletsTraceRetrieveResponse::Http200(r#response.json::<PowerOutlet>()?)) },
		r#other_status => { Ok(DcimPowerOutletsTraceRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPowerPanelsListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimPowerPanelsListResponse {
	Http200(PaginatedPowerPanelList),
	Other(Response)
}
/// Get a list of power panel objects.
pub fn dcim_power_panels_list(state: &ThanixClient, query: DcimPowerPanelsListQuery) -> Result<DcimPowerPanelsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/power-panels/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPanelsListResponse::Http200(r#response.json::<PaginatedPowerPanelList>()?)) },
		r#other_status => { Ok(DcimPowerPanelsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsBulkUpdateResponse {
	Http200(Vec<PowerPanel>),
	Other(Response)
}
/// Put a list of power panel objects.
pub fn dcim_power_panels_bulk_update(state: &ThanixClient, body: Vec<PowerPanelRequest>) -> Result<DcimPowerPanelsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-panels/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPanelsBulkUpdateResponse::Http200(r#response.json::<Vec<PowerPanel>>()?)) },
		r#other_status => { Ok(DcimPowerPanelsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsCreateResponse {
	Http201(PowerPanel),
	Other(Response)
}
/// Post a list of power panel objects.
pub fn dcim_power_panels_create(state: &ThanixClient, body: WritablePowerPanelRequest) -> Result<DcimPowerPanelsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/power-panels/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPowerPanelsCreateResponse::Http201(r#response.json::<PowerPanel>()?)) },
		r#other_status => { Ok(DcimPowerPanelsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of power panel objects.
pub fn dcim_power_panels_bulk_destroy(state: &ThanixClient, body: Vec<PowerPanelRequest>) -> Result<DcimPowerPanelsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-panels/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerPanelsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsBulkPartialUpdateResponse {
	Http200(Vec<PowerPanel>),
	Other(Response)
}
/// Patch a list of power panel objects.
pub fn dcim_power_panels_bulk_partial_update(state: &ThanixClient, body: Vec<PowerPanelRequest>) -> Result<DcimPowerPanelsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-panels/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPanelsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<PowerPanel>>()?)) },
		r#other_status => { Ok(DcimPowerPanelsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsRetrieveResponse {
	Http200(PowerPanel),
	Other(Response)
}
/// Get a power panel object.
pub fn dcim_power_panels_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerPanelsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-panels/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPanelsRetrieveResponse::Http200(r#response.json::<PowerPanel>()?)) },
		r#other_status => { Ok(DcimPowerPanelsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsUpdateResponse {
	Http200(PowerPanel),
	Other(Response)
}
/// Put a power panel object.
pub fn dcim_power_panels_update(state: &ThanixClient, body: WritablePowerPanelRequest, id: i64) -> Result<DcimPowerPanelsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-panels/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPanelsUpdateResponse::Http200(r#response.json::<PowerPanel>()?)) },
		r#other_status => { Ok(DcimPowerPanelsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a power panel object.
pub fn dcim_power_panels_destroy(state: &ThanixClient, id: i64) -> Result<DcimPowerPanelsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-panels/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerPanelsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPanelsPartialUpdateResponse {
	Http200(PowerPanel),
	Other(Response)
}
/// Patch a power panel object.
pub fn dcim_power_panels_partial_update(state: &ThanixClient, body: PatchedWritablePowerPanelRequest, id: i64) -> Result<DcimPowerPanelsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-panels/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPanelsPartialUpdateResponse::Http200(r#response.json::<PowerPanel>()?)) },
		r#other_status => { Ok(DcimPowerPanelsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPowerPortTemplatesListQuery {
	pub allocated_draw: Option<Vec<i64>>,
	pub allocated_draw__empty: Option<bool>,
	pub allocated_draw__gt: Option<Vec<i64>>,
	pub allocated_draw__gte: Option<Vec<i64>>,
	pub allocated_draw__lt: Option<Vec<i64>>,
	pub allocated_draw__lte: Option<Vec<i64>>,
	pub allocated_draw__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub maximum_draw: Option<Vec<i64>>,
	pub maximum_draw__empty: Option<bool>,
	pub maximum_draw__gt: Option<Vec<i64>>,
	pub maximum_draw__gte: Option<Vec<i64>>,
	pub maximum_draw__lt: Option<Vec<i64>>,
	pub maximum_draw__lte: Option<Vec<i64>>,
	pub maximum_draw__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub r#type: Option<String>,
	pub type__n: Option<String>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesListResponse {
	Http200(PaginatedPowerPortTemplateList),
	Other(Response)
}
/// Get a list of power port template objects.
pub fn dcim_power_port_templates_list(state: &ThanixClient, query: DcimPowerPortTemplatesListQuery) -> Result<DcimPowerPortTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/power-port-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortTemplatesListResponse::Http200(r#response.json::<PaginatedPowerPortTemplateList>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesBulkUpdateResponse {
	Http200(Vec<PowerPortTemplate>),
	Other(Response)
}
/// Put a list of power port template objects.
pub fn dcim_power_port_templates_bulk_update(state: &ThanixClient, body: Vec<PowerPortTemplateRequest>) -> Result<DcimPowerPortTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<PowerPortTemplate>>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesCreateResponse {
	Http201(PowerPortTemplate),
	Other(Response)
}
/// Post a list of power port template objects.
pub fn dcim_power_port_templates_create(state: &ThanixClient, body: WritablePowerPortTemplateRequest) -> Result<DcimPowerPortTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/power-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPowerPortTemplatesCreateResponse::Http201(r#response.json::<PowerPortTemplate>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of power port template objects.
pub fn dcim_power_port_templates_bulk_destroy(state: &ThanixClient, body: Vec<PowerPortTemplateRequest>) -> Result<DcimPowerPortTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerPortTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesBulkPartialUpdateResponse {
	Http200(Vec<PowerPortTemplate>),
	Other(Response)
}
/// Patch a list of power port template objects.
pub fn dcim_power_port_templates_bulk_partial_update(state: &ThanixClient, body: Vec<PowerPortTemplateRequest>) -> Result<DcimPowerPortTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<PowerPortTemplate>>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesRetrieveResponse {
	Http200(PowerPortTemplate),
	Other(Response)
}
/// Get a power port template object.
pub fn dcim_power_port_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerPortTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortTemplatesRetrieveResponse::Http200(r#response.json::<PowerPortTemplate>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesUpdateResponse {
	Http200(PowerPortTemplate),
	Other(Response)
}
/// Put a power port template object.
pub fn dcim_power_port_templates_update(state: &ThanixClient, body: WritablePowerPortTemplateRequest, id: i64) -> Result<DcimPowerPortTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortTemplatesUpdateResponse::Http200(r#response.json::<PowerPortTemplate>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a power port template object.
pub fn dcim_power_port_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimPowerPortTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerPortTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortTemplatesPartialUpdateResponse {
	Http200(PowerPortTemplate),
	Other(Response)
}
/// Patch a power port template object.
pub fn dcim_power_port_templates_partial_update(state: &ThanixClient, body: PatchedWritablePowerPortTemplateRequest, id: i64) -> Result<DcimPowerPortTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortTemplatesPartialUpdateResponse::Http200(r#response.json::<PowerPortTemplate>()?)) },
		r#other_status => { Ok(DcimPowerPortTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimPowerPortsListQuery {
	pub allocated_draw: Option<Vec<i64>>,
	pub allocated_draw__empty: Option<bool>,
	pub allocated_draw__gt: Option<Vec<i64>>,
	pub allocated_draw__gte: Option<Vec<i64>>,
	pub allocated_draw__lt: Option<Vec<i64>>,
	pub allocated_draw__lte: Option<Vec<i64>>,
	pub allocated_draw__n: Option<Vec<i64>>,
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub connected: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub maximum_draw: Option<Vec<i64>>,
	pub maximum_draw__empty: Option<bool>,
	pub maximum_draw__gt: Option<Vec<i64>>,
	pub maximum_draw__gte: Option<Vec<i64>>,
	pub maximum_draw__lt: Option<Vec<i64>>,
	pub maximum_draw__lte: Option<Vec<i64>>,
	pub maximum_draw__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Physical port type
	pub r#type: Option<Vec<String>>,
	/// Physical port type
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimPowerPortsListResponse {
	Http200(PaginatedPowerPortList),
	Other(Response)
}
/// Get a list of power port objects.
pub fn dcim_power_ports_list(state: &ThanixClient, query: DcimPowerPortsListQuery) -> Result<DcimPowerPortsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/power-ports/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsListResponse::Http200(r#response.json::<PaginatedPowerPortList>()?)) },
		r#other_status => { Ok(DcimPowerPortsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsBulkUpdateResponse {
	Http200(Vec<PowerPort>),
	Other(Response)
}
/// Put a list of power port objects.
pub fn dcim_power_ports_bulk_update(state: &ThanixClient, body: Vec<PowerPortRequest>) -> Result<DcimPowerPortsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsBulkUpdateResponse::Http200(r#response.json::<Vec<PowerPort>>()?)) },
		r#other_status => { Ok(DcimPowerPortsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsCreateResponse {
	Http201(PowerPort),
	Other(Response)
}
/// Post a list of power port objects.
pub fn dcim_power_ports_create(state: &ThanixClient, body: WritablePowerPortRequest) -> Result<DcimPowerPortsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/power-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimPowerPortsCreateResponse::Http201(r#response.json::<PowerPort>()?)) },
		r#other_status => { Ok(DcimPowerPortsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of power port objects.
pub fn dcim_power_ports_bulk_destroy(state: &ThanixClient, body: Vec<PowerPortRequest>) -> Result<DcimPowerPortsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerPortsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsBulkPartialUpdateResponse {
	Http200(Vec<PowerPort>),
	Other(Response)
}
/// Patch a list of power port objects.
pub fn dcim_power_ports_bulk_partial_update(state: &ThanixClient, body: Vec<PowerPortRequest>) -> Result<DcimPowerPortsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<PowerPort>>()?)) },
		r#other_status => { Ok(DcimPowerPortsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsRetrieveResponse {
	Http200(PowerPort),
	Other(Response)
}
/// Get a power port object.
pub fn dcim_power_ports_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerPortsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsRetrieveResponse::Http200(r#response.json::<PowerPort>()?)) },
		r#other_status => { Ok(DcimPowerPortsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsUpdateResponse {
	Http200(PowerPort),
	Other(Response)
}
/// Put a power port object.
pub fn dcim_power_ports_update(state: &ThanixClient, body: WritablePowerPortRequest, id: i64) -> Result<DcimPowerPortsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/power-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsUpdateResponse::Http200(r#response.json::<PowerPort>()?)) },
		r#other_status => { Ok(DcimPowerPortsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a power port object.
pub fn dcim_power_ports_destroy(state: &ThanixClient, id: i64) -> Result<DcimPowerPortsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/power-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimPowerPortsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsPartialUpdateResponse {
	Http200(PowerPort),
	Other(Response)
}
/// Patch a power port object.
pub fn dcim_power_ports_partial_update(state: &ThanixClient, body: PatchedWritablePowerPortRequest, id: i64) -> Result<DcimPowerPortsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/power-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsPartialUpdateResponse::Http200(r#response.json::<PowerPort>()?)) },
		r#other_status => { Ok(DcimPowerPortsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimPowerPortsTraceRetrieveResponse {
	Http200(PowerPort),
	Other(Response)
}
/// Trace a complete cable path and return each segment as a three-tuple of (termination, cable, termination).
pub fn dcim_power_ports_trace_retrieve(state: &ThanixClient, id: i64) -> Result<DcimPowerPortsTraceRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/power-ports/{id}/trace/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimPowerPortsTraceRetrieveResponse::Http200(r#response.json::<PowerPort>()?)) },
		r#other_status => { Ok(DcimPowerPortsTraceRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimRackReservationsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<i64>>,
	/// Location (slug)
	pub location__n: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	/// User (name)
	pub user: Option<Vec<String>>,
	/// User (name)
	pub user__n: Option<Vec<String>>,
	/// User (ID)
	pub user_id: Option<Vec<i64>>,
	/// User (ID)
	pub user_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimRackReservationsListResponse {
	Http200(PaginatedRackReservationList),
	Other(Response)
}
/// Get a list of rack reservation objects.
pub fn dcim_rack_reservations_list(state: &ThanixClient, query: DcimRackReservationsListQuery) -> Result<DcimRackReservationsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/rack-reservations/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackReservationsListResponse::Http200(r#response.json::<PaginatedRackReservationList>()?)) },
		r#other_status => { Ok(DcimRackReservationsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsBulkUpdateResponse {
	Http200(Vec<RackReservation>),
	Other(Response)
}
/// Put a list of rack reservation objects.
pub fn dcim_rack_reservations_bulk_update(state: &ThanixClient, body: Vec<RackReservationRequest>) -> Result<DcimRackReservationsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rack-reservations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackReservationsBulkUpdateResponse::Http200(r#response.json::<Vec<RackReservation>>()?)) },
		r#other_status => { Ok(DcimRackReservationsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsCreateResponse {
	Http201(RackReservation),
	Other(Response)
}
/// Post a list of rack reservation objects.
pub fn dcim_rack_reservations_create(state: &ThanixClient, body: WritableRackReservationRequest) -> Result<DcimRackReservationsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/rack-reservations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimRackReservationsCreateResponse::Http201(r#response.json::<RackReservation>()?)) },
		r#other_status => { Ok(DcimRackReservationsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of rack reservation objects.
pub fn dcim_rack_reservations_bulk_destroy(state: &ThanixClient, body: Vec<RackReservationRequest>) -> Result<DcimRackReservationsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rack-reservations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRackReservationsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsBulkPartialUpdateResponse {
	Http200(Vec<RackReservation>),
	Other(Response)
}
/// Patch a list of rack reservation objects.
pub fn dcim_rack_reservations_bulk_partial_update(state: &ThanixClient, body: Vec<RackReservationRequest>) -> Result<DcimRackReservationsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rack-reservations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackReservationsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<RackReservation>>()?)) },
		r#other_status => { Ok(DcimRackReservationsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsRetrieveResponse {
	Http200(RackReservation),
	Other(Response)
}
/// Get a rack reservation object.
pub fn dcim_rack_reservations_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRackReservationsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/rack-reservations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackReservationsRetrieveResponse::Http200(r#response.json::<RackReservation>()?)) },
		r#other_status => { Ok(DcimRackReservationsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsUpdateResponse {
	Http200(RackReservation),
	Other(Response)
}
/// Put a rack reservation object.
pub fn dcim_rack_reservations_update(state: &ThanixClient, body: WritableRackReservationRequest, id: i64) -> Result<DcimRackReservationsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rack-reservations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackReservationsUpdateResponse::Http200(r#response.json::<RackReservation>()?)) },
		r#other_status => { Ok(DcimRackReservationsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a rack reservation object.
pub fn dcim_rack_reservations_destroy(state: &ThanixClient, id: i64) -> Result<DcimRackReservationsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rack-reservations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRackReservationsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackReservationsPartialUpdateResponse {
	Http200(RackReservation),
	Other(Response)
}
/// Patch a rack reservation object.
pub fn dcim_rack_reservations_partial_update(state: &ThanixClient, body: PatchedWritableRackReservationRequest, id: i64) -> Result<DcimRackReservationsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rack-reservations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackReservationsPartialUpdateResponse::Http200(r#response.json::<RackReservation>()?)) },
		r#other_status => { Ok(DcimRackReservationsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimRackRolesListQuery {
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimRackRolesListResponse {
	Http200(PaginatedRackRoleList),
	Other(Response)
}
/// Get a list of rack role objects.
pub fn dcim_rack_roles_list(state: &ThanixClient, query: DcimRackRolesListQuery) -> Result<DcimRackRolesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/rack-roles/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackRolesListResponse::Http200(r#response.json::<PaginatedRackRoleList>()?)) },
		r#other_status => { Ok(DcimRackRolesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesBulkUpdateResponse {
	Http200(Vec<RackRole>),
	Other(Response)
}
/// Put a list of rack role objects.
pub fn dcim_rack_roles_bulk_update(state: &ThanixClient, body: Vec<RackRoleRequest>) -> Result<DcimRackRolesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rack-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackRolesBulkUpdateResponse::Http200(r#response.json::<Vec<RackRole>>()?)) },
		r#other_status => { Ok(DcimRackRolesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesCreateResponse {
	Http201(RackRole),
	Other(Response)
}
/// Post a list of rack role objects.
pub fn dcim_rack_roles_create(state: &ThanixClient, body: RackRoleRequest) -> Result<DcimRackRolesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/rack-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimRackRolesCreateResponse::Http201(r#response.json::<RackRole>()?)) },
		r#other_status => { Ok(DcimRackRolesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of rack role objects.
pub fn dcim_rack_roles_bulk_destroy(state: &ThanixClient, body: Vec<RackRoleRequest>) -> Result<DcimRackRolesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rack-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRackRolesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesBulkPartialUpdateResponse {
	Http200(Vec<RackRole>),
	Other(Response)
}
/// Patch a list of rack role objects.
pub fn dcim_rack_roles_bulk_partial_update(state: &ThanixClient, body: Vec<RackRoleRequest>) -> Result<DcimRackRolesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rack-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackRolesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<RackRole>>()?)) },
		r#other_status => { Ok(DcimRackRolesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesRetrieveResponse {
	Http200(RackRole),
	Other(Response)
}
/// Get a rack role object.
pub fn dcim_rack_roles_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRackRolesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/rack-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackRolesRetrieveResponse::Http200(r#response.json::<RackRole>()?)) },
		r#other_status => { Ok(DcimRackRolesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesUpdateResponse {
	Http200(RackRole),
	Other(Response)
}
/// Put a rack role object.
pub fn dcim_rack_roles_update(state: &ThanixClient, body: RackRoleRequest, id: i64) -> Result<DcimRackRolesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rack-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackRolesUpdateResponse::Http200(r#response.json::<RackRole>()?)) },
		r#other_status => { Ok(DcimRackRolesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a rack role object.
pub fn dcim_rack_roles_destroy(state: &ThanixClient, id: i64) -> Result<DcimRackRolesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rack-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRackRolesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRackRolesPartialUpdateResponse {
	Http200(RackRole),
	Other(Response)
}
/// Patch a rack role object.
pub fn dcim_rack_roles_partial_update(state: &ThanixClient, body: PatchedRackRoleRequest, id: i64) -> Result<DcimRackRolesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rack-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRackRolesPartialUpdateResponse::Http200(r#response.json::<RackRole>()?)) },
		r#other_status => { Ok(DcimRackRolesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimRacksListQuery {
	pub asset_tag: Option<Vec<String>>,
	pub asset_tag__empty: Option<bool>,
	pub asset_tag__ic: Option<Vec<String>>,
	pub asset_tag__ie: Option<Vec<String>>,
	pub asset_tag__iew: Option<Vec<String>>,
	pub asset_tag__isw: Option<Vec<String>>,
	pub asset_tag__n: Option<Vec<String>>,
	pub asset_tag__nic: Option<Vec<String>>,
	pub asset_tag__nie: Option<Vec<String>>,
	pub asset_tag__niew: Option<Vec<String>>,
	pub asset_tag__nisw: Option<Vec<String>>,
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub desc_units: Option<bool>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub facility_id: Option<Vec<String>>,
	pub facility_id__empty: Option<bool>,
	pub facility_id__ic: Option<Vec<String>>,
	pub facility_id__ie: Option<Vec<String>>,
	pub facility_id__iew: Option<Vec<String>>,
	pub facility_id__isw: Option<Vec<String>>,
	pub facility_id__n: Option<Vec<String>>,
	pub facility_id__nic: Option<Vec<String>>,
	pub facility_id__nie: Option<Vec<String>>,
	pub facility_id__niew: Option<Vec<String>>,
	pub facility_id__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<i64>>,
	/// Location (slug)
	pub location__n: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub max_weight: Option<Vec<i64>>,
	pub max_weight__empty: Option<bool>,
	pub max_weight__gt: Option<Vec<i64>>,
	pub max_weight__gte: Option<Vec<i64>>,
	pub max_weight__lt: Option<Vec<i64>>,
	pub max_weight__lte: Option<Vec<i64>>,
	pub max_weight__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub mounting_depth: Option<Vec<i64>>,
	pub mounting_depth__empty: Option<bool>,
	pub mounting_depth__gt: Option<Vec<i64>>,
	pub mounting_depth__gte: Option<Vec<i64>>,
	pub mounting_depth__lt: Option<Vec<i64>>,
	pub mounting_depth__lte: Option<Vec<i64>>,
	pub mounting_depth__n: Option<Vec<i64>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub outer_depth: Option<Vec<i64>>,
	pub outer_depth__empty: Option<bool>,
	pub outer_depth__gt: Option<Vec<i64>>,
	pub outer_depth__gte: Option<Vec<i64>>,
	pub outer_depth__lt: Option<Vec<i64>>,
	pub outer_depth__lte: Option<Vec<i64>>,
	pub outer_depth__n: Option<Vec<i64>>,
	pub outer_unit: Option<String>,
	pub outer_unit__n: Option<String>,
	pub outer_width: Option<Vec<i64>>,
	pub outer_width__empty: Option<bool>,
	pub outer_width__gt: Option<Vec<i64>>,
	pub outer_width__gte: Option<Vec<i64>>,
	pub outer_width__lt: Option<Vec<i64>>,
	pub outer_width__lte: Option<Vec<i64>>,
	pub outer_width__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	pub serial: Option<Vec<String>>,
	pub serial__empty: Option<bool>,
	pub serial__ic: Option<Vec<String>>,
	pub serial__ie: Option<Vec<String>>,
	pub serial__iew: Option<Vec<String>>,
	pub serial__isw: Option<Vec<String>>,
	pub serial__n: Option<Vec<String>>,
	pub serial__nic: Option<Vec<String>>,
	pub serial__nie: Option<Vec<String>>,
	pub serial__niew: Option<Vec<String>>,
	pub serial__nisw: Option<Vec<String>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub starting_unit: Option<Vec<i64>>,
	pub starting_unit__empty: Option<bool>,
	pub starting_unit__gt: Option<Vec<i64>>,
	pub starting_unit__gte: Option<Vec<i64>>,
	pub starting_unit__lt: Option<Vec<i64>>,
	pub starting_unit__lte: Option<Vec<i64>>,
	pub starting_unit__n: Option<Vec<i64>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub u_height: Option<Vec<i64>>,
	pub u_height__empty: Option<bool>,
	pub u_height__gt: Option<Vec<i64>>,
	pub u_height__gte: Option<Vec<i64>>,
	pub u_height__lt: Option<Vec<i64>>,
	pub u_height__lte: Option<Vec<i64>>,
	pub u_height__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,
	pub weight: Option<Vec<f64>>,
	pub weight__empty: Option<bool>,
	pub weight__gt: Option<Vec<f64>>,
	pub weight__gte: Option<Vec<f64>>,
	pub weight__lt: Option<Vec<f64>>,
	pub weight__lte: Option<Vec<f64>>,
	pub weight__n: Option<Vec<f64>>,
	pub weight_unit: Option<String>,
	pub weight_unit__n: Option<String>,
	/// Rail-to-rail width
	pub width: Option<Vec<i64>>,
	/// Rail-to-rail width
	pub width__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimRacksListResponse {
	Http200(PaginatedRackList),
	Other(Response)
}
/// Get a list of rack objects.
pub fn dcim_racks_list(state: &ThanixClient, query: DcimRacksListQuery) -> Result<DcimRacksListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/racks/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksListResponse::Http200(r#response.json::<PaginatedRackList>()?)) },
		r#other_status => { Ok(DcimRacksListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksBulkUpdateResponse {
	Http200(Vec<Rack>),
	Other(Response)
}
/// Put a list of rack objects.
pub fn dcim_racks_bulk_update(state: &ThanixClient, body: Vec<RackRequest>) -> Result<DcimRacksBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/racks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksBulkUpdateResponse::Http200(r#response.json::<Vec<Rack>>()?)) },
		r#other_status => { Ok(DcimRacksBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksCreateResponse {
	Http201(Rack),
	Other(Response)
}
/// Post a list of rack objects.
pub fn dcim_racks_create(state: &ThanixClient, body: WritableRackRequest) -> Result<DcimRacksCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/racks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimRacksCreateResponse::Http201(r#response.json::<Rack>()?)) },
		r#other_status => { Ok(DcimRacksCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of rack objects.
pub fn dcim_racks_bulk_destroy(state: &ThanixClient, body: Vec<RackRequest>) -> Result<DcimRacksBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/racks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRacksBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksBulkPartialUpdateResponse {
	Http200(Vec<Rack>),
	Other(Response)
}
/// Patch a list of rack objects.
pub fn dcim_racks_bulk_partial_update(state: &ThanixClient, body: Vec<RackRequest>) -> Result<DcimRacksBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/racks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Rack>>()?)) },
		r#other_status => { Ok(DcimRacksBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksRetrieveResponse {
	Http200(Rack),
	Other(Response)
}
/// Get a rack object.
pub fn dcim_racks_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRacksRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/racks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksRetrieveResponse::Http200(r#response.json::<Rack>()?)) },
		r#other_status => { Ok(DcimRacksRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksUpdateResponse {
	Http200(Rack),
	Other(Response)
}
/// Put a rack object.
pub fn dcim_racks_update(state: &ThanixClient, body: WritableRackRequest, id: i64) -> Result<DcimRacksUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/racks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksUpdateResponse::Http200(r#response.json::<Rack>()?)) },
		r#other_status => { Ok(DcimRacksUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a rack object.
pub fn dcim_racks_destroy(state: &ThanixClient, id: i64) -> Result<DcimRacksDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/racks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRacksDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksPartialUpdateResponse {
	Http200(Rack),
	Other(Response)
}
/// Patch a rack object.
pub fn dcim_racks_partial_update(state: &ThanixClient, body: PatchedWritableRackRequest, id: i64) -> Result<DcimRacksPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/racks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksPartialUpdateResponse::Http200(r#response.json::<Rack>()?)) },
		r#other_status => { Ok(DcimRacksPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRacksElevationRetrieveResponse {
	Http200(Rack),
	Other(Response)
}
/// Rack elevation representing the list of rack units. Also supports rendering the elevation as an SVG.
pub fn dcim_racks_elevation_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRacksElevationRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/racks/{id}/elevation/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRacksElevationRetrieveResponse::Http200(r#response.json::<Rack>()?)) },
		r#other_status => { Ok(DcimRacksElevationRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimRearPortTemplatesListQuery {
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type (ID)
	pub devicetype_id: Option<Vec<Option<i64>>>,
	/// Device type (ID)
	pub devicetype_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// Module type (ID)
	pub moduletype_id: Option<Vec<Option<i64>>>,
	/// Module type (ID)
	pub moduletype_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub positions: Option<Vec<i64>>,
	pub positions__empty: Option<bool>,
	pub positions__gt: Option<Vec<i64>>,
	pub positions__gte: Option<Vec<i64>>,
	pub positions__lt: Option<Vec<i64>>,
	pub positions__lte: Option<Vec<i64>>,
	pub positions__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimRearPortTemplatesListResponse {
	Http200(PaginatedRearPortTemplateList),
	Other(Response)
}
/// Get a list of rear port template objects.
pub fn dcim_rear_port_templates_list(state: &ThanixClient, query: DcimRearPortTemplatesListQuery) -> Result<DcimRearPortTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/rear-port-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortTemplatesListResponse::Http200(r#response.json::<PaginatedRearPortTemplateList>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesBulkUpdateResponse {
	Http200(Vec<RearPortTemplate>),
	Other(Response)
}
/// Put a list of rear port template objects.
pub fn dcim_rear_port_templates_bulk_update(state: &ThanixClient, body: Vec<RearPortTemplateRequest>) -> Result<DcimRearPortTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rear-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<RearPortTemplate>>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesCreateResponse {
	Http201(RearPortTemplate),
	Other(Response)
}
/// Post a list of rear port template objects.
pub fn dcim_rear_port_templates_create(state: &ThanixClient, body: WritableRearPortTemplateRequest) -> Result<DcimRearPortTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/rear-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimRearPortTemplatesCreateResponse::Http201(r#response.json::<RearPortTemplate>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of rear port template objects.
pub fn dcim_rear_port_templates_bulk_destroy(state: &ThanixClient, body: Vec<RearPortTemplateRequest>) -> Result<DcimRearPortTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rear-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRearPortTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesBulkPartialUpdateResponse {
	Http200(Vec<RearPortTemplate>),
	Other(Response)
}
/// Patch a list of rear port template objects.
pub fn dcim_rear_port_templates_bulk_partial_update(state: &ThanixClient, body: Vec<RearPortTemplateRequest>) -> Result<DcimRearPortTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rear-port-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<RearPortTemplate>>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesRetrieveResponse {
	Http200(RearPortTemplate),
	Other(Response)
}
/// Get a rear port template object.
pub fn dcim_rear_port_templates_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRearPortTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/rear-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortTemplatesRetrieveResponse::Http200(r#response.json::<RearPortTemplate>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesUpdateResponse {
	Http200(RearPortTemplate),
	Other(Response)
}
/// Put a rear port template object.
pub fn dcim_rear_port_templates_update(state: &ThanixClient, body: WritableRearPortTemplateRequest, id: i64) -> Result<DcimRearPortTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rear-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortTemplatesUpdateResponse::Http200(r#response.json::<RearPortTemplate>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a rear port template object.
pub fn dcim_rear_port_templates_destroy(state: &ThanixClient, id: i64) -> Result<DcimRearPortTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rear-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRearPortTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortTemplatesPartialUpdateResponse {
	Http200(RearPortTemplate),
	Other(Response)
}
/// Patch a rear port template object.
pub fn dcim_rear_port_templates_partial_update(state: &ThanixClient, body: PatchedWritableRearPortTemplateRequest, id: i64) -> Result<DcimRearPortTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rear-port-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortTemplatesPartialUpdateResponse::Http200(r#response.json::<RearPortTemplate>()?)) },
		r#other_status => { Ok(DcimRearPortTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimRearPortsListQuery {
	pub cable_end: Option<String>,
	pub cable_end__n: Option<String>,
	pub cabled: Option<bool>,
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub device_role: Option<Vec<String>>,
	/// Device role (slug)
	pub device_role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub device_role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub device_role_id__n: Option<Vec<i64>>,
	/// Device type (model)
	pub device_type: Option<Vec<String>>,
	/// Device type (model)
	pub device_type__n: Option<Vec<String>>,
	/// Device type (ID)
	pub device_type_id: Option<Vec<i64>>,
	/// Device type (ID)
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub label: Option<Vec<String>>,
	pub label__empty: Option<bool>,
	pub label__ic: Option<Vec<String>>,
	pub label__ie: Option<Vec<String>>,
	pub label__iew: Option<Vec<String>>,
	pub label__isw: Option<Vec<String>>,
	pub label__n: Option<Vec<String>>,
	pub label__nic: Option<Vec<String>>,
	pub label__nie: Option<Vec<String>>,
	pub label__niew: Option<Vec<String>>,
	pub label__nisw: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location (ID)
	pub location_id: Option<Vec<i64>>,
	/// Location (ID)
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	/// Module (ID)
	pub module_id: Option<Vec<Option<i64>>>,
	/// Module (ID)
	pub module_id__n: Option<Vec<Option<i64>>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub occupied: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub positions: Option<Vec<i64>>,
	pub positions__empty: Option<bool>,
	pub positions__gt: Option<Vec<i64>>,
	pub positions__gte: Option<Vec<i64>>,
	pub positions__lt: Option<Vec<i64>>,
	pub positions__lte: Option<Vec<i64>>,
	pub positions__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Rack (name)
	pub rack: Option<Vec<String>>,
	/// Rack (name)
	pub rack__n: Option<Vec<String>>,
	/// Rack (ID)
	pub rack_id: Option<Vec<i64>>,
	/// Rack (ID)
	pub rack_id__n: Option<Vec<i64>>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Device role (slug)
	pub role: Option<Vec<String>>,
	/// Device role (slug)
	pub role__n: Option<Vec<String>>,
	/// Device role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Device role (ID)
	pub role_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual Chassis
	pub virtual_chassis: Option<Vec<String>>,
	/// Virtual Chassis
	pub virtual_chassis__n: Option<Vec<String>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id: Option<Vec<i64>>,
	/// Virtual Chassis (ID)
	pub virtual_chassis_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum DcimRearPortsListResponse {
	Http200(PaginatedRearPortList),
	Other(Response)
}
/// Get a list of rear port objects.
pub fn dcim_rear_ports_list(state: &ThanixClient, query: DcimRearPortsListQuery) -> Result<DcimRearPortsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/rear-ports/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsListResponse::Http200(r#response.json::<PaginatedRearPortList>()?)) },
		r#other_status => { Ok(DcimRearPortsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsBulkUpdateResponse {
	Http200(Vec<RearPort>),
	Other(Response)
}
/// Put a list of rear port objects.
pub fn dcim_rear_ports_bulk_update(state: &ThanixClient, body: Vec<RearPortRequest>) -> Result<DcimRearPortsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rear-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsBulkUpdateResponse::Http200(r#response.json::<Vec<RearPort>>()?)) },
		r#other_status => { Ok(DcimRearPortsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsCreateResponse {
	Http201(RearPort),
	Other(Response)
}
/// Post a list of rear port objects.
pub fn dcim_rear_ports_create(state: &ThanixClient, body: WritableRearPortRequest) -> Result<DcimRearPortsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/rear-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimRearPortsCreateResponse::Http201(r#response.json::<RearPort>()?)) },
		r#other_status => { Ok(DcimRearPortsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of rear port objects.
pub fn dcim_rear_ports_bulk_destroy(state: &ThanixClient, body: Vec<RearPortRequest>) -> Result<DcimRearPortsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rear-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRearPortsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsBulkPartialUpdateResponse {
	Http200(Vec<RearPort>),
	Other(Response)
}
/// Patch a list of rear port objects.
pub fn dcim_rear_ports_bulk_partial_update(state: &ThanixClient, body: Vec<RearPortRequest>) -> Result<DcimRearPortsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rear-ports/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<RearPort>>()?)) },
		r#other_status => { Ok(DcimRearPortsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsRetrieveResponse {
	Http200(RearPort),
	Other(Response)
}
/// Get a rear port object.
pub fn dcim_rear_ports_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRearPortsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/rear-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsRetrieveResponse::Http200(r#response.json::<RearPort>()?)) },
		r#other_status => { Ok(DcimRearPortsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsUpdateResponse {
	Http200(RearPort),
	Other(Response)
}
/// Put a rear port object.
pub fn dcim_rear_ports_update(state: &ThanixClient, body: WritableRearPortRequest, id: i64) -> Result<DcimRearPortsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/rear-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsUpdateResponse::Http200(r#response.json::<RearPort>()?)) },
		r#other_status => { Ok(DcimRearPortsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a rear port object.
pub fn dcim_rear_ports_destroy(state: &ThanixClient, id: i64) -> Result<DcimRearPortsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/rear-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRearPortsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsPartialUpdateResponse {
	Http200(RearPort),
	Other(Response)
}
/// Patch a rear port object.
pub fn dcim_rear_ports_partial_update(state: &ThanixClient, body: PatchedWritableRearPortRequest, id: i64) -> Result<DcimRearPortsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/rear-ports/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsPartialUpdateResponse::Http200(r#response.json::<RearPort>()?)) },
		r#other_status => { Ok(DcimRearPortsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRearPortsPathsRetrieveResponse {
	Http200(RearPort),
	Other(Response)
}
/// Return all CablePaths which traverse a given pass-through port.
pub fn dcim_rear_ports_paths_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRearPortsPathsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/rear-ports/{id}/paths/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRearPortsPathsRetrieveResponse::Http200(r#response.json::<RearPort>()?)) },
		r#other_status => { Ok(DcimRearPortsPathsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimRegionsListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent region (slug)
	pub parent: Option<Vec<String>>,
	/// Parent region (slug)
	pub parent__n: Option<Vec<String>>,
	/// Parent region (ID)
	pub parent_id: Option<Vec<Option<i64>>>,
	/// Parent region (ID)
	pub parent_id__n: Option<Vec<Option<i64>>>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimRegionsListResponse {
	Http200(PaginatedRegionList),
	Other(Response)
}
/// Get a list of region objects.
pub fn dcim_regions_list(state: &ThanixClient, query: DcimRegionsListQuery) -> Result<DcimRegionsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/regions/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRegionsListResponse::Http200(r#response.json::<PaginatedRegionList>()?)) },
		r#other_status => { Ok(DcimRegionsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsBulkUpdateResponse {
	Http200(Vec<Region>),
	Other(Response)
}
/// Put a list of region objects.
pub fn dcim_regions_bulk_update(state: &ThanixClient, body: Vec<RegionRequest>) -> Result<DcimRegionsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/regions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRegionsBulkUpdateResponse::Http200(r#response.json::<Vec<Region>>()?)) },
		r#other_status => { Ok(DcimRegionsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsCreateResponse {
	Http201(Region),
	Other(Response)
}
/// Post a list of region objects.
pub fn dcim_regions_create(state: &ThanixClient, body: WritableRegionRequest) -> Result<DcimRegionsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/regions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimRegionsCreateResponse::Http201(r#response.json::<Region>()?)) },
		r#other_status => { Ok(DcimRegionsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of region objects.
pub fn dcim_regions_bulk_destroy(state: &ThanixClient, body: Vec<RegionRequest>) -> Result<DcimRegionsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/regions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRegionsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsBulkPartialUpdateResponse {
	Http200(Vec<Region>),
	Other(Response)
}
/// Patch a list of region objects.
pub fn dcim_regions_bulk_partial_update(state: &ThanixClient, body: Vec<RegionRequest>) -> Result<DcimRegionsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/regions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRegionsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Region>>()?)) },
		r#other_status => { Ok(DcimRegionsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsRetrieveResponse {
	Http200(Region),
	Other(Response)
}
/// Get a region object.
pub fn dcim_regions_retrieve(state: &ThanixClient, id: i64) -> Result<DcimRegionsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/regions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRegionsRetrieveResponse::Http200(r#response.json::<Region>()?)) },
		r#other_status => { Ok(DcimRegionsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsUpdateResponse {
	Http200(Region),
	Other(Response)
}
/// Put a region object.
pub fn dcim_regions_update(state: &ThanixClient, body: WritableRegionRequest, id: i64) -> Result<DcimRegionsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/regions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRegionsUpdateResponse::Http200(r#response.json::<Region>()?)) },
		r#other_status => { Ok(DcimRegionsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a region object.
pub fn dcim_regions_destroy(state: &ThanixClient, id: i64) -> Result<DcimRegionsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/regions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimRegionsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimRegionsPartialUpdateResponse {
	Http200(Region),
	Other(Response)
}
/// Patch a region object.
pub fn dcim_regions_partial_update(state: &ThanixClient, body: PatchedWritableRegionRequest, id: i64) -> Result<DcimRegionsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/regions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimRegionsPartialUpdateResponse::Http200(r#response.json::<Region>()?)) },
		r#other_status => { Ok(DcimRegionsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimSiteGroupsListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent site group (slug)
	pub parent: Option<Vec<String>>,
	/// Parent site group (slug)
	pub parent__n: Option<Vec<String>>,
	/// Parent site group (ID)
	pub parent_id: Option<Vec<Option<i64>>>,
	/// Parent site group (ID)
	pub parent_id__n: Option<Vec<Option<i64>>>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimSiteGroupsListResponse {
	Http200(PaginatedSiteGroupList),
	Other(Response)
}
/// Get a list of site group objects.
pub fn dcim_site_groups_list(state: &ThanixClient, query: DcimSiteGroupsListQuery) -> Result<DcimSiteGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/site-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSiteGroupsListResponse::Http200(r#response.json::<PaginatedSiteGroupList>()?)) },
		r#other_status => { Ok(DcimSiteGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsBulkUpdateResponse {
	Http200(Vec<SiteGroup>),
	Other(Response)
}
/// Put a list of site group objects.
pub fn dcim_site_groups_bulk_update(state: &ThanixClient, body: Vec<SiteGroupRequest>) -> Result<DcimSiteGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/site-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSiteGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<SiteGroup>>()?)) },
		r#other_status => { Ok(DcimSiteGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsCreateResponse {
	Http201(SiteGroup),
	Other(Response)
}
/// Post a list of site group objects.
pub fn dcim_site_groups_create(state: &ThanixClient, body: WritableSiteGroupRequest) -> Result<DcimSiteGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/site-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimSiteGroupsCreateResponse::Http201(r#response.json::<SiteGroup>()?)) },
		r#other_status => { Ok(DcimSiteGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of site group objects.
pub fn dcim_site_groups_bulk_destroy(state: &ThanixClient, body: Vec<SiteGroupRequest>) -> Result<DcimSiteGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/site-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimSiteGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsBulkPartialUpdateResponse {
	Http200(Vec<SiteGroup>),
	Other(Response)
}
/// Patch a list of site group objects.
pub fn dcim_site_groups_bulk_partial_update(state: &ThanixClient, body: Vec<SiteGroupRequest>) -> Result<DcimSiteGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/site-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSiteGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<SiteGroup>>()?)) },
		r#other_status => { Ok(DcimSiteGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsRetrieveResponse {
	Http200(SiteGroup),
	Other(Response)
}
/// Get a site group object.
pub fn dcim_site_groups_retrieve(state: &ThanixClient, id: i64) -> Result<DcimSiteGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/site-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSiteGroupsRetrieveResponse::Http200(r#response.json::<SiteGroup>()?)) },
		r#other_status => { Ok(DcimSiteGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsUpdateResponse {
	Http200(SiteGroup),
	Other(Response)
}
/// Put a site group object.
pub fn dcim_site_groups_update(state: &ThanixClient, body: WritableSiteGroupRequest, id: i64) -> Result<DcimSiteGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/site-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSiteGroupsUpdateResponse::Http200(r#response.json::<SiteGroup>()?)) },
		r#other_status => { Ok(DcimSiteGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a site group object.
pub fn dcim_site_groups_destroy(state: &ThanixClient, id: i64) -> Result<DcimSiteGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/site-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimSiteGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSiteGroupsPartialUpdateResponse {
	Http200(SiteGroup),
	Other(Response)
}
/// Patch a site group object.
pub fn dcim_site_groups_partial_update(state: &ThanixClient, body: PatchedWritableSiteGroupRequest, id: i64) -> Result<DcimSiteGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/site-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSiteGroupsPartialUpdateResponse::Http200(r#response.json::<SiteGroup>()?)) },
		r#other_status => { Ok(DcimSiteGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimSitesListQuery {
	/// AS (ID)
	pub asn: Option<Vec<u32>>,
	/// AS (ID)
	pub asn__n: Option<Vec<u32>>,
	/// AS (ID)
	pub asn_id: Option<Vec<i64>>,
	/// AS (ID)
	pub asn_id__n: Option<Vec<i64>>,
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub facility: Option<Vec<String>>,
	pub facility__empty: Option<bool>,
	pub facility__ic: Option<Vec<String>>,
	pub facility__ie: Option<Vec<String>>,
	pub facility__iew: Option<Vec<String>>,
	pub facility__isw: Option<Vec<String>>,
	pub facility__n: Option<Vec<String>>,
	pub facility__nic: Option<Vec<String>>,
	pub facility__nie: Option<Vec<String>>,
	pub facility__niew: Option<Vec<String>>,
	pub facility__nisw: Option<Vec<String>>,
	/// Group (slug)
	pub group: Option<Vec<i64>>,
	/// Group (slug)
	pub group__n: Option<Vec<i64>>,
	/// Group (ID)
	pub group_id: Option<Vec<i64>>,
	/// Group (ID)
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	pub latitude: Option<Vec<f64>>,
	pub latitude__empty: Option<bool>,
	pub latitude__gt: Option<Vec<f64>>,
	pub latitude__gte: Option<Vec<f64>>,
	pub latitude__lt: Option<Vec<f64>>,
	pub latitude__lte: Option<Vec<f64>>,
	pub latitude__n: Option<Vec<f64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub longitude: Option<Vec<f64>>,
	pub longitude__empty: Option<bool>,
	pub longitude__gt: Option<Vec<f64>>,
	pub longitude__gte: Option<Vec<f64>>,
	pub longitude__lt: Option<Vec<f64>>,
	pub longitude__lte: Option<Vec<f64>>,
	pub longitude__n: Option<Vec<f64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimSitesListResponse {
	Http200(PaginatedSiteList),
	Other(Response)
}
/// Get a list of site objects.
pub fn dcim_sites_list(state: &ThanixClient, query: DcimSitesListQuery) -> Result<DcimSitesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/sites/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSitesListResponse::Http200(r#response.json::<PaginatedSiteList>()?)) },
		r#other_status => { Ok(DcimSitesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesBulkUpdateResponse {
	Http200(Vec<Site>),
	Other(Response)
}
/// Put a list of site objects.
pub fn dcim_sites_bulk_update(state: &ThanixClient, body: Vec<SiteRequest>) -> Result<DcimSitesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/sites/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSitesBulkUpdateResponse::Http200(r#response.json::<Vec<Site>>()?)) },
		r#other_status => { Ok(DcimSitesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesCreateResponse {
	Http201(Site),
	Other(Response)
}
/// Post a list of site objects.
pub fn dcim_sites_create(state: &ThanixClient, body: WritableSiteRequest) -> Result<DcimSitesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/sites/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimSitesCreateResponse::Http201(r#response.json::<Site>()?)) },
		r#other_status => { Ok(DcimSitesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of site objects.
pub fn dcim_sites_bulk_destroy(state: &ThanixClient, body: Vec<SiteRequest>) -> Result<DcimSitesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/sites/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimSitesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesBulkPartialUpdateResponse {
	Http200(Vec<Site>),
	Other(Response)
}
/// Patch a list of site objects.
pub fn dcim_sites_bulk_partial_update(state: &ThanixClient, body: Vec<SiteRequest>) -> Result<DcimSitesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/sites/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSitesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Site>>()?)) },
		r#other_status => { Ok(DcimSitesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesRetrieveResponse {
	Http200(Site),
	Other(Response)
}
/// Get a site object.
pub fn dcim_sites_retrieve(state: &ThanixClient, id: i64) -> Result<DcimSitesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/sites/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSitesRetrieveResponse::Http200(r#response.json::<Site>()?)) },
		r#other_status => { Ok(DcimSitesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesUpdateResponse {
	Http200(Site),
	Other(Response)
}
/// Put a site object.
pub fn dcim_sites_update(state: &ThanixClient, body: WritableSiteRequest, id: i64) -> Result<DcimSitesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/sites/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSitesUpdateResponse::Http200(r#response.json::<Site>()?)) },
		r#other_status => { Ok(DcimSitesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a site object.
pub fn dcim_sites_destroy(state: &ThanixClient, id: i64) -> Result<DcimSitesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/sites/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimSitesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimSitesPartialUpdateResponse {
	Http200(Site),
	Other(Response)
}
/// Patch a site object.
pub fn dcim_sites_partial_update(state: &ThanixClient, body: PatchedWritableSiteRequest, id: i64) -> Result<DcimSitesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/sites/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimSitesPartialUpdateResponse::Http200(r#response.json::<Site>()?)) },
		r#other_status => { Ok(DcimSitesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimVirtualChassisListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub domain: Option<Vec<String>>,
	pub domain__empty: Option<bool>,
	pub domain__ic: Option<Vec<String>>,
	pub domain__ie: Option<Vec<String>>,
	pub domain__iew: Option<Vec<String>>,
	pub domain__isw: Option<Vec<String>>,
	pub domain__n: Option<Vec<String>>,
	pub domain__nic: Option<Vec<String>>,
	pub domain__nie: Option<Vec<String>>,
	pub domain__niew: Option<Vec<String>>,
	pub domain__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Master (name)
	pub master: Option<Vec<Option<String>>>,
	/// Master (name)
	pub master__n: Option<Vec<Option<String>>>,
	/// Master (ID)
	pub master_id: Option<Vec<Option<i64>>>,
	/// Master (ID)
	pub master_id__n: Option<Vec<Option<i64>>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site name (slug)
	pub site: Option<Vec<String>>,
	/// Site name (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimVirtualChassisListResponse {
	Http200(PaginatedVirtualChassisList),
	Other(Response)
}
/// Get a list of virtual chassis objects.
pub fn dcim_virtual_chassis_list(state: &ThanixClient, query: DcimVirtualChassisListQuery) -> Result<DcimVirtualChassisListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/virtual-chassis/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualChassisListResponse::Http200(r#response.json::<PaginatedVirtualChassisList>()?)) },
		r#other_status => { Ok(DcimVirtualChassisListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisBulkUpdateResponse {
	Http200(Vec<VirtualChassis>),
	Other(Response)
}
/// Put a list of virtual chassis objects.
pub fn dcim_virtual_chassis_bulk_update(state: &ThanixClient, body: Vec<VirtualChassisRequest>) -> Result<DcimVirtualChassisBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/virtual-chassis/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualChassisBulkUpdateResponse::Http200(r#response.json::<Vec<VirtualChassis>>()?)) },
		r#other_status => { Ok(DcimVirtualChassisBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisCreateResponse {
	Http201(VirtualChassis),
	Other(Response)
}
/// Post a list of virtual chassis objects.
pub fn dcim_virtual_chassis_create(state: &ThanixClient, body: WritableVirtualChassisRequest) -> Result<DcimVirtualChassisCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/virtual-chassis/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimVirtualChassisCreateResponse::Http201(r#response.json::<VirtualChassis>()?)) },
		r#other_status => { Ok(DcimVirtualChassisCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of virtual chassis objects.
pub fn dcim_virtual_chassis_bulk_destroy(state: &ThanixClient, body: Vec<VirtualChassisRequest>) -> Result<DcimVirtualChassisBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/virtual-chassis/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimVirtualChassisBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisBulkPartialUpdateResponse {
	Http200(Vec<VirtualChassis>),
	Other(Response)
}
/// Patch a list of virtual chassis objects.
pub fn dcim_virtual_chassis_bulk_partial_update(state: &ThanixClient, body: Vec<VirtualChassisRequest>) -> Result<DcimVirtualChassisBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/virtual-chassis/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualChassisBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VirtualChassis>>()?)) },
		r#other_status => { Ok(DcimVirtualChassisBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisRetrieveResponse {
	Http200(VirtualChassis),
	Other(Response)
}
/// Get a virtual chassis object.
pub fn dcim_virtual_chassis_retrieve(state: &ThanixClient, id: i64) -> Result<DcimVirtualChassisRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/virtual-chassis/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualChassisRetrieveResponse::Http200(r#response.json::<VirtualChassis>()?)) },
		r#other_status => { Ok(DcimVirtualChassisRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisUpdateResponse {
	Http200(VirtualChassis),
	Other(Response)
}
/// Put a virtual chassis object.
pub fn dcim_virtual_chassis_update(state: &ThanixClient, body: WritableVirtualChassisRequest, id: i64) -> Result<DcimVirtualChassisUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/virtual-chassis/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualChassisUpdateResponse::Http200(r#response.json::<VirtualChassis>()?)) },
		r#other_status => { Ok(DcimVirtualChassisUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a virtual chassis object.
pub fn dcim_virtual_chassis_destroy(state: &ThanixClient, id: i64) -> Result<DcimVirtualChassisDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/virtual-chassis/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimVirtualChassisDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualChassisPartialUpdateResponse {
	Http200(VirtualChassis),
	Other(Response)
}
/// Patch a virtual chassis object.
pub fn dcim_virtual_chassis_partial_update(state: &ThanixClient, body: PatchedWritableVirtualChassisRequest, id: i64) -> Result<DcimVirtualChassisPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/virtual-chassis/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualChassisPartialUpdateResponse::Http200(r#response.json::<VirtualChassis>()?)) },
		r#other_status => { Ok(DcimVirtualChassisPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DcimVirtualDeviceContextsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device model
	pub device: Option<Vec<i64>>,
	/// Device model
	pub device__n: Option<Vec<i64>>,
	/// VDC (ID)
	pub device_id: Option<Vec<i64>>,
	/// VDC (ID)
	pub device_id__n: Option<Vec<i64>>,
	/// Has a primary IP
	pub has_primary_ip: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Primary IPv4 (ID)
	pub primary_ip4_id: Option<Vec<i64>>,
	/// Primary IPv4 (ID)
	pub primary_ip4_id__n: Option<Vec<i64>>,
	/// Primary IPv6 (ID)
	pub primary_ip6_id: Option<Vec<i64>>,
	/// Primary IPv6 (ID)
	pub primary_ip6_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsListResponse {
	Http200(PaginatedVirtualDeviceContextList),
	Other(Response)
}
/// Get a list of virtual device context objects.
pub fn dcim_virtual_device_contexts_list(state: &ThanixClient, query: DcimVirtualDeviceContextsListQuery) -> Result<DcimVirtualDeviceContextsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/dcim/virtual-device-contexts/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualDeviceContextsListResponse::Http200(r#response.json::<PaginatedVirtualDeviceContextList>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsBulkUpdateResponse {
	Http200(Vec<VirtualDeviceContext>),
	Other(Response)
}
/// Put a list of virtual device context objects.
pub fn dcim_virtual_device_contexts_bulk_update(state: &ThanixClient, body: Vec<VirtualDeviceContextRequest>) -> Result<DcimVirtualDeviceContextsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/virtual-device-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualDeviceContextsBulkUpdateResponse::Http200(r#response.json::<Vec<VirtualDeviceContext>>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsCreateResponse {
	Http201(VirtualDeviceContext),
	Other(Response)
}
/// Post a list of virtual device context objects.
pub fn dcim_virtual_device_contexts_create(state: &ThanixClient, body: WritableVirtualDeviceContextRequest) -> Result<DcimVirtualDeviceContextsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/dcim/virtual-device-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(DcimVirtualDeviceContextsCreateResponse::Http201(r#response.json::<VirtualDeviceContext>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of virtual device context objects.
pub fn dcim_virtual_device_contexts_bulk_destroy(state: &ThanixClient, body: Vec<VirtualDeviceContextRequest>) -> Result<DcimVirtualDeviceContextsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/virtual-device-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimVirtualDeviceContextsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsBulkPartialUpdateResponse {
	Http200(Vec<VirtualDeviceContext>),
	Other(Response)
}
/// Patch a list of virtual device context objects.
pub fn dcim_virtual_device_contexts_bulk_partial_update(state: &ThanixClient, body: Vec<VirtualDeviceContextRequest>) -> Result<DcimVirtualDeviceContextsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/virtual-device-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualDeviceContextsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VirtualDeviceContext>>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsRetrieveResponse {
	Http200(VirtualDeviceContext),
	Other(Response)
}
/// Get a virtual device context object.
pub fn dcim_virtual_device_contexts_retrieve(state: &ThanixClient, id: i64) -> Result<DcimVirtualDeviceContextsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/dcim/virtual-device-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualDeviceContextsRetrieveResponse::Http200(r#response.json::<VirtualDeviceContext>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsUpdateResponse {
	Http200(VirtualDeviceContext),
	Other(Response)
}
/// Put a virtual device context object.
pub fn dcim_virtual_device_contexts_update(state: &ThanixClient, body: WritableVirtualDeviceContextRequest, id: i64) -> Result<DcimVirtualDeviceContextsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/dcim/virtual-device-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualDeviceContextsUpdateResponse::Http200(r#response.json::<VirtualDeviceContext>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a virtual device context object.
pub fn dcim_virtual_device_contexts_destroy(state: &ThanixClient, id: i64) -> Result<DcimVirtualDeviceContextsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/dcim/virtual-device-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(DcimVirtualDeviceContextsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum DcimVirtualDeviceContextsPartialUpdateResponse {
	Http200(VirtualDeviceContext),
	Other(Response)
}
/// Patch a virtual device context object.
pub fn dcim_virtual_device_contexts_partial_update(state: &ThanixClient, body: PatchedWritableVirtualDeviceContextRequest, id: i64) -> Result<DcimVirtualDeviceContextsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/dcim/virtual-device-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(DcimVirtualDeviceContextsPartialUpdateResponse::Http200(r#response.json::<VirtualDeviceContext>()?)) },
		r#other_status => { Ok(DcimVirtualDeviceContextsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasBookmarksListQuery {
	pub created: Option<String>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub object_id: Option<Vec<i64>>,
	pub object_id__empty: Option<bool>,
	pub object_id__gt: Option<Vec<i64>>,
	pub object_id__gte: Option<Vec<i64>>,
	pub object_id__lt: Option<Vec<i64>>,
	pub object_id__lte: Option<Vec<i64>>,
	pub object_id__n: Option<Vec<i64>>,
	pub object_type: Option<String>,
	pub object_type__n: Option<String>,
	pub object_type_id: Option<Vec<i64>>,
	pub object_type_id__empty: Option<Vec<i64>>,
	pub object_type_id__gt: Option<Vec<i64>>,
	pub object_type_id__gte: Option<Vec<i64>>,
	pub object_type_id__lt: Option<Vec<i64>>,
	pub object_type_id__lte: Option<Vec<i64>>,
	pub object_type_id__n: Option<Vec<i64>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// User (name)
	pub user: Option<Vec<String>>,
	/// User (name)
	pub user__n: Option<Vec<String>>,
	/// User (ID)
	pub user_id: Option<Vec<i64>>,
	/// User (ID)
	pub user_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum ExtrasBookmarksListResponse {
	Http200(PaginatedBookmarkList),
	Other(Response)
}
/// Get a list of bookmark objects.
pub fn extras_bookmarks_list(state: &ThanixClient, query: ExtrasBookmarksListQuery) -> Result<ExtrasBookmarksListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/bookmarks/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasBookmarksListResponse::Http200(r#response.json::<PaginatedBookmarkList>()?)) },
		r#other_status => { Ok(ExtrasBookmarksListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksBulkUpdateResponse {
	Http200(Vec<Bookmark>),
	Other(Response)
}
/// Put a list of bookmark objects.
pub fn extras_bookmarks_bulk_update(state: &ThanixClient, body: Vec<BookmarkRequest>) -> Result<ExtrasBookmarksBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/bookmarks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasBookmarksBulkUpdateResponse::Http200(r#response.json::<Vec<Bookmark>>()?)) },
		r#other_status => { Ok(ExtrasBookmarksBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksCreateResponse {
	Http201(Bookmark),
	Other(Response)
}
/// Post a list of bookmark objects.
pub fn extras_bookmarks_create(state: &ThanixClient, body: WritableBookmarkRequest) -> Result<ExtrasBookmarksCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/bookmarks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasBookmarksCreateResponse::Http201(r#response.json::<Bookmark>()?)) },
		r#other_status => { Ok(ExtrasBookmarksCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of bookmark objects.
pub fn extras_bookmarks_bulk_destroy(state: &ThanixClient, body: Vec<BookmarkRequest>) -> Result<ExtrasBookmarksBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/bookmarks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasBookmarksBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksBulkPartialUpdateResponse {
	Http200(Vec<Bookmark>),
	Other(Response)
}
/// Patch a list of bookmark objects.
pub fn extras_bookmarks_bulk_partial_update(state: &ThanixClient, body: Vec<BookmarkRequest>) -> Result<ExtrasBookmarksBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/bookmarks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasBookmarksBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Bookmark>>()?)) },
		r#other_status => { Ok(ExtrasBookmarksBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksRetrieveResponse {
	Http200(Bookmark),
	Other(Response)
}
/// Get a bookmark object.
pub fn extras_bookmarks_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasBookmarksRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/bookmarks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasBookmarksRetrieveResponse::Http200(r#response.json::<Bookmark>()?)) },
		r#other_status => { Ok(ExtrasBookmarksRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksUpdateResponse {
	Http200(Bookmark),
	Other(Response)
}
/// Put a bookmark object.
pub fn extras_bookmarks_update(state: &ThanixClient, body: WritableBookmarkRequest, id: i64) -> Result<ExtrasBookmarksUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/bookmarks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasBookmarksUpdateResponse::Http200(r#response.json::<Bookmark>()?)) },
		r#other_status => { Ok(ExtrasBookmarksUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a bookmark object.
pub fn extras_bookmarks_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasBookmarksDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/bookmarks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasBookmarksDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasBookmarksPartialUpdateResponse {
	Http200(Bookmark),
	Other(Response)
}
/// Patch a bookmark object.
pub fn extras_bookmarks_partial_update(state: &ThanixClient, body: PatchedWritableBookmarkRequest, id: i64) -> Result<ExtrasBookmarksPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/bookmarks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasBookmarksPartialUpdateResponse::Http200(r#response.json::<Bookmark>()?)) },
		r#other_status => { Ok(ExtrasBookmarksPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasConfigContextsListQuery {
	/// Cluster group (slug)
	pub cluster_group: Option<Vec<String>>,
	/// Cluster group (slug)
	pub cluster_group__n: Option<Vec<String>>,
	/// Cluster group
	pub cluster_group_id: Option<Vec<i64>>,
	/// Cluster group
	pub cluster_group_id__n: Option<Vec<i64>>,
	/// Cluster
	pub cluster_id: Option<Vec<i64>>,
	/// Cluster
	pub cluster_id__n: Option<Vec<i64>>,
	/// Cluster type (slug)
	pub cluster_type: Option<Vec<String>>,
	/// Cluster type (slug)
	pub cluster_type__n: Option<Vec<String>>,
	/// Cluster type
	pub cluster_type_id: Option<Vec<i64>>,
	/// Cluster type
	pub cluster_type_id__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	/// Data file (ID)
	pub data_file_id: Option<Vec<Option<i64>>>,
	/// Data file (ID)
	pub data_file_id__n: Option<Vec<Option<i64>>>,
	/// Data source (ID)
	pub data_source_id: Option<Vec<Option<i64>>>,
	/// Data source (ID)
	pub data_source_id__n: Option<Vec<Option<i64>>>,
	pub data_synced: Option<Vec<String>>,
	pub data_synced__empty: Option<bool>,
	pub data_synced__gt: Option<Vec<String>>,
	pub data_synced__gte: Option<Vec<String>>,
	pub data_synced__lt: Option<Vec<String>>,
	pub data_synced__lte: Option<Vec<String>>,
	pub data_synced__n: Option<Vec<String>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device type
	pub device_type_id: Option<Vec<i64>>,
	/// Device type
	pub device_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub is_active: Option<bool>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Location (slug)
	pub location: Option<Vec<String>>,
	/// Location (slug)
	pub location__n: Option<Vec<String>>,
	/// Location
	pub location_id: Option<Vec<i64>>,
	/// Location
	pub location_id__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Platform (slug)
	pub platform: Option<Vec<String>>,
	/// Platform (slug)
	pub platform__n: Option<Vec<String>>,
	/// Platform
	pub platform_id: Option<Vec<i64>>,
	/// Platform
	pub platform_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<String>>,
	/// Region (slug)
	pub region__n: Option<Vec<String>>,
	/// Region
	pub region_id: Option<Vec<i64>>,
	/// Region
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role
	pub role_id: Option<Vec<i64>>,
	/// Role
	pub role_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<String>>,
	/// Site group
	pub site_group_id: Option<Vec<i64>>,
	/// Site group
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site
	pub site_id: Option<Vec<i64>>,
	/// Site
	pub site_id__n: Option<Vec<i64>>,
	/// Tag (slug)
	pub tag: Option<Vec<String>>,
	/// Tag (slug)
	pub tag__n: Option<Vec<String>>,
	/// Tag
	pub tag_id: Option<Vec<i64>>,
	/// Tag
	pub tag_id__n: Option<Vec<i64>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant group (slug)
	pub tenant_group: Option<Vec<String>>,
	/// Tenant group (slug)
	pub tenant_group__n: Option<Vec<String>>,
	/// Tenant group
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant group
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant
	pub tenant_id: Option<Vec<i64>>,
	/// Tenant
	pub tenant_id__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasConfigContextsListResponse {
	Http200(PaginatedConfigContextList),
	Other(Response)
}
/// Get a list of config context objects.
pub fn extras_config_contexts_list(state: &ThanixClient, query: ExtrasConfigContextsListQuery) -> Result<ExtrasConfigContextsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/config-contexts/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsListResponse::Http200(r#response.json::<PaginatedConfigContextList>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsBulkUpdateResponse {
	Http200(Vec<ConfigContext>),
	Other(Response)
}
/// Put a list of config context objects.
pub fn extras_config_contexts_bulk_update(state: &ThanixClient, body: Vec<ConfigContextRequest>) -> Result<ExtrasConfigContextsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/config-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsBulkUpdateResponse::Http200(r#response.json::<Vec<ConfigContext>>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsCreateResponse {
	Http201(ConfigContext),
	Other(Response)
}
/// Post a list of config context objects.
pub fn extras_config_contexts_create(state: &ThanixClient, body: WritableConfigContextRequest) -> Result<ExtrasConfigContextsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/config-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasConfigContextsCreateResponse::Http201(r#response.json::<ConfigContext>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of config context objects.
pub fn extras_config_contexts_bulk_destroy(state: &ThanixClient, body: Vec<ConfigContextRequest>) -> Result<ExtrasConfigContextsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/config-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasConfigContextsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsBulkPartialUpdateResponse {
	Http200(Vec<ConfigContext>),
	Other(Response)
}
/// Patch a list of config context objects.
pub fn extras_config_contexts_bulk_partial_update(state: &ThanixClient, body: Vec<ConfigContextRequest>) -> Result<ExtrasConfigContextsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/config-contexts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ConfigContext>>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsRetrieveResponse {
	Http200(ConfigContext),
	Other(Response)
}
/// Get a config context object.
pub fn extras_config_contexts_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasConfigContextsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/config-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsRetrieveResponse::Http200(r#response.json::<ConfigContext>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsUpdateResponse {
	Http200(ConfigContext),
	Other(Response)
}
/// Put a config context object.
pub fn extras_config_contexts_update(state: &ThanixClient, body: WritableConfigContextRequest, id: i64) -> Result<ExtrasConfigContextsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/config-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsUpdateResponse::Http200(r#response.json::<ConfigContext>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a config context object.
pub fn extras_config_contexts_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasConfigContextsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/config-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasConfigContextsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsPartialUpdateResponse {
	Http200(ConfigContext),
	Other(Response)
}
/// Patch a config context object.
pub fn extras_config_contexts_partial_update(state: &ThanixClient, body: PatchedWritableConfigContextRequest, id: i64) -> Result<ExtrasConfigContextsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/config-contexts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsPartialUpdateResponse::Http200(r#response.json::<ConfigContext>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigContextsSyncCreateResponse {
	Http200(ConfigContext),
	Other(Response)
}
/// Provide a /sync API endpoint to synchronize an object's data from its associated DataFile (if any).
pub fn extras_config_contexts_sync_create(state: &ThanixClient, body: WritableConfigContextRequest, id: i64) -> Result<ExtrasConfigContextsSyncCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/config-contexts/{id}/sync/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigContextsSyncCreateResponse::Http200(r#response.json::<ConfigContext>()?)) },
		r#other_status => { Ok(ExtrasConfigContextsSyncCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasConfigTemplatesListQuery {
	/// Data file (ID)
	pub data_file_id: Option<Vec<Option<i64>>>,
	/// Data file (ID)
	pub data_file_id__n: Option<Vec<Option<i64>>>,
	/// Data source (ID)
	pub data_source_id: Option<Vec<Option<i64>>>,
	/// Data source (ID)
	pub data_source_id__n: Option<Vec<Option<i64>>>,
	pub data_synced: Option<Vec<String>>,
	pub data_synced__empty: Option<bool>,
	pub data_synced__gt: Option<Vec<String>>,
	pub data_synced__gte: Option<Vec<String>>,
	pub data_synced__lt: Option<Vec<String>>,
	pub data_synced__lte: Option<Vec<String>>,
	pub data_synced__n: Option<Vec<String>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,

}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesListResponse {
	Http200(PaginatedConfigTemplateList),
	Other(Response)
}
/// Get a list of config template objects.
pub fn extras_config_templates_list(state: &ThanixClient, query: ExtrasConfigTemplatesListQuery) -> Result<ExtrasConfigTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/config-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesListResponse::Http200(r#response.json::<PaginatedConfigTemplateList>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesBulkUpdateResponse {
	Http200(Vec<ConfigTemplate>),
	Other(Response)
}
/// Put a list of config template objects.
pub fn extras_config_templates_bulk_update(state: &ThanixClient, body: Vec<ConfigTemplateRequest>) -> Result<ExtrasConfigTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/config-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<ConfigTemplate>>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesCreateResponse {
	Http201(ConfigTemplate),
	Other(Response)
}
/// Post a list of config template objects.
pub fn extras_config_templates_create(state: &ThanixClient, body: WritableConfigTemplateRequest) -> Result<ExtrasConfigTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/config-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasConfigTemplatesCreateResponse::Http201(r#response.json::<ConfigTemplate>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of config template objects.
pub fn extras_config_templates_bulk_destroy(state: &ThanixClient, body: Vec<ConfigTemplateRequest>) -> Result<ExtrasConfigTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/config-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasConfigTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesBulkPartialUpdateResponse {
	Http200(Vec<ConfigTemplate>),
	Other(Response)
}
/// Patch a list of config template objects.
pub fn extras_config_templates_bulk_partial_update(state: &ThanixClient, body: Vec<ConfigTemplateRequest>) -> Result<ExtrasConfigTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/config-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ConfigTemplate>>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesRetrieveResponse {
	Http200(ConfigTemplate),
	Other(Response)
}
/// Get a config template object.
pub fn extras_config_templates_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasConfigTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/config-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesRetrieveResponse::Http200(r#response.json::<ConfigTemplate>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesUpdateResponse {
	Http200(ConfigTemplate),
	Other(Response)
}
/// Put a config template object.
pub fn extras_config_templates_update(state: &ThanixClient, body: WritableConfigTemplateRequest, id: i64) -> Result<ExtrasConfigTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/config-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesUpdateResponse::Http200(r#response.json::<ConfigTemplate>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a config template object.
pub fn extras_config_templates_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasConfigTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/config-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasConfigTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesPartialUpdateResponse {
	Http200(ConfigTemplate),
	Other(Response)
}
/// Patch a config template object.
pub fn extras_config_templates_partial_update(state: &ThanixClient, body: PatchedWritableConfigTemplateRequest, id: i64) -> Result<ExtrasConfigTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/config-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesPartialUpdateResponse::Http200(r#response.json::<ConfigTemplate>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasConfigTemplatesRenderCreateQuery {
	pub format: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesRenderCreateResponse {
	Http200(ConfigTemplate),
	Other(Response)
}
/// Render a ConfigTemplate using the context data provided (if any). If the client requests "text/plain" data,
/// return the raw rendered content, rather than serialized JSON.
pub fn extras_config_templates_render_create(state: &ThanixClient, query: ExtrasConfigTemplatesRenderCreateQuery, body: WritableConfigTemplateRequest, id: i64) -> Result<ExtrasConfigTemplatesRenderCreateResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.post(format!("{}/api/extras/config-templates/{id}/render/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesRenderCreateResponse::Http200(r#response.json::<ConfigTemplate>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesRenderCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasConfigTemplatesSyncCreateResponse {
	Http200(ConfigTemplate),
	Other(Response)
}
/// Provide a /sync API endpoint to synchronize an object's data from its associated DataFile (if any).
pub fn extras_config_templates_sync_create(state: &ThanixClient, body: WritableConfigTemplateRequest, id: i64) -> Result<ExtrasConfigTemplatesSyncCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/config-templates/{id}/sync/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasConfigTemplatesSyncCreateResponse::Http200(r#response.json::<ConfigTemplate>()?)) },
		r#other_status => { Ok(ExtrasConfigTemplatesSyncCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasContentTypesListQuery {
	pub app_label: Option<String>,
	pub id: Option<i64>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub model: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasContentTypesListResponse {
	Http200(PaginatedContentTypeList),
	Other(Response)
}
/// Read-only list of ContentTypes. Limit results to ContentTypes pertinent to NetBox objects.
pub fn extras_content_types_list(state: &ThanixClient, query: ExtrasContentTypesListQuery) -> Result<ExtrasContentTypesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/content-types/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasContentTypesListResponse::Http200(r#response.json::<PaginatedContentTypeList>()?)) },
		r#other_status => { Ok(ExtrasContentTypesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasContentTypesRetrieveResponse {
	Http200(ContentType),
	Other(Response)
}
/// Read-only list of ContentTypes. Limit results to ContentTypes pertinent to NetBox objects.
pub fn extras_content_types_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasContentTypesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/content-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasContentTypesRetrieveResponse::Http200(r#response.json::<ContentType>()?)) },
		r#other_status => { Ok(ExtrasContentTypesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasCustomFieldChoiceSetsListQuery {
	/// Base set of predefined choices (optional)
	pub base_choices: Option<String>,
	/// Base set of predefined choices (optional)
	pub base_choices__n: Option<String>,
	pub choice: Option<Vec<String>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	pub order_alphabetically: Option<bool>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsListResponse {
	Http200(PaginatedCustomFieldChoiceSetList),
	Other(Response)
}
/// Get a list of custom field choice set objects.
pub fn extras_custom_field_choice_sets_list(state: &ThanixClient, query: ExtrasCustomFieldChoiceSetsListQuery) -> Result<ExtrasCustomFieldChoiceSetsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/custom-field-choice-sets/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsListResponse::Http200(r#response.json::<PaginatedCustomFieldChoiceSetList>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsBulkUpdateResponse {
	Http200(Vec<CustomFieldChoiceSet>),
	Other(Response)
}
/// Put a list of custom field choice set objects.
pub fn extras_custom_field_choice_sets_bulk_update(state: &ThanixClient, body: Vec<CustomFieldChoiceSetRequest>) -> Result<ExtrasCustomFieldChoiceSetsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/custom-field-choice-sets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsBulkUpdateResponse::Http200(r#response.json::<Vec<CustomFieldChoiceSet>>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsCreateResponse {
	Http201(CustomFieldChoiceSet),
	Other(Response)
}
/// Post a list of custom field choice set objects.
pub fn extras_custom_field_choice_sets_create(state: &ThanixClient, body: WritableCustomFieldChoiceSetRequest) -> Result<ExtrasCustomFieldChoiceSetsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/custom-field-choice-sets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasCustomFieldChoiceSetsCreateResponse::Http201(r#response.json::<CustomFieldChoiceSet>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of custom field choice set objects.
pub fn extras_custom_field_choice_sets_bulk_destroy(state: &ThanixClient, body: Vec<CustomFieldChoiceSetRequest>) -> Result<ExtrasCustomFieldChoiceSetsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/custom-field-choice-sets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsBulkPartialUpdateResponse {
	Http200(Vec<CustomFieldChoiceSet>),
	Other(Response)
}
/// Patch a list of custom field choice set objects.
pub fn extras_custom_field_choice_sets_bulk_partial_update(state: &ThanixClient, body: Vec<CustomFieldChoiceSetRequest>) -> Result<ExtrasCustomFieldChoiceSetsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/custom-field-choice-sets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<CustomFieldChoiceSet>>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsRetrieveResponse {
	Http200(CustomFieldChoiceSet),
	Other(Response)
}
/// Get a custom field choice set object.
pub fn extras_custom_field_choice_sets_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasCustomFieldChoiceSetsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/custom-field-choice-sets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsRetrieveResponse::Http200(r#response.json::<CustomFieldChoiceSet>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsUpdateResponse {
	Http200(CustomFieldChoiceSet),
	Other(Response)
}
/// Put a custom field choice set object.
pub fn extras_custom_field_choice_sets_update(state: &ThanixClient, body: WritableCustomFieldChoiceSetRequest, id: i64) -> Result<ExtrasCustomFieldChoiceSetsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/custom-field-choice-sets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsUpdateResponse::Http200(r#response.json::<CustomFieldChoiceSet>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a custom field choice set object.
pub fn extras_custom_field_choice_sets_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasCustomFieldChoiceSetsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/custom-field-choice-sets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsPartialUpdateResponse {
	Http200(CustomFieldChoiceSet),
	Other(Response)
}
/// Patch a custom field choice set object.
pub fn extras_custom_field_choice_sets_partial_update(state: &ThanixClient, body: PatchedWritableCustomFieldChoiceSetRequest, id: i64) -> Result<ExtrasCustomFieldChoiceSetsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/custom-field-choice-sets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsPartialUpdateResponse::Http200(r#response.json::<CustomFieldChoiceSet>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldChoiceSetsChoicesRetrieveResponse {
	Http200(CustomFieldChoiceSet),
	Other(Response)
}
/// Provides an endpoint to iterate through each choice in a set.
pub fn extras_custom_field_choice_sets_choices_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasCustomFieldChoiceSetsChoicesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/custom-field-choice-sets/{id}/choices/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldChoiceSetsChoicesRetrieveResponse::Http200(r#response.json::<CustomFieldChoiceSet>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldChoiceSetsChoicesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasCustomFieldsListQuery {
	pub choice_set: Option<Vec<String>>,
	pub choice_set__n: Option<Vec<String>>,
	pub choice_set_id: Option<Vec<Option<i64>>>,
	pub choice_set_id__n: Option<Vec<Option<i64>>>,
	pub content_type_id: Option<Vec<i64>>,
	pub content_type_id__empty: Option<Vec<i64>>,
	pub content_type_id__gt: Option<Vec<i64>>,
	pub content_type_id__gte: Option<Vec<i64>>,
	pub content_type_id__lt: Option<Vec<i64>>,
	pub content_type_id__lte: Option<Vec<i64>>,
	pub content_type_id__n: Option<Vec<i64>>,
	pub content_types: Option<String>,
	pub content_types__ic: Option<String>,
	pub content_types__ie: Option<String>,
	pub content_types__iew: Option<String>,
	pub content_types__isw: Option<String>,
	pub content_types__n: Option<String>,
	pub content_types__nic: Option<String>,
	pub content_types__nie: Option<String>,
	pub content_types__niew: Option<String>,
	pub content_types__nisw: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Loose matches any instance of a given string; exact matches the entire field.
	pub filter_logic: Option<String>,
	/// Loose matches any instance of a given string; exact matches the entire field.
	pub filter_logic__n: Option<String>,
	pub group_name: Option<Vec<String>>,
	pub group_name__empty: Option<bool>,
	pub group_name__ic: Option<Vec<String>>,
	pub group_name__ie: Option<Vec<String>>,
	pub group_name__iew: Option<Vec<String>>,
	pub group_name__isw: Option<Vec<String>>,
	pub group_name__n: Option<Vec<String>>,
	pub group_name__nic: Option<Vec<String>>,
	pub group_name__nie: Option<Vec<String>>,
	pub group_name__niew: Option<Vec<String>>,
	pub group_name__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub is_cloneable: Option<bool>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub required: Option<bool>,
	pub search_weight: Option<Vec<i64>>,
	pub search_weight__empty: Option<bool>,
	pub search_weight__gt: Option<Vec<i64>>,
	pub search_weight__gte: Option<Vec<i64>>,
	pub search_weight__lt: Option<Vec<i64>>,
	pub search_weight__lte: Option<Vec<i64>>,
	pub search_weight__n: Option<Vec<i64>>,
	/// The type of data this custom field holds
	pub r#type: Option<Vec<String>>,
	/// The type of data this custom field holds
	pub type__n: Option<Vec<String>>,
	/// Specifies the visibility of custom field in the UI
	pub ui_visibility: Option<String>,
	/// Specifies the visibility of custom field in the UI
	pub ui_visibility__n: Option<String>,
	pub weight: Option<Vec<i64>>,
	pub weight__empty: Option<bool>,
	pub weight__gt: Option<Vec<i64>>,
	pub weight__gte: Option<Vec<i64>>,
	pub weight__lt: Option<Vec<i64>>,
	pub weight__lte: Option<Vec<i64>>,
	pub weight__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum ExtrasCustomFieldsListResponse {
	Http200(PaginatedCustomFieldList),
	Other(Response)
}
/// Get a list of custom field objects.
pub fn extras_custom_fields_list(state: &ThanixClient, query: ExtrasCustomFieldsListQuery) -> Result<ExtrasCustomFieldsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/custom-fields/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldsListResponse::Http200(r#response.json::<PaginatedCustomFieldList>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsBulkUpdateResponse {
	Http200(Vec<CustomField>),
	Other(Response)
}
/// Put a list of custom field objects.
pub fn extras_custom_fields_bulk_update(state: &ThanixClient, body: Vec<CustomFieldRequest>) -> Result<ExtrasCustomFieldsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/custom-fields/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldsBulkUpdateResponse::Http200(r#response.json::<Vec<CustomField>>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsCreateResponse {
	Http201(CustomField),
	Other(Response)
}
/// Post a list of custom field objects.
pub fn extras_custom_fields_create(state: &ThanixClient, body: WritableCustomFieldRequest) -> Result<ExtrasCustomFieldsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/custom-fields/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasCustomFieldsCreateResponse::Http201(r#response.json::<CustomField>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of custom field objects.
pub fn extras_custom_fields_bulk_destroy(state: &ThanixClient, body: Vec<CustomFieldRequest>) -> Result<ExtrasCustomFieldsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/custom-fields/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasCustomFieldsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsBulkPartialUpdateResponse {
	Http200(Vec<CustomField>),
	Other(Response)
}
/// Patch a list of custom field objects.
pub fn extras_custom_fields_bulk_partial_update(state: &ThanixClient, body: Vec<CustomFieldRequest>) -> Result<ExtrasCustomFieldsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/custom-fields/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<CustomField>>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsRetrieveResponse {
	Http200(CustomField),
	Other(Response)
}
/// Get a custom field object.
pub fn extras_custom_fields_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasCustomFieldsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/custom-fields/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldsRetrieveResponse::Http200(r#response.json::<CustomField>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsUpdateResponse {
	Http200(CustomField),
	Other(Response)
}
/// Put a custom field object.
pub fn extras_custom_fields_update(state: &ThanixClient, body: WritableCustomFieldRequest, id: i64) -> Result<ExtrasCustomFieldsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/custom-fields/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldsUpdateResponse::Http200(r#response.json::<CustomField>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a custom field object.
pub fn extras_custom_fields_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasCustomFieldsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/custom-fields/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasCustomFieldsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomFieldsPartialUpdateResponse {
	Http200(CustomField),
	Other(Response)
}
/// Patch a custom field object.
pub fn extras_custom_fields_partial_update(state: &ThanixClient, body: PatchedWritableCustomFieldRequest, id: i64) -> Result<ExtrasCustomFieldsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/custom-fields/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomFieldsPartialUpdateResponse::Http200(r#response.json::<CustomField>()?)) },
		r#other_status => { Ok(ExtrasCustomFieldsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasCustomLinksListQuery {
	pub content_type_id: Option<Vec<i64>>,
	pub content_type_id__empty: Option<Vec<i64>>,
	pub content_type_id__gt: Option<Vec<i64>>,
	pub content_type_id__gte: Option<Vec<i64>>,
	pub content_type_id__lt: Option<Vec<i64>>,
	pub content_type_id__lte: Option<Vec<i64>>,
	pub content_type_id__n: Option<Vec<i64>>,
	pub content_types: Option<String>,
	pub content_types__ic: Option<String>,
	pub content_types__ie: Option<String>,
	pub content_types__iew: Option<String>,
	pub content_types__isw: Option<String>,
	pub content_types__n: Option<String>,
	pub content_types__nic: Option<String>,
	pub content_types__nie: Option<String>,
	pub content_types__niew: Option<String>,
	pub content_types__nisw: Option<String>,
	pub enabled: Option<bool>,
	pub group_name: Option<Vec<String>>,
	pub group_name__empty: Option<bool>,
	pub group_name__ic: Option<Vec<String>>,
	pub group_name__ie: Option<Vec<String>>,
	pub group_name__iew: Option<Vec<String>>,
	pub group_name__isw: Option<Vec<String>>,
	pub group_name__n: Option<Vec<String>>,
	pub group_name__nic: Option<Vec<String>>,
	pub group_name__nie: Option<Vec<String>>,
	pub group_name__niew: Option<Vec<String>>,
	pub group_name__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub link_text: Option<String>,
	pub link_text__ic: Option<String>,
	pub link_text__ie: Option<String>,
	pub link_text__iew: Option<String>,
	pub link_text__isw: Option<String>,
	pub link_text__n: Option<String>,
	pub link_text__nic: Option<String>,
	pub link_text__nie: Option<String>,
	pub link_text__niew: Option<String>,
	pub link_text__nisw: Option<String>,
	pub link_url: Option<String>,
	pub link_url__ic: Option<String>,
	pub link_url__ie: Option<String>,
	pub link_url__iew: Option<String>,
	pub link_url__isw: Option<String>,
	pub link_url__n: Option<String>,
	pub link_url__nic: Option<String>,
	pub link_url__nie: Option<String>,
	pub link_url__niew: Option<String>,
	pub link_url__nisw: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub new_window: Option<bool>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub weight: Option<Vec<i64>>,
	pub weight__empty: Option<bool>,
	pub weight__gt: Option<Vec<i64>>,
	pub weight__gte: Option<Vec<i64>>,
	pub weight__lt: Option<Vec<i64>>,
	pub weight__lte: Option<Vec<i64>>,
	pub weight__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum ExtrasCustomLinksListResponse {
	Http200(PaginatedCustomLinkList),
	Other(Response)
}
/// Get a list of custom link objects.
pub fn extras_custom_links_list(state: &ThanixClient, query: ExtrasCustomLinksListQuery) -> Result<ExtrasCustomLinksListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/custom-links/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomLinksListResponse::Http200(r#response.json::<PaginatedCustomLinkList>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksBulkUpdateResponse {
	Http200(Vec<CustomLink>),
	Other(Response)
}
/// Put a list of custom link objects.
pub fn extras_custom_links_bulk_update(state: &ThanixClient, body: Vec<CustomLinkRequest>) -> Result<ExtrasCustomLinksBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/custom-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomLinksBulkUpdateResponse::Http200(r#response.json::<Vec<CustomLink>>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksCreateResponse {
	Http201(CustomLink),
	Other(Response)
}
/// Post a list of custom link objects.
pub fn extras_custom_links_create(state: &ThanixClient, body: CustomLinkRequest) -> Result<ExtrasCustomLinksCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/custom-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasCustomLinksCreateResponse::Http201(r#response.json::<CustomLink>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of custom link objects.
pub fn extras_custom_links_bulk_destroy(state: &ThanixClient, body: Vec<CustomLinkRequest>) -> Result<ExtrasCustomLinksBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/custom-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasCustomLinksBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksBulkPartialUpdateResponse {
	Http200(Vec<CustomLink>),
	Other(Response)
}
/// Patch a list of custom link objects.
pub fn extras_custom_links_bulk_partial_update(state: &ThanixClient, body: Vec<CustomLinkRequest>) -> Result<ExtrasCustomLinksBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/custom-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomLinksBulkPartialUpdateResponse::Http200(r#response.json::<Vec<CustomLink>>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksRetrieveResponse {
	Http200(CustomLink),
	Other(Response)
}
/// Get a custom link object.
pub fn extras_custom_links_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasCustomLinksRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/custom-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomLinksRetrieveResponse::Http200(r#response.json::<CustomLink>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksUpdateResponse {
	Http200(CustomLink),
	Other(Response)
}
/// Put a custom link object.
pub fn extras_custom_links_update(state: &ThanixClient, body: CustomLinkRequest, id: i64) -> Result<ExtrasCustomLinksUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/custom-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomLinksUpdateResponse::Http200(r#response.json::<CustomLink>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a custom link object.
pub fn extras_custom_links_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasCustomLinksDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/custom-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasCustomLinksDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasCustomLinksPartialUpdateResponse {
	Http200(CustomLink),
	Other(Response)
}
/// Patch a custom link object.
pub fn extras_custom_links_partial_update(state: &ThanixClient, body: PatchedCustomLinkRequest, id: i64) -> Result<ExtrasCustomLinksPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/custom-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasCustomLinksPartialUpdateResponse::Http200(r#response.json::<CustomLink>()?)) },
		r#other_status => { Ok(ExtrasCustomLinksPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasDashboardRetrieveResponse {
	Http200(Dashboard),
	Other(Response)
}
/// Get a list of dashboard objects.
pub fn extras_dashboard_retrieve(state: &ThanixClient) -> Result<ExtrasDashboardRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/dashboard/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasDashboardRetrieveResponse::Http200(r#response.json::<Dashboard>()?)) },
		r#other_status => { Ok(ExtrasDashboardRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasDashboardUpdateResponse {
	Http200(Dashboard),
	Other(Response)
}
/// Put a list of dashboard objects.
pub fn extras_dashboard_update(state: &ThanixClient, body: DashboardRequest) -> Result<ExtrasDashboardUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/dashboard/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasDashboardUpdateResponse::Http200(r#response.json::<Dashboard>()?)) },
		r#other_status => { Ok(ExtrasDashboardUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasDashboardDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of dashboard objects.
pub fn extras_dashboard_destroy(state: &ThanixClient) -> Result<ExtrasDashboardDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/dashboard/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasDashboardDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasDashboardPartialUpdateResponse {
	Http200(Dashboard),
	Other(Response)
}
/// Patch a list of dashboard objects.
pub fn extras_dashboard_partial_update(state: &ThanixClient, body: PatchedDashboardRequest) -> Result<ExtrasDashboardPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/dashboard/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasDashboardPartialUpdateResponse::Http200(r#response.json::<Dashboard>()?)) },
		r#other_status => { Ok(ExtrasDashboardPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasExportTemplatesListQuery {
	pub content_type_id: Option<Vec<i64>>,
	pub content_type_id__empty: Option<Vec<i64>>,
	pub content_type_id__gt: Option<Vec<i64>>,
	pub content_type_id__gte: Option<Vec<i64>>,
	pub content_type_id__lt: Option<Vec<i64>>,
	pub content_type_id__lte: Option<Vec<i64>>,
	pub content_type_id__n: Option<Vec<i64>>,
	pub content_types: Option<String>,
	pub content_types__ic: Option<String>,
	pub content_types__ie: Option<String>,
	pub content_types__iew: Option<String>,
	pub content_types__isw: Option<String>,
	pub content_types__n: Option<String>,
	pub content_types__nic: Option<String>,
	pub content_types__nie: Option<String>,
	pub content_types__niew: Option<String>,
	pub content_types__nisw: Option<String>,
	/// Data file (ID)
	pub data_file_id: Option<Vec<Option<i64>>>,
	/// Data file (ID)
	pub data_file_id__n: Option<Vec<Option<i64>>>,
	/// Data source (ID)
	pub data_source_id: Option<Vec<Option<i64>>>,
	/// Data source (ID)
	pub data_source_id__n: Option<Vec<Option<i64>>>,
	pub data_synced: Option<Vec<String>>,
	pub data_synced__empty: Option<bool>,
	pub data_synced__gt: Option<Vec<String>>,
	pub data_synced__gte: Option<Vec<String>>,
	pub data_synced__lt: Option<Vec<String>>,
	pub data_synced__lte: Option<Vec<String>>,
	pub data_synced__n: Option<Vec<String>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasExportTemplatesListResponse {
	Http200(PaginatedExportTemplateList),
	Other(Response)
}
/// Get a list of export template objects.
pub fn extras_export_templates_list(state: &ThanixClient, query: ExtrasExportTemplatesListQuery) -> Result<ExtrasExportTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/export-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesListResponse::Http200(r#response.json::<PaginatedExportTemplateList>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesBulkUpdateResponse {
	Http200(Vec<ExportTemplate>),
	Other(Response)
}
/// Put a list of export template objects.
pub fn extras_export_templates_bulk_update(state: &ThanixClient, body: Vec<ExportTemplateRequest>) -> Result<ExtrasExportTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/export-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<ExportTemplate>>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesCreateResponse {
	Http201(ExportTemplate),
	Other(Response)
}
/// Post a list of export template objects.
pub fn extras_export_templates_create(state: &ThanixClient, body: WritableExportTemplateRequest) -> Result<ExtrasExportTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/export-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasExportTemplatesCreateResponse::Http201(r#response.json::<ExportTemplate>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of export template objects.
pub fn extras_export_templates_bulk_destroy(state: &ThanixClient, body: Vec<ExportTemplateRequest>) -> Result<ExtrasExportTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/export-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasExportTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesBulkPartialUpdateResponse {
	Http200(Vec<ExportTemplate>),
	Other(Response)
}
/// Patch a list of export template objects.
pub fn extras_export_templates_bulk_partial_update(state: &ThanixClient, body: Vec<ExportTemplateRequest>) -> Result<ExtrasExportTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/export-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ExportTemplate>>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesRetrieveResponse {
	Http200(ExportTemplate),
	Other(Response)
}
/// Get a export template object.
pub fn extras_export_templates_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasExportTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/export-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesRetrieveResponse::Http200(r#response.json::<ExportTemplate>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesUpdateResponse {
	Http200(ExportTemplate),
	Other(Response)
}
/// Put a export template object.
pub fn extras_export_templates_update(state: &ThanixClient, body: WritableExportTemplateRequest, id: i64) -> Result<ExtrasExportTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/export-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesUpdateResponse::Http200(r#response.json::<ExportTemplate>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a export template object.
pub fn extras_export_templates_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasExportTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/export-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasExportTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesPartialUpdateResponse {
	Http200(ExportTemplate),
	Other(Response)
}
/// Patch a export template object.
pub fn extras_export_templates_partial_update(state: &ThanixClient, body: PatchedWritableExportTemplateRequest, id: i64) -> Result<ExtrasExportTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/export-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesPartialUpdateResponse::Http200(r#response.json::<ExportTemplate>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasExportTemplatesSyncCreateResponse {
	Http200(ExportTemplate),
	Other(Response)
}
/// Provide a /sync API endpoint to synchronize an object's data from its associated DataFile (if any).
pub fn extras_export_templates_sync_create(state: &ThanixClient, body: WritableExportTemplateRequest, id: i64) -> Result<ExtrasExportTemplatesSyncCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/export-templates/{id}/sync/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasExportTemplatesSyncCreateResponse::Http200(r#response.json::<ExportTemplate>()?)) },
		r#other_status => { Ok(ExtrasExportTemplatesSyncCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasImageAttachmentsListQuery {
	pub content_type: Option<String>,
	pub content_type__n: Option<String>,
	pub content_type_id: Option<i64>,
	pub content_type_id__n: Option<i64>,
	pub created: Option<String>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub object_id: Option<Vec<i64>>,
	pub object_id__empty: Option<bool>,
	pub object_id__gt: Option<Vec<i64>>,
	pub object_id__gte: Option<Vec<i64>>,
	pub object_id__lt: Option<Vec<i64>>,
	pub object_id__lte: Option<Vec<i64>>,
	pub object_id__n: Option<Vec<i64>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsListResponse {
	Http200(PaginatedImageAttachmentList),
	Other(Response)
}
/// Get a list of image attachment objects.
pub fn extras_image_attachments_list(state: &ThanixClient, query: ExtrasImageAttachmentsListQuery) -> Result<ExtrasImageAttachmentsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/image-attachments/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasImageAttachmentsListResponse::Http200(r#response.json::<PaginatedImageAttachmentList>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsBulkUpdateResponse {
	Http200(Vec<ImageAttachment>),
	Other(Response)
}
/// Put a list of image attachment objects.
pub fn extras_image_attachments_bulk_update(state: &ThanixClient, body: Vec<ImageAttachmentRequest>) -> Result<ExtrasImageAttachmentsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/image-attachments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasImageAttachmentsBulkUpdateResponse::Http200(r#response.json::<Vec<ImageAttachment>>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsCreateResponse {
	Http201(ImageAttachment),
	Other(Response)
}
/// Post a list of image attachment objects.
pub fn extras_image_attachments_create(state: &ThanixClient, body: ImageAttachmentRequest) -> Result<ExtrasImageAttachmentsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/image-attachments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasImageAttachmentsCreateResponse::Http201(r#response.json::<ImageAttachment>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of image attachment objects.
pub fn extras_image_attachments_bulk_destroy(state: &ThanixClient, body: Vec<ImageAttachmentRequest>) -> Result<ExtrasImageAttachmentsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/image-attachments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasImageAttachmentsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsBulkPartialUpdateResponse {
	Http200(Vec<ImageAttachment>),
	Other(Response)
}
/// Patch a list of image attachment objects.
pub fn extras_image_attachments_bulk_partial_update(state: &ThanixClient, body: Vec<ImageAttachmentRequest>) -> Result<ExtrasImageAttachmentsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/image-attachments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasImageAttachmentsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ImageAttachment>>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsRetrieveResponse {
	Http200(ImageAttachment),
	Other(Response)
}
/// Get a image attachment object.
pub fn extras_image_attachments_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasImageAttachmentsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/image-attachments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasImageAttachmentsRetrieveResponse::Http200(r#response.json::<ImageAttachment>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsUpdateResponse {
	Http200(ImageAttachment),
	Other(Response)
}
/// Put a image attachment object.
pub fn extras_image_attachments_update(state: &ThanixClient, body: ImageAttachmentRequest, id: i64) -> Result<ExtrasImageAttachmentsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/image-attachments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasImageAttachmentsUpdateResponse::Http200(r#response.json::<ImageAttachment>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a image attachment object.
pub fn extras_image_attachments_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasImageAttachmentsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/image-attachments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasImageAttachmentsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasImageAttachmentsPartialUpdateResponse {
	Http200(ImageAttachment),
	Other(Response)
}
/// Patch a image attachment object.
pub fn extras_image_attachments_partial_update(state: &ThanixClient, body: PatchedImageAttachmentRequest, id: i64) -> Result<ExtrasImageAttachmentsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/image-attachments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasImageAttachmentsPartialUpdateResponse::Http200(r#response.json::<ImageAttachment>()?)) },
		r#other_status => { Ok(ExtrasImageAttachmentsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasJournalEntriesListQuery {
	pub assigned_object_id: Option<Vec<i64>>,
	pub assigned_object_id__empty: Option<bool>,
	pub assigned_object_id__gt: Option<Vec<i64>>,
	pub assigned_object_id__gte: Option<Vec<i64>>,
	pub assigned_object_id__lt: Option<Vec<i64>>,
	pub assigned_object_id__lte: Option<Vec<i64>>,
	pub assigned_object_id__n: Option<Vec<i64>>,
	pub assigned_object_type: Option<String>,
	pub assigned_object_type__n: Option<String>,
	pub assigned_object_type_id: Option<Vec<i64>>,
	pub assigned_object_type_id__n: Option<Vec<i64>>,
	pub created_after: Option<String>,
	pub created_before: Option<String>,
	/// User (name)
	pub created_by: Option<Vec<String>>,
	/// User (name)
	pub created_by__n: Option<Vec<String>>,
	/// User (ID)
	pub created_by_id: Option<Vec<Option<i64>>>,
	/// User (ID)
	pub created_by_id__n: Option<Vec<Option<i64>>>,
	pub created_by_request: Option<String>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub kind: Option<Vec<String>>,
	pub kind__n: Option<Vec<String>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasJournalEntriesListResponse {
	Http200(PaginatedJournalEntryList),
	Other(Response)
}
/// Get a list of journal entry objects.
pub fn extras_journal_entries_list(state: &ThanixClient, query: ExtrasJournalEntriesListQuery) -> Result<ExtrasJournalEntriesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/journal-entries/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasJournalEntriesListResponse::Http200(r#response.json::<PaginatedJournalEntryList>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesBulkUpdateResponse {
	Http200(Vec<JournalEntry>),
	Other(Response)
}
/// Put a list of journal entry objects.
pub fn extras_journal_entries_bulk_update(state: &ThanixClient, body: Vec<JournalEntryRequest>) -> Result<ExtrasJournalEntriesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/journal-entries/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasJournalEntriesBulkUpdateResponse::Http200(r#response.json::<Vec<JournalEntry>>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesCreateResponse {
	Http201(JournalEntry),
	Other(Response)
}
/// Post a list of journal entry objects.
pub fn extras_journal_entries_create(state: &ThanixClient, body: WritableJournalEntryRequest) -> Result<ExtrasJournalEntriesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/journal-entries/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasJournalEntriesCreateResponse::Http201(r#response.json::<JournalEntry>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of journal entry objects.
pub fn extras_journal_entries_bulk_destroy(state: &ThanixClient, body: Vec<JournalEntryRequest>) -> Result<ExtrasJournalEntriesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/journal-entries/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasJournalEntriesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesBulkPartialUpdateResponse {
	Http200(Vec<JournalEntry>),
	Other(Response)
}
/// Patch a list of journal entry objects.
pub fn extras_journal_entries_bulk_partial_update(state: &ThanixClient, body: Vec<JournalEntryRequest>) -> Result<ExtrasJournalEntriesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/journal-entries/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasJournalEntriesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<JournalEntry>>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesRetrieveResponse {
	Http200(JournalEntry),
	Other(Response)
}
/// Get a journal entry object.
pub fn extras_journal_entries_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasJournalEntriesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/journal-entries/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasJournalEntriesRetrieveResponse::Http200(r#response.json::<JournalEntry>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesUpdateResponse {
	Http200(JournalEntry),
	Other(Response)
}
/// Put a journal entry object.
pub fn extras_journal_entries_update(state: &ThanixClient, body: WritableJournalEntryRequest, id: i64) -> Result<ExtrasJournalEntriesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/journal-entries/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasJournalEntriesUpdateResponse::Http200(r#response.json::<JournalEntry>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a journal entry object.
pub fn extras_journal_entries_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasJournalEntriesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/journal-entries/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasJournalEntriesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasJournalEntriesPartialUpdateResponse {
	Http200(JournalEntry),
	Other(Response)
}
/// Patch a journal entry object.
pub fn extras_journal_entries_partial_update(state: &ThanixClient, body: PatchedWritableJournalEntryRequest, id: i64) -> Result<ExtrasJournalEntriesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/journal-entries/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasJournalEntriesPartialUpdateResponse::Http200(r#response.json::<JournalEntry>()?)) },
		r#other_status => { Ok(ExtrasJournalEntriesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasObjectChangesListQuery {
	pub action: Option<String>,
	pub action__n: Option<String>,
	pub changed_object_id: Option<Vec<i64>>,
	pub changed_object_id__empty: Option<bool>,
	pub changed_object_id__gt: Option<Vec<i64>>,
	pub changed_object_id__gte: Option<Vec<i64>>,
	pub changed_object_id__lt: Option<Vec<i64>>,
	pub changed_object_id__lte: Option<Vec<i64>>,
	pub changed_object_id__n: Option<Vec<i64>>,
	pub changed_object_type: Option<String>,
	pub changed_object_type__n: Option<String>,
	pub changed_object_type_id: Option<Vec<i64>>,
	pub changed_object_type_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub object_repr: Option<Vec<String>>,
	pub object_repr__empty: Option<bool>,
	pub object_repr__ic: Option<Vec<String>>,
	pub object_repr__ie: Option<Vec<String>>,
	pub object_repr__iew: Option<Vec<String>>,
	pub object_repr__isw: Option<Vec<String>>,
	pub object_repr__n: Option<Vec<String>>,
	pub object_repr__nic: Option<Vec<String>>,
	pub object_repr__nie: Option<Vec<String>>,
	pub object_repr__niew: Option<Vec<String>>,
	pub object_repr__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub request_id: Option<String>,
	pub time_after: Option<String>,
	pub time_before: Option<String>,
	/// User name
	pub user: Option<Vec<String>>,
	/// User name
	pub user__n: Option<Vec<String>>,
	/// User (ID)
	pub user_id: Option<Vec<Option<i64>>>,
	/// User (ID)
	pub user_id__n: Option<Vec<Option<i64>>>,
	pub user_name: Option<Vec<String>>,
	pub user_name__empty: Option<bool>,
	pub user_name__ic: Option<Vec<String>>,
	pub user_name__ie: Option<Vec<String>>,
	pub user_name__iew: Option<Vec<String>>,
	pub user_name__isw: Option<Vec<String>>,
	pub user_name__n: Option<Vec<String>>,
	pub user_name__nic: Option<Vec<String>>,
	pub user_name__nie: Option<Vec<String>>,
	pub user_name__niew: Option<Vec<String>>,
	pub user_name__nisw: Option<Vec<String>>,

}
#[derive(Debug)]
pub enum ExtrasObjectChangesListResponse {
	Http200(PaginatedObjectChangeList),
	Other(Response)
}
/// Retrieve a list of recent changes.
pub fn extras_object_changes_list(state: &ThanixClient, query: ExtrasObjectChangesListQuery) -> Result<ExtrasObjectChangesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/object-changes/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasObjectChangesListResponse::Http200(r#response.json::<PaginatedObjectChangeList>()?)) },
		r#other_status => { Ok(ExtrasObjectChangesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasObjectChangesRetrieveResponse {
	Http200(ObjectChange),
	Other(Response)
}
/// Retrieve a list of recent changes.
pub fn extras_object_changes_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasObjectChangesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/object-changes/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasObjectChangesRetrieveResponse::Http200(r#response.json::<ObjectChange>()?)) },
		r#other_status => { Ok(ExtrasObjectChangesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasSavedFiltersListQuery {
	pub content_type_id: Option<Vec<i64>>,
	pub content_type_id__empty: Option<Vec<i64>>,
	pub content_type_id__gt: Option<Vec<i64>>,
	pub content_type_id__gte: Option<Vec<i64>>,
	pub content_type_id__lt: Option<Vec<i64>>,
	pub content_type_id__lte: Option<Vec<i64>>,
	pub content_type_id__n: Option<Vec<i64>>,
	pub content_types: Option<String>,
	pub content_types__ic: Option<String>,
	pub content_types__ie: Option<String>,
	pub content_types__iew: Option<String>,
	pub content_types__isw: Option<String>,
	pub content_types__n: Option<String>,
	pub content_types__nic: Option<String>,
	pub content_types__nie: Option<String>,
	pub content_types__niew: Option<String>,
	pub content_types__nisw: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub enabled: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub shared: Option<bool>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub usable: Option<bool>,
	/// User (name)
	pub user: Option<Vec<String>>,
	/// User (name)
	pub user__n: Option<Vec<String>>,
	/// User (ID)
	pub user_id: Option<Vec<Option<i64>>>,
	/// User (ID)
	pub user_id__n: Option<Vec<Option<i64>>>,
	pub weight: Option<Vec<i64>>,
	pub weight__empty: Option<bool>,
	pub weight__gt: Option<Vec<i64>>,
	pub weight__gte: Option<Vec<i64>>,
	pub weight__lt: Option<Vec<i64>>,
	pub weight__lte: Option<Vec<i64>>,
	pub weight__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum ExtrasSavedFiltersListResponse {
	Http200(PaginatedSavedFilterList),
	Other(Response)
}
/// Get a list of saved filter objects.
pub fn extras_saved_filters_list(state: &ThanixClient, query: ExtrasSavedFiltersListQuery) -> Result<ExtrasSavedFiltersListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/saved-filters/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasSavedFiltersListResponse::Http200(r#response.json::<PaginatedSavedFilterList>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersBulkUpdateResponse {
	Http200(Vec<SavedFilter>),
	Other(Response)
}
/// Put a list of saved filter objects.
pub fn extras_saved_filters_bulk_update(state: &ThanixClient, body: Vec<SavedFilterRequest>) -> Result<ExtrasSavedFiltersBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/saved-filters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasSavedFiltersBulkUpdateResponse::Http200(r#response.json::<Vec<SavedFilter>>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersCreateResponse {
	Http201(SavedFilter),
	Other(Response)
}
/// Post a list of saved filter objects.
pub fn extras_saved_filters_create(state: &ThanixClient, body: SavedFilterRequest) -> Result<ExtrasSavedFiltersCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/saved-filters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasSavedFiltersCreateResponse::Http201(r#response.json::<SavedFilter>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of saved filter objects.
pub fn extras_saved_filters_bulk_destroy(state: &ThanixClient, body: Vec<SavedFilterRequest>) -> Result<ExtrasSavedFiltersBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/saved-filters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasSavedFiltersBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersBulkPartialUpdateResponse {
	Http200(Vec<SavedFilter>),
	Other(Response)
}
/// Patch a list of saved filter objects.
pub fn extras_saved_filters_bulk_partial_update(state: &ThanixClient, body: Vec<SavedFilterRequest>) -> Result<ExtrasSavedFiltersBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/saved-filters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasSavedFiltersBulkPartialUpdateResponse::Http200(r#response.json::<Vec<SavedFilter>>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersRetrieveResponse {
	Http200(SavedFilter),
	Other(Response)
}
/// Get a saved filter object.
pub fn extras_saved_filters_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasSavedFiltersRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/saved-filters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasSavedFiltersRetrieveResponse::Http200(r#response.json::<SavedFilter>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersUpdateResponse {
	Http200(SavedFilter),
	Other(Response)
}
/// Put a saved filter object.
pub fn extras_saved_filters_update(state: &ThanixClient, body: SavedFilterRequest, id: i64) -> Result<ExtrasSavedFiltersUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/saved-filters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasSavedFiltersUpdateResponse::Http200(r#response.json::<SavedFilter>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a saved filter object.
pub fn extras_saved_filters_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasSavedFiltersDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/saved-filters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasSavedFiltersDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasSavedFiltersPartialUpdateResponse {
	Http200(SavedFilter),
	Other(Response)
}
/// Patch a saved filter object.
pub fn extras_saved_filters_partial_update(state: &ThanixClient, body: PatchedSavedFilterRequest, id: i64) -> Result<ExtrasSavedFiltersPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/saved-filters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasSavedFiltersPartialUpdateResponse::Http200(r#response.json::<SavedFilter>()?)) },
		r#other_status => { Ok(ExtrasSavedFiltersPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasTagsListQuery {
	pub color: Option<Vec<String>>,
	pub color__empty: Option<bool>,
	pub color__ic: Option<Vec<String>>,
	pub color__ie: Option<Vec<String>>,
	pub color__iew: Option<Vec<String>>,
	pub color__isw: Option<Vec<String>>,
	pub color__n: Option<Vec<String>>,
	pub color__nic: Option<Vec<String>>,
	pub color__nie: Option<Vec<String>>,
	pub color__niew: Option<Vec<String>>,
	pub color__nisw: Option<Vec<String>>,
	pub content_type: Option<Vec<String>>,
	pub content_type_id: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub for_object_type_id: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub object_types: Option<Vec<i64>>,
	pub object_types__n: Option<Vec<i64>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasTagsListResponse {
	Http200(PaginatedTagList),
	Other(Response)
}
/// Get a list of tag objects.
pub fn extras_tags_list(state: &ThanixClient, query: ExtrasTagsListQuery) -> Result<ExtrasTagsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/tags/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasTagsListResponse::Http200(r#response.json::<PaginatedTagList>()?)) },
		r#other_status => { Ok(ExtrasTagsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsBulkUpdateResponse {
	Http200(Vec<Tag>),
	Other(Response)
}
/// Put a list of tag objects.
pub fn extras_tags_bulk_update(state: &ThanixClient, body: Vec<TagRequest>) -> Result<ExtrasTagsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/tags/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasTagsBulkUpdateResponse::Http200(r#response.json::<Vec<Tag>>()?)) },
		r#other_status => { Ok(ExtrasTagsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsCreateResponse {
	Http201(Tag),
	Other(Response)
}
/// Post a list of tag objects.
pub fn extras_tags_create(state: &ThanixClient, body: TagRequest) -> Result<ExtrasTagsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/tags/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasTagsCreateResponse::Http201(r#response.json::<Tag>()?)) },
		r#other_status => { Ok(ExtrasTagsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of tag objects.
pub fn extras_tags_bulk_destroy(state: &ThanixClient, body: Vec<TagRequest>) -> Result<ExtrasTagsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/tags/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasTagsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsBulkPartialUpdateResponse {
	Http200(Vec<Tag>),
	Other(Response)
}
/// Patch a list of tag objects.
pub fn extras_tags_bulk_partial_update(state: &ThanixClient, body: Vec<TagRequest>) -> Result<ExtrasTagsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/tags/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasTagsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Tag>>()?)) },
		r#other_status => { Ok(ExtrasTagsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsRetrieveResponse {
	Http200(Tag),
	Other(Response)
}
/// Get a tag object.
pub fn extras_tags_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasTagsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/tags/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasTagsRetrieveResponse::Http200(r#response.json::<Tag>()?)) },
		r#other_status => { Ok(ExtrasTagsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsUpdateResponse {
	Http200(Tag),
	Other(Response)
}
/// Put a tag object.
pub fn extras_tags_update(state: &ThanixClient, body: TagRequest, id: i64) -> Result<ExtrasTagsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/tags/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasTagsUpdateResponse::Http200(r#response.json::<Tag>()?)) },
		r#other_status => { Ok(ExtrasTagsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a tag object.
pub fn extras_tags_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasTagsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/tags/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasTagsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasTagsPartialUpdateResponse {
	Http200(Tag),
	Other(Response)
}
/// Patch a tag object.
pub fn extras_tags_partial_update(state: &ThanixClient, body: PatchedTagRequest, id: i64) -> Result<ExtrasTagsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/tags/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasTagsPartialUpdateResponse::Http200(r#response.json::<Tag>()?)) },
		r#other_status => { Ok(ExtrasTagsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtrasWebhooksListQuery {
	pub ca_file_path: Option<Vec<String>>,
	pub ca_file_path__empty: Option<bool>,
	pub ca_file_path__ic: Option<Vec<String>>,
	pub ca_file_path__ie: Option<Vec<String>>,
	pub ca_file_path__iew: Option<Vec<String>>,
	pub ca_file_path__isw: Option<Vec<String>>,
	pub ca_file_path__n: Option<Vec<String>>,
	pub ca_file_path__nic: Option<Vec<String>>,
	pub ca_file_path__nie: Option<Vec<String>>,
	pub ca_file_path__niew: Option<Vec<String>>,
	pub ca_file_path__nisw: Option<Vec<String>>,
	pub content_type_id: Option<Vec<i64>>,
	pub content_type_id__empty: Option<Vec<i64>>,
	pub content_type_id__gt: Option<Vec<i64>>,
	pub content_type_id__gte: Option<Vec<i64>>,
	pub content_type_id__lt: Option<Vec<i64>>,
	pub content_type_id__lte: Option<Vec<i64>>,
	pub content_type_id__n: Option<Vec<i64>>,
	pub content_types: Option<String>,
	pub content_types__ic: Option<String>,
	pub content_types__ie: Option<String>,
	pub content_types__iew: Option<String>,
	pub content_types__isw: Option<String>,
	pub content_types__n: Option<String>,
	pub content_types__nic: Option<String>,
	pub content_types__nie: Option<String>,
	pub content_types__niew: Option<String>,
	pub content_types__nisw: Option<String>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub enabled: Option<bool>,
	pub http_content_type: Option<Vec<String>>,
	pub http_content_type__empty: Option<bool>,
	pub http_content_type__ic: Option<Vec<String>>,
	pub http_content_type__ie: Option<Vec<String>>,
	pub http_content_type__iew: Option<Vec<String>>,
	pub http_content_type__isw: Option<Vec<String>>,
	pub http_content_type__n: Option<Vec<String>>,
	pub http_content_type__nic: Option<Vec<String>>,
	pub http_content_type__nie: Option<Vec<String>>,
	pub http_content_type__niew: Option<Vec<String>>,
	pub http_content_type__nisw: Option<Vec<String>>,
	pub http_method: Option<Vec<String>>,
	pub http_method__n: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub payload_url: Option<Vec<String>>,
	pub payload_url__empty: Option<bool>,
	pub payload_url__ic: Option<Vec<String>>,
	pub payload_url__ie: Option<Vec<String>>,
	pub payload_url__iew: Option<Vec<String>>,
	pub payload_url__isw: Option<Vec<String>>,
	pub payload_url__n: Option<Vec<String>>,
	pub payload_url__nic: Option<Vec<String>>,
	pub payload_url__nie: Option<Vec<String>>,
	pub payload_url__niew: Option<Vec<String>>,
	pub payload_url__nisw: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	pub secret: Option<Vec<String>>,
	pub secret__empty: Option<bool>,
	pub secret__ic: Option<Vec<String>>,
	pub secret__ie: Option<Vec<String>>,
	pub secret__iew: Option<Vec<String>>,
	pub secret__isw: Option<Vec<String>>,
	pub secret__n: Option<Vec<String>>,
	pub secret__nic: Option<Vec<String>>,
	pub secret__nie: Option<Vec<String>>,
	pub secret__niew: Option<Vec<String>>,
	pub secret__nisw: Option<Vec<String>>,
	pub ssl_verification: Option<bool>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub type_create: Option<bool>,
	pub type_delete: Option<bool>,
	pub type_job_end: Option<bool>,
	pub type_job_start: Option<bool>,
	pub type_update: Option<bool>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum ExtrasWebhooksListResponse {
	Http200(PaginatedWebhookList),
	Other(Response)
}
/// Get a list of webhook objects.
pub fn extras_webhooks_list(state: &ThanixClient, query: ExtrasWebhooksListQuery) -> Result<ExtrasWebhooksListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/extras/webhooks/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasWebhooksListResponse::Http200(r#response.json::<PaginatedWebhookList>()?)) },
		r#other_status => { Ok(ExtrasWebhooksListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksBulkUpdateResponse {
	Http200(Vec<Webhook>),
	Other(Response)
}
/// Put a list of webhook objects.
pub fn extras_webhooks_bulk_update(state: &ThanixClient, body: Vec<WebhookRequest>) -> Result<ExtrasWebhooksBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/webhooks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasWebhooksBulkUpdateResponse::Http200(r#response.json::<Vec<Webhook>>()?)) },
		r#other_status => { Ok(ExtrasWebhooksBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksCreateResponse {
	Http201(Webhook),
	Other(Response)
}
/// Post a list of webhook objects.
pub fn extras_webhooks_create(state: &ThanixClient, body: WebhookRequest) -> Result<ExtrasWebhooksCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/extras/webhooks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(ExtrasWebhooksCreateResponse::Http201(r#response.json::<Webhook>()?)) },
		r#other_status => { Ok(ExtrasWebhooksCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of webhook objects.
pub fn extras_webhooks_bulk_destroy(state: &ThanixClient, body: Vec<WebhookRequest>) -> Result<ExtrasWebhooksBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/webhooks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasWebhooksBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksBulkPartialUpdateResponse {
	Http200(Vec<Webhook>),
	Other(Response)
}
/// Patch a list of webhook objects.
pub fn extras_webhooks_bulk_partial_update(state: &ThanixClient, body: Vec<WebhookRequest>) -> Result<ExtrasWebhooksBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/webhooks/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasWebhooksBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Webhook>>()?)) },
		r#other_status => { Ok(ExtrasWebhooksBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksRetrieveResponse {
	Http200(Webhook),
	Other(Response)
}
/// Get a webhook object.
pub fn extras_webhooks_retrieve(state: &ThanixClient, id: i64) -> Result<ExtrasWebhooksRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/extras/webhooks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasWebhooksRetrieveResponse::Http200(r#response.json::<Webhook>()?)) },
		r#other_status => { Ok(ExtrasWebhooksRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksUpdateResponse {
	Http200(Webhook),
	Other(Response)
}
/// Put a webhook object.
pub fn extras_webhooks_update(state: &ThanixClient, body: WebhookRequest, id: i64) -> Result<ExtrasWebhooksUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/extras/webhooks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasWebhooksUpdateResponse::Http200(r#response.json::<Webhook>()?)) },
		r#other_status => { Ok(ExtrasWebhooksUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a webhook object.
pub fn extras_webhooks_destroy(state: &ThanixClient, id: i64) -> Result<ExtrasWebhooksDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/extras/webhooks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(ExtrasWebhooksDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum ExtrasWebhooksPartialUpdateResponse {
	Http200(Webhook),
	Other(Response)
}
/// Patch a webhook object.
pub fn extras_webhooks_partial_update(state: &ThanixClient, body: PatchedWebhookRequest, id: i64) -> Result<ExtrasWebhooksPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/extras/webhooks/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(ExtrasWebhooksPartialUpdateResponse::Http200(r#response.json::<Webhook>()?)) },
		r#other_status => { Ok(ExtrasWebhooksPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamAggregatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub date_added: Option<Vec<String>>,
	pub date_added__empty: Option<bool>,
	pub date_added__gt: Option<Vec<String>>,
	pub date_added__gte: Option<Vec<String>>,
	pub date_added__lt: Option<Vec<String>>,
	pub date_added__lte: Option<Vec<String>>,
	pub date_added__n: Option<Vec<String>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub family: Option<f64>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Prefix
	pub prefix: Option<String>,
	/// Search
	pub q: Option<String>,
	/// RIR (slug)
	pub rir: Option<Vec<String>>,
	/// RIR (slug)
	pub rir__n: Option<Vec<String>>,
	/// RIR (ID)
	pub rir_id: Option<Vec<i64>>,
	/// RIR (ID)
	pub rir_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamAggregatesListResponse {
	Http200(PaginatedAggregateList),
	Other(Response)
}
/// Get a list of aggregate objects.
pub fn ipam_aggregates_list(state: &ThanixClient, query: IpamAggregatesListQuery) -> Result<IpamAggregatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/aggregates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAggregatesListResponse::Http200(r#response.json::<PaginatedAggregateList>()?)) },
		r#other_status => { Ok(IpamAggregatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesBulkUpdateResponse {
	Http200(Vec<Aggregate>),
	Other(Response)
}
/// Put a list of aggregate objects.
pub fn ipam_aggregates_bulk_update(state: &ThanixClient, body: Vec<AggregateRequest>) -> Result<IpamAggregatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/aggregates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAggregatesBulkUpdateResponse::Http200(r#response.json::<Vec<Aggregate>>()?)) },
		r#other_status => { Ok(IpamAggregatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesCreateResponse {
	Http201(Aggregate),
	Other(Response)
}
/// Post a list of aggregate objects.
pub fn ipam_aggregates_create(state: &ThanixClient, body: WritableAggregateRequest) -> Result<IpamAggregatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/aggregates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamAggregatesCreateResponse::Http201(r#response.json::<Aggregate>()?)) },
		r#other_status => { Ok(IpamAggregatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of aggregate objects.
pub fn ipam_aggregates_bulk_destroy(state: &ThanixClient, body: Vec<AggregateRequest>) -> Result<IpamAggregatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/aggregates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamAggregatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesBulkPartialUpdateResponse {
	Http200(Vec<Aggregate>),
	Other(Response)
}
/// Patch a list of aggregate objects.
pub fn ipam_aggregates_bulk_partial_update(state: &ThanixClient, body: Vec<AggregateRequest>) -> Result<IpamAggregatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/aggregates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAggregatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Aggregate>>()?)) },
		r#other_status => { Ok(IpamAggregatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesRetrieveResponse {
	Http200(Aggregate),
	Other(Response)
}
/// Get a aggregate object.
pub fn ipam_aggregates_retrieve(state: &ThanixClient, id: i64) -> Result<IpamAggregatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/aggregates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAggregatesRetrieveResponse::Http200(r#response.json::<Aggregate>()?)) },
		r#other_status => { Ok(IpamAggregatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesUpdateResponse {
	Http200(Aggregate),
	Other(Response)
}
/// Put a aggregate object.
pub fn ipam_aggregates_update(state: &ThanixClient, body: WritableAggregateRequest, id: i64) -> Result<IpamAggregatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/aggregates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAggregatesUpdateResponse::Http200(r#response.json::<Aggregate>()?)) },
		r#other_status => { Ok(IpamAggregatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a aggregate object.
pub fn ipam_aggregates_destroy(state: &ThanixClient, id: i64) -> Result<IpamAggregatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/aggregates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamAggregatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAggregatesPartialUpdateResponse {
	Http200(Aggregate),
	Other(Response)
}
/// Patch a aggregate object.
pub fn ipam_aggregates_partial_update(state: &ThanixClient, body: PatchedWritableAggregateRequest, id: i64) -> Result<IpamAggregatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/aggregates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAggregatesPartialUpdateResponse::Http200(r#response.json::<Aggregate>()?)) },
		r#other_status => { Ok(IpamAggregatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamAsnRangesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub end: Option<Vec<i64>>,
	pub end__empty: Option<bool>,
	pub end__gt: Option<Vec<i64>>,
	pub end__gte: Option<Vec<i64>>,
	pub end__lt: Option<Vec<i64>>,
	pub end__lte: Option<Vec<i64>>,
	pub end__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// RIR (slug)
	pub rir: Option<Vec<String>>,
	/// RIR (slug)
	pub rir__n: Option<Vec<String>>,
	/// RIR (ID)
	pub rir_id: Option<Vec<i64>>,
	/// RIR (ID)
	pub rir_id__n: Option<Vec<i64>>,
	pub start: Option<Vec<i64>>,
	pub start__empty: Option<bool>,
	pub start__gt: Option<Vec<i64>>,
	pub start__gte: Option<Vec<i64>>,
	pub start__lt: Option<Vec<i64>>,
	pub start__lte: Option<Vec<i64>>,
	pub start__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamAsnRangesListResponse {
	Http200(PaginatedASNRangeList),
	Other(Response)
}
/// Get a list of ASN range objects.
pub fn ipam_asn_ranges_list(state: &ThanixClient, query: IpamAsnRangesListQuery) -> Result<IpamAsnRangesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/asn-ranges/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesListResponse::Http200(r#response.json::<PaginatedASNRangeList>()?)) },
		r#other_status => { Ok(IpamAsnRangesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesBulkUpdateResponse {
	Http200(Vec<ASNRange>),
	Other(Response)
}
/// Put a list of ASN range objects.
pub fn ipam_asn_ranges_bulk_update(state: &ThanixClient, body: Vec<ASNRangeRequest>) -> Result<IpamAsnRangesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/asn-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesBulkUpdateResponse::Http200(r#response.json::<Vec<ASNRange>>()?)) },
		r#other_status => { Ok(IpamAsnRangesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesCreateResponse {
	Http201(ASNRange),
	Other(Response)
}
/// Post a list of ASN range objects.
pub fn ipam_asn_ranges_create(state: &ThanixClient, body: WritableASNRangeRequest) -> Result<IpamAsnRangesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/asn-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamAsnRangesCreateResponse::Http201(r#response.json::<ASNRange>()?)) },
		r#other_status => { Ok(IpamAsnRangesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of ASN range objects.
pub fn ipam_asn_ranges_bulk_destroy(state: &ThanixClient, body: Vec<ASNRangeRequest>) -> Result<IpamAsnRangesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/asn-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamAsnRangesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesBulkPartialUpdateResponse {
	Http200(Vec<ASNRange>),
	Other(Response)
}
/// Patch a list of ASN range objects.
pub fn ipam_asn_ranges_bulk_partial_update(state: &ThanixClient, body: Vec<ASNRangeRequest>) -> Result<IpamAsnRangesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/asn-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ASNRange>>()?)) },
		r#other_status => { Ok(IpamAsnRangesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesRetrieveResponse {
	Http200(ASNRange),
	Other(Response)
}
/// Get a ASN range object.
pub fn ipam_asn_ranges_retrieve(state: &ThanixClient, id: i64) -> Result<IpamAsnRangesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/asn-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesRetrieveResponse::Http200(r#response.json::<ASNRange>()?)) },
		r#other_status => { Ok(IpamAsnRangesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesUpdateResponse {
	Http200(ASNRange),
	Other(Response)
}
/// Put a ASN range object.
pub fn ipam_asn_ranges_update(state: &ThanixClient, body: WritableASNRangeRequest, id: i64) -> Result<IpamAsnRangesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/asn-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesUpdateResponse::Http200(r#response.json::<ASNRange>()?)) },
		r#other_status => { Ok(IpamAsnRangesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a ASN range object.
pub fn ipam_asn_ranges_destroy(state: &ThanixClient, id: i64) -> Result<IpamAsnRangesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/asn-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamAsnRangesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesPartialUpdateResponse {
	Http200(ASNRange),
	Other(Response)
}
/// Patch a ASN range object.
pub fn ipam_asn_ranges_partial_update(state: &ThanixClient, body: PatchedWritableASNRangeRequest, id: i64) -> Result<IpamAsnRangesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/asn-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesPartialUpdateResponse::Http200(r#response.json::<ASNRange>()?)) },
		r#other_status => { Ok(IpamAsnRangesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesAvailableAsnsListResponse {
	Http200(Vec<AvailableASN>),
	Other(Response)
}
/// Get a ASN object.
pub fn ipam_asn_ranges_available_asns_list(state: &ThanixClient, id: i64) -> Result<IpamAsnRangesAvailableAsnsListResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/asn-ranges/{id}/available-asns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnRangesAvailableAsnsListResponse::Http200(r#response.json::<Vec<AvailableASN>>()?)) },
		r#other_status => { Ok(IpamAsnRangesAvailableAsnsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnRangesAvailableAsnsCreateResponse {
	Http201(Vec<ASN>),
	Other(Response)
}
/// Post a ASN object.
pub fn ipam_asn_ranges_available_asns_create(state: &ThanixClient, body: Vec<ASNRequest>, id: i64) -> Result<IpamAsnRangesAvailableAsnsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/asn-ranges/{id}/available-asns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamAsnRangesAvailableAsnsCreateResponse::Http201(r#response.json::<Vec<ASN>>()?)) },
		r#other_status => { Ok(IpamAsnRangesAvailableAsnsCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamAsnsListQuery {
	pub asn: Option<Vec<i64>>,
	pub asn__empty: Option<bool>,
	pub asn__gt: Option<Vec<i64>>,
	pub asn__gte: Option<Vec<i64>>,
	pub asn__lt: Option<Vec<i64>>,
	pub asn__lte: Option<Vec<i64>>,
	pub asn__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// RIR (slug)
	pub rir: Option<Vec<String>>,
	/// RIR (slug)
	pub rir__n: Option<Vec<String>>,
	/// RIR (ID)
	pub rir_id: Option<Vec<i64>>,
	/// RIR (ID)
	pub rir_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site (ID)
	pub site_id: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamAsnsListResponse {
	Http200(PaginatedASNList),
	Other(Response)
}
/// Get a list of ASN objects.
pub fn ipam_asns_list(state: &ThanixClient, query: IpamAsnsListQuery) -> Result<IpamAsnsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/asns/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnsListResponse::Http200(r#response.json::<PaginatedASNList>()?)) },
		r#other_status => { Ok(IpamAsnsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsBulkUpdateResponse {
	Http200(Vec<ASN>),
	Other(Response)
}
/// Put a list of ASN objects.
pub fn ipam_asns_bulk_update(state: &ThanixClient, body: Vec<ASNRequest>) -> Result<IpamAsnsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/asns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnsBulkUpdateResponse::Http200(r#response.json::<Vec<ASN>>()?)) },
		r#other_status => { Ok(IpamAsnsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsCreateResponse {
	Http201(ASN),
	Other(Response)
}
/// Post a list of ASN objects.
pub fn ipam_asns_create(state: &ThanixClient, body: WritableASNRequest) -> Result<IpamAsnsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/asns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamAsnsCreateResponse::Http201(r#response.json::<ASN>()?)) },
		r#other_status => { Ok(IpamAsnsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of ASN objects.
pub fn ipam_asns_bulk_destroy(state: &ThanixClient, body: Vec<ASNRequest>) -> Result<IpamAsnsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/asns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamAsnsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsBulkPartialUpdateResponse {
	Http200(Vec<ASN>),
	Other(Response)
}
/// Patch a list of ASN objects.
pub fn ipam_asns_bulk_partial_update(state: &ThanixClient, body: Vec<ASNRequest>) -> Result<IpamAsnsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/asns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ASN>>()?)) },
		r#other_status => { Ok(IpamAsnsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsRetrieveResponse {
	Http200(ASN),
	Other(Response)
}
/// Get a ASN object.
pub fn ipam_asns_retrieve(state: &ThanixClient, id: i64) -> Result<IpamAsnsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/asns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnsRetrieveResponse::Http200(r#response.json::<ASN>()?)) },
		r#other_status => { Ok(IpamAsnsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsUpdateResponse {
	Http200(ASN),
	Other(Response)
}
/// Put a ASN object.
pub fn ipam_asns_update(state: &ThanixClient, body: WritableASNRequest, id: i64) -> Result<IpamAsnsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/asns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnsUpdateResponse::Http200(r#response.json::<ASN>()?)) },
		r#other_status => { Ok(IpamAsnsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a ASN object.
pub fn ipam_asns_destroy(state: &ThanixClient, id: i64) -> Result<IpamAsnsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/asns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamAsnsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamAsnsPartialUpdateResponse {
	Http200(ASN),
	Other(Response)
}
/// Patch a ASN object.
pub fn ipam_asns_partial_update(state: &ThanixClient, body: PatchedWritableASNRequest, id: i64) -> Result<IpamAsnsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/asns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamAsnsPartialUpdateResponse::Http200(r#response.json::<ASN>()?)) },
		r#other_status => { Ok(IpamAsnsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamFhrpGroupAssignmentsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub device: Option<Vec<String>>,
	pub device_id: Option<Vec<i64>>,
	/// Group (ID)
	pub group_id: Option<Vec<i64>>,
	/// Group (ID)
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub interface_id: Option<Vec<i64>>,
	pub interface_id__empty: Option<bool>,
	pub interface_id__gt: Option<Vec<i64>>,
	pub interface_id__gte: Option<Vec<i64>>,
	pub interface_id__lt: Option<Vec<i64>>,
	pub interface_id__lte: Option<Vec<i64>>,
	pub interface_id__n: Option<Vec<i64>>,
	pub interface_type: Option<String>,
	pub interface_type__n: Option<String>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub priority: Option<Vec<i64>>,
	pub priority__empty: Option<bool>,
	pub priority__gt: Option<Vec<i64>>,
	pub priority__gte: Option<Vec<i64>>,
	pub priority__lt: Option<Vec<i64>>,
	pub priority__lte: Option<Vec<i64>>,
	pub priority__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,
	pub virtual_machine: Option<Vec<String>>,
	pub virtual_machine_id: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsListResponse {
	Http200(PaginatedFHRPGroupAssignmentList),
	Other(Response)
}
/// Get a list of FHRP group assignment objects.
pub fn ipam_fhrp_group_assignments_list(state: &ThanixClient, query: IpamFhrpGroupAssignmentsListQuery) -> Result<IpamFhrpGroupAssignmentsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/fhrp-group-assignments/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupAssignmentsListResponse::Http200(r#response.json::<PaginatedFHRPGroupAssignmentList>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsBulkUpdateResponse {
	Http200(Vec<FHRPGroupAssignment>),
	Other(Response)
}
/// Put a list of FHRP group assignment objects.
pub fn ipam_fhrp_group_assignments_bulk_update(state: &ThanixClient, body: Vec<FHRPGroupAssignmentRequest>) -> Result<IpamFhrpGroupAssignmentsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/fhrp-group-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupAssignmentsBulkUpdateResponse::Http200(r#response.json::<Vec<FHRPGroupAssignment>>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsCreateResponse {
	Http201(FHRPGroupAssignment),
	Other(Response)
}
/// Post a list of FHRP group assignment objects.
pub fn ipam_fhrp_group_assignments_create(state: &ThanixClient, body: WritableFHRPGroupAssignmentRequest) -> Result<IpamFhrpGroupAssignmentsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/fhrp-group-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamFhrpGroupAssignmentsCreateResponse::Http201(r#response.json::<FHRPGroupAssignment>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of FHRP group assignment objects.
pub fn ipam_fhrp_group_assignments_bulk_destroy(state: &ThanixClient, body: Vec<FHRPGroupAssignmentRequest>) -> Result<IpamFhrpGroupAssignmentsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/fhrp-group-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamFhrpGroupAssignmentsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsBulkPartialUpdateResponse {
	Http200(Vec<FHRPGroupAssignment>),
	Other(Response)
}
/// Patch a list of FHRP group assignment objects.
pub fn ipam_fhrp_group_assignments_bulk_partial_update(state: &ThanixClient, body: Vec<FHRPGroupAssignmentRequest>) -> Result<IpamFhrpGroupAssignmentsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/fhrp-group-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupAssignmentsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<FHRPGroupAssignment>>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsRetrieveResponse {
	Http200(FHRPGroupAssignment),
	Other(Response)
}
/// Get a FHRP group assignment object.
pub fn ipam_fhrp_group_assignments_retrieve(state: &ThanixClient, id: i64) -> Result<IpamFhrpGroupAssignmentsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/fhrp-group-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupAssignmentsRetrieveResponse::Http200(r#response.json::<FHRPGroupAssignment>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsUpdateResponse {
	Http200(FHRPGroupAssignment),
	Other(Response)
}
/// Put a FHRP group assignment object.
pub fn ipam_fhrp_group_assignments_update(state: &ThanixClient, body: WritableFHRPGroupAssignmentRequest, id: i64) -> Result<IpamFhrpGroupAssignmentsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/fhrp-group-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupAssignmentsUpdateResponse::Http200(r#response.json::<FHRPGroupAssignment>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a FHRP group assignment object.
pub fn ipam_fhrp_group_assignments_destroy(state: &ThanixClient, id: i64) -> Result<IpamFhrpGroupAssignmentsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/fhrp-group-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamFhrpGroupAssignmentsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupAssignmentsPartialUpdateResponse {
	Http200(FHRPGroupAssignment),
	Other(Response)
}
/// Patch a FHRP group assignment object.
pub fn ipam_fhrp_group_assignments_partial_update(state: &ThanixClient, body: PatchedWritableFHRPGroupAssignmentRequest, id: i64) -> Result<IpamFhrpGroupAssignmentsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/fhrp-group-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupAssignmentsPartialUpdateResponse::Http200(r#response.json::<FHRPGroupAssignment>()?)) },
		r#other_status => { Ok(IpamFhrpGroupAssignmentsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamFhrpGroupsListQuery {
	pub auth_key: Option<Vec<String>>,
	pub auth_key__empty: Option<bool>,
	pub auth_key__ic: Option<Vec<String>>,
	pub auth_key__ie: Option<Vec<String>>,
	pub auth_key__iew: Option<Vec<String>>,
	pub auth_key__isw: Option<Vec<String>>,
	pub auth_key__n: Option<Vec<String>>,
	pub auth_key__nic: Option<Vec<String>>,
	pub auth_key__nie: Option<Vec<String>>,
	pub auth_key__niew: Option<Vec<String>>,
	pub auth_key__nisw: Option<Vec<String>>,
	pub auth_type: Option<Vec<String>>,
	pub auth_type__n: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub group_id: Option<Vec<i64>>,
	pub group_id__empty: Option<bool>,
	pub group_id__gt: Option<Vec<i64>>,
	pub group_id__gte: Option<Vec<i64>>,
	pub group_id__lt: Option<Vec<i64>>,
	pub group_id__lte: Option<Vec<i64>>,
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub protocol: Option<Vec<String>>,
	pub protocol__n: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	pub related_ip: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamFhrpGroupsListResponse {
	Http200(PaginatedFHRPGroupList),
	Other(Response)
}
/// Get a list of FHRP group objects.
pub fn ipam_fhrp_groups_list(state: &ThanixClient, query: IpamFhrpGroupsListQuery) -> Result<IpamFhrpGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/fhrp-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupsListResponse::Http200(r#response.json::<PaginatedFHRPGroupList>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsBulkUpdateResponse {
	Http200(Vec<FHRPGroup>),
	Other(Response)
}
/// Put a list of FHRP group objects.
pub fn ipam_fhrp_groups_bulk_update(state: &ThanixClient, body: Vec<FHRPGroupRequest>) -> Result<IpamFhrpGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/fhrp-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<FHRPGroup>>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsCreateResponse {
	Http201(FHRPGroup),
	Other(Response)
}
/// Post a list of FHRP group objects.
pub fn ipam_fhrp_groups_create(state: &ThanixClient, body: FHRPGroupRequest) -> Result<IpamFhrpGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/fhrp-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamFhrpGroupsCreateResponse::Http201(r#response.json::<FHRPGroup>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of FHRP group objects.
pub fn ipam_fhrp_groups_bulk_destroy(state: &ThanixClient, body: Vec<FHRPGroupRequest>) -> Result<IpamFhrpGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/fhrp-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamFhrpGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsBulkPartialUpdateResponse {
	Http200(Vec<FHRPGroup>),
	Other(Response)
}
/// Patch a list of FHRP group objects.
pub fn ipam_fhrp_groups_bulk_partial_update(state: &ThanixClient, body: Vec<FHRPGroupRequest>) -> Result<IpamFhrpGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/fhrp-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<FHRPGroup>>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsRetrieveResponse {
	Http200(FHRPGroup),
	Other(Response)
}
/// Get a FHRP group object.
pub fn ipam_fhrp_groups_retrieve(state: &ThanixClient, id: i64) -> Result<IpamFhrpGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/fhrp-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupsRetrieveResponse::Http200(r#response.json::<FHRPGroup>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsUpdateResponse {
	Http200(FHRPGroup),
	Other(Response)
}
/// Put a FHRP group object.
pub fn ipam_fhrp_groups_update(state: &ThanixClient, body: FHRPGroupRequest, id: i64) -> Result<IpamFhrpGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/fhrp-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupsUpdateResponse::Http200(r#response.json::<FHRPGroup>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a FHRP group object.
pub fn ipam_fhrp_groups_destroy(state: &ThanixClient, id: i64) -> Result<IpamFhrpGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/fhrp-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamFhrpGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamFhrpGroupsPartialUpdateResponse {
	Http200(FHRPGroup),
	Other(Response)
}
/// Patch a FHRP group object.
pub fn ipam_fhrp_groups_partial_update(state: &ThanixClient, body: PatchedFHRPGroupRequest, id: i64) -> Result<IpamFhrpGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/fhrp-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamFhrpGroupsPartialUpdateResponse::Http200(r#response.json::<FHRPGroup>()?)) },
		r#other_status => { Ok(IpamFhrpGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamIpAddressesListQuery {
	pub address: Option<Vec<String>>,
	/// Is assigned
	pub assigned: Option<bool>,
	/// Is assigned to an interface
	pub assigned_to_interface: Option<bool>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub device: Option<Vec<String>>,
	pub device_id: Option<Vec<i64>>,
	pub dns_name: Option<Vec<String>>,
	pub dns_name__empty: Option<bool>,
	pub dns_name__ic: Option<Vec<String>>,
	pub dns_name__ie: Option<Vec<String>>,
	pub dns_name__iew: Option<Vec<String>>,
	pub dns_name__isw: Option<Vec<String>>,
	pub dns_name__n: Option<Vec<String>>,
	pub dns_name__nic: Option<Vec<String>>,
	pub dns_name__nie: Option<Vec<String>>,
	pub dns_name__niew: Option<Vec<String>>,
	pub dns_name__nisw: Option<Vec<String>>,
	pub family: Option<f64>,
	/// FHRP group (ID)
	pub fhrpgroup_id: Option<Vec<i64>>,
	/// FHRP group (ID)
	pub fhrpgroup_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Interface (name)
	pub interface: Option<Vec<String>>,
	/// Interface (name)
	pub interface__n: Option<Vec<String>>,
	/// Interface (ID)
	pub interface_id: Option<Vec<i64>>,
	/// Interface (ID)
	pub interface_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub mask_length: Option<Vec<i64>>,
	pub mask_length__gte: Option<f64>,
	pub mask_length__lte: Option<f64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub parent: Option<Vec<String>>,
	pub present_in_vrf: Option<String>,
	pub present_in_vrf_id: Option<String>,
	/// Search
	pub q: Option<String>,
	/// The functional role of this IP
	pub role: Option<Vec<String>>,
	/// The functional role of this IP
	pub role__n: Option<Vec<String>>,
	/// The operational status of this IP
	pub status: Option<Vec<String>>,
	/// The operational status of this IP
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	pub virtual_machine: Option<Vec<String>>,
	pub virtual_machine_id: Option<Vec<i64>>,
	/// VM interface (name)
	pub vminterface: Option<Vec<String>>,
	/// VM interface (name)
	pub vminterface__n: Option<Vec<String>>,
	/// VM interface (ID)
	pub vminterface_id: Option<Vec<i64>>,
	/// VM interface (ID)
	pub vminterface_id__n: Option<Vec<i64>>,
	/// VRF (RD)
	pub vrf: Option<Vec<Option<String>>>,
	/// VRF (RD)
	pub vrf__n: Option<Vec<Option<String>>>,
	/// VRF
	pub vrf_id: Option<Vec<Option<i64>>>,
	/// VRF
	pub vrf_id__n: Option<Vec<Option<i64>>>,

}
#[derive(Debug)]
pub enum IpamIpAddressesListResponse {
	Http200(PaginatedIPAddressList),
	Other(Response)
}
/// Get a list of IP address objects.
pub fn ipam_ip_addresses_list(state: &ThanixClient, query: IpamIpAddressesListQuery) -> Result<IpamIpAddressesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/ip-addresses/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpAddressesListResponse::Http200(r#response.json::<PaginatedIPAddressList>()?)) },
		r#other_status => { Ok(IpamIpAddressesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesBulkUpdateResponse {
	Http200(Vec<IPAddress>),
	Other(Response)
}
/// Put a list of IP address objects.
pub fn ipam_ip_addresses_bulk_update(state: &ThanixClient, body: Vec<IPAddressRequest>) -> Result<IpamIpAddressesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/ip-addresses/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpAddressesBulkUpdateResponse::Http200(r#response.json::<Vec<IPAddress>>()?)) },
		r#other_status => { Ok(IpamIpAddressesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesCreateResponse {
	Http201(IPAddress),
	Other(Response)
}
/// Post a list of IP address objects.
pub fn ipam_ip_addresses_create(state: &ThanixClient, body: WritableIPAddressRequest) -> Result<IpamIpAddressesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/ip-addresses/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamIpAddressesCreateResponse::Http201(r#response.json::<IPAddress>()?)) },
		r#other_status => { Ok(IpamIpAddressesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of IP address objects.
pub fn ipam_ip_addresses_bulk_destroy(state: &ThanixClient, body: Vec<IPAddressRequest>) -> Result<IpamIpAddressesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/ip-addresses/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamIpAddressesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesBulkPartialUpdateResponse {
	Http200(Vec<IPAddress>),
	Other(Response)
}
/// Patch a list of IP address objects.
pub fn ipam_ip_addresses_bulk_partial_update(state: &ThanixClient, body: Vec<IPAddressRequest>) -> Result<IpamIpAddressesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/ip-addresses/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpAddressesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<IPAddress>>()?)) },
		r#other_status => { Ok(IpamIpAddressesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesRetrieveResponse {
	Http200(IPAddress),
	Other(Response)
}
/// Get a IP address object.
pub fn ipam_ip_addresses_retrieve(state: &ThanixClient, id: i64) -> Result<IpamIpAddressesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/ip-addresses/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpAddressesRetrieveResponse::Http200(r#response.json::<IPAddress>()?)) },
		r#other_status => { Ok(IpamIpAddressesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesUpdateResponse {
	Http200(IPAddress),
	Other(Response)
}
/// Put a IP address object.
pub fn ipam_ip_addresses_update(state: &ThanixClient, body: WritableIPAddressRequest, id: i64) -> Result<IpamIpAddressesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/ip-addresses/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpAddressesUpdateResponse::Http200(r#response.json::<IPAddress>()?)) },
		r#other_status => { Ok(IpamIpAddressesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a IP address object.
pub fn ipam_ip_addresses_destroy(state: &ThanixClient, id: i64) -> Result<IpamIpAddressesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/ip-addresses/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamIpAddressesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpAddressesPartialUpdateResponse {
	Http200(IPAddress),
	Other(Response)
}
/// Patch a IP address object.
pub fn ipam_ip_addresses_partial_update(state: &ThanixClient, body: PatchedWritableIPAddressRequest, id: i64) -> Result<IpamIpAddressesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/ip-addresses/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpAddressesPartialUpdateResponse::Http200(r#response.json::<IPAddress>()?)) },
		r#other_status => { Ok(IpamIpAddressesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamIpRangesListQuery {
	/// Ranges which contain this prefix or IP
	pub contains: Option<String>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub end_address: Option<Vec<String>>,
	pub family: Option<f64>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub mark_utilized: Option<bool>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub parent: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	pub start_address: Option<Vec<String>>,
	/// Operational status of this range
	pub status: Option<Vec<String>>,
	/// Operational status of this range
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	/// VRF (RD)
	pub vrf: Option<Vec<Option<String>>>,
	/// VRF (RD)
	pub vrf__n: Option<Vec<Option<String>>>,
	/// VRF
	pub vrf_id: Option<Vec<Option<i64>>>,
	/// VRF
	pub vrf_id__n: Option<Vec<Option<i64>>>,

}
#[derive(Debug)]
pub enum IpamIpRangesListResponse {
	Http200(PaginatedIPRangeList),
	Other(Response)
}
/// Get a list of IP range objects.
pub fn ipam_ip_ranges_list(state: &ThanixClient, query: IpamIpRangesListQuery) -> Result<IpamIpRangesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/ip-ranges/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesListResponse::Http200(r#response.json::<PaginatedIPRangeList>()?)) },
		r#other_status => { Ok(IpamIpRangesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesBulkUpdateResponse {
	Http200(Vec<IPRange>),
	Other(Response)
}
/// Put a list of IP range objects.
pub fn ipam_ip_ranges_bulk_update(state: &ThanixClient, body: Vec<IPRangeRequest>) -> Result<IpamIpRangesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/ip-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesBulkUpdateResponse::Http200(r#response.json::<Vec<IPRange>>()?)) },
		r#other_status => { Ok(IpamIpRangesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesCreateResponse {
	Http201(IPRange),
	Other(Response)
}
/// Post a list of IP range objects.
pub fn ipam_ip_ranges_create(state: &ThanixClient, body: WritableIPRangeRequest) -> Result<IpamIpRangesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/ip-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamIpRangesCreateResponse::Http201(r#response.json::<IPRange>()?)) },
		r#other_status => { Ok(IpamIpRangesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of IP range objects.
pub fn ipam_ip_ranges_bulk_destroy(state: &ThanixClient, body: Vec<IPRangeRequest>) -> Result<IpamIpRangesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/ip-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamIpRangesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesBulkPartialUpdateResponse {
	Http200(Vec<IPRange>),
	Other(Response)
}
/// Patch a list of IP range objects.
pub fn ipam_ip_ranges_bulk_partial_update(state: &ThanixClient, body: Vec<IPRangeRequest>) -> Result<IpamIpRangesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/ip-ranges/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<IPRange>>()?)) },
		r#other_status => { Ok(IpamIpRangesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesRetrieveResponse {
	Http200(IPRange),
	Other(Response)
}
/// Get a IP range object.
pub fn ipam_ip_ranges_retrieve(state: &ThanixClient, id: i64) -> Result<IpamIpRangesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/ip-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesRetrieveResponse::Http200(r#response.json::<IPRange>()?)) },
		r#other_status => { Ok(IpamIpRangesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesUpdateResponse {
	Http200(IPRange),
	Other(Response)
}
/// Put a IP range object.
pub fn ipam_ip_ranges_update(state: &ThanixClient, body: WritableIPRangeRequest, id: i64) -> Result<IpamIpRangesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/ip-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesUpdateResponse::Http200(r#response.json::<IPRange>()?)) },
		r#other_status => { Ok(IpamIpRangesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a IP range object.
pub fn ipam_ip_ranges_destroy(state: &ThanixClient, id: i64) -> Result<IpamIpRangesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/ip-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamIpRangesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesPartialUpdateResponse {
	Http200(IPRange),
	Other(Response)
}
/// Patch a IP range object.
pub fn ipam_ip_ranges_partial_update(state: &ThanixClient, body: PatchedWritableIPRangeRequest, id: i64) -> Result<IpamIpRangesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/ip-ranges/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesPartialUpdateResponse::Http200(r#response.json::<IPRange>()?)) },
		r#other_status => { Ok(IpamIpRangesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesAvailableIpsListResponse {
	Http200(Vec<AvailableIP>),
	Other(Response)
}
/// Get a IP address object.
pub fn ipam_ip_ranges_available_ips_list(state: &ThanixClient, id: i64) -> Result<IpamIpRangesAvailableIpsListResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/ip-ranges/{id}/available-ips/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamIpRangesAvailableIpsListResponse::Http200(r#response.json::<Vec<AvailableIP>>()?)) },
		r#other_status => { Ok(IpamIpRangesAvailableIpsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamIpRangesAvailableIpsCreateResponse {
	Http201(Vec<IPAddress>),
	Other(Response)
}
/// Post a IP address object.
pub fn ipam_ip_ranges_available_ips_create(state: &ThanixClient, body: Vec<IPAddressRequest>, id: i64) -> Result<IpamIpRangesAvailableIpsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/ip-ranges/{id}/available-ips/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamIpRangesAvailableIpsCreateResponse::Http201(r#response.json::<Vec<IPAddress>>()?)) },
		r#other_status => { Ok(IpamIpRangesAvailableIpsCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamL2VpnTerminationsListQuery {
	pub assigned_object_type: Option<String>,
	pub assigned_object_type__n: Option<String>,
	pub assigned_object_type_id: Option<i64>,
	pub assigned_object_type_id__n: Option<i64>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<i64>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Interface (name)
	pub interface: Option<Vec<String>>,
	/// Interface (name)
	pub interface__n: Option<Vec<String>>,
	/// Interface (ID)
	pub interface_id: Option<Vec<i64>>,
	/// Interface (ID)
	pub interface_id__n: Option<Vec<i64>>,
	/// L2VPN (slug)
	pub l2vpn: Option<Vec<String>>,
	/// L2VPN (slug)
	pub l2vpn__n: Option<Vec<String>>,
	/// L2VPN (ID)
	pub l2vpn_id: Option<Vec<i64>>,
	/// L2VPN (ID)
	pub l2vpn_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub region: Option<Vec<String>>,
	pub region_id: Option<Vec<i64>>,
	pub site: Option<Vec<String>>,
	pub site_id: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual machine (name)
	pub virtual_machine: Option<Vec<String>>,
	/// Virtual machine (name)
	pub virtual_machine__n: Option<Vec<String>>,
	/// Virtual machine (ID)
	pub virtual_machine_id: Option<Vec<i64>>,
	/// Virtual machine (ID)
	pub virtual_machine_id__n: Option<Vec<i64>>,
	/// VLAN (name)
	pub vlan: Option<Vec<String>>,
	/// VLAN (name)
	pub vlan__n: Option<Vec<String>>,
	/// VLAN (ID)
	pub vlan_id: Option<Vec<i64>>,
	/// VLAN (ID)
	pub vlan_id__n: Option<Vec<i64>>,
	/// VLAN number (1-4094)
	pub vlan_vid: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__empty: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__gt: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__gte: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__lt: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__lte: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__n: Option<i64>,
	/// VM interface (name)
	pub vminterface: Option<Vec<String>>,
	/// VM interface (name)
	pub vminterface__n: Option<Vec<String>>,
	/// VM Interface (ID)
	pub vminterface_id: Option<Vec<i64>>,
	/// VM Interface (ID)
	pub vminterface_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsListResponse {
	Http200(PaginatedL2VPNTerminationList),
	Other(Response)
}
/// Get a list of L2VPN termination objects.
pub fn ipam_l2vpn_terminations_list(state: &ThanixClient, query: IpamL2VpnTerminationsListQuery) -> Result<IpamL2VpnTerminationsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/l2vpn-terminations/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnTerminationsListResponse::Http200(r#response.json::<PaginatedL2VPNTerminationList>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsBulkUpdateResponse {
	Http200(Vec<L2VPNTermination>),
	Other(Response)
}
/// Put a list of L2VPN termination objects.
pub fn ipam_l2vpn_terminations_bulk_update(state: &ThanixClient, body: Vec<L2VPNTerminationRequest>) -> Result<IpamL2VpnTerminationsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/l2vpn-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnTerminationsBulkUpdateResponse::Http200(r#response.json::<Vec<L2VPNTermination>>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsCreateResponse {
	Http201(L2VPNTermination),
	Other(Response)
}
/// Post a list of L2VPN termination objects.
pub fn ipam_l2vpn_terminations_create(state: &ThanixClient, body: WritableL2VPNTerminationRequest) -> Result<IpamL2VpnTerminationsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/l2vpn-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamL2VpnTerminationsCreateResponse::Http201(r#response.json::<L2VPNTermination>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of L2VPN termination objects.
pub fn ipam_l2vpn_terminations_bulk_destroy(state: &ThanixClient, body: Vec<L2VPNTerminationRequest>) -> Result<IpamL2VpnTerminationsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/l2vpn-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamL2VpnTerminationsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsBulkPartialUpdateResponse {
	Http200(Vec<L2VPNTermination>),
	Other(Response)
}
/// Patch a list of L2VPN termination objects.
pub fn ipam_l2vpn_terminations_bulk_partial_update(state: &ThanixClient, body: Vec<L2VPNTerminationRequest>) -> Result<IpamL2VpnTerminationsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/l2vpn-terminations/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnTerminationsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<L2VPNTermination>>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsRetrieveResponse {
	Http200(L2VPNTermination),
	Other(Response)
}
/// Get a L2VPN termination object.
pub fn ipam_l2vpn_terminations_retrieve(state: &ThanixClient, id: i64) -> Result<IpamL2VpnTerminationsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/l2vpn-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnTerminationsRetrieveResponse::Http200(r#response.json::<L2VPNTermination>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsUpdateResponse {
	Http200(L2VPNTermination),
	Other(Response)
}
/// Put a L2VPN termination object.
pub fn ipam_l2vpn_terminations_update(state: &ThanixClient, body: WritableL2VPNTerminationRequest, id: i64) -> Result<IpamL2VpnTerminationsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/l2vpn-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnTerminationsUpdateResponse::Http200(r#response.json::<L2VPNTermination>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a L2VPN termination object.
pub fn ipam_l2vpn_terminations_destroy(state: &ThanixClient, id: i64) -> Result<IpamL2VpnTerminationsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/l2vpn-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamL2VpnTerminationsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnTerminationsPartialUpdateResponse {
	Http200(L2VPNTermination),
	Other(Response)
}
/// Patch a L2VPN termination object.
pub fn ipam_l2vpn_terminations_partial_update(state: &ThanixClient, body: PatchedWritableL2VPNTerminationRequest, id: i64) -> Result<IpamL2VpnTerminationsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/l2vpn-terminations/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnTerminationsPartialUpdateResponse::Http200(r#response.json::<L2VPNTermination>()?)) },
		r#other_status => { Ok(IpamL2VpnTerminationsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamL2VpnsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Export target (name)
	pub export_target: Option<Vec<String>>,
	/// Export target (name)
	pub export_target__n: Option<Vec<String>>,
	/// Export target
	pub export_target_id: Option<Vec<i64>>,
	/// Export target
	pub export_target_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub identifier: Option<Vec<i64>>,
	pub identifier__empty: Option<bool>,
	pub identifier__gt: Option<Vec<i64>>,
	pub identifier__gte: Option<Vec<i64>>,
	pub identifier__lt: Option<Vec<i64>>,
	pub identifier__lte: Option<Vec<i64>>,
	pub identifier__n: Option<Vec<i64>>,
	/// Import target (name)
	pub import_target: Option<Vec<String>>,
	/// Import target (name)
	pub import_target__n: Option<Vec<String>>,
	/// Import target
	pub import_target_id: Option<Vec<i64>>,
	/// Import target
	pub import_target_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub r#type: Option<Vec<String>>,
	pub type__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamL2VpnsListResponse {
	Http200(PaginatedL2VPNList),
	Other(Response)
}
/// Get a list of L2VPN objects.
pub fn ipam_l2vpns_list(state: &ThanixClient, query: IpamL2VpnsListQuery) -> Result<IpamL2VpnsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/l2vpns/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnsListResponse::Http200(r#response.json::<PaginatedL2VPNList>()?)) },
		r#other_status => { Ok(IpamL2VpnsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsBulkUpdateResponse {
	Http200(Vec<L2VPN>),
	Other(Response)
}
/// Put a list of L2VPN objects.
pub fn ipam_l2vpns_bulk_update(state: &ThanixClient, body: Vec<L2VPNRequest>) -> Result<IpamL2VpnsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/l2vpns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnsBulkUpdateResponse::Http200(r#response.json::<Vec<L2VPN>>()?)) },
		r#other_status => { Ok(IpamL2VpnsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsCreateResponse {
	Http201(L2VPN),
	Other(Response)
}
/// Post a list of L2VPN objects.
pub fn ipam_l2vpns_create(state: &ThanixClient, body: WritableL2VPNRequest) -> Result<IpamL2VpnsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/l2vpns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamL2VpnsCreateResponse::Http201(r#response.json::<L2VPN>()?)) },
		r#other_status => { Ok(IpamL2VpnsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of L2VPN objects.
pub fn ipam_l2vpns_bulk_destroy(state: &ThanixClient, body: Vec<L2VPNRequest>) -> Result<IpamL2VpnsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/l2vpns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamL2VpnsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsBulkPartialUpdateResponse {
	Http200(Vec<L2VPN>),
	Other(Response)
}
/// Patch a list of L2VPN objects.
pub fn ipam_l2vpns_bulk_partial_update(state: &ThanixClient, body: Vec<L2VPNRequest>) -> Result<IpamL2VpnsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/l2vpns/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<L2VPN>>()?)) },
		r#other_status => { Ok(IpamL2VpnsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsRetrieveResponse {
	Http200(L2VPN),
	Other(Response)
}
/// Get a L2VPN object.
pub fn ipam_l2vpns_retrieve(state: &ThanixClient, id: i64) -> Result<IpamL2VpnsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/l2vpns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnsRetrieveResponse::Http200(r#response.json::<L2VPN>()?)) },
		r#other_status => { Ok(IpamL2VpnsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsUpdateResponse {
	Http200(L2VPN),
	Other(Response)
}
/// Put a L2VPN object.
pub fn ipam_l2vpns_update(state: &ThanixClient, body: WritableL2VPNRequest, id: i64) -> Result<IpamL2VpnsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/l2vpns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnsUpdateResponse::Http200(r#response.json::<L2VPN>()?)) },
		r#other_status => { Ok(IpamL2VpnsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a L2VPN object.
pub fn ipam_l2vpns_destroy(state: &ThanixClient, id: i64) -> Result<IpamL2VpnsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/l2vpns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamL2VpnsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamL2VpnsPartialUpdateResponse {
	Http200(L2VPN),
	Other(Response)
}
/// Patch a L2VPN object.
pub fn ipam_l2vpns_partial_update(state: &ThanixClient, body: PatchedWritableL2VPNRequest, id: i64) -> Result<IpamL2VpnsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/l2vpns/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamL2VpnsPartialUpdateResponse::Http200(r#response.json::<L2VPN>()?)) },
		r#other_status => { Ok(IpamL2VpnsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamPrefixesListQuery {
	pub children: Option<Vec<i64>>,
	pub children__empty: Option<Vec<i64>>,
	pub children__gt: Option<Vec<i64>>,
	pub children__gte: Option<Vec<i64>>,
	pub children__lt: Option<Vec<i64>>,
	pub children__lte: Option<Vec<i64>>,
	pub children__n: Option<Vec<i64>>,
	/// Prefixes which contain this prefix or IP
	pub contains: Option<String>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub depth: Option<Vec<i64>>,
	pub depth__empty: Option<Vec<i64>>,
	pub depth__gt: Option<Vec<i64>>,
	pub depth__gte: Option<Vec<i64>>,
	pub depth__lt: Option<Vec<i64>>,
	pub depth__lte: Option<Vec<i64>>,
	pub depth__n: Option<Vec<i64>>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub family: Option<f64>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub is_pool: Option<bool>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub mark_utilized: Option<bool>,
	pub mask_length: Option<Vec<i64>>,
	pub mask_length__gte: Option<f64>,
	pub mask_length__lte: Option<f64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub prefix: Option<Vec<String>>,
	pub present_in_vrf: Option<String>,
	pub present_in_vrf_id: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<Option<i64>>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<Option<i64>>>,
	/// Operational status of this prefix
	pub status: Option<Vec<String>>,
	/// Operational status of this prefix
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	/// VLAN (ID)
	pub vlan_id: Option<Vec<Option<i64>>>,
	/// VLAN (ID)
	pub vlan_id__n: Option<Vec<Option<i64>>>,
	/// VLAN number (1-4094)
	pub vlan_vid: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__empty: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__gt: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__gte: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__lt: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__lte: Option<i64>,
	/// VLAN number (1-4094)
	pub vlan_vid__n: Option<i64>,
	/// VRF (RD)
	pub vrf: Option<Vec<Option<String>>>,
	/// VRF (RD)
	pub vrf__n: Option<Vec<Option<String>>>,
	/// VRF
	pub vrf_id: Option<Vec<Option<i64>>>,
	/// VRF
	pub vrf_id__n: Option<Vec<Option<i64>>>,
	/// Within prefix
	pub within: Option<String>,
	/// Within and including prefix
	pub within_include: Option<String>,

}
#[derive(Debug)]
pub enum IpamPrefixesListResponse {
	Http200(PaginatedPrefixList),
	Other(Response)
}
/// Get a list of prefix objects.
pub fn ipam_prefixes_list(state: &ThanixClient, query: IpamPrefixesListQuery) -> Result<IpamPrefixesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/prefixes/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesListResponse::Http200(r#response.json::<PaginatedPrefixList>()?)) },
		r#other_status => { Ok(IpamPrefixesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesBulkUpdateResponse {
	Http200(Vec<Prefix>),
	Other(Response)
}
/// Put a list of prefix objects.
pub fn ipam_prefixes_bulk_update(state: &ThanixClient, body: Vec<PrefixRequest>) -> Result<IpamPrefixesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/prefixes/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesBulkUpdateResponse::Http200(r#response.json::<Vec<Prefix>>()?)) },
		r#other_status => { Ok(IpamPrefixesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesCreateResponse {
	Http201(Prefix),
	Other(Response)
}
/// Post a list of prefix objects.
pub fn ipam_prefixes_create(state: &ThanixClient, body: WritablePrefixRequest) -> Result<IpamPrefixesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/prefixes/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamPrefixesCreateResponse::Http201(r#response.json::<Prefix>()?)) },
		r#other_status => { Ok(IpamPrefixesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of prefix objects.
pub fn ipam_prefixes_bulk_destroy(state: &ThanixClient, body: Vec<PrefixRequest>) -> Result<IpamPrefixesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/prefixes/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamPrefixesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesBulkPartialUpdateResponse {
	Http200(Vec<Prefix>),
	Other(Response)
}
/// Patch a list of prefix objects.
pub fn ipam_prefixes_bulk_partial_update(state: &ThanixClient, body: Vec<PrefixRequest>) -> Result<IpamPrefixesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/prefixes/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Prefix>>()?)) },
		r#other_status => { Ok(IpamPrefixesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesRetrieveResponse {
	Http200(Prefix),
	Other(Response)
}
/// Get a prefix object.
pub fn ipam_prefixes_retrieve(state: &ThanixClient, id: i64) -> Result<IpamPrefixesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/prefixes/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesRetrieveResponse::Http200(r#response.json::<Prefix>()?)) },
		r#other_status => { Ok(IpamPrefixesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesUpdateResponse {
	Http200(Prefix),
	Other(Response)
}
/// Put a prefix object.
pub fn ipam_prefixes_update(state: &ThanixClient, body: WritablePrefixRequest, id: i64) -> Result<IpamPrefixesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/prefixes/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesUpdateResponse::Http200(r#response.json::<Prefix>()?)) },
		r#other_status => { Ok(IpamPrefixesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a prefix object.
pub fn ipam_prefixes_destroy(state: &ThanixClient, id: i64) -> Result<IpamPrefixesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/prefixes/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamPrefixesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesPartialUpdateResponse {
	Http200(Prefix),
	Other(Response)
}
/// Patch a prefix object.
pub fn ipam_prefixes_partial_update(state: &ThanixClient, body: PatchedWritablePrefixRequest, id: i64) -> Result<IpamPrefixesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/prefixes/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesPartialUpdateResponse::Http200(r#response.json::<Prefix>()?)) },
		r#other_status => { Ok(IpamPrefixesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesAvailableIpsListResponse {
	Http200(Vec<AvailableIP>),
	Other(Response)
}
/// Get a IP address object.
pub fn ipam_prefixes_available_ips_list(state: &ThanixClient, id: i64) -> Result<IpamPrefixesAvailableIpsListResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/prefixes/{id}/available-ips/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesAvailableIpsListResponse::Http200(r#response.json::<Vec<AvailableIP>>()?)) },
		r#other_status => { Ok(IpamPrefixesAvailableIpsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesAvailableIpsCreateResponse {
	Http201(Vec<IPAddress>),
	Other(Response)
}
/// Post a IP address object.
pub fn ipam_prefixes_available_ips_create(state: &ThanixClient, body: Vec<IPAddressRequest>, id: i64) -> Result<IpamPrefixesAvailableIpsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/prefixes/{id}/available-ips/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamPrefixesAvailableIpsCreateResponse::Http201(r#response.json::<Vec<IPAddress>>()?)) },
		r#other_status => { Ok(IpamPrefixesAvailableIpsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesAvailablePrefixesListResponse {
	Http200(Vec<AvailablePrefix>),
	Other(Response)
}
/// Get a prefix object.
pub fn ipam_prefixes_available_prefixes_list(state: &ThanixClient, id: i64) -> Result<IpamPrefixesAvailablePrefixesListResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/prefixes/{id}/available-prefixes/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamPrefixesAvailablePrefixesListResponse::Http200(r#response.json::<Vec<AvailablePrefix>>()?)) },
		r#other_status => { Ok(IpamPrefixesAvailablePrefixesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamPrefixesAvailablePrefixesCreateResponse {
	Http201(Vec<Prefix>),
	Other(Response)
}
/// Post a prefix object.
pub fn ipam_prefixes_available_prefixes_create(state: &ThanixClient, body: Vec<PrefixRequest>, id: i64) -> Result<IpamPrefixesAvailablePrefixesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/prefixes/{id}/available-prefixes/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamPrefixesAvailablePrefixesCreateResponse::Http201(r#response.json::<Vec<Prefix>>()?)) },
		r#other_status => { Ok(IpamPrefixesAvailablePrefixesCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamRirsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub is_private: Option<bool>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamRirsListResponse {
	Http200(PaginatedRIRList),
	Other(Response)
}
/// Get a list of RIR objects.
pub fn ipam_rirs_list(state: &ThanixClient, query: IpamRirsListQuery) -> Result<IpamRirsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/rirs/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRirsListResponse::Http200(r#response.json::<PaginatedRIRList>()?)) },
		r#other_status => { Ok(IpamRirsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsBulkUpdateResponse {
	Http200(Vec<RIR>),
	Other(Response)
}
/// Put a list of RIR objects.
pub fn ipam_rirs_bulk_update(state: &ThanixClient, body: Vec<RIRRequest>) -> Result<IpamRirsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/rirs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRirsBulkUpdateResponse::Http200(r#response.json::<Vec<RIR>>()?)) },
		r#other_status => { Ok(IpamRirsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsCreateResponse {
	Http201(RIR),
	Other(Response)
}
/// Post a list of RIR objects.
pub fn ipam_rirs_create(state: &ThanixClient, body: RIRRequest) -> Result<IpamRirsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/rirs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamRirsCreateResponse::Http201(r#response.json::<RIR>()?)) },
		r#other_status => { Ok(IpamRirsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of RIR objects.
pub fn ipam_rirs_bulk_destroy(state: &ThanixClient, body: Vec<RIRRequest>) -> Result<IpamRirsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/rirs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamRirsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsBulkPartialUpdateResponse {
	Http200(Vec<RIR>),
	Other(Response)
}
/// Patch a list of RIR objects.
pub fn ipam_rirs_bulk_partial_update(state: &ThanixClient, body: Vec<RIRRequest>) -> Result<IpamRirsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/rirs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRirsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<RIR>>()?)) },
		r#other_status => { Ok(IpamRirsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsRetrieveResponse {
	Http200(RIR),
	Other(Response)
}
/// Get a RIR object.
pub fn ipam_rirs_retrieve(state: &ThanixClient, id: i64) -> Result<IpamRirsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/rirs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRirsRetrieveResponse::Http200(r#response.json::<RIR>()?)) },
		r#other_status => { Ok(IpamRirsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsUpdateResponse {
	Http200(RIR),
	Other(Response)
}
/// Put a RIR object.
pub fn ipam_rirs_update(state: &ThanixClient, body: RIRRequest, id: i64) -> Result<IpamRirsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/rirs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRirsUpdateResponse::Http200(r#response.json::<RIR>()?)) },
		r#other_status => { Ok(IpamRirsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a RIR object.
pub fn ipam_rirs_destroy(state: &ThanixClient, id: i64) -> Result<IpamRirsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/rirs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamRirsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRirsPartialUpdateResponse {
	Http200(RIR),
	Other(Response)
}
/// Patch a RIR object.
pub fn ipam_rirs_partial_update(state: &ThanixClient, body: PatchedRIRRequest, id: i64) -> Result<IpamRirsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/rirs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRirsPartialUpdateResponse::Http200(r#response.json::<RIR>()?)) },
		r#other_status => { Ok(IpamRirsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamRolesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamRolesListResponse {
	Http200(PaginatedRoleList),
	Other(Response)
}
/// Get a list of role objects.
pub fn ipam_roles_list(state: &ThanixClient, query: IpamRolesListQuery) -> Result<IpamRolesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/roles/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRolesListResponse::Http200(r#response.json::<PaginatedRoleList>()?)) },
		r#other_status => { Ok(IpamRolesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesBulkUpdateResponse {
	Http200(Vec<Role>),
	Other(Response)
}
/// Put a list of role objects.
pub fn ipam_roles_bulk_update(state: &ThanixClient, body: Vec<RoleRequest>) -> Result<IpamRolesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRolesBulkUpdateResponse::Http200(r#response.json::<Vec<Role>>()?)) },
		r#other_status => { Ok(IpamRolesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesCreateResponse {
	Http201(Role),
	Other(Response)
}
/// Post a list of role objects.
pub fn ipam_roles_create(state: &ThanixClient, body: RoleRequest) -> Result<IpamRolesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamRolesCreateResponse::Http201(r#response.json::<Role>()?)) },
		r#other_status => { Ok(IpamRolesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of role objects.
pub fn ipam_roles_bulk_destroy(state: &ThanixClient, body: Vec<RoleRequest>) -> Result<IpamRolesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamRolesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesBulkPartialUpdateResponse {
	Http200(Vec<Role>),
	Other(Response)
}
/// Patch a list of role objects.
pub fn ipam_roles_bulk_partial_update(state: &ThanixClient, body: Vec<RoleRequest>) -> Result<IpamRolesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRolesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Role>>()?)) },
		r#other_status => { Ok(IpamRolesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesRetrieveResponse {
	Http200(Role),
	Other(Response)
}
/// Get a role object.
pub fn ipam_roles_retrieve(state: &ThanixClient, id: i64) -> Result<IpamRolesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRolesRetrieveResponse::Http200(r#response.json::<Role>()?)) },
		r#other_status => { Ok(IpamRolesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesUpdateResponse {
	Http200(Role),
	Other(Response)
}
/// Put a role object.
pub fn ipam_roles_update(state: &ThanixClient, body: RoleRequest, id: i64) -> Result<IpamRolesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRolesUpdateResponse::Http200(r#response.json::<Role>()?)) },
		r#other_status => { Ok(IpamRolesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a role object.
pub fn ipam_roles_destroy(state: &ThanixClient, id: i64) -> Result<IpamRolesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamRolesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRolesPartialUpdateResponse {
	Http200(Role),
	Other(Response)
}
/// Patch a role object.
pub fn ipam_roles_partial_update(state: &ThanixClient, body: PatchedRoleRequest, id: i64) -> Result<IpamRolesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRolesPartialUpdateResponse::Http200(r#response.json::<Role>()?)) },
		r#other_status => { Ok(IpamRolesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamRouteTargetsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Export VRF (RD)
	pub exporting_vrf: Option<Vec<Option<String>>>,
	/// Export VRF (RD)
	pub exporting_vrf__n: Option<Vec<Option<String>>>,
	/// Exporting VRF
	pub exporting_vrf_id: Option<Vec<i64>>,
	/// Exporting VRF
	pub exporting_vrf_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Import VRF (RD)
	pub importing_vrf: Option<Vec<Option<String>>>,
	/// Import VRF (RD)
	pub importing_vrf__n: Option<Vec<Option<String>>>,
	/// Importing VRF
	pub importing_vrf_id: Option<Vec<i64>>,
	/// Importing VRF
	pub importing_vrf_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamRouteTargetsListResponse {
	Http200(PaginatedRouteTargetList),
	Other(Response)
}
/// Get a list of route target objects.
pub fn ipam_route_targets_list(state: &ThanixClient, query: IpamRouteTargetsListQuery) -> Result<IpamRouteTargetsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/route-targets/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRouteTargetsListResponse::Http200(r#response.json::<PaginatedRouteTargetList>()?)) },
		r#other_status => { Ok(IpamRouteTargetsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsBulkUpdateResponse {
	Http200(Vec<RouteTarget>),
	Other(Response)
}
/// Put a list of route target objects.
pub fn ipam_route_targets_bulk_update(state: &ThanixClient, body: Vec<RouteTargetRequest>) -> Result<IpamRouteTargetsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/route-targets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRouteTargetsBulkUpdateResponse::Http200(r#response.json::<Vec<RouteTarget>>()?)) },
		r#other_status => { Ok(IpamRouteTargetsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsCreateResponse {
	Http201(RouteTarget),
	Other(Response)
}
/// Post a list of route target objects.
pub fn ipam_route_targets_create(state: &ThanixClient, body: WritableRouteTargetRequest) -> Result<IpamRouteTargetsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/route-targets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamRouteTargetsCreateResponse::Http201(r#response.json::<RouteTarget>()?)) },
		r#other_status => { Ok(IpamRouteTargetsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of route target objects.
pub fn ipam_route_targets_bulk_destroy(state: &ThanixClient, body: Vec<RouteTargetRequest>) -> Result<IpamRouteTargetsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/route-targets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamRouteTargetsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsBulkPartialUpdateResponse {
	Http200(Vec<RouteTarget>),
	Other(Response)
}
/// Patch a list of route target objects.
pub fn ipam_route_targets_bulk_partial_update(state: &ThanixClient, body: Vec<RouteTargetRequest>) -> Result<IpamRouteTargetsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/route-targets/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRouteTargetsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<RouteTarget>>()?)) },
		r#other_status => { Ok(IpamRouteTargetsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsRetrieveResponse {
	Http200(RouteTarget),
	Other(Response)
}
/// Get a route target object.
pub fn ipam_route_targets_retrieve(state: &ThanixClient, id: i64) -> Result<IpamRouteTargetsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/route-targets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRouteTargetsRetrieveResponse::Http200(r#response.json::<RouteTarget>()?)) },
		r#other_status => { Ok(IpamRouteTargetsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsUpdateResponse {
	Http200(RouteTarget),
	Other(Response)
}
/// Put a route target object.
pub fn ipam_route_targets_update(state: &ThanixClient, body: WritableRouteTargetRequest, id: i64) -> Result<IpamRouteTargetsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/route-targets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRouteTargetsUpdateResponse::Http200(r#response.json::<RouteTarget>()?)) },
		r#other_status => { Ok(IpamRouteTargetsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a route target object.
pub fn ipam_route_targets_destroy(state: &ThanixClient, id: i64) -> Result<IpamRouteTargetsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/route-targets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamRouteTargetsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamRouteTargetsPartialUpdateResponse {
	Http200(RouteTarget),
	Other(Response)
}
/// Patch a route target object.
pub fn ipam_route_targets_partial_update(state: &ThanixClient, body: PatchedWritableRouteTargetRequest, id: i64) -> Result<IpamRouteTargetsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/route-targets/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamRouteTargetsPartialUpdateResponse::Http200(r#response.json::<RouteTarget>()?)) },
		r#other_status => { Ok(IpamRouteTargetsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamServiceTemplatesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub port: Option<f64>,
	pub protocol: Option<String>,
	pub protocol__n: Option<String>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamServiceTemplatesListResponse {
	Http200(PaginatedServiceTemplateList),
	Other(Response)
}
/// Get a list of service template objects.
pub fn ipam_service_templates_list(state: &ThanixClient, query: IpamServiceTemplatesListQuery) -> Result<IpamServiceTemplatesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/service-templates/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServiceTemplatesListResponse::Http200(r#response.json::<PaginatedServiceTemplateList>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesBulkUpdateResponse {
	Http200(Vec<ServiceTemplate>),
	Other(Response)
}
/// Put a list of service template objects.
pub fn ipam_service_templates_bulk_update(state: &ThanixClient, body: Vec<ServiceTemplateRequest>) -> Result<IpamServiceTemplatesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/service-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServiceTemplatesBulkUpdateResponse::Http200(r#response.json::<Vec<ServiceTemplate>>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesCreateResponse {
	Http201(ServiceTemplate),
	Other(Response)
}
/// Post a list of service template objects.
pub fn ipam_service_templates_create(state: &ThanixClient, body: WritableServiceTemplateRequest) -> Result<IpamServiceTemplatesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/service-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamServiceTemplatesCreateResponse::Http201(r#response.json::<ServiceTemplate>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of service template objects.
pub fn ipam_service_templates_bulk_destroy(state: &ThanixClient, body: Vec<ServiceTemplateRequest>) -> Result<IpamServiceTemplatesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/service-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamServiceTemplatesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesBulkPartialUpdateResponse {
	Http200(Vec<ServiceTemplate>),
	Other(Response)
}
/// Patch a list of service template objects.
pub fn ipam_service_templates_bulk_partial_update(state: &ThanixClient, body: Vec<ServiceTemplateRequest>) -> Result<IpamServiceTemplatesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/service-templates/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServiceTemplatesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ServiceTemplate>>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesRetrieveResponse {
	Http200(ServiceTemplate),
	Other(Response)
}
/// Get a service template object.
pub fn ipam_service_templates_retrieve(state: &ThanixClient, id: i64) -> Result<IpamServiceTemplatesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/service-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServiceTemplatesRetrieveResponse::Http200(r#response.json::<ServiceTemplate>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesUpdateResponse {
	Http200(ServiceTemplate),
	Other(Response)
}
/// Put a service template object.
pub fn ipam_service_templates_update(state: &ThanixClient, body: WritableServiceTemplateRequest, id: i64) -> Result<IpamServiceTemplatesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/service-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServiceTemplatesUpdateResponse::Http200(r#response.json::<ServiceTemplate>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a service template object.
pub fn ipam_service_templates_destroy(state: &ThanixClient, id: i64) -> Result<IpamServiceTemplatesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/service-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamServiceTemplatesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServiceTemplatesPartialUpdateResponse {
	Http200(ServiceTemplate),
	Other(Response)
}
/// Patch a service template object.
pub fn ipam_service_templates_partial_update(state: &ThanixClient, body: PatchedWritableServiceTemplateRequest, id: i64) -> Result<IpamServiceTemplatesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/service-templates/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServiceTemplatesPartialUpdateResponse::Http200(r#response.json::<ServiceTemplate>()?)) },
		r#other_status => { Ok(IpamServiceTemplatesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamServicesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device (name)
	pub device: Option<Vec<Option<String>>>,
	/// Device (name)
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<Option<i64>>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// IP address
	pub ipaddress: Option<Vec<String>>,
	/// IP address
	pub ipaddress__n: Option<Vec<String>>,
	/// IP address (ID)
	pub ipaddress_id: Option<Vec<i64>>,
	/// IP address (ID)
	pub ipaddress_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub port: Option<f64>,
	pub protocol: Option<String>,
	pub protocol__n: Option<String>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual machine (name)
	pub virtual_machine: Option<Vec<String>>,
	/// Virtual machine (name)
	pub virtual_machine__n: Option<Vec<String>>,
	/// Virtual machine (ID)
	pub virtual_machine_id: Option<Vec<Option<i64>>>,
	/// Virtual machine (ID)
	pub virtual_machine_id__n: Option<Vec<Option<i64>>>,

}
#[derive(Debug)]
pub enum IpamServicesListResponse {
	Http200(PaginatedServiceList),
	Other(Response)
}
/// Get a list of service objects.
pub fn ipam_services_list(state: &ThanixClient, query: IpamServicesListQuery) -> Result<IpamServicesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/services/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServicesListResponse::Http200(r#response.json::<PaginatedServiceList>()?)) },
		r#other_status => { Ok(IpamServicesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesBulkUpdateResponse {
	Http200(Vec<Service>),
	Other(Response)
}
/// Put a list of service objects.
pub fn ipam_services_bulk_update(state: &ThanixClient, body: Vec<ServiceRequest>) -> Result<IpamServicesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/services/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServicesBulkUpdateResponse::Http200(r#response.json::<Vec<Service>>()?)) },
		r#other_status => { Ok(IpamServicesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesCreateResponse {
	Http201(Service),
	Other(Response)
}
/// Post a list of service objects.
pub fn ipam_services_create(state: &ThanixClient, body: WritableServiceRequest) -> Result<IpamServicesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/services/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamServicesCreateResponse::Http201(r#response.json::<Service>()?)) },
		r#other_status => { Ok(IpamServicesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of service objects.
pub fn ipam_services_bulk_destroy(state: &ThanixClient, body: Vec<ServiceRequest>) -> Result<IpamServicesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/services/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamServicesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesBulkPartialUpdateResponse {
	Http200(Vec<Service>),
	Other(Response)
}
/// Patch a list of service objects.
pub fn ipam_services_bulk_partial_update(state: &ThanixClient, body: Vec<ServiceRequest>) -> Result<IpamServicesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/services/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServicesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Service>>()?)) },
		r#other_status => { Ok(IpamServicesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesRetrieveResponse {
	Http200(Service),
	Other(Response)
}
/// Get a service object.
pub fn ipam_services_retrieve(state: &ThanixClient, id: i64) -> Result<IpamServicesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/services/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServicesRetrieveResponse::Http200(r#response.json::<Service>()?)) },
		r#other_status => { Ok(IpamServicesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesUpdateResponse {
	Http200(Service),
	Other(Response)
}
/// Put a service object.
pub fn ipam_services_update(state: &ThanixClient, body: WritableServiceRequest, id: i64) -> Result<IpamServicesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/services/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServicesUpdateResponse::Http200(r#response.json::<Service>()?)) },
		r#other_status => { Ok(IpamServicesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a service object.
pub fn ipam_services_destroy(state: &ThanixClient, id: i64) -> Result<IpamServicesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/services/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamServicesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamServicesPartialUpdateResponse {
	Http200(Service),
	Other(Response)
}
/// Patch a service object.
pub fn ipam_services_partial_update(state: &ThanixClient, body: PatchedWritableServiceRequest, id: i64) -> Result<IpamServicesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/services/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamServicesPartialUpdateResponse::Http200(r#response.json::<Service>()?)) },
		r#other_status => { Ok(IpamServicesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamVlanGroupsListQuery {
	pub cluster: Option<i64>,
	pub clustergroup: Option<f64>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub location: Option<i64>,
	pub max_vid: Option<Vec<i64>>,
	pub max_vid__empty: Option<bool>,
	pub max_vid__gt: Option<Vec<i64>>,
	pub max_vid__gte: Option<Vec<i64>>,
	pub max_vid__lt: Option<Vec<i64>>,
	pub max_vid__lte: Option<Vec<i64>>,
	pub max_vid__n: Option<Vec<i64>>,
	pub min_vid: Option<Vec<i64>>,
	pub min_vid__empty: Option<bool>,
	pub min_vid__gt: Option<Vec<i64>>,
	pub min_vid__gte: Option<Vec<i64>>,
	pub min_vid__lt: Option<Vec<i64>>,
	pub min_vid__lte: Option<Vec<i64>>,
	pub min_vid__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub rack: Option<i64>,
	pub region: Option<i64>,
	pub scope_id: Option<Vec<i64>>,
	pub scope_id__empty: Option<bool>,
	pub scope_id__gt: Option<Vec<i64>>,
	pub scope_id__gte: Option<Vec<i64>>,
	pub scope_id__lt: Option<Vec<i64>>,
	pub scope_id__lte: Option<Vec<i64>>,
	pub scope_id__n: Option<Vec<i64>>,
	pub scope_type: Option<String>,
	pub scope_type__n: Option<String>,
	pub site: Option<i64>,
	pub sitegroup: Option<f64>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamVlanGroupsListResponse {
	Http200(PaginatedVLANGroupList),
	Other(Response)
}
/// Get a list of VLAN group objects.
pub fn ipam_vlan_groups_list(state: &ThanixClient, query: IpamVlanGroupsListQuery) -> Result<IpamVlanGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/vlan-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsListResponse::Http200(r#response.json::<PaginatedVLANGroupList>()?)) },
		r#other_status => { Ok(IpamVlanGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsBulkUpdateResponse {
	Http200(Vec<VLANGroup>),
	Other(Response)
}
/// Put a list of VLAN group objects.
pub fn ipam_vlan_groups_bulk_update(state: &ThanixClient, body: Vec<VLANGroupRequest>) -> Result<IpamVlanGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/vlan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<VLANGroup>>()?)) },
		r#other_status => { Ok(IpamVlanGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsCreateResponse {
	Http201(VLANGroup),
	Other(Response)
}
/// Post a list of VLAN group objects.
pub fn ipam_vlan_groups_create(state: &ThanixClient, body: VLANGroupRequest) -> Result<IpamVlanGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/vlan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamVlanGroupsCreateResponse::Http201(r#response.json::<VLANGroup>()?)) },
		r#other_status => { Ok(IpamVlanGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of VLAN group objects.
pub fn ipam_vlan_groups_bulk_destroy(state: &ThanixClient, body: Vec<VLANGroupRequest>) -> Result<IpamVlanGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/vlan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamVlanGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsBulkPartialUpdateResponse {
	Http200(Vec<VLANGroup>),
	Other(Response)
}
/// Patch a list of VLAN group objects.
pub fn ipam_vlan_groups_bulk_partial_update(state: &ThanixClient, body: Vec<VLANGroupRequest>) -> Result<IpamVlanGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/vlan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VLANGroup>>()?)) },
		r#other_status => { Ok(IpamVlanGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsRetrieveResponse {
	Http200(VLANGroup),
	Other(Response)
}
/// Get a VLAN group object.
pub fn ipam_vlan_groups_retrieve(state: &ThanixClient, id: i64) -> Result<IpamVlanGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/vlan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsRetrieveResponse::Http200(r#response.json::<VLANGroup>()?)) },
		r#other_status => { Ok(IpamVlanGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsUpdateResponse {
	Http200(VLANGroup),
	Other(Response)
}
/// Put a VLAN group object.
pub fn ipam_vlan_groups_update(state: &ThanixClient, body: VLANGroupRequest, id: i64) -> Result<IpamVlanGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/vlan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsUpdateResponse::Http200(r#response.json::<VLANGroup>()?)) },
		r#other_status => { Ok(IpamVlanGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a VLAN group object.
pub fn ipam_vlan_groups_destroy(state: &ThanixClient, id: i64) -> Result<IpamVlanGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/vlan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamVlanGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsPartialUpdateResponse {
	Http200(VLANGroup),
	Other(Response)
}
/// Patch a VLAN group object.
pub fn ipam_vlan_groups_partial_update(state: &ThanixClient, body: PatchedVLANGroupRequest, id: i64) -> Result<IpamVlanGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/vlan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsPartialUpdateResponse::Http200(r#response.json::<VLANGroup>()?)) },
		r#other_status => { Ok(IpamVlanGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsAvailableVlansListResponse {
	Http200(Vec<AvailableVLAN>),
	Other(Response)
}
/// Get a VLAN object.
pub fn ipam_vlan_groups_available_vlans_list(state: &ThanixClient, id: i64) -> Result<IpamVlanGroupsAvailableVlansListResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/vlan-groups/{id}/available-vlans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlanGroupsAvailableVlansListResponse::Http200(r#response.json::<Vec<AvailableVLAN>>()?)) },
		r#other_status => { Ok(IpamVlanGroupsAvailableVlansListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlanGroupsAvailableVlansCreateResponse {
	Http201(Vec<VLAN>),
	Other(Response)
}
/// Post a VLAN object.
pub fn ipam_vlan_groups_available_vlans_create(state: &ThanixClient, body: Vec<VLANRequest>, id: i64) -> Result<IpamVlanGroupsAvailableVlansCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/vlan-groups/{id}/available-vlans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamVlanGroupsAvailableVlansCreateResponse::Http201(r#response.json::<Vec<VLAN>>()?)) },
		r#other_status => { Ok(IpamVlanGroupsAvailableVlansCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamVlansListQuery {
	pub available_at_site: Option<String>,
	pub available_on_device: Option<String>,
	pub available_on_virtualmachine: Option<String>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Group
	pub group: Option<Vec<String>>,
	/// Group
	pub group__n: Option<Vec<String>>,
	/// Group (ID)
	pub group_id: Option<Vec<Option<i64>>>,
	/// Group (ID)
	pub group_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// L2VPN
	pub l2vpn: Option<Vec<Option<i64>>>,
	/// L2VPN
	pub l2vpn__n: Option<Vec<Option<i64>>>,
	/// L2VPN (ID)
	pub l2vpn_id: Option<Vec<i64>>,
	/// L2VPN (ID)
	pub l2vpn_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<Option<i64>>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<Option<i64>>>,
	/// Operational status of this VLAN
	pub status: Option<Vec<String>>,
	/// Operational status of this VLAN
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	pub vid: Option<Vec<i64>>,
	pub vid__empty: Option<bool>,
	pub vid__gt: Option<Vec<i64>>,
	pub vid__gte: Option<Vec<i64>>,
	pub vid__lt: Option<Vec<i64>>,
	pub vid__lte: Option<Vec<i64>>,
	pub vid__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum IpamVlansListResponse {
	Http200(PaginatedVLANList),
	Other(Response)
}
/// Get a list of VLAN objects.
pub fn ipam_vlans_list(state: &ThanixClient, query: IpamVlansListQuery) -> Result<IpamVlansListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/vlans/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlansListResponse::Http200(r#response.json::<PaginatedVLANList>()?)) },
		r#other_status => { Ok(IpamVlansListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansBulkUpdateResponse {
	Http200(Vec<VLAN>),
	Other(Response)
}
/// Put a list of VLAN objects.
pub fn ipam_vlans_bulk_update(state: &ThanixClient, body: Vec<VLANRequest>) -> Result<IpamVlansBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/vlans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlansBulkUpdateResponse::Http200(r#response.json::<Vec<VLAN>>()?)) },
		r#other_status => { Ok(IpamVlansBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansCreateResponse {
	Http201(VLAN),
	Other(Response)
}
/// Post a list of VLAN objects.
pub fn ipam_vlans_create(state: &ThanixClient, body: WritableVLANRequest) -> Result<IpamVlansCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/vlans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamVlansCreateResponse::Http201(r#response.json::<VLAN>()?)) },
		r#other_status => { Ok(IpamVlansCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of VLAN objects.
pub fn ipam_vlans_bulk_destroy(state: &ThanixClient, body: Vec<VLANRequest>) -> Result<IpamVlansBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/vlans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamVlansBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansBulkPartialUpdateResponse {
	Http200(Vec<VLAN>),
	Other(Response)
}
/// Patch a list of VLAN objects.
pub fn ipam_vlans_bulk_partial_update(state: &ThanixClient, body: Vec<VLANRequest>) -> Result<IpamVlansBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/vlans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlansBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VLAN>>()?)) },
		r#other_status => { Ok(IpamVlansBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansRetrieveResponse {
	Http200(VLAN),
	Other(Response)
}
/// Get a VLAN object.
pub fn ipam_vlans_retrieve(state: &ThanixClient, id: i64) -> Result<IpamVlansRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/vlans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlansRetrieveResponse::Http200(r#response.json::<VLAN>()?)) },
		r#other_status => { Ok(IpamVlansRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansUpdateResponse {
	Http200(VLAN),
	Other(Response)
}
/// Put a VLAN object.
pub fn ipam_vlans_update(state: &ThanixClient, body: WritableVLANRequest, id: i64) -> Result<IpamVlansUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/vlans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlansUpdateResponse::Http200(r#response.json::<VLAN>()?)) },
		r#other_status => { Ok(IpamVlansUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a VLAN object.
pub fn ipam_vlans_destroy(state: &ThanixClient, id: i64) -> Result<IpamVlansDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/vlans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamVlansDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVlansPartialUpdateResponse {
	Http200(VLAN),
	Other(Response)
}
/// Patch a VLAN object.
pub fn ipam_vlans_partial_update(state: &ThanixClient, body: PatchedWritableVLANRequest, id: i64) -> Result<IpamVlansPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/vlans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVlansPartialUpdateResponse::Http200(r#response.json::<VLAN>()?)) },
		r#other_status => { Ok(IpamVlansPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpamVrfsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub enforce_unique: Option<bool>,
	/// Export target (name)
	pub export_target: Option<Vec<String>>,
	/// Export target (name)
	pub export_target__n: Option<Vec<String>>,
	/// Export target
	pub export_target_id: Option<Vec<i64>>,
	/// Export target
	pub export_target_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Import target (name)
	pub import_target: Option<Vec<String>>,
	/// Import target (name)
	pub import_target__n: Option<Vec<String>>,
	/// Import target
	pub import_target_id: Option<Vec<i64>>,
	/// Import target
	pub import_target_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub rd: Option<Vec<String>>,
	pub rd__empty: Option<bool>,
	pub rd__ic: Option<Vec<String>>,
	pub rd__ie: Option<Vec<String>>,
	pub rd__iew: Option<Vec<String>>,
	pub rd__isw: Option<Vec<String>>,
	pub rd__n: Option<Vec<String>>,
	pub rd__nic: Option<Vec<String>>,
	pub rd__nie: Option<Vec<String>>,
	pub rd__niew: Option<Vec<String>>,
	pub rd__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum IpamVrfsListResponse {
	Http200(PaginatedVRFList),
	Other(Response)
}
/// Get a list of VRF objects.
pub fn ipam_vrfs_list(state: &ThanixClient, query: IpamVrfsListQuery) -> Result<IpamVrfsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/ipam/vrfs/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVrfsListResponse::Http200(r#response.json::<PaginatedVRFList>()?)) },
		r#other_status => { Ok(IpamVrfsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsBulkUpdateResponse {
	Http200(Vec<VRF>),
	Other(Response)
}
/// Put a list of VRF objects.
pub fn ipam_vrfs_bulk_update(state: &ThanixClient, body: Vec<VRFRequest>) -> Result<IpamVrfsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/vrfs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVrfsBulkUpdateResponse::Http200(r#response.json::<Vec<VRF>>()?)) },
		r#other_status => { Ok(IpamVrfsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsCreateResponse {
	Http201(VRF),
	Other(Response)
}
/// Post a list of VRF objects.
pub fn ipam_vrfs_create(state: &ThanixClient, body: WritableVRFRequest) -> Result<IpamVrfsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/ipam/vrfs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(IpamVrfsCreateResponse::Http201(r#response.json::<VRF>()?)) },
		r#other_status => { Ok(IpamVrfsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of VRF objects.
pub fn ipam_vrfs_bulk_destroy(state: &ThanixClient, body: Vec<VRFRequest>) -> Result<IpamVrfsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/vrfs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamVrfsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsBulkPartialUpdateResponse {
	Http200(Vec<VRF>),
	Other(Response)
}
/// Patch a list of VRF objects.
pub fn ipam_vrfs_bulk_partial_update(state: &ThanixClient, body: Vec<VRFRequest>) -> Result<IpamVrfsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/vrfs/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVrfsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VRF>>()?)) },
		r#other_status => { Ok(IpamVrfsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsRetrieveResponse {
	Http200(VRF),
	Other(Response)
}
/// Get a VRF object.
pub fn ipam_vrfs_retrieve(state: &ThanixClient, id: i64) -> Result<IpamVrfsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/ipam/vrfs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVrfsRetrieveResponse::Http200(r#response.json::<VRF>()?)) },
		r#other_status => { Ok(IpamVrfsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsUpdateResponse {
	Http200(VRF),
	Other(Response)
}
/// Put a VRF object.
pub fn ipam_vrfs_update(state: &ThanixClient, body: WritableVRFRequest, id: i64) -> Result<IpamVrfsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/ipam/vrfs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVrfsUpdateResponse::Http200(r#response.json::<VRF>()?)) },
		r#other_status => { Ok(IpamVrfsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a VRF object.
pub fn ipam_vrfs_destroy(state: &ThanixClient, id: i64) -> Result<IpamVrfsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/ipam/vrfs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(IpamVrfsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum IpamVrfsPartialUpdateResponse {
	Http200(VRF),
	Other(Response)
}
/// Patch a VRF object.
pub fn ipam_vrfs_partial_update(state: &ThanixClient, body: PatchedWritableVRFRequest, id: i64) -> Result<IpamVrfsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/ipam/vrfs/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(IpamVrfsPartialUpdateResponse::Http200(r#response.json::<VRF>()?)) },
		r#other_status => { Ok(IpamVrfsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SchemaRetrieveQuery {
	pub format: Option<String>,

}
#[derive(Debug)]
pub enum SchemaRetrieveResponse {
	Http200(Option<std::collections::HashMap<String, serde_json::Value>>),
	Other(Response)
}
/// OpenApi3 schema for this API. Format can be selected via content negotiation.
/// 
/// - YAML: application/vnd.oai.openapi
/// - JSON: application/vnd.oai.openapi+json
pub fn schema_retrieve(state: &ThanixClient, query: SchemaRetrieveQuery) -> Result<SchemaRetrieveResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/schema/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(SchemaRetrieveResponse::Http200(r#response.json::<Option<std::collections::HashMap<String, serde_json::Value>>>()?)) },
		r#other_status => { Ok(SchemaRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum StatusRetrieveResponse {
	Http200(Option<std::collections::HashMap<String, serde_json::Value>>),
	Other(Response)
}
/// A lightweight read-only endpoint for conveying NetBox's current operational status.
pub fn status_retrieve(state: &ThanixClient) -> Result<StatusRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/status/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(StatusRetrieveResponse::Http200(r#response.json::<Option<std::collections::HashMap<String, serde_json::Value>>>()?)) },
		r#other_status => { Ok(StatusRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenancyContactAssignmentsListQuery {
	/// Contact (ID)
	pub contact_id: Option<Vec<i64>>,
	/// Contact (ID)
	pub contact_id__n: Option<Vec<i64>>,
	pub content_type: Option<String>,
	pub content_type__n: Option<String>,
	pub content_type_id: Option<i64>,
	pub content_type_id__n: Option<i64>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	/// Contact group (slug)
	pub group: Option<Vec<i64>>,
	/// Contact group (slug)
	pub group__n: Option<Vec<i64>>,
	/// Contact group (ID)
	pub group_id: Option<Vec<i64>>,
	/// Contact group (ID)
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub object_id: Option<Vec<i64>>,
	pub object_id__empty: Option<bool>,
	pub object_id__gt: Option<Vec<i64>>,
	pub object_id__gte: Option<Vec<i64>>,
	pub object_id__lt: Option<Vec<i64>>,
	pub object_id__lte: Option<Vec<i64>>,
	pub object_id__n: Option<Vec<i64>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub priority: Option<String>,
	pub priority__n: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Contact role (slug)
	pub role: Option<Vec<String>>,
	/// Contact role (slug)
	pub role__n: Option<Vec<String>>,
	/// Contact role (ID)
	pub role_id: Option<Vec<i64>>,
	/// Contact role (ID)
	pub role_id__n: Option<Vec<i64>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum TenancyContactAssignmentsListResponse {
	Http200(PaginatedContactAssignmentList),
	Other(Response)
}
/// Get a list of contact assignment objects.
pub fn tenancy_contact_assignments_list(state: &ThanixClient, query: TenancyContactAssignmentsListQuery) -> Result<TenancyContactAssignmentsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/tenancy/contact-assignments/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactAssignmentsListResponse::Http200(r#response.json::<PaginatedContactAssignmentList>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsBulkUpdateResponse {
	Http200(Vec<ContactAssignment>),
	Other(Response)
}
/// Put a list of contact assignment objects.
pub fn tenancy_contact_assignments_bulk_update(state: &ThanixClient, body: Vec<ContactAssignmentRequest>) -> Result<TenancyContactAssignmentsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contact-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactAssignmentsBulkUpdateResponse::Http200(r#response.json::<Vec<ContactAssignment>>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsCreateResponse {
	Http201(ContactAssignment),
	Other(Response)
}
/// Post a list of contact assignment objects.
pub fn tenancy_contact_assignments_create(state: &ThanixClient, body: WritableContactAssignmentRequest) -> Result<TenancyContactAssignmentsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/tenancy/contact-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(TenancyContactAssignmentsCreateResponse::Http201(r#response.json::<ContactAssignment>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of contact assignment objects.
pub fn tenancy_contact_assignments_bulk_destroy(state: &ThanixClient, body: Vec<ContactAssignmentRequest>) -> Result<TenancyContactAssignmentsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contact-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactAssignmentsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsBulkPartialUpdateResponse {
	Http200(Vec<ContactAssignment>),
	Other(Response)
}
/// Patch a list of contact assignment objects.
pub fn tenancy_contact_assignments_bulk_partial_update(state: &ThanixClient, body: Vec<ContactAssignmentRequest>) -> Result<TenancyContactAssignmentsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contact-assignments/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactAssignmentsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ContactAssignment>>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsRetrieveResponse {
	Http200(ContactAssignment),
	Other(Response)
}
/// Get a contact assignment object.
pub fn tenancy_contact_assignments_retrieve(state: &ThanixClient, id: i64) -> Result<TenancyContactAssignmentsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/tenancy/contact-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactAssignmentsRetrieveResponse::Http200(r#response.json::<ContactAssignment>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsUpdateResponse {
	Http200(ContactAssignment),
	Other(Response)
}
/// Put a contact assignment object.
pub fn tenancy_contact_assignments_update(state: &ThanixClient, body: WritableContactAssignmentRequest, id: i64) -> Result<TenancyContactAssignmentsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contact-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactAssignmentsUpdateResponse::Http200(r#response.json::<ContactAssignment>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a contact assignment object.
pub fn tenancy_contact_assignments_destroy(state: &ThanixClient, id: i64) -> Result<TenancyContactAssignmentsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contact-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactAssignmentsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactAssignmentsPartialUpdateResponse {
	Http200(ContactAssignment),
	Other(Response)
}
/// Patch a contact assignment object.
pub fn tenancy_contact_assignments_partial_update(state: &ThanixClient, body: PatchedWritableContactAssignmentRequest, id: i64) -> Result<TenancyContactAssignmentsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contact-assignments/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactAssignmentsPartialUpdateResponse::Http200(r#response.json::<ContactAssignment>()?)) },
		r#other_status => { Ok(TenancyContactAssignmentsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenancyContactGroupsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Contact group (slug)
	pub parent: Option<Vec<String>>,
	/// Contact group (slug)
	pub parent__n: Option<Vec<String>>,
	/// Contact group (ID)
	pub parent_id: Option<Vec<Option<i64>>>,
	/// Contact group (ID)
	pub parent_id__n: Option<Vec<Option<i64>>>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum TenancyContactGroupsListResponse {
	Http200(PaginatedContactGroupList),
	Other(Response)
}
/// Get a list of contact group objects.
pub fn tenancy_contact_groups_list(state: &ThanixClient, query: TenancyContactGroupsListQuery) -> Result<TenancyContactGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/tenancy/contact-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactGroupsListResponse::Http200(r#response.json::<PaginatedContactGroupList>()?)) },
		r#other_status => { Ok(TenancyContactGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsBulkUpdateResponse {
	Http200(Vec<ContactGroup>),
	Other(Response)
}
/// Put a list of contact group objects.
pub fn tenancy_contact_groups_bulk_update(state: &ThanixClient, body: Vec<ContactGroupRequest>) -> Result<TenancyContactGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contact-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<ContactGroup>>()?)) },
		r#other_status => { Ok(TenancyContactGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsCreateResponse {
	Http201(ContactGroup),
	Other(Response)
}
/// Post a list of contact group objects.
pub fn tenancy_contact_groups_create(state: &ThanixClient, body: WritableContactGroupRequest) -> Result<TenancyContactGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/tenancy/contact-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(TenancyContactGroupsCreateResponse::Http201(r#response.json::<ContactGroup>()?)) },
		r#other_status => { Ok(TenancyContactGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of contact group objects.
pub fn tenancy_contact_groups_bulk_destroy(state: &ThanixClient, body: Vec<ContactGroupRequest>) -> Result<TenancyContactGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contact-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsBulkPartialUpdateResponse {
	Http200(Vec<ContactGroup>),
	Other(Response)
}
/// Patch a list of contact group objects.
pub fn tenancy_contact_groups_bulk_partial_update(state: &ThanixClient, body: Vec<ContactGroupRequest>) -> Result<TenancyContactGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contact-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ContactGroup>>()?)) },
		r#other_status => { Ok(TenancyContactGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsRetrieveResponse {
	Http200(ContactGroup),
	Other(Response)
}
/// Get a contact group object.
pub fn tenancy_contact_groups_retrieve(state: &ThanixClient, id: i64) -> Result<TenancyContactGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/tenancy/contact-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactGroupsRetrieveResponse::Http200(r#response.json::<ContactGroup>()?)) },
		r#other_status => { Ok(TenancyContactGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsUpdateResponse {
	Http200(ContactGroup),
	Other(Response)
}
/// Put a contact group object.
pub fn tenancy_contact_groups_update(state: &ThanixClient, body: WritableContactGroupRequest, id: i64) -> Result<TenancyContactGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contact-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactGroupsUpdateResponse::Http200(r#response.json::<ContactGroup>()?)) },
		r#other_status => { Ok(TenancyContactGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a contact group object.
pub fn tenancy_contact_groups_destroy(state: &ThanixClient, id: i64) -> Result<TenancyContactGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contact-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactGroupsPartialUpdateResponse {
	Http200(ContactGroup),
	Other(Response)
}
/// Patch a contact group object.
pub fn tenancy_contact_groups_partial_update(state: &ThanixClient, body: PatchedWritableContactGroupRequest, id: i64) -> Result<TenancyContactGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contact-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactGroupsPartialUpdateResponse::Http200(r#response.json::<ContactGroup>()?)) },
		r#other_status => { Ok(TenancyContactGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenancyContactRolesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum TenancyContactRolesListResponse {
	Http200(PaginatedContactRoleList),
	Other(Response)
}
/// Get a list of contact role objects.
pub fn tenancy_contact_roles_list(state: &ThanixClient, query: TenancyContactRolesListQuery) -> Result<TenancyContactRolesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/tenancy/contact-roles/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactRolesListResponse::Http200(r#response.json::<PaginatedContactRoleList>()?)) },
		r#other_status => { Ok(TenancyContactRolesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesBulkUpdateResponse {
	Http200(Vec<ContactRole>),
	Other(Response)
}
/// Put a list of contact role objects.
pub fn tenancy_contact_roles_bulk_update(state: &ThanixClient, body: Vec<ContactRoleRequest>) -> Result<TenancyContactRolesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contact-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactRolesBulkUpdateResponse::Http200(r#response.json::<Vec<ContactRole>>()?)) },
		r#other_status => { Ok(TenancyContactRolesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesCreateResponse {
	Http201(ContactRole),
	Other(Response)
}
/// Post a list of contact role objects.
pub fn tenancy_contact_roles_create(state: &ThanixClient, body: ContactRoleRequest) -> Result<TenancyContactRolesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/tenancy/contact-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(TenancyContactRolesCreateResponse::Http201(r#response.json::<ContactRole>()?)) },
		r#other_status => { Ok(TenancyContactRolesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of contact role objects.
pub fn tenancy_contact_roles_bulk_destroy(state: &ThanixClient, body: Vec<ContactRoleRequest>) -> Result<TenancyContactRolesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contact-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactRolesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesBulkPartialUpdateResponse {
	Http200(Vec<ContactRole>),
	Other(Response)
}
/// Patch a list of contact role objects.
pub fn tenancy_contact_roles_bulk_partial_update(state: &ThanixClient, body: Vec<ContactRoleRequest>) -> Result<TenancyContactRolesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contact-roles/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactRolesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ContactRole>>()?)) },
		r#other_status => { Ok(TenancyContactRolesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesRetrieveResponse {
	Http200(ContactRole),
	Other(Response)
}
/// Get a contact role object.
pub fn tenancy_contact_roles_retrieve(state: &ThanixClient, id: i64) -> Result<TenancyContactRolesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/tenancy/contact-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactRolesRetrieveResponse::Http200(r#response.json::<ContactRole>()?)) },
		r#other_status => { Ok(TenancyContactRolesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesUpdateResponse {
	Http200(ContactRole),
	Other(Response)
}
/// Put a contact role object.
pub fn tenancy_contact_roles_update(state: &ThanixClient, body: ContactRoleRequest, id: i64) -> Result<TenancyContactRolesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contact-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactRolesUpdateResponse::Http200(r#response.json::<ContactRole>()?)) },
		r#other_status => { Ok(TenancyContactRolesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a contact role object.
pub fn tenancy_contact_roles_destroy(state: &ThanixClient, id: i64) -> Result<TenancyContactRolesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contact-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactRolesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactRolesPartialUpdateResponse {
	Http200(ContactRole),
	Other(Response)
}
/// Patch a contact role object.
pub fn tenancy_contact_roles_partial_update(state: &ThanixClient, body: PatchedContactRoleRequest, id: i64) -> Result<TenancyContactRolesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contact-roles/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactRolesPartialUpdateResponse::Http200(r#response.json::<ContactRole>()?)) },
		r#other_status => { Ok(TenancyContactRolesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenancyContactsListQuery {
	pub address: Option<Vec<String>>,
	pub address__empty: Option<bool>,
	pub address__ic: Option<Vec<String>>,
	pub address__ie: Option<Vec<String>>,
	pub address__iew: Option<Vec<String>>,
	pub address__isw: Option<Vec<String>>,
	pub address__n: Option<Vec<String>>,
	pub address__nic: Option<Vec<String>>,
	pub address__nie: Option<Vec<String>>,
	pub address__niew: Option<Vec<String>>,
	pub address__nisw: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub email: Option<Vec<String>>,
	pub email__empty: Option<bool>,
	pub email__ic: Option<Vec<String>>,
	pub email__ie: Option<Vec<String>>,
	pub email__iew: Option<Vec<String>>,
	pub email__isw: Option<Vec<String>>,
	pub email__n: Option<Vec<String>>,
	pub email__nic: Option<Vec<String>>,
	pub email__nie: Option<Vec<String>>,
	pub email__niew: Option<Vec<String>>,
	pub email__nisw: Option<Vec<String>>,
	/// Contact group (slug)
	pub group: Option<Vec<i64>>,
	/// Contact group (slug)
	pub group__n: Option<Vec<i64>>,
	/// Contact group (ID)
	pub group_id: Option<Vec<i64>>,
	/// Contact group (ID)
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub link: Option<Vec<String>>,
	pub link__empty: Option<bool>,
	pub link__ic: Option<Vec<String>>,
	pub link__ie: Option<Vec<String>>,
	pub link__iew: Option<Vec<String>>,
	pub link__isw: Option<Vec<String>>,
	pub link__n: Option<Vec<String>>,
	pub link__nic: Option<Vec<String>>,
	pub link__nie: Option<Vec<String>>,
	pub link__niew: Option<Vec<String>>,
	pub link__nisw: Option<Vec<String>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub phone: Option<Vec<String>>,
	pub phone__empty: Option<bool>,
	pub phone__ic: Option<Vec<String>>,
	pub phone__ie: Option<Vec<String>>,
	pub phone__iew: Option<Vec<String>>,
	pub phone__isw: Option<Vec<String>>,
	pub phone__n: Option<Vec<String>>,
	pub phone__nic: Option<Vec<String>>,
	pub phone__nie: Option<Vec<String>>,
	pub phone__niew: Option<Vec<String>>,
	pub phone__nisw: Option<Vec<String>>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub title: Option<Vec<String>>,
	pub title__empty: Option<bool>,
	pub title__ic: Option<Vec<String>>,
	pub title__ie: Option<Vec<String>>,
	pub title__iew: Option<Vec<String>>,
	pub title__isw: Option<Vec<String>>,
	pub title__n: Option<Vec<String>>,
	pub title__nic: Option<Vec<String>>,
	pub title__nie: Option<Vec<String>>,
	pub title__niew: Option<Vec<String>>,
	pub title__nisw: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum TenancyContactsListResponse {
	Http200(PaginatedContactList),
	Other(Response)
}
/// Get a list of contact objects.
pub fn tenancy_contacts_list(state: &ThanixClient, query: TenancyContactsListQuery) -> Result<TenancyContactsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/tenancy/contacts/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactsListResponse::Http200(r#response.json::<PaginatedContactList>()?)) },
		r#other_status => { Ok(TenancyContactsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsBulkUpdateResponse {
	Http200(Vec<Contact>),
	Other(Response)
}
/// Put a list of contact objects.
pub fn tenancy_contacts_bulk_update(state: &ThanixClient, body: Vec<ContactRequest>) -> Result<TenancyContactsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contacts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactsBulkUpdateResponse::Http200(r#response.json::<Vec<Contact>>()?)) },
		r#other_status => { Ok(TenancyContactsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsCreateResponse {
	Http201(Contact),
	Other(Response)
}
/// Post a list of contact objects.
pub fn tenancy_contacts_create(state: &ThanixClient, body: WritableContactRequest) -> Result<TenancyContactsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/tenancy/contacts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(TenancyContactsCreateResponse::Http201(r#response.json::<Contact>()?)) },
		r#other_status => { Ok(TenancyContactsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of contact objects.
pub fn tenancy_contacts_bulk_destroy(state: &ThanixClient, body: Vec<ContactRequest>) -> Result<TenancyContactsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contacts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsBulkPartialUpdateResponse {
	Http200(Vec<Contact>),
	Other(Response)
}
/// Patch a list of contact objects.
pub fn tenancy_contacts_bulk_partial_update(state: &ThanixClient, body: Vec<ContactRequest>) -> Result<TenancyContactsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contacts/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Contact>>()?)) },
		r#other_status => { Ok(TenancyContactsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsRetrieveResponse {
	Http200(Contact),
	Other(Response)
}
/// Get a contact object.
pub fn tenancy_contacts_retrieve(state: &ThanixClient, id: i64) -> Result<TenancyContactsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/tenancy/contacts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactsRetrieveResponse::Http200(r#response.json::<Contact>()?)) },
		r#other_status => { Ok(TenancyContactsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsUpdateResponse {
	Http200(Contact),
	Other(Response)
}
/// Put a contact object.
pub fn tenancy_contacts_update(state: &ThanixClient, body: WritableContactRequest, id: i64) -> Result<TenancyContactsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/contacts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactsUpdateResponse::Http200(r#response.json::<Contact>()?)) },
		r#other_status => { Ok(TenancyContactsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a contact object.
pub fn tenancy_contacts_destroy(state: &ThanixClient, id: i64) -> Result<TenancyContactsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/contacts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyContactsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyContactsPartialUpdateResponse {
	Http200(Contact),
	Other(Response)
}
/// Patch a contact object.
pub fn tenancy_contacts_partial_update(state: &ThanixClient, body: PatchedWritableContactRequest, id: i64) -> Result<TenancyContactsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/contacts/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyContactsPartialUpdateResponse::Http200(r#response.json::<Contact>()?)) },
		r#other_status => { Ok(TenancyContactsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenancyTenantGroupsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Tenant group (slug)
	pub parent: Option<Vec<String>>,
	/// Tenant group (slug)
	pub parent__n: Option<Vec<String>>,
	/// Tenant group (ID)
	pub parent_id: Option<Vec<Option<i64>>>,
	/// Tenant group (ID)
	pub parent_id__n: Option<Vec<Option<i64>>>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum TenancyTenantGroupsListResponse {
	Http200(PaginatedTenantGroupList),
	Other(Response)
}
/// Get a list of tenant group objects.
pub fn tenancy_tenant_groups_list(state: &ThanixClient, query: TenancyTenantGroupsListQuery) -> Result<TenancyTenantGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/tenancy/tenant-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantGroupsListResponse::Http200(r#response.json::<PaginatedTenantGroupList>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsBulkUpdateResponse {
	Http200(Vec<TenantGroup>),
	Other(Response)
}
/// Put a list of tenant group objects.
pub fn tenancy_tenant_groups_bulk_update(state: &ThanixClient, body: Vec<TenantGroupRequest>) -> Result<TenancyTenantGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/tenant-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<TenantGroup>>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsCreateResponse {
	Http201(TenantGroup),
	Other(Response)
}
/// Post a list of tenant group objects.
pub fn tenancy_tenant_groups_create(state: &ThanixClient, body: WritableTenantGroupRequest) -> Result<TenancyTenantGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/tenancy/tenant-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(TenancyTenantGroupsCreateResponse::Http201(r#response.json::<TenantGroup>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of tenant group objects.
pub fn tenancy_tenant_groups_bulk_destroy(state: &ThanixClient, body: Vec<TenantGroupRequest>) -> Result<TenancyTenantGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/tenant-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyTenantGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsBulkPartialUpdateResponse {
	Http200(Vec<TenantGroup>),
	Other(Response)
}
/// Patch a list of tenant group objects.
pub fn tenancy_tenant_groups_bulk_partial_update(state: &ThanixClient, body: Vec<TenantGroupRequest>) -> Result<TenancyTenantGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/tenant-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<TenantGroup>>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsRetrieveResponse {
	Http200(TenantGroup),
	Other(Response)
}
/// Get a tenant group object.
pub fn tenancy_tenant_groups_retrieve(state: &ThanixClient, id: i64) -> Result<TenancyTenantGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/tenancy/tenant-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantGroupsRetrieveResponse::Http200(r#response.json::<TenantGroup>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsUpdateResponse {
	Http200(TenantGroup),
	Other(Response)
}
/// Put a tenant group object.
pub fn tenancy_tenant_groups_update(state: &ThanixClient, body: WritableTenantGroupRequest, id: i64) -> Result<TenancyTenantGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/tenant-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantGroupsUpdateResponse::Http200(r#response.json::<TenantGroup>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a tenant group object.
pub fn tenancy_tenant_groups_destroy(state: &ThanixClient, id: i64) -> Result<TenancyTenantGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/tenant-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyTenantGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantGroupsPartialUpdateResponse {
	Http200(TenantGroup),
	Other(Response)
}
/// Patch a tenant group object.
pub fn tenancy_tenant_groups_partial_update(state: &ThanixClient, body: PatchedWritableTenantGroupRequest, id: i64) -> Result<TenancyTenantGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/tenant-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantGroupsPartialUpdateResponse::Http200(r#response.json::<TenantGroup>()?)) },
		r#other_status => { Ok(TenancyTenantGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TenancyTenantsListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Tenant group (slug)
	pub group: Option<Vec<i64>>,
	/// Tenant group (slug)
	pub group__n: Option<Vec<i64>>,
	/// Tenant group (ID)
	pub group_id: Option<Vec<i64>>,
	/// Tenant group (ID)
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum TenancyTenantsListResponse {
	Http200(PaginatedTenantList),
	Other(Response)
}
/// Get a list of tenant objects.
pub fn tenancy_tenants_list(state: &ThanixClient, query: TenancyTenantsListQuery) -> Result<TenancyTenantsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/tenancy/tenants/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantsListResponse::Http200(r#response.json::<PaginatedTenantList>()?)) },
		r#other_status => { Ok(TenancyTenantsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsBulkUpdateResponse {
	Http200(Vec<Tenant>),
	Other(Response)
}
/// Put a list of tenant objects.
pub fn tenancy_tenants_bulk_update(state: &ThanixClient, body: Vec<TenantRequest>) -> Result<TenancyTenantsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/tenants/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantsBulkUpdateResponse::Http200(r#response.json::<Vec<Tenant>>()?)) },
		r#other_status => { Ok(TenancyTenantsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsCreateResponse {
	Http201(Tenant),
	Other(Response)
}
/// Post a list of tenant objects.
pub fn tenancy_tenants_create(state: &ThanixClient, body: WritableTenantRequest) -> Result<TenancyTenantsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/tenancy/tenants/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(TenancyTenantsCreateResponse::Http201(r#response.json::<Tenant>()?)) },
		r#other_status => { Ok(TenancyTenantsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of tenant objects.
pub fn tenancy_tenants_bulk_destroy(state: &ThanixClient, body: Vec<TenantRequest>) -> Result<TenancyTenantsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/tenants/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyTenantsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsBulkPartialUpdateResponse {
	Http200(Vec<Tenant>),
	Other(Response)
}
/// Patch a list of tenant objects.
pub fn tenancy_tenants_bulk_partial_update(state: &ThanixClient, body: Vec<TenantRequest>) -> Result<TenancyTenantsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/tenants/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Tenant>>()?)) },
		r#other_status => { Ok(TenancyTenantsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsRetrieveResponse {
	Http200(Tenant),
	Other(Response)
}
/// Get a tenant object.
pub fn tenancy_tenants_retrieve(state: &ThanixClient, id: i64) -> Result<TenancyTenantsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/tenancy/tenants/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantsRetrieveResponse::Http200(r#response.json::<Tenant>()?)) },
		r#other_status => { Ok(TenancyTenantsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsUpdateResponse {
	Http200(Tenant),
	Other(Response)
}
/// Put a tenant object.
pub fn tenancy_tenants_update(state: &ThanixClient, body: WritableTenantRequest, id: i64) -> Result<TenancyTenantsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/tenancy/tenants/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantsUpdateResponse::Http200(r#response.json::<Tenant>()?)) },
		r#other_status => { Ok(TenancyTenantsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a tenant object.
pub fn tenancy_tenants_destroy(state: &ThanixClient, id: i64) -> Result<TenancyTenantsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/tenancy/tenants/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(TenancyTenantsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum TenancyTenantsPartialUpdateResponse {
	Http200(Tenant),
	Other(Response)
}
/// Patch a tenant object.
pub fn tenancy_tenants_partial_update(state: &ThanixClient, body: PatchedWritableTenantRequest, id: i64) -> Result<TenancyTenantsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/tenancy/tenants/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(TenancyTenantsPartialUpdateResponse::Http200(r#response.json::<Tenant>()?)) },
		r#other_status => { Ok(TenancyTenantsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersConfigRetrieveResponse {
	Http200(Option<std::collections::HashMap<String, serde_json::Value>>),
	Other(Response)
}
/// An API endpoint via which a user can update his or her own UserConfig data (but no one else's).
pub fn users_config_retrieve(state: &ThanixClient) -> Result<UsersConfigRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/users/config/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersConfigRetrieveResponse::Http200(r#response.json::<Option<std::collections::HashMap<String, serde_json::Value>>>()?)) },
		r#other_status => { Ok(UsersConfigRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UsersGroupsListQuery {
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,

}
#[derive(Debug)]
pub enum UsersGroupsListResponse {
	Http200(PaginatedGroupList),
	Other(Response)
}
/// Get a list of group objects.
pub fn users_groups_list(state: &ThanixClient, query: UsersGroupsListQuery) -> Result<UsersGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/users/groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersGroupsListResponse::Http200(r#response.json::<PaginatedGroupList>()?)) },
		r#other_status => { Ok(UsersGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsBulkUpdateResponse {
	Http200(Vec<Group>),
	Other(Response)
}
/// Put a list of group objects.
pub fn users_groups_bulk_update(state: &ThanixClient, body: Vec<GroupRequest>) -> Result<UsersGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<Group>>()?)) },
		r#other_status => { Ok(UsersGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsCreateResponse {
	Http201(Group),
	Other(Response)
}
/// Post a list of group objects.
pub fn users_groups_create(state: &ThanixClient, body: GroupRequest) -> Result<UsersGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/users/groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(UsersGroupsCreateResponse::Http201(r#response.json::<Group>()?)) },
		r#other_status => { Ok(UsersGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of group objects.
pub fn users_groups_bulk_destroy(state: &ThanixClient, body: Vec<GroupRequest>) -> Result<UsersGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsBulkPartialUpdateResponse {
	Http200(Vec<Group>),
	Other(Response)
}
/// Patch a list of group objects.
pub fn users_groups_bulk_partial_update(state: &ThanixClient, body: Vec<GroupRequest>) -> Result<UsersGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Group>>()?)) },
		r#other_status => { Ok(UsersGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsRetrieveResponse {
	Http200(Group),
	Other(Response)
}
/// Get a group object.
pub fn users_groups_retrieve(state: &ThanixClient, id: i64) -> Result<UsersGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/users/groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersGroupsRetrieveResponse::Http200(r#response.json::<Group>()?)) },
		r#other_status => { Ok(UsersGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsUpdateResponse {
	Http200(Group),
	Other(Response)
}
/// Put a group object.
pub fn users_groups_update(state: &ThanixClient, body: GroupRequest, id: i64) -> Result<UsersGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersGroupsUpdateResponse::Http200(r#response.json::<Group>()?)) },
		r#other_status => { Ok(UsersGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a group object.
pub fn users_groups_destroy(state: &ThanixClient, id: i64) -> Result<UsersGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersGroupsPartialUpdateResponse {
	Http200(Group),
	Other(Response)
}
/// Patch a group object.
pub fn users_groups_partial_update(state: &ThanixClient, body: PatchedGroupRequest, id: i64) -> Result<UsersGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersGroupsPartialUpdateResponse::Http200(r#response.json::<Group>()?)) },
		r#other_status => { Ok(UsersGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UsersPermissionsListQuery {
	pub can_add: Option<bool>,
	pub can_change: Option<bool>,
	pub can_delete: Option<bool>,
	pub can_view: Option<bool>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub enabled: Option<bool>,
	/// Group (name)
	pub group: Option<Vec<String>>,
	/// Group (name)
	pub group__n: Option<Vec<String>>,
	/// Group
	pub group_id: Option<Vec<i64>>,
	/// Group
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	pub object_types: Option<Vec<i64>>,
	pub object_types__n: Option<Vec<i64>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// User (name)
	pub user: Option<Vec<String>>,
	/// User (name)
	pub user__n: Option<Vec<String>>,
	/// User
	pub user_id: Option<Vec<i64>>,
	/// User
	pub user_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum UsersPermissionsListResponse {
	Http200(PaginatedObjectPermissionList),
	Other(Response)
}
/// Get a list of permission objects.
pub fn users_permissions_list(state: &ThanixClient, query: UsersPermissionsListQuery) -> Result<UsersPermissionsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/users/permissions/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersPermissionsListResponse::Http200(r#response.json::<PaginatedObjectPermissionList>()?)) },
		r#other_status => { Ok(UsersPermissionsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsBulkUpdateResponse {
	Http200(Vec<ObjectPermission>),
	Other(Response)
}
/// Put a list of permission objects.
pub fn users_permissions_bulk_update(state: &ThanixClient, body: Vec<ObjectPermissionRequest>) -> Result<UsersPermissionsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/permissions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersPermissionsBulkUpdateResponse::Http200(r#response.json::<Vec<ObjectPermission>>()?)) },
		r#other_status => { Ok(UsersPermissionsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsCreateResponse {
	Http201(ObjectPermission),
	Other(Response)
}
/// Post a list of permission objects.
pub fn users_permissions_create(state: &ThanixClient, body: WritableObjectPermissionRequest) -> Result<UsersPermissionsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/users/permissions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(UsersPermissionsCreateResponse::Http201(r#response.json::<ObjectPermission>()?)) },
		r#other_status => { Ok(UsersPermissionsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of permission objects.
pub fn users_permissions_bulk_destroy(state: &ThanixClient, body: Vec<ObjectPermissionRequest>) -> Result<UsersPermissionsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/permissions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersPermissionsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsBulkPartialUpdateResponse {
	Http200(Vec<ObjectPermission>),
	Other(Response)
}
/// Patch a list of permission objects.
pub fn users_permissions_bulk_partial_update(state: &ThanixClient, body: Vec<ObjectPermissionRequest>) -> Result<UsersPermissionsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/permissions/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersPermissionsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ObjectPermission>>()?)) },
		r#other_status => { Ok(UsersPermissionsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsRetrieveResponse {
	Http200(ObjectPermission),
	Other(Response)
}
/// Get a permission object.
pub fn users_permissions_retrieve(state: &ThanixClient, id: i64) -> Result<UsersPermissionsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/users/permissions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersPermissionsRetrieveResponse::Http200(r#response.json::<ObjectPermission>()?)) },
		r#other_status => { Ok(UsersPermissionsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsUpdateResponse {
	Http200(ObjectPermission),
	Other(Response)
}
/// Put a permission object.
pub fn users_permissions_update(state: &ThanixClient, body: WritableObjectPermissionRequest, id: i64) -> Result<UsersPermissionsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/permissions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersPermissionsUpdateResponse::Http200(r#response.json::<ObjectPermission>()?)) },
		r#other_status => { Ok(UsersPermissionsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a permission object.
pub fn users_permissions_destroy(state: &ThanixClient, id: i64) -> Result<UsersPermissionsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/permissions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersPermissionsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersPermissionsPartialUpdateResponse {
	Http200(ObjectPermission),
	Other(Response)
}
/// Patch a permission object.
pub fn users_permissions_partial_update(state: &ThanixClient, body: PatchedWritableObjectPermissionRequest, id: i64) -> Result<UsersPermissionsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/permissions/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersPermissionsPartialUpdateResponse::Http200(r#response.json::<ObjectPermission>()?)) },
		r#other_status => { Ok(UsersPermissionsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UsersTokensListQuery {
	pub created: Option<String>,
	pub created__gte: Option<String>,
	pub created__lte: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub expires: Option<String>,
	pub expires__gte: Option<String>,
	pub expires__lte: Option<String>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub key: Option<Vec<String>>,
	pub key__empty: Option<bool>,
	pub key__ic: Option<Vec<String>>,
	pub key__ie: Option<Vec<String>>,
	pub key__iew: Option<Vec<String>>,
	pub key__isw: Option<Vec<String>>,
	pub key__n: Option<Vec<String>>,
	pub key__nic: Option<Vec<String>>,
	pub key__nie: Option<Vec<String>>,
	pub key__niew: Option<Vec<String>>,
	pub key__nisw: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// User (name)
	pub user: Option<Vec<String>>,
	/// User (name)
	pub user__n: Option<Vec<String>>,
	/// User
	pub user_id: Option<Vec<i64>>,
	/// User
	pub user_id__n: Option<Vec<i64>>,
	pub write_enabled: Option<bool>,

}
#[derive(Debug)]
pub enum UsersTokensListResponse {
	Http200(PaginatedTokenList),
	Other(Response)
}
/// Get a list of token objects.
pub fn users_tokens_list(state: &ThanixClient, query: UsersTokensListQuery) -> Result<UsersTokensListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/users/tokens/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersTokensListResponse::Http200(r#response.json::<PaginatedTokenList>()?)) },
		r#other_status => { Ok(UsersTokensListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensBulkUpdateResponse {
	Http200(Vec<Token>),
	Other(Response)
}
/// Put a list of token objects.
pub fn users_tokens_bulk_update(state: &ThanixClient, body: Vec<TokenRequest>) -> Result<UsersTokensBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/tokens/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersTokensBulkUpdateResponse::Http200(r#response.json::<Vec<Token>>()?)) },
		r#other_status => { Ok(UsersTokensBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensCreateResponse {
	Http201(Token),
	Other(Response)
}
/// Post a list of token objects.
pub fn users_tokens_create(state: &ThanixClient, body: WritableTokenRequest) -> Result<UsersTokensCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/users/tokens/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(UsersTokensCreateResponse::Http201(r#response.json::<Token>()?)) },
		r#other_status => { Ok(UsersTokensCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of token objects.
pub fn users_tokens_bulk_destroy(state: &ThanixClient, body: Vec<TokenRequest>) -> Result<UsersTokensBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/tokens/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersTokensBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensBulkPartialUpdateResponse {
	Http200(Vec<Token>),
	Other(Response)
}
/// Patch a list of token objects.
pub fn users_tokens_bulk_partial_update(state: &ThanixClient, body: Vec<TokenRequest>) -> Result<UsersTokensBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/tokens/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersTokensBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Token>>()?)) },
		r#other_status => { Ok(UsersTokensBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensRetrieveResponse {
	Http200(Token),
	Other(Response)
}
/// Get a token object.
pub fn users_tokens_retrieve(state: &ThanixClient, id: i64) -> Result<UsersTokensRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/users/tokens/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersTokensRetrieveResponse::Http200(r#response.json::<Token>()?)) },
		r#other_status => { Ok(UsersTokensRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensUpdateResponse {
	Http200(Token),
	Other(Response)
}
/// Put a token object.
pub fn users_tokens_update(state: &ThanixClient, body: WritableTokenRequest, id: i64) -> Result<UsersTokensUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/tokens/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersTokensUpdateResponse::Http200(r#response.json::<Token>()?)) },
		r#other_status => { Ok(UsersTokensUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a token object.
pub fn users_tokens_destroy(state: &ThanixClient, id: i64) -> Result<UsersTokensDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/tokens/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersTokensDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensPartialUpdateResponse {
	Http200(Token),
	Other(Response)
}
/// Patch a token object.
pub fn users_tokens_partial_update(state: &ThanixClient, body: PatchedWritableTokenRequest, id: i64) -> Result<UsersTokensPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/tokens/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersTokensPartialUpdateResponse::Http200(r#response.json::<Token>()?)) },
		r#other_status => { Ok(UsersTokensPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersTokensProvisionCreateResponse {
	Http201(TokenProvision),
	Http401(Option<std::collections::HashMap<String, serde_json::Value>>),
	Other(Response)
}
/// Non-authenticated REST API endpoint via which a user may create a Token.
pub fn users_tokens_provision_create(state: &ThanixClient, body: TokenProvisionRequest) -> Result<UsersTokensProvisionCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/users/tokens/provision/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(UsersTokensProvisionCreateResponse::Http201(r#response.json::<TokenProvision>()?)) },
		401 => { Ok(UsersTokensProvisionCreateResponse::Http401(r#response.json::<Option<std::collections::HashMap<String, serde_json::Value>>>()?)) },
		r#other_status => { Ok(UsersTokensProvisionCreateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UsersUsersListQuery {
	pub email: Option<Vec<String>>,
	pub email__empty: Option<bool>,
	pub email__ic: Option<Vec<String>>,
	pub email__ie: Option<Vec<String>>,
	pub email__iew: Option<Vec<String>>,
	pub email__isw: Option<Vec<String>>,
	pub email__n: Option<Vec<String>>,
	pub email__nic: Option<Vec<String>>,
	pub email__nie: Option<Vec<String>>,
	pub email__niew: Option<Vec<String>>,
	pub email__nisw: Option<Vec<String>>,
	pub first_name: Option<Vec<String>>,
	pub first_name__empty: Option<bool>,
	pub first_name__ic: Option<Vec<String>>,
	pub first_name__ie: Option<Vec<String>>,
	pub first_name__iew: Option<Vec<String>>,
	pub first_name__isw: Option<Vec<String>>,
	pub first_name__n: Option<Vec<String>>,
	pub first_name__nic: Option<Vec<String>>,
	pub first_name__nie: Option<Vec<String>>,
	pub first_name__niew: Option<Vec<String>>,
	pub first_name__nisw: Option<Vec<String>>,
	/// Group (name)
	pub group: Option<Vec<String>>,
	/// Group (name)
	pub group__n: Option<Vec<String>>,
	/// Group
	pub group_id: Option<Vec<i64>>,
	/// Group
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub is_active: Option<bool>,
	pub is_staff: Option<bool>,
	pub is_superuser: Option<bool>,
	pub last_name: Option<Vec<String>>,
	pub last_name__empty: Option<bool>,
	pub last_name__ic: Option<Vec<String>>,
	pub last_name__ie: Option<Vec<String>>,
	pub last_name__iew: Option<Vec<String>>,
	pub last_name__isw: Option<Vec<String>>,
	pub last_name__n: Option<Vec<String>>,
	pub last_name__nic: Option<Vec<String>>,
	pub last_name__nie: Option<Vec<String>>,
	pub last_name__niew: Option<Vec<String>>,
	pub last_name__nisw: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub username: Option<Vec<String>>,
	pub username__empty: Option<bool>,
	pub username__ic: Option<Vec<String>>,
	pub username__ie: Option<Vec<String>>,
	pub username__iew: Option<Vec<String>>,
	pub username__isw: Option<Vec<String>>,
	pub username__n: Option<Vec<String>>,
	pub username__nic: Option<Vec<String>>,
	pub username__nie: Option<Vec<String>>,
	pub username__niew: Option<Vec<String>>,
	pub username__nisw: Option<Vec<String>>,

}
#[derive(Debug)]
pub enum UsersUsersListResponse {
	Http200(PaginatedUserList),
	Other(Response)
}
/// Get a list of user objects.
pub fn users_users_list(state: &ThanixClient, query: UsersUsersListQuery) -> Result<UsersUsersListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/users/users/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersUsersListResponse::Http200(r#response.json::<PaginatedUserList>()?)) },
		r#other_status => { Ok(UsersUsersListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersBulkUpdateResponse {
	Http200(Vec<User>),
	Other(Response)
}
/// Put a list of user objects.
pub fn users_users_bulk_update(state: &ThanixClient, body: Vec<UserRequest>) -> Result<UsersUsersBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/users/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersUsersBulkUpdateResponse::Http200(r#response.json::<Vec<User>>()?)) },
		r#other_status => { Ok(UsersUsersBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersCreateResponse {
	Http201(User),
	Other(Response)
}
/// Post a list of user objects.
pub fn users_users_create(state: &ThanixClient, body: WritableUserRequest) -> Result<UsersUsersCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/users/users/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(UsersUsersCreateResponse::Http201(r#response.json::<User>()?)) },
		r#other_status => { Ok(UsersUsersCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of user objects.
pub fn users_users_bulk_destroy(state: &ThanixClient, body: Vec<UserRequest>) -> Result<UsersUsersBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/users/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersUsersBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersBulkPartialUpdateResponse {
	Http200(Vec<User>),
	Other(Response)
}
/// Patch a list of user objects.
pub fn users_users_bulk_partial_update(state: &ThanixClient, body: Vec<UserRequest>) -> Result<UsersUsersBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/users/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersUsersBulkPartialUpdateResponse::Http200(r#response.json::<Vec<User>>()?)) },
		r#other_status => { Ok(UsersUsersBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersRetrieveResponse {
	Http200(User),
	Other(Response)
}
/// Get a user object.
pub fn users_users_retrieve(state: &ThanixClient, id: i64) -> Result<UsersUsersRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/users/users/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersUsersRetrieveResponse::Http200(r#response.json::<User>()?)) },
		r#other_status => { Ok(UsersUsersRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersUpdateResponse {
	Http200(User),
	Other(Response)
}
/// Put a user object.
pub fn users_users_update(state: &ThanixClient, body: WritableUserRequest, id: i64) -> Result<UsersUsersUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/users/users/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersUsersUpdateResponse::Http200(r#response.json::<User>()?)) },
		r#other_status => { Ok(UsersUsersUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a user object.
pub fn users_users_destroy(state: &ThanixClient, id: i64) -> Result<UsersUsersDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/users/users/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(UsersUsersDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum UsersUsersPartialUpdateResponse {
	Http200(User),
	Other(Response)
}
/// Patch a user object.
pub fn users_users_partial_update(state: &ThanixClient, body: PatchedWritableUserRequest, id: i64) -> Result<UsersUsersPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/users/users/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(UsersUsersPartialUpdateResponse::Http200(r#response.json::<User>()?)) },
		r#other_status => { Ok(UsersUsersPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualizationClusterGroupsListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsListResponse {
	Http200(PaginatedClusterGroupList),
	Other(Response)
}
/// Get a list of cluster group objects.
pub fn virtualization_cluster_groups_list(state: &ThanixClient, query: VirtualizationClusterGroupsListQuery) -> Result<VirtualizationClusterGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/virtualization/cluster-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterGroupsListResponse::Http200(r#response.json::<PaginatedClusterGroupList>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsBulkUpdateResponse {
	Http200(Vec<ClusterGroup>),
	Other(Response)
}
/// Put a list of cluster group objects.
pub fn virtualization_cluster_groups_bulk_update(state: &ThanixClient, body: Vec<ClusterGroupRequest>) -> Result<VirtualizationClusterGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/cluster-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<ClusterGroup>>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsCreateResponse {
	Http201(ClusterGroup),
	Other(Response)
}
/// Post a list of cluster group objects.
pub fn virtualization_cluster_groups_create(state: &ThanixClient, body: ClusterGroupRequest) -> Result<VirtualizationClusterGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/virtualization/cluster-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(VirtualizationClusterGroupsCreateResponse::Http201(r#response.json::<ClusterGroup>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of cluster group objects.
pub fn virtualization_cluster_groups_bulk_destroy(state: &ThanixClient, body: Vec<ClusterGroupRequest>) -> Result<VirtualizationClusterGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/cluster-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationClusterGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsBulkPartialUpdateResponse {
	Http200(Vec<ClusterGroup>),
	Other(Response)
}
/// Patch a list of cluster group objects.
pub fn virtualization_cluster_groups_bulk_partial_update(state: &ThanixClient, body: Vec<ClusterGroupRequest>) -> Result<VirtualizationClusterGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/cluster-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ClusterGroup>>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsRetrieveResponse {
	Http200(ClusterGroup),
	Other(Response)
}
/// Get a cluster group object.
pub fn virtualization_cluster_groups_retrieve(state: &ThanixClient, id: i64) -> Result<VirtualizationClusterGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/virtualization/cluster-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterGroupsRetrieveResponse::Http200(r#response.json::<ClusterGroup>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsUpdateResponse {
	Http200(ClusterGroup),
	Other(Response)
}
/// Put a cluster group object.
pub fn virtualization_cluster_groups_update(state: &ThanixClient, body: ClusterGroupRequest, id: i64) -> Result<VirtualizationClusterGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/cluster-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterGroupsUpdateResponse::Http200(r#response.json::<ClusterGroup>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a cluster group object.
pub fn virtualization_cluster_groups_destroy(state: &ThanixClient, id: i64) -> Result<VirtualizationClusterGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/cluster-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationClusterGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterGroupsPartialUpdateResponse {
	Http200(ClusterGroup),
	Other(Response)
}
/// Patch a cluster group object.
pub fn virtualization_cluster_groups_partial_update(state: &ThanixClient, body: PatchedClusterGroupRequest, id: i64) -> Result<VirtualizationClusterGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/cluster-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterGroupsPartialUpdateResponse::Http200(r#response.json::<ClusterGroup>()?)) },
		r#other_status => { Ok(VirtualizationClusterGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualizationClusterTypesListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum VirtualizationClusterTypesListResponse {
	Http200(PaginatedClusterTypeList),
	Other(Response)
}
/// Get a list of cluster type objects.
pub fn virtualization_cluster_types_list(state: &ThanixClient, query: VirtualizationClusterTypesListQuery) -> Result<VirtualizationClusterTypesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/virtualization/cluster-types/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterTypesListResponse::Http200(r#response.json::<PaginatedClusterTypeList>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesBulkUpdateResponse {
	Http200(Vec<ClusterType>),
	Other(Response)
}
/// Put a list of cluster type objects.
pub fn virtualization_cluster_types_bulk_update(state: &ThanixClient, body: Vec<ClusterTypeRequest>) -> Result<VirtualizationClusterTypesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/cluster-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterTypesBulkUpdateResponse::Http200(r#response.json::<Vec<ClusterType>>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesCreateResponse {
	Http201(ClusterType),
	Other(Response)
}
/// Post a list of cluster type objects.
pub fn virtualization_cluster_types_create(state: &ThanixClient, body: ClusterTypeRequest) -> Result<VirtualizationClusterTypesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/virtualization/cluster-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(VirtualizationClusterTypesCreateResponse::Http201(r#response.json::<ClusterType>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of cluster type objects.
pub fn virtualization_cluster_types_bulk_destroy(state: &ThanixClient, body: Vec<ClusterTypeRequest>) -> Result<VirtualizationClusterTypesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/cluster-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationClusterTypesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesBulkPartialUpdateResponse {
	Http200(Vec<ClusterType>),
	Other(Response)
}
/// Patch a list of cluster type objects.
pub fn virtualization_cluster_types_bulk_partial_update(state: &ThanixClient, body: Vec<ClusterTypeRequest>) -> Result<VirtualizationClusterTypesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/cluster-types/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterTypesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<ClusterType>>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesRetrieveResponse {
	Http200(ClusterType),
	Other(Response)
}
/// Get a cluster type object.
pub fn virtualization_cluster_types_retrieve(state: &ThanixClient, id: i64) -> Result<VirtualizationClusterTypesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/virtualization/cluster-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterTypesRetrieveResponse::Http200(r#response.json::<ClusterType>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesUpdateResponse {
	Http200(ClusterType),
	Other(Response)
}
/// Put a cluster type object.
pub fn virtualization_cluster_types_update(state: &ThanixClient, body: ClusterTypeRequest, id: i64) -> Result<VirtualizationClusterTypesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/cluster-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterTypesUpdateResponse::Http200(r#response.json::<ClusterType>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a cluster type object.
pub fn virtualization_cluster_types_destroy(state: &ThanixClient, id: i64) -> Result<VirtualizationClusterTypesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/cluster-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationClusterTypesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClusterTypesPartialUpdateResponse {
	Http200(ClusterType),
	Other(Response)
}
/// Patch a cluster type object.
pub fn virtualization_cluster_types_partial_update(state: &ThanixClient, body: PatchedClusterTypeRequest, id: i64) -> Result<VirtualizationClusterTypesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/cluster-types/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClusterTypesPartialUpdateResponse::Http200(r#response.json::<ClusterType>()?)) },
		r#other_status => { Ok(VirtualizationClusterTypesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualizationClustersListQuery {
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Parent group (slug)
	pub group: Option<Vec<String>>,
	/// Parent group (slug)
	pub group__n: Option<Vec<String>>,
	/// Parent group (ID)
	pub group_id: Option<Vec<Option<i64>>>,
	/// Parent group (ID)
	pub group_id__n: Option<Vec<Option<i64>>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<Option<i64>>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<Option<i64>>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	/// Cluster type (slug)
	pub r#type: Option<Vec<String>>,
	/// Cluster type (slug)
	pub type__n: Option<Vec<String>>,
	/// Cluster type (ID)
	pub type_id: Option<Vec<i64>>,
	/// Cluster type (ID)
	pub type_id__n: Option<Vec<i64>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum VirtualizationClustersListResponse {
	Http200(PaginatedClusterList),
	Other(Response)
}
/// Get a list of cluster objects.
pub fn virtualization_clusters_list(state: &ThanixClient, query: VirtualizationClustersListQuery) -> Result<VirtualizationClustersListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/virtualization/clusters/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClustersListResponse::Http200(r#response.json::<PaginatedClusterList>()?)) },
		r#other_status => { Ok(VirtualizationClustersListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersBulkUpdateResponse {
	Http200(Vec<Cluster>),
	Other(Response)
}
/// Put a list of cluster objects.
pub fn virtualization_clusters_bulk_update(state: &ThanixClient, body: Vec<ClusterRequest>) -> Result<VirtualizationClustersBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/clusters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClustersBulkUpdateResponse::Http200(r#response.json::<Vec<Cluster>>()?)) },
		r#other_status => { Ok(VirtualizationClustersBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersCreateResponse {
	Http201(Cluster),
	Other(Response)
}
/// Post a list of cluster objects.
pub fn virtualization_clusters_create(state: &ThanixClient, body: WritableClusterRequest) -> Result<VirtualizationClustersCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/virtualization/clusters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(VirtualizationClustersCreateResponse::Http201(r#response.json::<Cluster>()?)) },
		r#other_status => { Ok(VirtualizationClustersCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of cluster objects.
pub fn virtualization_clusters_bulk_destroy(state: &ThanixClient, body: Vec<ClusterRequest>) -> Result<VirtualizationClustersBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/clusters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationClustersBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersBulkPartialUpdateResponse {
	Http200(Vec<Cluster>),
	Other(Response)
}
/// Patch a list of cluster objects.
pub fn virtualization_clusters_bulk_partial_update(state: &ThanixClient, body: Vec<ClusterRequest>) -> Result<VirtualizationClustersBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/clusters/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClustersBulkPartialUpdateResponse::Http200(r#response.json::<Vec<Cluster>>()?)) },
		r#other_status => { Ok(VirtualizationClustersBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersRetrieveResponse {
	Http200(Cluster),
	Other(Response)
}
/// Get a cluster object.
pub fn virtualization_clusters_retrieve(state: &ThanixClient, id: i64) -> Result<VirtualizationClustersRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/virtualization/clusters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClustersRetrieveResponse::Http200(r#response.json::<Cluster>()?)) },
		r#other_status => { Ok(VirtualizationClustersRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersUpdateResponse {
	Http200(Cluster),
	Other(Response)
}
/// Put a cluster object.
pub fn virtualization_clusters_update(state: &ThanixClient, body: WritableClusterRequest, id: i64) -> Result<VirtualizationClustersUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/clusters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClustersUpdateResponse::Http200(r#response.json::<Cluster>()?)) },
		r#other_status => { Ok(VirtualizationClustersUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a cluster object.
pub fn virtualization_clusters_destroy(state: &ThanixClient, id: i64) -> Result<VirtualizationClustersDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/clusters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationClustersDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationClustersPartialUpdateResponse {
	Http200(Cluster),
	Other(Response)
}
/// Patch a cluster object.
pub fn virtualization_clusters_partial_update(state: &ThanixClient, body: PatchedWritableClusterRequest, id: i64) -> Result<VirtualizationClustersPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/clusters/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationClustersPartialUpdateResponse::Http200(r#response.json::<Cluster>()?)) },
		r#other_status => { Ok(VirtualizationClustersPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualizationInterfacesListQuery {
	/// Bridged interface (ID)
	pub bridge_id: Option<Vec<i64>>,
	/// Bridged interface (ID)
	pub bridge_id__n: Option<Vec<i64>>,
	/// Cluster
	pub cluster: Option<Vec<String>>,
	/// Cluster
	pub cluster__n: Option<Vec<String>>,
	/// Cluster (ID)
	pub cluster_id: Option<Vec<i64>>,
	/// Cluster (ID)
	pub cluster_id__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub enabled: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	/// L2VPN
	pub l2vpn: Option<Vec<Option<i64>>>,
	/// L2VPN
	pub l2vpn__n: Option<Vec<Option<i64>>>,
	/// L2VPN (ID)
	pub l2vpn_id: Option<Vec<i64>>,
	/// L2VPN (ID)
	pub l2vpn_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub mac_address: Option<Vec<String>>,
	pub mac_address__ic: Option<Vec<String>>,
	pub mac_address__ie: Option<Vec<String>>,
	pub mac_address__iew: Option<Vec<String>>,
	pub mac_address__isw: Option<Vec<String>>,
	pub mac_address__n: Option<Vec<String>>,
	pub mac_address__nic: Option<Vec<String>>,
	pub mac_address__nie: Option<Vec<String>>,
	pub mac_address__niew: Option<Vec<String>>,
	pub mac_address__nisw: Option<Vec<String>>,
	pub modified_by_request: Option<String>,
	pub mtu: Option<Vec<i64>>,
	pub mtu__empty: Option<bool>,
	pub mtu__gt: Option<Vec<i64>>,
	pub mtu__gte: Option<Vec<i64>>,
	pub mtu__lt: Option<Vec<i64>>,
	pub mtu__lte: Option<Vec<i64>>,
	pub mtu__n: Option<Vec<i64>>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Parent interface (ID)
	pub parent_id: Option<Vec<i64>>,
	/// Parent interface (ID)
	pub parent_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,
	/// Virtual machine
	pub virtual_machine: Option<Vec<String>>,
	/// Virtual machine
	pub virtual_machine__n: Option<Vec<String>>,
	/// Virtual machine (ID)
	pub virtual_machine_id: Option<Vec<i64>>,
	/// Virtual machine (ID)
	pub virtual_machine_id__n: Option<Vec<i64>>,
	/// Assigned VID
	pub vlan: Option<String>,
	/// Assigned VLAN
	pub vlan_id: Option<String>,
	/// VRF (RD)
	pub vrf: Option<Vec<Option<String>>>,
	/// VRF (RD)
	pub vrf__n: Option<Vec<Option<String>>>,
	/// VRF
	pub vrf_id: Option<Vec<i64>>,
	/// VRF
	pub vrf_id__n: Option<Vec<i64>>,

}
#[derive(Debug)]
pub enum VirtualizationInterfacesListResponse {
	Http200(PaginatedVMInterfaceList),
	Other(Response)
}
/// Get a list of interface objects.
pub fn virtualization_interfaces_list(state: &ThanixClient, query: VirtualizationInterfacesListQuery) -> Result<VirtualizationInterfacesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/virtualization/interfaces/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationInterfacesListResponse::Http200(r#response.json::<PaginatedVMInterfaceList>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesBulkUpdateResponse {
	Http200(Vec<VMInterface>),
	Other(Response)
}
/// Put a list of interface objects.
pub fn virtualization_interfaces_bulk_update(state: &ThanixClient, body: Vec<VMInterfaceRequest>) -> Result<VirtualizationInterfacesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationInterfacesBulkUpdateResponse::Http200(r#response.json::<Vec<VMInterface>>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesCreateResponse {
	Http201(VMInterface),
	Other(Response)
}
/// Post a list of interface objects.
pub fn virtualization_interfaces_create(state: &ThanixClient, body: WritableVMInterfaceRequest) -> Result<VirtualizationInterfacesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/virtualization/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(VirtualizationInterfacesCreateResponse::Http201(r#response.json::<VMInterface>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of interface objects.
pub fn virtualization_interfaces_bulk_destroy(state: &ThanixClient, body: Vec<VMInterfaceRequest>) -> Result<VirtualizationInterfacesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationInterfacesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesBulkPartialUpdateResponse {
	Http200(Vec<VMInterface>),
	Other(Response)
}
/// Patch a list of interface objects.
pub fn virtualization_interfaces_bulk_partial_update(state: &ThanixClient, body: Vec<VMInterfaceRequest>) -> Result<VirtualizationInterfacesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/interfaces/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationInterfacesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VMInterface>>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesRetrieveResponse {
	Http200(VMInterface),
	Other(Response)
}
/// Get a interface object.
pub fn virtualization_interfaces_retrieve(state: &ThanixClient, id: i64) -> Result<VirtualizationInterfacesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/virtualization/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationInterfacesRetrieveResponse::Http200(r#response.json::<VMInterface>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesUpdateResponse {
	Http200(VMInterface),
	Other(Response)
}
/// Put a interface object.
pub fn virtualization_interfaces_update(state: &ThanixClient, body: WritableVMInterfaceRequest, id: i64) -> Result<VirtualizationInterfacesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationInterfacesUpdateResponse::Http200(r#response.json::<VMInterface>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a interface object.
pub fn virtualization_interfaces_destroy(state: &ThanixClient, id: i64) -> Result<VirtualizationInterfacesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationInterfacesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationInterfacesPartialUpdateResponse {
	Http200(VMInterface),
	Other(Response)
}
/// Patch a interface object.
pub fn virtualization_interfaces_partial_update(state: &ThanixClient, body: PatchedWritableVMInterfaceRequest, id: i64) -> Result<VirtualizationInterfacesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/interfaces/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationInterfacesPartialUpdateResponse::Http200(r#response.json::<VMInterface>()?)) },
		r#other_status => { Ok(VirtualizationInterfacesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VirtualizationVirtualMachinesListQuery {
	/// Cluster
	pub cluster: Option<Vec<String>>,
	/// Cluster
	pub cluster__n: Option<Vec<String>>,
	/// Cluster group (slug)
	pub cluster_group: Option<Vec<String>>,
	/// Cluster group (slug)
	pub cluster_group__n: Option<Vec<String>>,
	/// Cluster group (ID)
	pub cluster_group_id: Option<Vec<i64>>,
	/// Cluster group (ID)
	pub cluster_group_id__n: Option<Vec<i64>>,
	/// Cluster (ID)
	pub cluster_id: Option<Vec<Option<i64>>>,
	/// Cluster (ID)
	pub cluster_id__n: Option<Vec<Option<i64>>>,
	/// Cluster type (slug)
	pub cluster_type: Option<Vec<String>>,
	/// Cluster type (slug)
	pub cluster_type__n: Option<Vec<String>>,
	/// Cluster type (ID)
	pub cluster_type_id: Option<Vec<i64>>,
	/// Cluster type (ID)
	pub cluster_type_id__n: Option<Vec<i64>>,
	/// Config template (ID)
	pub config_template_id: Option<Vec<Option<i64>>>,
	/// Config template (ID)
	pub config_template_id__n: Option<Vec<Option<i64>>>,
	/// Contact
	pub contact: Option<Vec<i64>>,
	/// Contact
	pub contact__n: Option<Vec<i64>>,
	/// Contact group
	pub contact_group: Option<Vec<i64>>,
	/// Contact group
	pub contact_group__n: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role: Option<Vec<i64>>,
	/// Contact Role
	pub contact_role__n: Option<Vec<i64>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	/// Device
	pub device: Option<Vec<Option<String>>>,
	/// Device
	pub device__n: Option<Vec<Option<String>>>,
	/// Device (ID)
	pub device_id: Option<Vec<Option<i64>>>,
	/// Device (ID)
	pub device_id__n: Option<Vec<Option<i64>>>,
	pub disk: Option<Vec<i64>>,
	pub disk__empty: Option<bool>,
	pub disk__gt: Option<Vec<i64>>,
	pub disk__gte: Option<Vec<i64>>,
	pub disk__lt: Option<Vec<i64>>,
	pub disk__lte: Option<Vec<i64>>,
	pub disk__n: Option<Vec<i64>>,
	/// Has a primary IP
	pub has_primary_ip: Option<bool>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	/// Has local config context data
	pub local_context_data: Option<bool>,
	pub mac_address: Option<Vec<String>>,
	pub mac_address__ic: Option<Vec<String>>,
	pub mac_address__ie: Option<Vec<String>>,
	pub mac_address__iew: Option<Vec<String>>,
	pub mac_address__isw: Option<Vec<String>>,
	pub mac_address__n: Option<Vec<String>>,
	pub mac_address__nic: Option<Vec<String>>,
	pub mac_address__nie: Option<Vec<String>>,
	pub mac_address__niew: Option<Vec<String>>,
	pub mac_address__nisw: Option<Vec<String>>,
	pub memory: Option<Vec<i64>>,
	pub memory__empty: Option<bool>,
	pub memory__gt: Option<Vec<i64>>,
	pub memory__gte: Option<Vec<i64>>,
	pub memory__lt: Option<Vec<i64>>,
	pub memory__lte: Option<Vec<i64>>,
	pub memory__n: Option<Vec<i64>>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Platform (slug)
	pub platform: Option<Vec<String>>,
	/// Platform (slug)
	pub platform__n: Option<Vec<String>>,
	/// Platform (ID)
	pub platform_id: Option<Vec<Option<i64>>>,
	/// Platform (ID)
	pub platform_id__n: Option<Vec<Option<i64>>>,
	/// Primary IPv4 (ID)
	pub primary_ip4_id: Option<Vec<i64>>,
	/// Primary IPv4 (ID)
	pub primary_ip4_id__n: Option<Vec<i64>>,
	/// Primary IPv6 (ID)
	pub primary_ip6_id: Option<Vec<i64>>,
	/// Primary IPv6 (ID)
	pub primary_ip6_id__n: Option<Vec<i64>>,
	/// Search
	pub q: Option<String>,
	/// Region (slug)
	pub region: Option<Vec<i64>>,
	/// Region (slug)
	pub region__n: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id: Option<Vec<i64>>,
	/// Region (ID)
	pub region_id__n: Option<Vec<i64>>,
	/// Role (slug)
	pub role: Option<Vec<String>>,
	/// Role (slug)
	pub role__n: Option<Vec<String>>,
	/// Role (ID)
	pub role_id: Option<Vec<Option<i64>>>,
	/// Role (ID)
	pub role_id__n: Option<Vec<Option<i64>>>,
	/// Site (slug)
	pub site: Option<Vec<String>>,
	/// Site (slug)
	pub site__n: Option<Vec<String>>,
	/// Site group (slug)
	pub site_group: Option<Vec<i64>>,
	/// Site group (slug)
	pub site_group__n: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id: Option<Vec<i64>>,
	/// Site group (ID)
	pub site_group_id__n: Option<Vec<i64>>,
	/// Site (ID)
	pub site_id: Option<Vec<Option<i64>>>,
	/// Site (ID)
	pub site_id__n: Option<Vec<Option<i64>>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	pub vcpus: Option<Vec<f64>>,
	pub vcpus__empty: Option<bool>,
	pub vcpus__gt: Option<Vec<f64>>,
	pub vcpus__gte: Option<Vec<f64>>,
	pub vcpus__lt: Option<Vec<f64>>,
	pub vcpus__lte: Option<Vec<f64>>,
	pub vcpus__n: Option<Vec<f64>>,

}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesListResponse {
	Http200(PaginatedVirtualMachineWithConfigContextList),
	Other(Response)
}
/// Get a list of virtual machine objects.
pub fn virtualization_virtual_machines_list(state: &ThanixClient, query: VirtualizationVirtualMachinesListQuery) -> Result<VirtualizationVirtualMachinesListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/virtualization/virtual-machines/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationVirtualMachinesListResponse::Http200(r#response.json::<PaginatedVirtualMachineWithConfigContextList>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesBulkUpdateResponse {
	Http200(Vec<VirtualMachineWithConfigContext>),
	Other(Response)
}
/// Put a list of virtual machine objects.
pub fn virtualization_virtual_machines_bulk_update(state: &ThanixClient, body: Vec<VirtualMachineWithConfigContextRequest>) -> Result<VirtualizationVirtualMachinesBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/virtual-machines/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationVirtualMachinesBulkUpdateResponse::Http200(r#response.json::<Vec<VirtualMachineWithConfigContext>>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesCreateResponse {
	Http201(VirtualMachineWithConfigContext),
	Other(Response)
}
/// Post a list of virtual machine objects.
pub fn virtualization_virtual_machines_create(state: &ThanixClient, body: WritableVirtualMachineWithConfigContextRequest) -> Result<VirtualizationVirtualMachinesCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/virtualization/virtual-machines/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(VirtualizationVirtualMachinesCreateResponse::Http201(r#response.json::<VirtualMachineWithConfigContext>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of virtual machine objects.
pub fn virtualization_virtual_machines_bulk_destroy(state: &ThanixClient, body: Vec<VirtualMachineWithConfigContextRequest>) -> Result<VirtualizationVirtualMachinesBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/virtual-machines/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationVirtualMachinesBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesBulkPartialUpdateResponse {
	Http200(Vec<VirtualMachineWithConfigContext>),
	Other(Response)
}
/// Patch a list of virtual machine objects.
pub fn virtualization_virtual_machines_bulk_partial_update(state: &ThanixClient, body: Vec<VirtualMachineWithConfigContextRequest>) -> Result<VirtualizationVirtualMachinesBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/virtual-machines/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationVirtualMachinesBulkPartialUpdateResponse::Http200(r#response.json::<Vec<VirtualMachineWithConfigContext>>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesRetrieveResponse {
	Http200(VirtualMachineWithConfigContext),
	Other(Response)
}
/// Get a virtual machine object.
pub fn virtualization_virtual_machines_retrieve(state: &ThanixClient, id: i64) -> Result<VirtualizationVirtualMachinesRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/virtualization/virtual-machines/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationVirtualMachinesRetrieveResponse::Http200(r#response.json::<VirtualMachineWithConfigContext>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesUpdateResponse {
	Http200(VirtualMachineWithConfigContext),
	Other(Response)
}
/// Put a virtual machine object.
pub fn virtualization_virtual_machines_update(state: &ThanixClient, body: WritableVirtualMachineWithConfigContextRequest, id: i64) -> Result<VirtualizationVirtualMachinesUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/virtualization/virtual-machines/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationVirtualMachinesUpdateResponse::Http200(r#response.json::<VirtualMachineWithConfigContext>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a virtual machine object.
pub fn virtualization_virtual_machines_destroy(state: &ThanixClient, id: i64) -> Result<VirtualizationVirtualMachinesDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/virtualization/virtual-machines/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(VirtualizationVirtualMachinesDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum VirtualizationVirtualMachinesPartialUpdateResponse {
	Http200(VirtualMachineWithConfigContext),
	Other(Response)
}
/// Patch a virtual machine object.
pub fn virtualization_virtual_machines_partial_update(state: &ThanixClient, body: PatchedWritableVirtualMachineWithConfigContextRequest, id: i64) -> Result<VirtualizationVirtualMachinesPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/virtualization/virtual-machines/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(VirtualizationVirtualMachinesPartialUpdateResponse::Http200(r#response.json::<VirtualMachineWithConfigContext>()?)) },
		r#other_status => { Ok(VirtualizationVirtualMachinesPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessWirelessLanGroupsListQuery {
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	pub name: Option<Vec<String>>,
	pub name__empty: Option<bool>,
	pub name__ic: Option<Vec<String>>,
	pub name__ie: Option<Vec<String>>,
	pub name__iew: Option<Vec<String>>,
	pub name__isw: Option<Vec<String>>,
	pub name__n: Option<Vec<String>>,
	pub name__nic: Option<Vec<String>>,
	pub name__nie: Option<Vec<String>>,
	pub name__niew: Option<Vec<String>>,
	pub name__nisw: Option<Vec<String>>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	pub parent: Option<Vec<String>>,
	pub parent__n: Option<Vec<String>>,
	pub parent_id: Option<Vec<Option<i64>>>,
	pub parent_id__n: Option<Vec<Option<i64>>>,
	/// Search
	pub q: Option<String>,
	pub slug: Option<Vec<String>>,
	pub slug__empty: Option<bool>,
	pub slug__ic: Option<Vec<String>>,
	pub slug__ie: Option<Vec<String>>,
	pub slug__iew: Option<Vec<String>>,
	pub slug__isw: Option<Vec<String>>,
	pub slug__n: Option<Vec<String>>,
	pub slug__nic: Option<Vec<String>>,
	pub slug__nie: Option<Vec<String>>,
	pub slug__niew: Option<Vec<String>>,
	pub slug__nisw: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsListResponse {
	Http200(PaginatedWirelessLANGroupList),
	Other(Response)
}
/// Get a list of wireless LAN group objects.
pub fn wireless_wireless_lan_groups_list(state: &ThanixClient, query: WirelessWirelessLanGroupsListQuery) -> Result<WirelessWirelessLanGroupsListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/wireless/wireless-lan-groups/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLanGroupsListResponse::Http200(r#response.json::<PaginatedWirelessLANGroupList>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsBulkUpdateResponse {
	Http200(Vec<WirelessLANGroup>),
	Other(Response)
}
/// Put a list of wireless LAN group objects.
pub fn wireless_wireless_lan_groups_bulk_update(state: &ThanixClient, body: Vec<WirelessLANGroupRequest>) -> Result<WirelessWirelessLanGroupsBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/wireless/wireless-lan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLanGroupsBulkUpdateResponse::Http200(r#response.json::<Vec<WirelessLANGroup>>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsCreateResponse {
	Http201(WirelessLANGroup),
	Other(Response)
}
/// Post a list of wireless LAN group objects.
pub fn wireless_wireless_lan_groups_create(state: &ThanixClient, body: WritableWirelessLANGroupRequest) -> Result<WirelessWirelessLanGroupsCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/wireless/wireless-lan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(WirelessWirelessLanGroupsCreateResponse::Http201(r#response.json::<WirelessLANGroup>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of wireless LAN group objects.
pub fn wireless_wireless_lan_groups_bulk_destroy(state: &ThanixClient, body: Vec<WirelessLANGroupRequest>) -> Result<WirelessWirelessLanGroupsBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/wireless/wireless-lan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(WirelessWirelessLanGroupsBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsBulkPartialUpdateResponse {
	Http200(Vec<WirelessLANGroup>),
	Other(Response)
}
/// Patch a list of wireless LAN group objects.
pub fn wireless_wireless_lan_groups_bulk_partial_update(state: &ThanixClient, body: Vec<WirelessLANGroupRequest>) -> Result<WirelessWirelessLanGroupsBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/wireless/wireless-lan-groups/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLanGroupsBulkPartialUpdateResponse::Http200(r#response.json::<Vec<WirelessLANGroup>>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsRetrieveResponse {
	Http200(WirelessLANGroup),
	Other(Response)
}
/// Get a wireless LAN group object.
pub fn wireless_wireless_lan_groups_retrieve(state: &ThanixClient, id: i64) -> Result<WirelessWirelessLanGroupsRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/wireless/wireless-lan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLanGroupsRetrieveResponse::Http200(r#response.json::<WirelessLANGroup>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsUpdateResponse {
	Http200(WirelessLANGroup),
	Other(Response)
}
/// Put a wireless LAN group object.
pub fn wireless_wireless_lan_groups_update(state: &ThanixClient, body: WritableWirelessLANGroupRequest, id: i64) -> Result<WirelessWirelessLanGroupsUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/wireless/wireless-lan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLanGroupsUpdateResponse::Http200(r#response.json::<WirelessLANGroup>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a wireless LAN group object.
pub fn wireless_wireless_lan_groups_destroy(state: &ThanixClient, id: i64) -> Result<WirelessWirelessLanGroupsDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/wireless/wireless-lan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(WirelessWirelessLanGroupsDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLanGroupsPartialUpdateResponse {
	Http200(WirelessLANGroup),
	Other(Response)
}
/// Patch a wireless LAN group object.
pub fn wireless_wireless_lan_groups_partial_update(state: &ThanixClient, body: PatchedWritableWirelessLANGroupRequest, id: i64) -> Result<WirelessWirelessLanGroupsPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/wireless/wireless-lan-groups/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLanGroupsPartialUpdateResponse::Http200(r#response.json::<WirelessLANGroup>()?)) },
		r#other_status => { Ok(WirelessWirelessLanGroupsPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessWirelessLansListQuery {
	pub auth_cipher: Option<Vec<String>>,
	pub auth_cipher__n: Option<Vec<String>>,
	pub auth_psk: Option<Vec<String>>,
	pub auth_psk__empty: Option<bool>,
	pub auth_psk__ic: Option<Vec<String>>,
	pub auth_psk__ie: Option<Vec<String>>,
	pub auth_psk__iew: Option<Vec<String>>,
	pub auth_psk__isw: Option<Vec<String>>,
	pub auth_psk__n: Option<Vec<String>>,
	pub auth_psk__nic: Option<Vec<String>>,
	pub auth_psk__nie: Option<Vec<String>>,
	pub auth_psk__niew: Option<Vec<String>>,
	pub auth_psk__nisw: Option<Vec<String>>,
	pub auth_type: Option<Vec<String>>,
	pub auth_type__n: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub group: Option<Vec<i64>>,
	pub group__n: Option<Vec<i64>>,
	pub group_id: Option<Vec<i64>>,
	pub group_id__n: Option<Vec<i64>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub ssid: Option<Vec<String>>,
	pub ssid__empty: Option<bool>,
	pub ssid__ic: Option<Vec<String>>,
	pub ssid__ie: Option<Vec<String>>,
	pub ssid__iew: Option<Vec<String>>,
	pub ssid__isw: Option<Vec<String>>,
	pub ssid__n: Option<Vec<String>>,
	pub ssid__nic: Option<Vec<String>>,
	pub ssid__nie: Option<Vec<String>>,
	pub ssid__niew: Option<Vec<String>>,
	pub ssid__nisw: Option<Vec<String>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,
	pub vlan_id: Option<Vec<Option<i64>>>,
	pub vlan_id__n: Option<Vec<Option<i64>>>,

}
#[derive(Debug)]
pub enum WirelessWirelessLansListResponse {
	Http200(PaginatedWirelessLANList),
	Other(Response)
}
/// Get a list of wireless LAN objects.
pub fn wireless_wireless_lans_list(state: &ThanixClient, query: WirelessWirelessLansListQuery) -> Result<WirelessWirelessLansListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/wireless/wireless-lans/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLansListResponse::Http200(r#response.json::<PaginatedWirelessLANList>()?)) },
		r#other_status => { Ok(WirelessWirelessLansListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansBulkUpdateResponse {
	Http200(Vec<WirelessLAN>),
	Other(Response)
}
/// Put a list of wireless LAN objects.
pub fn wireless_wireless_lans_bulk_update(state: &ThanixClient, body: Vec<WirelessLANRequest>) -> Result<WirelessWirelessLansBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/wireless/wireless-lans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLansBulkUpdateResponse::Http200(r#response.json::<Vec<WirelessLAN>>()?)) },
		r#other_status => { Ok(WirelessWirelessLansBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansCreateResponse {
	Http201(WirelessLAN),
	Other(Response)
}
/// Post a list of wireless LAN objects.
pub fn wireless_wireless_lans_create(state: &ThanixClient, body: WritableWirelessLANRequest) -> Result<WirelessWirelessLansCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/wireless/wireless-lans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(WirelessWirelessLansCreateResponse::Http201(r#response.json::<WirelessLAN>()?)) },
		r#other_status => { Ok(WirelessWirelessLansCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of wireless LAN objects.
pub fn wireless_wireless_lans_bulk_destroy(state: &ThanixClient, body: Vec<WirelessLANRequest>) -> Result<WirelessWirelessLansBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/wireless/wireless-lans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(WirelessWirelessLansBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansBulkPartialUpdateResponse {
	Http200(Vec<WirelessLAN>),
	Other(Response)
}
/// Patch a list of wireless LAN objects.
pub fn wireless_wireless_lans_bulk_partial_update(state: &ThanixClient, body: Vec<WirelessLANRequest>) -> Result<WirelessWirelessLansBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/wireless/wireless-lans/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLansBulkPartialUpdateResponse::Http200(r#response.json::<Vec<WirelessLAN>>()?)) },
		r#other_status => { Ok(WirelessWirelessLansBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansRetrieveResponse {
	Http200(WirelessLAN),
	Other(Response)
}
/// Get a wireless LAN object.
pub fn wireless_wireless_lans_retrieve(state: &ThanixClient, id: i64) -> Result<WirelessWirelessLansRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/wireless/wireless-lans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLansRetrieveResponse::Http200(r#response.json::<WirelessLAN>()?)) },
		r#other_status => { Ok(WirelessWirelessLansRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansUpdateResponse {
	Http200(WirelessLAN),
	Other(Response)
}
/// Put a wireless LAN object.
pub fn wireless_wireless_lans_update(state: &ThanixClient, body: WritableWirelessLANRequest, id: i64) -> Result<WirelessWirelessLansUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/wireless/wireless-lans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLansUpdateResponse::Http200(r#response.json::<WirelessLAN>()?)) },
		r#other_status => { Ok(WirelessWirelessLansUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a wireless LAN object.
pub fn wireless_wireless_lans_destroy(state: &ThanixClient, id: i64) -> Result<WirelessWirelessLansDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/wireless/wireless-lans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(WirelessWirelessLansDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLansPartialUpdateResponse {
	Http200(WirelessLAN),
	Other(Response)
}
/// Patch a wireless LAN object.
pub fn wireless_wireless_lans_partial_update(state: &ThanixClient, body: PatchedWritableWirelessLANRequest, id: i64) -> Result<WirelessWirelessLansPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/wireless/wireless-lans/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLansPartialUpdateResponse::Http200(r#response.json::<WirelessLAN>()?)) },
		r#other_status => { Ok(WirelessWirelessLansPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WirelessWirelessLinksListQuery {
	pub auth_cipher: Option<Vec<String>>,
	pub auth_cipher__n: Option<Vec<String>>,
	pub auth_psk: Option<Vec<String>>,
	pub auth_psk__empty: Option<bool>,
	pub auth_psk__ic: Option<Vec<String>>,
	pub auth_psk__ie: Option<Vec<String>>,
	pub auth_psk__iew: Option<Vec<String>>,
	pub auth_psk__isw: Option<Vec<String>>,
	pub auth_psk__n: Option<Vec<String>>,
	pub auth_psk__nic: Option<Vec<String>>,
	pub auth_psk__nie: Option<Vec<String>>,
	pub auth_psk__niew: Option<Vec<String>>,
	pub auth_psk__nisw: Option<Vec<String>>,
	pub auth_type: Option<Vec<String>>,
	pub auth_type__n: Option<Vec<String>>,
	pub created: Option<Vec<String>>,
	pub created__empty: Option<Vec<String>>,
	pub created__gt: Option<Vec<String>>,
	pub created__gte: Option<Vec<String>>,
	pub created__lt: Option<Vec<String>>,
	pub created__lte: Option<Vec<String>>,
	pub created__n: Option<Vec<String>>,
	pub created_by_request: Option<String>,
	pub description: Option<Vec<String>>,
	pub description__empty: Option<bool>,
	pub description__ic: Option<Vec<String>>,
	pub description__ie: Option<Vec<String>>,
	pub description__iew: Option<Vec<String>>,
	pub description__isw: Option<Vec<String>>,
	pub description__n: Option<Vec<String>>,
	pub description__nic: Option<Vec<String>>,
	pub description__nie: Option<Vec<String>>,
	pub description__niew: Option<Vec<String>>,
	pub description__nisw: Option<Vec<String>>,
	pub id: Option<Vec<i64>>,
	pub id__empty: Option<bool>,
	pub id__gt: Option<Vec<i64>>,
	pub id__gte: Option<Vec<i64>>,
	pub id__lt: Option<Vec<i64>>,
	pub id__lte: Option<Vec<i64>>,
	pub id__n: Option<Vec<i64>>,
	pub interface_a_id: Option<Vec<i64>>,
	pub interface_a_id__empty: Option<Vec<i64>>,
	pub interface_a_id__gt: Option<Vec<i64>>,
	pub interface_a_id__gte: Option<Vec<i64>>,
	pub interface_a_id__lt: Option<Vec<i64>>,
	pub interface_a_id__lte: Option<Vec<i64>>,
	pub interface_a_id__n: Option<Vec<i64>>,
	pub interface_b_id: Option<Vec<i64>>,
	pub interface_b_id__empty: Option<Vec<i64>>,
	pub interface_b_id__gt: Option<Vec<i64>>,
	pub interface_b_id__gte: Option<Vec<i64>>,
	pub interface_b_id__lt: Option<Vec<i64>>,
	pub interface_b_id__lte: Option<Vec<i64>>,
	pub interface_b_id__n: Option<Vec<i64>>,
	pub last_updated: Option<Vec<String>>,
	pub last_updated__empty: Option<Vec<String>>,
	pub last_updated__gt: Option<Vec<String>>,
	pub last_updated__gte: Option<Vec<String>>,
	pub last_updated__lt: Option<Vec<String>>,
	pub last_updated__lte: Option<Vec<String>>,
	pub last_updated__n: Option<Vec<String>>,
	/// Number of results to return per page.
	pub limit: Option<i64>,
	pub modified_by_request: Option<String>,
	/// The initial index from which to return the results.
	pub offset: Option<i64>,
	/// Which field to use when ordering the results.
	pub ordering: Option<String>,
	/// Search
	pub q: Option<String>,
	pub ssid: Option<Vec<String>>,
	pub ssid__empty: Option<bool>,
	pub ssid__ic: Option<Vec<String>>,
	pub ssid__ie: Option<Vec<String>>,
	pub ssid__iew: Option<Vec<String>>,
	pub ssid__isw: Option<Vec<String>>,
	pub ssid__n: Option<Vec<String>>,
	pub ssid__nic: Option<Vec<String>>,
	pub ssid__nie: Option<Vec<String>>,
	pub ssid__niew: Option<Vec<String>>,
	pub ssid__nisw: Option<Vec<String>>,
	pub status: Option<Vec<String>>,
	pub status__n: Option<Vec<String>>,
	pub tag: Option<Vec<String>>,
	pub tag__n: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant: Option<Vec<String>>,
	/// Tenant (slug)
	pub tenant__n: Option<Vec<String>>,
	/// Tenant Group (slug)
	pub tenant_group: Option<Vec<i64>>,
	/// Tenant Group (slug)
	pub tenant_group__n: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id: Option<Vec<i64>>,
	/// Tenant Group (ID)
	pub tenant_group_id__n: Option<Vec<i64>>,
	/// Tenant (ID)
	pub tenant_id: Option<Vec<Option<i64>>>,
	/// Tenant (ID)
	pub tenant_id__n: Option<Vec<Option<i64>>>,
	pub updated_by_request: Option<String>,

}
#[derive(Debug)]
pub enum WirelessWirelessLinksListResponse {
	Http200(PaginatedWirelessLinkList),
	Other(Response)
}
/// Get a list of wireless link objects.
pub fn wireless_wireless_links_list(state: &ThanixClient, query: WirelessWirelessLinksListQuery) -> Result<WirelessWirelessLinksListResponse, Error> {
	let qstring = serde_qs::to_string(&query).unwrap();
	let qstring_clean = remove_square_braces(&qstring);
	let r#response = state.client.get(format!("{}/api/wireless/wireless-links/?{}", state.base_url, qstring_clean))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLinksListResponse::Http200(r#response.json::<PaginatedWirelessLinkList>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksListResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksBulkUpdateResponse {
	Http200(Vec<WirelessLink>),
	Other(Response)
}
/// Put a list of wireless link objects.
pub fn wireless_wireless_links_bulk_update(state: &ThanixClient, body: Vec<WirelessLinkRequest>) -> Result<WirelessWirelessLinksBulkUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/wireless/wireless-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLinksBulkUpdateResponse::Http200(r#response.json::<Vec<WirelessLink>>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksBulkUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksCreateResponse {
	Http201(WirelessLink),
	Other(Response)
}
/// Post a list of wireless link objects.
pub fn wireless_wireless_links_create(state: &ThanixClient, body: WritableWirelessLinkRequest) -> Result<WirelessWirelessLinksCreateResponse, Error> {
	let r#response = state.client.post(format!("{}/api/wireless/wireless-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		201 => { Ok(WirelessWirelessLinksCreateResponse::Http201(r#response.json::<WirelessLink>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksCreateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksBulkDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a list of wireless link objects.
pub fn wireless_wireless_links_bulk_destroy(state: &ThanixClient, body: Vec<WirelessLinkRequest>) -> Result<WirelessWirelessLinksBulkDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/wireless/wireless-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(WirelessWirelessLinksBulkDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksBulkPartialUpdateResponse {
	Http200(Vec<WirelessLink>),
	Other(Response)
}
/// Patch a list of wireless link objects.
pub fn wireless_wireless_links_bulk_partial_update(state: &ThanixClient, body: Vec<WirelessLinkRequest>) -> Result<WirelessWirelessLinksBulkPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/wireless/wireless-links/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLinksBulkPartialUpdateResponse::Http200(r#response.json::<Vec<WirelessLink>>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksBulkPartialUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksRetrieveResponse {
	Http200(WirelessLink),
	Other(Response)
}
/// Get a wireless link object.
pub fn wireless_wireless_links_retrieve(state: &ThanixClient, id: i64) -> Result<WirelessWirelessLinksRetrieveResponse, Error> {
	let r#response = state.client.get(format!("{}/api/wireless/wireless-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLinksRetrieveResponse::Http200(r#response.json::<WirelessLink>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksRetrieveResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksUpdateResponse {
	Http200(WirelessLink),
	Other(Response)
}
/// Put a wireless link object.
pub fn wireless_wireless_links_update(state: &ThanixClient, body: WritableWirelessLinkRequest, id: i64) -> Result<WirelessWirelessLinksUpdateResponse, Error> {
	let r#response = state.client.put(format!("{}/api/wireless/wireless-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLinksUpdateResponse::Http200(r#response.json::<WirelessLink>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksUpdateResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksDestroyResponse {
	Http204,
	Other(Response)
}
/// Delete a wireless link object.
pub fn wireless_wireless_links_destroy(state: &ThanixClient, id: i64) -> Result<WirelessWirelessLinksDestroyResponse, Error> {
	let r#response = state.client.delete(format!("{}/api/wireless/wireless-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.send()?;
	match r#response.status().as_u16() {
		r#other_status => { Ok(WirelessWirelessLinksDestroyResponse::Other(r#response)) }
	}
}
#[derive(Debug)]
pub enum WirelessWirelessLinksPartialUpdateResponse {
	Http200(WirelessLink),
	Other(Response)
}
/// Patch a wireless link object.
pub fn wireless_wireless_links_partial_update(state: &ThanixClient, body: PatchedWritableWirelessLinkRequest, id: i64) -> Result<WirelessWirelessLinksPartialUpdateResponse, Error> {
	let r#response = state.client.patch(format!("{}/api/wireless/wireless-links/{id}/", state.base_url))
		.header("Authorization", format!("Token {}", state.authentication_token))
		.json(&body)
		.send()?;
	match r#response.status().as_u16() {
		200 => { Ok(WirelessWirelessLinksPartialUpdateResponse::Http200(r#response.json::<WirelessLink>()?)) },
		r#other_status => { Ok(WirelessWirelessLinksPartialUpdateResponse::Other(r#response)) }
	}
}

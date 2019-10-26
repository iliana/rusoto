// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateCreatedArtifactRequest {
    /// <p>An ARN of the AWS resource related to the migration (e.g., AMI, EC2 instance, RDS instance, etc.) </p>
    #[serde(rename = "CreatedArtifact")]
    pub created_artifact: CreatedArtifact,
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct AssociateCreatedArtifactResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateDiscoveredResourceRequest {
    /// <p>Object representing a Resource.</p>
    #[serde(rename = "DiscoveredResource")]
    pub discovered_resource: DiscoveredResource,
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The identifier given to the MigrationTask.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream.</p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct AssociateDiscoveredResourceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProgressUpdateStreamRequest {
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStreamName")]
    pub progress_update_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CreateProgressUpdateStreamResult {}

/// <p>An ARN of the AWS cloud resource target receiving the migration (e.g., AMI, EC2 instance, RDS instance, etc.).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatedArtifact {
    /// <p>A description that can be free-form text to record additional detail about the artifact for clarity or for later reference.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An ARN that uniquely identifies the result of a migration task.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProgressUpdateStreamRequest {
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStreamName")]
    pub progress_update_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DeleteProgressUpdateStreamResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeApplicationStateRequest {
    /// <p>The configurationId in ADS that uniquely identifies the grouped application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribeApplicationStateResult {
    /// <p>Status of the application - Not Started, In-Progress, Complete.</p>
    #[serde(rename = "ApplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<String>,
    /// <p>The timestamp when the application status was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMigrationTaskRequest {
    /// <p>The identifier given to the MigrationTask.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribeMigrationTaskResult {
    /// <p>Object encapsulating information about the migration task.</p>
    #[serde(rename = "MigrationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_task: Option<MigrationTask>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateCreatedArtifactRequest {
    /// <p>An ARN of the AWS resource related to the migration (e.g., AMI, EC2 instance, RDS instance, etc.)</p>
    #[serde(rename = "CreatedArtifactName")]
    pub created_artifact_name: String,
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Unique identifier that references the migration task to be disassociated with the artifact.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DisassociateCreatedArtifactResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDiscoveredResourceRequest {
    /// <p>ConfigurationId of the ADS resource to be disassociated.</p>
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The identifier given to the MigrationTask.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream.</p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DisassociateDiscoveredResourceResult {}

/// <p>Object representing the on-premises resource being migrated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredResource {
    /// <p>The configurationId in ADS that uniquely identifies the on-premise resource.</p>
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,
    /// <p>A description that can be free-form text to record additional detail about the discovered resource for clarity or later reference.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportMigrationTaskRequest {
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ImportMigrationTaskResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCreatedArtifactsRequest {
    /// <p>Maximum number of results to be returned per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListCreatedArtifactsResult {
    /// <p>List of created artifacts up to the maximum number of results specified in the request.</p>
    #[serde(rename = "CreatedArtifactList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_artifact_list: Option<Vec<CreatedArtifact>>,
    /// <p>If there are more created artifacts than the max result, return the next token to be passed to the next call as a bookmark of where to start from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDiscoveredResourcesRequest {
    /// <p>The maximum number of results returned per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the MigrationTask.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the ProgressUpdateStream.</p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListDiscoveredResourcesResult {
    /// <p>Returned list of discovered resources associated with the given MigrationTask.</p>
    #[serde(rename = "DiscoveredResourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_resource_list: Option<Vec<DiscoveredResource>>,
    /// <p>If there are more discovered resources than the max result, return the next token to be passed to the next call as a bookmark of where to start from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListMigrationTasksRequest {
    /// <p>Value to specify how many results are returned per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filter migration tasks by discovered resource name.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListMigrationTasksResult {
    /// <p>Lists the migration task's summary which includes: <code>MigrationTaskName</code>, <code>ProgressPercent</code>, <code>ProgressUpdateStream</code>, <code>Status</code>, and the <code>UpdateDateTime</code> for each task.</p>
    #[serde(rename = "MigrationTaskSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_task_summary_list: Option<Vec<MigrationTaskSummary>>,
    /// <p>If there are more migration tasks than the max result, return the next token to be passed to the next call as a bookmark of where to start from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProgressUpdateStreamsRequest {
    /// <p>Filter to limit the maximum number of results to list per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListProgressUpdateStreamsResult {
    /// <p>If there are more streams created than the max result, return the next token to be passed to the next call as a bookmark of where to start from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of progress update streams up to the max number of results passed in the input.</p>
    #[serde(rename = "ProgressUpdateStreamSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_stream_summary_list: Option<Vec<ProgressUpdateStreamSummary>>,
}

/// <p>Represents a migration task in a migration tool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct MigrationTask {
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_task_name: Option<String>,
    /// <p>A name that identifies the vendor of the migration tool being used.</p>
    #[serde(rename = "ProgressUpdateStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_stream: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "ResourceAttributeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attribute_list: Option<Vec<ResourceAttribute>>,
    /// <p>Task object encapsulating task information.</p>
    #[serde(rename = "Task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
    /// <p>The timestamp when the task was gathered.</p>
    #[serde(rename = "UpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date_time: Option<f64>,
}

/// <p>MigrationTaskSummary includes <code>MigrationTaskName</code>, <code>ProgressPercent</code>, <code>ProgressUpdateStream</code>, <code>Status</code>, and <code>UpdateDateTime</code> for each task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct MigrationTaskSummary {
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_task_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// <p>An AWS resource used for access control. It should uniquely identify the migration tool as it is used for all updates made by the tool.</p>
    #[serde(rename = "ProgressUpdateStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_stream: Option<String>,
    /// <p>Status of the task.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Detail information of what is being done within the overall status state.</p>
    #[serde(rename = "StatusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    /// <p>The timestamp when the task was gathered.</p>
    #[serde(rename = "UpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NotifyApplicationStateRequest {
    /// <p>The configurationId in ADS that uniquely identifies the grouped application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Status of the application - Not Started, In-Progress, Complete.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct NotifyApplicationStateResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NotifyMigrationTaskStateRequest {
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>Number of seconds after the UpdateDateTime within which the Migration Hub can expect an update. If Migration Hub does not receive an update within the specified interval, then the migration task will be considered stale.</p>
    #[serde(rename = "NextUpdateSeconds")]
    pub next_update_seconds: i64,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
    /// <p>Information about the task's progress and status.</p>
    #[serde(rename = "Task")]
    pub task: Task,
    /// <p>The timestamp when the task was gathered.</p>
    #[serde(rename = "UpdateDateTime")]
    pub update_date_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct NotifyMigrationTaskStateResult {}

/// <p>Summary of the AWS resource used for access control that is implicitly linked to your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ProgressUpdateStreamSummary {
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutResourceAttributesRequest {
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Unique identifier that references the migration task.</p>
    #[serde(rename = "MigrationTaskName")]
    pub migration_task_name: String,
    /// <p>The name of the ProgressUpdateStream. </p>
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: String,
    /// <p><p>Information about the resource that is being migrated. This data will be used to map the task to a resource in the Application Discovery Service (ADS)&#39;s repository.</p> <note> <p>Takes the object array of <code>ResourceAttribute</code> where the <code>Type</code> field is reserved for the following values: <code>IPV4<em>ADDRESS | IPV6</em>ADDRESS | MAC<em>ADDRESS | FQDN | VM</em>MANAGER<em>ID | VM</em>MANAGED<em>OBJECT</em>REFERENCE | VM<em>NAME | VM</em>PATH | BIOS<em>ID | MOTHERBOARD</em>SERIAL<em>NUMBER</code> where the identifying value can be a string up to 256 characters.</p> </note> <important> <ul> <li> <p>If any &quot;VM&quot; related value is set for a <code>ResourceAttribute</code> object, it is required that <code>VM</em>MANAGER<em>ID</code>, as a minimum, is always set. If <code>VM</em>MANAGER<em>ID</code> is not set, then all &quot;VM&quot; fields will be discarded and &quot;VM&quot; fields will not be used for matching the migration task to a server in Application Discovery Service (ADS)&#39;s repository. See the &lt;a href=&quot;https://docs.aws.amazon.com/migrationhub/latest/ug/API</em>PutResourceAttributes.html#API<em>PutResourceAttributes</em>Examples&quot;&gt;Example</a> section below for a use case of specifying &quot;VM&quot; related values.</p> </li> <li> <p> If a server you are trying to match has multiple IP or MAC addresses, you should provide as many as you know in separate type/value pairs passed to the <code>ResourceAttributeList</code> parameter to maximize the chances of matching.</p> </li> </ul> </important></p>
    #[serde(rename = "ResourceAttributeList")]
    pub resource_attribute_list: Vec<ResourceAttribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct PutResourceAttributesResult {}

/// <p><p>Attribute associated with a resource.</p> <p>Note the corresponding format required per type listed below:</p> <dl> <dt>IPV4</dt> <dd> <p> <code>x.x.x.x</code> </p> <p> <i>where x is an integer in the range [0,255]</i> </p> </dd> <dt>IPV6</dt> <dd> <p> <code>y : y : y : y : y : y : y : y</code> </p> <p> <i>where y is a hexadecimal between 0 and FFFF. [0, FFFF]</i> </p> </dd> <dt>MAC_ADDRESS</dt> <dd> <p> <code>^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$</code> </p> </dd> <dt>FQDN</dt> <dd> <p> <code>^[^&lt;&gt;{}\\/?,=\p{Cntrl}]{1,256}$</code> </p> </dd> </dl></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAttribute {
    /// <p>Type of resource.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>Value of the resource type.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Task object encapsulating task information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    /// <p>Indication of the percentage completion of the task.</p>
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// <p>Status of the task - Not Started, In-Progress, Complete.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>Details of task status as notified by a migration tool. A tool might use this field to provide clarifying information about the status that is unique to that tool or that explains an error state.</p>
    #[serde(rename = "StatusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
}

/// Errors returned by AssociateCreatedArtifact
#[derive(Debug, PartialEq)]
pub enum AssociateCreatedArtifactError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl AssociateCreatedArtifactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateCreatedArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateCreatedArtifactError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(AssociateCreatedArtifactError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        AssociateCreatedArtifactError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AssociateCreatedArtifactError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateCreatedArtifactError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AssociateCreatedArtifactError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        AssociateCreatedArtifactError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateCreatedArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateCreatedArtifactError {
    fn description(&self) -> &str {
        match *self {
            AssociateCreatedArtifactError::AccessDenied(ref cause) => cause,
            AssociateCreatedArtifactError::DryRunOperation(ref cause) => cause,
            AssociateCreatedArtifactError::InternalServerError(ref cause) => cause,
            AssociateCreatedArtifactError::InvalidInput(ref cause) => cause,
            AssociateCreatedArtifactError::ResourceNotFound(ref cause) => cause,
            AssociateCreatedArtifactError::ServiceUnavailable(ref cause) => cause,
            AssociateCreatedArtifactError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateDiscoveredResource
#[derive(Debug, PartialEq)]
pub enum AssociateDiscoveredResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when there are problems accessing ADS (Application Discovery Service); most likely due to a misconfigured policy or the <code>migrationhub-discovery</code> role is missing or not configured correctly.</p>
    PolicyError(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl AssociateDiscoveredResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateDiscoveredResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateDiscoveredResourceError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(AssociateDiscoveredResourceError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        AssociateDiscoveredResourceError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AssociateDiscoveredResourceError::InvalidInput(
                        err.msg,
                    ))
                }
                "PolicyErrorException" => {
                    return RusotoError::Service(AssociateDiscoveredResourceError::PolicyError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateDiscoveredResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AssociateDiscoveredResourceError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        AssociateDiscoveredResourceError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateDiscoveredResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDiscoveredResourceError {
    fn description(&self) -> &str {
        match *self {
            AssociateDiscoveredResourceError::AccessDenied(ref cause) => cause,
            AssociateDiscoveredResourceError::DryRunOperation(ref cause) => cause,
            AssociateDiscoveredResourceError::InternalServerError(ref cause) => cause,
            AssociateDiscoveredResourceError::InvalidInput(ref cause) => cause,
            AssociateDiscoveredResourceError::PolicyError(ref cause) => cause,
            AssociateDiscoveredResourceError::ResourceNotFound(ref cause) => cause,
            AssociateDiscoveredResourceError::ServiceUnavailable(ref cause) => cause,
            AssociateDiscoveredResourceError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProgressUpdateStream
#[derive(Debug, PartialEq)]
pub enum CreateProgressUpdateStreamError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl CreateProgressUpdateStreamError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateProgressUpdateStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateProgressUpdateStreamError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(CreateProgressUpdateStreamError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        CreateProgressUpdateStreamError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateProgressUpdateStreamError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CreateProgressUpdateStreamError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        CreateProgressUpdateStreamError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateProgressUpdateStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProgressUpdateStreamError {
    fn description(&self) -> &str {
        match *self {
            CreateProgressUpdateStreamError::AccessDenied(ref cause) => cause,
            CreateProgressUpdateStreamError::DryRunOperation(ref cause) => cause,
            CreateProgressUpdateStreamError::InternalServerError(ref cause) => cause,
            CreateProgressUpdateStreamError::InvalidInput(ref cause) => cause,
            CreateProgressUpdateStreamError::ServiceUnavailable(ref cause) => cause,
            CreateProgressUpdateStreamError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProgressUpdateStream
#[derive(Debug, PartialEq)]
pub enum DeleteProgressUpdateStreamError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl DeleteProgressUpdateStreamError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteProgressUpdateStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteProgressUpdateStreamError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(DeleteProgressUpdateStreamError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeleteProgressUpdateStreamError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteProgressUpdateStreamError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProgressUpdateStreamError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteProgressUpdateStreamError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        DeleteProgressUpdateStreamError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteProgressUpdateStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProgressUpdateStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteProgressUpdateStreamError::AccessDenied(ref cause) => cause,
            DeleteProgressUpdateStreamError::DryRunOperation(ref cause) => cause,
            DeleteProgressUpdateStreamError::InternalServerError(ref cause) => cause,
            DeleteProgressUpdateStreamError::InvalidInput(ref cause) => cause,
            DeleteProgressUpdateStreamError::ResourceNotFound(ref cause) => cause,
            DeleteProgressUpdateStreamError::ServiceUnavailable(ref cause) => cause,
            DeleteProgressUpdateStreamError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApplicationState
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationStateError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when there are problems accessing ADS (Application Discovery Service); most likely due to a misconfigured policy or the <code>migrationhub-discovery</code> role is missing or not configured correctly.</p>
    PolicyError(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
}

impl DescribeApplicationStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeApplicationStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeApplicationStateError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeApplicationStateError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeApplicationStateError::InvalidInput(
                        err.msg,
                    ))
                }
                "PolicyErrorException" => {
                    return RusotoError::Service(DescribeApplicationStateError::PolicyError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeApplicationStateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeApplicationStateError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeApplicationStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeApplicationStateError {
    fn description(&self) -> &str {
        match *self {
            DescribeApplicationStateError::AccessDenied(ref cause) => cause,
            DescribeApplicationStateError::InternalServerError(ref cause) => cause,
            DescribeApplicationStateError::InvalidInput(ref cause) => cause,
            DescribeApplicationStateError::PolicyError(ref cause) => cause,
            DescribeApplicationStateError::ResourceNotFound(ref cause) => cause,
            DescribeApplicationStateError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMigrationTask
#[derive(Debug, PartialEq)]
pub enum DescribeMigrationTaskError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
}

impl DescribeMigrationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMigrationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeMigrationTaskError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeMigrationTaskError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeMigrationTaskError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeMigrationTaskError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeMigrationTaskError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMigrationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMigrationTaskError {
    fn description(&self) -> &str {
        match *self {
            DescribeMigrationTaskError::AccessDenied(ref cause) => cause,
            DescribeMigrationTaskError::InternalServerError(ref cause) => cause,
            DescribeMigrationTaskError::InvalidInput(ref cause) => cause,
            DescribeMigrationTaskError::ResourceNotFound(ref cause) => cause,
            DescribeMigrationTaskError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateCreatedArtifact
#[derive(Debug, PartialEq)]
pub enum DisassociateCreatedArtifactError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl DisassociateCreatedArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateCreatedArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateCreatedArtifactError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(DisassociateCreatedArtifactError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DisassociateCreatedArtifactError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateCreatedArtifactError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateCreatedArtifactError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateCreatedArtifactError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        DisassociateCreatedArtifactError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateCreatedArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateCreatedArtifactError {
    fn description(&self) -> &str {
        match *self {
            DisassociateCreatedArtifactError::AccessDenied(ref cause) => cause,
            DisassociateCreatedArtifactError::DryRunOperation(ref cause) => cause,
            DisassociateCreatedArtifactError::InternalServerError(ref cause) => cause,
            DisassociateCreatedArtifactError::InvalidInput(ref cause) => cause,
            DisassociateCreatedArtifactError::ResourceNotFound(ref cause) => cause,
            DisassociateCreatedArtifactError::ServiceUnavailable(ref cause) => cause,
            DisassociateCreatedArtifactError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateDiscoveredResource
#[derive(Debug, PartialEq)]
pub enum DisassociateDiscoveredResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl DisassociateDiscoveredResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateDiscoveredResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateDiscoveredResourceError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(
                        DisassociateDiscoveredResourceError::DryRunOperation(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DisassociateDiscoveredResourceError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateDiscoveredResourceError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDiscoveredResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateDiscoveredResourceError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        DisassociateDiscoveredResourceError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateDiscoveredResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDiscoveredResourceError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDiscoveredResourceError::AccessDenied(ref cause) => cause,
            DisassociateDiscoveredResourceError::DryRunOperation(ref cause) => cause,
            DisassociateDiscoveredResourceError::InternalServerError(ref cause) => cause,
            DisassociateDiscoveredResourceError::InvalidInput(ref cause) => cause,
            DisassociateDiscoveredResourceError::ResourceNotFound(ref cause) => cause,
            DisassociateDiscoveredResourceError::ServiceUnavailable(ref cause) => cause,
            DisassociateDiscoveredResourceError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportMigrationTask
#[derive(Debug, PartialEq)]
pub enum ImportMigrationTaskError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl ImportMigrationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportMigrationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ImportMigrationTaskError::AccessDenied(err.msg))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(ImportMigrationTaskError::DryRunOperation(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ImportMigrationTaskError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ImportMigrationTaskError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ImportMigrationTaskError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ImportMigrationTaskError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(ImportMigrationTaskError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportMigrationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportMigrationTaskError {
    fn description(&self) -> &str {
        match *self {
            ImportMigrationTaskError::AccessDenied(ref cause) => cause,
            ImportMigrationTaskError::DryRunOperation(ref cause) => cause,
            ImportMigrationTaskError::InternalServerError(ref cause) => cause,
            ImportMigrationTaskError::InvalidInput(ref cause) => cause,
            ImportMigrationTaskError::ResourceNotFound(ref cause) => cause,
            ImportMigrationTaskError::ServiceUnavailable(ref cause) => cause,
            ImportMigrationTaskError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCreatedArtifacts
#[derive(Debug, PartialEq)]
pub enum ListCreatedArtifactsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
}

impl ListCreatedArtifactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCreatedArtifactsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListCreatedArtifactsError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListCreatedArtifactsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListCreatedArtifactsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListCreatedArtifactsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListCreatedArtifactsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCreatedArtifactsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCreatedArtifactsError {
    fn description(&self) -> &str {
        match *self {
            ListCreatedArtifactsError::AccessDenied(ref cause) => cause,
            ListCreatedArtifactsError::InternalServerError(ref cause) => cause,
            ListCreatedArtifactsError::InvalidInput(ref cause) => cause,
            ListCreatedArtifactsError::ResourceNotFound(ref cause) => cause,
            ListCreatedArtifactsError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDiscoveredResources
#[derive(Debug, PartialEq)]
pub enum ListDiscoveredResourcesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
}

impl ListDiscoveredResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDiscoveredResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDiscoveredResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDiscoveredResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListDiscoveredResourcesError::AccessDenied(ref cause) => cause,
            ListDiscoveredResourcesError::InternalServerError(ref cause) => cause,
            ListDiscoveredResourcesError::InvalidInput(ref cause) => cause,
            ListDiscoveredResourcesError::ResourceNotFound(ref cause) => cause,
            ListDiscoveredResourcesError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by ListMigrationTasks
#[derive(Debug, PartialEq)]
pub enum ListMigrationTasksError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when there are problems accessing ADS (Application Discovery Service); most likely due to a misconfigured policy or the <code>migrationhub-discovery</code> role is missing or not configured correctly.</p>
    PolicyError(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
}

impl ListMigrationTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMigrationTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListMigrationTasksError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListMigrationTasksError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListMigrationTasksError::InvalidInput(err.msg))
                }
                "PolicyErrorException" => {
                    return RusotoError::Service(ListMigrationTasksError::PolicyError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListMigrationTasksError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListMigrationTasksError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListMigrationTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMigrationTasksError {
    fn description(&self) -> &str {
        match *self {
            ListMigrationTasksError::AccessDenied(ref cause) => cause,
            ListMigrationTasksError::InternalServerError(ref cause) => cause,
            ListMigrationTasksError::InvalidInput(ref cause) => cause,
            ListMigrationTasksError::PolicyError(ref cause) => cause,
            ListMigrationTasksError::ResourceNotFound(ref cause) => cause,
            ListMigrationTasksError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProgressUpdateStreams
#[derive(Debug, PartialEq)]
pub enum ListProgressUpdateStreamsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
}

impl ListProgressUpdateStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProgressUpdateStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListProgressUpdateStreamsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        ListProgressUpdateStreamsError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListProgressUpdateStreamsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListProgressUpdateStreamsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListProgressUpdateStreamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProgressUpdateStreamsError {
    fn description(&self) -> &str {
        match *self {
            ListProgressUpdateStreamsError::AccessDenied(ref cause) => cause,
            ListProgressUpdateStreamsError::InternalServerError(ref cause) => cause,
            ListProgressUpdateStreamsError::InvalidInput(ref cause) => cause,
            ListProgressUpdateStreamsError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by NotifyApplicationState
#[derive(Debug, PartialEq)]
pub enum NotifyApplicationStateError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when there are problems accessing ADS (Application Discovery Service); most likely due to a misconfigured policy or the <code>migrationhub-discovery</code> role is missing or not configured correctly.</p>
    PolicyError(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl NotifyApplicationStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<NotifyApplicationStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(NotifyApplicationStateError::AccessDenied(err.msg))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(NotifyApplicationStateError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(NotifyApplicationStateError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(NotifyApplicationStateError::InvalidInput(err.msg))
                }
                "PolicyErrorException" => {
                    return RusotoError::Service(NotifyApplicationStateError::PolicyError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(NotifyApplicationStateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(NotifyApplicationStateError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        NotifyApplicationStateError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for NotifyApplicationStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for NotifyApplicationStateError {
    fn description(&self) -> &str {
        match *self {
            NotifyApplicationStateError::AccessDenied(ref cause) => cause,
            NotifyApplicationStateError::DryRunOperation(ref cause) => cause,
            NotifyApplicationStateError::InternalServerError(ref cause) => cause,
            NotifyApplicationStateError::InvalidInput(ref cause) => cause,
            NotifyApplicationStateError::PolicyError(ref cause) => cause,
            NotifyApplicationStateError::ResourceNotFound(ref cause) => cause,
            NotifyApplicationStateError::ServiceUnavailable(ref cause) => cause,
            NotifyApplicationStateError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by NotifyMigrationTaskState
#[derive(Debug, PartialEq)]
pub enum NotifyMigrationTaskStateError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl NotifyMigrationTaskStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<NotifyMigrationTaskStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(NotifyMigrationTaskStateError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(NotifyMigrationTaskStateError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        NotifyMigrationTaskStateError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(NotifyMigrationTaskStateError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(NotifyMigrationTaskStateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(NotifyMigrationTaskStateError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(
                        NotifyMigrationTaskStateError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for NotifyMigrationTaskStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for NotifyMigrationTaskStateError {
    fn description(&self) -> &str {
        match *self {
            NotifyMigrationTaskStateError::AccessDenied(ref cause) => cause,
            NotifyMigrationTaskStateError::DryRunOperation(ref cause) => cause,
            NotifyMigrationTaskStateError::InternalServerError(ref cause) => cause,
            NotifyMigrationTaskStateError::InvalidInput(ref cause) => cause,
            NotifyMigrationTaskStateError::ResourceNotFound(ref cause) => cause,
            NotifyMigrationTaskStateError::ServiceUnavailable(ref cause) => cause,
            NotifyMigrationTaskStateError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by PutResourceAttributes
#[derive(Debug, PartialEq)]
pub enum PutResourceAttributesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate a successfully authorized action when the <code>DryRun</code> flag is set to "true".</p>
    DryRunOperation(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when the request references a resource (ADS configuration, update stream, migration task, etc.) that does not exist in ADS (Application Discovery Service) or in Migration Hub's repository.</p>
    ResourceNotFound(String),
    /// <p>Exception raised when there is an internal, configuration, or dependency error encountered.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised to indicate a request was not authorized when the <code>DryRun</code> flag is set to "true".</p>
    UnauthorizedOperation(String),
}

impl PutResourceAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourceAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutResourceAttributesError::AccessDenied(err.msg))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(PutResourceAttributesError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutResourceAttributesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutResourceAttributesError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutResourceAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutResourceAttributesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperation" => {
                    return RusotoError::Service(PutResourceAttributesError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutResourceAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutResourceAttributesError {
    fn description(&self) -> &str {
        match *self {
            PutResourceAttributesError::AccessDenied(ref cause) => cause,
            PutResourceAttributesError::DryRunOperation(ref cause) => cause,
            PutResourceAttributesError::InternalServerError(ref cause) => cause,
            PutResourceAttributesError::InvalidInput(ref cause) => cause,
            PutResourceAttributesError::ResourceNotFound(ref cause) => cause,
            PutResourceAttributesError::ServiceUnavailable(ref cause) => cause,
            PutResourceAttributesError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Migration Hub API. AWS Migration Hub clients implement this trait.
pub trait MigrationHub {
    /// <p><p>Associates a created artifact of an AWS cloud resource, the target receiving the migration, with the migration task performed by a migration tool. This API has the following traits:</p> <ul> <li> <p>Migration tools can call the <code>AssociateCreatedArtifact</code> operation to indicate which AWS artifact is associated with a migration task.</p> </li> <li> <p>The created artifact name must be provided in ARN (Amazon Resource Name) format which will contain information about type and region; for example: <code>arn:aws:ec2:us-east-1:488216288981:image/ami-6d0ba87b</code>.</p> </li> <li> <p>Examples of the AWS resource behind the created artifact are, AMI&#39;s, EC2 instance, or DMS endpoint, etc.</p> </li> </ul></p>
    fn associate_created_artifact(
        &self,
        input: AssociateCreatedArtifactRequest,
    ) -> RusotoFuture<AssociateCreatedArtifactResult, AssociateCreatedArtifactError>;

    /// <p>Associates a discovered resource ID from Application Discovery Service (ADS) with a migration task.</p>
    fn associate_discovered_resource(
        &self,
        input: AssociateDiscoveredResourceRequest,
    ) -> RusotoFuture<AssociateDiscoveredResourceResult, AssociateDiscoveredResourceError>;

    /// <p>Creates a progress update stream which is an AWS resource used for access control as well as a namespace for migration task names that is implicitly linked to your AWS account. It must uniquely identify the migration tool as it is used for all updates made by the tool; however, it does not need to be unique for each AWS account because it is scoped to the AWS account.</p>
    fn create_progress_update_stream(
        &self,
        input: CreateProgressUpdateStreamRequest,
    ) -> RusotoFuture<CreateProgressUpdateStreamResult, CreateProgressUpdateStreamError>;

    /// <p><p>Deletes a progress update stream, including all of its tasks, which was previously created as an AWS resource used for access control. This API has the following traits:</p> <ul> <li> <p>The only parameter needed for <code>DeleteProgressUpdateStream</code> is the stream name (same as a <code>CreateProgressUpdateStream</code> call).</p> </li> <li> <p>The call will return, and a background process will asynchronously delete the stream and all of its resources (tasks, associated resources, resource attributes, created artifacts).</p> </li> <li> <p>If the stream takes time to be deleted, it might still show up on a <code>ListProgressUpdateStreams</code> call.</p> </li> <li> <p> <code>CreateProgressUpdateStream</code>, <code>ImportMigrationTask</code>, <code>NotifyMigrationTaskState</code>, and all Associate[*] APIs realted to the tasks belonging to the stream will throw &quot;InvalidInputException&quot; if the stream of the same name is in the process of being deleted.</p> </li> <li> <p>Once the stream and all of its resources are deleted, <code>CreateProgressUpdateStream</code> for a stream of the same name will succeed, and that stream will be an entirely new logical resource (without any resources associated with the old stream).</p> </li> </ul></p>
    fn delete_progress_update_stream(
        &self,
        input: DeleteProgressUpdateStreamRequest,
    ) -> RusotoFuture<DeleteProgressUpdateStreamResult, DeleteProgressUpdateStreamError>;

    /// <p>Gets the migration status of an application.</p>
    fn describe_application_state(
        &self,
        input: DescribeApplicationStateRequest,
    ) -> RusotoFuture<DescribeApplicationStateResult, DescribeApplicationStateError>;

    /// <p>Retrieves a list of all attributes associated with a specific migration task.</p>
    fn describe_migration_task(
        &self,
        input: DescribeMigrationTaskRequest,
    ) -> RusotoFuture<DescribeMigrationTaskResult, DescribeMigrationTaskError>;

    /// <p><p>Disassociates a created artifact of an AWS resource with a migration task performed by a migration tool that was previously associated. This API has the following traits:</p> <ul> <li> <p>A migration user can call the <code>DisassociateCreatedArtifacts</code> operation to disassociate a created AWS Artifact from a migration task.</p> </li> <li> <p>The created artifact name must be provided in ARN (Amazon Resource Name) format which will contain information about type and region; for example: <code>arn:aws:ec2:us-east-1:488216288981:image/ami-6d0ba87b</code>.</p> </li> <li> <p>Examples of the AWS resource behind the created artifact are, AMI&#39;s, EC2 instance, or RDS instance, etc.</p> </li> </ul></p>
    fn disassociate_created_artifact(
        &self,
        input: DisassociateCreatedArtifactRequest,
    ) -> RusotoFuture<DisassociateCreatedArtifactResult, DisassociateCreatedArtifactError>;

    /// <p>Disassociate an Application Discovery Service (ADS) discovered resource from a migration task.</p>
    fn disassociate_discovered_resource(
        &self,
        input: DisassociateDiscoveredResourceRequest,
    ) -> RusotoFuture<DisassociateDiscoveredResourceResult, DisassociateDiscoveredResourceError>;

    /// <p>Registers a new migration task which represents a server, database, etc., being migrated to AWS by a migration tool.</p> <p>This API is a prerequisite to calling the <code>NotifyMigrationTaskState</code> API as the migration tool must first register the migration task with Migration Hub.</p>
    fn import_migration_task(
        &self,
        input: ImportMigrationTaskRequest,
    ) -> RusotoFuture<ImportMigrationTaskResult, ImportMigrationTaskError>;

    /// <p><p>Lists the created artifacts attached to a given migration task in an update stream. This API has the following traits:</p> <ul> <li> <p>Gets the list of the created artifacts while migration is taking place.</p> </li> <li> <p>Shows the artifacts created by the migration tool that was associated by the <code>AssociateCreatedArtifact</code> API. </p> </li> <li> <p>Lists created artifacts in a paginated interface. </p> </li> </ul></p>
    fn list_created_artifacts(
        &self,
        input: ListCreatedArtifactsRequest,
    ) -> RusotoFuture<ListCreatedArtifactsResult, ListCreatedArtifactsError>;

    /// <p>Lists discovered resources associated with the given <code>MigrationTask</code>.</p>
    fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListDiscoveredResourcesResult, ListDiscoveredResourcesError>;

    /// <p><p>Lists all, or filtered by resource name, migration tasks associated with the user account making this call. This API has the following traits:</p> <ul> <li> <p>Can show a summary list of the most recent migration tasks.</p> </li> <li> <p>Can show a summary list of migration tasks associated with a given discovered resource.</p> </li> <li> <p>Lists migration tasks in a paginated interface.</p> </li> </ul></p>
    fn list_migration_tasks(
        &self,
        input: ListMigrationTasksRequest,
    ) -> RusotoFuture<ListMigrationTasksResult, ListMigrationTasksError>;

    /// <p>Lists progress update streams associated with the user account making this call.</p>
    fn list_progress_update_streams(
        &self,
        input: ListProgressUpdateStreamsRequest,
    ) -> RusotoFuture<ListProgressUpdateStreamsResult, ListProgressUpdateStreamsError>;

    /// <p>Sets the migration state of an application. For a given application identified by the value passed to <code>ApplicationId</code>, its status is set or updated by passing one of three values to <code>Status</code>: <code>NOT_STARTED | IN_PROGRESS | COMPLETED</code>.</p>
    fn notify_application_state(
        &self,
        input: NotifyApplicationStateRequest,
    ) -> RusotoFuture<NotifyApplicationStateResult, NotifyApplicationStateError>;

    /// <p><p>Notifies Migration Hub of the current status, progress, or other detail regarding a migration task. This API has the following traits:</p> <ul> <li> <p>Migration tools will call the <code>NotifyMigrationTaskState</code> API to share the latest progress and status.</p> </li> <li> <p> <code>MigrationTaskName</code> is used for addressing updates to the correct target.</p> </li> <li> <p> <code>ProgressUpdateStream</code> is used for access control and to provide a namespace for each migration tool.</p> </li> </ul></p>
    fn notify_migration_task_state(
        &self,
        input: NotifyMigrationTaskStateRequest,
    ) -> RusotoFuture<NotifyMigrationTaskStateResult, NotifyMigrationTaskStateError>;

    /// <p><p>Provides identifying details of the resource being migrated so that it can be associated in the Application Discovery Service (ADS)&#39;s repository. This association occurs asynchronously after <code>PutResourceAttributes</code> returns.</p> <important> <ul> <li> <p>Keep in mind that subsequent calls to PutResourceAttributes will override previously stored attributes. For example, if it is first called with a MAC address, but later, it is desired to <i>add</i> an IP address, it will then be required to call it with <i>both</i> the IP and MAC addresses to prevent overiding the MAC address.</p> </li> <li> <p>Note the instructions regarding the special use case of the <a href="https://docs.aws.amazon.com/migrationhub/latest/ug/API_PutResourceAttributes.html#migrationhub-PutResourceAttributes-request-ResourceAttributeList"> <code>ResourceAttributeList</code> </a> parameter when specifying any &quot;VM&quot; related value. </p> </li> </ul> </important> <note> <p>Because this is an asynchronous call, it will always return 200, whether an association occurs or not. To confirm if an association was found based on the provided details, call <code>ListDiscoveredResources</code>.</p> </note></p>
    fn put_resource_attributes(
        &self,
        input: PutResourceAttributesRequest,
    ) -> RusotoFuture<PutResourceAttributesResult, PutResourceAttributesError>;
}
/// A client for the AWS Migration Hub API.
#[derive(Clone)]
pub struct MigrationHubClient {
    client: Client,
    region: region::Region,
}

impl MigrationHubClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MigrationHubClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MigrationHubClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MigrationHubClient {
        MigrationHubClient { client, region }
    }
}

impl MigrationHub for MigrationHubClient {
    /// <p><p>Associates a created artifact of an AWS cloud resource, the target receiving the migration, with the migration task performed by a migration tool. This API has the following traits:</p> <ul> <li> <p>Migration tools can call the <code>AssociateCreatedArtifact</code> operation to indicate which AWS artifact is associated with a migration task.</p> </li> <li> <p>The created artifact name must be provided in ARN (Amazon Resource Name) format which will contain information about type and region; for example: <code>arn:aws:ec2:us-east-1:488216288981:image/ami-6d0ba87b</code>.</p> </li> <li> <p>Examples of the AWS resource behind the created artifact are, AMI&#39;s, EC2 instance, or DMS endpoint, etc.</p> </li> </ul></p>
    fn associate_created_artifact(
        &self,
        input: AssociateCreatedArtifactRequest,
    ) -> RusotoFuture<AssociateCreatedArtifactResult, AssociateCreatedArtifactError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.AssociateCreatedArtifact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateCreatedArtifactResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateCreatedArtifactError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates a discovered resource ID from Application Discovery Service (ADS) with a migration task.</p>
    fn associate_discovered_resource(
        &self,
        input: AssociateDiscoveredResourceRequest,
    ) -> RusotoFuture<AssociateDiscoveredResourceResult, AssociateDiscoveredResourceError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSMigrationHub.AssociateDiscoveredResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateDiscoveredResourceResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDiscoveredResourceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a progress update stream which is an AWS resource used for access control as well as a namespace for migration task names that is implicitly linked to your AWS account. It must uniquely identify the migration tool as it is used for all updates made by the tool; however, it does not need to be unique for each AWS account because it is scoped to the AWS account.</p>
    fn create_progress_update_stream(
        &self,
        input: CreateProgressUpdateStreamRequest,
    ) -> RusotoFuture<CreateProgressUpdateStreamResult, CreateProgressUpdateStreamError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.CreateProgressUpdateStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateProgressUpdateStreamResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateProgressUpdateStreamError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Deletes a progress update stream, including all of its tasks, which was previously created as an AWS resource used for access control. This API has the following traits:</p> <ul> <li> <p>The only parameter needed for <code>DeleteProgressUpdateStream</code> is the stream name (same as a <code>CreateProgressUpdateStream</code> call).</p> </li> <li> <p>The call will return, and a background process will asynchronously delete the stream and all of its resources (tasks, associated resources, resource attributes, created artifacts).</p> </li> <li> <p>If the stream takes time to be deleted, it might still show up on a <code>ListProgressUpdateStreams</code> call.</p> </li> <li> <p> <code>CreateProgressUpdateStream</code>, <code>ImportMigrationTask</code>, <code>NotifyMigrationTaskState</code>, and all Associate[*] APIs realted to the tasks belonging to the stream will throw &quot;InvalidInputException&quot; if the stream of the same name is in the process of being deleted.</p> </li> <li> <p>Once the stream and all of its resources are deleted, <code>CreateProgressUpdateStream</code> for a stream of the same name will succeed, and that stream will be an entirely new logical resource (without any resources associated with the old stream).</p> </li> </ul></p>
    fn delete_progress_update_stream(
        &self,
        input: DeleteProgressUpdateStreamRequest,
    ) -> RusotoFuture<DeleteProgressUpdateStreamResult, DeleteProgressUpdateStreamError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.DeleteProgressUpdateStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteProgressUpdateStreamResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProgressUpdateStreamError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the migration status of an application.</p>
    fn describe_application_state(
        &self,
        input: DescribeApplicationStateRequest,
    ) -> RusotoFuture<DescribeApplicationStateResult, DescribeApplicationStateError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.DescribeApplicationState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeApplicationStateResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeApplicationStateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of all attributes associated with a specific migration task.</p>
    fn describe_migration_task(
        &self,
        input: DescribeMigrationTaskRequest,
    ) -> RusotoFuture<DescribeMigrationTaskResult, DescribeMigrationTaskError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.DescribeMigrationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMigrationTaskResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeMigrationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Disassociates a created artifact of an AWS resource with a migration task performed by a migration tool that was previously associated. This API has the following traits:</p> <ul> <li> <p>A migration user can call the <code>DisassociateCreatedArtifacts</code> operation to disassociate a created AWS Artifact from a migration task.</p> </li> <li> <p>The created artifact name must be provided in ARN (Amazon Resource Name) format which will contain information about type and region; for example: <code>arn:aws:ec2:us-east-1:488216288981:image/ami-6d0ba87b</code>.</p> </li> <li> <p>Examples of the AWS resource behind the created artifact are, AMI&#39;s, EC2 instance, or RDS instance, etc.</p> </li> </ul></p>
    fn disassociate_created_artifact(
        &self,
        input: DisassociateCreatedArtifactRequest,
    ) -> RusotoFuture<DisassociateCreatedArtifactResult, DisassociateCreatedArtifactError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSMigrationHub.DisassociateCreatedArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateCreatedArtifactResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateCreatedArtifactError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociate an Application Discovery Service (ADS) discovered resource from a migration task.</p>
    fn disassociate_discovered_resource(
        &self,
        input: DisassociateDiscoveredResourceRequest,
    ) -> RusotoFuture<DisassociateDiscoveredResourceResult, DisassociateDiscoveredResourceError>
    {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSMigrationHub.DisassociateDiscoveredResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateDiscoveredResourceResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDiscoveredResourceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Registers a new migration task which represents a server, database, etc., being migrated to AWS by a migration tool.</p> <p>This API is a prerequisite to calling the <code>NotifyMigrationTaskState</code> API as the migration tool must first register the migration task with Migration Hub.</p>
    fn import_migration_task(
        &self,
        input: ImportMigrationTaskRequest,
    ) -> RusotoFuture<ImportMigrationTaskResult, ImportMigrationTaskError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.ImportMigrationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportMigrationTaskResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportMigrationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Lists the created artifacts attached to a given migration task in an update stream. This API has the following traits:</p> <ul> <li> <p>Gets the list of the created artifacts while migration is taking place.</p> </li> <li> <p>Shows the artifacts created by the migration tool that was associated by the <code>AssociateCreatedArtifact</code> API. </p> </li> <li> <p>Lists created artifacts in a paginated interface. </p> </li> </ul></p>
    fn list_created_artifacts(
        &self,
        input: ListCreatedArtifactsRequest,
    ) -> RusotoFuture<ListCreatedArtifactsResult, ListCreatedArtifactsError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.ListCreatedArtifacts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCreatedArtifactsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListCreatedArtifactsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists discovered resources associated with the given <code>MigrationTask</code>.</p>
    fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListDiscoveredResourcesResult, ListDiscoveredResourcesError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.ListDiscoveredResources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDiscoveredResourcesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDiscoveredResourcesError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Lists all, or filtered by resource name, migration tasks associated with the user account making this call. This API has the following traits:</p> <ul> <li> <p>Can show a summary list of the most recent migration tasks.</p> </li> <li> <p>Can show a summary list of migration tasks associated with a given discovered resource.</p> </li> <li> <p>Lists migration tasks in a paginated interface.</p> </li> </ul></p>
    fn list_migration_tasks(
        &self,
        input: ListMigrationTasksRequest,
    ) -> RusotoFuture<ListMigrationTasksResult, ListMigrationTasksError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.ListMigrationTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListMigrationTasksResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListMigrationTasksError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists progress update streams associated with the user account making this call.</p>
    fn list_progress_update_streams(
        &self,
        input: ListProgressUpdateStreamsRequest,
    ) -> RusotoFuture<ListProgressUpdateStreamsResult, ListProgressUpdateStreamsError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.ListProgressUpdateStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListProgressUpdateStreamsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListProgressUpdateStreamsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets the migration state of an application. For a given application identified by the value passed to <code>ApplicationId</code>, its status is set or updated by passing one of three values to <code>Status</code>: <code>NOT_STARTED | IN_PROGRESS | COMPLETED</code>.</p>
    fn notify_application_state(
        &self,
        input: NotifyApplicationStateRequest,
    ) -> RusotoFuture<NotifyApplicationStateResult, NotifyApplicationStateError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.NotifyApplicationState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<NotifyApplicationStateResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(NotifyApplicationStateError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Notifies Migration Hub of the current status, progress, or other detail regarding a migration task. This API has the following traits:</p> <ul> <li> <p>Migration tools will call the <code>NotifyMigrationTaskState</code> API to share the latest progress and status.</p> </li> <li> <p> <code>MigrationTaskName</code> is used for addressing updates to the correct target.</p> </li> <li> <p> <code>ProgressUpdateStream</code> is used for access control and to provide a namespace for each migration tool.</p> </li> </ul></p>
    fn notify_migration_task_state(
        &self,
        input: NotifyMigrationTaskStateRequest,
    ) -> RusotoFuture<NotifyMigrationTaskStateResult, NotifyMigrationTaskStateError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.NotifyMigrationTaskState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<NotifyMigrationTaskStateResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(NotifyMigrationTaskStateError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Provides identifying details of the resource being migrated so that it can be associated in the Application Discovery Service (ADS)&#39;s repository. This association occurs asynchronously after <code>PutResourceAttributes</code> returns.</p> <important> <ul> <li> <p>Keep in mind that subsequent calls to PutResourceAttributes will override previously stored attributes. For example, if it is first called with a MAC address, but later, it is desired to <i>add</i> an IP address, it will then be required to call it with <i>both</i> the IP and MAC addresses to prevent overiding the MAC address.</p> </li> <li> <p>Note the instructions regarding the special use case of the <a href="https://docs.aws.amazon.com/migrationhub/latest/ug/API_PutResourceAttributes.html#migrationhub-PutResourceAttributes-request-ResourceAttributeList"> <code>ResourceAttributeList</code> </a> parameter when specifying any &quot;VM&quot; related value. </p> </li> </ul> </important> <note> <p>Because this is an asynchronous call, it will always return 200, whether an association occurs or not. To confirm if an association was found based on the provided details, call <code>ListDiscoveredResources</code>.</p> </note></p>
    fn put_resource_attributes(
        &self,
        input: PutResourceAttributesRequest,
    ) -> RusotoFuture<PutResourceAttributesResult, PutResourceAttributesError> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMigrationHub.PutResourceAttributes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutResourceAttributesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutResourceAttributesError::from_response(response))
                    }),
                )
            }
        })
    }
}

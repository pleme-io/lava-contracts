//! lava-contracts — typed result contracts for lava architectures.
//!
//! Cross-architecture composition validates at *compile time* via
//! typed result objects. Pangea's `NetworkResult.vpc.id` →
//! `IamResult.role_arn` → `ClusterResult.kubeconfig_secret_ref`
//! chain becomes lava's:
//!
//! ```lisp
//! (define net (build-architecture aws-vpc-network :cidr "10.0.0.0/16"))
//! (define iam (build-architecture aws-iam-roles    :vpc-id (network-vpc-id net)))
//! (define cls (build-architecture aws-eks-cluster
//!               :network net
//!               :assume-role (iam-role-arn iam)))
//! ```
//!
//! Each `*Result` is a typed Rust struct that:
//! 1. Holds typed [`lava_core::ResourceRef`]s — not raw strings.
//! 2. Exposes named accessors (`network_result.vpc_id()`) so
//!    consumers can't fat-finger attribute names.
//! 3. Round-trips through serde for caixa-source ingestion + remote
//!    state crossing.
//!
//! The Trait-per-Concern pattern (★★ TYPED EMISSION corollary):
//! every result kind implements [`ArchitectureResult`]. Operators
//! consume an `Arc<dyn ArchitectureResult>` when generic composition
//! is needed; specific consumers downcast to the typed shape.

#![allow(clippy::module_name_repetitions)]

use lava_core::ResourceRef;
use serde::{Deserialize, Serialize};

/// Common trait every architecture result implements. Downstream
/// architectures take `&dyn ArchitectureResult` when generic over
/// the result shape; specific consumers downcast.
pub trait ArchitectureResult {
    /// Architecture name that produced this result.
    fn architecture_name(&self) -> &str;
    /// Stable typed key — used by routing layers + drift detectors.
    fn result_kind(&self) -> &str;
}

/// Output of an AWS VPC network architecture. Holds typed refs to
/// every resource downstream architectures consume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkResult {
    pub architecture: String,
    pub vpc_id: ResourceRef,
    pub vpc_cidr: String,
    pub public_subnet_ids: Vec<ResourceRef>,
    pub private_subnet_ids: Vec<ResourceRef>,
    pub internet_gateway_id: Option<ResourceRef>,
    pub nat_gateway_ids: Vec<ResourceRef>,
}

impl ArchitectureResult for NetworkResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "network" }
}

/// Output of an IAM/identity architecture. The role ARN + assume-role
/// policy serve as the typed key downstream workload architectures
/// consume when assuming the role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IamResult {
    pub architecture: String,
    pub role_arn: ResourceRef,
    pub role_name: ResourceRef,
    pub assume_role_policy_arn: Option<ResourceRef>,
    pub instance_profile_arn: Option<ResourceRef>,
}

impl ArchitectureResult for IamResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "iam" }
}

/// Output of a Kubernetes cluster architecture. The kubeconfig is
/// passed as a typed SecretRef so downstream HelmRelease/Kustomization
/// architectures can consume the cluster without spreading credentials.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterResult {
    pub architecture: String,
    pub cluster_endpoint: ResourceRef,
    pub cluster_name: ResourceRef,
    pub kubeconfig_secret_ref: Option<SecretRef>,
    pub ca_certificate: ResourceRef,
    pub oidc_provider_arn: Option<ResourceRef>,
}

impl ArchitectureResult for ClusterResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "cluster" }
}

/// Output of a secrets-management architecture. Typed refs to vault/
/// akeyless paths consumers fetch through the secret backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretsResult {
    pub architecture: String,
    pub backend_kind: SecretBackendKind,
    pub secret_refs: Vec<SecretRef>,
}

impl ArchitectureResult for SecretsResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "secrets" }
}

/// Typed handle to a secret value. Never the value itself — always
/// the reference. Resolved through the matching backend at apply time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRef {
    pub backend: SecretBackendKind,
    pub path: String,
    /// Optional sub-key within the secret (for k=v stores).
    pub key: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBackendKind {
    Akeyless,
    HashicorpVault,
    KubernetesSecret,
    AwsSecretsManager,
    GcpSecretManager,
    AzureKeyVault,
    Sops,
}

/// Output of a DNS architecture. Each Record is a typed entry the
/// downstream certificate/cdn/ingress architectures consume by name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsResult {
    pub architecture: String,
    pub zone_id: ResourceRef,
    pub zone_name: String,
    pub records: Vec<DnsRecord>,
}

impl ArchitectureResult for DnsResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "dns" }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsRecord {
    pub fqdn: String,
    pub record_type: String,
    pub record_id: ResourceRef,
}

/// Output of a storage architecture (S3 bucket / GCS bucket / Azure
/// blob container). The arn + region + name flow to downstream
/// workload architectures that need to read/write objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageResult {
    pub architecture: String,
    pub bucket_id: ResourceRef,
    pub bucket_arn: ResourceRef,
    pub region: String,
}

impl ArchitectureResult for StorageResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "storage" }
}

/// Output of a load-balancer architecture. Endpoint + listener arns
/// for downstream target-group registration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerResult {
    pub architecture: String,
    pub lb_arn: ResourceRef,
    pub lb_dns_name: ResourceRef,
    pub listener_arns: Vec<ResourceRef>,
}

impl ArchitectureResult for LoadBalancerResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "load_balancer" }
}

/// Output of an observability architecture (Datadog / Grafana /
/// NewRelic). API keys + ingest endpoints + dashboard refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservabilityResult {
    pub architecture: String,
    pub api_key_secret_ref: SecretRef,
    pub ingest_endpoint: String,
    pub dashboards: Vec<ResourceRef>,
}

impl ArchitectureResult for ObservabilityResult {
    fn architecture_name(&self) -> &str { &self.architecture }
    fn result_kind(&self) -> &str { "observability" }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rref(t: &str, n: &str, a: &str) -> ResourceRef {
        ResourceRef {
            type_id: t.to_string(),
            name: n.to_string(),
            attribute: a.to_string(),
        }
    }

    #[test]
    fn network_result_round_trips_through_serde() {
        let net = NetworkResult {
            architecture: "aws-vpc-network".to_string(),
            vpc_id: rref("aws_vpc", "main", "id"),
            vpc_cidr: "10.0.0.0/16".to_string(),
            public_subnet_ids: vec![
                rref("aws_subnet", "public-a", "id"),
                rref("aws_subnet", "public-b", "id"),
            ],
            private_subnet_ids: vec![],
            internet_gateway_id: Some(rref("aws_internet_gateway", "igw", "id")),
            nat_gateway_ids: vec![],
        };
        let json = serde_json::to_string(&net).unwrap();
        let parsed: NetworkResult = serde_json::from_str(&json).unwrap();
        assert_eq!(net, parsed);
        assert_eq!(net.architecture_name(), "aws-vpc-network");
        assert_eq!(net.result_kind(), "network");
    }

    #[test]
    fn secrets_result_typed_backend_kind() {
        let s = SecretsResult {
            architecture: "akeyless-aws-integration".to_string(),
            backend_kind: SecretBackendKind::Akeyless,
            secret_refs: vec![SecretRef {
                backend: SecretBackendKind::Akeyless,
                path: "/pleme/prod/db-password".to_string(),
                key: None,
            }],
        };
        assert_eq!(s.result_kind(), "secrets");
        assert!(matches!(s.backend_kind, SecretBackendKind::Akeyless));
    }

    #[test]
    fn architecture_result_trait_is_object_safe() {
        // Composition layer holds heterogeneous results; verify the
        // trait can be boxed into Arc<dyn ArchitectureResult>.
        let net: Box<dyn ArchitectureResult> = Box::new(NetworkResult {
            architecture: "a".to_string(),
            vpc_id: rref("aws_vpc", "v", "id"),
            vpc_cidr: "10.0.0.0/16".to_string(),
            public_subnet_ids: vec![],
            private_subnet_ids: vec![],
            internet_gateway_id: None,
            nat_gateway_ids: vec![],
        });
        assert_eq!(net.result_kind(), "network");
    }
}

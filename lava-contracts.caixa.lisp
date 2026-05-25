(defcaixa
  :name
  "lava-contracts"
  :kind
  :Biblioteca
  :ecosystem
  :rust-single-crate
  :package
  {:name "lava-contracts"
   :version "0.1.0"
   :description "Typed result contracts for lava architectures: NetworkResult / IamResult / ClusterResult / etc. Cross-architecture composition validates at compile time, not apply time. Pangea::Contracts::*Result analog."
   :license "MIT"
   :repository "https://github.com/pleme-io/lava-contracts"}
  :ci-config
  {:bump {:default-type "patch"}
   :publish {:no-verify true}}
  :workflows
  [:auto-release :pre-merge-gate :security-gate])

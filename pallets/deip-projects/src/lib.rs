#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode}, debug,
    decl_module, decl_storage, decl_event, decl_error, 
    StorageMap
};
use frame_system::{ self as system, ensure_signed };
use sp_std::vec::Vec;
use sp_runtime::{ RuntimeDebug };
use sp_core::{ H160 };

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Project<Hash, H160> {
    is_private: bool,
    external_id: H160,
    team: H160,
    description: Hash,
    
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
    pub enum Event<T> 
    where 
        AccountId = <T as frame_system::Trait>::AccountId,
        Hash = <T as system::Trait>::Hash
    {
        /// Event emitted when a project has been created. [BelongsTo, Project]
        ProjectCreated(AccountId, Project<Hash, H160>),
        /// Event emitted when a project is removed by the owner. [BelongsTo, Project]
        ProjectRemoved(AccountId, Project<Hash, H160>),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// The project does not exist, so it cannot be removed.
        NoSuchProject,
        /// The project is created by another account, so caller can't remove it.
        NotProjectOwner,
    }
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as DeipProjects {
        /// The storage item for our projects.
        /// It maps a projects to the user who created the them.
        Projects get(fn projects): map hasher(blake2_128_concat) T::AccountId => Vec<Project<T::Hash, H160>>;
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Allow a user to create project.
        #[weight = 10_000]
        fn create_project(origin, is_private: bool, external_id: H160, team: H160, description: T::Hash) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;

            let mut projects = Projects::<T>::get(&account);
 
            let project = Project {
                is_private,
                external_id,
                team,
                description
            };

            // Modify Projects List via adding new Project
            projects.push(project);

            // Store the projects related to accountt
            Projects::<T>::insert(&account, projects);

            // Emit an event that the project was created.
            // Self::deposit_event(RawEvent::ProjectCreated(account, project));
        }
        
        /// Allow a user to create full project.
        #[weight = 10_000]
        fn create_project_full(origin, project: Project<T::Hash, H160>) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;

            let mut projects = Projects::<T>::get(&account);

            // Modify Projects List via adding new Project
            projects.push(project);

            // Store the projects related to accountt
            Projects::<T>::insert(&account, projects);

            // Emit an event that the project was created.
            // Self::deposit_event(RawEvent::ProjectCreated(account, project));
        }
    }
}
